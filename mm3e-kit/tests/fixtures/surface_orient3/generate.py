"""Fraction determinant4 signs for original finite f32 stored mesh points."""
from fractions import Fraction as F
from pathlib import Path
import hashlib,itertools,json,math,random,struct
ROOT=Path(__file__).resolve().parent
PERMS=list(itertools.permutations(range(4)))
def f32(v):return struct.unpack('<f',struct.pack('<f',v))[0]
def bits(v):return struct.pack('>f',v).hex()
def sgn(v):return (v>0)-(v<0)
def determinant4(p):
 rows=[list(map(F,q))+[F(1)] for q in p];total=F(0)
 for perm in PERMS:
  term=F(1)
  for i,j in enumerate(perm):term*=rows[i][j]
  total+=term*((-1)**sum(perm[i]>perm[j] for i in range(4) for j in range(i+1,4)))
 return total

def main():
 rows=['# surface-orient3-f32-fraction-v1'];counts={-1:0,0:0,1:0};groups={};naive_wrong=0
 def add(name,p):
  nonlocal naive_wrong
  p=[[f32(v) for v in q] for q in p];assert all(math.isfinite(v) for q in p for v in q)
  expected=sgn(determinant4(p));counts[expected]+=1
  a,b,c=[[q[i]-p[3][i] for i in range(3)] for q in p[:3]]
  rounded=a[0]*b[1]*c[2]+a[1]*b[2]*c[0]+a[2]*b[0]*c[1]-a[0]*b[2]*c[1]-a[1]*b[0]*c[2]-a[2]*b[1]*c[0]
  naive_wrong+=sgn(rounded)!=expected
  rows.append(','.join([name,*[bits(v) for q in p for v in q],str(expected)]))
 def permutations(name,p):
  for i,perm in enumerate(PERMS):add(f'{name}_p{i}',[p[j] for j in perm])
 for exponent in range(-149,128):
  t=math.ldexp(1.,exponent)
  add(f'exponent_{exponent}',[[0,0,0],[t,0,0],[0,t,0],[0,0,t]])
  add(f'exponent_reverse_{exponent}',[[0,0,0],[0,t,0],[t,0,0],[0,0,t]])
 groups['all_f32_binary_exponents']=277
 # The three large points are in a plane through the origin; moving d by one
 # subnormal leaves nonzero volume that rounded differences can erase entirely.
 for exponent in [-80,-32,0,32,64,100,125]:
  large=math.ldexp(1.,exponent)
  for departure in [0,math.ldexp(1.,-149),-math.ldexp(1.,-149)]:
   permutations(f'difference_tail_{exponent}_{sgn(departure)}',[[large,1,0],[2*large,0,1],[3*large,1,1],[departure,0,0]])
 groups['difference_tail_plane_families']=7
 # Exact differences can have more than26 significant bits, so their first
 # pair product also needs its FMA remainder. These quads share an oblique
 # plane; neighboring f32 departures prevent zero-only testing.
 for exponent in [-25,-29,-35,-43,-51,-60,-90,-120]:
  s=math.ldexp(1.,exponent)
  for offset in [-1,0,1]:
   x=struct.unpack('>f',(int(bits(s),16)+offset).to_bytes(4,'big'))[0]
   permutations(f'first_product_tail_{exponent}_{offset}',[[3,7,11],[2,-5,13],[5,2,24],[x,12*s,-2*s]])
 groups['first_product_residual_plane_families']=8
 # Products need their low part even when every difference itself is exact.
 rng=random.Random(0x53FD_3D11)
 for case in range(32):
  matrix=[[1,0,0],[0,1,0],[0,0,1]]
  for _ in range(100):
   a,b=rng.sample(range(3),2);m=rng.choice([-17,-7,-3,-1,1,3,7,17]);row=[x+m*y for x,y in zip(matrix[a],matrix[b])]
   if max(map(abs,row))<(1<<24):matrix[a]=row
  assert determinant4(matrix+[[0,0,0]])==1
  permutations(f'unimodular_{case}',matrix+[[0,0,0]])
 groups['unit_volume_large_integer_families']=32
 # General mixed-bit inputs exercise all low/high combinations and signs.
 for case in range(64):
  p=[]
  for _ in range(4):
   q=[]
   for _ in range(3):
    value=rng.randrange(1<<32)
    if (value>>23)&255==255:value^=1<<23
    q.append(struct.unpack('>f',value.to_bytes(4,'big'))[0])
   p.append(q)
  permutations(f'random_bits_{case}',p)
 groups['random_finite_quadruples']=64
 # All axes vary in these tilted coplanar inputs; neighboring one-ULP samples
 # on either side must not be rounded back into the plane.
 for exponent in [-120,-60,0,60,120]:
  t=math.ldexp(1.,exponent)
  p=[[0,0,0],[t,0,t],[0,t,-t],[t,t,0]]
  permutations(f'tilted_coplanar_{exponent}',p)
  for departure in [-math.ldexp(1.,-149),math.ldexp(1.,-149)]:
   changed=[q[:] for q in p];changed[3][2]=departure
   permutations(f'tilted_departure_{exponent}_{sgn(departure)}',changed)
 groups['tilted_coplanar_families']=5
 data='\n'.join(rows)+'\n';(ROOT/'oracle.csv').write_text(data)
 manifest={'format':'surface-orient3-f32-fraction-v1','queries':len(rows)-1,'sign_counts':counts,'groups':groups,'naive_f64_difference_determinant_wrong_signs':naive_wrong,'generator_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'oracle_sha256':hashlib.sha256(data.encode()).hexdigest(),'method':'independent 24-term Fraction determinant of original f32 coordinates, all24 permutations for cancellation/bit-pattern families'}
 assert naive_wrong>0
 (ROOT/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n');print(json.dumps(manifest,indent=2))
if __name__=='__main__':main()
