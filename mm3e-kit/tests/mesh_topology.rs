use mm3e_kit::{
    mesh_topology::{can_collapse, validate},
    meshing::MAX_MESH_VERTICES,
};

const TETRAHEDRON: [[u32; 3]; 4] = [[0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]];
const OCTAHEDRON: [[u32; 3]; 8] =
    [[0, 2, 3], [0, 3, 4], [0, 4, 5], [0, 5, 2], [1, 3, 2], [1, 4, 3], [1, 5, 4], [1, 2, 5]];
const WORK: usize = 1_000_000;

#[test]
fn closed_oriented_components_and_sparse_ids_are_counted_without_coordinate_assumptions() {
    let report = validate(&TETRAHEDRON, WORK).unwrap();
    assert_eq!((report.components, report.vertices, report.edges, report.triangles), (1, 4, 6, 4));
    let mut two = TETRAHEDRON.to_vec();
    two.extend(TETRAHEDRON.map(|t| t.map(|i| i + 4)));
    let report = validate(&two, WORK).unwrap();
    assert_eq!((report.components, report.vertices, report.edges, report.triangles), (2, 8, 12, 8));
    // Reversing every face preserves a coherent orientation. Duplicate opposite
    // faces, tested below, are a different invalid complex.
    let reversed = two.iter().map(|&[a, b, c]| [a, c, b]).collect::<Vec<_>>();
    assert_eq!(validate(&reversed, WORK).unwrap().components, 2);
    let sparse = TETRAHEDRON.map(|t| t.map(|i| MAX_MESH_VERTICES as u32 - 1 - i * 7));
    assert_eq!(validate(&sparse, WORK).unwrap().vertices, 4);
}

#[test]
fn a_tetrahedron_edge_fails_the_full_link_despite_exactly_two_common_neighbors() {
    // For edge0–1 the common link vertices are {2,3}, matching the two edge
    // faces. But both vertex links ALSO contain edge(2,3); the edge link does
    // not. A neighbor-count-only check would admit duplicate surviving faces.
    for (a, b) in [(0, 1), (1, 0)] {
        let failure = can_collapse(&TETRAHEDRON, a, b, WORK).unwrap_err();
        assert!(failure.message.contains("common link edge"), "{}", failure.message);
        assert!(failure.work > 0);
    }
    let invalid_image = [[1, 3, 2], [1, 2, 3]];
    assert!(validate(&invalid_image, WORK).unwrap_err().message.contains("duplicate face"));
}

#[test]
fn extra_common_link_vertices_are_rejected_on_a_closed_periodic_torus() {
    let id = |x: u32, y: u32| (y % 3) * 3 + x % 3;
    let mut torus = Vec::new();
    for y in 0..3 {
        for x in 0..3 {
            torus.push([id(x, y), id(x + 1, y), id(x + 1, y + 1)]);
            torus.push([id(x, y), id(x + 1, y + 1), id(x, y + 1)]);
        }
    }
    let report = validate(&torus, WORK).unwrap();
    assert_eq!((report.components, report.vertices, report.edges, report.triangles), (1, 9, 27, 18));
    let failure = can_collapse(&torus, 0, 1, WORK).unwrap_err();
    assert!(failure.message.contains("common link vertices"), "{}", failure.message);
}

#[test]
fn octahedron_endpoint_contractions_preserve_the_valid_complex_in_both_directions() {
    for (removed, retained) in [(0, 2), (2, 0)] {
        let report = can_collapse(&OCTAHEDRON, removed, retained, WORK).unwrap();
        assert_eq!(
            (report.components, report.edge_faces, report.common_link_vertices, report.common_link_edges),
            (1, 2, 2, 0)
        );
        let image: Vec<_> = OCTAHEDRON
            .iter()
            .filter(|t| !(t.contains(&removed) && t.contains(&retained)))
            .map(|t| t.map(|v| if v == removed { retained } else { v }))
            .collect();
        let after = validate(&image, WORK).unwrap();
        assert_eq!((after.components, after.vertices, after.edges, after.triangles), (1, 5, 9, 6));
    }
    let mut disconnected = OCTAHEDRON.to_vec();
    disconnected.extend(TETRAHEDRON.map(|t| t.map(|i| i + 6)));
    assert_eq!(can_collapse(&disconnected, 0, 2, WORK).unwrap().components, 2);
}

#[test]
fn duplicate_reversed_open_inconsistently_oriented_and_overfull_faces_are_rejected() {
    for duplicate in [[0, 2, 1], [0, 1, 2]] {
        let mut triangles = TETRAHEDRON.to_vec();
        triangles.push(duplicate);
        assert!(validate(&triangles, WORK).unwrap_err().message.contains("duplicate face"));
    }
    assert!(validate(&TETRAHEDRON[..3], WORK).unwrap_err().message.contains("is open"));
    let mut flipped = TETRAHEDRON;
    flipped[0].swap(0, 1);
    assert!(validate(&flipped, WORK).unwrap_err().message.contains("inconsistent face orientation"));
    let mut three_faces = TETRAHEDRON.to_vec();
    three_faces.push([0, 1, 4]);
    assert!(validate(&three_faces, WORK).unwrap_err().message.contains("more than two"));
}

#[test]
fn two_closed_shells_pinched_at_one_vertex_fail_the_single_cycle_link_gate() {
    let mut pinched = TETRAHEDRON.to_vec();
    pinched.extend(TETRAHEDRON.map(|t| t.map(|i| if i == 0 { 0 } else { i + 3 })));
    let failure = validate(&pinched, WORK).unwrap_err();
    assert!(failure.message.contains("disconnected link (vertex pinch)"), "{}", failure.message);
    assert!(failure.work > 0);
    // Collapse admission does not assume a separate caller already validated
    // the rest of the source complex.
    assert!(can_collapse(&pinched, 1, 2, WORK).unwrap_err().message.contains("vertex pinch"));
}

#[test]
fn malformed_indices_degenerate_faces_and_absent_source_edges_fail_explicitly() {
    assert!(validate(&[], WORK).unwrap_err().message.contains("nonempty"));
    assert!(validate(&[[0, 0, 1]], WORK).unwrap_err().message.contains("repeats a vertex"));
    assert!(validate(&[[0, 1, MAX_MESH_VERTICES as u32]], WORK).unwrap_err().message.contains("index exceeds"));
    assert!(can_collapse(&OCTAHEDRON, 0, 0, WORK).unwrap_err().message.contains("distinct"));
    assert!(can_collapse(&OCTAHEDRON, 0, u32::MAX, WORK).unwrap_err().message.contains("bounded"));
    let absent = can_collapse(&OCTAHEDRON, 0, 1, WORK).unwrap_err();
    assert!(absent.message.contains("do not form a source edge"));
    assert!(absent.work > 0);
}

#[test]
fn successful_topology_and_link_budgets_replay_exactly_and_one_less_fails() {
    let topology = validate(&OCTAHEDRON, WORK).unwrap();
    assert_eq!(validate(&OCTAHEDRON, topology.work).unwrap(), topology);
    let failure = validate(&OCTAHEDRON, topology.work - 1).unwrap_err();
    assert!(failure.message.contains("budget") && failure.work < topology.work);
    let collapse = can_collapse(&OCTAHEDRON, 0, 2, WORK).unwrap();
    assert_eq!(can_collapse(&OCTAHEDRON, 0, 2, collapse.work).unwrap(), collapse);
    let failure = can_collapse(&OCTAHEDRON, 0, 2, collapse.work - 1).unwrap_err();
    assert!(failure.message.contains("budget") && failure.work < collapse.work);
    assert_eq!(validate(&OCTAHEDRON, 0).unwrap_err().work, 0);
}

#[test]
fn rejected_topology_and_full_link_attempts_keep_spent_work_for_retries() {
    let mut pinched = TETRAHEDRON.to_vec();
    pinched.extend(TETRAHEDRON.map(|t| t.map(|i| if i == 0 { 0 } else { i + 3 })));
    let failed_validation = validate(&pinched, WORK).unwrap_err();
    assert_eq!(validate(&pinched, failed_validation.work).unwrap_err(), failed_validation);
    let short = validate(&pinched, failed_validation.work - 1).unwrap_err();
    assert!(short.message.contains("budget") && short.work < failed_validation.work);
    let rejected = can_collapse(&TETRAHEDRON, 0, 1, WORK).unwrap_err();
    assert!(rejected.work > 0 && rejected.message.contains("common link edge"));
    assert_eq!(can_collapse(&TETRAHEDRON, 0, 1, rejected.work).unwrap_err(), rejected);
    let allowance = rejected.work * 2 - 1;
    let second = can_collapse(&TETRAHEDRON, 0, 1, allowance - rejected.work).unwrap_err();
    assert!(second.message.contains("budget"));
    assert!(second.work > 0 && second.work + rejected.work <= allowance);
}

#[test]
fn both_captured_source_edges_pass_full_links_on_the_complete_real_complexes() {
    for (csv, expected, edge) in [
        (include_str!("fixtures/source_contraction_captured/flat.csv"), (2511, 5018), (0, 3)),
        (include_str!("fixtures/source_contraction_captured/folded.csv"), (1946, 3888), (410, 692)),
    ] {
        let triangles: Vec<[u32; 3]> = csv
            .lines()
            .filter_map(|line| line.strip_prefix("t,"))
            .map(|line| {
                let ids = line.split(',').map(|v| v.parse().unwrap()).collect::<Vec<_>>();
                [ids[0], ids[1], ids[2]]
            })
            .collect();
        let report = validate(&triangles, WORK).unwrap();
        assert_eq!((report.vertices, report.triangles), expected);
        assert_eq!(report.components, 1);
        for (a, b) in [edge, (edge.1, edge.0)] {
            let admission = can_collapse(&triangles, a, b, WORK).unwrap();
            assert_eq!((admission.edge_faces, admission.common_link_vertices, admission.common_link_edges), (2, 2, 0));
        }
    }
}

#[test]
fn prepared_topology_rejects_original_tetra_torus_and_pinched_controls() {
    use mm3e_kit::mesh_topology::PreparedTopology;
    let tetra = PreparedTopology::new(&TETRAHEDRON, WORK).unwrap();
    assert!(tetra.check_collapse(0, 1, WORK).unwrap_err().message.contains("common link edge"));
    let id = |x: u32, y: u32| (y % 3) * 3 + x % 3;
    let mut torus = Vec::new();
    for y in 0..3 {
        for x in 0..3 {
            torus.extend([[id(x, y), id(x + 1, y), id(x + 1, y + 1)], [id(x, y), id(x + 1, y + 1), id(x, y + 1)]]);
        }
    }
    let prepared = PreparedTopology::new(&torus, WORK).unwrap();
    assert!(prepared.check_collapse(0, 1, WORK).unwrap_err().message.contains("common link vertices"));
    let mut pinched = TETRAHEDRON.to_vec();
    pinched.extend(TETRAHEDRON.map(|t| t.map(|i| if i == 0 { 0 } else { i + 3 })));
    assert!(PreparedTopology::new(&pinched, WORK).unwrap_err().message.contains("disconnected link"));
}

#[test]
fn prepared_contractions_are_atomic_reject_stale_foreign_tickets_and_keep_owned_input() {
    use mm3e_kit::mesh_topology::PreparedTopology;
    let mut input = OCTAHEDRON.to_vec();
    let mut prepared = PreparedTopology::new(&input, WORK).unwrap();
    let initial = prepared.snapshot(WORK).unwrap();
    input.clear();
    let mut altered_snapshot = prepared.snapshot(WORK).unwrap();
    altered_snapshot.triangles.clear();
    assert_eq!(prepared.snapshot(WORK).unwrap(), initial);
    let foreign = PreparedTopology::new(&OCTAHEDRON, WORK).unwrap();
    let error = prepared.commit_collapse(foreign.check_collapse(0, 2, WORK).unwrap(), WORK).unwrap_err();
    assert!(error.work > 0 && error.message.contains("another complex"));
    assert_eq!(prepared.snapshot(WORK).unwrap(), initial);
    let first = prepared.check_collapse(0, 2, WORK).unwrap();
    let stale = prepared.check_collapse(2, 0, WORK).unwrap();
    let report = prepared.commit_collapse(first, WORK).unwrap();
    assert_eq!(report.removed_faces.len(), 2);
    assert_eq!(report.updated_faces.len(), 2);
    let after = prepared.snapshot(WORK).unwrap();
    let error = prepared.commit_collapse(stale, WORK).unwrap_err();
    assert!(error.work > 0 && error.message.contains("stale"));
    assert_eq!(prepared.snapshot(WORK).unwrap(), after);
    assert_eq!(validate(&after.triangles, WORK).unwrap().components, 1);
    assert!(prepared.check_collapse(0, 2, WORK).unwrap_err().message.contains("current source edge"));
    for (row, triangle) in after.original_face_indices.iter().zip(&after.triangles) {
        assert_eq!(*triangle, OCTAHEDRON[*row].map(|i| if i == 0 { 2 } else { i }));
    }
}

#[test]
fn prepared_sequential_collapses_match_independent_rebuilds_and_reduce_real_work() {
    use mm3e_kit::mesh_topology::PreparedTopology;
    let csv = include_str!("fixtures/source_contraction_captured/flat.csv");
    let original: Vec<[u32; 3]> = csv
        .lines()
        .filter_map(|line| line.strip_prefix("t,"))
        .map(|line| {
            let ids: Vec<_> = line.split(',').map(|v| v.parse().unwrap()).collect();
            [ids[0], ids[1], ids[2]]
        })
        .collect();
    let mut prepared = PreparedTopology::new(&original, WORK).unwrap();
    let mut expected: Vec<_> = original.iter().copied().enumerate().collect();
    let mut prepared_work = prepared.initial_report().work;
    let mut repeated_work = 0;
    for step in 0..20 {
        let snapshot = prepared.snapshot(WORK).unwrap();
        assert_eq!(snapshot.triangles, expected.iter().map(|(_, t)| *t).collect::<Vec<_>>());
        let mut selected = None;
        for triangle in &snapshot.triangles {
            for (removed, retained) in
                [(triangle[0], triangle[1]), (triangle[1], triangle[2]), (triangle[2], triangle[0])]
            {
                match prepared.check_collapse(removed, retained, WORK) {
                    Ok(ticket) => {
                        selected = Some((removed, retained, ticket));
                        break;
                    }
                    Err(error) => {
                        prepared_work += error.work;
                    }
                }
            }
            if selected.is_some() {
                break;
            }
        }
        let (removed, retained, ticket) = selected.expect("captured complex must support multiple valid contractions");
        let rebuilt = can_collapse(&snapshot.triangles, removed, retained, WORK).unwrap();
        repeated_work += rebuilt.work;
        assert_eq!(ticket.report().components, rebuilt.components);
        assert_eq!(ticket.report().common_link_vertices, rebuilt.common_link_vertices);
        assert_eq!(ticket.report().common_link_edges, rebuilt.common_link_edges);
        prepared_work += ticket.report().work;
        let committed = prepared.commit_collapse(ticket, WORK).unwrap();
        prepared_work += committed.work;
        expected.retain(|(_, t)| !(t.contains(&removed) && t.contains(&retained)));
        for (_, triangle) in &mut expected {
            *triangle = triangle.map(|i| if i == removed { retained } else { i });
        }
        let current = prepared.snapshot(WORK).unwrap();
        assert_eq!(current.revision, step + 1);
        assert_eq!(current.original_face_indices, expected.iter().map(|(row, _)| *row).collect::<Vec<_>>());
        assert_eq!(current.triangles, expected.iter().map(|(_, t)| *t).collect::<Vec<_>>());
        let checked = validate(&current.triangles, WORK).unwrap();
        assert_eq!(checked.components, 1);
        assert_eq!(checked.triangles, original.len() - 2 * (step as usize + 1));
    }
    eprintln!("PREPARED_TOPOLOGY 20 contractions initial_plus_checks_and_commits={prepared_work} repeated_whole_checks={repeated_work}");
    assert!(prepared_work * 3 < repeated_work, "prepared work {prepared_work} versus repeated {repeated_work}");
}

#[test]
fn prepared_work_budgets_replay_and_failed_commit_preserves_all_state() {
    use mm3e_kit::mesh_topology::PreparedTopology;
    let prepared = PreparedTopology::new(&OCTAHEDRON, WORK).unwrap();
    let initial_work = prepared.initial_report().work;
    assert_eq!(PreparedTopology::new(&OCTAHEDRON, initial_work).unwrap().initial_report().work, initial_work);
    assert!(PreparedTopology::new(&OCTAHEDRON, initial_work - 1).unwrap_err().message.contains("budget"));
    let snapshot = prepared.snapshot(WORK).unwrap();
    assert_eq!(prepared.snapshot(snapshot.work).unwrap(), snapshot);
    assert!(prepared.snapshot(snapshot.work - 1).unwrap_err().message.contains("budget"));
    let admission = prepared.check_collapse(0, 2, WORK).unwrap();
    let check_work = admission.report().work;
    assert_eq!(prepared.check_collapse(0, 2, check_work).unwrap().report(), admission.report());
    assert!(prepared.check_collapse(0, 2, check_work - 1).unwrap_err().message.contains("budget"));
    let mut trial = PreparedTopology::new(&OCTAHEDRON, WORK).unwrap();
    let ticket = trial.check_collapse(0, 2, WORK).unwrap();
    let successful = trial.commit_collapse(ticket, WORK).unwrap();
    for allowance in [0, 1, successful.work / 2, successful.work - 1] {
        let mut current = PreparedTopology::new(&OCTAHEDRON, WORK).unwrap();
        let ticket = current.check_collapse(0, 2, WORK).unwrap();
        let failure = current.commit_collapse(ticket, allowance).unwrap_err();
        assert!(failure.message.contains("budget"));
        assert!(failure.work <= allowance);
        assert_eq!(current.snapshot(WORK).unwrap(), snapshot);
        // Every edge decision must still agree with a fresh proof after failure.
        for t in OCTAHEDRON {
            assert_eq!(
                current.check_collapse(t[0], t[1], WORK).is_ok(),
                can_collapse(&OCTAHEDRON, t[0], t[1], WORK).is_ok()
            );
        }
        let ticket = current.check_collapse(0, 2, WORK).unwrap();
        assert_eq!(current.commit_collapse(ticket, successful.work).unwrap(), successful);
        assert_eq!(current.snapshot(WORK).unwrap(), trial.snapshot(WORK).unwrap());
    }
    let first = prepared.check_collapse(0, 0, WORK).unwrap_err();
    assert!(first.work > 0);
    assert_eq!(prepared.check_collapse(0, 0, first.work).unwrap_err(), first);
    assert!(prepared.check_collapse(0, 0, first.work - 1).unwrap_err().message.contains("budget"));
}

#[test]
fn prepared_all_endpoint_decisions_track_sequential_changes_in_separate_components() {
    use mm3e_kit::mesh_topology::PreparedTopology;
    let mut source = OCTAHEDRON.to_vec();
    source.extend(OCTAHEDRON.map(|t| t.map(|i| i + 6)));
    let mut prepared = PreparedTopology::new(&source, WORK).unwrap();
    for stage in 0..=4 {
        let snapshot = prepared.snapshot(WORK).unwrap();
        assert_eq!(validate(&snapshot.triangles, WORK).unwrap().components, 2);
        let mut next = None;
        // Compare every cached adjacency/link decision against an independent
        // full rebuild, including absent vertices and cross-component pairs.
        for removed in 0..12 {
            for retained in 0..12 {
                let cached = prepared.check_collapse(removed, retained, WORK);
                let rebuilt = can_collapse(&snapshot.triangles, removed, retained, WORK);
                assert_eq!(cached.is_ok(), rebuilt.is_ok(), "stage {stage}, edge {removed}->{retained}");
                if let (Ok(ticket), Ok(report)) = (cached, rebuilt) {
                    assert_eq!(ticket.report().components, report.components);
                    assert_eq!(ticket.report().edge_faces, report.edge_faces);
                    assert_eq!(ticket.report().common_link_vertices, report.common_link_vertices);
                    assert_eq!(ticket.report().common_link_edges, report.common_link_edges);
                    if next.is_none() {
                        next = Some(ticket);
                    }
                }
            }
        }
        if stage < 4 {
            prepared.commit_collapse(next.expect("two octahedra must contract to two tetrahedra"), WORK).unwrap();
        } else {
            assert!(next.is_none(), "the final tetrahedra have no full-link-admissible edge");
            assert_eq!(snapshot.triangles.len(), 8);
        }
    }
}
