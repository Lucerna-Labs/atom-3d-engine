//! Generic callback-based projection and conforming surface refinement.
//! Source tetrahedral coordinates are retained: nearby nonzero leaf coordinates
//! blend smoothly toward the actual field, preventing a moving crease from
//! folding triangles against unadjusted neighboring single-leaf vertices.

use super::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub(super) struct Report {
    pub projected_vertices: usize,
    pub passes: usize,
    pub added_vertices: usize,
    pub added_triangles: usize,
    pub maximum_displacement: f32,
}

struct Evaluation {
    point: Vec3,
    values: [f32; MAX_BOOLEAN_CHANNELS],
    gradients: [[f64; 3]; MAX_BOOLEAN_CHANNELS],
}

struct Projector<'a, F> {
    sample: &'a F,
    lattice: &'a Lattice,
    grid_values: &'a [f32],
    channels: usize,
    used: u128,
    min: Vec3,
    max: Vec3,
    spacing: Vec3,
    options: &'a BooleanExtractionOptions,
    work: &'a mut WorkBudget,
}

impl<F: Fn(Vec3, &mut [f32])> Projector<'_, F> {
    fn values(&mut self, p: Vec3) -> Result<[f32; MAX_BOOLEAN_CHANNELS], String> {
        charge_field(self.work)?;
        let mut values = [f32::NAN; MAX_BOOLEAN_CHANNELS];
        (self.sample)(p, &mut values[..self.channels]);
        if values[..self.channels].iter().any(|v| !v.is_finite()) {
            return Err(format!("Boolean projection callback returned a nonfinite channel at {p:?}"));
        }
        Ok(values)
    }

    fn evaluation(&mut self, p: Vec3) -> Result<Evaluation, String> {
        let values = self.values(p)?;
        let mut gradients = [[0.0; 3]; MAX_BOOLEAN_CHANNELS];
        let coordinates = coords(p);
        let cell = coords(self.spacing).into_iter().fold(f32::INFINITY, f32::min);
        for axis in 0..3 {
            let h = (f64::from(cell) * 0.01)
                .max(f64::from(coordinates[axis].abs().max(1.0)) * f64::from(f32::EPSILON) * 8.0);
            let mut plus = coordinates;
            let mut minus = coordinates;
            plus[axis] = (f64::from(coordinates[axis]) + h) as f32;
            minus[axis] = (f64::from(coordinates[axis]) - h) as f32;
            let span = f64::from(plus[axis]) - f64::from(minus[axis]);
            if !span.is_finite() || span <= 0.0 {
                return Err("Boolean projection gradient probes are not representable".into());
            }
            let a = self.values(Vec3::new(plus[0], plus[1], plus[2]))?;
            let b = self.values(Vec3::new(minus[0], minus[1], minus[2]))?;
            for channel in 0..self.channels {
                gradients[channel][axis] = (f64::from(a[channel]) - f64::from(b[channel])) / span;
            }
        }
        Ok(Evaluation { point: p, values, gradients })
    }

    fn newton(&mut self, start: Vec3, origin: [f64; 3], targets: &[(usize, f64)]) -> Result<Evaluation, String> {
        let mut point = start;
        let max_move = f64::from(self.spacing.length()) * 0.5;
        for _ in 0..self.options.max_projection_iterations {
            let state = self.evaluation(point)?;
            let coordinate_scale = coords(point).into_iter().map(f32::abs).fold(1.0, f32::max);
            let tolerance = (f64::from(coordinate_scale) * f64::from(f32::EPSILON)).max(1e-7);
            let mut step = [0.0; 3];
            let mut basis: Vec<[f64; 3]> = Vec::new();
            let mut error = 0.0f64;
            for &(channel, target) in targets {
                charge(self.work, 16)?;
                let gradient = state.gradients[channel];
                let magnitude = dot(gradient, gradient).sqrt();
                if !magnitude.is_finite() || magnitude <= 1e-15 {
                    return Err("Boolean projection encountered a zero/nonfinite constrained gradient".into());
                }
                let normal = scale(gradient, 1.0 / magnitude);
                let residual = (f64::from(state.values[channel]) - target) / magnitude;
                error = error.max(residual.abs());
                let mut orthogonal = normal;
                for &axis in &basis {
                    orthogonal = sub(orthogonal, scale(axis, dot(normal, axis)));
                }
                let squared = dot(orthogonal, orthogonal);
                let correction = -residual - dot(normal, step);
                if squared > 1e-6 {
                    step = add(step, scale(orthogonal, correction / squared));
                    basis.push(scale(orthogonal, 1.0 / squared.sqrt()));
                } else if correction.abs() > tolerance * 2.0 {
                    return Err("Boolean projection constraints are locally singular or incompatible".into());
                }
            }
            if error <= tolerance {
                return Ok(state);
            }
            let length = dot(step, step).sqrt();
            if !length.is_finite() {
                return Err("Boolean projection step is nonfinite".into());
            }
            if length > max_move {
                step = scale(step, max_move / length);
            }
            let next = add(doubles(point), step);
            let next = Vec3::new(next[0] as f32, next[1] as f32, next[2] as f32);
            let displacement = sub(doubles(next), origin);
            if dot(displacement, displacement).sqrt() > max_move
                || (0..3).any(|i| coords(next)[i] <= coords(self.min)[i] || coords(next)[i] >= coords(self.max)[i])
            {
                return Err(format!("Boolean projection would leave its bounded neighborhood or export bounds: {point:?} -> {next:?}, targets {targets:?}"));
            }
            if next == point {
                return Err("Boolean projection reached native coordinate precision before convergence".into());
            }
            point = next;
        }
        Err(format!(
            "Boolean projection did not converge within its iteration budget at {point:?}, targets {targets:?}"
        ))
    }

    fn affine(&self, source: &SourcePoint, channel: usize) -> f64 {
        (0..4)
            .filter(|&i| source.nodes[i] != u32::MAX)
            .map(|i| {
                source.weights[i] * f64::from(self.grid_values[source.nodes[i] as usize * self.channels + channel])
            })
            .sum()
    }

    fn project(&mut self, source: &SourcePoint, constraints: u128) -> Result<Vec3, String> {
        let primary: Vec<_> =
            (0..self.channels).filter(|i| constraints & (1u128 << i) != 0).map(|i| (i, 0.0)).collect();
        if primary.is_empty() {
            return Err("Boolean projection vertex has no source leaf".into());
        }
        let p = source.point;
        let start = Vec3::new(p[0] as f32, p[1] as f32, p[2] as f32);
        let state = self.newton(start, source.point, &primary)?;
        let mut basis = Vec::new();
        for &(channel, _) in &primary {
            independent_axis(state.gradients[channel], &mut basis);
        }
        let band = f64::from(self.spacing.length()) * 2.0;
        let mut nearby = Vec::new();
        for channel in 0..self.channels {
            if self.used & (1u128 << channel) == 0 || constraints & (1u128 << channel) != 0 {
                continue;
            }
            let magnitude = dot(state.gradients[channel], state.gradients[channel]).sqrt();
            if !magnitude.is_finite() || magnitude <= 1e-15 {
                continue;
            }
            let original = self.affine(source, channel);
            let distance = original.abs() / magnitude;
            if distance < band {
                let weight = 1.0 - distance / band;
                let target = weight * original + (1.0 - weight) * f64::from(state.values[channel]);
                nearby.push((distance, channel, target));
            }
        }
        nearby.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        let mut targets = primary;
        for (_, channel, target) in nearby {
            if basis.len() == 3 {
                break;
            }
            if independent_axis(state.gradients[channel], &mut basis) {
                targets.push((channel, target));
            }
        }
        if targets.len() == constraints.count_ones() as usize {
            return Ok(state.point);
        }
        Ok(self.newton(state.point, source.point, &targets)?.point)
    }

    fn source_midpoint(&self, a: &SourcePoint, b: &SourcePoint) -> Result<SourcePoint, String> {
        let point = scale(add(a.point, b.point), 0.5);
        let mut cell = [0usize; 3];
        let mut fractions = [0.0; 3];
        for axis in 0..3 {
            let values = &self.lattice.axes[axis];
            let upper = values.partition_point(|&v| f64::from(v) <= point[axis]);
            if upper == 0 || upper >= values.len() {
                return Err("Boolean refinement source midpoint is outside the lattice".into());
            }
            cell[axis] = upper - 1;
            fractions[axis] = (point[axis] - f64::from(values[upper - 1]))
                / (f64::from(values[upper]) - f64::from(values[upper - 1]));
        }
        let mut order = [0, 1, 2];
        order.sort_by(|&a, &b| fractions[b].total_cmp(&fractions[a]).then(a.cmp(&b)));
        let sorted = order.map(|i| fractions[i]);
        let weights = [1.0 - sorted[0], sorted[0] - sorted[1], sorted[1] - sorted[2], sorted[2]];
        let mut nodes = [0u32; 4];
        for i in 0..4 {
            if i > 0 {
                cell[order[i - 1]] += 1;
            }
            nodes[i] = self.lattice.index(cell[0], cell[1], cell[2]) as u32;
        }
        Ok(SourcePoint { point, nodes, weights, affine: None })
    }

    fn backwards(&mut self, output: &Output, expr: &BooleanExpr) -> Result<BTreeSet<(u32, u32)>, String> {
        let mut marked = BTreeSet::new();
        for &[a, b, c] in &output.triangles {
            charge(self.work, 1)?;
            let points = [a, b, c].map(|i| output.positions[i as usize]);
            let normal =
                cross(sub(doubles(points[1]), doubles(points[0])), sub(doubles(points[2]), doubles(points[0])));
            let length = dot(normal, normal).sqrt();
            if length == 0.0 {
                continue;
            }
            let normal = scale(normal, 1.0 / length).map(|v| v as f32);
            let center = (points[0] + points[1] + points[2]) * (1.0 / 3.0);
            // Exact source-fidelity probe contract used by independent delivery
            // acceptance, with f64 offsets rounded once at the native boundary.
            let plus = std::array::from_fn::<_, 3, _>(|i| {
                (f64::from(coords(center)[i]) + f64::from(normal[i]) * self.options.normal_probe_distance_m) as f32
            });
            let minus = std::array::from_fn::<_, 3, _>(|i| {
                (f64::from(coords(center)[i]) - f64::from(normal[i]) * self.options.normal_probe_distance_m) as f32
            });
            if plus == minus || plus.iter().chain(&minus).any(|v| !v.is_finite()) {
                return Err("Boolean normal probes are not representable at this world scale".into());
            }
            let plus = self.values(Vec3::new(plus[0], plus[1], plus[2]))?;
            let minus = self.values(Vec3::new(minus[0], minus[1], minus[2]))?;
            let plus = expr.scalar_with_work(&plus[..self.channels], self.work)?;
            let minus = expr.scalar_with_work(&minus[..self.channels], self.work)?;
            if f64::from(plus) + self.options.normal_comparison_tolerance < f64::from(minus) {
                // Red refinement shortens every side of a narrow curved-boundary
                // triangle. Neighboring green subdivisions keep all edges shared.
                for (i, j) in [(a, b), (b, c), (c, a)] {
                    marked.insert((i.min(j), i.max(j)));
                }
            }
        }
        Ok(marked)
    }
}

fn independent_axis(gradient: [f64; 3], basis: &mut Vec<[f64; 3]>) -> bool {
    let length = dot(gradient, gradient).sqrt();
    if !length.is_finite() || length <= 1e-15 {
        return false;
    }
    let normal = scale(gradient, 1.0 / length);
    let mut residual = normal;
    for &axis in basis.iter() {
        residual = sub(residual, scale(axis, dot(normal, axis)));
    }
    let squared = dot(residual, residual);
    if squared > 1e-6 && basis.len() < 3 {
        basis.push(scale(residual, 1.0 / squared.sqrt()));
        true
    } else {
        false
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn run(
    output: &mut Output,
    lattice: &Lattice,
    grid_values: &[f32],
    channels: usize,
    used: u128,
    expr: &BooleanExpr,
    sample: &impl Fn(Vec3, &mut [f32]),
    options: &BooleanExtractionOptions,
    work: &mut WorkBudget,
    min: Vec3,
    max: Vec3,
    spacing: Vec3,
) -> Result<Report, String> {
    let mut projector = Projector { sample, lattice, grid_values, channels, used, min, max, spacing, options, work };
    let mut report = Report::default();
    // Previously rounded-flat faces may regain representable area after a
    // provenance-preserving move; reconsider them before final strict cleanup.
    output.triangles.append(&mut output.collapsed);
    for i in 0..output.positions.len() {
        let next = projector.project(&output.sources[i], output.constraints[i])?;
        if next != output.positions[i] {
            report.projected_vertices += 1;
        }
        let d = sub(doubles(next), output.sources[i].point);
        report.maximum_displacement = report.maximum_displacement.max(dot(d, d).sqrt() as f32);
        output.positions[i] = next;
    }
    for pass in 0..=options.max_refinement_passes {
        let marked = projector.backwards(output, expr)?;
        if marked.is_empty() {
            let mut valid = Vec::with_capacity(output.triangles.len());
            for tri in output.triangles.drain(..) {
                let [a, b, c] = tri.map(|i| doubles(output.positions[i as usize]));
                let n = cross(sub(b, a), sub(c, a));
                if dot(n, n) == 0.0 {
                    output.collapsed.push(tri);
                } else {
                    valid.push(tri);
                }
            }
            output.triangles = valid;
            return Ok(report);
        }
        if pass == options.max_refinement_passes {
            return Err("Boolean source-normal fidelity did not converge within refinement budget".into());
        }
        let mut midpoints = BTreeMap::new();
        for (a, b) in marked {
            charge(projector.work, 1)?;
            if output.positions.len() >= MAX_MESH_VERTICES {
                return Err("Boolean refinement vertex budget exceeded".into());
            }
            let source = projector.source_midpoint(&output.sources[a as usize], &output.sources[b as usize])?;
            let constraints = output.constraints[a as usize] & output.constraints[b as usize];
            let point = projector.project(&source, constraints)?;
            let d = sub(doubles(point), source.point);
            report.maximum_displacement = report.maximum_displacement.max(dot(d, d).sqrt() as f32);
            let index = output.positions.len() as u32;
            output.positions.push(point);
            output.sources.push(source);
            output.constraints.push(constraints);
            midpoints.insert((a, b), index);
            report.added_vertices += 1;
            report.projected_vertices += 1;
        }
        let mut refined = Vec::new();
        for tri in &output.triangles {
            let [a, b, c] = *tri;
            let midpoint = [(a, b), (b, c), (c, a)].map(|(i, j)| midpoints.get(&(i.min(j), i.max(j))).copied());
            let count = midpoint.iter().filter(|m| m.is_some()).count();
            let mut append = |triangle| -> Result<(), String> {
                charge(projector.work, 1)?;
                if refined.len() >= MAX_MESH_TRIANGLES {
                    return Err("Boolean refinement triangle budget exceeded".into());
                }
                refined.push(triangle);
                Ok(())
            };
            match count {
                0 => append(*tri)?,
                3 => {
                    let [ab, bc, ca] = midpoint.map(Option::unwrap);
                    for triangle in [[a, ab, ca], [ab, b, bc], [ca, bc, c], [ab, bc, ca]] {
                        append(triangle)?;
                    }
                }
                1 => {
                    let i = midpoint.iter().position(Option::is_some).unwrap();
                    let [a, b, c] = [tri[i], tri[(i + 1) % 3], tri[(i + 2) % 3]];
                    let m = midpoint[i].unwrap();
                    append([a, m, c])?;
                    append([m, b, c])?;
                }
                _ => {
                    let i = midpoint.iter().position(Option::is_none).unwrap();
                    let [a, b, c] = [tri[i], tri[(i + 1) % 3], tri[(i + 2) % 3]];
                    let bc = midpoint[(i + 1) % 3].unwrap();
                    let ca = midpoint[(i + 2) % 3].unwrap();
                    for triangle in [[c, ca, bc], [a, b, ca], [b, bc, ca]] {
                        append(triangle)?;
                    }
                }
            }
        }
        report.added_triangles += refined.len() - output.triangles.len();
        output.triangles = refined;
        report.passes += 1;
    }
    unreachable!("bounded refinement returns on success or final pass")
}
