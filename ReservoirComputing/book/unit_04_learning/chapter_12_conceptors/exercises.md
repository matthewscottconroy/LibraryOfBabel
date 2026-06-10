# Chapter 12 Exercises

## Analytical Exercises

**Exercise 12.1 (Conceptor derivation from optimization).**

(a) Re-derive the conceptor formula $C = R(R + \alpha^{-2}I)^{-1}$ by completing the matrix calculus derivation in Section 12.2.2. Specifically, compute $\frac{\partial}{\partial C}\text{tr}(CR^pC^\top)$ and verify the sign, then solve the optimality condition.

(b) Show that the alternative form $C = I - \alpha^{-2}(R + \alpha^{-2}I)^{-1}$ is equivalent to the primary form $C = R(R+\alpha^{-2}I)^{-1}$. (Hint: $R(R+\alpha^{-2}I)^{-1} = (R + \alpha^{-2}I - \alpha^{-2}I)(R+\alpha^{-2}I)^{-1} = I - \alpha^{-2}(R+\alpha^{-2}I)^{-1}$.)

(c) Let $f(R) = R(R+\alpha^{-2}I)^{-1}$. Show that $f$ is monotone: if $R_1 \preceq R_2$ (PSD order), then $f(R_1) \preceq f(R_2)$.

(d) What is the conceptor when $R = \sigma^2 I$ (a spherically symmetric state distribution)? Compute $C$ and its singular values. What does this represent geometrically?

(e) What happens to the conceptor as $\sigma^2 \to 0$ (all states near origin)? As $\sigma^2 \to \infty$?

---

**Exercise 12.2 (Boolean operations — algebraic verification).**

(a) Verify De Morgan's law for conceptors: $C \wedge B = \lnot(\lnot C \vee \lnot B)$ by showing that the explicit formula for AND derived from De Morgan agrees with the direct formula.

(b) Show the absorption law: $C \vee (C \wedge B) = C$. (Hint: use the OR formula $C \vee A = C(C + A - CA)^{-1}A$ with $A = C \wedge B$. You will need the explicit AND formula.)

(c) Show the distributive law $C \wedge (B_1 \vee B_2) = (C \wedge B_1) \vee (C \wedge B_2)$ for a simple case: $B_1 = b_1 \mathbf{v}\mathbf{v}^\top$ and $B_2 = b_2 \mathbf{v}\mathbf{v}^\top$ (rank-1 conceptors in the same direction $\mathbf{v}$). Show both sides equal $C \wedge ((b_1 + b_2 - b_1 b_2)\mathbf{v}\mathbf{v}^\top)$.

(d) Verify that $C \vee I = I$ and $C \wedge \mathbf{0} = \mathbf{0}$ using the algebraic formulas.

---

**Exercise 12.3 (Aperture and capacity).**

(a) Show that the total activation $\text{tr}(C)$ of a conceptor is an increasing function of $\alpha$: $\frac{d}{d\alpha}\text{tr}(C) > 0$ for $\alpha > 0$.

(b) For a reservoir with $N$ neurons and a single pattern, show that $\text{tr}(C) \to 0$ as $\alpha \to 0$ and $\text{tr}(C) \to \text{rank}(R)$ as $\alpha \to \infty$.

(c) The *conceptor quota* is defined as $q = \text{tr}(C)/N \in [0,1]$. For a set of $P$ stored patterns with conceptors $C_1, \ldots, C_P$, the quota of the OR of all conceptors is $q_\vee = \text{tr}(C_1 \vee \cdots \vee C_P)/N$. Show that $q_\vee \leq 1$ (the total quota cannot exceed 1), and interpret this as a capacity constraint.

(d) For two orthogonal patterns (state subspaces $V_1 \perp V_2$, each of dimension $d$), compute the quotas $q_1 = q_2 = d/N$ and $q_\vee = q_1 + q_2 = 2d/N$. How does this relate to the capacity constraint in (c)?

---

**Exercise 12.4 (Pattern interpolation via aperture morphing).**

Given two conceptors $C_1$ and $C_2$, define the $\lambda$-interpolation as the conceptor corresponding to the $\lambda$-mixture of state covariances:

$$C(\lambda) = \text{conceptor}\!\bigl((1-\lambda)R_1 + \lambda R_2, \alpha\bigr), \quad \lambda \in [0,1].$$

(a) Show that $C(0) = C_1$ and $C(1) = C_2$.

(b) Compute $C(\lambda)$ in the eigenbasis of the combined covariance $R_1 + R_2$ (assuming $R_1$ and $R_2$ commute — i.e., they have the same eigenvectors). Express the interpolated singular values as a function of $\lambda$.

(c) In the noncommuting case, argue qualitatively that the interpolation $C(\lambda)$ smoothly transitions from $C_1$ to $C_2$ as $\lambda$ goes from 0 to 1.

(d) What would running the reservoir with the interpolated conceptor $C(\lambda)$ produce? Describe the expected output for $\lambda = 0, 0.5, 1$ when pattern 1 is a 2 Hz sinusoid and pattern 2 is a 4 Hz sinusoid.

---

**Exercise 12.5 (Conceptors and memory capacity).**

(a) When the reservoir is driven by pattern $p$ for recall (using conceptor $C^p$), the effective dynamics are $\mathbf{r}(t) = C^p \tanh(W\mathbf{r}(t-1) + \mathbf{w}^{fb}z(t-1))$. The conceptor effectively replaces $W$ with $C^p W$ (approximately). What is the spectral radius of $C^p W$ in terms of $C^p$ and $W$? Under what conditions is this stable?

(b) Show that the memory capacity of the conceptor-modified reservoir is bounded by $\text{tr}(C^p)$ (the trace of the conceptor), not $N$. Interpret this as "the conceptor reduces the effective dimensionality of the reservoir."

(c) For two patterns with orthogonal subspaces, show that the reservoir's effective capacity (using the OR conceptor) is approximately $\text{tr}(C_1 \vee C_2) = \text{tr}(C_1) + \text{tr}(C_2)$. What does this tell you about the memory allocated to each pattern?

---

## Thought Experiments

**Thought Experiment 12.1: Conceptors and Neural Representation.**

(a) The conceptor $C^p$ encodes the *geometry* of the reservoir's activity during pattern $p$, not the pattern itself. Does the reservoir "know" pattern $p$ in the same sense as a lookup table knows it? In what ways is the conceptor representation richer, and in what ways is it poorer?

(b) The Boolean operations allow conceptors to be combined: $C^1 \vee C^2$ is the OR of two patterns. In a biological neural circuit, what would "OR of two patterns" correspond to? Is there evidence that biological memory systems support operations analogous to conceptor NOT, AND, OR?

(c) The conceptor framework stores patterns as geometric constraints on reservoir activity, not as fixed point attractors (as in a Hopfield network). What are the advantages of the conceptor approach over the Hopfield approach for storing many patterns? What are the disadvantages?

---

**Thought Experiment 12.2: What Are the Limits of Conceptor Logic?**

(a) The set of conceptors forms a bounded distributive lattice. Classical Boolean logic also forms a Boolean algebra — a bounded distributive lattice with complementation. Are conceptors a Boolean algebra? (Hint: a Boolean algebra requires $C \wedge \lnot C = \mathbf{0}$ and $C \vee \lnot C = I$. Check whether these hold for conceptors.)

(b) If conceptors do not form a Boolean algebra, what additional structure is missing? Is there a natural way to "fix" this? (Hint: consider what happens to $C \wedge \lnot C$ when the eigenvalues of $C$ are in $(0,1)$ rather than $\{0,1\}$.)

(c) The conceptor lattice can be mapped to the Boolean lattice by the operation $C \mapsto \text{sign}(C - \epsilon I)$ for small $\epsilon > 0$ (thresholding the eigenvalues at $\epsilon$). Does this thresholded conceptor satisfy $C \wedge \lnot C = \mathbf{0}$? What is lost by this thresholding?

---

## Key Concepts

**1. Conceptor Matrix**
A symmetric positive semidefinite matrix $C \in \mathbb{R}^{N \times N}$ with eigenvalues in $[0,1]$, defined as $C = R(R + \alpha^{-2}I)^{-1}$ where $R$ is the state covariance of a pattern and $\alpha$ is the aperture. The conceptor encodes the geometry of the pattern's state activity as a soft projection onto the pattern's active subspace.

**2. State Covariance Matrix $R$**
The empirical covariance $R = \frac{1}{T}\sum_t \mathbf{r}(t)\mathbf{r}(t)^\top$ of reservoir states when driven by a specific pattern. Encodes the shape, orientation, and variance of the pattern's state cloud. The conceptor is derived directly from $R$.

**3. Aperture $\alpha$**
The hyperparameter controlling the "softness" of the conceptor's projection. Small $\alpha$: strong regularization, conceptor near zero (suppresses everything). Large $\alpha$: weak regularization, conceptor near the hard projection onto the pattern's subspace. The critical value is $\sigma_i \approx 1/\alpha$, which separates "retained" directions ($\sigma_i > 1/\alpha$, large singular value in $C$) from "suppressed" directions ($\sigma_i < 1/\alpha$, small singular value in $C$).

**4. Conceptor NOT ($\lnot C = I - C$)**
The complement conceptor. Its singular values are $1 - d_i$ where $d_i$ are $C$'s singular values. Passes directions suppressed by $C$ and suppresses directions passed by $C$. Geometric interpretation: approximately projects onto the orthogonal complement of $C$'s active subspace.

**5. Conceptor OR ($C \vee B$)**
The conceptor capturing the union of the active subspaces of $C$ and $B$. Defined algebraically as $C \vee B = C(C + B - CB)^{-1}B$. Passes directions active in either $C$ or $B$.

**6. Conceptor AND ($C \wedge B$)**
The conceptor capturing the intersection of the active subspaces of $C$ and $B$. Defined via De Morgan's law: $C \wedge B = \lnot(\lnot C \vee \lnot B)$. Passes only directions active in both $C$ and $B$.

**7. Bounded Distributive Lattice**
The algebraic structure formed by conceptors under NOT, AND, OR operations and the PSD partial order. Bottom element: $\mathbf{0}$; top element: $I$. Distributive law: $C \wedge (B \vee A) = (C \wedge B) \vee (C \wedge A)$. This structure is the same as classical Boolean logic, except that conceptors do not satisfy the complementation law $C \wedge \lnot C = \mathbf{0}$ (they form a Heyting algebra, not a Boolean algebra).

**8. Conceptor Quota**
The trace $\text{tr}(C)/N \in [0,1]$, measuring the total "activation" of the conceptor. Analogous to the fraction of the reservoir's dimensions claimed by the pattern. The quota of $C_1 \vee C_2$ equals $\text{tr}(C_1) + \text{tr}(C_2) - \text{tr}(C_1 C_2)$, reflecting the inclusion-exclusion principle for overlapping subspaces.

**9. Pattern Recall via Conceptor Constraint**
Using the conceptor $C^p$ during reservoir autonomous operation: $\mathbf{r}(t) = C^p\tanh(W\mathbf{r}(t-1) + \mathbf{w}^{fb}z(t-1))$. The conceptor constrains the reservoir state to lie near the pattern's active subspace, steering the reservoir toward the stored pattern's dynamics.

**10. Aperture Adaptation**
The operation of changing a conceptor's aperture from $\alpha$ to $\gamma\alpha$: $C^{(\gamma\alpha)} = C(C + \gamma^{-2}(I-C))^{-1}$. This allows the aperture to be tuned post-hoc without recomputing the conceptor from data. Large $\gamma$ (increasing aperture) makes the conceptor more "open" (passes more directions); small $\gamma$ makes it more restrictive.

---

## Key Researchers

## Herbert Jaeger

Conceptors are, in their totality, the work of one person: Herbert Jaeger. The 2014 monograph "Controlling Recurrent Neural Networks by Conceptors" [Jaeger2014] is 110 pages long, mathematically dense, and represents Jaeger's most ambitious theoretical work in reservoir computing. It develops the full mathematical theory from scratch: the definition of conceptors as regularized projections, the Boolean lattice structure, the aperture adaptation formulas, the conceptor-based pattern storage and recall algorithms, and numerous applications to cognitive and neuroscience-inspired tasks.

Jaeger's background is in cognitive science and dynamical systems, and the conceptor monograph reflects both: the mathematics is rigorous but the motivation is always cognitive — what does it mean for a neural system to "remember," to "combine," to "negate" a concept? The conceptor framework gives these words precise mathematical content.

The monograph predated Jaeger's move to Constructor University (formerly Jacobs University) Bremen, where he has continued to develop the framework. A key later contribution is the extension of conceptors to the "conceptor-aided backpropagation" framework for training deep networks — an application that moves conceptors from the specialized world of reservoir computing into the mainstream of deep learning.

Jaeger has been consistently generous with his time in explaining conceptors to new researchers. His extensive lecture notes, tutorial papers, and online course materials are available on his university website and have been essential in spreading the conceptor framework beyond his immediate collaborators. The density of the 2014 monograph means that it rewards careful, repeated reading — students who invest this time consistently report that the effort was worthwhile.

---

## Further Reading

**Jaeger, H. (2014). Controlling recurrent neural networks by conceptors. *arXiv preprint arXiv:1403.3369*.**
[Jaeger2014]
The primary source. A 110-page monograph covering the complete theory. Sections 1–5 are accessible with the background from this textbook; Sections 6–9 cover advanced applications. The appendix contains full proofs of the lattice theorems.

**Jaeger, H. (2017). Using conceptors to manage neural long-term memories for temporal patterns. *Journal of Machine Learning Research*, 18(13), 1–43.**
[Jaeger2017]
A more focused paper on the memory management application, shorter than the 2014 monograph and a good entry point.

**Jaeger, H., Noheda, B., & van der Wiel, W. G. (2023). Toward a formal theory for computing machines made out of whatever physics offers. *Nature Communications*, 14, 4911.**
[Jaeger2023]
A recent paper connecting the conceptor framework to the broader agenda of physical computing substrates — relevant context for the physical reservoir computing discussed elsewhere in this book.
