// Frozen pre-cache ear clipping reference; original charge weights and arithmetic.
fn reference_ear_clipping(
    perimeter: &[u32],
    coordinate: impl Fn(u32) -> [f64; 3],
    normal: [f64; 3],
    work: &mut WorkBudget,
) -> Result<Option<Vec<[u32; 3]>>, String> {
    let axis = (0..3).max_by(|&a, &b| normal[a].abs().total_cmp(&normal[b].abs())).unwrap();
    if normal[axis] == 0. || !normal.iter().all(|v| v.is_finite()) {
        return Err("convex facet has no finite source orientation".into());
    }
    let orient = |a: [f64; 3], b: [f64; 3], c: [f64; 3]| {
        let u = (axis + 1) % 3;
        let v = (axis + 2) % 3;
        (b[u] - a[u]) * (c[v] - a[v]) - (b[v] - a[v]) * (c[u] - a[u])
    };
    let on_segment = |a: [f64; 3], b: [f64; 3], p: [f64; 3]| {
        [(axis + 1) % 3, (axis + 2) % 3].into_iter().all(|i| p[i] >= a[i].min(b[i]) && p[i] <= a[i].max(b[i]))
    };
    for i in 0..perimeter.len() {
        for j in i + 1..perimeter.len() {
            if j == i + 1 || i == 0 && j + 1 == perimeter.len() {
                continue;
            }
            charge(work, 12)?;
            let [a, b, c, d] = [
                perimeter[i],
                perimeter[(i + 1) % perimeter.len()],
                perimeter[j],
                perimeter[(j + 1) % perimeter.len()],
            ]
            .map(&coordinate);
            let [ab_c, ab_d, cd_a, cd_b] = [orient(a, b, c), orient(a, b, d), orient(c, d, a), orient(c, d, b)];
            if ab_c * ab_d < 0. && cd_a * cd_b < 0.
                || ab_c == 0. && on_segment(a, b, c)
                || ab_d == 0. && on_segment(a, b, d)
                || cd_a == 0. && on_segment(c, d, a)
                || cd_b == 0. && on_segment(c, d, b)
            {
                return Ok(None);
            }
        }
    }
    charge(work, perimeter.len())?;
    let mut perimeter = perimeter.to_vec();
    let mut pieces = Vec::new();
    while perimeter.len() > 3 {
        let n = perimeter.len();
        let mut chosen = None;
        for i in 0..n {
            charge(work, n * 16)?;
            let ids = [perimeter[(i + n - 1) % n], perimeter[i], perimeter[(i + 1) % n]];
            let [a, b, c] = ids.map(&coordinate);
            let face_cross = cross(sub(b, a), sub(c, a));
            if dot(face_cross, normal) <= 0. {
                continue;
            }
            if perimeter.iter().filter(|id| !ids.contains(id)).any(|&id| {
                let p = coordinate(id);
                [(a, b), (b, c), (c, a)].into_iter().all(|(a, b)| dot(cross(sub(b, a), sub(p, a)), normal) >= 0.)
            }) {
                continue;
            }
            let ab = sub(b, a);
            let ac = sub(c, a);
            let bc = sub(c, b);
            let sum = dot(ab, ab) + dot(ac, ac) + dot(bc, bc);
            let quality = dot(face_cross, face_cross) / (sum * sum);
            if chosen.is_none_or(|(_, best)| quality > best) {
                chosen = Some((i, quality));
            }
        }
        let Some((i, _)) = chosen else {
            return Ok(None);
        };
        pieces.push([perimeter[(i + n - 1) % n], perimeter[i], perimeter[(i + 1) % n]]);
        perimeter.remove(i);
    }
    let triangle: [u32; 3] = perimeter.try_into().map_err(|_| "convex perimeter lost its triangle")?;
    let [a, b, c] = triangle.map(&coordinate);
    if dot(cross(sub(b, a), sub(c, a)), normal) <= 0. {
        return Ok(None);
    }
    pieces.push(triangle);
    Ok(Some(pieces))
}
