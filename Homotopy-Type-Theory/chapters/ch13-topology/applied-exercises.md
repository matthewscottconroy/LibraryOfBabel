# Applied Exercises

Point-set topology is often taught as an abstract framework divorced from concrete applications. But the core ideas — open sets, continuity, quotient spaces, connectedness, compactness, the fundamental group — arise naturally whenever you need to reason rigorously about the "shape" of a configuration space, a data set, or a physical system. The exercises below situate the topological concepts of Chapter 13 in robotic planning, data analysis, network design, computer graphics, and sensor coverage, with the goal of showing that topology is not merely a prerequisite for algebraic topology but a practical language for reasoning about space.

---

## Exercise B.1: Configuration Spaces in Robotics
*Domain: Robotics / Motion Planning*

**Setup:** A robotic arm with two joints, each rotating freely in the plane, has a **configuration space** (C-space) that encodes all possible states. If the first joint can rotate through angle $\theta_1 \in [0, 2\pi)$ and the second through $\theta_2 \in [0, 2\pi)$, the C-space is the set of pairs $(\theta_1, \theta_2)$. Because the angles wrap around, each $[0, 2\pi)$ with endpoints identified is homeomorphic to the circle $S^1$, and the full C-space is $S^1 \times S^1$ — the **torus** $T^2$.

**Questions:**
1. Verify that $S^1$ is homeomorphic to $[0,1]$ with $0$ and $1$ identified (the quotient $[0,1]/\{0\sim 1\}$). Write down the quotient map $q : [0,1] \to S^1$ and verify that it is continuous, surjective, and that the quotient topology on $S^1$ coincides with the standard (metric) topology on the unit circle in $\mathbb{R}^2$.
2. The torus $T^2 = S^1 \times S^1$ can be represented as the quotient of the square $[0,1]^2$ with identifications $(x,0) \sim (x,1)$ and $(0,y) \sim (1,y)$. Draw the identification diagram and write out the equivalence classes of the points $(0,0)$, $(1/2, 0)$, and $(0, 1/2)$. How many distinct points in $T^2$ do the four corners of the square correspond to?
3. Motion planning for the two-joint arm requires finding a path from one configuration $(\theta_1^0, \theta_2^0)$ to another $(\theta_1^1, \theta_2^1)$ in $T^2$, avoiding obstacles. An obstacle region is a closed subset $O \subset T^2$. The arm can move between two configurations without obstacle if and only if they are in the same connected component of $T^2 \setminus O$. Given a single obstacle $O = \{(\theta_1, \theta_2) \mid \theta_1 \in (\pi/4, 3\pi/4)\} \times \{(\theta_1, \theta_2) \mid \theta_2 \in (0, \pi/2)\}$ (a rectangular region in angle-space), determine whether $T^2 \setminus O$ is path-connected. Does the fundamental group $\pi_1(T^2 \setminus O)$ change depending on the shape of the obstacle?

*Abstract concept illustrated: Quotient spaces and the quotient topology; the torus as a fundamental example; path-connectedness and connected components as the topological obstruction to motion planning.*

---

## Exercise B.2: Topological Data Analysis and Persistent Homology
*Domain: Data Analysis / Machine Learning*

**Setup:** Given a finite point cloud $X = \{x_1, \ldots, x_n\} \subset \mathbb{R}^d$, we want to find topological features of the "shape" of the data — clusters, loops, voids — without committing to a particular scale. The approach: for each $\varepsilon > 0$, build the **Vietoris-Rips complex** $VR_\varepsilon(X)$ by adding a $k$-simplex $[x_{i_0}, \ldots, x_{i_k}]$ whenever all pairwise distances $d(x_{i_a}, x_{i_b}) \leq \varepsilon$. As $\varepsilon$ increases, the complex grows: connected components merge, loops fill in, and the topology simplifies. **Persistent homology** tracks how topological features are born and die across $\varepsilon$.

**Questions:**
1. Consider the point cloud $X = \{(0,0), (1,0), (0,1), (1,1)\}$ (four corners of a square). Compute $VR_\varepsilon(X)$ for $\varepsilon = 0.9$, $\varepsilon = 1.1$, and $\varepsilon = 1.6$ (approximate values chosen so that $\sqrt{2} \approx 1.41$). At each scale, list the 0-simplices, 1-simplices, and any 2-simplices. Describe the connectivity: how many connected components are there at each scale?
2. At $\varepsilon = 1.1$, the Vietoris-Rips complex has a 1-cycle (a loop). Identify it. At $\varepsilon = 1.6$, this loop "dies" because the full square $[x_1, x_2, x_3, x_4]$ is not a simplex in $VR_{1.6}$ (since the diagonal has length $\sqrt{2} < 1.6$, but we need all pairs $\leq \varepsilon$ and there are diagonals). Reconsider: at what exact value of $\varepsilon$ does the 1-cycle die? The **persistence** of this feature is $\varepsilon_\text{death} - \varepsilon_\text{birth}$.
3. The connection to Chapter 13's topology: the Vietoris-Rips complex $VR_\varepsilon(X)$ is a topological space (a simplicial complex with the standard topology). The **nerve theorem** says that if $\varepsilon$ is chosen correctly relative to the geometry of $X$, $VR_\varepsilon(X)$ is homotopy equivalent to the "true" underlying space from which $X$ was sampled. Suppose $X$ was sampled from an annulus (a disk with a hole). At what scale $\varepsilon$ would you expect a persistent $H_1$ feature (a loop that "lives for a long time")? Why do short-lived features correspond to noise while long-lived features correspond to true topology?

*Abstract concept illustrated: Simplicial complexes as topological spaces; the Nerve theorem; connected components as $\pi_0$; loops as generators of $\pi_1$ and $H_1$; continuity and homeomorphism as the topology-preserving maps.*

---

## Exercise B.3: Network Topology and Graph Connectivity
*Domain: Computer Networks / Distributed Systems*

**Setup:** A computer network can be modeled as a graph $G = (V, E)$, where vertices $V$ are computers and edges $E$ are communication links. A graph is naturally a topological space: give each edge $e \cong [0,1]$ the standard topology, and glue the endpoints together according to the graph structure. The resulting **geometric realization** $|G|$ is a topological space. Two graphs are homeomorphic (as topological spaces) if and only if they have the same number of vertices of degree $\neq 2$ and the same number of edges in a cycle basis.

**Questions:**
1. The geometric realization of the complete graph $K_4$ (four vertices, all six edges present) is a topological space. Compute the **Euler characteristic** $\chi(K_4) = V - E + F$ for the CW complex structure: $V = 4$ vertices (0-cells), $E = 6$ edges (1-cells), $F = 0$ faces (2-cells). Compare this to the Euler characteristic of $S^1$ (the circle). How does $\chi$ detect the topology: what does $\chi = V - E$ tell you about the number of independent cycles in the graph?
2. A graph is **connected** (as a topological space) if and only if $|G|$ is path-connected. Prove directly from the topological definition: if $G$ has two connected components $G_1$ and $G_2$, then $|G| = |G_1| \sqcup |G_2|$ (disjoint union), and this is disconnected as a topological space (both $|G_1|$ and $|G_2|$ are simultaneously open and closed in $|G|$).
3. The **fundamental group** $\pi_1(|G|)$ of a connected graph has a clean description: if $G$ has $n$ vertices and $m$ edges, then $\pi_1(|G|) \cong F_{m-n+1}$ (the free group on $m - n + 1$ generators). Here $m - n + 1 = 1 - \chi(G)$ is the cycle rank (first Betti number) of the graph. For the graph of the internet backbone (millions of nodes, approximately tree-like), what does this say about $\pi_1$ of the network? For a graph with a single cycle (a ring network), verify that $\pi_1 \cong \mathbb{Z}$. What does a path in $\pi_1$ represent in terms of network routing?

*Abstract concept illustrated: Geometric realization of a CW complex; connectedness and path-connectedness; the fundamental group $\pi_1$ of a graph; Euler characteristic as a topological invariant.*

---

## Exercise B.4: Surface Topology in Computer Graphics
*Domain: Computer Graphics / Computational Geometry*

**Setup:** In computer graphics, 3D surfaces are represented as **polygon meshes**: collections of vertices, edges, and faces (typically triangles) forming a 2-dimensional surface in $\mathbb{R}^3$. The **genus** of a surface (the number of handles, or "holes through the surface") is a topological invariant that determines, among other things: whether the surface can be UV-unwrapped without cuts, how many independent texture seams are needed, and the structure of the medial axis of the surface.

The **Euler characteristic** $\chi = V - E + F$ of a closed connected orientable surface satisfies $\chi = 2 - 2g$, where $g$ is the genus. The sphere has $g = 0$, $\chi = 2$; the torus has $g = 1$, $\chi = 0$; a donut with two holes has $g = 2$, $\chi = -2$.

**Questions:**
1. A mesh artist creates a model of a coffee mug (homeomorphic to a torus, $g = 1$). The mesh has 500 vertices and 1000 edges. How many faces (triangles) must the mesh have? Now the artist adds a second handle, making the surface homeomorphic to a genus-2 surface. If they add 50 vertices and 200 edges to create the second handle, how many new faces must they add to maintain a valid (manifold) triangulation?
2. For UV-unwrapping (mapping a 3D surface to a 2D texture), a genus-$g$ surface requires at least $2g$ cuts (seams) to be unfolded into a planar region without overlaps. Explain why this is true using the theory of the fundamental group: $\pi_1(T^2) = \mathbb{Z} \times \mathbb{Z}$ (requiring 2 generators, hence 2 cuts), and $\pi_1(\Sigma_g) = \langle a_1, b_1, \ldots, a_g, b_g \mid [a_1, b_1] \cdots [a_g, b_g] = 1 \rangle$ (requiring $2g$ generators). What does "cutting along a loop" do to the fundamental group?
3. The **genus computation algorithm**: given a mesh, compute $V$, $E$, $F$, use $\chi = V - E + F$, and then $g = (2 - \chi)/2$. Suppose the mesh is not a closed surface but a surface **with boundary** (like a disc with holes). The formula changes: for a compact orientable surface with boundary having $b$ boundary components, $\chi = 2 - 2g - b$. A mesh represents a pair of pants (sphere with 3 holes), with 300 vertices, 600 edges, and 300 faces. Compute $\chi$, $g$, and $b$, and verify they are consistent.

*Abstract concept illustrated: The classification of compact surfaces; the Euler characteristic as a topological invariant; the fundamental group of surfaces; quotient spaces and CW-complex structure.*

---

## Exercise B.5: Sensor Coverage as a Topological Problem
*Domain: Sensor Networks / Applied Topology*

**Setup:** A team of robots with limited-range sensors is deployed in a region $R \subset \mathbb{R}^2$ to monitor it. Each robot $i$ covers a "sensing disk" $B(x_i, r)$ (an open ball of radius $r$ around its location $x_i$). The **coverage question** asks: is every point of $R$ within sensing range of at least one robot — that is, is $R \subseteq \bigcup_i B(x_i, r)$?

Classical approaches solve this using coordinate geometry (check every point) or by solving an optimization problem. The topological approach of de Silva and Ghrist (2007) answers the question using the **Čech complex**: a simplicial complex built from the intersection pattern of the sensing disks, without knowing the coordinates of the robots (only which pairs of robots can communicate, i.e., are within distance $2r$ of each other).

**Questions:**
1. Define the Čech complex $\check{C}_r(\{x_1, \ldots, x_n\})$: a set $\{x_{i_0}, \ldots, x_{i_k}\}$ is a $k$-simplex if and only if $\bigcap_{j=0}^k B(x_{i_j}, r) \neq \emptyset$ (the sensing disks all have a common intersection). For four robots at positions $(0,0)$, $(1,0)$, $(0,1)$, $(1,1)$ with $r = 0.75$, list all the simplices of $\check{C}_r$. Does the Čech complex have a 2-simplex (a filled triangle)?
2. The **nerve theorem** says: if the sensing region $R$ is a convex subset of $\mathbb{R}^2$ and the sensing disks are convex and have the property that any intersection of disks is contractible (which is true for balls), then $\check{C}_r$ is homotopy equivalent to $\bigcup_i B(x_i, r)$. The topological coverage condition then becomes: $\check{C}_r$ is homotopy equivalent to $R$. For the square region $R = [0,1]^2$ and the four robots above, the coverage is complete if the union of disks covers $R$. Using the nerve theorem, what topological property of $\check{C}_r$ would confirm complete coverage without computing the union directly?
3. Suppose two robots fail and are removed from the network. How does the topology of the Čech complex change? Define "coverage gap" topologically: it is a point $p \in R$ not covered by any disk, contributing a nontrivial connected component or a "hole" in the union of disks. Explain why the presence of a 1-cycle in $\check{C}_r$ that bounds no 2-chain (i.e., a generator of $H_1(\check{C}_r) \neq 0$) detects a coverage gap. Why might an engineer prefer this homological criterion over direct coordinate checking?

*Abstract concept illustrated: The nerve of an open cover; the nerve theorem (homotopy equivalence between nerve and union); homology as a tool for detecting topological features; coverage as a problem about surjectivity of a continuous map.*
