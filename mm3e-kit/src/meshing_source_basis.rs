//! Immutable, node-bound exact basis proofs for source-plane queries.
//! Original finite f32 coefficients are mandatory. A nonzero determinant is
//! established before a zero/proportional query can reuse the basis equations.
//! All unmatched queries keep their complete expansion/FMA sign calculation.
use super::*;

#[derive(Clone, Copy)]
struct BasisInterval {
    lo: f64,
    hi: f64,
}
impl BasisInterval {
    fn point(value: f64) -> Self {
        Self { lo: value, hi: value }
    }
    fn add(self, other: Self) -> Self {
        Self { lo: (self.lo + other.lo).next_down(), hi: (self.hi + other.hi).next_up() }
    }
    fn sub(self, other: Self) -> Self {
        Self { lo: (self.lo - other.hi).next_down(), hi: (self.hi - other.lo).next_up() }
    }
    fn mul(self, other: Self) -> Self {
        let products = [self.lo * other.lo, self.lo * other.hi, self.hi * other.lo, self.hi * other.hi];
        Self {
            lo: products.into_iter().fold(f64::INFINITY, f64::min).next_down(),
            hi: products.into_iter().fold(f64::NEG_INFINITY, f64::max).next_up(),
        }
    }
}

fn basis_determinant_interval(local: &[usize], rows: &[[f64; 4]]) -> BasisInterval {
    let mut matrix = [[BasisInterval::point(0.); 3]; 3];
    for (row_index, row) in rows.iter().enumerate() {
        for (column, &node) in local.iter().skip(1).enumerate() {
            matrix[row_index][column] = BasisInterval::point(row[node]).sub(BasisInterval::point(row[local[0]]));
        }
    }
    if local.len() == 3 {
        return matrix[0][0].mul(matrix[1][1]).sub(matrix[0][1].mul(matrix[1][0]));
    }
    matrix[0][0]
        .mul(matrix[1][1].mul(matrix[2][2]).sub(matrix[1][2].mul(matrix[2][1])))
        .sub(matrix[0][1].mul(matrix[1][0].mul(matrix[2][2]).sub(matrix[1][2].mul(matrix[2][0]))))
        .add(matrix[0][2].mul(matrix[1][0].mul(matrix[2][1]).sub(matrix[1][1].mul(matrix[2][0]))))
}

pub(super) struct ExactBasisProof {
    nodes: [u32; 4],
    local: Vec<usize>,
    signed_cofactors: Vec<Vec<f64>>,
    basis_rows: Vec<[f64; 4]>,
    denominator_negative: bool,
}
impl Clone for AffineSource {
    fn clone(&self) -> Self {
        // A deep clone may subsequently be edited before Arc ownership.
        // Only ordinary Arc clones share the immutable proof.
        Self {
            tetra: self.tetra,
            points: self.points,
            basis: self.basis,
            planes: self.planes.clone(),
            exact_basis: Default::default(),
        }
    }
}
impl ExactBasisProof {
    fn sign(&self, row: [f64; 4], work: &mut WorkBudget) -> Result<i8, String> {
        charge(work, self.local.len() * 3 + 1)?;
        for &node in &self.local {
            if !row[node].is_finite() || f64::from(row[node] as f32) != row[node] {
                return Err("cached source plane query requires original finite f32 samples".into());
            }
        }
        if self.local.iter().all(|&i| row[i] == 0.) {
            return Ok(0);
        }
        // Uniqueness was established before publishing this basis proof.
        // Actual coefficient proportionality to an independent basis equation
        // therefore proves zero, regardless of IDs/nonbasis support claims.
        for basis in &self.basis_rows {
            charge(work, self.local.len() * 4 + 1)?;
            let pivot = *self.local.iter().find(|&&i| basis[i] != 0.).unwrap();
            if self.local.iter().all(|&i| row[i] * basis[pivot] == basis[i] * row[pivot]) {
                return Ok(0);
            }
        }
        if self.signed_cofactors.is_empty() {
            charge(work, if self.local.len() == 3 { 256 } else { 4096 })?;
            let basis: Vec<_> = (0..self.basis_rows.len()).collect();
            return exact_plane_sign(&self.local, &basis, &self.basis_rows, row);
        }
        let mut numerator = Vec::new();
        for (&node, components) in self.local.iter().zip(&self.signed_cofactors) {
            for &component in components {
                charge(work, 2)?;
                let product = component * row[node];
                let error = component.mul_add(row[node], -product);
                charge(work, 8 * (numerator.len() + 1))?;
                numerator = grow_expansion(&numerator, error);
                charge(work, 8 * (numerator.len() + 1))?;
                numerator = grow_expansion(&numerator, product);
            }
        }
        Ok(numerator
            .last()
            .map_or(0, |value| if value.is_sign_negative() == self.denominator_negative { 1 } else { -1 }))
    }
}
pub(super) fn source_plane_sign(
    source: &SourcePoint,
    nodes: &[u32],
    query: &[f64],
    work: &mut WorkBudget,
) -> Result<i8, String> {
    charge(work, 1)?;
    // Production source supports are sorted and padded. Preserve the cheap
    // legacy path without building its local support vector twice.
    if source.nodes[2] == u32::MAX {
        return source_plane_sign_uncached(source, nodes, query, work);
    }
    let affine = source.affine.as_ref().ok_or("convex source identity lacks original samples")?;
    let local: Vec<_> = source
        .nodes
        .iter()
        .filter(|&&id| id != u32::MAX)
        .map(|id| affine.tetra.iter().position(|node| node == id).ok_or("source identity lost lattice support"))
        .collect::<Result<_, _>>()?;
    if local.len() < 3 {
        return source_plane_sign_uncached(source, nodes, query, work);
    }
    charge(work, 5 + local.len())?;
    let mut row = [0.; 4];
    for &i in &local {
        let column =
            nodes.iter().position(|id| *id == affine.tetra[i]).ok_or("source identity queries a different simplex")?;
        row[i] = query[column];
    }
    if let Some(proof) = affine.exact_basis.get().filter(|p| p.nodes == source.nodes) {
        return proof.sign(row, work);
    }
    let mut basis_rows = Vec::new();
    for id in affine.basis.iter().filter(|&&id| id != u64::MAX) {
        charge(work, 1)?;
        let mut found = None;
        for (other, values) in &affine.planes {
            charge(work, 1)?;
            if other == id {
                charge(work, 4)?;
                found = Some(*values);
                break;
            }
        }
        basis_rows.push(found.ok_or("source identity lost its basis")?);
    }
    if basis_rows.len() + 1 != local.len() {
        return Err("convex exact sign has inconsistent source rank".into());
    }
    charge(work, basis_rows.len() * local.len() * 3)?;
    for values in &basis_rows {
        for &c in &local {
            if !values[c].is_finite() || f64::from(values[c] as f32) != values[c] {
                return Err("cached source basis requires original finite f32 samples".into());
            }
        }
    }
    charge(work, if local.len() == 3 { 64 } else { 256 })?;
    let interval = basis_determinant_interval(&local, &basis_rows);
    let filtered = interval.lo.is_finite() && interval.hi.is_finite() && (interval.lo > 0. || interval.hi < 0.);
    let (signed_cofactors, denominator_negative) = if filtered {
        (Vec::new(), interval.hi < 0.)
    } else {
        charge(work, if local.len() == 3 { 256 } else { 4096 })?;
        let basis: Vec<_> = (0..basis_rows.len()).collect();
        let mut denominator = Vec::new();
        let mut signed_cofactors = Vec::with_capacity(local.len());
        for column in 0..local.len() {
            let sign = if column % 2 == 0 { 1. } else { -1. };
            let components: Vec<_> =
                exact_cofactor(&local, &basis, column, &basis_rows).into_iter().map(|v| sign * v).collect();
            for &component in &components {
                denominator = grow_expansion(&denominator, component);
            }
            signed_cofactors.push(components);
        }
        let negative = denominator.last().ok_or("convex exact sign has a zero source determinant")?.is_sign_negative();
        (signed_cofactors, negative)
    };
    let proof = ExactBasisProof { nodes: source.nodes, local, signed_cofactors, basis_rows, denominator_negative };
    match affine.exact_basis.set(proof) {
        Ok(()) => affine.exact_basis.get().unwrap().sign(row, work),
        Err(proof) => proof.sign(row, work),
    }
}
