use super::*;
use crate::surface_intersections::validate_counted;
const WORK: usize = 200_000_000;
fn sphere(wraps: usize) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let mut p = vec![Vec3::new(0., 0., 1.), Vec3::new(0., 0., -1.)];
    let ring = [Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.), Vec3::new(-1., 0., 0.), Vec3::new(0., -1., 0.)];
    for _ in 0..wraps {
        p.extend(ring);
    }
    let mut t = Vec::new();
    for i in 0..4 * wraps {
        let a = (2 + i) as u32;
        let b = (2 + (i + 1) % (4 * wraps)) as u32;
        t.extend([[0, a, b], [1, b, a]]);
    }
    (p, t)
}
fn ledgers(w: &WorkReport) {
    assert_eq!(w.work, w.structural_work + w.topology_work + w.point_work + w.centre_work + w.plane_work + w.ray_work);
}
#[test]
fn ordinary_and_reversed_spheres_have_complete_degree_one_certificates() {
    let (p, t) = sphere(1);
    let mut reversed = t.clone();
    for t in &mut reversed {
        t.swap(1, 2);
    }
    for (t, degree) in [(&t, 1), (&reversed, -1)] {
        let certificate = certify(&p, t, WORK).unwrap();
        let report = certificate.report();
        assert_eq!(report.degree, degree);
        assert_eq!(report.forward_face_hits, 1);
        assert_eq!(report.euler_characteristic, 2);
        assert_eq!((report.faces, report.edges, report.topology.vertices), (8, 12, 6));
        assert_eq!(report.centre.kind, CentreKind::BoundsMidpoint);
        assert_eq!(report.centre.denominator, 2.);
        assert_eq!(report.ray.direction, DIRECTIONS[0]);
        assert_eq!(report.scope, SCOPE);
        ledgers(&report.work);
        validate_counted(&p, t, WORK).unwrap();
    }
}
#[test]
fn twice_wrapped_sphere_passes_plane_and_euler_gates_but_degree_two_rejects() {
    let (p, t) = sphere(2);
    let topology = mesh_topology::validate(&t, WORK).unwrap();
    assert_eq!((topology.vertices, topology.edges, topology.triangles, topology.components), (10, 24, 16, 1));
    let failure = certify(&p, &t, WORK).unwrap_err();
    assert_eq!(failure.kind, FailureKind::Inconclusive);
    assert!(failure.message.contains("2 forward hits (signed degree 2)"));
    assert_eq!(failure.work.face_plane_tests, t.len());
    assert_eq!(failure.work.edge_cone_tests, t.len() * 3);
    ledgers(&failure.work);
    assert!(validate_counted(&p, &t, WORK).is_err());
}
#[test]
fn extra_components_and_a_closed_torus_do_not_receive_spherical_certificates() {
    let (mut p, mut t) = sphere(1);
    let (other, faces) = sphere(1);
    let base = p.len() as u32;
    p.extend(other.into_iter().map(|p| p + Vec3::new(5., 0., 0.)));
    t.extend(faces.into_iter().map(|t| t.map(|i| i + base)));
    let failure = certify(&p, &t, WORK).unwrap_err();
    assert_eq!(failure.kind, FailureKind::Inconclusive);
    assert!(failure.message.contains("one component"));
    assert_eq!(failure.work.point_work, 0);
    validate_counted(&p, &t, WORK).unwrap();
    let id = |x: u32, y: u32| (y % 3) * 3 + x % 3;
    let mut torus = Vec::new();
    for y in 0..3 {
        for x in 0..3 {
            torus.extend([[id(x, y), id(x + 1, y), id(x + 1, y + 1)], [id(x, y), id(x + 1, y + 1), id(x, y + 1)]]);
        }
    }
    let p: Vec<_> = (0..9).map(|i| Vec3::new((i % 3) as f32, (i / 3) as f32, 0.)).collect();
    let proof = mesh_topology::validate(&torus, WORK).unwrap();
    assert_eq!(proof.vertices + proof.triangles, proof.edges);
    let failure = certify(&p, &torus, WORK).unwrap_err();
    assert_eq!(failure.kind, FailureKind::Inconclusive);
    assert!(failure.message.contains("Euler characteristic 2"));
    assert_eq!(failure.work.point_work, 0);
}
#[test]
fn failed_bbox_candidate_and_nongeneric_first_ray_remain_in_successful_work() {
    let (mut p, t) = sphere(1);
    for v in &mut p {
        v.x = if v.x > 0. { 100. } else { v.x };
        v.y = if v.y > 0. { 100. } else { v.y };
        v.z = if v.z > 0. { 100. } else { v.z };
    }
    let certificate = certify(&p, &t, WORK).unwrap();
    let report = certificate.report();
    assert_eq!(report.centre.kind, CentreKind::VertexMean);
    assert_eq!(report.work.centres_attempted, 2);
    assert!(report.work.face_plane_tests > t.len());
    assert_eq!(report.centre.denominator, 6.);
    ledgers(&report.work);
    validate_counted(&p, &t, WORK).unwrap();
    let (mut p, t) = sphere(1);
    let [x, y, z] = DIRECTIONS[0];
    for p in &mut p {
        *p = Vec3::new(p.x * x as f32, p.y + p.x * y as f32, p.z + p.x * z as f32);
    }
    let certificate = certify(&p, &t, WORK).unwrap();
    let report = certificate.report();
    assert_eq!(report.work.rays_attempted, 2);
    assert_eq!(report.ray.direction, DIRECTIONS[1]);
    assert!(report.work.edge_cone_tests > 3 * t.len());
    assert!(report.work.integer_fallbacks > 0);
    ledgers(&report.work);
    assert!(report.work.centres_attempted <= MAX_CENTRE_CANDIDATES);
    assert!(report.work.rays_attempted <= MAX_CENTRE_CANDIDATES * MAX_RAY_CANDIDATES);
}
#[test]
fn edge_vertex_and_centre_plane_degeneracies_are_inconclusive_without_epsilon() {
    let (p, t) = sphere(1);
    let mut budget = Budget { limit: WORK, work: WorkReport::default() };
    let mut points = PointCache::new(&p, &mut budget).unwrap();
    let c = make_point(&[vec![0.], vec![0.], vec![0.]], 1., Ledger::Centre, &mut budget).unwrap();
    assert_eq!(common_plane_sign(&mut points, &t, &c, &mut budget).unwrap(), Some(1));
    for direction in [[1., 0., 0.], [1., 1., 0.]] {
        let q = make_point(&direction.map(|x| vec![x]), 1., Ledger::Centre, &mut budget).unwrap();
        assert_eq!(ray_hits(&points, &t, &c, &q, 1, &mut budget).unwrap(), None);
    }
    let vertex_centre = make_point(&[vec![0.], vec![0.], vec![1.]], 1., Ledger::Centre, &mut budget).unwrap();
    assert_eq!(common_plane_sign(&mut points, &t, &vertex_centre, &mut budget).unwrap(), None);
    assert!(budget.work.integer_fallbacks > 0);
}
#[test]
fn malformed_geometry_rejects_and_open_or_pinched_complexes_are_not_certified() {
    let (p, t) = sphere(1);
    for input in [Vec::new(), vec![Vec3::ZERO]] {
        let failure = certify(&input, &t, WORK).unwrap_err();
        assert_eq!(failure.kind, FailureKind::InvalidInput);
        assert!(failure.work.work > 0);
    }
    assert_eq!(certify(&p, &[], WORK).unwrap_err().kind, FailureKind::InvalidInput);
    for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let mut invalid = p.clone();
        invalid[0].x = value;
        assert_eq!(certify(&invalid, &t, WORK).unwrap_err().kind, FailureKind::InvalidInput);
    }
    for triangle in [[0, 0, 1], [0, 1, p.len() as u32], [0, 1, u32::MAX]] {
        assert_eq!(certify(&p, &[triangle], WORK).unwrap_err().kind, FailureKind::InvalidInput);
    }
    assert_eq!(certify(&p, &t[..t.len() - 1], WORK).unwrap_err().kind, FailureKind::Inconclusive);
    let mut bad = t.clone();
    bad[0].swap(1, 2);
    assert_eq!(certify(&p, &bad, WORK).unwrap_err().kind, FailureKind::Inconclusive);
    let mut vertices = p.clone();
    vertices.extend(p.iter().skip(1).copied());
    let mut pinch = t.clone();
    pinch.extend(t.iter().map(|t| t.map(|i| if i == 0 { 0 } else { i + 5 })));
    assert_eq!(certify(&vertices, &pinch, WORK).unwrap_err().kind, FailureKind::Inconclusive);
}
#[test]
fn exact_budget_replay_and_failed_retry_charges_never_create_partial_success() {
    let (p, t) = sphere(1);
    let certificate = certify(&p, &t, WORK).unwrap();
    let expected = certificate.into_report();
    assert_eq!(certify(&p, &t, expected.work.work).unwrap().into_report(), expected);
    let failure = certify(&p, &t, expected.work.work - 1).unwrap_err();
    assert_eq!(failure.kind, FailureKind::Budget);
    assert!(failure.work.work < expected.work.work);
    ledgers(&failure.work);
    assert_eq!(certify(&p, &t, 0).unwrap_err().work.work, 0);
    let (p, t) = sphere(2);
    let failure = certify(&p, &t, WORK).unwrap_err();
    assert_eq!(failure.kind, FailureKind::Inconclusive);
    assert_eq!(certify(&p, &t, failure.work.work).unwrap_err(), failure);
    let allowance = 2 * failure.work.work - 1;
    let retried = certify(&p, &t, allowance - failure.work.work).unwrap_err();
    assert_eq!(retried.kind, FailureKind::Budget);
    assert!(failure.work.work + retried.work.work <= allowance);
    ledgers(&retried.work);
    assert_eq!(certify(&p, &t, WORK).unwrap_err(), failure);
}
#[test]
fn virtual_ray_endpoint_preserves_a_direction_lost_by_rounded_addition() {
    let mut budget = Budget { limit: WORK, work: WorkReport::default() };
    let large = 2f64.powi(100);
    let c = make_point(&[vec![large], vec![0.], vec![0.]], 1., Ledger::Centre, &mut budget).unwrap();
    let q = make_point(&[vec![large, 1.], vec![5. / 16.], vec![11. / 64.]], 1., Ledger::Centre, &mut budget).unwrap();
    assert_eq!(large + 1., large);
    assert_eq!(exact_geometry::compare_axis(&q, &c, 0, WORK).unwrap().sign, 1);
}

#[test]
fn failed_face_prefix_does_not_convert_unvisited_vertices_or_refund_work() {
    let (mut p, mut t) = sphere(1);
    // Put the top apex inside the lower half: the first two oriented faces
    // disagree at both exact centres. This remains valid indexed topology.
    p[0] = Vec3::new(0.25, 0.25, -0.25);
    // An unused finite vertex still participates in centre definitions, but
    // must never require a rational face point. Its finiteness remains checked.
    p.push(Vec3::new(0.125, 0.125, 0.125));
    t.rotate_left(1);
    let error = certify(&p, &t, WORK).unwrap_err();
    assert_eq!(error.kind, FailureKind::Inconclusive);
    assert_eq!(certify(&p, &t, error.work.work).unwrap_err(), error);
    assert_eq!(certify(&p, &t, error.work.work - 1).unwrap_err().kind, FailureKind::Budget);
    ledgers(&error.work);
    let mut eager = Budget { limit: WORK, work: WorkReport::default() };
    for point in &p {
        make_point(&[vec![point.x as f64], vec![point.y as f64], vec![point.z as f64]], 1., Ledger::Point, &mut eager)
            .unwrap();
    }
    assert!(error.work.point_work < eager.work.point_work, "public failed-prefix proof must skip unvisited points");
    let mut budget = Budget { limit: WORK, work: WorkReport::default() };
    let mut cache = PointCache::new(&p, &mut budget).unwrap();
    cache.prepare(t[0], &mut budget).unwrap();
    assert_eq!(cache.points.iter().filter(|p| p.is_some()).count(), 3);
    let point_work = budget.work.point_work;
    cache.prepare(t[0], &mut budget).unwrap();
    assert_eq!(budget.work.point_work, point_work);
    assert_eq!(budget.work.structural_work, p.len() + 6);
    p.last_mut().unwrap().x = f32::NAN;
    assert_eq!(certify(&p, &t, WORK).unwrap_err().kind, FailureKind::InvalidInput);
}
