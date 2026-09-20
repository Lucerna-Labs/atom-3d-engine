use mm3e_kit::{
    radial_embedding::{certify, CentreKind, FailureKind},
    surface_intersections::validate_counted,
    Vec3,
};
const WORK: usize = 200_000_000;
fn actual() -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let data = include_bytes!("fixtures/radial_embedding/general-warped-refined.bin");
    let mut offset = 0;
    let mut word = || {
        let value = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;
        value
    };
    let nv = word() as usize;
    let nf = word() as usize;
    assert_eq!((nv, nf), (16464, 32924));
    let mut p = Vec::with_capacity(nv);
    let mut t = Vec::with_capacity(nf);
    for _ in 0..nv {
        p.push(Vec3::new(f32::from_bits(word()), f32::from_bits(word()), f32::from_bits(word())));
    }
    for _ in 0..nf {
        t.push([word(), word(), word()]);
    }
    assert_eq!(offset, data.len());
    (p, t)
}
#[test]
fn unchanged_refined_mesh_radial_certificate_agrees_with_complete_pair_validation() {
    let (p, t) = actual();
    let source_bits: Vec<_> = p.iter().map(|p| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()]).collect();
    let original_triangles = t.clone();
    let complete = validate_counted(&p, &t, WORK).unwrap();
    let certificate = certify(&p, &t, WORK).unwrap();
    let report = certificate.into_report();
    assert_eq!(report.centre.kind, CentreKind::BoundsMidpoint);
    assert_eq!(report.centre.denominator, 2.);
    assert!(report.centre.numerators.iter().all(|row| row.iter().sum::<f64>() == 0.));
    assert_eq!((report.degree, report.forward_face_hits, report.faces, report.edges), (1, 1, 32924, 49386));
    assert_eq!(report.work.face_plane_tests, t.len());
    assert_eq!(report.work.edge_cone_tests, 3 * t.len());
    assert_eq!(report.work.integer_fallbacks, 0);
    assert_eq!(report.work.centres_attempted, 1);
    assert_eq!(report.work.rays_attempted, 1);
    assert!(report.work.work * 2 < complete.work, "radial {} versus full {}", report.work.work, complete.work);
    assert_eq!(certify(&p, &t, report.work.work).unwrap().into_report(), report);
    let failure = certify(&p, &t, report.work.work - 1).unwrap_err();
    assert_eq!(failure.kind, FailureKind::Budget);
    assert!(failure.work.work < report.work.work);
    assert_eq!(source_bits, p.iter().map(|p| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()]).collect::<Vec<_>>());
    assert_eq!(original_triangles, t);
    eprintln!(
        "RADIAL_ACTUAL faces={} vertices={} radial_work={} full_pair_work={} candidate_pairs={} report={report:?}",
        t.len(),
        p.len(),
        report.work.work,
        complete.work,
        complete.candidate_pairs
    );
}
