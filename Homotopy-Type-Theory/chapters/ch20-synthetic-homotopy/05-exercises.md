# Chapter 20 Exercises: Synthetic Homotopy Theory

---

## Section 1: Encode-Decode Method

**Exercise 1.1.** Apply the encode-decode method to compute $\pi_1(S^1 \vee S^1)$:
1. Define a code family over $S^1 \vee S^1$ using the free group $F_2$ and the loops at each circle
2. Define encode and decode maps
3. Show they're inverses and conclude $\pi_1(S^1 \vee S^1) = F_2$

**Exercise 1.2.** Apply encode-decode to compute $\pi_1(K(G, 1))$ for an arbitrary group $G$:
1. The code family: $\mathsf{code}(*, -) = G$ at the basepoint, with transport along each loop $g$ being left-multiplication by $g$
2. Define encode and decode
3. Show $\pi_1(K(G, 1)) = G$

**Exercise 1.3.** Compute $\pi_1(\mathbb{RP}^2)$:
1. Recall $\mathbb{RP}^2$ is the HIT with one point, one loop, and a 2-cell saying $\mathsf{loop}^2 = \mathsf{refl}$
2. Define a code family with $\mathsf{code}(\mathsf{base}) = \mathbb{Z}/2\mathbb{Z}$
3. Carry out the encode-decode argument

**Exercise 1.4 (Encode for the torus).** The torus $T^2 = S^1 \times S^1$ has $\pi_1 = \mathbb{Z}^2$. Describe the code family over $T^2$ and the encode/decode maps. What is $\mathsf{code}(\mathsf{base}, \mathsf{base})$?

---

## Section 2: π₁ of the Circle

**Exercise 2.1.** Verify the computation $\mathsf{encode}(\mathsf{loop}^n) = n$ for $n = -2, -1, 0, 1, 2$ by direct calculation using the transport rules.

**Exercise 2.2.** Show that the decode map $n \mapsto \mathsf{loop}^n$ is a group homomorphism from $(\mathbb{Z}, +)$ to $(\Omega(S^1, \mathsf{base}), \cdot)$:
$$\mathsf{decode}(m + n) = \mathsf{decode}(m) \cdot \mathsf{decode}(n)$$

**Exercise 2.3.** Using $\pi_1(S^1) = \mathbb{Z}$, prove that $S^1$ is a 1-type (h-level 1): $\pi_k(S^1) = 0$ for $k \geq 2$.

*Hint:* The universal cover of $S^1$ is $\mathbb{R}$ (or $\tilde{S}^1 = \sum_{x:S^1}(\mathsf{base} = x)$), which is contractible. Use this to show the higher homotopy groups vanish.

**Exercise 2.4 (Winding number program).** Write the `windingNumber` function in Cubical Agda (or as pseudocode):
```agda
windingNumber : base ≡ base → ℤ
windingNumber p = transport code p 0
```
Compute `windingNumber loop`, `windingNumber (loop ∙ loop)`, and `windingNumber loop⁻¹`.

---

## Section 3: Van Kampen Computations

**Exercise 3.1.** Use van Kampen's theorem to compute $\pi_1$ of:
1. The wedge $S^1 \vee S^1$ (two circles sharing a point)
2. The torus $T^2$ (using the decomposition as two cylinders glued along equatorial circles)
3. The Klein bottle $K$ (using the polygon identification)

**Exercise 3.2 (Mayer-Vietoris for the circle).** Decompose $S^1$ as two arcs $A$ and $B$ with overlap $A \cap B \simeq S^0$ (two points). Apply the Mayer-Vietoris sequence (a weak form of van Kampen) to compute $\pi_1(S^1)$:
$$\cdots \to \pi_1(S^0) \to \pi_1(A) \oplus \pi_1(B) \to \pi_1(S^1) \to \pi_0(S^0) \to \pi_0(A) \oplus \pi_0(B) \to \pi_0(S^1) \to 0$$

**Exercise 3.3.** Prove: if $X = A \sqcup_C B$ where $C$ is contractible, then $\pi_1(X) = \pi_1(A) * \pi_1(B)$ (free product, not amalgamated).

**Exercise 3.4.** Compute $\pi_1(\mathbb{RP}^2 \# \mathbb{RP}^2)$ (connected sum of two projective planes) using van Kampen.

---

## Section 4: Freudenthal and Stability

**Exercise 4.1.** State the Freudenthal suspension theorem precisely (for $n$-connected types). Give the bound for when $\pi_k(A) \cong \pi_{k+1}(\Sigma A)$.

**Exercise 4.2.** Using Freudenthal, show that $\pi_2(S^2) = \mathbb{Z}$:
1. Apply Freudenthal to $S^1$ (which is 0-connected) to get $\pi_1(S^1) \cong \pi_2(S^2)$
2. Use $\pi_1(S^1) = \mathbb{Z}$

**Exercise 4.3.** Show that $\pi_k(S^n) = 0$ for $k < n$, using:
1. $S^n$ is $(n-1)$-connected (a consequence of the Freudenthal and cell structure)
2. Connectivity implies trivial lower homotopy groups

**Exercise 4.4.** Using Freudenthal, show that:
- $\pi_3(S^3) = \mathbb{Z}$ (from $\pi_2(S^2) = \mathbb{Z}$ and Freudenthal for $S^2$)
- $\pi_n(S^n) = \mathbb{Z}$ for all $n \geq 1$ (by induction)

---

## Section 5: Hopf Fibration

**Exercise 5.1.** Define the Hopf fibration $h : S^3 \to S^2$ using the join construction:
1. The join $S^1 * S^1 \simeq S^3$ (construct this explicitly using the join HIT)
2. The map $h : S^1 * S^1 \to \Sigma S^1 \simeq S^2$ using the H-space multiplication

**Exercise 5.2.** Compute the long exact sequence of the Hopf fibration $S^1 \to S^3 \to S^2$:
$$\cdots \to \pi_n(S^1) \to \pi_n(S^3) \to \pi_n(S^2) \to \pi_{n-1}(S^1) \to \cdots$$

Fill in all known values and identify the isomorphisms.

**Exercise 5.3.** From the long exact sequence, derive:
1. $\pi_3(S^2) \cong \pi_3(S^3) = \mathbb{Z}$
2. $\pi_2(S^2) \cong \pi_2(S^3) \times_{\pi_2(S^1)} ?$... (work out the exact statement)

**Exercise 5.4 (Hopf invariant).** The Hopf invariant of a map $f : S^3 \to S^2$ is defined as follows: consider the cofiber $Cf = S^2 \cup_f D^4$ (a 4-dimensional CW complex). The Hopf invariant is the integer $h(f)$ such that $e^2 \cup e^2 = h(f) \cdot e^4$ in $H^*(Cf)$ (where $e^2, e^4$ are generators of $H^2$ and $H^4$).

Show that the Hopf fibration has Hopf invariant 1. (This is a difficult but fundamental calculation.)

---

## Section 6: Research-Level Exercises

**Exercise 6.1 (Brunerie's number).** Brunerie's computation $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ involves a "Brunerie number" $n$ such that $\pi_4(S^3) = \mathbb{Z}/n\mathbb{Z}$.

The computation shows $n \in \{1, 2\}$ (so $n = 2$). Describe the key steps:
1. Use $\pi_4(S^3) \cong \pi_4(S^2)$ (from the Hopf fibration)
2. Use the James construction to compute $\pi_4(S^2)$
3. Identify $n = 2$

**Exercise 6.2 (Stable homotopy groups).** Define the stable homotopy groups $\pi_k^s = \pi_{n+k}(S^n)$ for large $n$. Show that these are well-defined (independent of $n$ for large $n$) using the Freudenthal suspension theorem.

The first few stable homotopy groups are:
- $\pi_0^s = \mathbb{Z}$ (stable stem 0)
- $\pi_1^s = \mathbb{Z}/2\mathbb{Z}$ (stable stem 1, from $\pi_3(S^2) = \mathbb{Z}$)
- $\pi_2^s = \mathbb{Z}/2\mathbb{Z}$ (stable stem 2)
- $\pi_3^s = \mathbb{Z}/24\mathbb{Z}$ (stable stem 3)

Can you see how these arise from the Hopf fibration chain?

**Exercise 6.3 (Formalization in Cubical Agda).** Formalize the proof $\pi_1(S^1) = \mathbb{Z}$ in Cubical Agda:
1. Define the circle as a HIT
2. Define the integers $\mathbb{Z}$ and successor equivalence
3. Define the code family using `ua(succ)`
4. Define encode and decode
5. Prove the two round-trip equalities
6. Conclude the equivalence

**Exercise 6.4 (HoTT Book Chapter 8).** The HoTT Book's Chapter 8 contains the full synthetic proof of:
- $\pi_1(S^1) = \mathbb{Z}$ (8.1)
- Seifert-van Kampen (8.7)
- Freudenthal suspension theorem (8.6)
- $\pi_n(S^n) = \mathbb{Z}$ (8.3)

Read one of these sections (of your choice) and write a summary of the key proof steps, identifying every use of: path induction, HIT eliminators, transport, Univalence.
