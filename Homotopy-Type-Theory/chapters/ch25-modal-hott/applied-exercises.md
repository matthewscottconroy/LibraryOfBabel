# Applied Exercises

The modalities of cohesive HoTT — shape $\int$, flat $\flat$, sharp $\sharp$ — encode geometric structure that has direct analogues in engineering, physics, and computing. The exercises below draw out these analogues: the flat modality as a type-safe distinction between discrete and continuous data, the shape modality as a formal tool for extracting topological invariants, synthetic differential equations as a clean formulation of continuous dynamics, and the Brouwer theorem as a convergence guarantee for iterative computation. Working these exercises requires translating the abstract modality language of Chapter 25 into the language of each domain, which is precisely the skill that turns knowledge of HoTT into research capability.

---

## Exercise B.1: Discrete and Continuous Data via the Flat Modality
*Domain: Sensor Networks / Signal Processing*

**Setup:** A temperature sensor network has two kinds of data: discrete readings (a finite sequence of measured values recorded at specific moments) and continuous signal (the actual temperature as a function of time). The distinction is important: continuous signals can be differentiated and integrated; discrete readings cannot, at least not without additional assumptions. In cohesive HoTT, the flat modality $\flat$ distinguishes these: $\flat A$ is the type of "locally constant functions of type $A$" — the type-level version of discrete data.

**Questions:**

1. Let $\mathbb{R}$ be the real numbers as a smooth type in cohesive HoTT. Explain the difference between a temperature reading $x : \mathbb{R}$ and a flat temperature reading $x : \flat \mathbb{R}$. The unit map $\flat A \to A$ says every discrete value is also a continuous value — what does this mean concretely for a sensor reading?

2. A discrete sensor record is a function $f : \{0, \ldots, n\} \to \flat \mathbb{R}$ (a sequence of flat real numbers). A continuous signal is a function $g : \mathbb{R}_{\geq 0} \to \mathbb{R}$ (a smooth function of time). In cohesive HoTT, there is a map $\flat \mathbb{R} \to \mathbb{R}$ (the counit of the $\flat$ modality). Describe what it means to "embed" the discrete sensor data into the continuous type, and why the type system distinguishes between $f$ and any continuous signal that "interpolates" $f$.

3. The flat modality $\flat$ satisfies the universal property: maps $A \to \flat B$ correspond to maps $A \to B$ that "factor through the discrete part." In sensor terms: a processing function $A \to \flat \mathbb{R}$ is one whose output is always a discrete value (independent of the continuous structure of $A$). Why is this a useful property for specifying "pure computation" (computation without continuous state)?

4. The sharp modality $\sharp$ is the right adjoint to the flat modality: $(\flat A \to B) \simeq (A \to \sharp B)$. Describe $\sharp \mathbb{R}$ intuitively — it is the type of "ways to observe $\mathbb{R}$ discretely" — and explain the adjunction in terms of the sensor scenario.

5. *Extension:* The cohesion axiom $\flat \mathbb{R} \simeq \mathbb{R}$ says that the discrete real numbers and the continuous real numbers are the same type. But $\flat : \text{Type} \to \text{Type}$ is not the identity functor: $\flat A$ and $A$ differ for most $A$. For which types $A$ does $\flat A \simeq A$ hold, and what is the topological significance of this condition?

*Abstract concept illustrated: The flat modality $\flat$ as a type-safe distinction between discrete and continuous data; the unit of $\flat$ as the inclusion of discrete values into continuous types.*

---

## Exercise B.2: Extracting Topological Invariants via the Shape Modality
*Domain: Topological Data Analysis / Persistent Homology*

**Setup:** Topological data analysis (TDA) extracts shape information from data. Given a point cloud $X \subset \mathbb{R}^n$, one computes persistent homology to find topological features (connected components, loops, voids) that persist across multiple scales. In cohesive HoTT, the shape modality $\int$ does this synthetically: for a smooth space $M$, the shape $\int M$ is the ∞-groupoid capturing the homotopy type of $M$ — its connected components, fundamental group, higher homotopy groups.

**Questions:**

1. Let $M = \mathbb{R}/\mathbb{Z}$ (the circle as a quotient of the reals by the integers). Cohesive HoTT provides the axiom $\int(\mathbb{R}/\mathbb{Z}) \simeq S^1$. Explain what this says about the topological invariants of $M$: its connected components (elements of $\pi_0$), its fundamental group ($\pi_1$), and its higher homotopy groups.

2. Consider the torus $T^2 = (\mathbb{R}/\mathbb{Z}) \times (\mathbb{R}/\mathbb{Z})$. Using the product rule for the shape modality ($\int(A \times B) \simeq \int A \times \int B$ for cohesive types — this follows from the fact that $\int$ is left adjoint and preserves colimits, plus the axioms), compute $\int T^2$. What are $\pi_0(T^2)$, $\pi_1(T^2)$, and $\pi_2(T^2)$?

3. The shape modality satisfies: $\int M$ only depends on the "soft" (topological) structure of $M$, not the "rigid" (smooth/metric) structure. In TDA terms, this is the key insight of persistent homology: topological features are invariant under deformation. Explain how the shape modality formalizes this: why does $\int M$ agree for two spaces that are homotopy equivalent but not diffeomorphic?

4. In TDA, the Vietoris-Rips complex $VR_\epsilon(X)$ is an approximation to the shape of a point cloud $X$ at scale $\epsilon$. As $\epsilon$ varies, the topological features change; "persistent" features are those present across a range of $\epsilon$. Describe how the shape modality $\int$ relates to the "limit" of $VR_\epsilon(X)$ as $\epsilon \to 0$ (or rather, as the approximation improves). What is the correct notion of convergence?

5. *Extension:* The cohesion axiom $\int \mathbb{R} \simeq \mathbf{1}$ (the real line is contractible) is key to many proofs in Chapter 25. This is the synthetic version of the classical result that $\mathbb{R}$ is contractible as a topological space. Using this axiom and induction, show that $\int \mathbb{R}^n \simeq \mathbf{1}$ for all $n$, and then show that $\int S^{n-1} \simeq S^{n-1}$ (the sphere has the homotopy type of itself). What cohesion axioms are needed for the second statement?

*Abstract concept illustrated: The shape modality $\int$ as a synthetic tool for computing homotopy invariants; $\int M$ as the "topological shadow" of a smooth space $M$.*

---

## Exercise B.3: Synthetic Differential Equations and Vector Fields
*Domain: Dynamical Systems / Control Theory*

**Setup:** In classical differential geometry, a vector field on a manifold $M$ is a section of the tangent bundle $TM$: a smooth assignment of a tangent vector to each point. The tangent space at $p \in M$ is the space of equivalence classes of curves through $p$. In cohesive HoTT, the tangent bundle is defined synthetically: $TM = M^D$ where $D = \{x : \mathbb{R} \mid x^2 = 0\}$ is the infinitesimal interval. A vector field is a section $v : M \to TM = M^D$, i.e., a function $v : M \times D \to M$ with $v(p, 0) = p$.

**Questions:**

1. Unpack the synthetic definition of a tangent vector at $p \in M$: it is a map $v : D \to M$ with $v(0) = p$. In the model where $M = \mathbb{R}$ and $D = \{x : \mathbb{R} \mid x^2 = 0\}$, what does such a map $v : D \to \mathbb{R}$ look like concretely? The Kock-Lawvere axiom says every such map is of the form $v(\epsilon) = a + b\epsilon$ for unique $a, b : \mathbb{R}$. Identify $a$ and $b$ in terms of the geometric notion of a tangent vector.

2. A vector field on $M$ is a section $v : M \to M^D$, i.e., a smooth map $v : M \times D \to M$ with $v(p, 0) = p$ for all $p$. What is the "synthetic ordinary differential equation" associated to $v$? The integral curve of $v$ through $p$ should be a map $\gamma : \mathbb{R} \to M$ with $\gamma(0) = p$ and $\dot{\gamma}(t) = v(\gamma(t))$ at each $t$. In the synthetic setting, "$\dot{\gamma}(t) = v(\gamma(t))$" translates to a condition involving the map $D \to M$ given by $\epsilon \mapsto \gamma(t + \epsilon)$.

3. The de Rham theorem in cohesive HoTT: the de Rham cohomology of $M$ (computed from differential forms) agrees with the cohomology of $\int M$ (the shape of $M$). For $M = S^1$ (which has $\int S^1 = S^1$), this says the de Rham cohomology of the circle is the singular cohomology of $S^1$: $H^0 = \mathbb{R}$, $H^1 = \mathbb{R}$, $H^k = 0$ for $k \geq 2$. Identify the differential 0-form (function) and the 1-form (angular form $d\theta$) that generate these cohomology groups in the synthetic setting.

4. In control theory, a control system is a vector field $v : X \times U \to TX$ on a state space $X$ depending on a control input $u : U$. Stability analysis asks: for which initial states $x_0$ and control inputs $u$, does the integral curve starting at $x_0$ converge to an equilibrium? In cohesive HoTT, this is a question about the topological type of the basin of attraction. What does the shape modality $\int$ say about the basin of attraction?

5. *Extension:* The Poincaré lemma states that every closed form on $\mathbb{R}^n$ is exact. In cohesive HoTT, this follows from the shape axiom $\int \mathbb{R}^n \simeq \mathbf{1}$ (the shape of $\mathbb{R}^n$ is contractible, so all its cohomology vanishes). Sketch the synthetic proof: the de Rham theorem reduces the Poincaré lemma to a statement about the cohomology of $\int \mathbb{R}^n = \mathbf{1}$, which is trivial. Where in this argument do you use the cohesion axioms?

*Abstract concept illustrated: Synthetic differential geometry as geometry from axioms; the Kock-Lawvere axiom as the foundation for calculus without limits.*

---

## Exercise B.4: The Brouwer Fixed-Point Theorem as a Convergence Guarantee
*Domain: Numerical Analysis / Fixed-Point Iteration*

**Setup:** The Brouwer fixed-point theorem states: every continuous function $f : D^n \to D^n$ from the closed unit ball to itself has a fixed point $x$ with $f(x) = x$. In numerical analysis, many iterative algorithms can be shown to converge by modeling them as continuous maps on a compact convex set and applying Brouwer (or its generalizations). Chapter 25 proves this theorem synthetically in cohesive HoTT using the shape modality.

**Questions:**

1. The synthetic Brouwer proof proceeds as follows: suppose $f : D^2 \to D^2$ has no fixed point. Then the "retraction map" $r : D^2 \to S^1$ (defined by extending the line from $f(x)$ through $x$ to the boundary $S^1$) would be a continuous map with $r|_{S^1} = \mathsf{id}_{S^1}$ — a retraction of $D^2$ onto $S^1$. But $\int D^2 \simeq \mathbf{1}$ (contractible) while $\int S^1 \simeq S^1$ (non-trivial), and a retraction would require $S^1$ to be a retract of $\mathbf{1}$ (impossible). Identify where the shape modality $\int$ enters this argument and why it gives the contradiction.

2. The Newton-Raphson method for finding a root of $g : \mathbb{R} \to \mathbb{R}$ can be formulated as a fixed-point iteration: find $x$ with $T(x) = x$ where $T(x) = x - g(x)/g'(x)$. Brouwer's theorem guarantees a fixed point if $T : D \to D$ for some compact convex $D$. In practice, one shows convergence by proving $T$ is a contraction (the Banach fixed-point theorem, a stronger result). What is the relationship between Brouwer's theorem (existence without uniqueness) and the Banach theorem (existence with uniqueness)? Which corresponds to the Segal condition (existence) and which to the Rezk condition (uniqueness)?

3. The Brouwer theorem is classically proved using singular homology or degree theory. The cohesive HoTT proof is synthetic: no triangulations, no homology groups, just modalities and the non-contractibility of $S^1$ (which follows from $\pi_1(S^1) = \mathbb{Z}$). Compare the two proof strategies: what is the key non-trivial topological fact in each? In the cohesive proof, where is the fact that the ball $D^2$ is "filled in" (contractible) used, and where is the fact that the circle $S^1$ is "not filled in" (non-contractible) used?

4. *Extension:* The Kakutani fixed-point theorem generalizes Brouwer to set-valued maps (correspondences): every upper-hemicontinuous correspondence $F : D^n \rightrightarrows D^n$ with non-empty convex values has a fixed point. This is the foundation of Nash equilibrium existence in game theory. Does the cohesive HoTT proof of Brouwer generalize to Kakutani? What additional cohesive structure would you need to formalize set-valued maps synthetically?

*Abstract concept illustrated: The shape modality as the topological invariant that makes the Brouwer proof work; cohesive HoTT as a setting where non-trivial analysis can be done from axioms.*

---

## Exercise B.5: Gauge Fields as Types — Chern-Simons Theory
*Domain: Theoretical Physics / Quantum Field Theory*

**Setup:** In classical differential geometry, a $U(1)$ gauge field (electromagnetic potential) on a 3-manifold $M$ is a connection 1-form $A \in \Omega^1(M; \mathfrak{u}(1))$. The Chern-Simons action is $\mathsf{CS}[A] = \int_M A \wedge dA$. Two gauge fields $A$ and $A' = A + d\lambda$ related by a gauge transformation are physically equivalent. In cohesive HoTT, the moduli space of gauge fields is not a set but a type: the type $BU(1)_\nabla$ of $U(1)$-bundles with connection, which is an ∞-groupoid where the paths are gauge transformations.

**Questions:**

1. In cohesive HoTT, a $U(1)$-principal bundle on $M$ is a map $M \to BU(1)$ where $BU(1) = K(\mathbb{Z}, 2)$ is the classifying type (the Eilenberg-MacLane type). A gauge field (connection) is a lift to $BU(1)_\nabla$ (the "moduli stack of $U(1)$-bundles with connection"). Explain why the flat modality $\flat$ distinguishes a flat connection (one with zero curvature) from a general connection: a flat connection is a map $M \to \flat BU(1)$, while a general connection is a map $M \to BU(1)_\nabla$.

2. A gauge transformation is a path in the type $[M, BU(1)_\nabla]$ of connections — two connections are in the same gauge orbit iff they are connected by a path. In cohesive HoTT, this means two gauge fields $A$ and $A'$ are gauge-equivalent iff they are *equal as types* (homotopic as maps $M \to BU(1)_\nabla$). Explain why "gauge equivalence = path in the type" is the correct synthetic statement, and why this is better than the classical formulation (which requires manually quotienting by gauge transformations).

3. The Chern-Simons level is the cohomology class $[A] \in H^1(M; U(1)) = [M, BU(1)]$. In cohesive HoTT, this is the element of $\pi_0([M, BU(1)])$. For $M = S^3$ (the 3-sphere), compute $[S^3, BU(1)] = [S^3, K(\mathbb{Z}, 2)]$. What does this say about the possible Chern-Simons levels on $S^3$?

4. *Extension:* Higher gauge theory involves 2-bundles with structure 2-group. In cohesive HoTT, a 2-bundle is a map $M \to B^2 U(1) = K(\mathbb{Z}, 3)$. The string group is a 2-group whose classifying space is $B^3 U(1) = K(\mathbb{Z}, 4)$ (shifted by one). These higher gauge fields appear in string theory (the B-field) and M-theory (the C-field). Describe what $\flat K(\mathbb{Z}, n)$ is for each $n$ and explain why the flat modality distinguishes flat from non-flat higher gauge fields.

*Abstract concept illustrated: Principal bundles as maps to classifying types; gauge equivalence as path-equality; the flat modality distinguishing curvature.*
