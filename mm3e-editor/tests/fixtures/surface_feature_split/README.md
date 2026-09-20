# Frozen thin-triangle feature-transition regression

`flat-alias.json` preserves the exact2312-vertex/4620-face preprocessed mesh from
the b4cba1ca frozen focused acceptance failure. Source/request/input hashes are
in `provenance.json`. Refinement domain fields, residual target,50µm normal step,
remaining109,954,694 work,65,536/131,072 geometry limits and six passes are
unchanged. Historical extraction counters are not needed by the public refiner;
the test loads geometry rather than repeating the producer or acceptance run.

The positive integration test requires the real native refiner to converge,
then independently checks sampled native/analytic field errors, closed topology,
full stored embedding,0.1% rounded volume and face/edge shoulder rays. It remains
a failing positive until the implementation achieves these original gates.

A separate exact-input cross-section counterexample shows why midpoint-only
splits need more than six iterations on observed edge409→416, while placing a
shared point at its face/edge shoulder before projection resolves its curvature
locally. That analytic control is not a substitute for the integration test.
