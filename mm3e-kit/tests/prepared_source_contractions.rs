use mm3e_kit::{
    mesh_topology::PreparedTopology,
    surface_intersections::{validate_source_contraction_counted, PreparedSourceContractions, SourceContractionReport},
};
use std::collections::BTreeSet;
type Bounds = [[f64; 3]; 2];
const WORK: usize = 200_000_000;
fn octahedron() -> (Vec<Bounds>, Vec<[u32; 3]>) {
    let p = [[1., 0., 0.], [-1., 0., 0.], [0., 1., 0.], [0., -1., 0.], [0., 0., 1.], [0., 0., -1.]];
    (
        p.map(|p| [p, p]).to_vec(),
        vec![[0, 2, 4], [2, 1, 4], [1, 3, 4], [3, 0, 4], [2, 0, 5], [1, 2, 5], [3, 1, 5], [0, 3, 5]],
    )
}
fn same_report(a: &SourceContractionReport, b: &SourceContractionReport) {
    let (mut a, mut b) = (a.clone(), b.clone());
    a.work = 0;
    b.work = 0;
    assert_eq!(a, b);
}
fn capture(csv: &str) -> (Vec<Bounds>, Vec<[u32; 3]>) {
    let mut bounds = Vec::new();
    let mut triangles = Vec::new();
    for line in csv.lines().filter(|s| !s.starts_with('#')) {
        let v: Vec<_> = line.split(',').collect();
        if v[0] == "v" {
            let p: Vec<f64> = v[2..].iter().map(|s| s.parse().unwrap()).collect();
            bounds.push([[p[0], p[1], p[2]], [p[3], p[4], p[5]]]);
        } else if v[0] == "t" {
            triangles.push([v[1].parse().unwrap(), v[2].parse().unwrap(), v[3].parse().unwrap()]);
        }
    }
    (bounds, triangles)
}
#[test]
fn complete_captured_source_proposals_match_uncached_counts_in_both_directions() {
    for (csv, edge) in [
        (include_str!("fixtures/source_contraction_captured/flat.csv"), (0, 3)),
        (include_str!("fixtures/source_contraction_captured/folded.csv"), (410, 692)),
    ] {
        let (bounds, triangles) = capture(csv);
        let prepared = PreparedSourceContractions::new(&bounds, &triangles, WORK).unwrap();
        for (a, b) in [edge, (edge.1, edge.0)] {
            let old = validate_source_contraction_counted(&bounds, &triangles, a, b, WORK).unwrap();
            let new = prepared.check_contraction(a, b, WORK).unwrap();
            same_report(new.report(), &old);
            assert!(new.report().work < old.work);
        }
    }
}
#[test]
fn prepared_source_owns_inputs_rejects_stale_foreign_tickets_and_reports_stable_rows() {
    let (mut bounds, mut triangles) = octahedron();
    let mut prepared = PreparedSourceContractions::new(&bounds, &triangles, WORK).unwrap();
    let initial = prepared.snapshot(WORK).unwrap();
    let foreign = PreparedSourceContractions::new(&bounds, &triangles, WORK).unwrap();
    bounds.clear();
    triangles.clear();
    let failure = prepared.commit_contraction(foreign.check_contraction(0, 2, WORK).unwrap(), WORK).unwrap_err();
    assert!(failure.message.contains("another index") && failure.work > 0);
    assert_eq!(prepared.snapshot(WORK).unwrap(), initial);
    let ticket = prepared.check_contraction(0, 2, WORK).unwrap();
    let stale = prepared.check_contraction(2, 0, WORK).unwrap();
    let committed = prepared.commit_contraction(ticket, WORK).unwrap();
    let after = prepared.snapshot(WORK).unwrap();
    assert_eq!(committed.removed_faces.len(), 2);
    assert_eq!(committed.updated_faces.len(), 2);
    assert!(committed.removed_faces.windows(2).all(|w| w[0] < w[1]));
    assert!(committed.updated_faces.windows(2).all(|w| w[0].0 < w[1].0));
    assert_eq!(after.revision, 1);
    for (&row, &t) in after.original_face_indices.iter().zip(&after.triangles) {
        assert_eq!(t, initial.triangles[row].map(|i| if i == 0 { 2 } else { i }));
    }
    let failed = prepared.commit_contraction(stale, WORK).unwrap_err();
    assert!(failed.work > 0 && failed.message.contains("stale"));
    assert_eq!(prepared.snapshot(WORK).unwrap(), after);
    let mut copy = after;
    copy.triangles.clear();
    copy.original_face_indices.clear();
    assert_eq!(prepared.snapshot(WORK).unwrap().triangles.len(), 6);
    assert!(prepared.check_contraction(0, 2, WORK).unwrap_err().message.contains("source edge"));
}
#[test]
fn setup_query_commit_and_snapshot_budgets_replay_and_fail_atomically() {
    let (bounds, triangles) = octahedron();
    let prepared = PreparedSourceContractions::new(&bounds, &triangles, WORK).unwrap();
    let setup = prepared.preparation_work();
    assert_eq!(PreparedSourceContractions::new(&bounds, &triangles, setup).unwrap().preparation_work(), setup);
    assert!(PreparedSourceContractions::new(&bounds, &triangles, setup - 1).unwrap_err().message.contains("budget"));
    let snapshot = prepared.snapshot(WORK).unwrap();
    assert_eq!(prepared.snapshot(snapshot.work).unwrap(), snapshot);
    assert!(prepared.snapshot(snapshot.work - 1).unwrap_err().message.contains("budget"));
    let query = prepared.check_contraction(0, 2, WORK).unwrap();
    let qwork = query.report().work;
    assert_eq!(prepared.check_contraction(0, 2, qwork).unwrap().report(), query.report());
    let failure = prepared.check_contraction(0, 2, qwork - 1).unwrap_err();
    assert!(failure.message.contains("budget") && failure.work < qwork && failure.work > 0);
    let mut trial = PreparedSourceContractions::new(&bounds, &triangles, WORK).unwrap();
    let ticket = trial.check_contraction(0, 2, WORK).unwrap();
    let committed = trial.commit_contraction(ticket, WORK).unwrap();
    for allowance in [0, 1, committed.work / 2, committed.work - 1] {
        let mut current = PreparedSourceContractions::new(&bounds, &triangles, WORK).unwrap();
        let ticket = current.check_contraction(0, 2, WORK).unwrap();
        let failed = current.commit_contraction(ticket, allowance).unwrap_err();
        assert!(failed.message.contains("budget") && failed.work <= allowance);
        assert_eq!(current.snapshot(WORK).unwrap(), snapshot);
        for t in &triangles {
            let old = validate_source_contraction_counted(&bounds, &triangles, t[0], t[1], WORK);
            let new = current.check_contraction(t[0], t[1], WORK);
            assert_eq!(new.is_ok(), old.is_ok());
            if let (Ok(a), Ok(b)) = (old, new) {
                same_report(&a, b.report());
            }
        }
        let ticket = current.check_contraction(0, 2, WORK).unwrap();
        assert_eq!(current.commit_contraction(ticket, committed.work).unwrap(), committed);
        assert_eq!(current.snapshot(WORK).unwrap(), trial.snapshot(WORK).unwrap());
    }
}
#[test]
fn malformed_enclosures_indices_edges_and_rejected_source_proposals_keep_work() {
    let (bounds, triangles) = octahedron();
    assert!(PreparedSourceContractions::new(&[], &triangles, WORK).is_err());
    assert!(PreparedSourceContractions::new(&bounds, &[], WORK).is_err());
    for value in [f64::NAN, f64::INFINITY, f64::from(f32::MAX) * 3.] {
        let mut bad = bounds.clone();
        bad[0][0][0] = value;
        assert!(PreparedSourceContractions::new(&bad, &triangles, WORK)
            .unwrap_err()
            .message
            .contains("finite ordered"));
    }
    let mut bad = bounds.clone();
    bad[0][0][0] = 2.;
    assert!(PreparedSourceContractions::new(&bad, &triangles, WORK).is_err());
    for t in [[0, 0, 1], [0, 1, 99], [0, 1, u32::MAX]] {
        assert!(PreparedSourceContractions::new(&bounds, &[t], WORK).unwrap_err().message.contains("indices"));
    }
    let prepared = PreparedSourceContractions::new(&bounds, &triangles, WORK).unwrap();
    for (a, b) in [(0, 0), (0, 1), (0, 99), (99, 0)] {
        let failed = prepared.check_contraction(a, b, WORK).unwrap_err();
        assert!(failed.work > 0);
        assert_eq!(prepared.check_contraction(a, b, failed.work).unwrap_err(), failed);
        assert!(prepared.check_contraction(a, b, failed.work - 1).unwrap_err().message.contains("budget"));
    }
    // Widening the fixed enclosures makes the same true shape unprovable; a
    // query must reject exactly as the uncached sufficient interval checker.
    let uncertain: Vec<_> = bounds.iter().map(|p| [p[0].map(|v| v - 0.5), p[1].map(|v| v + 0.5)]).collect();
    let prepared = PreparedSourceContractions::new(&uncertain, &triangles, WORK).unwrap();
    let old = validate_source_contraction_counted(&uncertain, &triangles, 0, 2, WORK).unwrap_err();
    let new = prepared.check_contraction(0, 2, WORK).unwrap_err();
    assert!(new.work > 0 && new.message.contains("cannot certify"));
    assert_eq!(new.message, old.message);
    let allowance = 2 * new.work - 1;
    let second = prepared.check_contraction(0, 2, allowance - new.work).unwrap_err();
    assert!(second.message.contains("budget") && new.work + second.work <= allowance);
}
#[test]
fn many_sequential_captured_collapses_match_every_uncached_gate_and_count() {
    let (bounds, triangles) = capture(include_str!("fixtures/source_contraction_captured/flat.csv"));
    let mut source = PreparedSourceContractions::new(&bounds, &triangles, WORK).unwrap();
    let mut topology = PreparedTopology::new(&triangles, WORK).unwrap();
    let mut cached_work = source.preparation_work();
    let mut uncached_work = 0;
    let mut attempts = 0;
    for step in 0..20 {
        let current = source.snapshot(WORK).unwrap();
        let current_topology = topology.snapshot(WORK).unwrap();
        assert_eq!(current.triangles, current_topology.triangles);
        assert_eq!(current.original_face_indices, current_topology.original_face_indices);
        let mut edges = BTreeSet::new();
        for &[a, b, c] in &current.triangles {
            for (a, b) in [(a, b), (b, c), (c, a)] {
                edges.insert((a.min(b), a.max(b)));
            }
        }
        let distance = |(a, b): (u32, u32)| {
            (0..3)
                .map(|axis| {
                    let d = bounds[a as usize][0][axis] - bounds[b as usize][0][axis];
                    d * d
                })
                .sum::<f64>()
        };
        let mut edges: Vec<_> = edges.into_iter().collect();
        edges.sort_by(|&a, &b| distance(a).total_cmp(&distance(b)).then(a.cmp(&b)));
        let mut selected = None;
        for (a, b) in edges.into_iter().take(512) {
            for (a, b) in [(a, b), (b, a)] {
                let Ok(topology_ticket) = topology.check_collapse(a, b, WORK) else { continue };
                let old = validate_source_contraction_counted(&bounds, &current.triangles, a, b, WORK);
                let new = source.check_contraction(a, b, WORK);
                attempts += 1;
                assert_eq!(new.is_ok(), old.is_ok(), "step{step} edge{a}->{b} old={old:?} new={new:?}");
                match (old, new) {
                    (Ok(old), Ok(new)) => {
                        uncached_work += old.work;
                        cached_work += new.report().work;
                        same_report(&old, new.report());
                        selected = Some((topology_ticket, new));
                        break;
                    }
                    (Err(old), Err(new)) => {
                        uncached_work += old.work;
                        cached_work += new.work;
                    }
                    _ => unreachable!(),
                }
            }
            if selected.is_some() {
                break;
            }
        }
        let (topology_ticket, source_ticket) = selected.expect("captured source must admit20 endpoint contractions");
        let a = topology.commit_collapse(topology_ticket, WORK).unwrap();
        let b = source.commit_contraction(source_ticket, WORK).unwrap();
        cached_work += b.work;
        assert_eq!(a.removed_faces, b.removed_faces);
        assert_eq!(a.updated_faces, b.updated_faces);
        assert_eq!(b.revision, step + 1);
    }
    let a = topology.snapshot(WORK).unwrap();
    let b = source.snapshot(WORK).unwrap();
    assert_eq!(a.triangles, b.triangles);
    assert_eq!(a.original_face_indices, b.original_face_indices);
    assert!(cached_work < uncached_work, "prepared {cached_work} versus uncached {uncached_work}");
    eprintln!(
        "PREPARED_SOURCE20 attempts={attempts} setup_checks_commits={cached_work} uncached_checks={uncached_work}"
    );
}

#[test]
fn every_mapped_changed_pair_is_checked_even_when_both_old_leaves_are_skipped() {
    // The baseline deliberately contains an unresolved duplicate-coordinate
    // patch. Preparation does not certify baseline embedding, and a proposal
    // cannot silently preserve its changed-vs-changed forbidden final contact.
    let p = [[0., 0., 0.], [10., 0., 0.], [0., 2., 0.], [0., 1., 0.], [0., 0., 1.], [0., 1., 0.], [0., 0., 1.]];
    let bounds = p.map(|p| [p, p]);
    let triangles = [[0, 1, 2], [0, 3, 4], [0, 5, 6]];
    let prepared = PreparedSourceContractions::new(&bounds, &triangles, WORK).unwrap();
    let old = validate_source_contraction_counted(&bounds, &triangles, 0, 1, WORK).unwrap_err();
    let new = prepared.check_contraction(0, 1, WORK).unwrap_err();
    assert!(old.message.contains("final embedding"));
    assert_eq!(new.message, old.message);
    assert!(new.work > 0);
}

#[test]
fn nonadjacent_stationary_component_inside_a_sweep_remains_a_rejection() {
    let (mut bounds, mut triangles) = octahedron();
    let base = bounds.len() as u32;
    for delta in [[0.02, 0.02, 0.02], [-0.02, -0.02, 0.02], [-0.02, 0.02, -0.02], [0.02, -0.02, -0.02]] {
        let p = [0.2 + delta[0], delta[1], 0.1 + delta[2]];
        bounds.push([p, p]);
    }
    triangles.extend([[0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]].map(|t| t.map(|i| i + base)));
    let prepared = PreparedSourceContractions::new(&bounds, &triangles, WORK).unwrap();
    let old = validate_source_contraction_counted(&bounds, &triangles, 0, 2, WORK).unwrap_err();
    let new = prepared.check_contraction(0, 2, WORK).unwrap_err();
    assert!(new.message.contains("nonadjacent swept"));
    assert_eq!(old.message, new.message);
    assert!(new.work > 0);
}
