//! One canonical native-project encoder for admission, saving and durable commits.
//! Counting admission does not allocate an output buffer. Both modes stop before
//! exceeding their byte budget and reserve room for future revision digit growth.
use crate::{model::Document, protocol::Failure, storage::MAX_DOCUMENT_BYTES};
use serde::Serialize;
use std::io::{self, Write};

#[derive(Serialize)]
struct View<'a> {
    format: &'static str,
    saved_revision: u64,
    document: &'a Document,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub(crate) struct Budget {
    pub encoded_bytes: u64,
    pub revision_reserve_bytes: u64,
    pub limit_bytes: u64,
    pub remaining_bytes: u64,
}

struct Output {
    bytes: Option<Vec<u8>>,
    written: u64,
    limit: u64,
    exceeded: bool,
}
impl Output {
    fn new(limit: u64, collect: bool) -> Self {
        Self { bytes: collect.then(Vec::new), written: 0, limit, exceeded: false }
    }
}
impl Write for Output {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        let next = self.written.checked_add(data.len() as u64);
        if next.is_none_or(|next| next > self.limit) {
            self.exceeded = true;
            return Err(io::Error::other("native project byte limit exceeded"));
        }
        if let Some(bytes) = &mut self.bytes {
            let needed =
                bytes.len().checked_add(data.len()).ok_or_else(|| io::Error::other("native output length overflow"))?;
            if needed > bytes.capacity() {
                // Geometric growth bounded by the serialization quota; allocation
                // errors are ordinary failed writes, not a partially accepted edit.
                let quota = usize::try_from(self.limit).unwrap_or(usize::MAX);
                let capacity = bytes.capacity().saturating_mul(2).max(needed).min(quota);
                bytes.try_reserve_exact(capacity - bytes.len()).map_err(|e| io::Error::other(e.to_string()))?;
            }
            bytes.extend_from_slice(data);
        }
        self.written = next.expect("checked above");
        Ok(data.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn revision_reserve(revision: u64) -> u64 {
    let digits = if revision == 0 { 1 } else { u64::from(revision.ilog10()) + 1 };
    20 - digits
}
fn serialize(
    document: &Document,
    revision: u64,
    limit: u64,
    collect: bool,
) -> Result<(Budget, Option<Vec<u8>>), Failure> {
    let reserve = revision_reserve(revision);
    let quota = limit.checked_sub(reserve).ok_or_else(|| quota_error(limit, reserve))?;
    let mut output = Output::new(quota, collect);
    let result = serde_json::to_writer_pretty(
        &mut output,
        &View { format: "mm3e-agent-project-v1", saved_revision: revision, document },
    );
    if let Err(error) = result {
        return Err(if output.exceeded {
            quota_error(limit, reserve)
        } else {
            Failure::invalid(format!("cannot serialize native project: {error}"))
        });
    }
    let budget = Budget {
        encoded_bytes: output.written,
        revision_reserve_bytes: reserve,
        limit_bytes: limit,
        remaining_bytes: quota - output.written,
    };
    Ok((budget, output.bytes))
}
fn quota_error(limit: u64, reserve: u64) -> Failure {
    Failure{code:"project_size_limit",message:format!("serialized native project exceeds its {limit}-byte budget, including {reserve} bytes reserved for future revision growth; candidate not accepted")}
}
pub(crate) fn budget(document: &Document, revision: u64) -> Result<Budget, Failure> {
    serialize(document, revision, MAX_DOCUMENT_BYTES, false).map(|(budget, _)| budget)
}
pub(crate) fn bytes(document: &Document, revision: u64) -> Result<Vec<u8>, Failure> {
    serialize(document, revision, MAX_DOCUMENT_BYTES, true)
        .map(|(_, bytes)| bytes.expect("collecting serializer retains bytes"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn document() -> Document {
        Document {objects:serde_json::from_value(json!([{"id":"shape","label":"nul\u{0000}, quote\", slash\\, line\n, café, 🦊","shape":{"type":"sphere","radius":0.2}}])).unwrap(),..Document::default()}
    }
    #[test]
    fn counter_and_collector_match_original_canonical_encoding_including_escapes_and_revision_digits() {
        let document = document();
        for revision in [0, 9, 10, 9999, 10_000, u64::MAX] {
            let expected = serde_json::to_vec_pretty(&View {
                format: "mm3e-agent-project-v1",
                saved_revision: revision,
                document: &document,
            })
            .unwrap();
            let count = budget(&document, revision).unwrap();
            assert_eq!(count.encoded_bytes, expected.len() as u64);
            assert_eq!(bytes(&document, revision).unwrap(), expected);
            assert_eq!(count.encoded_bytes + count.revision_reserve_bytes + count.remaining_bytes, MAX_DOCUMENT_BYTES);
        }
    }
    #[test]
    fn exact_budget_is_accepted_and_one_byte_less_is_rejected_in_both_modes() {
        let document = document();
        for revision in [0, 9, 10, u64::MAX] {
            let actual = budget(&document, revision).unwrap();
            let exact = actual.encoded_bytes + actual.revision_reserve_bytes;
            for collect in [false, true] {
                let (result, data) = serialize(&document, revision, exact, collect).unwrap();
                assert_eq!(result.remaining_bytes, 0);
                assert_eq!(data.is_some(), collect);
                assert_eq!(serialize(&document, revision, exact - 1, collect).unwrap_err().code, "project_size_limit");
            }
        }
    }
    #[test]
    fn accepted_snapshot_budget_is_independent_of_future_revision_digit_growth() {
        let document = document();
        let first = budget(&document, 9).unwrap();
        let exact = first.encoded_bytes + first.revision_reserve_bytes;
        for revision in [9, 10, 100, 10_000, u64::MAX] {
            let (result, _) = serialize(&document, revision, exact, false).unwrap();
            assert_eq!(result.remaining_bytes, 0);
        }
    }
    #[test]
    fn writer_stops_before_overflowing_chunk_and_counting_mode_never_collects_bytes() {
        for collect in [false, true] {
            let mut output = Output::new(4, collect);
            output.write_all(b"ab").unwrap();
            assert!(output.write_all(b"cde").is_err());
            assert_eq!(output.written, 2);
            assert_eq!(output.bytes.as_deref(), if collect { Some(&b"ab"[..]) } else { None });
            assert!(output.exceeded);
        }
        let mut output = Output::new(u64::MAX, false);
        output.written = u64::MAX;
        assert!(output.write_all(b"x").is_err());
    }
}
