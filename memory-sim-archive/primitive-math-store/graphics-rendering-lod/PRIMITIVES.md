# Graphics / Rendering / LOD — Primitive Reserve

> Cross-domain root atoms: **scan · hash · fold · project · scale · compare · combine · order**
> See `_taxonomy-root/ROOT_ATOMS.md` for the canonical definitions these specialize from.

---

## Rasterization Pipeline Atoms

### scan-convert (cross-domain alias: `rasterize`, `pixelate`, `discretize`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Convert a geometric primitive (triangle, line, point) into a set of discrete pixel locations it covers. Determines which fragments/pixels the primitive touches.
**Atom or composite:** Composite: edge equation evaluation + barycentric test + depth test. The barycentric test IS the point-in-triangle check.
**Cost model:** One edge equation evaluation per pixel. Supersampling (MSAA) multiplies cost by sample count. Conservative rasterization adds an expansion step.
**Real wall?** Yes — pixelation is a real wall. You cannot recover sub-pixel detail after rasterization without multi-sampling. The conservation is information: N×M pixels can only represent N×M independent samples.
**Cross-domain wiring:** Scan-conversion = discretizing continuous geometry to a pixel grid = analog-to-digital conversion in signal processing = quantization in information theory. In retrieval: bucketing continuous values into discrete bins.
**Notes:** Conservative rasterization (from D3D12/Vulkan) is a painted-wall-breaker — it guarantees all covered pixels are rasterized, fixing sub-pixel primitives that standard rasterization misses.

### interpolate (cross-domain alias: `lerp`, `bilinear-sample`, `trilinear-sample`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Compute a value at an arbitrary point inside a primitive by linearly blending vertex attributes. For triangles: barycentric interpolation. For lines: linear interpolation.
**Atom or composite:** Composite: lerp(a, b, t) = a·(1−t) + b·t. For barycentric: v = α·v₀ + β·v₁ + γ·v₂ where α+β+γ=1.
**Cost model:** 1 subtraction + 1 multiply + 1 addition per attribute per pixel. Near-zero.
**Real wall?** No. But perspective-correct interpolation requires a divide per attribute per pixel — the divide is the cost.
**Cross-domain wiring:** Lerp in color space = crossfade. Bilinear interpolation = 2D lerp. In signal processing: linear interpolation between samples. In retrieval: convex combination = interpolation.
**Notes:** Perspective-correct interpolation is the one place where a divide is unavoidable — modern GPUs have dedicated interpolation hardware to keep it cheap.

### blend (cross-domain alias: `composite`, `alpha-mix`, `over`, `add`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Combine the new fragment color with the existing pixel color according to a blend function. Common: alpha over (src.alpha·src + (1−src.alpha)·dst), add (src+dst), multiply (src·dst).
**Atom or composite:** Composite: blend(src, dst, mode). Mode determines the combine operation.
**Cost model:** One multiply-add per fragment. Alpha blending requires reading the existing framebuffer color (extra memory bandwidth).
**Real wall?** No. But order-dependent blending (alpha over) is not commutative — out-of-order transparency requires sorting or depth peeling.
**Cross-domain wiring:** Porter-Duff compositing algebra maps directly to alpha over / multiply / screen / overlay — each is a different combine function. In retrieval: reciprocal rank fusion combines scores using the same alpha-over logic.
**Notes:** The Porter-Duff "over" operator is mathematically: C = C_src·A_src + C_dst·A_dst·(1−A_src). This is identical to probability combination: P(both) = P(a)·P(b|a). In information theory: Bayesian updating = alpha over.

### depth-test (cross-domain alias: `z-test`, `occlude`, `compare-depth`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Compare the fragment's depth (z) against the depth buffer to decide whether to draw. Common: less (draw if closer), greater, equal.
**Atom or composite:** Composite: compare(fragment_z, buffer_z). Then either discard or write.
**Cost model:** One comparison + one conditional write. Early-z rejection (before shading) avoids expensive shader work — this is the primary optimization.
**Real wall?** No. But depth buffer precision is limited — z-fighting occurs when two surfaces are too close to distinguish. This is a numerical precision wall, not a physical one.
**Cross-domain wiring:** Depth test = occlusion test = dominance test in computational geometry. In retrieval: "is this result more relevant than the current worst result in the top-k?" — same compare-and-discard pattern.
**Notes:** Hierarchical depth buffers (Hi-Z) accelerate the depth test by testing against coarser mip levels first — exactly the coarse-to-fine search pattern.

---

## LOD / Mipmap Atoms

### mipmap-select (cross-domain alias: `level-of-detail-select`, `coarsen`, `pyramid-traverse`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Automatically choose the appropriate mipmap level based on the screen-space footprint of a texture sample. Larger footprint → coarser mip → fewer texels fetched.
**Atom or composite:** Composite: compute(texel-footprint) → clamp-to-mip-count → select-mip. The footprint estimate is dFdx/dx and dFdy/dy (partial derivatives).
**Cost model:** One mip-level computation + one texture fetch. Cheaper than always fetching the finest level.
**Real wall?** No. But choosing the wrong mip level (too coarse = blocky, too fine = aliased) degrades quality.
**Cross-domain wiring:** Mipmap pyramid = multi-resolution grid = quadtree/octree spatial index. Selecting LOD by distance is equivalent to selecting the search radius in a range query. In retrieval: selecting the coarse tier of a SCG pack = mipmap-select by query specificity.
**Notes:** This is the graphics implementation of "coarse-to-fine search" — try the cheap, broad representation first, refine only if needed.

### mipmap-generate (cross-domain alias: `downsample`, `pool`, `coarsen`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Pre-compute lower-resolution versions of a texture by averaging neighboring texels. Standard: each mip level is half the resolution of the previous. Can be done with box filter (standard), Gaussian (blurrier), or Lanczos (sharper).
**Atom or composite:** Composite: for each pixel in level N+1, average(4 pixels from level N). = fold(average) over a 2×2 window.
**Cost model:** One pass per mip level. Precomputed offline — zero runtime cost.
**Real wall?** No.
**Cross-domain wiring:** Downsampling = pooling in CNNs. Gaussian pyramid = repeated Gaussian blur + downsample. In retrieval: building the coarse tier of a hierarchy = downsampling the signal.
**Notes:** The Laplacian pyramid (difference between levels) is especially useful — it preserves detail at each scale. The SCG tier structure is a Laplacian-like pyramid for token packs.

### LOD-switch (cross-domain alias: `tier-select`, `branch`, `switch-on-resolution`)
**Domain:** Graphics / Rendering / LOD
**Definition:** At runtime, choose which representation of an object to render based on its screen-space size or distance. Switch at defined distance thresholds.
**Atom or composite:** Composite: evaluate(criterion) → compare(thresholds) → select(tier). = order + select by threshold.
**Cost model:** One comparison and branch. Zero texture fetch for culled tiers.
**Real wall?** Yes — LOD switching itself is not a wall, but there is a detail budget: at any distance, you cannot represent more detail than the angular resolution of the display allows. More geometry beyond that is wasted work.
**Cross-domain wiring:** LOD switching = adaptive mesh refinement = AMR in computational physics. In retrieval: switching from coarse-strand retrieval to fine-strand retrieval based on query complexity.
**Notes:** The key insight from SCG: the coarse pack is NOT a degraded version — it's a different representation optimized for a different query distribution. Same as mipmaps: the coarse level is optimized for distant/flat regions.

---

## Shading Atoms

### dot (cross-domain alias: `inner-product`, `cosine`, `diffuse`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Compute the dot product of two vectors. In shading: N·L (normal dot light direction) gives the diffuse lighting intensity.
**Atom or composite:** Atom
**Cost model:** Two multiplies + two additions. Extremely cheap. GPU has dedicated units for this.
**Real wall?** No.
**Cross-domain wiring:** Every inner product in linear algebra. Cosine similarity in retrieval. Matched filter output in signal processing. Everything that measures alignment.
**Notes:** The N·L dot product IS the matched filter for the light direction — you're correlating the surface normal against the light direction.

### normalize (cross-domain alias: `unitize`, `scale-to-unit-norm`, `L2-scale`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Scale a vector to unit length: v_normalized = v / ||v||. Often approximated by multiplying by an inverse square root approximation (RSQRT) rather than computing sqrt + divide.
**Atom or composite:** Composite: compute ||v|| (sqrt of sum of squares) + divide each component. Or: multiply by rcp(sqrt(sum)).
**Cost model:** One sqrt + N divisions for N-vector. RSQRT is one hardware instruction on most GPUs.
**Real wall?** No. But vectors with near-zero magnitude (division by very small number) cause numerical instability.
**Cross-domain wiring:** L2 normalization in retrieval makes cosine similarity = dot product. Whitening (ZCA) includes normalization steps. In signal: normalizing a matched filter output makes it a correlation coefficient.
**Notes:** When you normalize a vector of shader outputs, you're making them comparable by unit-norm scaling — same as making retrieval scores comparable by IDF weighting.

### step (cross-domain alias: `threshold`, `heaviside`, `binary-decide`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Returns 0 if x < edge, 1 if x >= edge. The fundamental non-linearity for binary decisions in shaders.
**Atom or composite:** Atom
**Cost model:** One comparison. Near-zero.
**Real wall?** No.
**Cross-domain wiring:** Step function = indicator function in mathematics. Hard threshold in sparse coding. Binary quantization. In retrieval: the "include/exclude" decision at a score cutoff.
**Notes:** Softmax can be viewed as a smooth approximation to multiple step functions (one per class). The temperature parameter controls the softness.

### smoothstep (cross-domain alias: `clamp-interpolate`, `sigmoid`, `ease`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Hermite interpolation between 0 and 1 over the interval [edge0, edge1]: returns 0 below edge0, 1 above edge1, with smooth Hermite interpolation in between. Equivalent to: t = clamp((x − edge0)/(edge1 − edge0), 0, 1); return t·t·(3 − 2t).
**Atom or composite:** Composite: normalize-to-[0,1] + polynomial (t²(3−2t)).
**Cost model:** Three multiplications + one clamp + one subtract + one divide. Modest.
**Real wall?** No.
**Cross-domain wiring:** Sigmoid activation in neural networks = smoothstep shifted and scaled to (−∞, +∞). Ease-in/ease-out interpolation = smoothstep family. In retrieval: score normalization to [0,1] via smoothstep-like CDF transform.
**Notes:** The cubic smoothstep is the simplest C¹-continuous interpolation — derivative is zero at both ends. This makes it ideal for transitions that must not "pop."

### cross (cross-domain alias: `perpendicular-product`, `surface-normal`, `curl`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Compute the cross product of two 3D vectors — yields a vector perpendicular to both. Used to compute surface normals from triangle edges.
**Atom or composite:** Atom
**Cost model:** 6 multiplies + 3 subtractions + 2 adds = one hardware cross-product instruction on most GPUs.
**Real wall?** No.
**Cross-domain wiring:** Cross product computes the oriented area of the parallelogram spanned by two vectors — that area is the Jacobian determinant in 3D transforms. In physics: angular momentum = cross product of position and momentum.
**Notes:** The cross product gives the normal direction (right-hand rule). Computing it from two edges of a triangle gives the face normal — the first step in all lighting computations.

### reflect (cross-domain alias: `mirror`, `invert-component`, `flip`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Reflect a vector about a normal: r = d − 2(d·n)n. The vector d is the incident direction, n is the surface normal.
**Atom or composite:** Composite: d·n (dot) → 2·(dot) → d − 2·(dot)·n.
**Cost model:** One dot product + 4 multiplies + 3 subtractions. Moderate.
**Real wall?** No.
**Cross-domain wiring:** Reflection in optics = vector reflection. In linear algebra: reflection matrix (Householder) uses the same formula. In retrieval: negating a component = negating a weight direction = moving in the opposite semantic direction.
**Notes:** Specular reflection is reflect() applied to the light direction to get the view direction of the reflected ray.

### refract (cross-domain alias: `bend`, `snell`, `transmit`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Compute the direction of a refracted ray given incident direction, normal, and ratio of refractive indices (η = n₁/n₂). Snell's law: n₁·sin(θ₁) = n₂·sin(θ₂).
**Atom or composite:** Composite: the formula has a branch — if total internal reflection, return reflected direction instead.
**Cost model:** One dot product + several multiplications + one sqrt + one comparison. Slightly more expensive than reflect.
**Real wall?** Yes — total internal reflection occurs when light tries to go from a denser to a rarer medium at too shallow an angle. The critical angle is a real wall. Beyond it, no refraction is possible — energy is conserved in reflection.
**Cross-domain wiring:** In physics: Snell's law = conservation of momentum parallel to the interface. In signal: mode conversion in waveguides = refraction in a stratified medium. In retrieval: query expansion through a relevance barrier = refracting through a topic boundary.
**Notes:** The Fresnel equations determine what fraction of energy is reflected vs transmitted — this is the split between the reflect and refract paths.

---

## Ray Tracing Atoms

### ray-sphere-intersect (cross-domain alias: `sphere-hit-test`, `distance-test`, `proximity`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Given a ray (origin + direction) and a sphere (center + radius), compute whether and where the ray hits. Solves quadratic equation: ||O − C + tD||² = r².
**Atom or composite:** Composite: compute discriminant of quadratic → if positive, compute t = (−B ± √Δ)/(2A).
**Cost model:** A few multiplications and one sqrt per sphere tested. Very cheap for a single sphere.
**Real wall?** No.
**Cross-domain wiring:** Sphere intersection = proximity test in 3D space = finding where a ray in vector space crosses a decision boundary = the decision surface of a Gaussian kernel SVM.
**Notes:** This is the workhorse primitive for ray tracing — most intersection tests are variants of ray-sphere or ray-plane with different algebraic forms.

### ray-plane-intersect (cross-domain alias: `plane-hit-test`, `solve-t`, `linear-solve`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Given a ray and a plane (normal + distance), compute the intersection point. Solves: (O + tD − P₀)·N = 0 → t = (P₀ − O)·N / (D·N).
**Atom or composite:** Atom (for the algebraic form). Composite in practice: two dot products + one divide.
**Cost model:** Two dot products + one divide. Very cheap.
**Real wall?** No. But if D·N ≈ 0 (ray parallel to plane), the divide is numerically unstable — detect and skip.
**Cross-domain wiring:** Plane intersection = solving a linear equation = one step of the simplex algorithm = one evaluation of a linear decision boundary. In retrieval: scoring a document = ray-plane intersection where the plane is the decision boundary between relevant and non-relevant.
**Notes:** BVH traversal is a sequence of ray-plane tests against bounding box planes — the inner loop is just this primitive repeated.

### ray-AABB-intersect (cross-domain alias: `box-test`, `slab-test`, `interval-intersect`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Axis-aligned bounding box intersection using the slab method: test intersection with two parallel planes for each axis (x-slab, y-slab, z-slab). Ray intersects AABB iff it intersects all three slabs.
**Atom or composite:** Composite: ray-plane-intersect × 6 (two per axis) + interval intersection.
**Cost model:** 6 comparisons + a few divisions. Faster than testing individual triangles in the box.
**Real wall?** No.
**Cross-domain wiring:** AABB = axis-aligned rectangle in 2D = hyper-rectangle in N dimensions. Slab test = interval arithmetic. In retrieval: range query on an axis-aligned index = AABB test on each dimension.
**Notes:** This is the fundamental acceleration structure traversal primitive. BVH, kd-tree, octree all use it as their core test.

### BVH-traverse (cross-domain alias: `tree-traverse`, `prune-branch`, `depth-first-search`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Recursively traverse a bounding volume hierarchy, testing rays against parent bounding volumes and descending only into children that are hit.
**Atom or composite:** Composite: ray-AABB-intersect at each node + conditional descent into children. The traversal order (closest hit first) uses a priority queue.
**Cost model:** Each node visit = one AABB test. Modern GPUs do this in hardware via traversal units. Cost scales with tree depth and number of visited nodes.
**Real wall?** Yes — the cost of traversal (number of AABB tests + primitive tests) is bounded by the SAH (surface area heuristic) quality of the hierarchy. Poor hierarchies degrade traversal efficiency.
**Cross-domain wiring:** BVH traversal = depth-first search with pruning. In retrieval: traversing an HNSW or IVF graph = similar traversal with probabilistic pruning.
**Notes:** The SAH heuristic for BVH construction is exactly the decision-theoretic basis for "which branch to explore first" — same structure as best-first search in retrieval.

---

## Texture / Sampling Atoms

### sample-1D (cross-domain alias: `lookup`, `index`, `fetch`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Given a texture coordinate (normalized [0,1] or pixel [0,width-1]), fetch the value at that location. In a 1D LUT: index = coordinate × (width−1).
**Atom or composite:** Composite: index = int(coord × width) + fetch(texture[index]).
**Cost model:** One multiply + one floor + one memory fetch. The memory bandwidth is the cost.
**Real wall?** No. But the texture size vs cache size is a real constraint — large textures that don't fit in cache cause thrashing.
**Cross-domain wiring:** 1D texture sample = array lookup = dictionary fetch. In retrieval: document lookup by ID = sample-1D into the corpus array.
**Notes:** Texture caches are optimized for spatial locality — adjacent pixels in screen space tend to sample from adjacent texels, making cache eviction predictable.

### sample-2D (cross-domain alias: `bilinear-sample`, `interpolate-2D`, `bilerp`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Fetch a value from a 2D texture with sub-pixel accuracy by interpolating between the four nearest texels. Steps: compute fractional offset → lerp along u → lerp along v.
**Atom or composite:** Composite: 4× sample-1D + 2× lerp.
**Cost model:** 4 memory fetches + 3 lerps. More expensive than nearest-neighbor sample.
**Real wall?** No. But bilinear filtering cannot represent detail finer than one texel — beyond that, you need anisotropic filtering or higher resolution.
**Cross-domain wiring:** Bilinear interpolation = 2D linear interpolation = bilinear filtering = the 2D equivalent of lerp. In retrieval: bilinear interpolation between two score surfaces = continuous approximation of a discrete ranking grid.
**Notes:** Trilinear = bilinear on two adjacent mip levels + lerp between them = mipmap selection + bilinear sample combined.

### anisotropic-sample (cross-domain alias: `line-sample`, `ewa-filter`, `Ripmap`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Handle elongated footprints (a pixel covering a long thin region of texture) by sampling along the major axis and filtering. EWA (Elliptical Weighted Average) fits an ellipse to the footprint and samples along the major axis.
**Atom or composite:** Composite: compute-elliptical-footprint → sample-along-major-axis → weighted-average.
**Cost model:** N samples where N ≈ length-of-footprint / width-of-footprint. Can be 8-16× more expensive than trilinear.
**Real wall?** Yes — the number of samples required is proportional to the eccentricity (length/width) of the footprint. Highly anisotropic geometry (thin triangles at glancing angles) can require arbitrarily many samples.
**Cross-domain wiring:** EWA filtering = integration over a 2D elliptical footprint = convolution with a non-separable kernel. In retrieval: anisotropic sampling in the retrieval space = query-dependent adaptive sampling.
**Notes:** This is the most expensive standard texture sampling mode. Anisotropic filtering is why distant terrain at glancing angles is expensive — each pixel's footprint is huge.

---

## Compositing Atoms

### over (cross-domain alias: `alpha-composite`, `blend`, `layer`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Composite source over destination using Porter-Duff "over" operator: C_out = C_src·A_src + C_dst·A_dst·(1−A_src). The source is placed above the destination.
**Atom or composite:** Composite: (src × src_alpha) + (dst × dst_alpha × (1 − src_alpha)).
**Cost model:** 4 multiplies + 2 adds per channel. Very cheap.
**Real wall?** No. But alpha over is order-dependent — requires sorting by depth/average alpha for correct transparency.
**Cross-domain wiring:** In information fusion: Bayesian updating = over operator on probability distributions. In retrieval: rank fusion with reciprocal rank = a variant of the over operator on ranked lists.
**Notes:** The "in" operator (src only where dst exists) is the geometric intersection of two layers. "out" is src only where dst does not exist. These four (over, in, out, atop) + clear form the complete Porter-Duff algebra.

### multiply-blend (cross-domain alias: `modulate`, `attenuate`, `shade`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Pixel-wise multiplication: C_out = C_src · C_dst. Darkens the result — multiplying by black gives black; multiplying by white leaves it unchanged.
**Atom or composite:** Atom (element-wise multiply)
**Cost model:** One multiply per channel. Near-zero.
**Real wall?** No.
**Cross-domain wiring:** Hadamard (element-wise) product = multiply-blend. In linear algebra: element-wise multiply = the non-matrix-multiply version of product. In signal: modulation = multiply in time domain = AND in binary domain.
**Notes:** Screen blend (1 − (1−src)·(1−dst)) is the dual of multiply — brightens the result. It's equivalent to 1 − multiply(1−src, 1−dst).

---

## Tone Mapping / Color Atoms

### tonemap (cross-domain alias: `compress-range`, `tone-compress`, `dynamic-range-reduce`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Compress a high dynamic range (HDR) radiance value to the display's limited [0,1] range. Common operators: Reinhard, ACES, Filmic, Durandeau.
**Atom or composite:** Composite: each operator has a specific non-linear mapping function — a chain of polynomial, exponential, and logarithmic operations.
**Cost model:** A handful of arithmetic operations per pixel. Moderate.
**Real wall?** Yes — tone mapping is a lossy operation. The dynamic range of the scene cannot be fully represented on the display. The compression ratio IS the conserved quantity — you're trading compression artifact for dynamic range.
**Cross-domain wiring:** Logarithmic tone mapping = log transform = same as log-odds in probability. ACES = a specific filmic curve = same as the sigmoid transform in neural networks. In retrieval: score normalization via CDF = tone mapping for retrieval scores.
**Notes:** The Reinhard operator: L_out = L / (1 + L). This compresses highlights while preserving midtones. ACES (Academy Color Encoding System) is the film industry standard — it's the most perceptually accurate for cinematics.

### gamma-correct (cross-domain alias: `gamma-encode`, `sRGB-transform`, `power-transform`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Apply γ encoding to a linear radiance value: V_out = V_in^(1/2.2) for gamma 2.2, or the piecewise sRGB formula for displays. The inverse (gamma decode) applies V^2.2.
**Atom or composite:** Composite: check if V <= 0.0031308 → if yes: V·12.92, else: 1.055·V^(1/2.4) − 0.055.
**Cost model:** One power (or approximation via table lookup) + a couple of multiplies. Very cheap.
**Real wall?** No.
**Cross-domain wiring:** Gamma encoding is a power-law transform. Log encoding is a related but different non-linear transform. In information theory: log transforms convert multiplicative noise to additive noise — same as Weber-Fechner law in human perception (logarithmic sensitivity).
**Notes:** Human perception of luminance is roughly logarithmic — gamma encoding makes the encoding perceptually uniform. sRGB is the standard for displays because it approximates the human visual system's response.

---

## Vello / Vector Path Atoms

### stroke (cross-domain alias: `outline`, `draw-curve`, `trace`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Render a variable-width outline along a path. The path is a series of Bézier segments; the stroke applies a thickness perpendicular to the path direction at each point.
**Atom or composite:** Composite: tessellate-path → expand-by-thickness → rasterize-outline.
**Cost model:** Tessellation is the dominant cost. Dash patterns and line caps/joins add conditional complexity.
**Real wall?** No.
**Cross-domain wiring:** Stroking a Bézier curve = tracing a path in the plane = generating a contour from an implicit surface. In retrieval: path traversal in a graph = following edges = same "follow a path" structure.
**Notes:** Stroke join styles (miter, round, bevel) determine how corners are handled — miter can be arbitrarily sharp, leading to long spikes on acute angles. This is the miter limit — a real wall on the stroke primitive.

### fill (cross-domain alias: `solidify`, `interior-fill`, `paint`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Determine the interior of a path and fill it with a solid color or gradient. Uses the even-odd rule or non-zero winding rule.
**Atom or composite:** Composite: point-in-polygon test + flood fill. The tessellation converts paths to triangles, which are then rasterized.
**Cost model:** Tessellation to triangles is the dominant cost. Modern GPUs have hardware path tessellation.
**Real wall?** No.
**Cross-domain wiring:** Point-in-polygon = point-in-set = membership test. In retrieval: "is this document in the relevance set?" = fill/membership test. Non-zero winding = counting how many times a path winds around a point = analogous to complex analysis residue theorem.
**Notes:** The winding rule handles self-intersecting paths correctly — counting how many times the path winds around the test point is equivalent to computing the winding number, which is the integral of the path tangent around the point.

### compose-transform (cross-domain alias: `affine-map`, `matrix-transform`, `projective-warp`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Apply a 2D affine transformation (translate, rotate, scale, shear) or projective transformation to a path, stroke, or fill. Represented as a 3×3 matrix operating on homogeneous coordinates.
**Atom or composite:** Composite: multiply each vertex position by the transformation matrix.
**Cost model:** 6 multiplications + 4 additions per vertex (affine). Projective is 8 multiplications + 8 additions (divide by z).
**Real wall?** No.
**Cross-domain wiring:** All linear transformations = matrix multiply in linear algebra. Affine transforms = linear transform + translation = homogeneous matrix multiply. In retrieval: query reformulation via transformation = applying a learned re-weighting = a matrix multiply on the query vector.
**Notes:** The projective transform (homography) can represent perspective foreshortening — this is what maps a flat document to the camera image in perspective correction.

---

## Summary: Graphics Atom → Cross-Domain Wiring

| Graphics Primitive | Retrieval Alias | Signal Alias | Linear Algebra Alias |
|---|---|---|---|
| scan-convert | bucketing, discretization | ADC sampling | grid quantization |
| lerp / blend | score interpolation | linear interpolation | convex combination |
| over (Porter-Duff) | rank fusion | signal addition with attenuation | Bayesian update |
| dot (N·L) | cosine similarity | matched filter | inner product |
| normalize | L2-normalize scores | normalize signal power | unit vector |
| smoothstep | sigmoid / softmax | smooth threshold | Hermite interpolation |
| mipmap-select | coarse-tier retrieval | coarse-resolution scan | multi-resolution basis select |
| mipmap-generate | corpus summarization | downsampling | pyramid / Laplacian |
| ray-AABB-intersect | range query | interval test | box constraint check |
| BVH-traverse | graph traversal with pruning | tree search | depth-first search |
| anisotropic-sample | query-adaptive refinement | non-uniform sampling | importance sampling |
| tonemap | score normalization | dynamic range compression | CDF transform |
| stroke | path following | trace outline | follow gradient direction |
| fill | set membership | flood fill | indicator function |

---

*Last updated: 2026-06-21*
*Source doctrine: The Painted Fence — Jesse*


---

## Path Tracing / Global Illumination Atoms

### path-trace (cross-domain alias: `Monte-Carlo-integration`, `recursive-raytrace`, `BDPT`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Trace rays from the camera through each pixel; at each bounce, sample a random direction from the BSDF. Each path contributes weight = (emissive + reflected) / PDF. Average many samples per pixel to converge.
**Atom or composite:** Composite: generate ray → intersect scene → if hit light: return emission; else: sample BSDF → generate new ray → recurse → accumulate.
**Cost model:** O(pixels × samples × bounces × complexity). Each sample is expensive. Variance reduction techniques (importance sampling, NEE) are critical.
**Real wall?** Yes — the convergence rate of naive path tracing is O(1/√N) per pixel. Caustics converge very slowly because they require paths that hit specific paths (light → specular → diffuse). This is the fundamental cost of unbiased rendering.
**Cross-domain wiring:** Path tracing = Monte Carlo integration over the space of light paths. In physics: path integral formulation of quantum mechanics = same thing. In retrieval: exploring the retrieval graph with random walks = path tracing in the document graph.
**Notes:** The rendering equation is an integral equation — path tracing is the Monte Carlo estimator for it. Russian roulette termination prevents bias while keeping the estimator consistent.

### NEE (cross-domain alias: `next-event-estimation`, `light-sample`, `direct-illum`)
**Domain:** Graphics / Rendering / LOD
**Definition:** At each diffuse bounce, explicitly sample the light source (sample a point on an emissive surface) in addition to the BSDF sample. This dramatically reduces variance for direct lighting.
**Atom or composite:** Composite: at each diffuse hit: sample light source directly (MIS weight) + BSDF sample → combine via multiple importance sampling.
**Cost model:** One extra ray cast to sample the light per bounce. The cost is usually worth it — variance reduction is typically 10-100× for interior scenes.
**Real wall?** No. But requires knowing which surfaces are emissive and their areas — the light sampling distribution must be representable in the renderer.
**Cross-domain wiring:** NEE = importance sampling in Monte Carlo integration. In retrieval: NEE = sampling directly from the most relevant documents (importance sampling) vs random walk sampling.
**Notes:** Multiple importance sampling (MIS) combines light sampling and BSDF sampling with optimal weights: the Veach-style MIS weight = w_i / (Σ w_j) gives minimum variance.

### photon-map (cross-domain alias: `caustic-trace`, `two-pass-GI`, `progressive-photon`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Two-pass algorithm: (1) trace photons from lights, building a photon map (KD-tree of photon positions and flux); (2) trace rays from camera, gathering nearby photons at each hit. Caustics are captured because photons accumulate on specular surfaces.
**Atom or composite:** Composite: pass1: emit photons → bounce on surfaces → store in photon map. pass2: trace camera rays → gather nearby photons → estimate irradiance.
**Cost model:** Pass 1: many photon paths (100K-10M). Pass 2: for each camera hit, query KD-tree (log N). Progressive photon mapping reduces memory.
**Real wall?** Yes — the number of photons needed for sharp caustics is very high. The photon density in the caustic region must be sufficient to resolve the pattern.
**Cross-domain wiring:** Photon mapping = particle transport in physics (neutron transport). The photon map = a density estimator = a kernel density estimation problem. In retrieval: photon mapping = building a corpus of document exemplars and gathering around them.
**Notes:** The kernel width in photon gathering must be tuned — too wide blurs details, too narrow introduces noise. Progressive photon mapping iteratively tightens the kernel to refine caustics.

### BDPT (cross-domain alias: `bidirectional-pathtrace`, `light-path-trace`, `metropolis-light`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Trace paths from both the camera and the lights; connect sub-paths with a shadow ray. Handles all light transport paths including caustics, SDS (specular-diffuse-specular), and glow from difficult-to-reach light sources.
**Atom or composite:** Composite: generate camera sub-paths (depth k) + light sub-paths (depth l) → for each pair: connect endpoints → evaluate path contribution → MIS combine.
**Cost model:** O(2^d) combinations for paths of depth d. Much more expensive than unidirectional path tracing but handles more paths correctly.
**Real wall?** No. But the number of path combinations grows quickly — practical BDPT uses only a few bounces from each side.
**Cross-domain wiring:** BDPT = exploring the full space of document-to-document connections. In physics: it's the path integral over all possible light paths. In retrieval: it samples both query-centric and corpus-centric paths.
**Notes:** Metropolis light transport (MLT) improves BDPT by mutating paths that make large contributions — it concentrates samples where they matter most (similar to importance sampling).

---

## PBR / BRDF Atoms

### brdf-evaluate (cross-domain alias: `reflectance-model`, `microfacet`, `Cook-Torrance`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Bidirectional Reflectance Distribution Function — describes how light is scattered at a surface. Cook-Torrance BRDF: f_r = D·F·G / (4·(n·o)(n·i)). Components: D (microfacet normal distribution), F (Fresnel), G (geometric shadowing).
**Atom or composite:** Composite: D(microfacet normal distribution) + F(Fresnel term) + G(geometric attenuation) → combine.
**Cost model:** One evaluation per BRDF sample per light source per shading point. The D, F, G each involve several arithmetic operations.
**Real wall?** Yes — the BRDF model is a simplification. Real surfaces have wavelength dependence (color), anisotropy (brushed metal), subsurface scattering (skin), and temporal effects (wet surfaces) that simple BRDFs miss.
**Cross-domain wiring:** BRDF = the reflectance operator for a surface. In retrieval: the retrieval score function = the "BRDF" of the document space — it describes how the "query light" is scattered into "document reflections."
**Notes:** The GGX (Trowbridge-Reitz) microfacet distribution is the current standard — it produces physically plausible long tails that the older Blinn-Phong distribution lacks.

### diffuse-brdf (cross-domain alias: `Lambertian`, `Oren-Nayar`, `rough-diffuse`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Lambertian (ideal diffuse): f_r = albedo/π. Oren-Nayar: accounts for rough surfaces by modeling the distribution of microfacet slopes. Produces a characteristic "retro-reflective" appearance.
**Atom or composite:** Composite (for Oren-Nayar): compute roughness parameters A, B → evaluate with dot products of normals and directions.
**Cost model:** Slightly more expensive than Lambertian (a few extra dot products) but well worth it for rough surfaces.
**Real wall?** No. But Oren-Nayar parameters must match the actual surface roughness — using wrong roughness values produces physically incorrect shading.
**Cross-domain wiring:** Diffuse BRDF = cosine-weighted hemispherical integration = the same as the Lambertian scattering model in radiative transfer. In retrieval: diffuse-like scoring = uniform relevance spread across multiple documents.
**Notes:** The π in Lambertian is the normalization factor for the hemispherical integral of the cosine — it ensures energy conservation (the integral of f_r·(n·i) over the hemisphere = albedo).

### fresnel-reflect (cross-domain alias: `Schlick-approx`, `dielectric-reflect`, `F0`)
**Domain:** Graphics / Rendering / LOD
**Definition:** The Fresnel equations give the fraction of light reflected vs transmitted at a dielectric interface. Schlick's approximation: F(v,n) = F₀ + (1−F₀)·(1−(v·n))⁵ where F₀ = ((n₁−n₂)/(n₁+n₂))².
**Atom or composite:** Composite: compute F₀ from IORs → compute Schlick term → mix reflection/transmission.
**Cost model:** One subtract, one power (5), two multiplies. Negligible.
**Real wall?** No. But the Schlick approximation breaks down for very oblique angles on high-index surfaces.
**Cross-domain wiring:** Fresnel = angle-dependent reflectivity = Snell's law for reflection. In retrieval: angle-dependent retrieval = query-dependent scoring (the more aligned the query and document, the higher the "Fresnel" reflection).
**Notes:** Metals have a complex IOR and F₀ is colored (gold has yellowish F₀, silver whitish). Dielectrics have low F₀ (typically 0.02-0.05) that is spectrally neutral.

### GGX-distrib (cross-domain alias: `Trowbridge-Reitz`, `microfacet-normal`, `roughness-alpha`)
**Domain:** Graphics / Rendering / LOD
**Definition:** The GGX microfacet normal distribution: D(m) = α² / (π·(n·m)⁴·(α² + tan²(m))²). Produces a sharp peak with long tails — unlike Blinn-Phong's sharper-than-physical peak.
**Atom or composite:** Atom (the mathematical form). Implementation: one division, a few multiplies.
**Cost model:** One D evaluation per sample — very cheap.
**Real wall?** No. But α = roughness² is the convention; using α directly produces the wrong distribution.
**Cross-domain wiring:** GGX = the Student-t distribution in statistics (with ν = 2). The "roughness" parameter maps to the degrees of freedom. In retrieval: GGX-like scoring = heavy-tailed relevance distributions.
**Notes:** GGX with Smith G term (G1) gives the complete GGX BRDF. The "Trowbridge-Reitz" distribution is identical to the Beckmann distribution with different parameterization (α = σ√2).

### SSS (cross-domain alias: `subsurface-scatter`, `diffusion-profile`, `translucency`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Light that penetrates beneath the surface and scatters before exiting. The diffusion approximation models SSS as a dipole diffusion profile: R(r) = (σ_s·P(r) + σ_a) where P(r) is a modified Bessel function falloff.
**Atom or composite:** Composite: estimate subsurface color (from IOR and albedo) → compute diffusion profile → integrate over the exit point neighborhood.
**Cost model:** One diffusion approximation per shading point is cheap. Physically accurate SSS requires computing the dipole solution (more expensive).
**Real wall?** Yes — the diffusion approximation breaks down near boundaries and for thin geometry. The "thin" vs "thick" scattering regimes require different models.
**Cross-domain wiring:** SSS = random walk in a semi-transparent medium = same as light transport in fog/mist. In retrieval: semantic "translucency" = information that penetrates surface meanings.
**Notes:** The Jensen dipole model is the standard subsurface approximation: it uses two image sources (one above, one below the surface) to model the diffusion profile.

---

## Texture / Material Atoms

### normal-map (cross-domain alias: `bump-derive`, `gradient-field`, `micro-detail`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Encode surface normal perturbations as RGB (x,y,z encoded to [0,1]). During shading, the normal map perturbs the geometric normal: n' = TBN·normal_map_sample. Tangent space encoding makes it rotation-invariant.
**Atom or composite:** Composite: fetch normal from texture → decode to [-1,1] range → transform by TBN (tangent, bitangent, normal) matrix → use as shading normal.
**Cost model:** One texture fetch + TBN matrix multiply per shading point. Very cheap.
**Real wall?** No. But normal maps cannot represent silhouette bumps (they only perturb the normal, not the geometry) — displacement mapping is needed for silhouette detail.
**Cross-domain wiring:** Normal map = gradient of the height field = gradient in signal processing = gradient descent direction in optimization. In retrieval: normal map = semantic gradient of the relevance surface.
**Notes:** The TBN (tangent-bitangent-normal) matrix transforms from tangent space to world space. It must be computed per-vertex and interpolated (or computed per-fragment using derivatives).

### displacement-map (cross-domain alias: `vertex-displace`, `heightfield-raster`, `tessellate-displace`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Displace vertices along the surface normal by a height value. Requires tessellation (subdivide triangles, then displace). Produces correct silhouettes and self-shadowing — unlike normal maps.
**Atom or composite:** Composite: tessellate mesh → for each vertex: read displacement map → offset position along normal.
**Cost model:** Tessellation is expensive — adds geometry. Dynamic tessellation (per-frame level adjustment) is even more expensive.
**Real wall?** Yes — tessellation adds triangles. The displacement map resolution must match the tessellation level or artifacts appear. Overly dense tessellation wastes GPU work on sub-pixel geometry.
**Cross-domain wiring:** Displacement mapping = computing the surface integral on a perturbed surface. In retrieval: displacement = semantic displacement from the base topic plane.
**Notes:** Parallax occlusion mapping (POM) approximates displacement mapping without tessellation — by ray marching through a height field in the fragment shader. Cheaper but not as accurate.

### ambient-occlusion (cross-domain alias: `SSAO`, `horizon-occlude`, `cavity-darken`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Darken creases and cavities where ambient light is occluded by nearby geometry. SSAO: sample hemisphere around the shading point, compare depth to see how many samples are occluded. HBAO: integrate horizon angles.
**Atom or composite:** Composite: sample hemisphere → for each sample: compare depth → accumulate occlusion → normalize.
**Cost model:** Full SSAO is expensive (many hemisphere samples per pixel). Half-resolution SSAO + bilateral blur is the common optimization.
**Real wall?** Yes — SSAO radius trades off scale vs quality: large radius covers large cavities but needs more samples; small radius misses large-scale occlusion.
**Cross-domain wiring:** AO = measuring the "solid angle" of occlusion = the integral of the occlusion function over the hemisphere. In retrieval: AO = the "relevance occlusion" — how much of the surrounding context supports a document's relevance.
**Notes:** GTAO (Ground Truth Ambient Occlusion) uses ray marching through a signed distance field — it gives more accurate results than SSAO's hemisphere sampling and works at any distance.

### roughness-texture (cross-domain alias: `pbr-roughness`, `micro-roughness`, `gloss-map`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Per-pixel roughness value controlling the microfacet distribution (D term in Cook-Torrance). Roughness = 0 → mirror-like. Roughness = 1 → very diffuse. Often derived from a grayscale texture.
**Atom or composite:** Composite: sample roughness texture → use as α parameter in GGX/Beckmann distribution.
**Cost model:** One texture fetch per shading point. Negligible.
**Real wall?** No. But the roughness distribution in the texture matters — uniform roughness is simple; anisotropic roughness (brushed metal) requires a 2D roughness field.
**Cross-domain wiring:** Roughness texture = uncertainty map of the surface orientation = variance field. In retrieval: roughness = the variance of the relevance estimate for a document.
**Notes:** The mipmap chain for roughness must be computed correctly — the average of roughness is not the roughness of the average. Better to store pre-squared roughness (α²) to avoid mipmap artifacts.

---

## Acceleration Structures Atoms

### kd-tree-build (cross-domain alias: `spatial-partition`, `median-split`, `surface-area-heuristic`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Recursively split the scene along the median of the longest axis. The SAH heuristic chooses the split position to minimize the sum of (left_area × left_count + right_area × right_count) × traversal_cost.
**Atom or composite:** Composite: find longest axis → evaluate SAH at candidate splits → pick best → partition triangles → recurse.
**Cost model:** O(N log N) with SAH. Naive median split is O(N log N) but doesn't minimize traversal cost. GPU kd-trees are a research area.
**Real wall?** Yes — building a high-quality kd-tree is offline work. Real-time kd-tree construction (for dynamic scenes) is still an open problem.
**Cross-domain wiring:** kd-tree = recursive binary space partitioning = decision tree learning. The SAH is the same as the split criterion in CART (Classification and Regression Tree).
**Notes:** BVH and kd-tree are the two dominant acceleration structures. BVH is easier to build and update (dynamic scenes); kd-tree has faster ray traversal for static scenes.

### ray-bvh-build (cross-domain alias: `bound-build`, `SAH-construction`, `bin-partition`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Bounding Volume Hierarchy: recursively partition primitives into two groups, each enclosed in an AABB. Built top-down using SAH binning (partition the bounding box into bins, evaluate SAH per bin, pick best split).
**Atom or composite:** Composite: compute bounding box of all primitives → bin the centroids → evaluate SAH per bin → split at best bin → recurse.
**Cost model:** O(N log N). SAH evaluation at each level is O(N) — the total is O(N log N). Incremental rebuilds (for dynamic scenes) can be faster.
**Real wall?** No. But poor triangle clustering (e.g., long thin triangles spanning many BVH nodes) degrades the hierarchy.
**Cross-domain wiring:** BVH = hierarchical clustering. In retrieval: BVH on document vectors = hierarchical document clustering (like a dendrogram). Traversal = traversing the cluster tree.
**Notes:** The "flattened" BVH (one array for nodes, one for primitives) is the cache-friendly layout. The "tight" BVH recomputes the bounding box of children (more compact). Most GPUs use the flattened layout.

### spatial-accel (cross-domain alias: `octree-traverse`, `grid-traverse`, `uniform-grid`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Acceleration via spatial partitioning: uniform grid (world divided into cells), octree (recursive 8-way split), or spatial kd-tree. Rays traverse by stepping through cells.
**Atom or composite:** Composite: determine entry cell → while cell is inside object: test intersection with objects in cell → compute next cell boundary → advance.
**Cost model:** Grid traversal: one cell test per step. The step size depends on ray direction — diagonal rays step through more cells.
**Real wall?** Yes — uniform grids are inefficient for non-uniform scenes (sparse regions waste cells, dense regions cause many cell tests). Octrees adapt to non-uniform density.
**Cross-domain wiring:** Uniform grid = the hash bucket structure in streaming algorithms. Octree = the quadtree in image processing = the spatial index in databases.
**Notes:** The hierarchical depth buffer (Hi-Z) in modern GPUs is a form of uniform grid — mip levels of the depth buffer serve as coarse spatial indices for early z rejection.

### sort-ray (cross-domain alias: `bundle-sort`, `coherent-bundle`, `GPU-traverse`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Sort rays by the object they hit first so coherent rays are processed together. On GPUs: group rays that hit the same objects to maximize shared memory usage and reduce divergence.
**Atom or composite:** Composite: for each ray: compute first-hit object → sort rays by hit object ID → shade groups of coherent rays together.
**Cost model:** One sort per frame (O(R log R) for R rays). The benefit must outweigh the sorting cost.
**Real wall?** No. But sorting adds latency — rays must be sorted before shading, which prevents immediate shading of the first hit.
**Notes:** Packet tracing (SIGGRAPH 2001) was the original coherent ray grouping technique. Modern GPU ray tracing (RTX) uses dedicated hardware for BVH traversal, making the sorting less critical.

---

## Post-Processing Atoms

### bloom (cross-domain alias: `glow`, `highlight-extract`, `Gaussian-blur-threshold`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Extract pixels above a luminance threshold → Gaussian blur → add back to original image. Creates a glow around bright areas.
**Atom or composite:** Composite: threshold(original) → Gaussian blur → add(blurred, original × intensity).
**Cost model:** One threshold pass + two Gaussian blurs (vertical + horizontal separable) + one add. The blur is the dominant cost.
**Real wall?** No. But bloom radius must match the display resolution — at 4K, a 10-pixel bloom radius looks different than at 1080p.
**Cross-domain wiring:** Bloom = diffusion of bright energy = the same as spreading activation in physics and retrieval. The threshold = the activation threshold of the spreading.
**Notes:** The bloom threshold (what luminance level triggers bloom) is a key artistic control. Physically accurate bloom thresholds are very high; game bloom thresholds are usually much lower for stylistic effect.

### depth-of-field (cross-domain alias: `DOF`, `circle-of-confusion`, `bokeh`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Model the finite aperture of a camera — only rays within the aperture circle at the focal plane are in focus. Defocused rays create a circle of confusion (CoC) whose diameter depends on defocus amount.
**Atom or composite:** Composite: compute CoC diameter from depth relative to focal plane → sample within CoC → average. Or: scatter samples from the lens aperture to compute per-pixel color.
**Cost model:** Many samples per pixel are needed for smooth bokeh. The number of lens samples = the number of rays through the aperture.
**Real wall?** Yes — the bokeh shape (circular, hexagonal, etc.) depends on the aperture blade count. Very fast lenses with few blades produce distinctive bokeh shapes that are hard to simulate.
**Cross-domain wiring:** DOF = focus / unfocus = the same as the precision / recall trade-off in retrieval. Focus = narrow CoC = high precision, low recall.
**Notes:** The CoC diameter = aperture × |focal_distance − object_distance| / object_distance. Objects at the focal distance have CoC = 0.

### motion-blur (cross-domain alias: `temporal-sample`, `velocity-blur`, `shutter-integration`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Average the scene over the shutter open time to capture motion blur. Each pixel integrates all objects that pass through it during the exposure.
**Atom or composite:** Composite: sample time uniformly within shutter interval → render at each sample → average.
**Cost model:** Number of time samples × cost of rendering at each time. 8-16 samples are needed for smooth results.
**Real wall?** No. But object velocity must be tracked through the shutter interval — objects can exit the frame or be occluded differently at different times.
**Cross-domain wiring:** Motion blur = temporal integration = the same as the temporal averaging that happens in retrieval over a session.
**Notes:** Subpixel jitter (randomizing sample position within the pixel) plus time sampling creates a good MTF (modulation transfer function) for motion.

### TAA (cross-domain alias: `temporal-anti-alias`, `history-clamp`, `velocity-reproject`)
**Domain:** Graphics / Rendering / LOD
**Definition:** Temporal anti-aliasing: reproject the previous frame to the current frame using per-pixel velocity → clamp the current sample to the neighborhood of the history sample → mix with current frame.
**Atom or composite:** Composite: reproject(prev_color, velocity) → clip to neighborhood of reprojected → blend(current, clamped_history).
**Cost model:** One velocity buffer read + neighborhood clamp + blend. Very cheap relative to the rendering cost.
**Real wall?** Yes — TAA introduces ghosting when objects move or disappear. The neighborhood clamping limits ghosting but also limits the temporal accumulation, reducing the anti-aliasing benefit.
**Cross-domain wiring:** TAA = exponential moving average over frames with adaptive clipping = same as the EMA for streaming statistics. In retrieval: TAA = relevance smoothing over time with clipping.
**Notes:** The velocity (motion vector) is the 2D screen-space displacement of each pixel from the previous frame. Computing this correctly is non-trivial for complex scenes.

---

*Last updated: 2026-06-21 (expanded with path tracing, PBR, texture, acceleration, post-processing)*
*Source doctrine: The Painted Fence — Jesse*
