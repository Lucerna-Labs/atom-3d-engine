These native projects were generated and successfully evaluated before the
Document.deformers and Clip.morph_tracks fields existed. They retain their original
cache source/frame fingerprints and numeric payloads; only JSON whitespace was
removed. The regression verifies actual old-cache integrity and freshness without
recomputing an expected fingerprint using the new implementation.

- `pre_deform_cloth_v3.json`: copied from
  `artifacts/v3-cloth-compatibility-2026-09-06/project.json`, original file SHA-256
  `47a65ae03968c93fbd64f0b3a1cbec0ce6b1f53f940adb6906cc7fe6ed572002`.
- `pre_deform_cloth_v4.json`: copied from
  `artifacts/sewn-panels-acceptance-20260906-r1/sewn/project.json`, original file SHA-256
  `5cf3bfe6ec6228c111f455303206dfbb436629f9ad5a9885adaf4640643b6694`.
