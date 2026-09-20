//! Exact immutable geometry snapshots, issued only after complete proofs.
use super::*;
use std::sync::Arc;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmbeddingProofMethod {
    FullPairs,
    PreparedBaseline,
    PreparedFinal,
    PreparedRetopologized,
    Radial,
    ExactReindex,
}
struct Snapshot {
    positions: Vec<[u32; 3]>,
    triangles: Vec<[u32; 3]>,
    method: EmbeddingProofMethod,
    proof_work: usize,
}
#[derive(Clone)]
pub struct EmbeddingCertificate {
    snapshot: Arc<Snapshot>,
}
impl std::fmt::Debug for EmbeddingCertificate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EmbeddingCertificate")
            .field("vertices", &self.snapshot.positions.len())
            .field("triangles", &self.snapshot.triangles.len())
            .field("method", &self.snapshot.method)
            .finish_non_exhaustive()
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CertificateMatch {
    pub work: usize,
    pub matches: bool,
}
#[derive(Clone, Debug)]
pub struct CertificateCapture {
    pub certificate: EmbeddingCertificate,
    pub work: usize,
}
#[derive(Clone, Debug)]
pub struct CertifiedIntersectionReport {
    pub report: IntersectionReport,
    pub certificate: EmbeddingCertificate,
    pub certificate_work: usize,
    /// New work performed by this call; original proof provenance is not recharged.
    pub work: usize,
}
impl EmbeddingCertificate {
    pub fn method(&self) -> EmbeddingProofMethod {
        self.snapshot.method
    }
    pub fn original_proof_work(&self) -> usize {
        self.snapshot.proof_work
    }
    pub fn vertex_count(&self) -> usize {
        self.snapshot.positions.len()
    }
    pub fn triangle_count(&self) -> usize {
        self.snapshot.triangles.len()
    }
    /// Every reuse compares the exact position bits and ordered index triples.
    /// A match result is descriptive only; trusted constructors repeat this
    /// check themselves and never accept a caller's boolean/report assertion.
    pub fn matches(
        &self,
        positions: &[Vec3],
        triangles: &[[u32; 3]],
        max_work: usize,
    ) -> Result<CertificateMatch, IntersectionFailure> {
        let mut budget = Budget { maximum: max_work, report: IntersectionReport::default(), failed_pair: None };
        let result = (|| {
            budget.charge(1)?;
            if positions.len() > MAX_MESH_VERTICES || triangles.len() > MAX_MESH_TRIANGLES {
                return Err("embedding certificate match exceeds geometry bounds".into());
            }
            if positions.len() != self.snapshot.positions.len() || triangles.len() != self.snapshot.triangles.len() {
                return Ok(false);
            }
            for (p, expected) in positions.iter().zip(&self.snapshot.positions) {
                budget.charge(3)?;
                if [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()] != *expected {
                    return Ok(false);
                }
            }
            for (t, expected) in triangles.iter().zip(&self.snapshot.triangles) {
                budget.charge(3)?;
                if t != expected {
                    return Ok(false);
                }
            }
            Ok(true)
        })();
        result
            .map(|matches| CertificateMatch { work: budget.report.work, matches })
            .map_err(|message| IntersectionFailure { work: budget.report.work, triangles: None, message })
    }

    /// Rebind only an exact bijective renaming of used vertices. Every face
    /// remains in its original slot and order. Unused old vertices must map to
    /// u32::MAX; no welding, new vertices, face removal, or geometric change is
    /// authorized. All verification and the new snapshot are charged here.
    pub fn reindexed(
        &self,
        positions: &[Vec3],
        triangles: &[[u32; 3]],
        old_to_new: &[u32],
        max_work: usize,
    ) -> Result<CertificateCapture, IntersectionFailure> {
        let mut budget = Budget { maximum: max_work, report: IntersectionReport::default(), failed_pair: None };
        let result = (|| {
            budget.charge(1)?;
            if positions.is_empty()
                || positions.len() > MAX_MESH_VERTICES
                || triangles.len() != self.snapshot.triangles.len()
                || old_to_new.len() != self.snapshot.positions.len()
            {
                return Err("embedding certificate reindex dimensions differ from the complete source".into());
            }
            budget.charge(self.snapshot.positions.len() + positions.len())?;
            let mut used = vec![false; self.snapshot.positions.len()];
            let mut inverse = vec![u32::MAX; positions.len()];
            for t in &self.snapshot.triangles {
                budget.charge(3)?;
                for &i in t {
                    used[i as usize] = true;
                }
            }
            for (old, (&active, &new)) in used.iter().zip(old_to_new).enumerate() {
                budget.charge(1)?;
                if !active {
                    if new != u32::MAX {
                        return Err("embedding certificate reindex retained an unused source vertex".into());
                    }
                    continue;
                }
                if new as usize >= positions.len() || inverse[new as usize] != u32::MAX {
                    return Err("embedding certificate reindex is not bijective on used vertices".into());
                }
                budget.charge(4)?;
                let p = positions[new as usize];
                if [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()] != self.snapshot.positions[old] {
                    return Err("embedding certificate reindex changed coordinate bits".into());
                }
                inverse[new as usize] = old as u32;
            }
            budget.charge(inverse.len())?;
            if inverse.contains(&u32::MAX) {
                return Err("embedding certificate reindex has an unmatched output vertex".into());
            }
            for (old, actual) in self.snapshot.triangles.iter().zip(triangles) {
                budget.charge(3)?;
                if old.map(|i| old_to_new[i as usize]) != *actual {
                    return Err("embedding certificate reindex changed an ordered source triangle".into());
                }
            }
            Ok(())
        })();
        result.map_err(|message| IntersectionFailure { work: budget.report.work, triangles: None, message })?;
        let mut captured = capture_completed_embedding(
            positions,
            triangles,
            EmbeddingProofMethod::ExactReindex,
            self.snapshot.proof_work,
            max_work - budget.report.work,
        )
        .map_err(|mut error| {
            error.work += budget.report.work;
            error
        })?;
        captured.work += budget.report.work;
        Ok(captured)
    }
}
/// Crate-private proof boundary: invoke only immediately after a complete
/// stored or radial proof of these same borrowed arrays. No public constructor
/// accepts a report, bool, hash, or purported partial-contact inventory.
pub(crate) fn capture_completed_embedding(
    positions: &[Vec3],
    triangles: &[[u32; 3]],
    method: EmbeddingProofMethod,
    proof_work: usize,
    max_work: usize,
) -> Result<CertificateCapture, IntersectionFailure> {
    let mut budget = Budget { maximum: max_work, report: IntersectionReport::default(), failed_pair: None };
    let result = (|| {
        budget.charge(1)?;
        if positions.is_empty()
            || triangles.is_empty()
            || positions.len() > MAX_MESH_VERTICES
            || triangles.len() > MAX_MESH_TRIANGLES
        {
            return Err("embedding certificate snapshot exceeds geometry bounds".into());
        }
        // Charge both allocations and every copied coordinate/index component
        // before publishing any immutable snapshot.
        budget.charge(positions.len() * 3 + triangles.len() * 3 + 2)?;
        let positions = positions.iter().map(|p| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()]).collect();
        let snapshot = Snapshot { positions, triangles: triangles.to_vec(), method, proof_work };
        Ok(EmbeddingCertificate { snapshot: Arc::new(snapshot) })
    })();
    result
        .map(|certificate| CertificateCapture { certificate, work: budget.report.work })
        .map_err(|message| IntersectionFailure { work: budget.report.work, triangles: None, message })
}
/// Full pair validation followed immediately by an exact immutable snapshot.
/// Existing `validate_counted` behavior is unchanged.
pub fn validate_certified(
    positions: &[Vec3],
    triangles: &[[u32; 3]],
    max_work: usize,
) -> Result<CertifiedIntersectionReport, IntersectionFailure> {
    let report = validate_counted(positions, triangles, max_work)?;
    let captured = capture_completed_embedding(
        positions,
        triangles,
        EmbeddingProofMethod::FullPairs,
        report.work,
        max_work - report.work,
    )
    .map_err(|mut error| {
        error.work += report.work;
        error
    })?;
    Ok(CertifiedIntersectionReport {
        work: report.work + captured.work,
        certificate_work: captured.work,
        report,
        certificate: captured.certificate,
    })
}
