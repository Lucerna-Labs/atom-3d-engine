# Film images and sampled shots

For frozen snapshots, bounded frame steps and recovery after interruption, see
[persistent render jobs](RENDER_JOBS.md).

The CPU film path evaluates real camera rays and the complete animated scene, including
facial controls, fitted garments and baked cloth. Beauty is accumulated in linear light.
Display PNGs are tone-mapped; EXRs retain scene-linear f32 Rec.709/D65 values.

For an existing `performance` clip, these settings enable coverage alpha, utility channels,
a thin lens and a centered 180-degree shutter at 24 fps. `set_settings` replaces the full
settings structure; inspect and retain any other values you need before applying it.

```json
{"id":"shot-settings","expected_revision":1,"command":{"op":"apply","operations":[{"op":"set_settings","settings":{"width":512,"height":512,"quality":"balanced","spatial_aa":2,"film":{"transparent_background":true,"exr_data_channels":true,"aperture_radius_m":0.004,"focus_distance_m":5,"shutter_open_seconds":-0.010416667,"shutter_close_seconds":0.010416667,"shutter_samples":8}}}]}}
{"id":"plate","command":{"op":"render","path":"performance.exr","animation":{"clip":"performance","time":0.5}}}
```

Supply the actual current revision. Exposure times are seconds relative to the requested
animation sample. Temporal samples use interval midpoints, and each sample evaluates the
body, face, cloth and camera pose independently. A nonzero shutter requires explicit
animation. A finite exposure interval needs at least two samples; zero exposure duration
uses exactly one. Sample count is bounded at 64 and offsets at ±1 second. Clip endpoint
behavior follows the specified playback mode, so plan handles around shots when clamping
would hold motion during exposure.

`spatial_aa` is samples **per axis**, from 1 to 8. A nonzero aperture requires at least 2.
The lens radius is in meters and focus distance is camera-forward depth. This geometric
lens does not model exposure compensation, lens aberration or physical shutter motion.
Blended subframe radiance produces motion blur; more samples may be needed for fast motion.

With `exr_data_channels:true`, EXR channels are `R G B A Z N.X N.Y N.Z material.ID`.
Transparent RGB is premultiplied foreground radiance; alpha is actual ray coverage and
excludes the sky. Opaque RGB includes the sky and alpha is one. Utility channels use the
exact reference-time pinhole ray rather than blurred/averaged values: Z is camera-forward
depth in meters, normal is world-space, and the unsigned material ID maps to source entity
IDs through metadata. Miss values are positive infinity for Z, zero normal and `u32::MAX`
for material ID. A ray starting inside geometry may have Z zero.

With active film settings but no data-channel flag, EXR writes exactly RGBA and no utility
channels. With all film features disabled it retains the legacy RGB EXR contract. PNG
converts transparent premultiplied linear RGB to straight alpha before display conversion;
fully transparent encoded pixels have zero RGB. BMP rejects transparent output. Film
settings and EXR sequences require beauty rather than display diagnostic passes.

`render_sequence` uses a new directory and writes the manifest last. It samples
`start + frame_index / fps` through the inclusive end bound and supports `format:"exr"`.
It preflights reference and shutter poses before creating output, preserving failures such
as a stale cloth cache or a rejected sampled collision. Images are limited to 8192 per
axis and 16,777,216 pixels. Sequence limits are 2400 frames and eight billion total pixels;
primary-sample budgets also include spatial, shutter, reference-AOV and diagnostic passes.
The limits bound requests; they do not establish practical throughput for a film shot.

Metadata records the sampled camera/FOV, shutter, material owner map and native source
fingerprint. An image with valid EXR channels is still not a complete color-managed pipeline:
OCIO configuration, compositing application setup, display mastering and renderer parity
remain separate production work. Likewise, utility channels are not Cryptomatte, and
coverage is not a guarantee of temporally converged hair, fabric or facial detail.

Real executable acceptance with independent OpenEXR decoding is in
`scripts/agent_shot_acceptance.py`; its tests include temporal radiance reconstruction,
depth of field, center-time AOVs, sparse transparency, cold reload and rejection without files.
