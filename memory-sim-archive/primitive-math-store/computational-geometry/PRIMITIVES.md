# Computational Geometry Primitives

> Domain: Computational Geometry — algorithms for spatial computation, geometric structures, and proximity problems.

## Primitive Catalog

---

### CG.001: Convex Hull (2D)
**Definition:** Compute the minimal convex polygon containing all points in a 2D set.
**Cost Model:** O(n log n) — sort by x-coordinate, then Graham scan or Andrew's monotone chain.
**Real Wall:** Sorting dominates; Graham scan is linear after sort.
**Cross-Domain Aliases:** CH₂, hull2d, convex_envelope
**Notes:** Gift wrapping (Jarvis march) is O(nh) — better when h << n.

---

### CG.002: Convex Hull (3D)
**Definition:** Compute the minimal convex polyhedron containing all points in 3D space.
**Cost Model:** O(n log n) average; O(n²) worst-case for degenerate inputs.
**Real Wall:** Numerical precision issues near coplanar points.
**Cross-Domain Aliases:** CH₃, hull3d, convex_polyhedron
**Notes:** QuickHull and randomized incremental construction are practical choices.

---

### CG.003: Point-in-Polygon (Ray Casting)
**Definition:** Determine if a point lies inside a polygon by counting edge crossings.
**Cost Model:** O(n) per query; O(n) preprocessing for simple polygons.
**Real Wall:** Edge cases at vertices require careful handling.
**Cross-Domain Aliases:** pip, inside_polygon, point_enclosure
**Notes:** Winding number method is more robust at boundary points.

---

### CG.004: Point-in-Polygon (Winding Number)
**Definition:** Count total winding of polygon edges around point; nonzero = inside.
**Cost Model:** O(n) per query; numerically stable.
**Real Wall:** Handles all boundary cases correctly including coincident vertices.
**Cross-Domain Aliases:** wn_pip, winding_test, enclosed_by_winding
**Notes:** Preferred over ray casting for precision-critical applications.

---

### CG.005: Graham Scan
**Definition:** Convex hull algorithm: sort points radially from lowest point, then scan removing right turns.
**Cost Model:** O(n log n) dominated by sorting.
**Real Wall:** Collinear points require special handling to avoid degenerate hulls.
**Cross-Domain Aliases:** graham_scan, radial_scan_hull
**Notes:** Produces counterclockwise hull from anchor point.

---

### CG.006: Andrew's Monotone Chain
**Definition:** Compute upper and lower hulls separately; concatenate for full convex hull.
**Cost Model:** O(n log n) with straightforward implementation.
**Real Wall:** Fewer edge cases than Graham scan; easier to make robust.
**Cross-Domain Aliases:** monotone_chain, andrew_hull, lower_upper_hull
**Notes:** The standard choice for production convex hull implementations.

---

### CG.007: Jarvis March (Gift Wrapping)
**Definition:** Start at leftmost point; repeatedly find most counterclockwise point to wrap hull.
**Cost Model:** O(nh) where h is number of hull vertices.
**Real Wall:** Excellent when hull has few vertices relative to n.
**Cross-Domain Aliases:** jarvis_march, gift_wrap, wrap_around
**Notes:** Natural parallelization; each vertex search is independent.

---

### CG.008: Kirkpatrick-Seidel Convex Hull
**Definition:** Output-sensitive convex hull algorithm achieving O(n log h).
**Cost Model:** O(n log h) where h is hull vertices; optimal for output sensitivity.
**Real Wall:** Complex implementation; justified only when h is known to be small.
**Cross-Domain Aliases:** kirkpatrick_seidel, upper_lower_hull_prune
**Notes:** Useful in computational geometry lower bound proofs.

---

### CG.009: Chazelle's Triangulation
**Definition:** Triangulate a simple polygon in O(n) time.
**Cost Model:** O(n) — linear time triangulation.
**Real Wall:** Highly complex; impractical except for theoretical bounds.
**Cross-Domain Aliases:** linear_triangulation, chazelle_triangulate
**Notes:** Demonstrates triangulation is not inherently quadratic.

---

### CG.010: Ear Clipping Triangulation
**Definition:** Repeatedly clip ears (triangles with one interior angle < 180°) from polygon.
**Cost Model:** O(n²) worst-case; O(n) for monotone polygons.
**Real Wall:** Simple and practical for moderate polygon sizes.
**Cross-Domain Aliases:** ear_clip, ear_cut_triangulation
**Notes:** Works for any simple polygon with ≥3 vertices.

---

### CG.011: Delaunay Triangulation
**Definition:** Triangulation maximizing minimum angle; no points inside circumcircles.
**Cost Model:** O(n log n) via divide-and-conquer or flip algorithms.
**Real Wall:** Preferred over arbitrary triangulations for interpolation.
**Cross-Domain Aliases:** DT, delaunay_mesh, circumcircle_property
**Notes:** Dual to Voronoi diagram; maximizes mesh quality.

---

### CG.012: Constrained Delaunay Triangulation
**Definition:** Delaunay triangulation respecting existing edges as constraints.
**Cost Model:** O(n log n + k) where k is number of constraints.
**Real Wall:** Maintains Delaunay properties where possible; violates locally at constraints.
**Cross-Domain Aliases:** CDT, constrained_delaunay, cdt_triangulate
**Notes:** Essential for mesh generation with boundary constraints.

---

### CG.013: Voronoi Diagram (2D)
**Definition:** Partition plane into regions closest to each input point; boundaries are perpendicular bisectors.
**Cost Model:** O(n log n) via Fortune's sweep or divide-and-conquer.
**Real Wall:** Each region is a convex polygon (possibly unbounded).
**Cross-Domain Aliases:** VD, voronoi, nearest_region_partition
**Notes:** Dual to Delaunay triangulation.

---

### CG.014: Voronoi Diagram (3D)
**Definition:** 3D partition into cells closest to each site; boundaries are portions of perpendicular bisecting planes.
**Cost Model:** O(n^(3/2)) expected; O(n²) worst-case.
**Real Wall:** Higher-dimensional diagrams are exponentially more complex.
**Cross-Domain Aliases:** voronoi3d, spatial_voronoi, 3d_voronoi
**Notes:** Used in protein structure analysis and materials science.

---

### CG.015: Farthest-Point Voronoi Diagram
**Definition:** Voronoi diagram of farthest-point regions; each cell is set of points farthest from a site.
**Cost Model:** O(n log n); computed from convex hull vertex order.
**Real Wall:** All regions are bounded; dual is convex hull.
**Cross-Domain Aliases:** fvd, farthest_voronoi, max_distance_voronoi
**Notes:** Useful for facility location problems.

---

### CG.016: Order-k Voronoi Diagram
**Definition:** Each region is set of points whose k nearest sites are a specific set.
**Cost Model:** O(k² n log n) for general k; space complexity is O(k² n).
**Real Wall:** Exponential in k for naive computation.
**Cross-Domain Aliases:** order_k_voronoi, kth_nearest_voronoi
**Notes:** k=1 is standard Voronoi; k=2 gives bisecting boundaries.

---

### CG.017: Addel's Algorithm for Voronoi
**Definition:** Incremental insertion of sites into existing Voronoi diagram.
**Cost Model:** O(n log n) expected; O(n²) worst-case.
**Real Wall:** Simple to implement; good for dynamic scenarios.
**Cross-Domain Aliases:** incremental_voronoi, addel_voronoi
**Notes:** Each insertion requires finding affected region and local rebuilding.

---

### CG.018: Bowyer-Watson Algorithm
**Definition:** Incremental Delaunay triangulation by inserting points and retriangulating cavities.
**Cost Model:** O(n²) worst-case; O(n log n) average.
**Real Wall:** Simple and widely used; handles point insertion well.
**Cross-Domain Aliases:** bowyer_watson, incremental_delaunay, delaunay_cavity
**Notes:** The go-to algorithm for incremental Delaunay construction.

---

### CG.019: Fortune's Sweep Line Algorithm
**Definition:** Compute Voronoi diagram using sweep line with beach line as parabola envelope.
**Cost Model:** O(n log n); optimal for planar Voronoi computation.
**Real Wall:** Numerically stable; handles parallel edges gracefully.
**Cross-Domain Aliases:** fortune_sweep, sweep_voronoi, beach_line_voronoi
**Notes:** Produces Voronoi diagram directly without triangulation intermediate.

---

### CG.020: Lloyd's Algorithm (Voronoi Iteration)
**Definition:** Alternately compute Voronoi diagram then centroid of each cell; repeat to equilibrium.
**Cost Model:** O(k · T_voronoi) where k is iterations and T_voronoi is Voronoi computation.
**Real Wall:** Converges slowly near optimal; used for k-means and mesh smoothing.
**Cross-Domain Aliases:** lloyd_iteration, voronoi_iteration, centroidal_voronoi
**Notes:** Guarantees convergence but not in polynomial time.

---

### CG.021: Power Diagram (Laguerre Voronoi)
**Definition:** Weighted Voronoi using power distance: dist²(x, s) - w_s.
**Cost Model:** O(n log n) via sweep or dual triangulation.
**Real Wall:** Weights model sites with different radii or importance.
**Cross-Domain Aliases:** power_diagram, weighted_voronoi, laguerre_diagram
**Notes:** Dual to weighted (regular) triangulation.

---

### CG.022: Medial Axis (2D)
**Definition:** Set of points with ≥2 nearest points on shape boundary; skeleton of the shape.
**Cost Model:** O(n log n) for polygon medial axis via Chordal Axis transform.
**Real Wall:** Computation in 3D is significantly harder.
**Cross-Domain Aliases:** medial_axis, skeleton, centerline
**Notes:** Used for shape matching and morphological analysis.

---

### CG.023: Straight Skeleton
**Definition:** Skeleton formed by shrinking polygon edges while maintaining topology.
**Cost Model:** O(n log n) via heap-based algorithm.
**Real Wall:** Different from medial axis; produces non-intersecting arcs.
**Cross-Domain Aliases:** straight_skeleton, polygonal_skeleton
**Notes:** Useful for roof generation and polygon offsetting.

---

### CG.024: Width of Point Set
**Definition:** Minimum distance between parallel supporting lines of point set.
**Cost Model:** O(n log n) via rotating calipers on convex hull.
**Real Wall:** Equivalent to minimum width of hull's projection.
**Cross-Domain Aliases:** width, minimum_width, pointset_width
**Notes:** Key parameter in approximation algorithms.

---

### CG.025: Minimum Enclosing Rectangle (Rotating Calipers)
**Definition:** Find smallest-area rectangle containing convex polygon; explore all hull orientations.
**Cost Model:** O(n) after convex hull computation.
**Real Wall:** Rectangle need not share edge with hull.
**Cross-Domain Aliases:** mer, minimum_bounding_rectangle, rotating_calipers
**Notes:** Also computes minimum-perimeter rectangle efficiently.

---

### CG.026: Minimum Enclosing Circle
**Definition:** Smallest circle containing all input points.
**Cost Model:** O(n) randomized; O(n²) deterministic worst-case.
**Real Wall:** Determined by either 1 point (radius 0), 2 points (diameter), or 3 points (circumcircle).
**Cross-Domain Aliases:** mec, smallest_enclosing_circle, welzl_algorithm
**Notes:** Welzl's algorithm is practical and fast in expected O(n).

---

### CG.027: Minimum Enclosing Ball (Higher Dimensions)
**Definition:** Smallest sphere containing all points in d dimensions.
**Cost Model:** O(d · n) for coreset-based approximation; exact is exponential in d.
**Real Wall:** NP-hard for arbitrary d; coreset approximations are practical.
**Cross-Domain Aliases:** smallest_enclosing_ball, miniball, mwbd
**Notes:** Welzl's algorithm extends to d dimensions but complexity grows.

---

### CG.028: Minimum Volume Enclosing Ellipsoid
**Definition:** Smallest-volume ellipsoid containing all points.
**Cost Model:** O(d² n) for Khachiyan's algorithm with ε-approximation.
**Real Wall:** John ellipsoid (minimum volume enclosing) has unique properties.
**Cross-Domain Aliases:** mvbe, john_ellipsoid, minimum_volume_ellipsoid
**Notes:** Central to Banach-Mazur distance and convex geometry.

---

### CG.029: Smallest Enclosing Box (OBB)
**Definition:** Minimum-volume oriented bounding box (any rotation allowed).
**Cost Model:** O(n) for candidate orientations from PCA of point cloud.
**Real Wall:** O(n² log n) for exhaustive rotation search.
**Cross-Domain Aliases:** obb, oriented_bounding_box, minimum_volume_box
**Notes:** Principal Component Analysis gives good approximation quickly.

---

### CG.030: Minimum Area Enclosing Parallelogram
**Definition:** Smallest-area parallelogram containing convex polygon.
**Cost Model:** O(n²) by exploring edge pairs and computing optimal placement.
**Real Wall:** Optimal parallelogram shares edges with convex hull subset.
**Cross-Domain Aliases:** mep, minimum_parallelogram
**Notes:** Related to affine transformations of the minimum enclosing rectangle.

---

### CG.031: 2D Range Searching (kd-Tree)
**Definition:** Hierarchical space partitioning for orthogonal and point queries.
**Cost Model:** O(n) build; O(log n + k) query; O(√n + k) worst-case for rectangular range.
**Real Wall:** kd-tree adapts to data distribution automatically.
**Cross-Domain Aliases:** kdtree_range, 2d_range_search, point_kdtree
**Notes:** Optimal for point queries; less so for large rectangular ranges.

---

### CG.032: 2D Range Searching (Range Tree)
**Definition:** Multi-level tree with secondary structures for efficient range queries.
**Cost Model:** O(n log^(d-1) n) build; O(log^(d-1) n + k) query in d dimensions.
**Real Wall:** Generalizes binary search tree to multiple dimensions.
**Cross-Domain Aliases:** range_tree, ddim_range_search, fractional_cascading
**Notes:** Fractional cascading speeds up secondary searches.

---

### CG.033: 2D Range Reporting (Priority Search Tree)
**Definition:** Report all points in rectangle [x₁,x₂] × (-∞, y] efficiently.
**Cost Model:** O(log n + k) per query; O(n log n) space.
**Real Wall:** Combines binary search tree with heap property.
**Cross-Domain Aliases:** priority_search_tree, 2d_partial_range
**Notes:** Fundamental data structure for 2D orthogonal range queries.

---

### CG.034: Range Tree with Fractional Cascading
**Definition:** Accelerate secondary dimension searches by storing shared lookups.
**Cost Model:** O(log^(d-1) n + k) query; O(n log^(d-1) n) space.
**Real Wall:** Reduces constant factors in multi-level range searches.
**Cross-Domain Aliases:** fractional_cascade_tree, cascaded_range_tree
**Notes:** Key optimization for computational geometry databases.

---

### CG.035: Segment Tree
**Definition:** Hierarchical interval decomposition supporting point and interval queries.
**Cost Model:** O(n log n) build; O(log n + k) query for any interval.
**Real Wall:** Each point appears in O(log n) canonical segments.
**Cross-Domain Aliases:** segtree, interval_tree, segment_decomposition
**Notes:** Basis for interval stabbing queries and range updates.

---

### CG.036: Interval Tree
**Definition:** Center-based interval decomposition for efficient overlap queries.
**Cost Model:** O(n log n) build; O(log n + k) query for intervals overlapping point.
**Real Wall:** Faster than segment tree for point queries.
**Cross-Domain Aliases:** interval_tree, center_interval_tree, overlap_tree
**Notes:** Each interval stored at level containing its midpoint.

---

### CG.037: Range Tree (Canonical Segment Method)
**Definition:** Decompose query range into O(log n) disjoint canonical subsets.
**Cost Model:** O(log^(d-1) n + k) query by recursively partitioning.
**Real Wall:** General framework unifying many range search structures.
**Cross-Domain Aliases:** canonical_range_tree, segment_method
**Notes:** Basis for understanding output-sensitive algorithms.

---

### CG.038: Quad Tree
**Definition:** Recursively subdivide 2D space into quadrants; stop when region contains ≤1 point.
**Cost Model:** O(n log n) average build; O(n²) worst-case (degenerate distributions).
**Real Wall:** Simple and intuitive; common in spatial indexing.
**Cross-Domain Aliases:** quadtree, qtree, spatial_quadtree
**Notes:** Variants: point quadtree, region quadtree, MX quadtree.

---

### CG.039: Octree
**Definition:** 3D extension of quadtree; subdivide cubes into 8 octants.
**Cost Model:** O(n log n) average build; O(n²) worst-case.
**Real Wall:** Standard structure for 3D spatial partitioning.
**Cross-Domain Aliases:** octree, 3d_octree, spatial_octree
**Notes:** Essential for 3D collision detection and mesh generation.

---

### CG.040: BSP Tree (Binary Space Partition)
**Definition:** Recursively split space with hyperplanes; create binary tree of regions.
**Cost Model:** O(n log n) average for n cuts; NP-hard optimal.
**Real Wall:** Good for visibility and collision detection.
**Cross-Domain Aliases:** bsp_tree, binary_space_partition, hyperplane_split
**Notes:** Order of cuts affects tree balance; heuristics needed.

---

### CG.041: R-Tree
**Definition:** Height-balanced tree grouping nearby rectangles in minimum bounding rectangles.
**Cost Model:** O(n log n) build (bulk loading); O(log n + k) query.
**Real Wall:** Rectangles may overlap; quality depends on insertion algorithm.
**Cross-Domain Aliases:** rtree, r_tree, spatial_index_tree
**Notes:** Standard in GIS databases and spatial extensions of SQL.

---

### CG.042: R+-Tree
**Definition:** R-tree variant where parent MBRs are split to prevent overlap.
**Cost Model:** O(n log n) build; O(log n + k) query with guaranteed non-overlap.
**Real Wall:** More space but faster point query (no need to search multiple paths).
**Cross-Domain Aliases:** rplus_tree, nonoverlap_rtree
**Notes:** Useful for window queries where overlap hurts performance.

---

### CG.043: R*-Tree
**Definition:** R-tree with re-insertion on overflow and optimized node split criteria.
**Cost Model:** O(n log n) build; better query performance than R-tree.
**Real Wall:** Re-insertion reduces tree depth and overlap.
**Cross-Domain Aliases:** rstar_tree, optimized_rtree
**Notes:** Best general-purpose R-tree variant in practice.

---

### CG.044: Hilbert R-Tree
**Definition:** R-tree variant using Hilbert space-filling curve ordering for bulk loading.
**Cost Model:** O(n log n) bulk load; O(log n + k) query.
**Real Wall:** Hilbert curve preserves locality better than Z-order.
**Cross-Domain Aliases:** hilbert_rtree, hrtree, hilbert_curve_rtree
**Notes:** Excellent for range queries on disk; used in many DBMS.

---

### CG.045: Point Location (Slab Decomposition)
**Definition:** Preprocess planar subdivision into vertical slabs; binary search per slab.
**Cost Model:** O(n log n) preprocess; O(log n) query.
**Real Wall:** Simple to implement; space is O(n log n).
**Cross-Domain Aliases:** slab_decomposition, vertical_decomposition
**Notes:** Each slab stores edges crossing it in sorted order.

---

### CG.046: Point Location (Trapezoidal Map)
**Definition:** Randomized incremental construction of trapezoidal decomposition; search structure for queries.
**Cost Model:** O(n log n) expected build; O(log n) query.
**Real Wall:** Query structure is a directed acyclic graph, not a tree.
**Cross-Domain Aliases:** trapezoidal_map, randomized_ploc
**Notes:** Basis for planar point location in computational geometry libraries.

---

### CG.047: Point Location (Jump & Walk)
**Definition:** Heuristic: start from arbitrary triangle, walk toward query point crossing edges.
**Cost Model:** O(√n) expected; O(n) worst-case.
**Real Wall:** Simple and adaptive; no preprocessing needed.
**Cross-Domain Aliases:** walk_ploc, naive_walk, point_location_walk
**Notes:** Surprisingly practical for many applications.

---

### CG.048: Kirkpatrick's Planar Point Location
**Definition:** Successively peel outer faces; store triangulation hierarchy for O(log n) query.
**Cost Model:** O(n) preprocess; O(log n) query; O(n) space.
**Real Wall:** Complex but theoretically optimal preprocessing/query tradeoff.
**Cross-Domain Aliases:** kirkpatrick_ploc, hierarchical_planar_pointloc
**Notes:** Theoretical interest; rarely used in practice.

---

### CG.049: Segment Intersection Detection
**Definition:** Determine if any two segments among n intersect.
**Cost Model:** O(n log n) via sweep line; O(n + k) reporting if k intersections exist.
**Real Wall:** Sweep line handles general position; boundary cases need care.
**Cross-Domain Aliases:** segment_intersection, line_intersection_test, si_detect
**Notes:** Foundation for many arrangement algorithms.

---

### CG.050: Segment Intersection Reporting (Bentley-Ottmann)
**Definition:** Report all k intersections among n line segments.
**Cost Model:** O((n + k) log n) via event queue with heap.
**Real Wall:** More expensive than detection but provides full output.
**Cross-Domain Aliases:** bentley_ottmann, report_intersections, bo_intersection
**Notes:** Key insight: only neighbors in sweep order can intersect.

---

### CG.051: Line Segment Intersection (Binary Search)
**Definition:** Given n horizontal and m vertical segments, report all intersections.
**Cost Model:** O((n + m) log n + k) with range tree; O(nm) naive.
**Real Wall:** Reduces to orthogonal range reporting.
**Cross-Domain Aliases:** orthogonal_intersection, hv_intersection
**Notes:** Much faster than general segment intersection for axis-aligned segments.

---

### CG.052: Polygon Intersection
**Definition:** Compute intersection of two simple polygons.
**Cost Model:** O(n + m + k) for union area where k is output complexity.
**Real Wall:** Output is a set of polygons; handling holes adds complexity.
**Cross-Domain Aliases:** polygon_intersection, poly_intersect
**Notes:** Related to Boolean operations on polygons.

---

### CG.053: Line Arrangement
**Definition:** Compute the planar subdivision formed by n lines.
**Cost Model:** O(n²) cells; O(n²) construction; O(n²) space.
**Real Wall:** Cells can have O(n) vertices in worst case.
**Cross-Domain Aliases:** line_arrangement, arrangement_of_lines
**Notes:** Every arrangement has O(n²) cells; many have Θ(n²).

---

### CG.054: Zone Theorem
**Definition:** Zone of a line = sum of cell complexities crossing it = O(n).
**Cost Model:** Zone can be computed incrementally in O(n²) total.
**Real Wall:** Each insertion adds O(n) complexity to zone.
**Cross-Domain Aliases:** zone_theorem, zone_computation
**Notes:** Basis for incremental arrangement algorithms.

---

### CG.055: Incremental Arrangement Construction
**Definition:** Insert lines one by one, updating cells and edges.
**Cost Model:** O(n²) total by Zone theorem; O(n² log n) for lines.
**Real Wall:** Each insertion takes O(n) amortized.
**Cross-Domain Aliases:** incremental_arrangement, add_line_arrangement
**Notes:** Simpler than divide-and-conquer; same complexity.

---

### CG.056: Arrangement of Curves (Bezier, Circular Arcs)
**Definition:** Compute arrangement of non-linear curves with defined intersection patterns.
**Cost Model:** O((n + k) log n) for curves with O(1) pairwise intersections.
**Real Wall:** Curve-curve intersection dominates for high-degree curves.
**Cross-Domain Aliases:** curve_arrangement, nonlinear_arrangement
**Notes:** Bentley-Ottmann generalizes to curves with bounded intersections.

---

### CG.057: Visibility Graph (Point Set)
**Definition:** Graph where vertices are points; edges connect visible pairs.
**Cost Model:** O(n² log n) worst-case; O(n²) average for random point sets.
**Real Wall:** Used for shortest path with obstacles.
**Cross-Domain Aliases:** visibility_graph, point_visibility_graph
**Notes:** Edges are unobstructed line-of-sight segments.

---

### CG.058: Visibility Graph (Polygon)
**Definition:** Graph of polygon vertices where edges are visible within polygon interior.
**Cost Model:** O(n²) worst-case; O(n log n + E) using angular sweep.
**Real Wall:** Used for shortest path in polygon with obstacles.
**Cross-Domain Aliases:** polygon_visibility_graph, vertex_visibility
**Notes:** Can be extracted from angularly sorted edge events.

---

### CG.059: Shortest Path in Polygon (Visibility Graph)
**Definition:** Compute shortest path between two points avoiding polygon obstacles.
**Cost Model:** O(n² log n) to build visibility graph + Dijkstra.
**Real Wall:** Path follows polygon edges or visibility graph edges.
**Cross-Domain Aliases:** visible_shortest_path, sp_in_polygon
**Notes:** Requires polygon vertices + source/dest as graph vertices.

---

### CG.060: Art Gallery Problem (Guard Placement)
**Definition:** Place minimum number of guards to see entire polygon interior.
**Cost Model:** NP-hard for visibility guards; O(n³) for vertex guards.
**Real Wall:** Art gallery theorem: ⌊n/3⌋ guards always sufficient and necessary.
**Cross-Domain Aliases:** art_gallery, guard_cover, visibility_cover
**Notes:** Approximation algorithms needed for practical sizes.

---

### CG.061: Point Set Visibility
**Definition:** Compute visibility region from a point within polygon/obstacle set.
**Cost Model:** O(n log n) by angular sweep; O(n + k) reporting.
**Real Wall:** Visibility region is star-shaped polygon.
**Cross-Domain Aliases:** visibility_region, point_visibility_region
**Notes:** Basis for robot motion planning.

---

### CG.062: Ray Shooting in Polygon
**Definition:** Given point and direction, find first intersection with polygon boundary.
**Cost Model:** O(log n) with trapezoidal map; O(n) naive.
**Real Wall:** Key operation in many visibility computations.
**Cross-Domain Aliases:** ray_shoot, visibility_ray
**Notes:** Related to point location with a direction parameter.

---

### CG.063: Shortest Path with Obstacles (Continuous Dijkstra)
**Definition:** Compute Euclidean shortest path avoiding polygonal obstacles.
**Cost Model:** O(n² log n) via visibility graph or continuous Dijkstra.
**Real Wall:** Continuous Dijkstra maintains wavefront of distances.
**Cross-Domain Aliases:** sp_no_obstacles, euclidean_spp, wavefront_spp
**Notes:** Path is polygonal with vertices at obstacle vertices.

---

### CG.064: Link Distance
**Definition:** Minimum number of straight segments connecting two points avoiding obstacles.
**Cost Model:** O(n²) via visibility graph; each edge is one link.
**Real Wall:** Equivalent to shortest path in visibility graph with unit edge weights.
**Cross-Domain Aliases:** link_distance, discrete_metric, visibility_link
**Notes:** Number of turns in shortest piecewise-linear path.

---

### CG.065: Shortest Watchman Route
**Definition:** Find shortest route that sees entire polygon from inside.
**Cost Model:** NP-hard; O(n⁴ log n) for simple polygon.
**Real Wall:** Related to art gallery but with route optimization.
**Cross-Domain Aliases:** watchman_route, shortest_visibility_route
**Notes:** Harder than guard placement; approximation is difficult.

---

### CG.066: 2D Euclidean Shortest Path (Weighted)
**Definition:** Shortest path in weighted region where speed varies by region.
**Cost Model:** Continuous Dijkstra with priority queue; O(n² log n).
**Real Wall:** Fastest path minimizes weighted distance, not geometric length.
**Cross-Domain Aliases:** weighted_spp, anisotropic_path, terrain_path
**Notes:** Applied to terrain navigation with varying costs.

---

### CG.067: Visibility from Moving Point
**Definition:** Compute portion of polygon visible at any position along a line segment.
**Cost Model:** O(n log n) via angular sweep.
**Real Wall:** Visibility polygon changes only at critical events.
**Cross-Domain Aliases:** moving_visibility, sweep_visibility
**Notes:** Relevant for patrolling and surveillance planning.

---

### CG.068: Kernel of Polygon
**Definition:** Set of points from which entire polygon is visible.
**Cost Model:** O(n) by intersecting half-planes from reflex vertices.
**Real Wall:** Empty if polygon is not star-shaped.
**Cross-Domain Aliases:** kernel, star_center, kernel_polygon
**Notes:** Kernel is convex intersection of half-planes.

---

### CG.069: Pairwise Distance (Closest Pair)
**Definition:** Find two points with minimum Euclidean distance in set of n.
**Cost Model:** O(n log n) via divide-and-conquer or kd-tree.
**Real Wall:** Naive O(n²) is prohibitive for large n.
**Cross-Domain Aliases:** closest_pair, min_pair_distance
**Notes:** Basis for clustering and nearest neighbor heuristics.

---

### CG.070: All Nearest Neighbors (2D)
**Definition:** For each point, find its nearest neighbor among all points.
**Cost Model:** O(n log n) via Delaunay triangulation; each edge is candidate.
**Real Wall:** Delaunay graph contains nearest neighbor edge for each point.
**Cross-Domain Aliases:** all_nn, nearest_neighbor_graph, nng
**Notes:** Nearest neighbor graph has O(n) edges.

---

### CG.071: k-Nearest Neighbors (2D)
**Definition:** For each point, find k closest points in the set.
**Cost Model:** O(k n log n) via kd-tree with priority queue.
**Real Wall:** Spatial queries are amortized O(log n + k).
**Cross-Domain Aliases:** knn, k_nearest, range_k_closest
**Notes:** Priority queue collects candidates during kd-tree traversal.

---

### CG.072: Fixed-Radius Near Neighbor
**Definition:** Report all points within distance r of query point.
**Cost Model:** O(log n + k) with range tree on squared distance.
**Real Wall:** r must be fixed at query time for this complexity.
**Cross-Domain Aliases:** radius_search, ball_query, fixed_radius_nn
**Notes:** Equivalent to ball range query.

---

### CG.073: Euclidean Minimum Spanning Tree (2D)
**Definition:** Minimum weight spanning tree connecting all points.
**Cost Model:** O(n log n) via Delaunay graph + MST; O(n²) naive Prim.
**Real Wall:** Delaunay MST is subgraph of Delaunay triangulation.
**Cross-Domain Aliases:** emst, euclidean_mst, planar_mst
**Notes:** EMST is subgraph of relative neighborhood graph and Gabriel graph.

---

### CG.074: Fixed-Radius Spanning Graph
**Definition:** Connect all points within distance r; report number of components.
**Cost Model:** O(n log n) using kd-tree for neighbor enumeration.
**Real Wall:** Simple structure for percolation and connectivity analysis.
**Cross-Domain Aliases:** r-spanner, radius_spanning_graph
**Notes:** r must be large enough to ensure single component.

---

### CG.075: Gabriel Graph
**Definition:** Edge uv exists iff no other point lies in circle with diameter uv.
**Cost Model:** O(n²) check all pairs; O(n log n) from Delaunay.
**Real Wall:** Subset of Delaunay; superset of MST.
**Cross-Domain Aliases:** gabriel_graph, empty_circle_graph
**Notes:** Geometric criterion for clustering and physics applications.

---

### CG.076: Relative Neighborhood Graph
**Definition:** Edge uv exists iff no other point lies in lens region of u and v.
**Cost Model:** O(n² log n) direct; O(n log n) from MST.
**Real Wall:** Locus of points closer to u than v and to v than u.
**Cross-Domain Aliases:** rng, relative_neighborhood_graph, lens_graph
**Notes:** Subset of Gabriel graph; defines proximity graph hierarchy.

---

### CG.077: Yao Graph
**Definition:** For each point, partition plane into k cones; connect to nearest in each cone.
**Cost Model:** O(n²) direct; O(n log n) using spatial sorting per cone.
**Real Wall:** Yao₁ is superset of minimum spanning tree.
**Cross-Domain Aliases:** yao_graph, cone_graph, angular_mst
**Notes:** Parameter k controls trade-off between edges and optimality.

---

### CG.078: Theta Graph
**Definition:** Yao graph variant using projected distance rather than Euclidean within cones.
**Cost Model:** O(n log n) for fixed k; O(nk log n) for construction.
**Real Wall:** Better bounded-degree than Yao graph; spanner property.
**Cross-Domain Aliases:** theta_graph, projected_yao, cone_spanner
**Notes:** Used in wireless network routing (geometric spanners).

---

### CG.079: WSPD (Well-Separated Pair Decomposition)
**Definition:** Decompose point set into pairs of well-separated clusters.
**Cost Model:** O(n) pairs; O(n log n) construction via fair-split tree.
**Real Wall:** Basis for spanner construction and geometric optimization.
**Cross-Domain Aliases:** wspd, well_separated_pair
**Notes:** Separation ratio s controls approximation quality.

---

### CG.080: Geometric Spanner (TSPN)
**Definition:** Subgraph of complete graph with bounded stretch factor and bounded degree.
**Cost Model:** O(n log n) to O(n²) depending on construction method.
**Real Wall:** Stretch factor t: shortest path ≤ t × Euclidean distance.
**Cross-Domain Aliases:** geometric_spanner, bounded_stretch_graph
**Notes:** Stretch factor 1+ε achievable with O(n/ε) edges.

---

### CG.081: Minimum Weight Triangulation
**Definition:** Triangulation minimizing sum of edge weights.
**Cost Model:** NP-hard; O(n²) dynamic programming for polygon.
**Real Wall:** General point set remains open problem.
**Cross-Domain Aliases:** mwt, optimal_triangulation
**Notes:** For convex polygon: O(n³) DP; O(n²) improved.

---

### CG.082: Steiner Tree (Planar)
**Definition:** Minimum tree connecting points allowing extra Steiner vertices.
**Cost Model:** NP-hard; O(n²) DP for points on convex hull.
**Real Wall:** Steiner points reduce total length by up to ~13.4%.
**Cross-Domain Aliases:** steiner_tree, euclidean_steiner
**Notes:** Exact algorithm uses Delaunay triangulation as candidate graph.

---

### CG.083: Euclidean Traveling Salesman (Exact)
**Definition:** Minimum Hamiltonian cycle visiting all points.
**Cost Model:** O(n² 2^n) via dynamic programming (Held-Karp).
**Real Wall:** Only practical for n < 20-25.
**Cross-Domain Aliases:** etsp, exact_tsp, planar_tsp_dp
**Notes:** Exponential in n; no polynomial-time exact algorithm known.

---

### CG.084: Euclidean Traveling Salesman (Approximation)
**Definition:** Approximate minimum tour within (3/2)-factor.
**Cost Model:** O(n²) for double-MST; O(n log n) for Lin-Kernighan.
**Real Wall:** Christofides algorithm achieves 1.5-approximation.
**Cross-Domain Aliases:** etsp_approx, tsp_approximate, heuristic_tsp
**Notes:** Double-MST gives 2-approximation; simpler than Christofides.

---

### CG.085: Minimum Bottleneck Hamiltonian Path
**Definition:** Path minimizing maximum edge weight (not total weight).
**Cost Model:** O(n log n) by sorting edges and testing connectivity.
**Real Wall:** Related to bottleneck traveling salesman problem.
**Cross-Domain Aliases:** bottleneck_path, mbtsp, minimax_hamiltonian
**Notes:** Approximation via MST ordering; optimal in O(n log n).

---

### CG.086: Planar Facility Location (k-Center)
**Definition:** Place k centers minimizing maximum distance to any client.
**Cost Model:** 2-approximation via greedy + doubling; O(n log n).
**Real Wall:** NP-hard; 2 is best possible polynomial factor.
**Cross-Domain Aliases:** kcenter, minimax_facility, planar_kcenter
**Notes:** Greedy farthest-first insertion gives 2-approximation.

---

### CG.087: k-Means Facility Location (Lloyd's)
**Definition:** Lloyd's algorithm for k-means clustering.
**Cost Model:** O(nki) per iteration; converges in O(nk) per iteration.
**Real Wall:** Converges to local optimum; sensitive to initialization.
**Cross-Domain Aliases:** kmeans, lloyd_algorithm, kmeans_clustering
**Notes:** K-means++ initialization improves convergence significantly.

---

### CG.088: Minimum Enclosing Triangle
**Definition:** Smallest-area triangle containing convex polygon.
**Cost Model:** O(n²) by enumerating hull edges and optimizing third vertex.
**Real Wall:** One edge must lie on hull; third vertex is support function.
**Cross-Domain Aliases:** met, minimum_area_triangle
**Notes:** Related to minimum enclosing parallelogram.

---

### CG.089: Minimum Perimeter Triangle
**Definition:** Smallest-perimeter triangle containing convex polygon.
**Cost Model:** O(n²) via rotating calipers on antipodal pairs.
**Real Wall:** Perimeter and area minima are different problems.
**Cross-Domain Aliases:** mpt, minimum_perimeter_triangle
**Notes:** Optimal triangle is always determined by hull edges.

---

### CG.090: Smallest Enclosing Triangle (Point Set)
**Definition:** Smallest-area triangle enclosing all points (not necessarily hull vertices).
**Cost Model:** O(n³) via rotating calipers for all edge orientations.
**Real Wall:** Triangle vertices need not be input points.
**Cross-Domain Aliases:** smallest_enclosing_triangle, point_enclosing_tri
**Notes:** Related to Welzl's algorithm for minimum enclosing circle.

---

### CG.091: Diameter of Point Set
**Definition:** Maximum distance between any two points in set.
**Cost Model:** O(n) after O(n log n) convex hull (rotating calipers).
**Real Wall:** Farthest-point pair always lies on convex hull.
**Cross-Domain Aliases:** diameter, max_pair_distance, point_diameter
**Notes:** Related to minimum width (minimum caliper width).

---

### CG.092: Antipodal Pairs
**Definition:** Find all pairs of hull points where tangent lines are parallel.
**Cost Model:** O(n) via rotating calipers on convex hull.
**Real Wall:** Each hull edge has exactly two antipodal points.
**Cross-Domain Aliases:** antipodal, antipodal_pairs
**Notes:** Foundation for many rotating calipers applications.

---

### CG.093: Minimum Width of Point Set
**Definition:** Minimum distance between parallel supporting lines.
**Cost Model:** O(n) via rotating calipers on convex hull.
**Real Wall:** Complement to maximum width (diameter).
**Cross-Domain Aliases:** minwidth, minimum_width_calipers
**Notes:** Related to width computation but minimizes rather than maximizes.

---

### CG.094: Area of Simple Polygon
**Definition:** Compute signed area using shoelace formula: ½ Σ(xᵢyᵢ₊₁ - xᵢ₊₁yᵢ).
**Cost Model:** O(n) linear time.
**Real Wall:** Positive for CCW, negative for CW orientation.
**Cross-Domain Aliases:** polygon_area, shoelace, signed_area
**Notes:** Fundamental polygon computation.

---

### CG.095: Centroid of Simple Polygon
**Definition:** Compute centroid (center of mass) of polygon area.
**Cost Model:** O(n) by integrating vertex contributions.
**Real Wall:** Weighted by area; centroid of boundary alone differs.
**Cross-Domain Aliases:** polygon_centroid, area_centroid
**Notes:** Useful for physics simulation and geographic analysis.

---

### CG.096: Perimeter of Simple Polygon
**Cost Model:** O(n) sum of edge lengths.
**Real Wall:** Simple formula: Σ √((xᵢ₊₁ - xᵢ)² + (yᵢ₊₁ - yᵢ)²).
**Cross-Domain Aliases:** polygon_perimeter, boundary_length
**Notes:** Often needed for shape descriptors.

---

### CG.097: Second Moment of Area
**Definition:** Compute Iₓ, Iᵧ, Iₓᵧ for polygon area moment of inertia.
**Cost Model:** O(n) via polygon moment formulas.
**Real Wall:** Key for structural engineering and physics.
**Cross-Domain Aliases:** moment_of_inertia, area_moment, polygon_moments
**Notes:** Principal moments computed from eigenvalue decomposition.

---

### CG.098: Polygon Smoothing (Laplacian)
**Definition:** Move each vertex toward average of neighbors; repeat.
**Cost Model:** O(n) per iteration.
**Real Wall:** Shrinks polygon; needs area normalization.
**Cross-Domain Aliases:** laplacian_smooth, polygon_flow, mean_curvature_flow
**Notes:** Related to curve shortening flow in differential geometry.

---

### CG.099: Constrained Delaunay Triangulation (CDT)
**Definition:** Triangulation respecting input edges as constraints; Delaunay elsewhere.
**Cost Model:** O(n log n + k) where k is constraint count.
**Real Wall:** Essential for meshing domains with internal boundaries.
**Cross-Domain Aliases:** cdt, constrained_delaunay, domain_meshing
**Notes:** Basis for quality mesh generation for FEM.

---

### CG.100: Ruppert's Delaunay Refinement
**Definition:** Refine mesh until all angles ≥ θ (default 20°) by inserting points.
**Cost Model:** O(n log n) expected; converges in practice.
**Real Wall:** θ=20° guarantees good quality; larger θ may fail.
**Cross-Domain Aliases:** ruppert_refine, angle_quality_mesh
**Notes:** Standard algorithm for guaranteed-quality triangular meshes.

---

### CG.101: Chew's Second Delaunay Refinement
**Definition:** Refine mesh ensuring no small input segments and angle bound.
**Cost Model:** O(n log n) expected; handles input segments better.
**Real Wall:** Better than Ruppert's for segments and boundaries.
**Cross-Domain Aliases:** chew_refine, segment_protected_refine
**Notes:** Preferred when input geometry contains small features.

---

### CG.102: Discrete Gaussian Curvature
**Definition:** Approximate curvature at polygon vertex via angular deficit.
**Cost Model:** O(n) for all vertices.
**Real Wall:** K = 2π - Σθᵢ for vertex with internal angles θᵢ.
**Cross-Domain Aliases:** gaussian_curvature, angular_defect, discrete_curvature
**Notes:** Integral of curvature relates to topology (Gauss-Bonnet).

---

### CG.103: Mean Curvature at Vertex
**Cost Model:** O(1) per vertex given neighbors.
**Real Wall:** Computed from dihedral angles in 3D or turning angle in 2D.
**Cross-Domain Aliases:** mean_curvature, vertex_curvature
**Notes:** Flow in direction of mean curvature smooths surfaces.

---

### CG.104: Isosurface Extraction (Marching Cubes)
**Definition:** Extract surface where 3D scalar field equals threshold value.
**Cost Model:** O(n) per grid cell; O(n^(2/3)) surface cells.
**Real Wall:** Ambiguity resolution affects topology; Marching Tetrahedra fixes some cases.
**Cross-Domain Aliases:** marching_cubes, iso_surface, level_set_extract
**Notes:** Standard for medical imaging and scientific visualization.

---

### CG.105: Marching Tetrahedra
**Definition:** Variant of marching cubes using tetrahedral grid decomposition.
**Cost Model:** O(n) per tetrahedron; more consistent topology handling.
**Real Wall:** Avoids ambiguous cases in marching cubes.
**Cross-Domain Aliases:** marching_tets, tet_isosurface
**Notes:** Slightly slower but more robust topology.

---

### CG.106: Euclidean Distance Transform (2D)
**Definition:** For each pixel, compute distance to nearest feature pixel.
**Cost Model:** O(n) with two-pass scan; O(n log n) with kd-tree.
**Real Wall:** Chamfer distance (approx) vs exact Euclidean.
**Cross-Domain Aliases:** edt, distance_transform, raster_distance
**Notes:** 3D EDT is O(n) via two-pass 6SSEDT algorithm.

---

### CG.107: Signed Distance Field (Polygon)
**Definition:** Compute exact signed distance from sample points to polygon boundary.
**Cost Model:** O(n) per point via closest edge; O(n log n) with spatial index.
**Real Wall:** Negative inside, positive outside, zero on boundary.
**Cross-Domain Aliases:** sdf, polygon_sdf, exact_distance_field
**Notes:** Useful for rendering, collision detection, offsetting.

---

### CG.108: Offset Polygon (Inward)
**Definition:** Shrink polygon by distance d; clip edges and add arcs at reflex vertices.
**Cost Model:** O(n) per offset step with proper data structure.
**Real Wall:** Complexity increases when offset causes self-intersection.
**Cross-Domain Aliases:** inward_offset, shrink_polygon, inset_polygon
**Notes:** Terminates when offset becomes empty or single point.

---

### CG.109: Offset Polygon (Outward)
**Definition:** Expand polygon by distance d; offset edges and add arcs at convex vertices.
**Cost Model:** O(n) per offset step; handle self-intersections if needed.
**Real Wall:** Naive offset produces self-intersections for large d.
**Cross-Domain Aliases:** outward_offset, expand_polygon, buffer_polygon
**Notes:** Boolean union of offset edges; often needs cleanup.

---

### CG.110: Minkowski Sum (Polygon)
**Definition:** Sum: {p + q | p ∈ P, q ∈ Q} for two polygons.
**Cost Model:** O((n + m) log(n + m)) via convolution of edge directions.
**Real Wall:** Convolution of edge angles gives complexity profile.
**Cross-Domain Aliases:** minkowski_sum, polygon_sum
**Notes:** Key operation in collision detection and robot motion.

---

### CG.111: Minkowski Difference
**Definition:** Minkowski sum of P and reflected Q: {p - q | p ∈ P, q ∈ Q}.
**Cost Model:** O((n + m) log(n + m)) by reflecting one polygon.
**Real Wall:** Collision-free iff origin lies in Minkowski difference.
**Cross-Domain Aliases:** minkowski_diff, collision_test
**Notes:** Foundation of configuration space obstacle method.

---

### CG.112: Convolution of Polygons
**Definition:** Compute convolution of two polygonal curves via angle sweep.
**Cost Model:** O((n + m) log(n + m)) via rotating calipers.
**Real Wall:** Convolution trace gives Minkowski sum boundary.
**Cross-Domain Aliases:** polygon_convolution, edge_convolution
**Notes:** Related to moving one polygon along another.

---

### CG.113: Visibility Kernel
**Definition:** Compute set of points from which entire polygon is visible.
**Cost Model:** O(n) by intersecting half-planes at reflex vertices.
**Real Wall:** Empty if polygon not star-shaped.
**Cross-Domain Aliases:** visibility_kernel, star_polygon_center
**Notes:** Kernel of star-shaped polygon is non-empty convex polygon.

---

### CG.114: Point-in-Polygon (Spatial Grid Acceleration)
**Definition:** Preprocess polygon into uniform grid for faster PIP queries.
**Cost Model:** O(n + g) build; O(1) average query (with grid cache).
**Real Wall:** Grid cell precomputation stores inside/outside/boundary status.
**Cross-Domain Aliases:** pip_grid, accelerated_point_in_polygon
**Notes:** Grid size balances query speed vs memory vs preprocessing.

---

### CG.115: Homogeneous Coordinates (2D Projective)
**Definition:** Represent point (x, y) as (x, y, 1) for projective transformations.
**Cost Model:** O(1) per operation; 3×3 matrix multiplication.
**Real Wall:** Lines also as (a, b, c); ax + by + c = 0 in affine.
**Cross-Domain Aliases:** homogeneous_2d, projective_point, p2_point
**Notes:** Enables unified treatment of points and points at infinity.

---

### CG.116: Projective Line Intersection
**Definition:** Compute intersection of two lines in projective plane.
**Cost Model:** O(1) via cross product of homogeneous coordinates.
**Real Wall:** Parallel lines intersect at point at infinity.
**Cross-Domain Aliases:** line_intersect_proj, projective_intersection
**Notes:** Cross product of (a₁,b₁,c₁) × (a₂,b₂,c₂) gives intersection.

---

### CG.117: Cross Ratio
**Definition:** Invariant of four collinear points under projective transformation.
**Cost Model:** O(1) to compute from coordinates.
**Real Wall:** (A,B;C,D) = (AC/BC) / (AD/BD) in signed distances.
**Cross-Domain Aliases:** cross_ratio, projective_invariant
**Notes:** Preserved under perspective projection; used in camera calibration.

---

### CG.118: Projective Conic through 5 Points
**Definition:** Unique conic passing through 5 points in general position.
**Cost Model:** O(1) to solve 5×5 linear system for conic coefficients.
**Real Wall:** Degenerate if points are collinear or conic is undefined.
**Cross-Domain Aliases:** five_point_conic, conic_from_points
**Notes:** Fundamental in multiple-view geometry.

---

### CG.119: Dual of Conic
**Definition:** Transform conic into line dual; useful for tangent computations.
**Cost Model:** O(1) for 3×3 matrix inverse in homogeneous coordinates.
**Real Wall:** Point conic C*: points x satisfy xᵀCx = 0.
**Cross-Domain Aliases:** dual_conic, conic_dual, tangent_dual
**Notes:** Line ℓ is tangent to conic C iff ℓᵀC*ℓ = 0.

---

### CG.120: Homography (2D Projective Transform)
**Definition:** 3×3 matrix mapping projective plane to itself; 8 DOF (9 parameters, scale invariant).
**Cost Model:** O(1) to apply; O(n) to estimate from point correspondences.
**Real Wall:** 4 point correspondences determine homography (DLT algorithm).
**Cross-Domain Aliases:** homography_2d, projective_transform, h_matrix
**Notes:** Models camera motion, planar object tracking, image stitching.

---

### CG.121: Affine Transformation (2D)
**Definition:** 2×3 matrix: [a b | c d | e f]; 6 DOF; preserves parallelism.
**Cost Model:** O(1) per point; O(n) to estimate from point pairs.
**Real Wall:** 3 non-collinear point correspondences determine affine transform.
**Cross-Domain Aliases:** affine_2d, affine_transform, linear_plus_translation
**Notes:** Subset of projective; preserves ratios of areas on parallel lines.

---

### CG.122: Similarity Transformation (2D)
**Definition:** Rotation + uniform scale + translation; 4 DOF; preserves angles.
**Cost Model:** O(1) per point; O(1) to estimate from point pairs.
**Real Wall:** 2 point correspondences determine similarity (up to reflection).
**Cross-Domain Aliases:** similarity_2d, similarity_transform, rigid_scale
**Notes:** Preserves shape; used in object recognition up to scale/rotation.

---

### CG.123: Rigid Transformation (2D)
**Definition:** Rotation + translation; 3 DOF; preserves distances and angles.
**Cost Model:** O(1) per point; O(n log n) to align point sets (ICP).
**Real Wall:** 2 point correspondences determine rigid transform.
**Cross-Domain Aliases:** rigid_2d, euclidean_transform, se2
**Notes:** Special case of similarity with scale = 1.

---

### CG.124: Similarity Registration (Point Clouds)
**Definition:** Find rigid+scale transformation minimizing RMSD between point sets.
**Cost Model:** O(n log n) for sorting by angle; O(n) for Procrustes.
**Real Wall:** Umeyama algorithm gives closed-form solution.
**Cross-Domain Aliases:** similarity_register, procrustes, point_cloud_align
**Notes:** Basis for shape matching and model registration.

---

### CG.125: Iterative Closest Point (ICP)
**Definition:** Alternately find correspondences (nearest neighbor) and estimate rigid transform.
**Cost Model:** O(n log n) per iteration for kd-tree search.
**Real Wall:** Converges monotonically but may get stuck in local minimum.
**Cross-Domain Aliases:** icp, iterative_closest_point, point_cloud_registration
**Notes:** Standard for 3D scan alignment; many variants improve speed/robustness.

---

### CG.126: Point-to-Line Distance (Signed)
**Definition:** Signed perpendicular distance from point to oriented line.
**Cost Model:** O(1) via cross product: (b-a) × (p-a) / |b-a|.
**Real Wall:** Sign indicates which side of line; zero on line.
**Cross-Domain Aliases:** signed_line_dist, point_side_test, oriented_dist
**Notes:** Key primitive for polygon winding and point-in-polygon tests.

---

### CG.127: Point-to-Segment Distance
**Definition:** Minimum distance from point to line segment.
**Cost Model:** O(1) by projecting and clamping to segment endpoints.
**Real Wall:** Returns 0 if perpendicular projection lies within segment.
**Cross-Domain Aliases:** point_seg_dist, segment_distance
**Notes:** Basis for distance field computation.

---

### CG.128: Point-to-Triangle Distance
**Definition:** Minimum distance from point to triangle (interior or edge).
**Cost Model:** O(1) via barycentric projection + edge clamping.
**Real Wall:** Project point onto triangle plane; clamp to edges if outside.
**Cross-Domain Aliases:** point_triangle_dist, triangle_distance
**Notes:** Key in collision detection and mesh distance queries.

---

### CG.129: Point-to-Polygon Distance (2D)
**Definition:** Minimum distance from point to any edge of polygon.
**Cost Model:** O(n) by checking all edges; O(log n) with spatial index.
**Real Wall:** Handles holes by checking inner boundaries too.
**Cross-Domain Aliases:** point_polygon_dist, boundary_distance
**Notes:** Interior distance requires distance transform.

---

### CG.130: Line Segment Clipping (Cohen-Sutherland)
**Definition:** Clip line segment against rectangular window using outcode.
**Cost Model:** O(1) amortized; at most 4 iterations.
**Real Wall:** Outcode encodes which edges are exceeded.
**Cross-Domain Aliases:** cohen_sutherland, rect_clip, outcode_clip
**Notes:** Basis for graphics rasterization pipeline.

---

### CG.131: Line Segment Clipping (Liang-Barsky)
**Cost Model:** O(1) parametric; more efficient than Cohen-Sutherland.
**Real Wall:** Uses parametric inequality: p + td for t ∈ [0,1].
**Cross-Domain Aliases:** liang_barsky, param_clip, linear_clip
**Notes:** Computes entry/exit parameters directly.

---

### CG.132: Polygon Clipping (Sutherland-Hodgman)
**Definition:** Clip polygon against convex window by iterating edge-by-edge.
**Cost Model:** O(nm) for n-sided polygon clipped to m-sided window.
**Real Wall:** Output may have more vertices than input.
**Cross-Domain Aliases:** sutherland_hodgman, poly_clip, convex_clip
**Notes:** Sequential edge clipping handles any convex clip region.

---

### CG.133: Weiler-Atherton Polygon Clipping
**Definition:** General polygon-polygon clipping via entering/exiting tracking.
**Cost Model:** O(n + m + k) where k is output complexity.
**Real Wall:** Handles concave clips and arbitrary polygon shapes.
**Cross-Domain Aliases:** weiler_atherton, general_poly_clip
**Notes:** More complex but handles non-convex clip regions.

---

### CG.134: Greiner-Hormann Polygon Clipping
**Definition:** Clip arbitrary polygons tracking entry/exit at intersection points.
**Cost Model:** O((n + m) log(n + m) + k) with sorting; O(n + m + k) unsorted.
**Real Wall:** Handles holes and multiple components.
**Cross-Domain Aliases:** greiner_hormann, arbitrary_poly_clip
**Notes:** Simpler than Weiler-Atherton; handles all degenerate cases.

---

### CG.135: Boolean Operations on Polygons
**Definition:** Compute union, intersection, difference of polygon regions.
**Cost Model:** O((n + m) log(n + m) + k) via sweep line or map overlay.
**Real Wall:** Vatti polygon clipper handles general polygons robustly.
**Cross-Domain Aliases:** bool_ops, polygon_boolean, poly_union
**Notes:** Foundation for CAD kernel and GIS overlay operations.

---

### CG.136: Map Overlay
**Definition:** Compute geometric intersection of two planar subdivisions.
**Cost Model:** O((n + m) log(n + m) + k) via plane sweep.
**Real Wall:** Result is a planar map with O(n + m + k) cells.
**Cross-Domain Aliases:** overlay, planar_overlay, subdivision_intersect
**Notes:** Key operation in GIS for combining spatial layers.

---

### CG.137: 2D Convex Hull (Incremental, Randomized)
**Definition:** Randomly permute points; insert incrementally maintaining hull.
**Cost Model:** O(n) expected time; O(n²) worst-case.
**Real Wall:** Simple to implement; good expected performance.
**Cross-Domain Aliases:** randomized_hull, incr_convex_hull
**Notes:** Basis for many randomized computational geometry algorithms.

---

### CG.138: kd-Tree Construction (2D)
**Definition:** Recursively split by median x/y alternately; stop at leaf size b.
**Cost Model:** O(n log n) build; O(log n) query for point, O(√n + k) range.
**Real Wall:** Balance trade-off: small b → deep tree, large b → fewer splits.
**Cross-Domain Aliases:** kdtree_build, balanced_kdtree, 2d_kdtree
**Notes:** Axis alternation gives approximately balanced tree.

---

### CG.139: Balanced kd-Tree (Split by Longest Dimension)
**Definition:** kd-tree variant splitting along dimension with largest extent.
**Cost Model:** O(n log n) to find medians; O(n log n) build.
**Real Wall:** Better adapted to anisotropic point distributions.
**Cross-Domain Aliases:** longdim_kdtree, adaptive_kdtree
**Notes:** Standard variant in many implementations.

---

### CG.140: Approximate Nearest Neighbor (kd-Tree)
**Definition:** kd-tree search with early termination when candidate distance < ε · best.
**Cost Model:** O(log n) average; O(n) worst-case.
**Real Wall:** Approximation factor (1+ε); faster than exact search.
**Cross-Domain Aliases:** ann_kdtree, epsilon_nn, approximate_knn
**Notes:** Used in high-dimensional ANN (but kd-tree degrades in high d).

---

### CG.141: Best Bin First (BBF) Search
**Definition:** kd-tree search prioritized by distance to splitting plane.
**Cost Model:** O(log n) average; prunes unlikely branches first.
**Real Wall:** Better than standard kd-tree traversal for ANN.
**Cross-Domain Aliases:** bbf_search, priority_kdtree
**Notes:** Used in locality-sensitive hashing comparisons.

---

### CG.142: Ball Tree
**Definition:** Hierarchical ball (hyper-sphere) covering of points.
**Cost Model:** O(n log n) build; O(log n) query for nearest neighbor.
**Real Wall:** Better in high dimensions than kd-tree.
**Cross-Domain Aliases:** balltree, hypersphere_tree
**Notes:** Query uses triangle inequality to prune subtrees.

---

### CG.143: Cover Tree
**Definition:** Hierarchical covering with separation and covering constants.
**Cost Model:** O(n log n) build; O(log n) query for nearest neighbor.
**Real Wall:** Theoretical bounds on query and construction.
**Cross-Domain Aliases:** covertree, cover_tree
**Notes:** Adapts to intrinsic dimensionality of data.

---

### CG.144: VP-Tree (Vantage Point Tree)
**Definition:** Recursive partition by distance from chosen vantage point.
**Cost Model:** O(n log n) build; O(log n) average query.
**Real Wall:** Works well with distance-based pruning.
**Cross-Domain Aliases:** vptree, vantage_point_tree
**Notes:** Triangle inequality enables effective pruning.

---

### CG.145: Geometric Median
**Definition:** Point minimizing sum of distances to all input points.
**Cost Model:** O(nd ε⁻²) via Weiszfeld's algorithm; not closed form.
**Real Wall:** Sensitive to outliers; use trimmed geometric median for robustness.
**Cross-Domain Aliases:** geometric_median, weber_point, fermat_point
**Notes:** Fermat point in L² is geometric median; differs for other norms.

---

### CG.146: Fermat Point (Weighted)
**Definition:** Point minimizing weighted sum of distances; generalizes geometric median.
**Cost Model:** O(n) iterative; O(n log n) for special case of triangle.
**Real Wall:** Weights allow emphasizing certain points.
**Cross-Domain Aliases:** weighted_fermat, weighted_median_point
**Notes:** Related to facility location: weighted k-median problem.

---

### CG.147: Hausdorff Distance (Point Sets)
**Definition:** Maximum distance from any point in A to nearest point in B.
**Cost Model:** O(nm) naive; O((n + m) log(n + m)) via kd-tree.
**Real Wall:** Directed: h(A,B) ≠ h(B,A) generally.
**Cross-Domain Aliases:** hausdorff_dist, directed_hausdorff
**Notes:** Complete metric on compact sets; used in shape matching.

---

### CG.148: Frechet Distance (Curves)
**Definition:** Minimum leash length to walk dog along one curve while owner walks other.
**Cost Model:** O(nm) via dynamic programming; O((n+m) log(n+m)) for curves on surfaces.
**Real Wall:** More sensitive than Hausdorff to curve parameterization.
**Cross-Domain Aliases:** frechet_dist, curve_distance
**Notes:** Can be computed with free space diagram approach.

---

### CG.149: Discrete Frechet Distance
**Cost Model:** O(nm) dynamic programming with memoization.
**Real Wall:** Approximates continuous Frechet distance.
**Cross-Domain Aliases:** discrete_frechet, polygonal_frechet
**Notes:** Easier to compute; used in trajectory analysis.

---

### CG.150: Earth Mover's Distance (2D Histograms)
**Definition:** Minimum cost to transform one distribution into another.
**Cost Model:** O(n³ log n) via Hungarian algorithm on cost matrix.
**Real Wall:** Equivalent to minimum cost flow on grid.
**Cross-Domain Aliases:** emd, wasserstein_dist, transport_distance
**Notes:** Works on weighted point sets; also called Wasserstein-1.

---

### CG.151: Turn Angle (Path)
**Definition:** Signed angle between consecutive path segments.
**Cost Model:** O(1) per vertex via dot/cross product.
**Real Wall:** CCW = positive, CW = negative; zero = straight.
**Cross-Domain Aliases:** turning_angle, exterior_angle, path_curvature_2d
**Notes:** Integral of curvature along closed curve = 2π (winding).

---

### CG.152: Interior Angle (Polygon Vertex)
**Definition:** Angle between edges entering/leaving vertex; inside polygon.
**Cost Model:** O(1) per vertex via vector dot product.
**Real Wall:** Sum of interior angles = (n-2)π for simple polygon.
**Cross-Domain Aliases:** internal_angle, polygon_vertex_angle
**Notes:** > π indicates reflex vertex.

---

### CG.153: Polygon Simplification (Douglas-Peucker)
**Definition:** Recursive approximation: keep endpoints; drop intermediate points if collinear within ε.
**Cost Model:** O(n²) worst-case; O(n) average for random walk.
**Real Wall:** Quality depends on ε; sensitive to noise.
**Cross-Domain Aliases:** douglas_peucker, poly_simplify, line_simplify
**Notes:** Classic algorithm; also called Ramer-Douglas-Peucker.

---

### CG.154: Visvalingam-Whyatt Simplification
**Definition:** Remove vertices with smallest area of virtual triangle; iterate with decreasing area.
**Cost Model:** O(n log n) with priority queue.
**Real Wall:** Better at preserving visual quality than Douglas-Peucker.
**Cross-Domain Aliases:** visvalingam, area_based_simplify
**Notes:** Area threshold adapts to local geometry complexity.

---

### CG.155: Chaikin's Corner Cutting
**Definition:** Smooth curve by repeatedly cutting each segment at 1/4 and 3/4.
**Cost Model:** O(n · 2ᵏ) after k iterations.
**Real Wall:** Produces C¹ curve converging to B-spline limit.
**Cross-Domain Aliases:** chaikin_smooth, corner_cutting, subdivision_curve
**Notes:** Linear interpolation of subdivision; other schemes produce C².

---

### CG.156: Catmull-Rom Spline (from Points)
**Definition:** Interpolating cubic spline passing through all control points.
**Cost Model:** O(n) to evaluate per segment; O(n) to compute tangents.
**Real Wall:** Centripetal variant avoids self-intersections.
**Cross-Domain Aliases:** catmull_rom, interpolating_spline
**Notes:** Basis for many animation and font curve systems.

---

### CG.157: B-Spline (Uniform)
**Definition:** Piecewise cubic polynomial with uniform knot spacing.
**Cost Model:** O(n) evaluation via de Boor algorithm.
**Real Wall:** C² continuous (except at knots).
**Cross-Domain Aliases:** b_spline, uniform_bspline
**Notes:** Basis function representation enables efficient evaluation.

---

### CG.158: Bezier Curve (Cubic)
**Definition:** Piecewise cubic polynomial defined by 4 control points.
**Cost Model:** O(1) via de Casteljau; O(n) for curve of n segments.
**Real Wall:** Convex hull property: curve lies inside control point hull.
**Cross-Domain Aliases:** cubic_bezier, bezier3
**Notes:** Standard in vector graphics (PostScript, SVG).

---

### CG.159: Bezier Curve Splitting
**Definition:** Subdivide cubic Bezier at parameter t into two Beziers.
**Cost Model:** O(1) via de Casteljau pyramid computation.
**Real Wall:** Each half has 4 control points.
**Cross-Domain Aliases:** bezier_split, subdivide_bezier, de_casteljau_split
**Notes:** Basis for adaptive Bezier rendering and curve approximation.

---

### CG.160: Bezier Distance to Point (Approximate)
**Definition:** Find closest point on Bezier to query point; solve ∂/∂t = 0.
**Cost Model:** O(log n) with Newton-Raphson + bounding; O(n) naive.
**Real Wall:** Root-finding on derivative (cubic) gives candidates.
**Cross-Domain Aliases:** bezier_closest_point, bezier_distance
**Notes:** Quintic polynomial root finding; subdivision yields robust solution.

---

## Section: Advanced Convex Hull

### CG.161: Chan's Algorithm
**Definition:** Output-sensitive planar convex hull combining Graham scan with gift wrapping.
**Cost Model:** O(n log h) where h is hull size; optimal in comparison model.
**Real Wall:** Requires guessing h via doubling; constant factor higher than Graham.
**Cross-Domain Aliases:** chan_hull, output_sensitive_hull, ultimate_planar_hull
**Notes:** Chan (1996); matches Kirkpatrick-Seidel ultimate convex hull bound.

---

### CG.162: Kirkpatrick-Seidel Ultimate Hull
**Definition:** Prune-and-search planar convex hull achieving O(n log h) without doubling.
**Cost Model:** O(n log h) deterministic via bridge-finding.
**Real Wall:** Complex recursion; mostly theoretical interest vs. Chan's.
**Cross-Domain Aliases:** ultimate_hull, ks_hull, marriage_before_conquest
**Notes:** Kirkpatrick & Seidel (1986); first output-sensitive planar hull.

---

### CG.163: Dynamic Convex Hull (Overmars-van Leeuwen)
**Definition:** Maintain planar convex hull under insertions and deletions of points.
**Cost Model:** O(log² n) per update, O(log n) per query.
**Real Wall:** Concatenable queue with semi-dynamic tournament tree; coding-heavy.
**Cross-Domain Aliases:** dynamic_hull, ovl_hull
**Notes:** Overmars & van Leeuwen (1981); Brodal & Jacob improved to O(log n).

---

### CG.164: Online Convex Hull
**Definition:** Convex hull maintained under point insertions only, with point-in-hull queries.
**Cost Model:** O(log n) amortized insertion, O(log n) query.
**Real Wall:** Balanced BST keyed by angle; no deletion support.
**Cross-Domain Aliases:** online_hull, semi_dynamic_hull, insertion_only_hull
**Notes:** Preparata's algorithm; foundation for many streaming geometric problems.

---

### CG.165: Kinetic Convex Hull
**Definition:** Maintain convex hull of points moving along known trajectories.
**Cost Model:** O(n^{2+ε}) total events; kinetic data structure with certificates.
**Real Wall:** Event queue over polynomial certificates; numerical robustness critical.
**Cross-Domain Aliases:** kinetic_hull, kds_hull, moving_hull
**Notes:** Basch, Guibas, Hershberger (1997); near-quadratic certificate complexity.

---

### CG.166: 3D Convex Hull (Quickhull)
**Definition:** Divide-and-conquer 3D convex hull selecting farthest points and recursing on facets.
**Cost Model:** O(n log n) expected, O(n²) worst case.
**Real Wall:** Coplanar/cocircular degeneracies require careful handling.
**Cross-Domain Aliases:** quickhull3d, ch3, hull3d
**Notes:** Barber, Dobkin, Huhdanpaa (1996); Qhull library implementation.

---

### CG.167: 3D Convex Hull (Randomized Incremental)
**Definition:** Insert points one at a time; update hull by removing visible facets.
**Cost Model:** O(n log n) expected via conflict graph.
**Real Wall:** O(n²) facets possible in d=3 for adversarial input.
**Cross-Domain Aliases:** ric_hull3d, incremental_3d_hull
**Notes:** Clarkson & Shor (1989); foundation for many randomized geometric algorithms.

---

### CG.168: Higher-Dimensional Convex Hull
**Definition:** Convex hull of n points in R^d via beneath-beyond or randomized incremental.
**Cost Model:** O(n^{⌊d/2⌋}) facets in worst case (upper bound theorem).
**Real Wall:** Combinatorial explosion above d=4 makes exact hull impractical.
**Cross-Domain Aliases:** dd_hull, polytope_hull
**Notes:** McMullen's upper bound theorem; CGAL d-dim hull.

---

### CG.169: Half-Space Intersection
**Definition:** Compute polytope as intersection of n half-spaces in R^d.
**Cost Model:** Dual to convex hull; O(n log n) in 2D, O(n^{⌊d/2⌋}) in d-dim.
**Real Wall:** Unbounded intersections need projective treatment.
**Cross-Domain Aliases:** halfspace_intersect, polytope_intersection, h_polytope
**Notes:** Dualize to convex hull via point-hyperplane duality.

---

### CG.170: Upper Envelope of Lines
**Definition:** Pointwise maximum of n lines forming a convex piecewise-linear function.
**Cost Model:** O(n log n) via convex hull of dual points.
**Real Wall:** Vertical lines need special handling in dual.
**Cross-Domain Aliases:** upper_envelope, max_of_lines, line_envelope
**Notes:** Equivalent to lower convex hull in dual space.

---

### CG.171: Lower Envelope of Segments
**Definition:** Pointwise minimum function of n line segments in plane.
**Cost Model:** O(n α(n)) complexity (Davenport-Schinzel sequence).
**Real Wall:** Combinatorial complexity not linear; α is inverse Ackermann.
**Cross-Domain Aliases:** lower_envelope, ds_sequence, segment_envelope
**Notes:** Hart & Sharir (1986); fundamental in motion planning.

---

### CG.172: Upper Envelope of Surfaces in 3D
**Definition:** Pointwise maximum of n bivariate functions producing 2D arrangement.
**Cost Model:** O(n^{2+ε}) for algebraic surfaces of bounded degree.
**Real Wall:** Near-quadratic complexity even for planes.
**Cross-Domain Aliases:** envelope_3d, surface_envelope
**Notes:** Sharir's envelope bounds; applications to motion planning.

---

## Section: Voronoi Extensions

### CG.173: Power Diagram
**Definition:** Voronoi-like partition using power distance ||x−p||² − w_p for weighted sites.
**Cost Model:** O(n log n) via lower envelope of paraboloids.
**Real Wall:** Empty cells possible when weight too small relative to neighbors.
**Cross-Domain Aliases:** power_voronoi, laguerre_diagram, radical_diagram
**Notes:** Aurenhammer (1987); dual is regular triangulation.

---

### CG.174: Multiplicatively Weighted Voronoi
**Definition:** Voronoi using d(x,p)/w_p; cells bounded by Apollonius circles.
**Cost Model:** O(n²) cells possible; harder than additive case.
**Real Wall:** Cells can be non-convex and disconnected.
**Cross-Domain Aliases:** mw_voronoi, apollonius_diagram
**Notes:** Used in market area analysis, growth simulation.

---

### CG.175: Additively Weighted Voronoi
**Definition:** Voronoi using d(x,p) − w_p; cells bounded by hyperbolic arcs.
**Cost Model:** O(n log n) via Fortune-style sweep.
**Real Wall:** Site can have empty cell if dominated by larger-weight neighbor.
**Cross-Domain Aliases:** aw_voronoi, additive_voronoi, hyperbolic_voronoi
**Notes:** Models Voronoi of disks (each site is disk of radius w).

---

### CG.176: Farthest-Point Voronoi Diagram
**Definition:** Partition where each cell is set of points farthest from a given site.
**Cost Model:** O(n log n); only hull points have nonempty cells.
**Real Wall:** Internal points have empty cells; tree-structured diagram.
**Cross-Domain Aliases:** fp_voronoi, farthest_voronoi, max_voronoi
**Notes:** Used for smallest enclosing circle and 1-center problems.

---

### CG.177: K-th Order Voronoi Diagram
**Definition:** Partition based on which k sites are nearest to query point.
**Cost Model:** O(k(n−k)) complexity; O(k(n−k) log n) construction.
**Real Wall:** Combinatorial explosion in k; level k of arrangement.
**Cross-Domain Aliases:** higher_order_voronoi, k_voronoi, k_nearest_diagram
**Notes:** Lee (1982); related to k-levels in arrangement of bisectors.

---

### CG.178: Voronoi Diagram of Line Segments
**Definition:** Partition by nearest line segment; cells bounded by parabolic arcs.
**Cost Model:** O(n log n) via Fortune-style sweep with parabolic bisectors.
**Real Wall:** Robust construction notoriously hard; CGAL provides exact version.
**Cross-Domain Aliases:** segment_voronoi, line_segment_diagram
**Notes:** Yap (1987); foundation for medial axis of polygons.

---

### CG.179: Abstract Voronoi Diagram
**Definition:** Voronoi defined by abstract bisecting curves satisfying axioms.
**Cost Model:** O(n log n) randomized construction.
**Real Wall:** Bisector axioms must be verified for each distance function.
**Cross-Domain Aliases:** avd, klein_voronoi
**Notes:** Klein (1989); unifying framework for many Voronoi variants.

---

### CG.180: Geodesic Voronoi Diagram
**Definition:** Voronoi diagram where distance is shortest path on a surface or in polygon.
**Cost Model:** O(n log n + nk) where k is polygon edges.
**Real Wall:** Bisectors are piecewise hyperbolic; exact arithmetic difficult.
**Cross-Domain Aliases:** geodesic_voronoi, intrinsic_voronoi
**Notes:** Aronov (1989); applied to mesh segmentation and surface partitioning.

---

### CG.181: Restricted Voronoi Diagram
**Definition:** Intersection of 3D Voronoi cells with a given surface (mesh).
**Cost Model:** O(n log n + k) where k is output size.
**Real Wall:** Surface-cell intersections need exact predicates.
**Cross-Domain Aliases:** rvd, surface_voronoi
**Notes:** Yan, Lévy, Liu et al. (2009); basis for centroidal Voronoi remeshing.

---

### CG.182: Centroidal Voronoi Tessellation (CVT)
**Definition:** Voronoi diagram where each site coincides with the centroid of its cell.
**Cost Model:** Lloyd's relaxation: O(n log n) per iteration; convergence variable.
**Real Wall:** Many local minima; sensitive to initialization.
**Cross-Domain Aliases:** cvt, lloyd_relaxation, voronoi_relaxation
**Notes:** Du, Faber, Gunzburger (1999); used in stippling and mesh generation.

---

## Section: Delaunay Extensions

### CG.183: Constrained Delaunay Triangulation
**Definition:** Triangulation containing prescribed edges, Delaunay where possible.
**Cost Model:** O(n log n) via Chew's incremental or sweep-line.
**Real Wall:** Constrained edges may violate empty-circle property.
**Cross-Domain Aliases:** cdt, constrained_delaunay
**Notes:** Chew (1989); standard in CAD and mesh generation (Triangle library).

---

### CG.184: Conforming Delaunay Triangulation
**Definition:** Triangulation where every prescribed edge is union of Delaunay edges.
**Cost Model:** O(n log n + m) where m is Steiner points added.
**Real Wall:** Steiner points required; can be many for tight constraints.
**Cross-Domain Aliases:** conforming_delaunay, ccdt
**Notes:** Stronger than CDT; required for FEM with edge constraints.

---

### CG.185: Regular Triangulation
**Definition:** Weighted Delaunay; dual of power diagram for weighted sites.
**Cost Model:** O(n log n) in 2D, O(n^{⌈d/2⌉}) in d-dim.
**Real Wall:** Weights can suppress points (redundant vertices).
**Cross-Domain Aliases:** weighted_delaunay, regular_subdivision
**Notes:** Generalizes Delaunay; key for fitting and surface reconstruction.

---

### CG.186: Ruppert's Delaunay Refinement
**Definition:** Insert Steiner points to refine CDT until all triangles meet quality criteria.
**Cost Model:** O(n log n + m log m); m bounded for angle threshold < ~20.7°.
**Real Wall:** Termination only proven for angles below threshold.
**Cross-Domain Aliases:** ruppert_refinement, quality_mesh
**Notes:** Ruppert (1995); foundation of Triangle library.

---

### CG.187: Chew's Delaunay Refinement
**Definition:** Refinement guaranteeing all triangles have angles ≥ 30° in 2D.
**Cost Model:** O(n log n + m); m can be quadratic in input size.
**Real Wall:** Higher minimum angle than Ruppert but more Steiner points.
**Cross-Domain Aliases:** chew_refinement, second_chew
**Notes:** Chew (1993); first to guarantee 30° quality bound.

---

### CG.188: 3D Delaunay Tetrahedralization
**Definition:** Tetrahedralization where every tet's circumsphere is empty.
**Cost Model:** O(n²) worst case; O(n log n) expected for random points.
**Real Wall:** Sliver tetrahedra (near-flat) cause numerical issues.
**Cross-Domain Aliases:** delaunay3d, tet_mesh, delaunay_tetrahedralization
**Notes:** Edelsbrunner & Shah (1996); CGAL, TetGen implementations.

---

### CG.189: Sliver Removal (Delaunay)
**Definition:** Perturb or weight points to eliminate near-degenerate tetrahedra.
**Cost Model:** O(n log n) with weight perturbation; iterative.
**Real Wall:** No universal sliver-free guarantee in 3D Delaunay.
**Cross-Domain Aliases:** sliver_exudation, sliver_removal
**Notes:** Cheng, Dey, Edelsbrunner et al. (2000); weight pumping technique.

---

### CG.190: Kinetic Delaunay Triangulation
**Definition:** Maintain Delaunay triangulation under continuous motion of points.
**Cost Model:** O(n^{2+ε}) events expected; certificate-based.
**Real Wall:** In-circle certificate roots are degree-4 polynomials.
**Cross-Domain Aliases:** kinetic_delaunay, moving_delaunay
**Notes:** Albers, Guibas, Mitchell, Roos (1998); used in molecular simulation.

---

### CG.191: Dynamic Delaunay Triangulation
**Definition:** Maintain Delaunay under point insertions and deletions.
**Cost Model:** O(log² n) expected insertion via conflict graph; deletion harder.
**Real Wall:** Deletion involves retriangulating star polygon.
**Cross-Domain Aliases:** dynamic_delaunay, semi_dynamic_dt
**Notes:** Devillers (1999); Mucke's hierarchy for fast point location.

---

## Section: Triangulations

### CG.192: Ear Clipping Triangulation
**Definition:** Triangulate simple polygon by repeatedly removing ear vertices.
**Cost Model:** O(n²) naive; O(n log n) with ear-status structure.
**Real Wall:** Holes require splitting via bridge edges first.
**Cross-Domain Aliases:** ear_clip, ear_cut, polygon_triangulation
**Notes:** Meisters (1975); simple but not optimal; used in many engines.

---

### CG.193: Monotone Polygon Triangulation
**Definition:** Triangulate y-monotone polygon via sweep using vertex stack.
**Cost Model:** O(n) once monotone decomposition obtained.
**Real Wall:** Requires preprocessing into monotone pieces first.
**Cross-Domain Aliases:** monotone_triangulation, y_monotone_triangulate
**Notes:** Garey, Johnson, Preparata, Tarjan (1978).

---

### CG.194: Seidel's Randomized Triangulation
**Definition:** Trapezoidalize via random incremental, then triangulate trapezoids.
**Cost Model:** O(n log* n) expected for simple polygon triangulation.
**Real Wall:** Probabilistic; close to Chazelle's deterministic O(n).
**Cross-Domain Aliases:** seidel_triangulation, randomized_trapezoid
**Notes:** Seidel (1991); practical alternative to Chazelle's complex linear algorithm.

---

### CG.195: Chazelle's Linear Polygon Triangulation
**Definition:** Deterministic O(n) triangulation of simple polygon.
**Cost Model:** O(n) worst case via complex deterministic algorithm.
**Real Wall:** Theoretically optimal but rarely implemented (huge constants).
**Cross-Domain Aliases:** chazelle_triangulation, linear_polygon_triangulation
**Notes:** Chazelle (1991); landmark result, mostly theoretical.

---

### CG.196: Trapezoidal Decomposition
**Definition:** Partition planar subdivision into trapezoids by extending vertical rays.
**Cost Model:** O(n log n) deterministic; O(n) expected via randomized incremental.
**Real Wall:** Vertical edges need infinitesimal perturbation.
**Cross-Domain Aliases:** trapezoidal_map, vertical_decomposition
**Notes:** Mulmuley (1990); basis for point location data structures.

---

### CG.197: Polygon Convex Decomposition
**Definition:** Partition polygon into minimum number of convex pieces.
**Cost Model:** O(n³) optimal; O(n) Hertel-Mehlhorn 4-approximation.
**Real Wall:** Optimal NP-hard with Steiner points; without is polynomial.
**Cross-Domain Aliases:** convex_partition, convex_decomposition
**Notes:** Chazelle & Dobkin (1985); Hertel-Mehlhorn for fast approximation.

---

### CG.198: Optimal Triangulation (Min Weight)
**Definition:** Triangulate convex polygon minimizing total edge length.
**Cost Model:** O(n³) via dynamic programming.
**Real Wall:** Non-convex polygon version is NP-hard.
**Cross-Domain Aliases:** mwt, min_weight_triangulation
**Notes:** Mulzer & Rote (2006) showed NP-hardness for general case.

---

### CG.199: Steiner Triangulation
**Definition:** Triangulation allowed to add extra vertices (Steiner points).
**Cost Model:** O(n log n + m) with m Steiner points added.
**Real Wall:** Choosing m and Steiner locations is optimization problem.
**Cross-Domain Aliases:** steiner_mesh, refined_triangulation
**Notes:** Essential for FEM quality bounds; foundation of mesh generation.

---

### CG.200: Polygon Trapezoidalization
**Definition:** Decompose polygon into trapezoids via vertical sweep.
**Cost Model:** O(n log n) for n vertices.
**Real Wall:** Output size linear; trapezoids may degenerate to triangles.
**Cross-Domain Aliases:** trapezoid_partition, vertical_trapezoidize
**Notes:** Step in Seidel's algorithm and point location.

---

### CG.201: Fan Triangulation
**Definition:** Triangulate convex polygon by connecting one vertex to all others.
**Cost Model:** O(n) trivially.
**Real Wall:** Only valid for convex polygons; bad aspect ratios.
**Cross-Domain Aliases:** fan_triangulate, polygon_fan
**Notes:** Simplest triangulation; standard GPU rendering primitive.

---

## Section: Spatial Data Structures

### CG.202: Point Quadtree
**Definition:** Tree subdividing plane at each point, partitioning into 4 quadrants.
**Cost Model:** O(n log n) build; O(log n) query (balanced).
**Real Wall:** Can be unbalanced for clustered data.
**Cross-Domain Aliases:** point_quadtree, finkel_bentley_quadtree
**Notes:** Finkel & Bentley (1974); first quadtree variant.

---

### CG.203: Region Quadtree
**Definition:** Recursively subdivide square region until cells are homogeneous.
**Cost Model:** O(p) space where p is perimeter of regions.
**Real Wall:** Resolution-dependent; not data-adaptive in same way as PR.
**Cross-Domain Aliases:** region_quadtree, image_quadtree
**Notes:** Klinger (1971); standard for raster compression.

---

### CG.204: PR Quadtree (Point-Region)
**Definition:** Subdivide region recursively until each cell has at most one point.
**Cost Model:** Depth depends on min point separation; O(log(1/δ)).
**Real Wall:** Unbalanced for close points; deep recursion.
**Cross-Domain Aliases:** pr_quadtree, point_region_quadtree
**Notes:** Samet (1990); standard textbook treatment.

---

### CG.205: MX-CIF Quadtree
**Definition:** Quadtree for rectangles indexed by minimal enclosing quadtree block.
**Cost Model:** O(n log n) build; O(log n + k) range query.
**Real Wall:** Long thin rectangles concentrate at root.
**Cross-Domain Aliases:** mx_cif, cif_quadtree, rectangle_quadtree
**Notes:** Samet (1990); used in VLSI design.

---

### CG.206: Octree
**Definition:** 3D analog of quadtree: each node has 8 children (octants).
**Cost Model:** O(n log n) build for n points; depth O(log n) balanced.
**Real Wall:** Memory grows as O(8^d) in worst depth d.
**Cross-Domain Aliases:** octree_3d, octant_tree
**Notes:** Meagher (1980); used in voxel rendering, n-body simulation.

---

### CG.207: KD-Tree Variant (Adaptive)
**Definition:** KD-tree where split axis chosen by largest spread instead of cyclic.
**Cost Model:** O(n log n) build; better query on skewed data.
**Real Wall:** Re-balancing under updates is expensive.
**Cross-Domain Aliases:** adaptive_kdtree, max_spread_kd
**Notes:** Friedman, Bentley, Finkel (1977); SAH variant common in ray tracing.

---

### CG.208: KDB-Tree
**Definition:** Disk-based kd-tree with bounded fanout for paging.
**Cost Model:** O(log_B n) I/O per query in B-page model.
**Real Wall:** Rebalancing requires forced splits and complex reorganization.
**Cross-Domain Aliases:** kdb_tree, paged_kd
**Notes:** Robinson (1981); bridges kd-tree and B-tree for databases.

---

### CG.209: R-Tree
**Definition:** Hierarchy of axis-aligned bounding rectangles for spatial indexing.
**Cost Model:** O(log_B n) expected query for clustered data.
**Real Wall:** Overlap between sibling MBRs degrades query performance.
**Cross-Domain Aliases:** r_tree, mbr_tree
**Notes:** Guttman (1984); foundation of spatial databases.

---

### CG.210: R*-Tree
**Definition:** R-tree variant minimizing overlap, margin, and area on insertion.
**Cost Model:** O(log_B n) query, more work per insertion.
**Real Wall:** Forced reinsertion adds complexity.
**Cross-Domain Aliases:** r_star_tree, rstar
**Notes:** Beckmann, Kriegel, Schneider, Seeger (1990); de facto industry standard.

---

### CG.211: R+ Tree
**Definition:** R-tree without overlap by clipping rectangles into multiple subtrees.
**Cost Model:** O(log_B n) query but storage overhead.
**Real Wall:** Rectangle splitting causes duplication.
**Cross-Domain Aliases:** r_plus_tree
**Notes:** Sellis, Roussopoulos, Faloutsos (1987).

---

### CG.212: Hilbert R-Tree
**Definition:** R-tree where entries ordered by Hilbert curve value of MBR center.
**Cost Model:** O(log_B n) query; better clustering than basic R-tree.
**Real Wall:** Hilbert value computation per insert.
**Cross-Domain Aliases:** hilbert_r_tree, sfc_rtree
**Notes:** Kamel & Faloutsos (1994); space-filling curve ordering.

---

### CG.213: X-Tree
**Definition:** R-tree for high-dimensional data using supernodes when split degenerates.
**Cost Model:** Avoids degeneration; can degrade to linear scan in very high dim.
**Real Wall:** Curse of dimensionality limits effectiveness beyond ~16D.
**Cross-Domain Aliases:** x_tree, extended_rtree
**Notes:** Berchtold, Keim, Kriegel (1996).

---

### CG.214: BSP Tree (Auto-Partition)
**Definition:** Binary space partition built from input segments/planes.
**Cost Model:** Θ(n log n) expected size in 2D, Θ(n²) worst.
**Real Wall:** Higher dim partition size can blow up exponentially.
**Cross-Domain Aliases:** auto_bsp, segment_bsp
**Notes:** Paterson & Yao (1990); used in Doom/Quake-era rendering.

---

### CG.215: AABB Tree
**Definition:** Binary tree of axis-aligned bounding boxes for collision/ray queries.
**Cost Model:** O(log n) expected query; O(n log n) build.
**Real Wall:** Loose bounds for rotated geometry hurt performance.
**Cross-Domain Aliases:** aabb_bvh, box_bvh
**Notes:** Standard BVH in CGAL and physics engines.

---

### CG.216: OBB Tree
**Definition:** Hierarchical oriented bounding boxes built from PCA of geometry.
**Cost Model:** O(n log n) build; tighter bounds than AABB.
**Real Wall:** Expensive overlap test (15 SAT axes per pair).
**Cross-Domain Aliases:** obb_tree, gottschalk_obb
**Notes:** Gottschalk, Lin, Manocha (1996); RAPID library.

---

### CG.217: k-DOP Tree
**Definition:** BVH using k-discrete-oriented polytopes (intersection of k slabs).
**Cost Model:** O(k) overlap test; better fit than AABB for k=14, 18, 26.
**Real Wall:** Higher k means more storage and overlap test cost.
**Cross-Domain Aliases:** kdop, slab_bvh
**Notes:** Klosowski et al. (1998); trade-off knob between AABB and OBB.

---

### CG.218: Sphere Tree
**Definition:** Hierarchy of bounding spheres for collision detection.
**Cost Model:** O(1) overlap test; loose for elongated objects.
**Real Wall:** Rotation-invariant but loose bounds.
**Cross-Domain Aliases:** sphere_bvh, ssv_tree
**Notes:** Hubbard (1996); good for deformable bodies.

---

### CG.219: Range Tree
**Definition:** Multi-level BST supporting d-dimensional orthogonal range queries.
**Cost Model:** O(n log^{d−1} n) space; O(log^d n + k) query.
**Real Wall:** Space blowup beyond 3D; deletions complex.
**Cross-Domain Aliases:** range_tree, multilevel_bst, bentley_range_tree
**Notes:** Bentley (1980); standard textbook structure.

---

### CG.220: Fractional Cascading
**Definition:** Technique saving log factor on iterated binary search across lists.
**Cost Model:** Replaces O(log^k n) with O(log n + k).
**Real Wall:** Implementation complex; auxiliary catalog pointers.
**Cross-Domain Aliases:** fractional_cascading, chazelle_guibas
**Notes:** Chazelle & Guibas (1986); applied to range tree queries.

---

## Section: Range Searching & Nearest Neighbor

### CG.221: Simplex Range Searching
**Definition:** Report or count points inside an arbitrary d-simplex.
**Cost Model:** O(n^{1−1/d + ε}) query with linear-space partition tree.
**Real Wall:** Lower bound matches via Chazelle's argument.
**Cross-Domain Aliases:** simplex_range, partition_tree_query
**Notes:** Matoušek (1992) partition trees; near-optimal.

---

### CG.222: Half-Space Range Counting
**Definition:** Count points on one side of a hyperplane.
**Cost Model:** O(n^{1−1/d}) query, O(n) space; O(log n) with O(n^d) space.
**Real Wall:** Time-space tradeoff governed by Chazelle's bounds.
**Cross-Domain Aliases:** halfspace_count, partition_tree
**Notes:** Application of partition theorem on point sets.

---

### CG.223: Semialgebraic Range Searching
**Definition:** Report points satisfying constant-degree polynomial inequalities.
**Cost Model:** Near O(n^{1−1/d}) using polynomial method.
**Real Wall:** Polynomial partitioning constants are huge.
**Cross-Domain Aliases:** semialgebraic_range, polynomial_partition
**Notes:** Agarwal, Matoušek, Sharir (2013); breakthrough via polynomial method.

---

### CG.224: Ball Tree
**Definition:** Recursive binary tree of nested balls for high-d nearest neighbor.
**Cost Model:** O(log n) average query, O(n) worst case.
**Real Wall:** Curse of dimensionality; degrades beyond ~20D.
**Cross-Domain Aliases:** ball_tree, metric_ball_tree
**Notes:** Omohundro (1989); standard in scikit-learn.

---

### CG.225: Cover Tree
**Definition:** Multi-resolution metric tree with explicit invariants for fast NN.
**Cost Model:** O(c^{12} log n) query where c is doubling dimension.
**Real Wall:** Hidden constant in c^{12} large; expansion rate matters.
**Cross-Domain Aliases:** cover_tree, beygelzimer_tree
**Notes:** Beygelzimer, Kakade, Langford (2006).

---

### CG.226: VP-Tree (Vantage-Point Tree)
**Definition:** Binary tree partitioning metric space by distance to a vantage point.
**Cost Model:** O(log n) average query; works in arbitrary metric space.
**Real Wall:** Tightness of partition depends on metric structure.
**Cross-Domain Aliases:** vp_tree, vantage_point_tree
**Notes:** Yianilos (1993).

---

### CG.227: M-Tree
**Definition:** Disk-based metric tree using routing objects and covering radii.
**Cost Model:** O(log_B n) I/O; supports range and k-NN.
**Real Wall:** Overlapping subtrees and insertion-order sensitivity.
**Cross-Domain Aliases:** m_tree, ciaccia_tree
**Notes:** Ciaccia, Patella, Zezula (1997); database-oriented.

---

### CG.228: Locality-Sensitive Hashing (ANN)
**Definition:** Hash family where similar points collide with higher probability.
**Cost Model:** O(n^{1/(1+ε)}) query for c-approximate NN.
**Real Wall:** Many hash tables required; high memory.
**Cross-Domain Aliases:** lsh, p_stable_lsh, ann_lsh
**Notes:** Indyk & Motwani (1998); Datar et al. (2004) for L_p norms.

---

### CG.229: FLANN (Fast Library for ANN)
**Definition:** Automatically tuned mixture of randomized kd-trees and k-means trees.
**Cost Model:** Sublinear empirically; tuned to data and precision target.
**Real Wall:** No worst-case guarantees; empirical optimization.
**Cross-Domain Aliases:** flann, randomized_kd_forest
**Notes:** Muja & Lowe (2009); widely used for SIFT/SURF descriptors.

---

### CG.230: HNSW (Hierarchical NSW)
**Definition:** Multi-layer proximity graph for approximate nearest neighbor.
**Cost Model:** O(log n) empirical; navigable small world graph.
**Real Wall:** Memory-heavy; deletions degrade quality.
**Cross-Domain Aliases:** hnsw, nsw_graph, navigable_graph
**Notes:** Malkov & Yashunin (2018); state-of-the-art for vector search.

---

## Section: Sweep-Line Algorithms

### CG.231: Bentley-Ottmann Segment Intersection
**Definition:** Sweep line reporting all k intersections among n segments.
**Cost Model:** O((n+k) log n).
**Real Wall:** Degenerate cases (collinear, vertical, common endpoints) tricky.
**Cross-Domain Aliases:** bo_sweep, segment_intersection, bentley_ottmann
**Notes:** Bentley & Ottmann (1979); foundation for plane-sweep technique.

---

### CG.232: Plane Sweep for Closest Pair
**Definition:** Find closest pair via line sweep maintaining strip of width δ.
**Cost Model:** O(n log n).
**Real Wall:** Constant factor higher than divide-and-conquer Shamos.
**Cross-Domain Aliases:** sweep_closest_pair, hinrichs_nievergelt
**Notes:** Hinrichs et al. (1988); alternative to D&C closest pair.

---

### CG.233: Topological Sweep
**Definition:** Sweep over arrangement via topological order, avoiding sorting.
**Cost Model:** O(n²) total for line arrangement; no sort log factor.
**Real Wall:** Implementation complex with elementary steps.
**Cross-Domain Aliases:** topo_sweep, edelsbrunner_guibas
**Notes:** Edelsbrunner & Guibas (1989); efficient arrangement traversal.

---

### CG.234: Largest Empty Rectangle (Sweep)
**Definition:** Find max-area axis-aligned rectangle in point set containing no points.
**Cost Model:** O(n²) with sweep; O(n log² n) with priority structures.
**Real Wall:** Reduces to many candidate rectangles; pruning critical.
**Cross-Domain Aliases:** max_empty_rect, ler
**Notes:** Atallah & Kosaraju (1985); applied to chip layout.

---

### CG.235: Sweep for Boolean Polygon Operations
**Definition:** Compute polygon union/intersection/difference via line sweep.
**Cost Model:** O((n+k) log n) with k output edges.
**Real Wall:** Numerical robustness of segment intersection.
**Cross-Domain Aliases:** boolean_sweep, polygon_boolean_sweep
**Notes:** Used in Clipper2 library and CGAL.

---

## Section: Collision Detection

### CG.236: GJK Algorithm
**Definition:** Detect collision between convex shapes by searching origin in Minkowski difference.
**Cost Model:** O(m) iterations expected, each O(d) support query.
**Real Wall:** Numerical robustness near contact; requires good support function.
**Cross-Domain Aliases:** gjk, gilbert_johnson_keerthi
**Notes:** Gilbert, Johnson, Keerthi (1988); foundation of modern physics engines.

---

### CG.237: EPA (Expanding Polytope Algorithm)
**Definition:** Extract penetration depth and normal from GJK simplex at collision.
**Cost Model:** O(iterations); each expands polytope toward origin.
**Real Wall:** Slow convergence for nearly-parallel faces.
**Cross-Domain Aliases:** epa, expanding_polytope
**Notes:** Van den Bergen (2001); pair with GJK for full contact info.

---

### CG.238: Minkowski Portal Refinement (MPR/XenoCollide)
**Definition:** Alternative to GJK using portal refinement in Minkowski difference.
**Cost Model:** O(m) iterations; often faster than GJK in practice.
**Real Wall:** Penetration depth not as accurate as EPA.
**Cross-Domain Aliases:** mpr, xenocollide
**Notes:** Snethen (2008); single-shot detection with normal.

---

### CG.239: Separating Axis Theorem (SAT)
**Definition:** Two convex shapes disjoint iff some axis separates their projections.
**Cost Model:** O(F·V) for OBBs; 15 axes per pair.
**Real Wall:** Doesn't generalize beyond convex polyhedra easily.
**Cross-Domain Aliases:** sat, separating_axis
**Notes:** Standard in game physics; Gottschalk for OBB.

---

### CG.240: Continuous Collision Detection (CCD)
**Definition:** Detect collision along time interval, not just endpoints.
**Cost Model:** O(log(1/ε)) bisection; conservative advancement.
**Real Wall:** Tunneling without CCD; full CCD is expensive.
**Cross-Domain Aliases:** ccd, continuous_collision, swept_collision
**Notes:** Used in cloth and rigid body simulation; Bullet, Box2D.

---

### CG.241: Conservative Advancement
**Definition:** Advance time by lower bound on time-of-impact each iteration.
**Cost Model:** O(log(1/ε)) iterations to TOI accuracy.
**Real Wall:** Slow for tangent contact; deadlock without epsilon.
**Cross-Domain Aliases:** ca_ccd, mirtich_advancement
**Notes:** Mirtich (2000); standard CCD algorithm in rigid body engines.

---

### CG.242: Swept Volume
**Definition:** Set occupied by moving object over time interval.
**Cost Model:** O(n²) for linear motion; nontrivial for rotation.
**Real Wall:** Exact swept volume requires algebraic surface intersection.
**Cross-Domain Aliases:** swept_volume, motion_volume
**Notes:** Foundation for CCD and motion planning safety.

---

## Section: Distance and Proximity

### CG.243: Hausdorff Distance
**Definition:** max of one-way max-min distances between two sets.
**Cost Model:** O((n+m) log(n+m)) for polygons; O(nm) naive for points.
**Real Wall:** Directed version asymmetric; small change can flip dominant pair.
**Cross-Domain Aliases:** hausdorff, hd_distance
**Notes:** Used in shape matching, mesh comparison; Alt et al. for polygons.

---

### CG.244: Fréchet Distance (Continuous)
**Definition:** Min over reparameterizations of max distance between two curves.
**Cost Model:** O(nm log(nm)) via free-space diagram and parametric search.
**Real Wall:** Cell decomposition decision problem inherent quadratic.
**Cross-Domain Aliases:** frechet, dog_leash_distance
**Notes:** Alt & Godau (1995); SETH lower bound suggests no strongly subquadratic.

---

### CG.245: Discrete Fréchet Distance
**Definition:** Fréchet over discrete polygonal vertex sequence (no continuous params).
**Cost Model:** O(nm) via dynamic programming.
**Real Wall:** Faster than continuous; less precise for curve geometry.
**Cross-Domain Aliases:** discrete_frechet, dfd
**Notes:** Eiter & Mannila (1994); used in trajectory analysis.

---

### CG.246: Dynamic Time Warping (DTW) for Trajectories
**Definition:** Min-cost alignment between two sequences allowing local time stretching.
**Cost Model:** O(nm) standard; subquadratic with pruning (FastDTW).
**Real Wall:** Not a metric (triangle inequality fails).
**Cross-Domain Aliases:** dtw, time_warp_distance
**Notes:** Berndt & Clifford (1994); ubiquitous in time-series.

---

### CG.247: Geodesic Distance on Mesh (MMP)
**Definition:** Exact polyhedral geodesic via window propagation across edges.
**Cost Model:** O(n² log n) worst; practical O(n²).
**Real Wall:** Window data structures complex; floating-point robustness.
**Cross-Domain Aliases:** mmp_geodesic, exact_geodesic
**Notes:** Mitchell, Mount, Papadimitriou (1987); CHL (Chen-Han) is faster.

---

### CG.248: Fast Marching Method
**Definition:** Approximate geodesic distance on grid via upwind Eikonal solver.
**Cost Model:** O(n log n) with heap; O(n) with Sethian's sweeping.
**Real Wall:** First-order accuracy; grid alignment artifacts.
**Cross-Domain Aliases:** fmm, eikonal_solver
**Notes:** Sethian (1996); standard in level set methods.

---

### CG.249: Heat Method for Geodesics
**Definition:** Diffuse heat briefly, normalize gradient, integrate to get geodesic.
**Cost Model:** Two sparse linear solves; precomputable factorization.
**Real Wall:** Approximation, not exact; degrades near concave regions.
**Cross-Domain Aliases:** heat_method, crane_heat
**Notes:** Crane, Weischedel, Wardetzky (2013).

---

### CG.250: Signed Distance Field (SDF)
**Definition:** Scalar field giving signed distance to nearest surface, sign by inside/out.
**Cost Model:** O(n) per query for explicit; O(d_max) for sphere tracing.
**Real Wall:** Sign computation requires watertight surface.
**Cross-Domain Aliases:** sdf, signed_distance
**Notes:** Foundation for sphere tracing, neural implicit surfaces.

---

## Section: Geometric Predicates

### CG.251: Shewchuk's Adaptive Predicates
**Definition:** Floating-point predicates using expansions of error-free sums.
**Cost Model:** O(1) when input is non-degenerate; O(predicate degree) worst.
**Real Wall:** Implementation requires careful bound tracking.
**Cross-Domain Aliases:** shewchuk_predicates, adaptive_orient
**Notes:** Shewchuk (1997); standard for orient2d/3d, incircle, insphere.

---

### CG.252: Filtered Predicates
**Definition:** Try fast floating-point first; fall back to exact only when needed.
**Cost Model:** O(1) fast path; O(N) exact fallback for degenerate.
**Real Wall:** Tuning epsilon for filter threshold.
**Cross-Domain Aliases:** filtered_predicate, interval_filter
**Notes:** Used throughout CGAL; key to robust+fast computation.

---

### CG.253: Exact Geometric Computation (EGC)
**Definition:** Compute predicates with arbitrary precision until sign determined.
**Cost Model:** Variable; depends on expression depth.
**Real Wall:** Lazy evaluation needed to maintain practical speed.
**Cross-Domain Aliases:** egc, exact_predicate
**Notes:** Yap & Dubé (1995); LEDA real and CGAL Number_type framework.

---

### CG.254: In-Sphere Predicate
**Definition:** Sign test for whether a point lies inside circumsphere of 4 others.
**Cost Model:** Determinant of 5x5 matrix; degree 6 polynomial in coords.
**Real Wall:** Catastrophic cancellation for near-cosperial points.
**Cross-Domain Aliases:** in_sphere, insphere_predicate
**Notes:** Core predicate for 3D Delaunay; Shewchuk's adaptive version.

---

### CG.255: Snap Rounding
**Definition:** Round vertices of arrangement to grid while preserving topology.
**Cost Model:** O((n+I) log n) where I is intersection count.
**Real Wall:** Hot pixels can capture many segments; quality of grid critical.
**Cross-Domain Aliases:** snap_rounding, robust_rounding
**Notes:** Greene & Yao (1986); Hobby (1999); standard robustness technique.

---

## Section: Polygon Operations

### CG.256: Weiler-Atherton Polygon Clipping
**Definition:** Clip arbitrary polygon against another using intersection-vertex linked lists.
**Cost Model:** O((n+m+k) log n) where k is intersection count.
**Real Wall:** Degenerate (collinear, touching) cases require careful classification.
**Cross-Domain Aliases:** weiler_atherton, wa_clip
**Notes:** Weiler & Atherton (1977); handles concave polygons with holes.

---

### CG.257: Vatti Polygon Clipping
**Definition:** Sweep-line Boolean clipping using scanbeams and active edge table.
**Cost Model:** O((n+m+k) log(n+m)).
**Real Wall:** Implementation complex; degenerate edges handled via fill rule.
**Cross-Domain Aliases:** vatti_clip, scanbeam_clip
**Notes:** Vatti (1992); basis for Clipper library.

---

### CG.258: Greiner-Hormann Polygon Clipping
**Definition:** Intersect polygons by toggling between traversals at intersection nodes.
**Cost Model:** O((n+m+k)) after intersection points found.
**Real Wall:** Original cannot handle degeneracies; extended versions exist.
**Cross-Domain Aliases:** greiner_hormann, gh_clip
**Notes:** Greiner & Hormann (1998); simple implementation.

---

### CG.259: Polygon Offsetting
**Definition:** Inflate or shrink polygon by distance d (Minkowski sum/diff with disk).
**Cost Model:** O(n²) worst; O(n log n) with arrangement-based.
**Real Wall:** Topology changes (splits, merges) at concave vertices.
**Cross-Domain Aliases:** polygon_offset, polygon_inflate, polygon_buffer
**Notes:** Clipper2 implementation widely used; foundation of CAM toolpath generation.

---

### CG.260: Straight Skeleton
**Definition:** Locus of self-intersection of inward edge offsets at constant speed.
**Cost Model:** O(n³ log n) general; O(n log n) for convex.
**Real Wall:** Simultaneous events near degenerate polygons.
**Cross-Domain Aliases:** straight_skeleton, aichholzer_skeleton
**Notes:** Aichholzer & Aurenhammer (1996); roof construction interpretation.

---

### CG.261: Medial Axis (Polygon)
**Definition:** Locus of centers of maximal inscribed disks in polygon.
**Cost Model:** O(n log n) for simple polygon via Voronoi of edges.
**Real Wall:** Parabolic arcs; numerical sensitivity at convex vertices.
**Cross-Domain Aliases:** medial_axis, ma, polygon_skeleton
**Notes:** Lee (1982); related to but distinct from straight skeleton.

---

### CG.262: Monotone Polygon Decomposition
**Definition:** Partition polygon into y-monotone sub-polygons.
**Cost Model:** O(n log n) via plane sweep.
**Real Wall:** Step before triangulation pipeline.
**Cross-Domain Aliases:** monotone_decomp, y_monotone_partition
**Notes:** de Berg, van Kreveld, Overmars, Schwarzkopf textbook.

---

### CG.263: Hertel-Mehlhorn Convex Partition
**Definition:** Remove non-essential diagonals from triangulation to merge convex pieces.
**Cost Model:** O(n) after triangulation.
**Real Wall:** 4-approximation, not optimal.
**Cross-Domain Aliases:** hm_convex, hertel_mehlhorn
**Notes:** Hertel & Mehlhorn (1985); practical convex decomposition.

---

### CG.264: Simple Polygon Test
**Definition:** Determine whether polygon has self-intersecting edges.
**Cost Model:** O(n log n) via Bentley-Ottmann.
**Real Wall:** Edge-touching corners vs. true crossing.
**Cross-Domain Aliases:** simple_polygon_test, self_intersect_test
**Notes:** Pre-pass for many polygon algorithms.

---

## Section: Visibility and Motion Planning

### CG.265: Visibility Polygon
**Definition:** Region of polygon visible from query point (line-of-sight).
**Cost Model:** O(n log n); O(n) when sorted radially.
**Real Wall:** Output complexity Θ(n); robustness on near-collinear edges.
**Cross-Domain Aliases:** visibility_polygon, isovist
**Notes:** Joe & Simpson (1987); used in robotics and lighting.

---

### CG.266: Visibility Graph
**Definition:** Graph with edges between mutually-visible vertices of polygonal obstacles.
**Cost Model:** O(n²) via radial sweep; O(n² log n) naive.
**Real Wall:** Quadratic edges typical; reduced visibility graph for shortest path.
**Cross-Domain Aliases:** visibility_graph, vis_graph
**Notes:** Lozano-Pérez & Wesley (1979); foundation of shortest-path motion planning.

---

### CG.267: Shortest Path in Polygon
**Definition:** Geodesic shortest path between two points inside simple polygon.
**Cost Model:** O(n) via funnel algorithm after triangulation.
**Real Wall:** Polygon-with-holes case is NP-hard for min-link variant.
**Cross-Domain Aliases:** funnel_path, lee_preparata_sp
**Notes:** Lee & Preparata (1984); Guibas et al. funnel-based linear algorithm.

---

### CG.268: Art Gallery Problem
**Definition:** Min number of guards to cover polygon interior.
**Cost Model:** NP-hard general; Chvátal's ⌊n/3⌋ upper bound for simple polygons.
**Real Wall:** Optimal placement NP-hard; approximation O(log n) ratio.
**Cross-Domain Aliases:** art_gallery, polygon_guarding, chvatal_theorem
**Notes:** Chvátal (1975), Fisk (1978) for elegant proof.

---

### CG.269: Watchman Route
**Definition:** Shortest closed route seeing entire interior of polygon.
**Cost Model:** Polynomial for simple polygon; NP-hard with holes.
**Real Wall:** Combination of TSP and visibility.
**Cross-Domain Aliases:** watchman_route, polygon_patrol
**Notes:** Chin & Ntafos (1991).

---

### CG.270: Probabilistic Roadmap (PRM)
**Definition:** Sample configurations, connect collision-free pairs, query via graph search.
**Cost Model:** O(n log n) per node connection; depends on sampling density.
**Real Wall:** Narrow passages need high sampling density.
**Cross-Domain Aliases:** prm, probabilistic_roadmap
**Notes:** Kavraki et al. (1996); workhorse of motion planning.

---

### CG.271: RRT (Rapidly-exploring Random Tree)
**Definition:** Incrementally grow tree by extending toward random samples.
**Cost Model:** Per iteration: NN query + collision check.
**Real Wall:** Asymptotically not optimal; biased to Voronoi-large regions.
**Cross-Domain Aliases:** rrt, lavalle_tree
**Notes:** LaValle (1998); standard single-query planner.

---

### CG.272: RRT*
**Definition:** RRT variant that rewires neighborhood to maintain asymptotic optimality.
**Cost Model:** Per iteration: O(log n) NN + neighborhood rewire.
**Real Wall:** Memory grows; rewire constants high.
**Cross-Domain Aliases:** rrt_star, karaman_frazzoli
**Notes:** Karaman & Frazzoli (2011); asymptotically optimal.

---

### CG.273: Configuration Space Obstacle
**Definition:** Image of workspace obstacle under robot kinematics in C-space.
**Cost Model:** Exponential in DOF; sample-based methods preferred.
**Real Wall:** Exact c-obstacle hard; usually approximated.
**Cross-Domain Aliases:** c_obstacle, cspace_obstacle
**Notes:** Lozano-Pérez (1983).

---

### CG.274: Minkowski Sum (Polygons)
**Definition:** Set of all sums of pairs of points from two polygons.
**Cost Model:** O((n+m) log(n+m)) convex case; O(n²m² log nm) general.
**Real Wall:** Holes and non-convexity dramatically increase output complexity.
**Cross-Domain Aliases:** minkowski_sum, polygon_dilation
**Notes:** Used for C-space obstacles; CGAL implementation.

---

### CG.275: Minkowski Difference
**Definition:** Set difference via Minkowski sum with reflected operand.
**Cost Model:** Same as Minkowski sum.
**Real Wall:** Empty result if no inclusion possible.
**Cross-Domain Aliases:** minkowski_diff, erosion
**Notes:** Foundation of GJK; collision detection in physics engines.

---

## Section: Mesh Generation and Processing

### CG.276: Advancing Front Mesh Generation
**Definition:** Grow mesh outward from boundary, adding elements one by one.
**Cost Model:** O(n log n) typical; closure step can be expensive.
**Real Wall:** Front collision detection; difficulty in closing pockets.
**Cross-Domain Aliases:** advancing_front, paving
**Notes:** Lo (1985); high-quality boundary fitted meshes.

---

### CG.277: Octree-Based Mesh Generation
**Definition:** Build mesh from octree subdivision of domain with surface fitting.
**Cost Model:** O(n log n) for octree; surface tracking adds cost.
**Real Wall:** Boundary fitting via dual contouring or warping.
**Cross-Domain Aliases:** octree_mesh, dual_contour_mesh
**Notes:** Schroeder & Shephard (1990).

---

### CG.278: Laplacian Mesh Smoothing
**Definition:** Move each vertex toward centroid of neighbors iteratively.
**Cost Model:** O(n) per iteration; O(k) per vertex with k neighbors.
**Real Wall:** Shrinkage; can invert elements in concave regions.
**Cross-Domain Aliases:** laplacian_smooth, umbrella_smooth
**Notes:** Field (1988); ubiquitous default smoother.

---

### CG.279: Angle-Based Smoothing
**Definition:** Adjust vertex position to equalize incident triangle angles.
**Cost Model:** O(n·k) per iteration.
**Real Wall:** Slower than Laplacian; no inversion if step bounded.
**Cross-Domain Aliases:** angle_smooth, zhou_shimada
**Notes:** Zhou & Shimada (2000); preserves volume better than Laplacian.

---

### CG.280: QEM Mesh Decimation
**Definition:** Edge collapses ordered by quadric error metric.
**Cost Model:** O(n log n) for n initial edges via priority queue.
**Real Wall:** Boundary preservation requires constraint quadrics.
**Cross-Domain Aliases:** qem, garland_heckbert, quadric_decimation
**Notes:** Garland & Heckbert (1997); standard mesh simplification.

---

### CG.281: Loop Subdivision
**Definition:** Subdivision surface scheme for triangle meshes producing C² limits.
**Cost Model:** O(n) per level, factor 4 per level.
**Real Wall:** C¹ at extraordinary vertices, not C².
**Cross-Domain Aliases:** loop_subdivision, triangle_subdivision
**Notes:** Charles Loop (1987); standard for triangle subdivision.

---

### CG.282: Catmull-Clark Subdivision
**Definition:** Subdivision scheme for quad meshes generalizing bicubic B-splines.
**Cost Model:** O(n) per level; factor 4.
**Real Wall:** C¹ at extraordinary vertices.
**Cross-Domain Aliases:** catmull_clark, cc_subdivision
**Notes:** Catmull & Clark (1978); used in Pixar's RenderMan.

---

### CG.283: Doo-Sabin Subdivision
**Definition:** Dual subdivision scheme creating new vertex per face corner.
**Cost Model:** O(n) per level; produces biquadratic B-spline limit.
**Real Wall:** Mesh growth and topology bookkeeping.
**Cross-Domain Aliases:** doo_sabin, ds_subdivision
**Notes:** Doo & Sabin (1978).

---

### CG.284: Remeshing (Isotropic)
**Definition:** Resample surface mesh for uniform edge length and quality.
**Cost Model:** O(n log n) per iteration; multiple iterations.
**Real Wall:** Feature preservation; tangential vs. normal motion.
**Cross-Domain Aliases:** isotropic_remesh, botsch_kobbelt
**Notes:** Botsch & Kobbelt (2004); standard pipeline.

---

## Section: Surface Reconstruction

### CG.285: Crust Algorithm
**Definition:** Reconstruct surface from points using Voronoi poles.
**Cost Model:** O(n log n) for Voronoi + tetrahedralization.
**Real Wall:** Sampling density assumption (ε-sample).
**Cross-Domain Aliases:** crust, amenta_crust
**Notes:** Amenta, Bern, Kamvysselis (1998); theoretical guarantees.

---

### CG.286: Power Crust
**Definition:** Reconstruction via power diagram of polar balls.
**Cost Model:** O(n log n + n²) for power diagram.
**Real Wall:** Output may include extra structure (medial axis approximation).
**Cross-Domain Aliases:** power_crust, amenta_choi_kolluri
**Notes:** Amenta, Choi, Kolluri (2001); watertight output.

---

### CG.287: Alpha Shape
**Definition:** Generalization of convex hull parametrized by radius α.
**Cost Model:** O(n log n) in 2D, O(n²) in 3D via Delaunay filtering.
**Real Wall:** Single α rarely captures multi-scale structure.
**Cross-Domain Aliases:** alpha_shape, edelsbrunner_alpha
**Notes:** Edelsbrunner & Mücke (1994).

---

### CG.288: Ball-Pivoting Algorithm
**Definition:** Roll a ball of radius ρ over points; mesh from triangles it rests on.
**Cost Model:** O(n log n) with spatial index.
**Real Wall:** Choosing ρ; misses thin features.
**Cross-Domain Aliases:** bpa, ball_pivoting, bernardini_bpa
**Notes:** Bernardini et al. (1999); classic scanner reconstruction.

---

### CG.289: Poisson Surface Reconstruction
**Definition:** Solve Poisson equation for indicator function from oriented points.
**Cost Model:** O(n) octree-based; sparse solve.
**Real Wall:** Requires good normals; smooths fine detail.
**Cross-Domain Aliases:** poisson_recon, kazhdan_recon
**Notes:** Kazhdan, Bolitho, Hoppe (2006).

---

### CG.290: Marching Cubes
**Definition:** Extract isosurface from scalar grid via per-cube lookup table.
**Cost Model:** O(n) for n grid cells.
**Real Wall:** Topological ambiguity in 14-case table; sharp features lost.
**Cross-Domain Aliases:** mc, marching_cubes, lorensen_cline
**Notes:** Lorensen & Cline (1987); ubiquitous in medical imaging.

---

### CG.291: Marching Tetrahedra
**Definition:** Marching cubes variant subdividing each cube into tets first.
**Cost Model:** O(n) with up to 6 tets per cube.
**Real Wall:** Output triangle count up to 6x; no ambiguity though.
**Cross-Domain Aliases:** marching_tets, mt_isosurface
**Notes:** Resolves marching cubes ambiguities at cost of triangle count.

---

### CG.292: Dual Contouring
**Definition:** Place vertex inside each cell minimizing QEF; connect across edges.
**Cost Model:** O(n) cells with linear least-squares solve per cell.
**Real Wall:** Self-intersections possible; QEF preserves sharp features.
**Cross-Domain Aliases:** dual_contour, ju_losasso_schaefer
**Notes:** Ju, Losasso, Schaefer, Warren (2002); preserves sharp features.

---

## Section: Curves, Surfaces, and Arrangements

### CG.293: NURBS Curve Evaluation
**Definition:** Non-uniform rational B-spline evaluated via de Boor with weights.
**Cost Model:** O(p²) for degree p at one parameter.
**Real Wall:** Weight choice affects shape; control net manipulation.
**Cross-Domain Aliases:** nurbs, rational_bspline
**Notes:** Industry standard in CAD; ISO 10303 STEP format.

---

### CG.294: Curve-Curve Intersection
**Definition:** Find all intersection points between two parametric curves.
**Cost Model:** O(d² log(1/ε)) for degree-d curves via Bezier subdivision.
**Real Wall:** Tangential and self-intersections require special handling.
**Cross-Domain Aliases:** cc_intersect, curve_intersect
**Notes:** Sederberg & Nishita (1990); foundation of boolean operations.

---

### CG.295: Ray-Surface Intersection
**Definition:** Find intersection of ray with parametric or implicit surface.
**Cost Model:** O(deg) for implicit polynomials; subdivision for parametric.
**Real Wall:** Numerical roots of high-degree polynomials.
**Cross-Domain Aliases:** ray_surf, surface_ray_cast
**Notes:** Foundation of ray tracing for NURBS and subdivision surfaces.

---

### CG.296: Line Arrangement
**Definition:** Subdivision of plane induced by n lines: O(n²) vertices/edges/faces.
**Cost Model:** O(n²) construction via incremental zone or topological sweep.
**Real Wall:** Quadratic output; cannot do better.
**Cross-Domain Aliases:** line_arrangement, arrangement_2d
**Notes:** Chazelle, Guibas, Lee (1985); foundation for many algorithms.

---

### CG.297: Zone Theorem
**Definition:** Complexity of cells crossed by a line in arrangement is O(n).
**Cost Model:** Linear zone size enables O(n²) incremental arrangement.
**Real Wall:** Constant factor 4 in zone; tight bound by Edelsbrunner.
**Cross-Domain Aliases:** zone_theorem, line_zone
**Notes:** Edelsbrunner, O'Rourke, Seidel (1986).

---

### CG.298: Hyperplane Arrangement (d-dim)
**Definition:** Subdivision of R^d by n hyperplanes; faces of various dimensions.
**Cost Model:** O(n^d) total complexity; O(n^d) construction.
**Real Wall:** Combinatorial explosion in d; useful via levels.
**Cross-Domain Aliases:** hyperplane_arrangement, dd_arrangement
**Notes:** Edelsbrunner (1987); foundation of computational geometry.

---

### CG.299: Levels in Arrangement
**Definition:** k-level: locus of points lying above exactly k hyperplanes.
**Cost Model:** O(n^{⌊d/2⌋+1}) bound; tight for some k.
**Real Wall:** k-set problem open; sublinear improvements over n^{d-1} hard.
**Cross-Domain Aliases:** k_level, k_set
**Notes:** Dey (1998); used for kth-nearest, ham-sandwich.

---

### CG.300: Cylindrical Algebraic Decomposition (CAD)
**Definition:** Decompose R^d into cells where polynomial signs are invariant.
**Cost Model:** Doubly exponential in d.
**Real Wall:** Foundational but impractical above d=4 for arbitrary polynomials.
**Cross-Domain Aliases:** cad, collins_cad
**Notes:** Collins (1975); basis of Mathematica's algebraic solver.

---

## Section: Geometric Optimization and Topology

### CG.301: Welzl's Smallest Enclosing Ball
**Definition:** Recursive algorithm for min-enclosing ball with at most d+1 boundary points.
**Cost Model:** O(n) expected for fixed d; O((d+1)!) constant.
**Real Wall:** Move-to-front heuristic essential for practical speed.
**Cross-Domain Aliases:** welzl_meb, smallest_enclosing_ball, miniball
**Notes:** Welzl (1991); Gärtner's implementation widely used.

---

### CG.302: Smallest Enclosing Rectangle (Min-Area)
**Definition:** Min-area rectangle enclosing point set; rotates with point hull.
**Cost Model:** O(n log n) for hull + O(n) rotating calipers.
**Real Wall:** Optimal rectangle has edge collinear with hull edge (theorem).
**Cross-Domain Aliases:** ser, min_area_rect, freeman_shapira
**Notes:** Freeman & Shapira (1975); Toussaint's rotating calipers.

---

### CG.303: Khachiyan's Min-Volume Ellipsoid
**Definition:** Smallest enclosing ellipsoid (Löwner-John) via interior-point.
**Cost Model:** O(n d² /ε) iterations.
**Real Wall:** Conditioning for high-dim or near-flat point sets.
**Cross-Domain Aliases:** khachiyan_ellipsoid, lj_ellipsoid, mvee
**Notes:** Khachiyan (1996); foundation of ellipsoid method in LP.

---

### CG.304: Löwner-John Ellipsoid
**Definition:** Unique min-volume ellipsoid containing convex body K.
**Cost Model:** SDP solve; O(d^{3.5}) interior point per iteration.
**Real Wall:** Tight if and only if d+ contact points exist.
**Cross-Domain Aliases:** lj_ellipsoid, john_ellipsoid
**Notes:** John (1948); fundamental in convex geometry.

---

### CG.305: Geometric Median (Weiszfeld)
**Definition:** Point minimizing sum of Euclidean distances to input points.
**Cost Model:** O(n) per iteration; subquadratic in ε.
**Real Wall:** No closed form; Weiszfeld iteration can fail at inputs.
**Cross-Domain Aliases:** geometric_median, fermat_weber, l1_median
**Notes:** Weiszfeld (1937); Cohen et al. (2016) O(n d log(1/ε)).

---

### CG.306: 1-Center Problem
**Definition:** Center minimizing max distance to point set (smallest enclosing ball).
**Cost Model:** O(n) expected via Welzl.
**Real Wall:** Reduces to MEB; same as CG.301.
**Cross-Domain Aliases:** one_center, minmax_facility
**Notes:** Equivalent to smallest enclosing ball.

---

### CG.307: Weber Facility Location
**Definition:** Minimize weighted sum of distances; weighted Fermat point.
**Cost Model:** O(n) per iteration of weighted Weiszfeld.
**Real Wall:** Non-smooth at input points.
**Cross-Domain Aliases:** weber_problem, fermat_weber
**Notes:** Fermat-Weber-Steiner problem; OR fundamental.

---

### CG.308: Simplicial Complex (Construction)
**Definition:** Set of simplices closed under face relation.
**Cost Model:** O(size) construction and traversal.
**Real Wall:** Memory grows rapidly in high dim or fine sampling.
**Cross-Domain Aliases:** simplicial_complex, abstract_complex
**Notes:** Foundation of computational topology; GUDHI library.

---

### CG.309: Vietoris-Rips Complex
**Definition:** Simplex when all pairs of vertices are within distance ε.
**Cost Model:** Up to O(n^{k+1}) k-simplices.
**Real Wall:** Combinatorial blowup; clique-based filtration.
**Cross-Domain Aliases:** vr_complex, rips_complex
**Notes:** Used in TDA; computationally cheaper than Čech.

---

### CG.310: Čech Complex
**Definition:** Simplex when balls of radius ε around vertices share common point.
**Cost Model:** Nerve of ε-ball cover; expensive to test.
**Real Wall:** Common-intersection check costly in high dim.
**Cross-Domain Aliases:** cech_complex, nerve_complex
**Notes:** Theoretically nicer than Rips (homotopy equivalent to union of balls).

---

### CG.311: Alpha Complex
**Definition:** Subcomplex of Delaunay where simplices have circumradius ≤ α.
**Cost Model:** O(n log n) in 2D, O(n²) in 3D via Delaunay.
**Real Wall:** Limited to ≤ d+1 in R^d.
**Cross-Domain Aliases:** alpha_complex, edelsbrunner_complex
**Notes:** Edelsbrunner & Mücke (1994); link to alpha shapes.

---

### CG.312: Persistent Homology
**Definition:** Track birth/death of topological features through filtration.
**Cost Model:** O(n^ω) matrix reduction; near-cubic in practice.
**Real Wall:** Stability under noise quantified by bottleneck distance.
**Cross-Domain Aliases:** persistent_homology, tda_persistence
**Notes:** Edelsbrunner, Letscher, Zomorodian (2002); foundation of TDA.

---

### CG.313: Mapper Algorithm
**Definition:** Reeb graph approximation via clustered cover of preimages.
**Cost Model:** O(n log n) with clustering; scale-dependent.
**Real Wall:** Parameter (lens, cover, cluster) tuning critical.
**Cross-Domain Aliases:** mapper, singh_carlsson_mapper
**Notes:** Singh, Mémoli, Carlsson (2007); core of Ayasdi TDA platform.

---

### CG.314: Reeb Graph
**Definition:** Quotient of manifold by connected components of level sets of function.
**Cost Model:** O(n log n) for surfaces; harder in higher dim.
**Real Wall:** Defined for Morse functions; needs critical points.
**Cross-Domain Aliases:** reeb_graph, level_set_graph
**Notes:** Reeb (1946); used in shape analysis.

---

## Section: GIS Algorithms and Spatial Hashing

### CG.315: Douglas-Peucker Line Simplification
**Definition:** Recursively keep point farthest from current segment until under ε.
**Cost Model:** O(n²) worst; O(n log n) with hull tricks.
**Real Wall:** Can produce self-intersections in adjacent simplifications.
**Cross-Domain Aliases:** rdp, douglas_peucker, ramer_douglas_peucker
**Notes:** Douglas & Peucker (1973); standard for polyline simplification.

---

### CG.316: Visvalingam-Whyatt Simplification
**Definition:** Iteratively remove vertex with smallest effective triangle area.
**Cost Model:** O(n log n) via heap.
**Real Wall:** Produces different visual character than Douglas-Peucker.
**Cross-Domain Aliases:** visvalingam, vw_simplify
**Notes:** Visvalingam & Whyatt (1993); better for cartography.

---

### CG.317: Morton (Z-Order) Curve
**Definition:** Map d-dim integer coords to 1-D index by bit interleaving.
**Cost Model:** O(d log U) per query for U-bit coords.
**Real Wall:** Larger "jumps" than Hilbert; worse locality.
**Cross-Domain Aliases:** morton_order, z_curve, bit_interleave
**Notes:** Morton (1966); cheapest space-filling curve to compute.

---

### CG.318: Hilbert Space-Filling Curve
**Definition:** Continuous fractal curve filling d-dim cube preserving locality.
**Cost Model:** O(d log U) per index computation.
**Real Wall:** More complex code than Morton; better neighbor coherence.
**Cross-Domain Aliases:** hilbert_curve, sfc_hilbert
**Notes:** Hilbert (1891); standard for R-tree clustering.

---

### CG.319: Geohash
**Definition:** Base-32 string encoding of lat-lon via interleaved bit subdivision.
**Cost Model:** O(precision) per encode/decode.
**Real Wall:** Boundary discontinuity (e.g., near 180° meridian).
**Cross-Domain Aliases:** geohash, niemeyer_geohash
**Notes:** Niemeyer (2008); proximity by string prefix.

---

### CG.320: H3 Hexagonal Hierarchical Grid
**Definition:** Hierarchical hexagonal global grid system on icosahedron projection.
**Cost Model:** O(1) per resolution; 16 levels of refinement.
**Real Wall:** Mixed pentagons at icosahedron vertices; not pure hex.
**Cross-Domain Aliases:** h3, uber_h3, hex_grid
**Notes:** Uber Engineering (2018); standard for ride-hailing analytics.
**Notes:** Used for path hit-testing and distance field generation.
