//! trace_filter: record traces, each tagged with a kind, against a node, then
//! report only the traces of a requested kind in record order. Each reported
//! trace is rendered as `kind@node`.

/// A filtered view of the trace log for a single kind: the kind that was
/// queried plus the matching traces, in record order, each rendered as
/// `kind@node`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraceReport {
    pub kind: String,
    pub items: Vec<String>,
}

/// The trace log. Each recorded trace remembers its node and its kind, in
/// record order, so a filter query can replay only the traces of one kind.
#[derive(Clone, Debug, Default)]
pub struct TraceLog {
    traces: Vec<(String, String)>,
}

impl TraceLog {
    /// Create an empty trace log.
    pub fn new() -> Self {
        TraceLog { traces: Vec::new() }
    }

    /// Record a trace at `node` tagged with `kind`, in record order.
    pub fn record(&mut self, node: &str, kind: &str) {
        self.traces.push((node.to_string(), kind.to_string()));
    }

    /// Report, in record order, the traces tagged with `kind`, each rendered
    /// as `kind@node`.
    pub fn of_kind(&self, kind: &str) -> TraceReport {
        let mut items: Vec<String> = Vec::new();
        for (node, trace_kind) in &self.traces {
            if trace_kind == kind {
                items.push(format!("{}@{}", trace_kind, node));
            }
        }
        TraceReport {
            kind: kind.to_string(),
            items,
        }
    }
}
