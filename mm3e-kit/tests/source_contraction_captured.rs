//! Real pre-rounding source complexes. These are required admission challenges,
//! not substitutes for topology, adjacent-sweep, or final-f32 embedding checks.
use mm3e_kit::surface_intersections::validate_source_contraction;

type Bounds = [[f64; 3]; 2];
struct Capture {
    bounds: Vec<Bounds>,
    triangles: Vec<[u32; 3]>,
    original_ids: Vec<u32>,
}
fn capture(csv: &str) -> Capture {
    let mut result = Capture { bounds: Vec::new(), triangles: Vec::new(), original_ids: Vec::new() };
    for line in csv.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<_> = line.split(',').collect();
        match fields[0] {
            "v" => {
                assert_eq!(fields.len(), 8);
                result.original_ids.push(fields[1].parse().unwrap());
                let coordinates: Vec<f64> = fields[2..].iter().map(|s| s.parse().unwrap()).collect();
                let pair = [
                    [coordinates[0], coordinates[1], coordinates[2]],
                    [coordinates[3], coordinates[4], coordinates[5]],
                ];
                for (lo, hi) in pair[0].into_iter().zip(pair[1]) {
                    assert!(lo.is_finite() && hi.is_finite() && lo <= hi);
                    assert!(lo == hi || lo.next_up() == hi, "fixture must use the smallest binary64 bracket");
                }
                result.bounds.push(pair);
            }
            "t" => {
                assert_eq!(fields.len(), 4);
                result.triangles.push([
                    fields[1].parse().unwrap(),
                    fields[2].parse().unwrap(),
                    fields[3].parse().unwrap(),
                ]);
            }
            other => panic!("unexpected fixture record {other:?}"),
        }
    }
    result
}
fn challenge(
    csv: &str,
    dimensions: (usize, usize),
    removed: u32,
    retained: u32,
    exact_swept: usize,
    exact_final: usize,
) {
    let c = capture(csv);
    assert_eq!((c.bounds.len(), c.triangles.len()), dimensions);
    let local = |id| c.original_ids.iter().position(|&v| v == id).unwrap() as u32;
    let (a, b) = (local(removed), local(retained));
    assert_eq!(c.triangles.iter().filter(|t| t.contains(&a) && t.contains(&b)).count(), 2);
    // Independent Fraction clipping of the identical frozen triangulation found
    // no prohibited intersections in these candidate sets. Interval rejection
    // is conservative, but remains an unresolved admission challenge here.
    let result = validate_source_contraction(&c.bounds, &c.triangles, a, b, 200_000_000);
    eprintln!("CAPTURED_SOURCE original_edge={removed}->{retained} local_edge={a}->{b} exact_oracle_swept={exact_swept} exact_oracle_final={exact_final} result={result:?}");
    let report = result.unwrap_or_else(|e| panic!("original edge {removed}->{retained} was not admitted: {e}"));
    assert!(!report.adjacent_sweeps_certified);
    assert_eq!(report.source_orientations, c.triangles.iter().filter(|t| t.contains(&a) && !t.contains(&b)).count());
    assert!(report.swept_pairs >= exact_swept);
    assert!(report.final_pairs >= exact_final);
    assert_eq!(validate_source_contraction(&c.bounds, &c.triangles, a, b, report.work).unwrap(), report);
    let error = validate_source_contraction(&c.bounds, &c.triangles, a, b, report.work - 1).unwrap_err();
    assert!(error.contains("budget"), "{error}");
}

const FLAT: &str = include_str!("fixtures/source_contraction_captured/flat.csv");
const FOLDED: &str = include_str!("fixtures/source_contraction_captured/folded.csv");

#[test]
fn complete_flat_source_zero_to_five_matches_exact_intersection_oracle() {
    challenge(FLAT, (2511, 5018), 0, 5, 21, 64);
}
#[test]
fn complete_flat_source_five_to_zero_matches_exact_intersection_oracle() {
    challenge(FLAT, (2511, 5018), 5, 0, 36, 93);
}
#[test]
fn complete_folded_source_510_to_846_matches_exact_intersection_oracle() {
    challenge(FOLDED, (1946, 3888), 510, 846, 1, 53);
}
#[test]
fn complete_folded_source_846_to_510_matches_exact_intersection_oracle() {
    challenge(FOLDED, (1946, 3888), 846, 510, 9, 61);
}
