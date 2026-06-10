# Applied Exercises

Synthetic homotopy theory is the program of proving theorems about topological spaces using only the axioms of type theory, without any underlying set-theoretic constructions. This might sound purely theoretical, but the core ideas — encode-decode maps, winding numbers, fiber sequences, connectivity bounds — connect directly to practical computation. Winding numbers are used in computational geometry (point-in-polygon tests, rotation counting in robotics). The Hopf fibration is the mathematical structure underlying unit quaternions and 3D rotation interpolation (SLERP). The Blakers-Massey connectivity theorem has analogs in topological data analysis. And the encode-decode proof itself is a program: the `encode` function literally computes winding numbers. The exercises below develop these connections concretely, from implementing winding number algorithms to working through the Hopf fibration's relevance for orientation-preserving maps in 3D.

---

## Exercise A.1: Implementing the Discrete Winding Number
*Domain: Computational Geometry / Algorithms / Program Extraction from Proofs*

**Setup:** The HoTT proof that $\pi_1(S^1) = \mathbb{Z}$ centers on the `encode` function:
$$\mathsf{encode} : (\mathsf{base} = \mathsf{base}) \to \mathbb{Z}$$
defined by $\mathsf{encode}(p) = \mathsf{transport}^\mathsf{code}(p, 0)$, which computes the winding number by counting how many times $p$ traverses `loop`. This is the type-theoretic formalization of the classical notion of winding number.

The discrete analog: given a closed discrete path in $\mathbb{Z}^2 \setminus \{(0,0)\}$ (a polygon not passing through the origin), the winding number counts how many times the path winds around the origin. This is computed by the standard algorithm: at each step from point $P_i$ to $P_{i+1}$, count $+1$ if the step crosses the positive x-axis upward, $-1$ if it crosses downward, and $0$ otherwise.

**Questions:**
1. Implement `winding_number(path: List[(Int, Int)]) -> Int` in Python or pseudocode, where the input is a closed polygon (a list of integer lattice points forming a closed loop — `path[0] == path[-1]`). Your implementation should use the "crossing number" algorithm: for each consecutive pair $(P_i, P_{i+1})$, determine whether the directed edge crosses the ray from the origin in the positive x-direction, and accumulate the count. Include a brief explanation of why this correctly implements the HoTT `encode` function's discrete analog.
2. Test your implementation on the following paths (all starting and ending at `(1, 0)`):
   - The counterclockwise unit square: `(1,0) → (0,1) → (-1,0) → (0,-1) → (1,0)`. Expected winding number: 1.
   - The clockwise unit square: `(1,0) → (0,-1) → (-1,0) → (0,1) → (1,0)`. Expected winding number: -1.
   - The figure-eight path that winds once counterclockwise and once clockwise. Expected winding number: 0.
3. (Extension) In HoTT, the `decode` function sends $n : \mathbb{Z}$ to `loop^n` — the loop traversed $n$ times. In the discrete setting, the analogous function `make_path(n: Int) -> List[(Int, Int)]` should produce a closed polygon with winding number exactly $n$ around the origin. Implement this function. Then verify `winding_number(make_path(n)) == n` for $n = -3, -2, -1, 0, 1, 2, 3$. This is the discrete encode-decode round trip.

*Abstract concept illustrated: the `encode` function for $\pi_1(S^1) = \mathbb{Z}$, the winding number as a program extracted from a HoTT proof.*

---

## Exercise A.2: Writing the Encode-Decode Proof in Cubical Agda
*Domain: Proof Assistants / Verified Mathematics / Cubical Agda*

**Setup:** The $\pi_1(S^1) = \mathbb{Z}$ proof in HoTT proceeds by:
1. Defining `code : S¹ → Type` with `code base = ℤ` and the loop transporting by successor.
2. Defining `encode : (x : S¹) → base ≡ x → code x` by `encode x p = transport code p 0`.
3. Defining `decode : (x : S¹) → code x → base ≡ x` by circle induction (sending `n : ℤ` to `loop^n`).
4. Proving `encode x (decode x c) = c` and `decode x (encode x p) = p`.

In Cubical Agda, the circle is defined by `data S¹ : Type where base : S¹; loop : base ≡ base`. The `transport` along `ua(succ)` reduces definitionally to applying `succ`, making the proof computational.

**Questions:**
1. Write the Agda definition of `code : S¹ → Type`. You will need to use `hcomp` (or `subst`/`transport`) and `ua`. The key line is:
   ```agda
   code : S¹ → Type
   code base    = ℤ
   code (loop i) = ua sucℤ-equiv i
   ```
   Fill in the type signature for `sucℤ-equiv : ℤ ≃ ℤ` (the successor equivalence) and write its definition in Agda, specifying the forward function, the inverse, and the proofs that they are mutually inverse.
2. Write the `encode` and `decode` functions in Agda. For `decode`, use circle induction:
   ```agda
   decode : (x : S¹) → code x → base ≡ x
   decode base    n = (loop ^ n)  -- loop iterated n times, using ℤ-induction
   decode (loop i) = ?            -- fill this in using PathP
   ```
   Explain what `PathP` type the second clause must inhabit, and state what the `?` should be (you do not need to prove it fully, but state it precisely).
3. (Extension) The key step in proving `decode (encode p) = p` is the following: for `p = loop`, `encode loop = transport code loop 0 = sucℤ 0 = 1`, and `decode 1 = loop`. Write this calculation out step by step, identifying which computation rule (the `ua` transport rule, the definition of `sucℤ`) is used at each step. Then explain why this calculation is *definitional* in Cubical Agda (the terms literally reduce) rather than *propositional* (requiring a separate proof), and why this distinction matters.

*Abstract concept illustrated: the full encode-decode proof for $\pi_1(S^1) = \mathbb{Z}$, cubical type theory's computational univalence.*

---

## Exercise A.3: Unit Quaternions, the Hopf Fibration, and SO(3)
*Domain: Computer Graphics / Robotics / Rotation Geometry*

**Setup:** The unit quaternions $\mathbb{H}_1 = \{q \in \mathbb{H} : |q| = 1\}$ form a group under quaternion multiplication, and as a topological space this group is homeomorphic to $S^3$. They are used in computer graphics and robotics to represent 3D rotations: there is a surjective group homomorphism $\phi : \mathbb{H}_1 \to \mathsf{SO}(3)$ (the group of orientation-preserving rotations of $\mathbb{R}^3$), and the kernel of $\phi$ is $\{+1, -1\} \cong \mathbb{Z}/2\mathbb{Z}$. So $\mathsf{SO}(3) \cong S^3 / (\mathbb{Z}/2\mathbb{Z}) = \mathbb{RP}^3$.

The Hopf fibration $S^1 \to S^3 \xrightarrow{h} S^2$ is related: viewing $S^3 \subset \mathbb{C}^2$ and $S^2 \subset \mathbb{C} \times \mathbb{R}$, the Hopf map $h(z_1, z_2) = (2z_1\bar{z}_2, |z_1|^2 - |z_2|^2)$ defines a fiber bundle with $S^1$ fibers. The homotopy groups follow from the long exact sequence: $\ldots \to \pi_3(S^1) \to \pi_3(S^3) \to \pi_3(S^2) \to \pi_2(S^1) \to \ldots$, giving $\pi_3(S^2) = \mathbb{Z}$ generated by $h$.

**Questions:**
1. The long exact sequence of the Hopf fibration $S^1 \to S^3 \to S^2$ includes the segment:
   $$\pi_3(S^1) \to \pi_3(S^3) \to \pi_3(S^2) \to \pi_2(S^1) \to \pi_2(S^3)$$
   Using the facts $\pi_3(S^1) = 0$, $\pi_3(S^3) = \mathbb{Z}$, $\pi_2(S^1) = 0$, and $\pi_2(S^3) = 0$, read off $\pi_3(S^2)$ from the exactness of this sequence.
2. The double cover $S^3 \to \mathsf{SO}(3)$ gives a fiber bundle $\mathbb{Z}/2\mathbb{Z} \to S^3 \to \mathsf{SO}(3)$. Write down the relevant portion of its long exact sequence (the segment involving $\pi_1$ and $\pi_2$) and use it to compute $\pi_1(\mathsf{SO}(3))$. What does this say about the "gimbal lock" / "rotation double cover" phenomenon in robotics: specifically, why is there no continuous way to represent all 3D rotations without "double cover" ambiguity?
3. (Extension) In HoTT, the Hopf fibration is constructed as follows: $S^3$ is defined as the join $S^1 * S^1$, and the Hopf map $h : S^1 * S^1 \to S^2$ is defined using the $S^1$-action on itself by multiplication. Define the $S^1$-action on $S^1$ as a function $\mathsf{act} : S^1 \to (S^1 \simeq S^1)$, sending $z : S^1$ to the rotation-by-$z$ equivalence. Explain why this action is well-defined using the circle's non-dependent eliminator, and describe how the Hopf map $h$ is built from this action using the join structure.

*Abstract concept illustrated: the Hopf fibration and its long exact sequence, $\pi_3(S^2) = \mathbb{Z}$, the HoTT construction of the Hopf map.*

---

## Exercise A.4: Connectivity and the Blakers-Massey Theorem in Data Topology
*Domain: Topological Data Analysis / Applied Topology / Machine Learning*

**Setup:** The Blakers-Massey theorem is a statement about *pushouts*: if $f : A \to B$ is $m$-connected and $g : A \to C$ is $n$-connected, then the natural map from $A$ to the homotopy pullback of $B \to B \sqcup_A C \leftarrow C$ is $(m + n)$-connected. This governs the "excision" phenomenon: the topology of a pushout is controlled by the connectivity of the maps going in.

In topological data analysis (TDA), one studies data sets by their topology: points in a metric space are connected by edges when they are close (the Vietoris-Rips complex), and the resulting simplicial complex's homotopy type encodes the "shape" of the data. The Blakers-Massey theorem applies here: if one "glues" two data regions along a shared boundary, the connectivity of the gluing determines how much topological information of the pieces survives into the total space.

**Questions:**
1. Informally, a map $f : X \to Y$ is $n$-connected if all maps from spheres $S^k \to Y$ (for $k \leq n$) can be lifted to maps $S^k \to X$ through $f$, up to homotopy. For a data scientist's purposes: a 0-connected map means every connected component of $Y$ is hit; a 1-connected map means every loop in $Y$ can be lifted. Given two Vietoris-Rips complexes $R_A$ and $R_B$ built from overlapping data sets $A$ and $B$, with $R_A \cap R_B = R_C$ (the complex of the overlap), describe in informal terms what it means for the inclusion $R_C \hookrightarrow R_A$ to be $m$-connected and $R_C \hookrightarrow R_B$ to be $n$-connected.
2. The Blakers-Massey theorem says the map $R_C \to \mathsf{hofib}(R_A \to R_A \sqcup_{R_C} R_B)$ is $(m+n)$-connected. What does this say about the relationship between the topology of the overlap region $R_C$ and the topology of the combined complex $R_A \sqcup_{R_C} R_B$? Specifically: if $m = n = 1$ (both inclusions are 1-connected), what connectivity bound does Blakers-Massey give for the homotopy pushout, and what does this mean for the fundamental group of the combined complex?
3. (Extension) The Seifert-van Kampen theorem is a special case of Blakers-Massey. State the van Kampen theorem as a theorem about pushouts of fundamental groupoids, and show how it follows from Blakers-Massey when the overlap $C$ is connected ($0$-connected) and the maps are at least $1$-connected. In TDA, when would you apply van Kampen rather than Blakers-Massey, and when would you need the stronger Blakers-Massey result?

*Abstract concept illustrated: the Blakers-Massey theorem, connectivity of maps, the van Kampen theorem as a special case.*

---

## Exercise A.5: The Euler Characteristic via Long Exact Sequences
*Domain: Algebraic Topology / Computational Topology / Computer Science Theory*

**Setup:** The Euler characteristic of a finite CW complex $X$ is $\chi(X) = \sum_n (-1)^n \cdot c_n$, where $c_n$ is the number of $n$-cells. For a surface of genus $g$, $\chi = 2 - 2g$ (for orientable surfaces) or $\chi = 2 - g$ (for non-orientable ones). The Euler characteristic is additive: for a pushout $X = A \cup_C B$ (where $C = A \cap B$), $\chi(X) = \chi(A) + \chi(B) - \chi(C)$.

The long exact sequence of a fibration $F \to E \to B$ connects the homotopy groups of the three spaces: $\ldots \to \pi_n(F) \to \pi_n(E) \to \pi_n(B) \to \pi_{n-1}(F) \to \ldots$. For the Hopf fibration $S^1 \to S^3 \to S^2$, this gives a powerful method for computing homotopy groups from the groups of simpler spaces.

**Questions:**
1. Compute the Euler characteristic of the following spaces from their CW decompositions:
   - $S^n$ (the $n$-sphere): one 0-cell and one $n$-cell.
   - $T^2$ (the torus): one 0-cell, two 1-cells, one 2-cell.
   - $\mathbb{RP}^2$ (the real projective plane): one 0-cell, one 1-cell, one 2-cell.
   - The genus-2 surface $\Sigma_2$: one 0-cell, four 1-cells, one 2-cell.
   Verify the formulas $\chi(S^n) = 1 + (-1)^n$, $\chi(T^2) = 0$, $\chi(\mathbb{RP}^2) = 1$, $\chi(\Sigma_2) = -2$.
2. Verify the additivity formula $\chi(S^2) = \chi(D^2_+) + \chi(D^2_-) - \chi(S^1)$ using the decomposition of $S^2$ as the union of the upper and lower hemispheres (each homeomorphic to $D^2$) along the equatorial circle $S^1$. (Answer: $1 + 1 - 0 = 2 = \chi(S^2)$.) Now use the Mayer-Vietoris sequence (the homological analog of van Kampen) to compute $H_*(S^2)$ from $H_*(D^2_+)$, $H_*(D^2_-)$, and $H_*(S^1)$. State the Mayer-Vietoris long exact sequence for this decomposition and read off the homology groups.
3. (Extension) The Hopf fibration $S^1 \to S^3 \to S^2$ does not contribute to the Euler characteristic directly (since $\chi(S^1) = 0$, $\chi(S^3) = 0$, $\chi(S^2) = 2$, and $0 \neq 0 \cdot 2$). This is because the Euler characteristic formula $\chi(E) = \chi(F) \cdot \chi(B)$ holds for fiber bundles with simply-connected base, but $S^2$ has $\pi_1(S^2) = 0$ and the formula does give $\chi(S^3) = \chi(S^1) \cdot \chi(S^2) = 0 \cdot 2 = 0$, which is correct. Verify this formula for the Hopf fibration, and also verify it for the covering $\mathbb{Z}/2\mathbb{Z} \to S^3 \to \mathbb{RP}^3$ (where $\chi(\mathbb{Z}/2\mathbb{Z}) = 2$, $\chi(S^3) = 0$, and $\chi(\mathbb{RP}^3) = 0$).

*Abstract concept illustrated: the Euler characteristic, Mayer-Vietoris, fiber bundle multiplicativity, the long exact sequence of a fibration.*

---

## Exercise A.6: The EHP Sequence and Stable Homotopy
*Domain: Algebraic Topology / Theoretical Computer Science / Homotopy Theory*

**Setup:** The EHP sequence is a long exact sequence:
$$\cdots \to \pi_{n}(S^{k-1}) \xrightarrow{E} \pi_{n+1}(S^k) \xrightarrow{H} \pi_{n+1}(S^{2k-1}) \xrightarrow{P} \pi_{n-1}(S^{k-1}) \to \cdots$$
where $E$ is the suspension (Einhängung in German, hence "E"), $H$ is the Hopf invariant, and $P$ is the Whitehead product. This is the key tool for inductively computing homotopy groups of spheres, and it is the sequence that Brunerie uses in his proof of $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$.

**Questions:**
1. Using the EHP sequence with $k = 2$ and $n = 3$, write the segment:
   $$\pi_3(S^1) \xrightarrow{E} \pi_4(S^2) \xrightarrow{H} \pi_4(S^3) \xrightarrow{P} \pi_2(S^1) \xrightarrow{E} \pi_3(S^2)$$
   Fill in the known groups: $\pi_3(S^1) = 0$ (since $S^1$ is a $K(\mathbb{Z},1)$, so all higher homotopy vanishes), $\pi_2(S^1) = 0$, $\pi_3(S^2) = \mathbb{Z}$ (Hopf), $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ (Brunerie). Use exactness to determine what $\pi_4(S^2)$ must be.
2. The Freudenthal suspension theorem says $\pi_k(S^n) \cong \pi_{k+1}(S^{n+1})$ for $k < 2n - 1$. The *stable* homotopy group $\pi_k^s$ is the common value $\pi_{k+n}(S^n)$ for large $n$. The first few stable homotopy groups of spheres are: $\pi_0^s = \mathbb{Z}$, $\pi_1^s = \mathbb{Z}/2\mathbb{Z}$, $\pi_2^s = \mathbb{Z}/2\mathbb{Z}$, $\pi_3^s = \mathbb{Z}/24\mathbb{Z}$. Verify that the "stabilization" $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z} = \pi_1^s$ is consistent with Freudenthal: since $S^3$ is 2-connected, $\pi_k(S^3) \cong \pi_{k+1}(S^4)$ for $k \leq 3$. Check this for $\pi_4(S^3) = \pi_5(S^4) = \mathbb{Z}/2\mathbb{Z}$.
3. (Extension) Brunerie's proof that $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ proceeds in HoTT by: (a) constructing the "Brunerie element" $\beta : \pi_4(S^3)$ as the composition $S^4 \xrightarrow{\Sigma\eta} S^3 \xrightarrow{\eta} S^2$, where $\eta : S^3 \to S^2$ is the Hopf map and $\Sigma\eta : S^4 \to S^3$ is its suspension; (b) showing $\pi_4(S^3) = \mathbb{Z}/\langle \beta \rangle$; (c) computing that the order of $\beta$ is 2 (the Brunerie number computation). Explain step (a) informally: why does composing the Hopf map with its own suspension give an element of $\pi_4(S^3)$? (Recall that $\pi_4(S^3)$ consists of homotopy classes of maps $S^4 \to S^3$; the composition $\Sigma\eta \circ \text{something}$ needs to land in $\mathsf{Map}(S^4, S^3)$. How?)

*Abstract concept illustrated: the EHP sequence, stable homotopy groups, the Brunerie element, the structure of $\pi_4(S^3)$.*
