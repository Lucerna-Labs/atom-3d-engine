use mm3e_kit::{meshing, Vec3};
#[path = "../src/rounding_constraints.rs"]
pub mod rounding_constraints;
use rounding_constraints::{solve, ComponentLimit, Limits, Status};

#[derive(Clone)]
struct Problem {
    positions: Vec<Vec3>,
    triangles: Vec<[u32; 3]>,
    normals: Vec<[f64; 3]>,
    candidates: Vec<Vec<Vec3>>,
}
fn limits() -> Limits {
    Limits { max_variables: 64, max_allowed_tuples: 50_000, max_decisions: 4096, max_work: 10_000_000 }
}
fn run(p: &Problem, seed: usize, l: Limits) -> rounding_constraints::Outcome {
    solve(&p.positions, &p.triangles, &p.normals, &p.candidates, seed, l).unwrap()
}
fn positive(p: &[Vec3], t: [u32; 3], n: [f64; 3]) -> bool {
    let [a, b, c] = t.map(|i| p[i as usize]);
    let (ab, ac) = (
        [f64::from(b.x) - f64::from(a.x), f64::from(b.y) - f64::from(a.y)],
        [f64::from(c.x) - f64::from(a.x), f64::from(c.y) - f64::from(a.y)],
    );
    (ab[0] * ac[1] - ab[1] * ac[0]) * n[2] > 0.
}
fn equality_chain() -> Problem {
    let positions = vec![
        Vec3::new(0., 0., 0.),
        Vec3::new(1., 0., 0.),
        Vec3::new(2., 0., 0.),
        Vec3::new(-100., 2., 0.),
        Vec3::new(100., 2., 0.),
        Vec3::new(-1., 0.5, 0.),
        Vec3::new(1., 0.5, 0.),
    ];
    let mut candidates: Vec<_> = positions.iter().map(|&p| vec![p]).collect();
    for i in 0..3 {
        candidates[i].push(Vec3::new(positions[i].x, 1., 0.));
    }
    let triangles = vec![[0, 5, 6], [0, 1, 3], [0, 1, 4], [1, 2, 3], [1, 2, 4]];
    Problem { normals: vec![[0., 0., 1.]; triangles.len()], positions, triangles, candidates }
}
fn exhaustive_binary_solutions(p: &Problem, variables: usize) -> Vec<Vec<Vec3>> {
    let mut solutions = Vec::new();
    for bits in 0..1usize << variables {
        let mut positions = p.positions.clone();
        for (i, point) in positions.iter_mut().enumerate().take(variables) {
            *point = p.candidates[i][(bits >> i) & 1];
        }
        if p.triangles.iter().zip(&p.normals).all(|(&t, &n)| positive(&positions, t, n)) {
            solutions.push(positions);
        }
    }
    solutions
}

#[test]
fn three_coupled_changes_are_solved_when_no_one_or_two_vertex_change_can_work() {
    let p = equality_chain();
    let original = p.clone();
    // The seed requires A=1. The four other oriented triangles impose A=B=C.
    let exhaustive = exhaustive_binary_solutions(&p, 3);
    assert_eq!(exhaustive.len(), 1);
    assert_eq!(exhaustive[0].iter().zip(&p.positions).filter(|(a, b)| a != b).count(), 3);
    let out = run(&p, 0, Limits { max_variables: 3, max_decisions: 0, ..limits() });
    assert_eq!(out.status, Status::Solved);
    assert_eq!((out.variables, out.participants, out.constraints, out.allowed_tuples, out.decisions), (3, 7, 5, 13, 0));
    assert_eq!(out.assignment.as_ref().unwrap(), &exhaustive[0]);
    assert_eq!(p.positions, original.positions);
    assert_eq!(p.candidates, original.candidates);
}

#[test]
fn stable_borders_and_fixed_shared_anchors_do_not_pull_in_unrelated_fragile_regions() {
    let mut p = equality_chain();
    // This independently fragile face shares only the two fixed seed anchors.
    let remote = p.positions.len() as u32;
    p.positions.push(Vec3::new(20., 0., 0.));
    p.candidates.push(vec![Vec3::new(20., 0., 0.), Vec3::new(20., 1., 0.)]);
    p.triangles.push([remote, 5, 6]);
    p.normals.push([0., 0., 1.]);
    // Every tuple of this C/S/T face is positive, so S/T cannot transmit choices.
    let s = p.positions.len() as u32;
    p.positions.push(Vec3::new(3., 10., 0.));
    p.candidates.push(vec![Vec3::new(3., 10., 0.), Vec3::new(3., 11., 0.)]);
    let t = p.positions.len() as u32;
    p.positions.push(Vec3::new(2., 20., 0.));
    p.candidates.push(vec![Vec3::new(2., 20., 0.), Vec3::new(2., 21., 0.)]);
    p.triangles.push([2, s, t]);
    p.normals.push([0., 0., 1.]);
    let out = run(&p, 0, Limits { max_variables: 3, ..limits() });
    assert_eq!(out.status, Status::Solved);
    assert_eq!(out.variables, 3);
    assert_eq!(out.participants, 7);
    assert_eq!(out.stable_faces, 1);
    let assignment = out.assignment.unwrap();
    assert_eq!(&assignment[7..], &p.positions[7..]);
    assert!(p.triangles[..5].iter().zip(&p.normals).all(|(&tri, &n)| positive(&assignment, tri, n)));
    assert!(positive(&assignment, [2, s, t], [0., 0., 1.]));
    assert!(!positive(&assignment, [remote, 5, 6], [0., 0., 1.]));
    let stable_seed = run(
        &p,
        p.triangles.len() - 1,
        Limits { max_variables: 0, max_allowed_tuples: 0, max_decisions: 0, ..limits() },
    );
    assert_eq!(stable_seed.status, Status::Solved);
    assert_eq!(stable_seed.variables, 0);
    assert_eq!(stable_seed.assignment, Some(p.positions));
}

#[test]
fn an_unsatisfiable_incident_constraint_returns_no_partial_assignment() {
    let mut p = equality_chain();
    p.triangles.push([1, 6, 5]);
    p.normals.push([0., 0., 1.]);
    assert!(exhaustive_binary_solutions(&p, 3).is_empty());
    let before = p.positions.clone();
    let out = run(&p, 0, limits());
    assert_eq!(out.status, Status::Unsatisfiable);
    assert!(out.assignment.is_none());
    assert_eq!(out.decisions, 0);
    assert_eq!(p.positions, before);
}

fn branching_problem() -> Problem {
    let positions = vec![Vec3::new(0., 0., 0.), Vec3::new(2., 0., 0.), Vec3::new(1., 0.25, 0.)];
    let candidates =
        vec![vec![positions[0], Vec3::new(0., 1., 0.)], vec![positions[1], Vec3::new(2., 1., 0.)], vec![positions[2]]];
    Problem { positions, candidates, triangles: vec![[0, 1, 2]], normals: vec![[0., 0., -1.]] }
}
fn inconsistent_cycle() -> Problem {
    let mut p = equality_chain();
    p.positions[5] = Vec3::new(1., 0.25, 0.);
    p.candidates[5] = vec![p.positions[5]];
    p.positions[6] = Vec3::new(1., 0.75, 0.);
    p.candidates[6] = vec![p.positions[6]];
    p.triangles = vec![[0, 2, 5], [0, 2, 6], [0, 1, 3], [0, 1, 4], [1, 2, 3], [1, 2, 4]];
    p.normals = vec![[0., 0., -1.], [0., 0., 1.], [0., 0., 1.], [0., 0., 1.], [0., 0., 1.], [0., 0., 1.]];
    p
}

#[test]
fn bounded_search_distinguishes_unexplored_branches_from_proven_unsatisfiability() {
    let p = branching_problem();
    assert_eq!(exhaustive_binary_solutions(&p, 2).len(), 3);
    let limited = run(&p, 0, Limits { max_decisions: 0, ..limits() });
    assert_eq!(limited.status, Status::DecisionLimit);
    assert!(limited.assignment.is_none());
    assert_eq!(limited.decisions, 0);
    let solved = run(&p, 0, Limits { max_decisions: 1, ..limits() });
    assert_eq!(solved.status, Status::Solved);
    assert_eq!(solved.decisions, 1);
    assert!(positive(solved.assignment.as_ref().unwrap(), p.triangles[0], p.normals[0]));
    let p = inconsistent_cycle();
    assert!(exhaustive_binary_solutions(&p, 3).is_empty());
    let incomplete = run(&p, 0, Limits { max_decisions: 1, ..limits() });
    assert_eq!(incomplete.status, Status::DecisionLimit);
    assert!(incomplete.assignment.is_none());
    assert_eq!(incomplete.decisions, 1);
    let exhaustive = run(&p, 0, Limits { max_decisions: 2, ..limits() });
    assert_eq!(exhaustive.status, Status::Unsatisfiable);
    assert!(exhaustive.assignment.is_none());
    assert_eq!(exhaustive.decisions, 2);
}

#[test]
fn variable_and_allowed_tuple_caps_return_explicit_limits_without_assignments() {
    let p = equality_chain();
    let variables = run(&p, 0, Limits { max_variables: 2, ..limits() });
    assert_eq!(variables.status, Status::ComponentLimit);
    assert!(variables.assignment.is_none());
    assert!(matches!(variables.component_limit, Some(ComponentLimit::Variables { required: 3 })));
    let tuples = run(&p, 0, Limits { max_allowed_tuples: 12, ..limits() });
    assert_eq!(tuples.status, Status::ComponentLimit);
    assert!(tuples.assignment.is_none());
    assert!(matches!(tuples.component_limit, Some(ComponentLimit::AllowedTuples { required: 13 })));
    assert_eq!(run(&p, 0, Limits { max_variables: 3, max_allowed_tuples: 13, ..limits() }).status, Status::Solved);
}

#[test]
fn every_status_replays_its_exact_work_and_one_less_never_returns_partial_state() {
    let cases = [
        (equality_chain(), limits()),
        (equality_chain(), Limits { max_variables: 2, ..limits() }),
        (equality_chain(), Limits { max_allowed_tuples: 12, ..limits() }),
        (branching_problem(), Limits { max_decisions: 0, ..limits() }),
        (inconsistent_cycle(), Limits { max_decisions: 2, ..limits() }),
    ];
    for (p, limit) in cases {
        let before = p.positions.clone();
        let out = run(&p, 0, limit);
        assert!(out.work > 0);
        assert_eq!(run(&p, 0, Limits { max_work: out.work, ..limit }), out);
        let error =
            solve(&p.positions, &p.triangles, &p.normals, &p.candidates, 0, Limits { max_work: out.work - 1, ..limit })
                .unwrap_err();
        assert!(error.message.contains("budget"), "{}", error.message);
        assert!(error.work < out.work);
        assert_eq!(p.positions, before);
    }
}

#[test]
fn invalid_inputs_and_missing_current_candidates_reject_with_counted_work() {
    let p = equality_chain();
    let mut missing = p.candidates.clone();
    missing[0].remove(0);
    let first = solve(&p.positions, &p.triangles, &p.normals, &missing, 0, limits()).unwrap_err();
    assert!(first.message.contains("absent from its candidates") && first.work > 0);
    assert_eq!(
        solve(&p.positions, &p.triangles, &p.normals, &missing, 0, Limits { max_work: first.work, ..limits() })
            .unwrap_err(),
        first
    );
    let error =
        solve(&p.positions, &p.triangles, &p.normals, &missing, 0, Limits { max_work: first.work - 1, ..limits() })
            .unwrap_err();
    assert!(error.message.contains("budget") && error.work < first.work);
    for domain in [vec![], vec![p.positions[0]; 9], vec![p.positions[0], Vec3::new(f32::NAN, 0., 0.)]] {
        let mut bad = p.candidates.clone();
        bad[0] = domain;
        assert!(solve(&p.positions, &p.triangles, &p.normals, &bad, 0, limits()).is_err());
    }
    let mut normals = p.normals.clone();
    normals[0][0] = f64::INFINITY;
    assert!(solve(&p.positions, &p.triangles, &normals, &p.candidates, 0, limits())
        .unwrap_err()
        .message
        .contains("normals must be finite"));
    let mut triangles = p.triangles.clone();
    triangles[0] = [0, 0, 1];
    assert!(solve(&p.positions, &triangles, &p.normals, &p.candidates, 0, limits())
        .unwrap_err()
        .message
        .contains("indices"));
    assert!(solve(&p.positions, &p.triangles, &p.normals, &p.candidates, p.triangles.len(), limits()).is_err());
    for l in [
        Limits { max_variables: 65, ..limits() },
        Limits { max_allowed_tuples: 50_001, ..limits() },
        Limits { max_decisions: 4097, ..limits() },
    ] {
        assert!(solve(&p.positions, &p.triangles, &p.normals, &p.candidates, 0, l)
            .unwrap_err()
            .message
            .contains("fixed bounds"));
    }
}

#[test]
fn duplicate_candidates_do_not_create_spurious_variables_or_expand_the_coordinate_set() {
    let mut p = equality_chain();
    for fixed in 3..7 {
        p.candidates[fixed] = vec![p.positions[fixed]; 8];
    }
    let alternate = p.candidates[0][1];
    p.candidates[0].extend([p.positions[0], alternate]);
    let out = run(&p, 0, Limits { max_variables: 3, ..limits() });
    assert_eq!(out.status, Status::Solved);
    assert_eq!(out.variables, 3);
    assert_eq!(out.allowed_tuples, 13);
    for (v, point) in out.assignment.unwrap().iter().enumerate() {
        assert!(p.candidates[v].contains(point));
    }
}

#[test]
fn a_failed_branch_is_restored_before_a_later_branch_finds_a_complete_solution() {
    let mut p = equality_chain();
    p.positions[5] = Vec3::new(1., 0.25, 0.);
    p.candidates[5] = vec![p.positions[5]];
    // B<=A, A or C, and B=C. A=0 has no joint solution, although
    // every initial table supports both values of every variable.
    p.triangles = vec![[0, 1, 4], [0, 2, 5], [1, 2, 3], [1, 2, 4]];
    p.normals = vec![[0., 0., 1.], [0., 0., -1.], [0., 0., 1.], [0., 0., 1.]];
    let solutions = exhaustive_binary_solutions(&p, 3);
    assert_eq!(solutions.len(), 2);
    let limited = run(&p, 1, Limits { max_decisions: 2, ..limits() });
    assert_eq!(limited.status, Status::DecisionLimit);
    assert!(limited.assignment.is_none());
    let result = run(&p, 1, Limits { max_decisions: 3, ..limits() });
    assert_eq!(result.status, Status::Solved);
    assert_eq!(result.decisions, 3);
    let assignment = result.assignment.unwrap();
    assert!(solutions.contains(&assignment));
    assert_eq!((assignment[0].y, assignment[1].y, assignment[2].y), (1., 0., 0.));
}

#[test]
fn non_axis_aligned_3d_normals_use_the_same_stored_orientation_constraints() {
    let base = equality_chain();
    let expected = run(&base, 0, limits()).assignment.unwrap();
    for tilted in [false, true] {
        let transform = |p: Vec3| if tilted { Vec3::new(p.x, p.y, p.x + 2. * p.y) } else { Vec3::new(p.z, p.x, p.y) };
        let mut p = base.clone();
        p.positions = p.positions.into_iter().map(transform).collect();
        p.candidates = p.candidates.into_iter().map(|domain| domain.into_iter().map(transform).collect()).collect();
        p.normals = vec![if tilted { [-1., -2., 1.] } else { [1., 0., 0.] }; p.triangles.len()];
        let result = run(&p, 0, limits());
        assert_eq!(result.status, Status::Solved);
        assert_eq!(result.allowed_tuples, 13);
        assert_eq!(result.assignment.unwrap(), expected.iter().copied().map(transform).collect::<Vec<_>>());
    }
}

#[test]
fn nonfinite_predicate_products_and_sums_fail_with_counted_work() {
    let cases = [
        // Exact dot is negative, but the legacy expression overflows to +inf.
        (
            [Vec3::new(0., 0., 0.), Vec3::new(1., 1., 1.), Vec3::new(0., -1., 1.)],
            [0.6 * f64::MAX, 0.9 * f64::MAX, 0.9 * f64::MAX],
        ),
        // Each product is finite; the intermediate sum overflows before a
        // negative third term could cancel it in real arithmetic.
        ([Vec3::new(0., 0., 0.), Vec3::new(1., 0., 1.), Vec3::new(0., -1., -1.)], [0.6 * f64::MAX; 3]),
        // Only the final addition overflows.
        ([Vec3::new(0., 0., 0.), Vec3::new(1., -1., 0.), Vec3::new(1., 0., -1.)], [0.4 * f64::MAX; 3]),
    ];
    for (positions, normal) in cases {
        let candidates: Vec<_> = positions.iter().map(|&p| vec![p]).collect();
        let attempt = |work| {
            solve(&positions, &[[0, 1, 2]], &[normal], &candidates, 0, Limits { max_work: work, ..limits() })
                .unwrap_err()
        };
        let first = attempt(limits().max_work);
        assert!(first.message.contains("nonfinite intermediate") && first.work > 0);
        assert_eq!(attempt(first.work), first);
        let short = attempt(first.work - 1);
        assert!(short.message.contains("budget") && short.work < first.work);
    }
}

#[test]
fn finite_underflow_retains_nominal_predicate_scope_without_claiming_an_exact_sign() {
    let positions = vec![Vec3::new(0., 0., 0.), Vec3::new(0.5, 0., 0.), Vec3::new(0., 0.5, 0.)];
    let candidates = positions.iter().map(|&p| vec![p]).collect::<Vec<_>>();
    // Exact area .25 times a positive subnormal is positive mathematically,
    // but the required finite legacy predicate rounds it to zero.
    let tiny = f64::from_bits(1);
    assert_eq!(0.25 * tiny, 0.);
    let result = solve(&positions, &[[0, 1, 2]], &[[0., 0., tiny]], &candidates, 0, limits()).unwrap();
    assert_eq!(result.status, Status::Unsatisfiable);
    assert!(result.assignment.is_none());
    assert_eq!(
        solve(&positions, &[[0, 1, 2]], &[[0., 0., 1.]], &candidates, 0, limits()).unwrap().status,
        Status::Solved
    );
}
