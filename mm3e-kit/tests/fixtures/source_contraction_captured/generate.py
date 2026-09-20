"""Rebuild tight binary64 fixtures from frozen exact-source capture evidence."""
from pathlib import Path
from fractions import Fraction as F
import argparse,json,hashlib,math

def solve(a,b):
 a=[list(map(F,row))+[F(v)] for row,v in zip(a,b)];n=len(a)
 for j in range(n):
  i=next(i for i in range(j,n) if a[i][j]);a[i],a[j]=a[j],a[i];d=a[j][j];a[j]=[v/d for v in a[j]]
  for i in range(n):
   if i!=j:
    f=a[i][j];a[i]=[x-f*y for x,y in zip(a[i],a[j])]
 return [r[-1] for r in a]
def point(s):
 cols=[s['tetra'].index(i) for i in s['nodes'] if i!=4294967295];rows={p['feature']:p['values'] for p in s['planes']};basis=[i for i in s['basis'] if i!=18446744073709551615]
 w=solve([[1]*len(cols)]+[[rows[i][j] for j in cols] for i in basis],[1]+[0]*len(basis))
 assert all(sum(a*F(row['values'][j]) for a,j in zip(w,cols))==0 for row in s['planes'])
 return tuple(sum(a*F(s['tetra_points'][j][axis]) for a,j in zip(w,cols)) for axis in range(3))
def bracket(x):
 q=float(x);lo=q if F(q)<=x else math.nextafter(q,-math.inf);hi=q if F(q)>=x else math.nextafter(q,math.inf)
 assert math.isfinite(lo) and math.isfinite(hi) and F(lo)<=x<=F(hi)
 assert lo==hi or math.nextafter(lo,math.inf)==hi
 assert float(repr(lo))==lo and float(repr(hi))==hi
 return lo,hi

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--capture-directory',type=Path,required=True);ap.add_argument('--json-output',type=Path,required=True);args=ap.parse_args();dest=Path(__file__).resolve().parent;args.json_output.mkdir(exist_ok=True)
 manifest={}
 for case,edge in [('flat',[0,5]),('folded',[510,846])]:
  logfile=args.capture_directory/(case+'-capture.log');tri_file=args.capture_directory/(case+'-triangulation.json');oracle_file=args.capture_directory/(case+'-embedding.json');d=json.loads(tri_file.read_text());sources={}
  for line in logfile.read_text().splitlines():
   if line.startswith('SOURCE '):s=json.loads(line[7:]);sources[s['vertex']]=s
  ids=sorted({x for t in d['triangles'] for x in t});index={i:j for j,i in enumerate(ids)};bounds=[];exact_axes=0;max_width=0
  for i in ids:
   p=point(sources[i]);assert p==tuple(map(F,d['exact_points'][str(i)]));pair=[bracket(x) for x in p];bounds.append([[v[0] for v in pair],[v[1] for v in pair]])
   for lo,hi in pair:exact_axes+=lo==hi;max_width=max(max_width,hi-lo)
  triangles=[[index[v] for v in t] for t in d['triangles']];metadata={'format':'mm3e-source-contraction-capture-v1','case':case,'original_vertex_ids':ids,'original_endpoint_ids':edge,'endpoint_indices':[index[v] for v in edge],'intervals':bounds,'triangles':triangles,'independent_exact_oracle':json.loads(oracle_file.read_text()),'provenance':{'source_capture_sha256':hashlib.sha256(logfile.read_bytes()).hexdigest(),'exact_triangulation_sha256':hashlib.sha256(tri_file.read_bytes()).hexdigest(),'exact_oracle_sha256':hashlib.sha256(oracle_file.read_bytes()).hexdigest(),'coordinate_method':'Exact rational Gaussian elimination of original captured tetrahedron/basis equations; cross-checked against prior exact_points. Smallest enclosing adjacent binary64 values, or singleton when exactly representable.'},'statistics':{'vertices':len(ids),'triangles':len(triangles),'singleton_coordinate_intervals':exact_axes,'maximum_interval_width':max_width}}
  (args.json_output/(case+'.json')).write_text(json.dumps(metadata,separators=(',',':'))+'\n')
  lines=['# mm3e-source-contraction-capture-v1',f'# {case}: original endpoints {edge}; local endpoint indices {[index[v] for v in edge]}']
  for i,(original,b) in enumerate(zip(ids,bounds)):lines.append(','.join(['v',str(original)]+[repr(x) for row in b for x in row]))
  for t in triangles:lines.append(','.join(['t']+list(map(str,t))))
  csv=dest/(case+'.csv');csv.write_text('\n'.join(lines)+'\n');manifest[case]={'statistics':metadata['statistics'],'original_endpoint_ids':edge,'endpoint_indices':metadata['endpoint_indices'],'csv_sha256':hashlib.sha256(csv.read_bytes()).hexdigest(),'json_sha256':hashlib.sha256((args.json_output/(case+'.json')).read_bytes()).hexdigest(),'provenance':metadata['provenance']}
  print(case,metadata['statistics'],metadata['endpoint_indices'])
 (dest/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
if __name__=='__main__':main()
