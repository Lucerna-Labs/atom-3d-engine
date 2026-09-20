//! Bounded, dependency-free predicates for rational source intersections.
//!
//! Coordinates are sums of finite binary64 expansion terms divided by a shared
//! denominator. Integer words retain every input bit; no coordinate division or
//! floating-point collinearity tolerance enters the fallback determinant.
use std::{cmp::Ordering, sync::Arc};

const MAX_TERMS: usize = 1024;
const MAX_WORDS: usize = 512;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Failure {
    pub work: usize,
    pub message: String,
}
struct Budget {
    used: usize,
    max: usize,
}
impl Budget {
    fn charge(&mut self, n: usize) -> Result<(), String> {
        if n > self.max.saturating_sub(self.used) {
            return Err("exact geometry exhausted work budget".into());
        }
        self.used += n;
        Ok(())
    }
}
#[derive(Clone, Debug, Default)]
struct Integer {
    negative: bool,
    words: Vec<u32>,
}
impl Integer {
    fn trim(&mut self) {
        while self.words.last() == Some(&0) {
            self.words.pop();
        }
        if self.words.is_empty() {
            self.negative = false;
        }
    }
    fn sign(&self) -> i8 {
        if self.words.is_empty() {
            0
        } else if self.negative {
            -1
        } else {
            1
        }
    }
    fn negate(&mut self) {
        if !self.words.is_empty() {
            self.negative = !self.negative;
        }
    }
    fn shifted(mantissa: u64, shift: usize, negative: bool, budget: &mut Budget) -> Result<Self, String> {
        if mantissa == 0 {
            budget.charge(1)?;
            return Ok(Self::default());
        }
        let n = ((u64::BITS - mantissa.leading_zeros()) as usize + shift).div_ceil(32);
        if n > MAX_WORDS {
            return Err("exact geometry integer exceeds word bound".into());
        }
        budget.charge(n + 4)?;
        let mut words = vec![0; n];
        let offset = shift / 32;
        let value = u128::from(mantissa) << (shift % 32);
        for (i, word) in words.iter_mut().enumerate().skip(offset) {
            *word = (value >> (32 * (i - offset))) as u32;
        }
        Ok(Self { negative, words })
    }
    fn magnitude_cmp(&self, b: &Self) -> Ordering {
        self.words.len().cmp(&b.words.len()).then_with(|| self.words.iter().rev().cmp(b.words.iter().rev()))
    }
    fn add(&self, b: &Self, budget: &mut Budget) -> Result<Self, String> {
        let n = self.words.len().max(b.words.len());
        if n + 1 > MAX_WORDS {
            return Err("exact geometry sum exceeds word bound".into());
        }
        budget.charge((n + 1) * 4)?;
        if self.words.is_empty() {
            return Ok(b.clone());
        }
        if b.words.is_empty() {
            return Ok(self.clone());
        }
        let mut result = Self { negative: self.negative, words: vec![0; n + 1] };
        if self.negative == b.negative {
            let mut carry = 0u64;
            for i in 0..n {
                let sum =
                    u64::from(*self.words.get(i).unwrap_or(&0)) + u64::from(*b.words.get(i).unwrap_or(&0)) + carry;
                result.words[i] = sum as u32;
                carry = sum >> 32;
            }
            result.words[n] = carry as u32;
        } else {
            let (large, small) = match self.magnitude_cmp(b) {
                Ordering::Equal => return Ok(Self::default()),
                Ordering::Greater => (self, b),
                Ordering::Less => (b, self),
            };
            result.negative = large.negative;
            let mut borrow = 0u64;
            for i in 0..n {
                let a = u64::from(*large.words.get(i).unwrap_or(&0));
                let b = u64::from(*small.words.get(i).unwrap_or(&0)) + borrow;
                if a >= b {
                    result.words[i] = (a - b) as u32;
                    borrow = 0;
                } else {
                    result.words[i] = ((1u64 << 32) + a - b) as u32;
                    borrow = 1;
                }
            }
        }
        result.trim();
        Ok(result)
    }
    fn mul(&self, b: &Self, budget: &mut Budget) -> Result<Self, String> {
        if self.words.is_empty() || b.words.is_empty() {
            budget.charge(1)?;
            return Ok(Self::default());
        }
        let n = self.words.len() + b.words.len();
        if n > MAX_WORDS {
            return Err("exact geometry product exceeds word bound".into());
        }
        budget.charge(self.words.len() * b.words.len() + n + 1)?;
        let mut result = Self { negative: self.negative != b.negative, words: vec![0; n] };
        for (i, &a) in self.words.iter().enumerate() {
            let mut carry = 0u64;
            for (j, &b) in b.words.iter().enumerate() {
                // The largest product + stored word + carry is u64::MAX.
                let value = u64::from(a) * u64::from(b) + u64::from(result.words[i + j]) + carry;
                result.words[i + j] = value as u32;
                carry = value >> 32;
            }
            result.words[i + b.words.len()] = carry as u32;
        }
        result.trim();
        Ok(result)
    }
}

#[derive(Clone, Copy, Debug)]
struct Interval {
    lo: f64,
    hi: f64,
}
impl Interval {
    fn sub(self, b: Self) -> Self {
        Self { lo: (self.lo - b.hi).next_down(), hi: (self.hi - b.lo).next_up() }
    }
    fn add(self, b: Self) -> Self {
        Self { lo: (self.lo + b.lo).next_down(), hi: (self.hi + b.hi).next_up() }
    }
    fn mul(self, b: Self) -> Self {
        let products = [self.lo * b.lo, self.lo * b.hi, self.hi * b.lo, self.hi * b.hi];
        if products.iter().any(|v| !v.is_finite()) {
            return Self { lo: f64::NEG_INFINITY, hi: f64::INFINITY };
        }
        Self {
            lo: products.into_iter().fold(f64::INFINITY, f64::min).next_down(),
            hi: products.into_iter().fold(f64::NEG_INFINITY, f64::max).next_up(),
        }
    }
    fn sign(self) -> Option<i8> {
        if !self.lo.is_finite() || !self.hi.is_finite() {
            None
        } else if self.lo > 0. {
            Some(1)
        } else if self.hi < 0. {
            Some(-1)
        } else {
            None
        }
    }
}
#[derive(Debug)]
struct Data {
    values: [Integer; 4],
    bounds: Option<[Interval; 3]>,
}
#[derive(Clone, Debug)]
pub struct Point {
    data: Arc<Data>,
}
#[derive(Clone, Debug)]
pub struct PointResult {
    pub point: Point,
    pub work: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignResult {
    pub sign: i8,
    pub work: usize,
    pub integer_fallback: bool,
}

fn decode(value: f64) -> Option<(bool, u64, i32)> {
    let bits = value.to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    let mut mantissa = bits & ((1u64 << 52) - 1);
    if exponent != 0 {
        mantissa |= 1u64 << 52;
    }
    if mantissa == 0 {
        return None;
    }
    let zeros = mantissa.trailing_zeros();
    mantissa >>= zeros;
    Some((bits >> 63 != 0, mantissa, (if exponent == 0 { -1074 } else { exponent - 1023 - 52 }) + zeros as i32))
}
impl Point {
    /// Convert at most 1024 total finite expansion terms into a cached exact
    /// homogeneous point. Denominator sign is normalized; a zero sum rejects.
    pub fn from_expansions(
        numerators: [&[f64]; 3],
        denominator: &[f64],
        max_work: usize,
    ) -> Result<PointResult, Failure> {
        let mut budget = Budget { used: 0, max: max_work };
        let result = (|| -> Result<Point, String> {
            let rows = [numerators[0], numerators[1], numerators[2], denominator];
            let total = rows
                .iter()
                .try_fold(0usize, |sum, row| sum.checked_add(row.len()))
                .ok_or("exact geometry term bound overflow")?;
            if total == 0 || total > MAX_TERMS {
                return Err("exact geometry expansion term bound exceeded".into());
            }
            budget.charge(total * 5 + 4)?;
            let mut terms = Vec::new();
            let mut minimum = i32::MAX;
            let mut intervals = [Interval { lo: 0., hi: 0. }; 4];
            for (row, values) in rows.into_iter().enumerate() {
                for &value in values {
                    if !value.is_finite() {
                        return Err("exact geometry expansion terms must be finite".into());
                    }
                    intervals[row] = intervals[row].add(Interval { lo: value, hi: value });
                    if let Some((negative, mantissa, exponent)) = decode(value) {
                        minimum = minimum.min(exponent);
                        terms.push((row, negative, mantissa, exponent));
                    }
                }
            }
            let mut values: [Integer; 4] = std::array::from_fn(|_| Integer::default());
            for (row, negative, mantissa, exponent) in terms {
                let term = Integer::shifted(mantissa, (exponent - minimum) as usize, negative, &mut budget)?;
                values[row] = values[row].add(&term, &mut budget)?;
            }
            let sign = values[3].sign();
            if sign == 0 {
                return Err("exact geometry homogeneous denominator is zero".into());
            }
            if sign < 0 {
                for value in &mut values {
                    value.negate();
                }
            }
            let bounds = (|| -> Option<[Interval; 3]> {
                let denominator = intervals[3];
                if !denominator.lo.is_finite()
                    || !denominator.hi.is_finite()
                    || denominator.lo <= 0. && denominator.hi >= 0.
                {
                    return None;
                }
                let mut bounds = [Interval { lo: 0., hi: 0. }; 3];
                for axis in 0..3 {
                    let n = intervals[axis];
                    let q =
                        [n.lo / denominator.lo, n.lo / denominator.hi, n.hi / denominator.lo, n.hi / denominator.hi];
                    if q.iter().any(|v| !v.is_finite()) {
                        return None;
                    }
                    bounds[axis] = Interval {
                        lo: q.into_iter().fold(f64::INFINITY, f64::min).next_down(),
                        hi: q.into_iter().fold(f64::NEG_INFINITY, f64::max).next_up(),
                    };
                }
                Some(bounds)
            })();
            budget.charge(20)?;
            Ok(Point { data: Arc::new(Data { values, bounds }) })
        })();
        match result {
            Ok(point) => Ok(PointResult { point, work: budget.used }),
            Err(message) => Err(Failure { work: budget.used, message }),
        }
    }
}
fn finish(result: Result<(i8, bool), String>, budget: Budget) -> Result<SignResult, Failure> {
    result
        .map(|(sign, integer_fallback)| SignResult { sign, work: budget.used, integer_fallback })
        .map_err(|message| Failure { work: budget.used, message })
}
/// Compare exact rational coordinates. Returned sign is sign(a[axis]-b[axis]).
pub fn compare_axis(a: &Point, b: &Point, axis: usize, max_work: usize) -> Result<SignResult, Failure> {
    let mut budget = Budget { used: 0, max: max_work };
    let result = (|| -> Result<(i8, bool), String> {
        budget.charge(2)?;
        if axis >= 3 {
            return Err("exact geometry comparison axis is invalid".into());
        }
        if Arc::ptr_eq(&a.data, &b.data) {
            return Ok((0, false));
        }
        if let (Some(aa), Some(bb)) = (a.data.bounds, b.data.bounds) {
            if let Some(sign) = aa[axis].sub(bb[axis]).sign() {
                return Ok((sign, false));
            }
        }
        let left = a.data.values[axis].mul(&b.data.values[3], &mut budget)?;
        let mut right = b.data.values[axis].mul(&a.data.values[3], &mut budget)?;
        right.negate();
        Ok((left.add(&right, &mut budget)?.sign(), true))
    })();
    finish(result, budget)
}
/// Exact projected orientation; positive means counter-clockwise in `axes`.
pub fn orient2(points: [&Point; 3], axes: [usize; 2], max_work: usize) -> Result<SignResult, Failure> {
    let mut budget = Budget { used: 0, max: max_work };
    let result = (|| -> Result<(i8, bool), String> {
        budget.charge(16)?;
        let [x, y] = axes;
        if x >= 3 || y >= 3 || x == y {
            return Err("exact geometry projection axes are invalid".into());
        }
        if let [Some(a), Some(b), Some(c)] = points.map(|p| p.data.bounds) {
            let det = b[x].sub(a[x]).mul(c[y].sub(a[y])).sub(b[y].sub(a[y]).mul(c[x].sub(a[x])));
            if let Some(sign) = det.sign() {
                return Ok((sign, false));
            }
        }
        let columns = [x, y, 3];
        let mut sum = Integer::default();
        for (permutation, negative) in [
            ([0, 1, 2], false),
            ([1, 2, 0], false),
            ([2, 0, 1], false),
            ([0, 2, 1], true),
            ([2, 1, 0], true),
            ([1, 0, 2], true),
        ] {
            let first = points[0].data.values[columns[permutation[0]]]
                .mul(&points[1].data.values[columns[permutation[1]]], &mut budget)?;
            let mut term = first.mul(&points[2].data.values[columns[permutation[2]]], &mut budget)?;
            if negative {
                term.negate();
            }
            sum = sum.add(&term, &mut budget)?;
        }
        Ok((sum.sign(), true))
    })();
    finish(result, budget)
}

/// Sign of det([a-d,b-d,c-d]), equivalently homogeneous rows [x,y,z,1].
/// Positive means d is below the positively oriented plane abc. Zero certifies
/// exact coplanarity of the supplied rational coordinates.
pub fn orient3(points: [&Point; 4], max_work: usize) -> Result<SignResult, Failure> {
    let mut budget = Budget { used: 0, max: max_work };
    let result = (|| -> Result<(i8, bool), String> {
        budget.charge(32)?;
        if let [Some(a), Some(b), Some(c), Some(d)] = points.map(|p| p.data.bounds) {
            let v: [[Interval; 3]; 3] = [a, b, c].map(|p| std::array::from_fn(|axis| p[axis].sub(d[axis])));
            let cross = [
                v[1][1].mul(v[2][2]).sub(v[1][2].mul(v[2][1])),
                v[1][2].mul(v[2][0]).sub(v[1][0].mul(v[2][2])),
                v[1][0].mul(v[2][1]).sub(v[1][1].mul(v[2][0])),
            ];
            let determinant = v[0][0].mul(cross[0]).add(v[0][1].mul(cross[1])).add(v[0][2].mul(cross[2]));
            if let Some(sign) = determinant.sign() {
                return Ok((sign, false));
            }
        }
        let mut sum = Integer::default();
        for a in 0..4 {
            for b in 0..4 {
                if b == a {
                    continue;
                }
                for c in 0..4 {
                    if c == a || c == b {
                        continue;
                    }
                    let permutation = [a, b, c, 6 - a - b - c];
                    budget.charge(6)?;
                    let inversions =
                        (0..4).map(|i| (i + 1..4).filter(|&j| permutation[i] > permutation[j]).count()).sum::<usize>();
                    let first = points[0].data.values[permutation[0]]
                        .mul(&points[1].data.values[permutation[1]], &mut budget)?;
                    let second = first.mul(&points[2].data.values[permutation[2]], &mut budget)?;
                    let mut term = second.mul(&points[3].data.values[permutation[3]], &mut budget)?;
                    if inversions % 2 != 0 {
                        term.negate();
                    }
                    sum = sum.add(&term, &mut budget)?;
                }
            }
        }
        Ok((sum.sign(), true))
    })();
    finish(result, budget)
}
