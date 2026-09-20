# Optional normal-map variance filtering

Native normal-map bindings accept `variance_filter:true`. This is an **opt-in
isotropic approximation**, not production-certified normal-distribution filtering.
The default is `false`, preserving the appearance of existing projects. The
independent reference test found substantial energy loss on directional detail;
this feature does not close the film-quality gate.

```json
{"op":"bind_texture","binding":{"object":"panel","uv_set":"uv","normal":{"texture":"detail","strength":1,"variance_filter":true}}}
```

A bind replaces the entire binding. Include existing albedo, scalar and emission
fields when retaining those maps. The flag persists in native projects, works
through ordinary dry-run/undo/redo, and appears in the generated request schema.
An omitted flag or `false` uses the previous vector-mipmap behavior.

## Contract

Shading directions retain the existing filtered RGB and posed triangle tangent
frames. On minification, an additional pyramid tracks the two-component slope mean
and its central variance (the trace, without anisotropic covariance). Source slopes
are `(2R-1, 2G-1) / max(2B-1, 2^-16)`. The denominator floor bounds grazing-normal
statistics and does not change the decoded shading direction. Area resampling
includes odd image dimensions. Weighted central-moment merging avoids subtracting
nearly equal raw moments; a constant map has exactly zero added variance, including
quantized normals whose decoded length differs from one.

The normal map's own sampler, wrapping and LOD bias determine the footprint.
Nearest/bilinear filtering remains at level zero and adds no variance. At level
zero, the authored interpolated field also receives zero extra variance;
interpolation into level one is continuous. Normal strength scales variance by
strength squared. After resolving any roughness map, the renderer uses:

```
variance = sampled_slope_variance * strength^2
filtered_roughness = min(1, (unfiltered_roughness^4 + variance)^(1/4))
```

The admitted material roughness never decreases. Zero strength and zero variance
preserve the previous floating-point result exactly. The compensated roughness
reaches existing direct/GI GGX shading; the engine's separate mirror-reflection
path does not become a rough-reflection integrator.

`material_state` reports `unfiltered_roughness`, resolved `roughness` and applied
`normal_variance`. The latter is zero when disabled, unminified or strength zero.
Depth, coverage, geometry, physical cloth caches and raw geometric-normal AOVs
remain independent of this material-only adjustment.

The slope-domain motivation is related to
[Filtering Distributions of Normals for Shading Antialiasing](https://research.nvidia.com/publication/2016-06_filtering-distributions-normals-shading-antialiasing)
(Kaplanyan, Hill, Patney and Lefohn, 2016). This implementation does not reproduce
that paper's NDF filtering algorithm. Its roughness formula is a local heuristic;
it is not exact GGX convolution, LEAN mapping, anisotropic filtering or an energy
preservation guarantee. The existing footprint estimate is isotropic.

## Cost

Moments are built lazily on the first enabled, minified query and shared through
the immutable image cache. They add 12 bytes per mip texel. A 4096-square normal map
adds about 256 MiB of moments to its existing 341 MiB RGBA mip pyramid. Existing
source/active-interpretation budgets still apply; no persistent source bytes change.
Zero-strength, disabled and level-zero queries allocate no moment pyramid.

`texture_state` with `as_data:true` reports `normal_moment_bytes` for the current
cached data interpretation. `decoded_mip_bytes` includes these bytes if the cache
has been initialized. Historical sources retain original PNG bytes while unused
decoded caches can be released.

## Verification and limits

The regression was reproduced on the preserved previous release in
`artifacts/normal-filter-baseline-20260906/`, with the failed source-population
comparison retained in `artifacts/normal-filter-before-2026-09-06.log`.

Focused tests cover constant quantized/grazing maps, odd-size population variance,
LOD boundaries, wrapping, bias, lazy allocation, strength, disabled controls and
moving light directions. The workspace run passed 462 tests, with six existing
opt-in tests ignored. This number is implementation evidence, not artistic approval.

The executable acceptance script imports an original 64-square checker normal PNG,
binds it to a native surface, samples material state, renders eleven HDR light
positions, saves both flag settings, restarts without the source image and verifies
recovered bytes. OpenEXR 3.4.5 independently decodes the delivered pixels. It checks
dry-run, undo, redo, legacy-default behavior, zero strength and exact cold reload.

Two references are retained separately: the two texel-normal population and a
256-by-256 stratified integration of the authored bilinear field over a period.
The latter uses fixed +Z view and constant incident light; it is not a general
camera/shutter reference. A 128-by-128 frontal estimate is also recorded so the
finite sampling error remains visible.

For the reproduced frontal glint, unfiltered radiance is about **2480.978** and
filtered radiance **0.01143**. The texel-population reference is approximately
**1.59e-7**, but the bilinear-field estimate is **26.285**. Thus the approximation
suppresses the extreme false peak while also losing substantial real highlight
energy. At roughly 73.47 degrees the source population has a radiance near **32.05**;
the approximation yields about **0.00375**. These retained mismatches are the reason
for keeping the feature opt-in. They must not be described as exact filtering,
fully solved shimmer, or film-production readiness.

Run on an identified local release:

```sh
cargo test --offline -p mm3e-kit --test normal_filter -p mm3e-orchestrator --test normal_filter
python3 scripts/agent_normal_filter_acceptance.py --editor target/release/mm3e-editor --output artifacts/normal-filter-new-run
```

The Python check requires the official OpenEXR reader; `--exr-reader-path` can point
to an already installed isolated dependency directory. CI runs the same check on
Linux and Windows and retains its artifacts. Configuring that job is not evidence
that remote CI or Windows execution has already passed.

The final checkpoint identifies source, binary, package and acceptance hashes.
Earlier failed harness output is retained: its first run incorrectly requested an
alpha channel without enabling film alpha. The corrected harness enables alpha
and geometry data channels explicitly.
