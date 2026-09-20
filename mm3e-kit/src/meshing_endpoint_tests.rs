//! Independent frozen Gaussian/cofactor reference for the exact endpoint shortcut.
//! Reference body captured from immutable protocol source b4cba1ca; production
//! endpoint recognition must preserve all returned bits and constraint rejection.
use super::*;
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
fn bits(v: &Vertex) -> Vec<u64> {
    let mut out = Vec::new();
    out.extend(v.key.nodes.map(u64::from));
    out.extend(v.key.planes);
    out.extend(v.weights.map(f64::to_bits));
    out.extend(v.point.map(f64::to_bits));
    out.extend([u64::from(v.faces), v.planes as u64, (v.planes >> 64) as u64]);
    out.extend(v.source.point.map(f64::to_bits));
    out.extend(v.source.nodes.map(u64::from));
    out.extend(v.source.weights.map(f64::to_bits));
    out.push(u64::from(v.source.affine.is_some()));
    if let Some(a) = &v.source.affine {
        out.extend(a.tetra.map(u64::from));
        out.extend(a.points.iter().flatten().map(|v| v.to_bits()));
        out.extend(a.basis);
        out.push(a.planes.len() as u64);
        for (id, row) in &a.planes {
            out.push(*id);
            out.extend(row.map(f64::to_bits));
        }
    }
    out.push(u64::from(v.global_planes.is_some()));
    if let Some(p) = &v.global_planes {
        out.push(p.len() as u64);
        out.extend(p);
    }
    out
}
fn legacy_vertex(context: &Context<'_>, faces: u8, planes: u128, work: &mut WorkBudget) -> Result<Vertex, String> {
    charge(work, 1)?;
    let mut local: Vec<_> = (0..4).filter(|i| faces & (1 << i) == 0).collect();
    local.sort_by_key(|&i| context.tetra[i]);
    let cuts: Vec<_> = (0..context.planes.len()).filter(|i| planes & (1u128 << i) != 0).collect();
    let dimension = local.len();
    if dimension == 0 || cuts.len() + 1 < dimension {
        return Err("Boolean vertex has insufficient support constraints".into());
    }
    let mut basis = Vec::new();
    let mut solution = None;
    for a in 0..cuts.len().max(1) {
        for b in (a + 1)..=cuts.len().max(a + 1) {
            for c in (b + 1)..=cuts.len().max(b + 1) {
                let selected = match dimension {
                    1 => vec![],
                    2 => vec![cuts[a]],
                    3 if b < cuts.len() => vec![cuts[a], cuts[b]],
                    4 if c < cuts.len() => vec![cuts[a], cuts[b], cuts[c]],
                    _ => continue,
                };
                charge(work, dimension.pow(3))?;
                if let Some(weights) = solve_weights(&local, &selected, context.planes) {
                    basis = selected;
                    solution = Some(weights);
                    break;
                }
                if dimension <= 2 {
                    break;
                }
            }
            if solution.is_some() || dimension <= 2 {
                break;
            }
        }
        if solution.is_some() {
            break;
        }
    }
    let mut solved = solution.ok_or_else(|| {
            if context.retain_affine {
                format!("Boolean plane arrangement has singular intersection constraints; tetra {:?}, points {:?}, local {:?}, cuts {:?}",context.tetra,context.points,local,cuts.iter().map(|&i|(context.feature_id(i),context.planes[i])).collect::<Vec<_>>())
            }else{"Boolean plane arrangement has singular intersection constraints".into()}
        })?;
    if solved[..dimension].iter().any(|&v| !(-1e-10..=1.0000000001).contains(&v)) {
        return Err("Boolean plane intersection lies outside its support simplex".into());
    }
    if context.feature_ids.is_some() && solved[..dimension].iter().any(|value| value.abs() <= 1e-10) {
        // Precharge bounded expansion construction before its arithmetic.
        charge(
            work,
            match dimension {
                1 => 1,
                2 => 8,
                3 => 64,
                _ => 2048,
            },
        )?;
        let exact = exact_weights(&local, &basis, context.planes)?;
        for i in 0..dimension {
            if solved[i].abs() <= 1e-10 && (exact[i] == 0.0 || solved[i] == 0.0) {
                if exact[i] == 0.0 && solved[i] != 0.0 {
                    work.exact_support_reductions += 1;
                }
                solved[i] = exact[i];
            }
        }
    }
    let mut weights = [0.0; 4];
    for (i, &local_index) in local.iter().enumerate() {
        weights[local_index] = solved[i];
    }
    for &cut in &cuts {
        let residual = (0..4).map(|i| weights[i] * context.planes[cut][i]).sum::<f64>().abs();
        let scale = local.iter().map(|&i| context.planes[cut][i].abs()).fold(0.0, f64::max);
        if residual > scale * 1e-10 {
            return Err("Boolean intersection has inconsistent supporting planes".into());
        }
    }
    let reduced_faces = (0..4).filter(|&i| weights[i] == 0.0).fold(0u8, |bits, i| bits | (1 << i));
    if reduced_faces != faces {
        return legacy_vertex(context, reduced_faces, planes, work);
    }
    let mut key = Key { nodes: [u32::MAX; 4], planes: [u64::MAX; 3] };
    for (i, &index) in local.iter().enumerate() {
        key.nodes[i] = context.tetra[index];
    }
    for (i, &plane) in basis.iter().enumerate() {
        key.planes[i] = context.feature_id(plane);
    }
    let mut point = [0.0; 3];
    for (i, &weight) in weights.iter().enumerate() {
        point = add(point, scale(context.points[i], weight));
    }
    Ok(Vertex {
        key,
        weights,
        point,
        faces,
        planes,
        source: SourcePoint {
            point,
            nodes: key.nodes,
            weights: solved,
            affine: if context.retain_affine {
                charge(work, cuts.len() * 4)?;
                Some(std::sync::Arc::new(AffineSource {
                    exact_basis: Default::default(),
                    tetra: context.tetra,
                    points: context.points,
                    basis: key.planes,
                    planes: cuts.iter().map(|&i| (context.feature_id(i), context.planes[i])).collect(),
                }))
            } else {
                None
            },
        },
        global_planes: context.feature_ids.map(|ids| cuts.iter().map(|&i| ids[i]).collect()),
    })
}

#[test]
fn exact_endpoints_match_frozen_reference_across_every_f32_exponent_and_support_edge() {
    let points = [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];
    let mut count = 0;
    for exponent in 0..255u32 {
        for mantissa in [0, 1, 0x007f_ffff] {
            for sign in [0, 0x8000_0000] {
                let value = f32::from_bits(sign | (exponent << 23) | mantissa);
                if value == 0. {
                    continue;
                }
                for a in 0..4 {
                    for b in a + 1..4 {
                        for endpoint in [a, b] {
                            for zero in [0.0, -0.0] {
                                let other = if endpoint == a { b } else { a };
                                let mut row = [0.; 4];
                                row[endpoint] = zero;
                                row[other] = f64::from(value);
                                let planes = [[0.; 4], row, row.map(|x| -x)];
                                for tetra in [[0, 1, 2, 3], [9, 7, 4, 1]] {
                                    let context = Context {
                                        tetra,
                                        points,
                                        planes: &planes,
                                        feature_ids: Some(&[7, 11, 19]),
                                        retain_affine: true,
                                        vertex_cache: Default::default(),
                                        proportional_cache: Default::default(),
                                    };
                                    let faces = 15 ^ ((1 << a) | (1 << b));
                                    let mut old = budget(100_000);
                                    let expected = legacy_vertex(&context, faces, 7, &mut old).unwrap();
                                    let mut new = budget(100_000);
                                    let actual = context.vertex(faces, 7, &mut new).unwrap();
                                    assert_eq!(
                                        bits(&actual),
                                        bits(&expected),
                                        "value={value:?} edge={a},{b} endpoint={endpoint}"
                                    );
                                    assert_eq!(new.exact_support_reductions, old.exact_support_reductions);
                                    assert!(new.used < old.used);
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(count > 70_000);
}
#[test]
fn endpoint_constraints_and_exact_budget_cannot_be_bypassed() {
    let planes = [[0., 1., 0., 0.], [0., -1., 0., 0.]];
    let context = Context {
        tetra: [3, 2, 1, 0],
        points: [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
        planes: &planes,
        feature_ids: Some(&[11, 12]),
        retain_affine: true,
        vertex_cache: Default::default(),
        proportional_cache: Default::default(),
    };
    let mut full = budget(100_000);
    let output = context.vertex(12, 3, &mut full).unwrap();
    let mut exact = budget(full.used);
    assert_eq!(bits(&context.vertex(12, 3, &mut exact).unwrap()), bits(&output));
    let mut short = budget(full.used - 1);
    assert!(context.vertex(12, 3, &mut short).is_err());
    assert!(short.used < full.used);
    let conflicting = [[0., 1., 0., 0.], [f64::from(f32::from_bits(1)), 1., 0., 0.]];
    let rejected = Context { planes: &conflicting, ..context };
    assert!(legacy_vertex(&rejected, 12, 3, &mut budget(100_000)).is_err());
    assert!(rejected.vertex(12, 3, &mut budget(100_000)).is_err());
    let nonendpoint = [[1., -1., 0., 0.]];
    let generic = Context { planes: &nonendpoint, feature_ids: None, ..rejected };
    let mut old = budget(100_000);
    let a = legacy_vertex(&generic, 12, 1, &mut old).unwrap();
    let mut new = budget(100_000);
    let b = generic.vertex(12, 1, &mut new).unwrap();
    assert_eq!(bits(&a), bits(&b));
    assert_eq!(old.used, new.used);
    let local_general = Context { planes: &planes, feature_ids: Some(&[11, 12]), retain_affine: false, ..generic };
    let mut old = budget(100_000);
    let a = legacy_vertex(&local_general, 12, 3, &mut old).unwrap();
    let mut new = budget(100_000);
    let b = local_general.vertex(12, 3, &mut new).unwrap();
    assert_eq!(bits(&a), bits(&b));
    assert_eq!(old.used, new.used);
}
