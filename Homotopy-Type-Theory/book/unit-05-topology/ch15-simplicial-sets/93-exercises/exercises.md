# Exercises: Simplicial Sets

---

**Exercise 15.1 (The Simplex Category).** Compute explicitly.

(a) List all morphisms $f : [2] \to [2]$ in $\Delta$ (order-preserving maps from $\{0,1,2\}$ to $\{0,1,2\}$). How many are there?

(b) List all morphisms $f : [1] \to [2]$. Identify which are face maps $d^0, d^1, d^2$ and which are not face maps.

(c) List all morphisms $f : [2] \to [1]$. Identify which are degeneracy maps $s^0, s^1$ and which are not.

(d) Is the composition $d^1 \circ d^0 : [0] \to [2]$ the same as $d^0 \circ d^0 : [0] \to [2]$ or $d^0 \circ d^1$? Compute and verify the face-face identity.

---

**Exercise 15.2 (Simplicial Identities).** Verify the following simplicial identities by explicit computation.

(a) $d^1 \circ d^0 = d^0 \circ d^0 : [0] \to [2]$. (Both skip vertices $0$ and $1$? Compute both sides.)

Wait — the face-face identity says $d^j \circ d^i = d^i \circ d^{j-1}$ for $i < j$. For $i=0, j=1$: $d^1 \circ d^0 = d^0 \circ d^0$. Compute both maps $[0] \to [2]$ and verify they are equal.

(b) Verify $s^0 \circ d^0 = \mathsf{id}_{[0]} : [0] \to [0]$ (the face-degeneracy identity with $i=j=0$).

(c) Verify $s^0 \circ d^1 = \mathsf{id}_{[0]} : [0] \to [0]$ (the face-degeneracy identity with $i=j+1=1, j=0$).

(d) Verify $d^0 \circ s^0 = s^0 \circ d^0 : [1] \to [1]$ (wait, this should follow from $i < j$ case with $i=0, j=1$: $s^1 \circ d^0 = d^0 \circ s^0$). Compute.

---

**Exercise 15.3 (Standard Simplex).** Describe the standard simplices.

(a) Describe $\Delta[0]_n$ for all $n \geq 0$. (What are the $n$-simplices of the standard 0-simplex?)

(b) Describe $\Delta[1]_n$ for $n = 0, 1, 2$. List all elements and identify which are degenerate.

(c) Describe $\Delta[2]_0, \Delta[2]_1, \Delta[2]_2$ explicitly. How many non-degenerate simplices are there in each degree?

(d) Show that $\text{Hom}_{\mathbf{sSet}}(\Delta[n], X) \cong X_n$ for any simplicial set $X$ (the Yoneda lemma for simplicial sets).

---

**Exercise 15.4 (Boundaries and Horns).** 

(a) Describe $\partial\Delta[2]$: what are its $0$-, $1$-, and $2$-simplices? Is it the simplicial set corresponding to the boundary of a triangle (three vertices, three edges, no interior)?

(b) Describe $\Lambda^2_1$ (the inner 2-horn): which $1$-simplices does it have? Which $1$-simplex is missing compared to $\partial\Delta[2]$?

(c) Describe $\Lambda^3_1$ (an inner 3-horn): it has all four faces of $\Delta[3]$ except the $1$st face $d^1 : [2] \to [3]$. What does the missing face correspond to geometrically?

(d) What is $|\Lambda^2_0|$ (the geometric realization of the outer 2-horn)? Is it contractible?

---

**Exercise 15.5 (A Simplicial Set from a Category).** The nerve $N(\mathcal{C})$ of a small category $\mathcal{C}$ is a simplicial set with $N(\mathcal{C})_n$ = composable sequences of $n$ morphisms.

(a) Compute the nerve of the category $\mathbf{2} = \{0 \xrightarrow{f} 1\}$ (one morphism and two identity morphisms). Describe $N(\mathbf{2})_0, N(\mathbf{2})_1, N(\mathbf{2})_2$.

(b) Show that $|N(\mathbf{2})| \cong [0,1]$ (the geometric realization of the nerve of the category $0 \to 1$ is the interval).

(c) Compute the nerve of the group $\mathbb{Z}/2\mathbb{Z}$ viewed as a category with one object. What are the $n$-simplices? What is $|N(\mathbb{Z}/2\mathbb{Z})|$?

(d) Is $N(\mathcal{C})$ always a Kan complex? (No: only when $\mathcal{C}$ is a groupoid — every morphism is invertible. Verify this for the category $\mathbf{2}$.)

---

**Exercise 15.6 (Geometric Realization).** Compute geometric realizations.

(a) Describe $|\Delta[1]|$ (the geometric realization of the standard 1-simplex). It should be homeomorphic to $[0,1]$.

(b) Describe $|\partial\Delta[2]|$ (the boundary of the standard 2-simplex). It should be homeomorphic to $S^1$.

(c) Describe $|\Delta[2]|$ (the standard 2-simplex). It should be homeomorphic to $D^2$ (the closed disk).

(d) The simplicial set $X$ with $X_0 = \{v\}$, $X_1 = \{e, s_0 v\}$ (where $d_0 e = d_1 e = v$), and all higher simplices degenerate. Show $|X| \cong S^1$.

---

**Exercise 15.7 (Kan Condition: Small Cases).** For each of the following simplicial sets, determine whether it satisfies the Kan condition.

(a) The simplicial set with $X_0 = \{0, 1\}$, $X_1 = \{e_{01}\}$ (one edge from $0$ to $1$), and degenerate simplices. Is $X$ a Kan complex? (A horn $\Lambda^1_0$ specifies the target vertex $1$; a filling asks for an edge to $1$ from some vertex. Does such an edge exist?)

(b) The nerve of the discrete category $\{0, 1\}$ (no non-identity morphisms). Is this a Kan complex?

(c) The simplicial set $\text{Sing}(S^1)$ (the singular complex of the circle). Is this a Kan complex?

---

**Exercise 15.8 (Horn Filling and Composition).** Show that the inner horn condition $\Lambda^2_1 \to X$ implies that any two composable 1-simplices have a (possibly non-unique) composition.

(a) Given 1-simplices $f : a \to b$ and $g : b \to c$ in $X$ (meaning $d_1 f = a$, $d_0 f = b$, $d_1 g = b$, $d_0 g = c$), describe the corresponding map $\Lambda^2_1 \to X$.

(b) A filling $\Delta[2] \to X$ of this horn provides a 2-simplex $h$ with $d_0 h = g$, $d_2 h = f$, and a third edge $d_1 h : a \to c$ which we call the composition $g \circ f$.

(c) Is the composition unique? (No, in general: there may be multiple fillings of the horn, giving different compositions. But the different compositions are homotopic — by the uniqueness-up-to-homotopy of fillings in a Kan complex.)

---

**Exercise 15.9 (The Dold-Kan Correspondence).** The Dold-Kan correspondence says: simplicial abelian groups are equivalent to chain complexes of abelian groups (in non-negative degrees).

(a) Given a simplicial abelian group $A$, define the normalized chain complex $NA$ where $(NA)_n$ = the intersection of kernels of all face maps except $d_0$: $NA_n = \ker(d_1) \cap \ker(d_2) \cap \cdots \cap \ker(d_n) \subseteq A_n$.

(b) The differential $\partial : NA_n \to NA_{n-1}$ is $d_0|_{NA_n}$. Show $\partial^2 = 0$.

(c) Compute $NA$ for the simplicial abelian group $A = \mathbb{Z}[\Delta[1]]$ (the free abelian group on the simplices of $\Delta[1]$). What chain complex do you get?

---

**Exercise 15.10 (Kan Fibrations).** Show the following maps are or are not Kan fibrations.

(a) The projection $\Delta[1] \times X \to \Delta[1]$ for any Kan complex $X$. (It is a Kan fibration: the fibers are copies of $X$, and the fibration is trivial.)

(b) The inclusion $\partial\Delta[n] \hookrightarrow \Delta[n]$. Is this a Kan fibration? (No: there is a horn $\Lambda^n_k \to \partial\Delta[n]$ that doesn't extend in $\partial\Delta[n]$ — the filling would need the interior $n$-simplex.)

(c) The map $\text{Sing}(f) : \text{Sing}(E) \to \text{Sing}(B)$ for a Serre fibration $f : E \to B$. Show this is a Kan fibration.

---

**Exercise 15.11 (Model Structure: Basic Properties).** In the Quillen model structure on $\mathbf{sSet}$:

(a) Show that every simplicial set is cofibrant. (The initial simplicial set is $\emptyset$, and the map $\emptyset \to X$ is injective on all $X_n$ since $\emptyset$ has no simplices.)

(b) Show that the terminal simplicial set $*$ (with exactly one $n$-simplex in each degree) is fibrant. (The map $* \to *$ is a Kan fibration: any horn into $*$ extends uniquely.)

(c) Show that a Kan fibration between Kan complexes is a Kan fibration in the model-category sense (i.e., it has the right lifting property against all acyclic cofibrations).

---

**Exercise 15.12 (Weak Equivalences).** A map $f : X \to Y$ of simplicial sets is a weak equivalence if $|f| : |X| \to |Y|$ is a weak homotopy equivalence.

(a) Show that every Kan complex is weakly equivalent to $\text{Sing}(|X|)$ via the unit of the adjunction $X \to \text{Sing}(|X|)$.

(b) Show that the inclusion $\partial\Delta[n] \hookrightarrow \Delta[n]$ is not a weak equivalence for $n \geq 1$. (The geometric realizations are $S^{n-1}$ and $D^n$, which are not weakly equivalent for $n \geq 2$.)

(c) Show that the inclusion of any Kan complex into its geometric realization's singular complex, $X \to \text{Sing}(|X|)$, is a weak equivalence. (This requires that the unit of the adjunction is a weak equivalence for fibrant objects — a key property of the Quillen equivalence.)

---

**Exercise 15.13 (Homotopy Groups of Kan Complexes).** Let $X$ be a Kan complex with vertex $v \in X_0$.

(a) Define $\pi_0(X)$ (the set of path-components) as the quotient of $X_0$ by the relation $a \sim b$ iff there exists $e \in X_1$ with $d_0 e = a$ and $d_1 e = b$. Show this is an equivalence relation.

(b) Define $\pi_1(X, v)$ using 1-simplices with both faces at $v$ and the inner horn composition. Verify the group axioms.

(c) Show $\pi_n(X, v) \cong \pi_n(|X|, v)$ for the case $n = 1$ and $X = \text{Sing}(S^1)$.

---

**Exercise 15.14 (The Voevodsky Model: Identity Types).** In the simplicial set model:

(a) Describe the identity type $a =_A b$ for $A = |\Delta[1]|$ (the interval) and $a = 0$, $b = 1$. What Kan complex does this correspond to?

(b) Describe the identity type $a =_A a$ for $A = S^1$ (the simplicial circle) and $a = \text{base}$. What Kan complex is $\text{base} =_{S^1} \text{base}$? (Answer: it is a Kan complex equivalent to $\Omega S^1 \simeq \mathbb{Z}$.)

(c) What does it mean for $a =_A a =_A a$ to have non-trivial elements (paths between paths at $a$)? Give a type $A$ and a point $a$ where this happens.

---

**Exercise 15.15 (Univalence in the Simplicial Model).** 

(a) Describe a path from $A = \text{Bool}$ to $B = \text{Bool}$ in the universe $\mathcal{U}$. By univalence, such a path corresponds to an equivalence $\text{Bool} \simeq \text{Bool}$. List all such equivalences.

(b) The path space $\mathcal{U}(\text{Bool}, \text{Bool})$ is equivalent to the type of equivalences $\text{Bool} \simeq \text{Bool}$, which has two elements (identity and swap). What is the fundamental group $\pi_1(\mathcal{U}, \text{Bool})$?

(c) For $A = \mathbb{N}$, the type of equivalences $\mathbb{N} \simeq \mathbb{N}$ is the type of bijections $\mathbb{N} \to \mathbb{N}$. This is the symmetric group $S_\infty = \text{colim} S_n$. What is $\pi_1(\mathcal{U}, \mathbb{N})$?

---

**Exercise 15.16 (Simplicial Circle).** Construct the simplicial circle explicitly.

(a) Define a simplicial set $S^1$ with $S^1_0 = \{v\}$ (one vertex), $S^1_1 = \{e, s_0 v\}$ (one non-degenerate edge $e$ and the degenerate edge at $v$), and all higher $S^1_n$ generated by degeneracy maps. Describe $S^1_2$ explicitly.

(b) Show $|S^1| \cong S^1$ (the topological circle). (The geometric realization has one vertex and one 1-cell, giving $S^1$.)

(c) Is $S^1$ (this simplicial set) a Kan complex? (It is not — the outer horns $\Lambda^1_0$ and $\Lambda^1_1$ do not fill. Compute the Kan completion of $S^1$ and show it has the same geometric realization.)

---

**Exercise 15.17 (Quasi-categories).** A quasi-category is a simplicial set where all inner horns fill.

(a) Show that every Kan complex is a quasi-category.

(b) Show that $N(\mathcal{C})$ (the nerve of an ordinary category) is a quasi-category. (The inner horn condition for nerves of categories follows from the associativity of composition.)

(c) Is $N(\mathcal{C})$ a Kan complex? (Only if $\mathcal{C}$ is a groupoid. Show that if $\mathcal{C}$ is not a groupoid — has a non-invertible morphism — then some outer horn of $N(\mathcal{C})$ does not fill.)

---

**Exercise 15.18 (The Singular Complex of $\mathbb{R}$).** Describe $\text{Sing}(\mathbb{R})$.

(a) What are the $0$-simplices of $\text{Sing}(\mathbb{R})$? (Continuous maps $|\Delta^0| = \{*\} \to \mathbb{R}$, i.e., points of $\mathbb{R}$.)

(b) What are the $1$-simplices? (Continuous maps $[0,1] \to \mathbb{R}$, i.e., paths in $\mathbb{R}$.)

(c) Is $\text{Sing}(\mathbb{R})$ a Kan complex? What are its homotopy groups? (Since $\mathbb{R}$ is contractible, $\pi_n(\text{Sing}(\mathbb{R})) = 0$ for all $n$.)

(d) How does $\text{Sing}(\mathbb{R})$ compare to $\text{Sing}(\{*\})$ (the simplicial set for a point)? Are they weakly equivalent?

---

**Exercise 15.19 (Cubical Sets: A Comparison).** Cubical sets are functors from the cube category $\square$ to $\mathbf{Set}$, where the objects of $\square$ are the standard cubes $[0,1]^n$.

(a) What are the face maps of $\square$ (in dimension 1)? (There are two face maps $\delta^0, \delta^1 : [0] \to [1]$ corresponding to the two endpoints of $[0,1]$.)

(b) Describe a "cubical complex" for $S^1$ (the circle as a cubical set). Compare with the simplicial description from Exercise 15.16.

(c) What is the advantage of cubical sets over simplicial sets for computation in HoTT? (The interval $[0,1]$ in cubical type theory has an abstract "connection" structure that makes the path types more computable.)

---

**Exercise 15.20 (Research: The Voevodsky Program).** 

(a) Voevodsky proved the Milnor conjecture and the Bloch-Kato conjecture using motivic cohomology. Look up a brief description of what these conjectures say. Why do they require homotopy theory for algebraic varieties?

(b) The Univalent Foundations program was Voevodsky's response to finding an error in a published proof. Look up what the error was and why it was not caught earlier. What does this suggest about the reliability of mathematical proofs?

(c) The Brunerie number $n$ was the unknown in the statement $\pi_4(S^3) = \mathbb{Z}/n\mathbb{Z}$ proved in HoTT. It was computed to be $n = 2$ using a computer. Look up how this computation was done. Does this count as a "proof"?

---

**Exercise 15.21 (Simplicial Abelian Groups and Chain Complexes).** The Dold-Kan correspondence.

(a) Given a chain complex $C = (C_n, d_n)$, describe the simplicial abelian group $K(C)$ such that $NK(C) \cong C$ (the inverse of the normalized chain complex functor).

(b) Compute $K(\mathbb{Z}[0]) = K(\cdots \to 0 \to \mathbb{Z} \to 0)$ (the chain complex with $\mathbb{Z}$ in degree $0$). What simplicial abelian group do you get?

(c) Compute $K(\mathbb{Z}[n])$ (the chain complex with $\mathbb{Z}$ in degree $n$). The geometric realization $|K(\mathbb{Z}[n])| \cong K(\mathbb{Z}, n)$ (the Eilenberg-MacLane space). Verify this for $n = 1$.

---

**Exercise 15.22 (Model Category Lifting).** In the Quillen model structure on $\mathbf{sSet}$:

(a) Show that the horn inclusions $\Lambda^n_k \hookrightarrow \Delta[n]$ are acyclic cofibrations (cofibrations that are also weak equivalences).

(b) Show that Kan fibrations have the right lifting property against all acyclic cofibrations. (This is part of the model structure axioms; verify for the specific case of the 2-horn $\Lambda^2_0 \hookrightarrow \Delta[2]$.)

(c) Show that every simplicial map factors as a monomorphism followed by a Kan fibration that is also a weak equivalence (the "acyclic fibration" part of the factorization axiom). Hint: use the small object argument with the boundary inclusions $\partial\Delta[n] \hookrightarrow \Delta[n]$.

---

**Exercise 15.23 (Path Objects).** In the model structure on $\mathbf{sSet}$, a path object for $X$ is a factorization of the diagonal $X \to X \times X$ as $X \xrightarrow{\sim} PX \to X \times X$.

(a) For a Kan complex $X$, describe the path object $PX$ as the Kan complex with $PX_n = X_{n+1}$ (the simplicial set "shifted by one").

(b) Show that the map $X \to PX$ (the "constant path" map, induced by $s_0$) is a weak equivalence.

(c) Show that the map $PX \to X \times X$ (sending each path to its endpoints) is a Kan fibration.

(d) How does this relate to the identity type $a =_A b$ in HoTT? (The fiber of $PX \to X \times X$ over the point $(a,b)$ is the path space $\text{Path}(X, a, b)$, corresponding to the identity type.)

---

**Exercise 15.24 (Bisimplicial Sets).** A bisimplicial set is a functor $\Delta^{op} \times \Delta^{op} \to \mathbf{Set}$.

(a) Give an example of a bisimplicial set. (Hint: given two simplicial sets $X$ and $Y$, their "external product" is a bisimplicial set with $(X \boxtimes Y)_{m,n} = X_m \times Y_n$.)

(b) The diagonal of a bisimplicial set $Z$ is the simplicial set $\text{diag}(Z)$ with $\text{diag}(Z)_n = Z_{n,n}$. Compute $\text{diag}(\Delta[1] \boxtimes \Delta[1])$.

(c) The Eilenberg-Zilber theorem says $|\text{diag}(Z)| \simeq |\text{Tot}(Z)|$ where $\text{Tot}$ is the "total space" construction. For the product $X \times Y$ of two simplicial sets, use this to show $|X \times Y| \simeq |X| \times |Y|$.

---

**Exercise 15.25 (HoTT Computations in the Model).** Carry out type-theoretic computations in the simplicial set model.

(a) Verify that $\text{refl}_v : v =_{S^1} v$ corresponds to the degenerate 1-simplex $s_0 v \in \text{Sing}(S^1)_1$ (the degenerate edge at the basepoint).

(b) The loop $\text{loop} : \text{base} =_{S^1} \text{base}$ in the circle HIT corresponds to a non-degenerate 1-simplex in the simplicial circle. Describe how this simplex represents a non-trivial element of $\pi_1(S^1) = \mathbb{Z}$.

(c) The transport function $\text{transport}^P : a = b \to P(a) \to P(b)$ for a type family $P : A \to \mathcal{U}$ corresponds in the simplicial model to the monodromy action of a path on the fiber of a fibration. Verify this for $P : S^1 \to \mathcal{U}$ defined by $P(\text{base}) = \mathbb{Z}$ and $P(\text{loop}) = \text{succ}$ (the successor function).
