"""Independent Fraction oracle for finite-expansion homogeneous predicates."""
from pathlib import Path
from fractions import Fraction as F
from functools import reduce
import argparse,hashlib,itertools,json,math,random,struct

def bits(x):return struct.pack('>d',x).hex()
def sign(x):return (x>0)-(x<0)
def solve(a,b):
 a=[list(map(F,row))+[F(v)] for row,v in zip(a,b)];n=len(a)
 for j in range(n):
  i=next(i for i in range(j,n) if a[i][j]);a[i],a[j]=a[j],a[i];d=a[j][j];a[j]=[v/d for v in a[j]]
  for i in range(n):
   if i!=j:
    f=a[i][j];a[i]=[x-f*y for x,y in zip(a[i],a[j])]
 return [r[-1] for r in a]
def captured_point(s):
 cols=[s['tetra'].index(i) for i in s['nodes'] if i!=4294967295];rows={p['feature']:p['values'] for p in s['planes']};basis=[i for i in s['basis'] if i!=18446744073709551615]
 w=solve([[1]*len(cols)]+[[rows[i][j] for j in cols] for i in basis],[1]+[0]*len(basis))
 assert all(sum(a*F(row['values'][j]) for a,j in zip(w,cols))==0 for row in s['planes'])
 return tuple(sum(a*F(s['tetra_points'][j][axis]) for a,j in zip(w,cols)) for axis in range(3))
def integer_terms(n,shift=0):
 negative=n<0;n=abs(n);terms=[]
 while n:
  limb=n&0xffffffff
  if limb:terms.append(math.ldexp(float(-limb if negative else limb),shift))
  n>>=32;shift+=32
 assert all(math.isfinite(x) for x in terms)
 return terms

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--capture-directory',type=Path,required=True);args=ap.parse_args();root=Path(__file__).resolve().parent;lines=['# exact-geometry-fraction-v1'];points=[];rows=[];counts={'orientation':0,'comparison':0};groups={};source_hashes={}
 def point(name,row):
  assert len(row)==4 and 0<sum(map(len,row))<=1024 and all(math.isfinite(x) for r in row for x in r)
  v=[sum(map(F,r),F(0)) for r in row];assert v[3]
  i=len(points);points.append(tuple(x/v[3] for x in v[:3]));rows.append(row);lines.append(','.join(['P',name]+[';'.join(bits(x) for x in r) for r in row]));return i
 def rational(name,p):
  d=math.lcm(*(x.denominator for x in p));integers=[int(x*d) for x in p]+[d];shift=min(0,1000-max(abs(x).bit_length() for x in integers));i=point(name,[integer_terms(x,shift) for x in integers]);assert points[i]==p;return i
 def orient(name,a,b,c,axes=(0,1)):
  x,y=axes;pa,pb,pc=[points[i] for i in [a,b,c]];det=(pb[x]-pa[x])*(pc[y]-pa[y])-(pb[y]-pa[y])*(pc[x]-pa[x]);lines.append(','.join(map(str,['O',name,a,b,c,x,y,sign(det)])));counts['orientation']+=1;return sign(det)
 def compare(name,a,b,axis=0):lines.append(','.join(map(str,['C',name,a,b,axis,sign(points[a][axis]-points[b][axis])])));counts['comparison']+=1
 def permutations(name,abc):
  for i,p in enumerate(itertools.permutations(abc)):
   for axes in [(0,1),(1,0),(1,2),(2,1),(2,0),(0,2)]:orient(f'{name}_p{i}_{axes[0]}{axes[1]}',*p,axes)
 origin=point('origin',[[],[],[],[1.]])
 # Entire binary64 exponent range, including all subnormal powers of two.
 for e in range(-1074,1024):
  value=math.ldexp(1.,e);a=point(f'exponent_x_{e}',[[value],[],[],[1.]]);b=point(f'exponent_y_{e}',[[],[value],[],[1.]])
  assert orient(f'exponent_{e}',origin,a,b)==1
  compare(f'exponent_positive_{e}',a,origin);compare(f'exponent_reverse_{e}',origin,a)
 groups['exponent_sweep']=2098
 # Maximum carry span and intermediate sums beyond finite binary64.
 huge=float.fromhex('0x1.fffffffffffffp+1023');tiny=math.ldexp(1.,-1074)
 cancel=point('overflow_cancel_tiny',[[huge,huge,-huge,-huge,tiny],[huge,-huge],[0.,-0.],[huge,-huge,1.]])
 exacttiny=point('tiny_reference',[[tiny],[],[],[1.]])
 compare('overflow_cancel_equal',cancel,exacttiny);compare('overflow_cancel_positive',cancel,origin)
 maximum_terms=point('maximum_1024_terms',[[math.ldexp(1.,1023)]*511+[-math.ldexp(1.,1023)]*511+[tiny],[],[],[1.]])
 compare('maximum_terms_preserves_low_bit',maximum_terms,exacttiny)
 for suffix,scale in [('small',math.ldexp(1.,-1000)),('large',math.ldexp(1.,1000)),('negative',-1.),('negative_small',-math.ldexp(1.,-1000))]:
  p=point('homogeneous_'+suffix,[[scale],[1.5*scale],[-.75*scale],[1.25*scale]])
  ref=rational('homogeneous_reference_'+suffix,(F(4,5),F(6,5),F(-3,5)))
  for axis in range(3):compare(f'homogeneous_equal_{suffix}_{axis}',p,ref,axis)
 enormous=point('ratio_beyond_f64',[[huge],[tiny],[],[tiny]])
 enormous_negative=point('ratio_beyond_f64_negative',[[-huge],[-tiny],[],[-tiny]])
 compare('negative_denominator_extreme_equal',enormous,enormous_negative)
 compare('unrepresentable_coordinate_positive',enormous,origin)
 # Collinearity with an exact one-subnormal perturbation hidden by f64 addition.
 a=point('near_line_a',[[1.],[1.],[],[1.]])
 b=point('near_line_b',[[2.],[2.],[],[1.]])
 c=point('near_line_exact',[[3.],[3.],[],[1.]])
 plus=point('near_line_plus',[[3.],[3.,tiny],[],[1.]])
 minus=point('near_line_minus',[[3.],[3.,-tiny],[],[1.]])
 for name,third in [('exact',c),('plus',plus),('minus',minus)]:permutations('near_line_'+name,[a,b,third])
 # Signed additions, cancellation, long borrow chains and multiplication carry.
 rng=random.Random(0x43A1_5EED)
 for case in range(128):
  abc=[]
  for k in range(3):
   row=[]
   for axis in range(4):
    terms=[]
    for j in range(12):
     exponent=rng.choice([-1074,-1022,-900,-500,-65,-32,-1,0,1,31,32,63,500,900,970])
     mantissa=rng.choice([1,3,0xffff,0xffffffff,0x1fffffffffffff]);value=math.ldexp(float(mantissa),exponent)
     if not math.isfinite(value):value=math.ldexp(float(mantissa),900)
     terms.append(value if rng.randrange(2) else -value)
    row.append(terms)
   if sum(map(F,row[3]),F(0))==0:row[3].append(1.)
   abc.append(point(f'carry_{case}_{k}',row))
  permutations(f'carry_{case}',abc)
  for axis in range(3):
   compare(f'carry_compare_{case}_{axis}',abc[0],abc[1],axis);compare(f'carry_reverse_{case}_{axis}',abc[1],abc[0],axis)
 groups['signed_carry_triples']=128
 # Long signed rational products that cancel exactly, or leave precisely one
 # subnormal perturbation. These deterministically force integer fallback.
 chainrng=random.Random(0xB011_0A55)
 for case in range(24):
  w=chainrng.getrandbits(160)|1;x=chainrng.getrandbits(192);y=-chainrng.getrandbits(191);u=chainrng.getrandbits(180)|1;v=-chainrng.getrandbits(178)
  if case%2:x=-x;u=-u
  a=rational(f'long_carry_a_{case}',(F(x,w),F(y,w),F(0)))
  b=rational(f'long_carry_b_{case}',(F(x+u,w),F(y+v,w),F(0)))
  refs=[]
  for label,offset in [('zero',F(0)),('plus',F(tiny)),('minus',-F(tiny))]:
   c=rational(f'long_carry_c_{case}_{label}',(F(x+2*u,w),F(y+2*v,w)+offset,F(0)));refs.append(c)
   permutations(f'long_carry_{case}_{label}',[a,b,c])
  compare(f'long_carry_plus_{case}',refs[1],refs[0],1);compare(f'long_carry_minus_{case}',refs[2],refs[0],1)
 groups['forced_long_carry_families']=24
 # Actual cofactor source points; integer expansions encode a common exact
 # rational denominator, not a float approximation to source coordinates.
 for case,triples in [('flat',[[2,3,24],[24,4,18],[0,18,5]]),('folded',[[510,846,530]])]:
  log=args.capture_directory/(case+'-capture.log');source_hashes[case]=hashlib.sha256(log.read_bytes()).hexdigest();sources={}
  for line in log.read_text().splitlines():
   if line.startswith('SOURCE '):s=json.loads(line[7:]);sources[s['vertex']]=s
  for ti,ids in enumerate(triples):
   abc=[rational(f'actual_{case}_{i}_{ti}',captured_point(sources[i])) for i in ids]
   permutations(f'actual_{case}_{ti}',abc)
   if (case,ti)!=('flat',2):assert orient(f'actual_collinear_{case}_{ti}',*abc)==0
   for axis in range(3):compare(f'actual_compare_{case}_{ti}_{axis}',abc[0],abc[1],axis)
 groups['actual_cofactor_triples']=4
 data='\n'.join(lines)+'\n';(root/'oracle.csv').write_text(data)
 manifest={'format':'exact-geometry-fraction-v1','generator_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'oracle_sha256':hashlib.sha256(data.encode()).hexdigest(),'points':len(points),'queries':counts,'groups':groups,'source_capture_sha256':source_hashes,'method':'Python Fraction sums of exact finite binary64 input terms; rational division by exact shared denominator; determinant and coordinate differences computed with unbounded integers. All six point permutations and six ordered axis pairs exercised for carry and source cases.'}
 (root/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n');print(json.dumps(manifest,indent=2))
if __name__=='__main__':main()
