//! Cache identities, bounded charged work, and oriented corner provenance.
use super::*;

const POINTS: [[f64; 3]; 4] = [[0., 0., 0.], [1., 0., 0.], [1., 1., 0.], [1., 1., 1.]];
const IDS: [u64; 4] = [11, 12, 13, 14];
fn budget(limit: usize) -> WorkBudget {
    WorkBudget {
        used: 0,
        structural: 0,
        extra_field_evaluations: 0,
        field_cost_per_callback: 1,
        limit,
        exact_support_reductions: 0,
    }
}
fn context(planes: &[[f64; 4]]) -> Context<'_> {
    Context::new([0, 1, 2, 3], POINTS, planes, Some(&IDS[..planes.len()]), true)
}

#[test]
fn full_relations_share_symmetric_keys_for_same_opposite_and_noncoincident_rows() {
    let planes = [[-1., 1., 2., 3.], [-2., 2., 4., 6.], [1., -1., -2., -3.], [-1., 1., 2., 4.]];
    for (other, expected) in [(1, Some(true)), (2, Some(false)), (3, None)] {
        let context = context(&planes);
        let mut cold = budget(10_000);
        assert_eq!(context.proportional_relation(0, other, &mut cold).unwrap(), expected);
        let mut reverse = budget(10_000);
        assert_eq!(context.proportional_relation(other, 0, &mut reverse).unwrap(), expected);
        assert_eq!(reverse.used, 3);
        assert!(reverse.used < cold.used);
        assert_eq!(context.proportional_cache.borrow().len(), 1);
    }
}

#[test]
fn support_mask_separates_full_noncoincidence_from_edge_coincidence() {
    let planes = [[-1., 1., 0., 0.], [-2., 2., 7., 9.], [0., 0., 1., 2.]];
    let context = context(&planes);
    assert_eq!(context.proportional_relation(0, 1, &mut budget(10_000)).unwrap(), None);
    assert_eq!(context.support_relation(12, 0, 1, &mut budget(10_000)).unwrap(), Some(true));
    assert_eq!(context.support_relation(8, 0, 1, &mut budget(10_000)).unwrap(), None);
    // A row vanishing on the support must not be used as a proportional pivot.
    assert_eq!(context.support_relation(12, 0, 2, &mut budget(10_000)).unwrap(), None);
    assert_eq!(context.support_relation(12, 2, 0, &mut budget(10_000)).unwrap(), None);
    let mut repeated = budget(3);
    assert_eq!(context.support_relation(12, 1, 0, &mut repeated).unwrap(), Some(true));
    assert_eq!(context.proportional_cache.borrow().len(), 4);
}

#[test]
fn local_cache_preserves_exact_zero_and_nonzero_vertex_values() {
    let planes = [[-1., 1., 0., 0.], [-2., 2., 7., 9.], [1., 3., 7., 9.]];
    let context = context(&planes);
    let vertex = context.vertex(12, 1, &mut budget(10_000)).unwrap();
    for (channel, expected) in [(1, 0.), (2, 2.)] {
        let mut cold = budget(10_000);
        assert_eq!(context.value(&vertex, channel, &mut cold).unwrap(), expected);
        let mut warm = budget(10_000);
        assert_eq!(context.value(&vertex, channel, &mut warm).unwrap(), expected);
        assert!(warm.used < cold.used);
        assert!(warm.used > 3, "cache hits still charge support setup and vertex evaluation");
        let mut exact = budget(warm.used);
        assert_eq!(context.value(&vertex, channel, &mut exact).unwrap(), expected);
        let mut short = budget(warm.used - 1);
        assert!(context.value(&vertex, channel, &mut short).is_err());
        assert!(short.used <= short.limit);
    }
}

#[test]
fn relation_misses_and_hits_replay_at_exact_work_and_reject_one_less() {
    let planes = [[-1., 1., 0., 0.], [-2., 2., 7., 9.], [1., -1., 0., 0.]];
    for (faces, other, expected) in [(0, 1, None), (12, 1, Some(true)), (0, 2, Some(false))] {
        let fresh = context(&planes);
        let mut measured = budget(10_000);
        assert_eq!(fresh.support_relation(faces, 0, other, &mut measured).unwrap(), expected);
        let exact = context(&planes);
        let mut work = budget(measured.used);
        assert_eq!(exact.support_relation(faces, other, 0, &mut work).unwrap(), expected);
        assert_eq!(work.used, measured.used);
        let short = context(&planes);
        let mut work = budget(measured.used - 1);
        assert!(short.support_relation(faces, 0, other, &mut work).is_err());
        assert!(short.proportional_cache.borrow().is_empty(), "failed insertion may not warm the cache");
        assert_eq!(fresh.support_relation(faces, other, 0, &mut budget(3)).unwrap(), expected);
        assert!(fresh.support_relation(faces, other, 0, &mut budget(2)).is_err());
    }
}

#[test]
fn cache_saturation_falls_back_to_charged_exact_predicates() {
    let planes: Vec<_> = (0..128).map(|i| [f64::from(i + 1), 1., 2., 3.]).collect();
    let context = Context::new([0, 1, 2, 3], POINTS, &planes, None, true);
    for a in 0..128 {
        for b in (a + 1)..128 {
            assert_eq!(context.proportional_relation(a, b, &mut budget(1000)).unwrap(), None);
        }
    }
    assert_eq!(context.proportional_cache.borrow().len(), MAX_CACHED_PLANE_RELATIONS);
    let mut first = budget(1000);
    assert_eq!(context.proportional_relation(126, 127, &mut first).unwrap(), None);
    let mut second = budget(1000);
    assert_eq!(context.proportional_relation(127, 126, &mut second).unwrap(), None);
    assert_eq!(first.used, second.used);
    assert!(first.used > 3);
    assert!(context.proportional_relation(126, 127, &mut budget(first.used - 1)).is_err());
    assert_eq!(context.proportional_cache.borrow().len(), MAX_CACHED_PLANE_RELATIONS);
}

#[test]
fn finite_f32_extremes_preserve_exact_relations() {
    let tiny = f64::from(f32::from_bits(1));
    let big = f64::from(f32::MAX);
    let planes = [[tiny, -tiny, big, -big], [-tiny, tiny, -big, big], [tiny, -tiny, big, big]];
    let context = context(&planes);
    assert_eq!(context.proportional_relation(0, 1, &mut budget(1000)).unwrap(), Some(false));
    assert_eq!(context.proportional_relation(0, 2, &mut budget(1000)).unwrap(), None);
    assert_eq!(context.support_relation(12, 0, 2, &mut budget(1000)).unwrap(), Some(true));
}

#[test]
fn fixed_edge_solver_retains_the_general_gaussian_work_charge() {
    let planes = [[-1., 1., 0., 0.]];
    let fast = context(&planes);
    let general = Context::new([0, 1, 2, 3], POINTS, &planes, None, true);
    let mut fast_work = budget(1000);
    let mut general_work = budget(1000);
    let actual = fast.vertex(12, 1, &mut fast_work).unwrap();
    let reference = general.vertex(12, 1, &mut general_work).unwrap();
    assert_eq!(actual.weights.map(f64::to_bits), reference.weights.map(f64::to_bits));
    // The fast path also performs the separately charged endpoint inspection.
    assert_eq!(fast_work.used, general_work.used + 3);
    assert_eq!(fast_work.used, 1 + 3 + 8 + 4);
    assert!(fast.vertex(12, 1, &mut budget(fast_work.used)).is_ok());
    assert!(fast.vertex(12, 1, &mut budget(fast_work.used - 1)).is_err());
}

fn triangle_vertex(point: [f64; 3], id: u32) -> Vertex {
    let nodes = [id, u32::MAX, u32::MAX, u32::MAX];
    let [x, y, _] = point;
    let hi_x = f64::from((x + 1.) as f32);
    let hi_y = f64::from((y + 1.) as f32);
    let source = SourcePoint {
        point,
        nodes,
        weights: [1., 0., 0., 0.],
        affine: Some(std::sync::Arc::new(AffineSource {
            exact_basis: Default::default(),
            tetra: [id, id + 10, id + 20, id + 30],
            points: [point, [hi_x, y, 0.], [hi_x, hi_y, 0.], [hi_x, hi_y, 1.]],
            basis: [u64::MAX; 3],
            planes: vec![(77, [0., 0., 0., -1.])],
        })),
    };
    Vertex {
        key: Key { nodes, planes: [u64::MAX; 3] },
        weights: [1., 0., 0., 0.],
        point,
        faces: 14,
        planes: 1,
        source,
        global_planes: Some(vec![77, 100 + u64::from(id)]),
    }
}

#[test]
fn direct_triangle_anchor_preserves_oriented_corners_and_source_provenance() {
    // Cyclic legacy scores differ for this finite-f32 planar triangle.
    let points = [
        [0.09822475165128708, 0.4119933247566223, 0.],
        [0.004628063645213842, 0.9932558536529541, 0.],
        [0.3418545424938202, 0.5300215482711792, 0.],
    ];
    let polygon: Vec<_> = points.iter().enumerate().map(|(i, &p)| triangle_vertex(p, i as u32)).collect();
    let scores = small_fan_scores(&polygon);
    assert!(scores[1] > scores[0] && scores[1] > scores[2]);
    for outward in [true, false] {
        for rotation in 0..3 {
            let mut input = polygon.clone();
            input.rotate_left(rotation);
            let mut expected = input.clone();
            if !outward {
                expected.reverse();
            }
            let first = expected.iter().position(|vertex| vertex.key.nodes[0] == 0).unwrap();
            expected.rotate_left(first);
            let mut output =
                Output { face_features: Some(Vec::new()), global_supports: Some(Vec::new()), ..Output::default() };
            output.polygon(input, outward, 77, Some([0., 0., -1.]), &mut budget(10_000)).unwrap();
            assert_eq!(output.triangles.len(), 1);
            assert_eq!(output.face_features.as_ref().unwrap(), &[77]);
            assert_eq!(output.convex_polygons[0].vertices, output.triangles[0]);
            assert!(output.collapsed.is_empty());
            for (corner, index) in output.triangles[0].iter().enumerate() {
                let index = *index as usize;
                let vertex = &expected[corner];
                assert_eq!(doubles(output.positions[index]), vertex.point);
                assert_eq!(output.sources[index].nodes, vertex.source.nodes);
                assert_eq!(output.sources[index].weights.map(f64::to_bits), vertex.source.weights.map(f64::to_bits));
                assert_eq!(output.sources[index].point.map(f64::to_bits), vertex.source.point.map(f64::to_bits));
                assert!(std::sync::Arc::ptr_eq(
                    output.sources[index].affine.as_ref().unwrap(),
                    vertex.source.affine.as_ref().unwrap()
                ));
                assert_eq!(output.constraints[index], vertex.planes);
                assert_eq!(&output.global_supports.as_ref().unwrap()[index], vertex.global_planes.as_ref().unwrap());
            }
        }
    }
}
