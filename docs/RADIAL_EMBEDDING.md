`mm3e_kit::radial_embedding::certify` provides an additive sufficient certificate for embedding of the supplied finite stored indexed triangle surface. It does not replace the general pairwise validators. The returned `Certificate` has a private constructor and exposes an immutable report only after every required gate succeeds. Its proof applies to the arrays at that call and must not be reused after geometry changes.

The API takes positions, triangles, and a work allowance. It returns either the complete certificate or a typed failure with every spent work unit. `InvalidInput` covers malformed dimensions, indices, or nonfinite positions. `Inconclusive` means this sufficient method did not establish embedding; the caller may use the unchanged general validator with the remaining shared allowance. `Budget` means the attempt exhausted its bounded work. Prior failed centre/ray attempts are included in successful and failed reports.

There are exactly two possible centre candidates (exact bounds midpoint and exact vertex mean) and four fixed nonzero dyadic ray directions per centre. No tolerance or caller-supplied unbounded candidate list is used. The work report separates topology, stored-point construction, centre construction, plane predicates, ray predicates, and structural work. The certificate includes exact centre and virtual-endpoint definitions, topology counts, the signed degree, and the forward face-interior hit count.

The following proof is a mesh-specific inference. The primary background cited below supplies degree terminology, but does not substitute for establishing the local face/edge/vertex hypotheses.

The input is a finite, connected, closed, consistently oriented simplicial triangle 2-manifold. Its index proof must establish distinct valid triangles, exactly two oppositely directed incident faces per edge, and one cyclic link per vertex. Retain the requested Euler-characteristic-two gate. Every position is finite. A single exact centre C must give the same nonzero sign s for orient3(a,b,c,C) on every oriented face. A nonzero exact direction D must avoid all projected mesh edges, and its forward ray must meet exactly one face interior. Zero determinants, inconsistent signs, malformed topology, failed work budgets, and nongeneric rays are inconclusive for this sufficient certificate; they require the existing general validator.

Here is the local-to-global argument. It is specific to triangular surfaces and does not invoke a higher-dimensional branch-set theorem.

1. Let g be the piecewise affine realization of the abstract triangle complex and r(x)=(g(x)-C)/|g(x)-C|. A strict face determinant puts C off the face plane. It also excludes zero-area faces, radial edge collapse, and C on any mesh point. Each closed face maps injectively to a convex spherical triangle in an open hemisphere. On its interior the radial map is a local diffeomorphism with orientation sign s.

2. At an edge interior, the two incident faces have opposite induced edge orientations. Their common strict sign places their spherical interiors on opposite sides of the projected edge. The radial map is therefore a local homeomorphism across that edge. Merely counting incident faces without coherent orientation would not establish this.

3. At a vertex, project each consecutive neighbor direction into the tangent plane of the vertex's spherical image. None vanishes, and consecutive directions are linearly independent; either failure would contradict an incident strict face determinant. In the cyclic link, their oriented angular increments have the same sign, each with magnitude strictly between zero and pi. Closing the single cycle gives total turning 2*pi*k with integer |k|>=1 and sign s. A sufficiently small spherical disk is covered by the incident sectors, so the radial map is open at the vertex. This allows a branch of multiplicity |k|; it does not yet prove |k|=1. The only possible branch points are the finite set of mesh vertices. Each fiber is finite because the map is injective on each of finitely many closed faces.

4. For a direction away from every projected edge, let N be its number of face-interior preimages. N is constant: a generic path on the sphere can avoid the finite projected vertices and arc intersections, and cross the finite projected-edge arrangement transversely. Crossing each indexed edge exchanges one incident face for the other. Coincident projected edge arcs can be crossed together, with each indexed edge still contributing zero net change. The common orientation sign prevents cancellation of opposite local degrees. Thus the single generic ray with one preimage gives N=1 at every generic direction.

5. The radial image is compact and contains the dense set of generic directions, hence it is the whole sphere. If two distinct abstract mesh points had the same radial image, disjoint neighborhoods around them would have intersecting open images by steps 1-3. Their intersection contains a generic direction, which would have at least two preimages, contradicting N=1. The radial map is therefore a continuous bijection from a compact space to the sphere, hence a homeomorphism. Injectivity of r implies injectivity of g. Consequently the actual triangle realization is embedded, and equals C+rho(u)*u for a continuous positive radial function rho.

Pankka's primary background describes local indices and their summation into degree (Theorem 3.3.14, printed page 72), and connects degree-one discrete open maps to homeomorphisms (Lemma 4.3.3, printed page 78). These support the degree interpretation; the mesh-specific face/edge/vertex openness argument above supplies the hypotheses rather than assuming them. The notes' introductory Chernavskii-Vaisala theorem for dimension at least three is not the justification for this two-dimensional surface case. [Degree theory and Branched covers, Pekka Pankka](https://www.helsinki.fi/~pankka/DegreeTheory-Branched_Covers_2017.pdf).

For exact ray classification, use Q=C+D constructed in homogeneous expansion coordinates, not a rounded point sum. For each oriented edge (a,b) of a face, E(a,b)=-orient3(C,Q,a,b). A forward interior hit requires all three E signs to equal that face's common plane sign s. This also establishes positive ray parameter: their sum has the sign of the face normal dotted with D, while the strict face-plane determinant gives the corresponding numerator sign. A zero edge sign is treated conservatively as nongeneric even when the ray lies on the supporting great circle outside the finite projected edge. Counting triangles hit by the backward ray would be incorrect.

The production centre and virtual endpoint use the same exact construction as the prototype. Bbox coordinates are represented by `[minimum, maximum] / 2`. Vertex-mean numerators use error-free expansion sums over the exact integer vertex count. Direction components are fixed dyadics. The endpoint numerator appends `denominator * D`, which is exactly representable for the bounded vertex count and chosen directions. A fixed 32-term bound protects centre expansion storage. A zero cone determinant is inconclusive rather than perturbed by an epsilon.

The mandatory degree-two control repeats four cardinal equator positions twice with eight distinct equator indices and shared north/south poles. Its connected oriented indexed sphere has Euler characteristic two and all face-plane signs strictly positive, but the generic ray has two forward hits. It is not certified. Ordinary and globally reversed octahedra certify with degree +1 and -1 respectively. Edge-aligned and vertex-aligned rays are inconclusive. Genus and component controls retain the complete topology precondition.

The frozen actual acceptance input is `tests/fixtures/radial_embedding/general-warped-refined.bin`: 16,464 vertices and 32,924 faces. Its source JSON and binary hashes are recorded in the adjacent manifest. The integration test requires agreement with the unchanged full pairwise validator, exact work-budget replay, one-less-budget failure, and unchanged input bits/indices. This establishes the sufficient method on that input; it does not establish original field coverage, facial/clothing export acceptance, between-frame deformation, or material/UV fidelity.

Historical proof/prototype measurements and negative controls remain in `artifacts/radial-embedding-prototype-20260910-r1/`. That prototype's first auxiliary signed-degree report incorrectly counted the backward ray; its forward-hit eligibility test was correct. Corrected results and all original evidence are preserved. The production tests explicitly check signed degrees +1, -1, and +2 and use only the forward cone criterion.

## September 19: lazy exact coordinates

The same mandatory topology, face-plane and ray gates now construct exact
rational vertex coordinates only as faces visit them. The cache belongs to
one immutable call, is bounded by the input vertex count, and reuses exact
points across both centre attempts. Allocation, three slot inspections per
face, point construction, and failed attempts remain charged. No coordinate
rounding, centre search, predicate, or acceptance limit changes.

On the preserved 46,740-vertex / 93,476-face flat-shoulder candidate, both
centres still fail after 660 face-plane tests. Rational point construction
falls from 4,490,709 to 19,008 units; total unsuccessful radial work falls from
10,270,148 to 5,800,427. The subsequent complete pair validator still exceeds
200,000,000 work, so this is not an accepted mesh or export. The measured
input and outcomes are recorded in
`artifacts/radial-lazy-20260919-r1/run.log`.
