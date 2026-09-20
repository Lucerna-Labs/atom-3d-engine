"""Independent Fraction 4x4 determinant oracle; no Rust predicate is reused."""
from fractions import Fraction as F
from pathlib import Path
import hashlib
import itertools
import json
import math
import random
import struct

ROOT = Path(__file__).resolve().parent
PERMUTATIONS = list(itertools.permutations(range(4)))


def sign(value):
    return (value > 0) - (value < 0)


def determinant4(rows):
    total = F(0)
    for permutation in PERMUTATIONS:
        inversions = sum(permutation[i] > permutation[j] for i in range(4) for j in range(i + 1, 4))
        product = F(1)
        for i, j in enumerate(permutation):
            product *= rows[i][j]
        total += product if inversions % 2 == 0 else -product
    return total


def integer_terms(value, exponent):
    negative = value < 0
    value = abs(value)
    terms = []
    while value:
        limb = value & 0xffffffff
        if limb:
            term = math.ldexp(float(-limb if negative else limb), exponent)
            assert term and math.isfinite(term)
            terms.append(term)
        value >>= 32
        exponent += 32
    return terms


def main():
    lines = ['# exact-geometry-3d-fraction-v1']
    points = []
    queries = 0
    signs = {-1: 0, 0: 0, 1: 0}
    groups = {}

    def point(name, rows):
        assert len(rows) == 4 and 0 < sum(map(len, rows)) <= 1024
        assert all(math.isfinite(x) for row in rows for x in row)
        homogeneous = [sum(map(F, row), F(0)) for row in rows]
        assert homogeneous[3]
        coordinates = tuple(x / homogeneous[3] for x in homogeneous[:3])
        index = len(points)
        points.append(coordinates)
        lines.append(','.join(['P', name] + [';'.join(struct.pack('>d', x).hex() for x in row) for row in rows]))
        return index

    def rational(name, xyz, homogeneous_scale=1):
        xyz = tuple(map(F, xyz))
        denominator = math.lcm(*(v.denominator for v in xyz))
        integers = [int(v * denominator) * homogeneous_scale for v in xyz] + [denominator * homogeneous_scale]
        exponent = min(0, 1024 - max(abs(v).bit_length() for v in integers))
        index = point(name, [integer_terms(v, exponent) for v in integers])
        assert points[index] == xyz, (name, xyz, points[index])
        return index

    def orientation(name, ids):
        nonlocal queries
        rows = [list(points[i]) + [F(1)] for i in ids]
        result = sign(determinant4(rows))
        lines.append(','.join(map(str, ['O3', name, *ids, result])))
        queries += 1
        signs[result] += 1
        return result

    def permutations(name, ids, expected=None):
        base = orientation(name + '_base', ids)
        if expected is not None:
            assert base == expected, (name, base, expected)
        for permutation in PERMUTATIONS:
            parity = (-1) ** sum(permutation[i] > permutation[j] for i in range(4) for j in range(i + 1, 4))
            assert orientation(name + '_p' + ''.join(map(str, permutation)), [ids[i] for i in permutation]) == base * parity

    origin = point('origin', [[], [], [], [1.]])
    # Both below-underflow and above-overflow ordinary floating determinants.
    for exponent in range(-1074, 1024):
        value = math.ldexp(1., exponent)
        x = point(f'axis_x_{exponent}', [[value], [], [], [1.]])
        y = point(f'axis_y_{exponent}', [[], [value], [], [1.]])
        z = point(f'axis_z_{exponent}', [[], [], [value], [1.]])
        assert orientation(f'exponent_{exponent}', [origin, x, y, z]) == -1
        assert orientation(f'exponent_reversed_{exponent}', [origin, y, x, z]) == 1
    groups['all_binary64_exponents'] = 2098

    tiny = F(math.ldexp(1., -1074))
    huge = float.fromhex('0x1.fffffffffffffp+1023')
    p = point('cancel_to_smallest_subnormal', [[huge, huge, -huge, -huge, float(tiny)], [], [], [huge, -huge, 1.]])
    y = point('cancel_y', [[], [huge, -huge, 1.], [], [-1., 2.]])
    z = point('cancel_z', [[], [], [1.], [1.]])
    permutations('overflow_cancel_volume', [origin, p, y, z], -1)
    many = point('maximum_1024_terms', [[math.ldexp(1., 1023)] * 511 + [-math.ldexp(1., 1023)] * 511 + [float(tiny)], [], [], [1.]])
    permutations('maximum_terms_volume', [origin, many, y, z], -1)
    for index, (a, b, c) in enumerate([(1023, -1074, 1023), (-1074, 1023, -1074), (-1022, 1023, 0), (1000, -1000, 0)]):
        x = point(f'mixed_x_{index}', [[math.ldexp(1., a)], [], [], [1.]])
        y = point(f'mixed_y_{index}', [[], [math.ldexp(1., b)], [], [1.]])
        z = point(f'mixed_z_{index}', [[], [], [math.ldexp(1., c)], [1.]])
        permutations(f'mixed_exponents_{index}', [origin, x, y, z], -1)
    extreme = [point('beyond_x', [[huge], [], [], [float(tiny)]]),
               point('beyond_y_negative_scale', [[], [-huge], [], [-float(tiny)]]),
               point('beyond_z', [[], [], [huge], [float(tiny)]])]
    permutations('coordinates_beyond_binary64', [origin, *extreme], -1)
    groups['cancellation_and_mixed_extremes'] = 7

    # Rational unit quaternions define exactly orthogonal SO(3) transforms.
    # This avoids testing approximate coplanarity produced by rounded trig.
    rng = random.Random(0x0A13_4D5E)
    for case in range(16):
        w, x, y, z = [rng.randrange(-19, 20) for _ in range(4)]
        norm = w*w + x*x + y*y + z*z
        assert norm
        rotation = [[F(w*w+x*x-y*y-z*z, norm), F(2*(x*y-w*z), norm), F(2*(x*z+w*y), norm)],
                    [F(2*(x*y+w*z), norm), F(w*w-x*x+y*y-z*z, norm), F(2*(y*z-w*x), norm)],
                    [F(2*(x*z-w*y), norm), F(2*(y*z+w*x), norm), F(w*w-x*x-y*y+z*z, norm)]]
        assert all(sum(rotation[i][k]*rotation[j][k] for k in range(3)) == (i == j) for i in range(3) for j in range(3))
        shift = [F(rng.randrange(-100, 101), rng.randrange(1, 20)) for _ in range(3)]
        def transform(v):
            return [shift[i] + sum(rotation[i][j]*v[j] for j in range(3)) for i in range(3)]
        xyz = [[F(0), F(0), F(0)], [F(2), F(0), F(0)], [F(0), F(3), F(0)], [F(2), F(3), F(0)]]
        for label, departure in [('coplanar', F(0)), ('above', tiny), ('below', -tiny)]:
            quad = [v[:] for v in xyz]
            quad[3][2] = departure
            ids = [rational(f'rotation_{case}_{label}_{i}', transform(v), [-3, 5, -7, 11][i]) for i, v in enumerate(quad)]
            permutations(f'rotation_{case}_{label}', ids, -sign(departure))
    groups['arbitrary_rational_rotations_with_negative_homogeneous_scales'] = 16

    # Huge rational quad coordinates, with non-axis-aligned source directions.
    # Exact determinants cancel across long products, leaving one subnormal.
    rng = random.Random(0x5EED_3D11)
    for case in range(16):
        denominator = rng.getrandbits(160) | 1
        base = [F((-1 if axis == 1 else 1)*rng.getrandbits(440), denominator) for axis in range(3)]
        u = [F(1), F(2), F(3)]
        v = [F(4), F(-1), F(2)]
        points_xyz = [base, [base[i]+u[i] for i in range(3)], [base[i]+v[i] for i in range(3)]]
        for label, departure in [('zero', F(0)), ('plus', tiny), ('minus', -tiny)]:
            fourth = [base[i] + u[i] + v[i] for i in range(3)]
            fourth[2] += departure
            ids = [rational(f'long_quad_{case}_{label}_{i}', p, [-1, 3, -5, 7][i]) for i, p in enumerate(points_xyz + [fourth])]
            permutations(f'long_quad_{case}_{label}', ids, sign(departure))
        # Four collinear points remain coplanar after one point departs from the
        # line: three remaining points are still collinear. Do not invent volume.
        collinear = [[base[i] + k*u[i] for i in range(3)] for k in range(4)]
        collinear[3][2] += tiny
        ids = [rational(f'long_line_{case}_{i}', p) for i, p in enumerate(collinear)]
        permutations(f'long_line_{case}', ids, 0)
    groups['long_rational_quad_and_collinear_families'] = 16

    # Signed mixed-exponent inputs independently test multiplication carry/borrow.
    rng = random.Random(0x4DCA_221E)
    for case in range(32):
        ids = []
        for vertex in range(4):
            rows = []
            for axis in range(4):
                terms = []
                for _ in range(10):
                    exponent = rng.choice([-1074, -1022, -500, -32, 0, 31, 500, 970])
                    mantissa = rng.choice([1, 3, 0xffff, 0xffffffff])
                    value = math.ldexp(float(mantissa), exponent)
                    terms.append(value if rng.randrange(2) else -value)
                rows.append(terms)
            if not sum(map(F, rows[3]), F(0)):
                rows[3].append(1.)
            ids.append(point(f'carry_{case}_{vertex}', rows))
        permutations(f'signed_carry_{case}', ids)
    groups['signed_expansion_quadruples'] = 32

    data = '\n'.join(lines) + '\n'
    (ROOT / 'oracle.csv').write_text(data)
    manifest = {'format': 'exact-geometry-3d-fraction-v1', 'points': len(points), 'orientation_queries': queries,
                'sign_counts': signs, 'groups': groups, 'sign_convention': 'determinant4 rows [x,y,z,1]; det(a-d,b-d,c-d)',
                'generator_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                'oracle_sha256': hashlib.sha256(data.encode()).hexdigest(),
                'method': 'Python Fraction expansion sums and exact homogeneous division; independent 24-term determinant4 and all 24 point permutations'}
    (ROOT / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    print(json.dumps(manifest, indent=2))


if __name__ == '__main__':
    main()
