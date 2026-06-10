# Exercises

---

**Exercise 15.1 (Simplicial Identities).** Verify the simplicial identities for face maps.

(a) Verify $\partial_i \partial_j = \partial_{j-1} \partial_i$ for $i < j$ by computing on elements of $\Delta[n]_m$.

(b) Explain geometrically: why should applying "face $i$" then "face $j$" equal applying "face $j-1$" then "face $i$" (for $i < j$)?

(c) Verify the face-degeneracy identity $\sigma_j \partial_i = \partial_i \sigma_{j-1}$ for $i < j$ by computation.

---

**Exercise 15.2 (Nerve of a Category).** For a small category $\mathcal{C}$:

(a) Describe $N(\mathcal{C})_0$, $N(\mathcal{C})_1$, $N(\mathcal{C})_2$, and $N(\mathcal{C})_3$ explicitly.

(b) Describe the face maps $\partial_0, \partial_1, \partial_2 : N(\mathcal{C})_2 \to N(\mathcal{C})_1$.

(c) Describe the degeneracy maps $\sigma_0, \sigma_1 : N(\mathcal{C})_1 \to N(\mathcal{C})_2$.

(d) Verify the simplicial identity $\partial_1 \sigma_0 = \mathsf{id}$ (the "left unit" for degeneracies).

---

**Exercise 15.3 (Nerve and Groupoids).** Show that $N(\mathcal{C})$ is a Kan complex iff $\mathcal{C}$ is a groupoid.

(a) Assume $\mathcal{C}$ is a groupoid. Show that the outer horn $\Lambda^2_0 \to N(\mathcal{C})$ always fills. What does the filling correspond to in $\mathcal{C}$?

(b) Assume $\mathcal{C}$ is not a groupoid: there's a morphism $f : A \to B$ with no inverse. Construct a specific outer horn that cannot be filled.

(c) Show that inner horn $\Lambda^2_1 \to N(\mathcal{C})$ fills uniquely (using composition in $\mathcal{C}$). Why unique?

---

**Exercise 15.4 (Singular Complex of $S^1$).** Compute $\mathsf{Sing}(S^1)$ in low dimensions.

(a) Describe $\mathsf{Sing}(S^1)_0$: what are the 0-simplices? (They are points of $S^1$.)

(b) Describe $\mathsf{Sing}(S^1)_1$: what are the 1-simplices? (They are continuous maps $[0,1] \to S^1$.)

(c) Describe $\pi_1(\mathsf{Sing}(S^1), *)$ (the fundamental group of the simplicial set): what are the loops? What is the group operation? (It should be $\mathbb{Z}$.)

---

**Exercise 15.5 (Geometric Realization).** Compute geometric realizations.

(a) $|\Delta[0]|$: what topological space is this? (It should be a point.)

(b) $|\Delta[1]|$: what topological space? (It should be a line segment $[0,1]$.)

(c) $|\partial\Delta[2]|$: it's the boundary of a triangle. Verify it's homeomorphic to $S^1$.

(d) $|N(G)|$ for a group $G$ (viewed as a one-object groupoid): the *classifying space* $BG$. Describe $|N(\mathbb{Z})|$ and show it's homeomorphic to $S^1$.

---

**Exercise 15.6 (Kan Fibrations).** 

(a) Show that any map $p : X \to Y$ where $X$ is a Kan complex and $Y = *$ is a Kan fibration iff $X$ is a Kan complex. (So Kan complexes = fibrant objects.)

(b) Show that the pullback of a Kan fibration is a Kan fibration. (Kan fibrations are stable under base change.)

(c) Show that the composition of two Kan fibrations is a Kan fibration.

---

**Exercise 15.7 (Path Object).** The path object factorization:

(a) For a Kan complex $X$, define the path object $X^{\Delta[1]}$ (the simplicial exponential). Describe its 0-simplices and 1-simplices.

(b) Show the factorization $X \xrightarrow{r} X^{\Delta[1]} \xrightarrow{(s,t)} X \times X$ where $r$ is the "constant path" map and $(s,t)$ evaluates at the endpoints.

(c) Show $r$ is an acyclic cofibration (hint: it's a deformation retraction) and $(s,t)$ is a Kan fibration.

(d) Identify the fiber of $(s,t)$ over $(a, b)$ with the path space $\{p : a \to b\}$ in $X$.

---

**Exercise 15.8 (The Univalence Axiom in the Model).** Work through the key idea.

(a) Describe informally what a 0-simplex, 1-simplex, and 2-simplex of the universe Kan complex $\hat{U}$ should be.

(b) A path in $\hat{U}$ from type $A$ to type $B$ is a 1-simplex $A \to B$ in $\hat{U}$. In the model, this corresponds to a weak equivalence $A \simeq B$. Explain why.

(c) The Univalence axiom says $\mathsf{ua} : (A \simeq B) \simeq (A =_{\mathsf{Type}} B)$. Explain what this means in terms of the Kan complex $\hat{U}$.

---

**Exercise 15.9 (HITs in the Simplicial Model).** The circle as a HIT.

(a) The circle HIT $S^1$ has constructors $\mathsf{base} : S^1$ and $\mathsf{loop} : \mathsf{base} = \mathsf{base}$. What should $\llbracket S^1 \rrbracket$ be as a simplicial set?

(b) We claim $\llbracket S^1 \rrbracket = N(\mathbf{B}\mathbb{Z})$ (the nerve of the one-object groupoid with automorphisms $\mathbb{Z}$). Verify: what is $N(\mathbf{B}\mathbb{Z})_0$? $N(\mathbf{B}\mathbb{Z})_1$?

(c) Compute $\pi_1(N(\mathbf{B}\mathbb{Z}), *)$ and show it equals $\mathbb{Z}$.

---

**Exercise 15.10 (Research: Cubical vs. Simplicial Models).** Compare the simplicial and cubical models of HoTT.

(a) What are the key differences between simplicial sets and cubical sets (over the De Morgan cube category)?

(b) In the simplicial model, Univalence is a theorem but has no canonical computation rule. In the cubical model, Univalence holds by construction with a computation rule (the Glue type). What is the practical significance of having computation rules?

(c) What is the status of the Brunerie number ($\pi_4(S^3) = \mathbb{Z}/n$) in the simplicial vs. cubical models? Has it been computed computationally?

(d) Are the simplicial and cubical models equivalent (as models of HoTT)? What would it mean for them to be equivalent?
