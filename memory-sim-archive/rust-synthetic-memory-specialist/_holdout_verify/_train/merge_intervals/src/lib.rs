// Ordo app: collect integer [start,end] ranges, then merge the ones that
// overlap or touch into the fewest covering ranges, reported low-to-high.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

// Private helper: one captured range, in insertion order. Normalized so that
// start <= end at the boundary.
struct Range {
    start: i32,
    end: i32,
}

pub struct MergeIntervals {
    ranges: Vec<Range>,
}

impl MergeIntervals {
    pub fn new() -> Self {
        MergeIntervals { ranges: Vec::new() }
    }

    // Mutating setup method returns () so the private Range type stays private.
    // Records one range; a flipped pair is normalized so start <= end.
    pub fn add(&mut self, start: i32, end: i32) {
        let (lo, hi) = if start <= end { (start, end) } else { (end, start) };
        self.ranges.push(Range { start: lo, end: hi });
    }

    // Query method returns the merged set as public plain-data Intervals.
    // Ranges are sorted by start (then end), then swept once: a range extends
    // the current run when it overlaps or is adjacent (next.start <= cur.end),
    // otherwise it begins a new run. Output is ordered low-to-high.
    pub fn merge(&self) -> Vec<Interval> {
        let mut sorted: Vec<&Range> = self.ranges.iter().collect();
        sorted.sort_by(|a, b| a.start.cmp(&b.start).then(a.end.cmp(&b.end)));
        let mut merged: Vec<Interval> = Vec::new();
        for range in sorted {
            match merged.last_mut() {
                Some(cur) if range.start <= cur.end => {
                    if range.end > cur.end {
                        cur.end = range.end;
                    }
                }
                _ => merged.push(Interval { start: range.start, end: range.end }),
            }
        }
        merged
    }
}
