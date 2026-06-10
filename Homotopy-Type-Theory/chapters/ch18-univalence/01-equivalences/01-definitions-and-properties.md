# 1.1 Equivalences: Definitions and Properties

## The Problem with Naive Bijections

In set theory, a bijection between sets $A$ and $B$ is a function $f : A \to B$ that has a two-sided inverse: there exists $g : B \to A$ with $g \circ f = \mathsf{id}_A$ and $f \circ g = \mathsf{id}_B$.

In HoTT, we can mimic this: a *quasi-equivalence* is a function $f : A \to B$ equipped with:
$$g : B \to A, \quad \eta : g \circ f \sim \mathsf{id}_A, \quad \varepsilon : f \circ g \sim \mathsf{id}_B$$

where $\sim$ denotes homotopy ($\prod_{x} f(x) = g(x)$).

The type of quasi-inverses for $f$ is:
$$\mathsf{qinv}(f) :\equiv \sum_{g : B \to A}\, (g \circ f \sim \mathsf{id}_A) \times (f \circ g \sim \mathsf{id}_B)$$

**The problem:** $\mathsf{qinv}(f)$ is *not* a proposition in general. A function $f$ can have multiple distinct quasi-inverses.

**Concrete example:** For $f = \mathsf{id}_A$, the type $\mathsf{qinv}(\mathsf{id}_A)$ contains at least the triple $(\mathsf{id}_A, \lambda x.\, \mathsf{refl}_x, \lambda x.\, \mathsf{refl}_x)$. But if $A$ has a non-trivial self-homotopy (a homotopy $\mathsf{id}_A \sim \mathsf{id}_A$ that isn't $\lambda x.\, \mathsf{refl}_x$), then there will be other elements of $\mathsf{qinv}(\mathsf{id}_A)$.

For Univalence to work correctly, we need $\mathsf{isEquiv}(f)$ to be a proposition: either $f$ is an equivalence or it isn't, with no additional "data" beyond that.

## Definition 1: Bi-Invertible Maps

The simplest fix: instead of one inverse that works on both sides, require a *left* inverse and a *right* inverse separately.

**Definition 1.1 (Bi-invertible map).** A function $f : A \to B$ is *bi-invertible* if:
$$\mathsf{isBiInv}(f) :\equiv \underbrace{\left(\sum_{g : B \to A}\, g \circ f \sim \mathsf{id}_A\right)}_{\text{left inverse}} \times \underbrace{\left(\sum_{h : B \to A}\, f \circ h \sim \mathsf{id}_B\right)}_{\text{right inverse}}$$

The key: the left inverse $g$ and right inverse $h$ need not be the same function.

**Theorem 1.2.** $\mathsf{isBiInv}(f)$ is a proposition.

*Proof sketch.* Both components are propositions:
- Left: $\sum_{g: B \to A}(g \circ f \sim \mathsf{id}_A)$. If $(g, \eta)$ and $(g', \eta')$ are both left inverses, then $g = g'$ (by: $g = g \circ f \circ g' = g'$ using $\eta$ and the right inverse... wait this uses both sides).

The correct argument: The type of left inverses $\sum_{g:B\to A}(g \circ f \sim \mathsf{id}_A)$ is a proposition when $f$ has *any* right inverse $h$ — because then $g = g \circ f \circ h = \mathsf{id}_B \circ h = h$ for any left inverse $g$. So knowing $f$ has a right inverse (from the second component) makes the first component a proposition, and vice versa.

The round-trip argument: Both components are contractible when $f$ has the other inverse. $\square$

## Definition 2: Half-Adjoint Equivalences

Another approach: keep one inverse but add a *coherence condition*.

**Definition 1.3 (Half-adjoint equivalence).** A function $f : A \to B$ is a *half-adjoint equivalence* if there exist:
- $g : B \to A$ (an inverse function)
- $\eta : g \circ f \sim \mathsf{id}_A$ (left homotopy)
- $\varepsilon : f \circ g \sim \mathsf{id}_B$ (right homotopy)
- $\tau : \prod_{x:A}\, \mathsf{ap}_f(\eta_x) = \varepsilon_{f(x)}$ (coherence)

The coherence condition $\tau$ says: the two natural ways to show $f \circ g \circ f \sim f$ agree:
- Path 1: apply $f$ to $\eta$ (the left homotopy, $g \circ f \sim \mathsf{id}$), giving $f \circ g \circ f \sim f \circ \mathsf{id} = f$
- Path 2: apply $\varepsilon$ at $f$ (the right homotopy $f \circ g \sim \mathsf{id}$, at the point $f(x)$)

These two paths should be the same — that's the coherence.

**Why add coherence?** The quasi-inverse type $\mathsf{qinv}(f)$ has $(\eta, \varepsilon)$ but no coherence. Different $(\eta, \varepsilon)$ pairs might be inequivalent. The coherence $\tau$ "pins down" the relationship between $\eta$ and $\varepsilon$, making the type a proposition.

**Theorem 1.4.** $\mathsf{isHAE}(f)$ (the type of half-adjoint equivalence structures on $f$) is a proposition.

*Proof sketch.* For any fixed $g$, the type of $(\eta, \varepsilon, \tau)$ is contractible (the coherence uniquely determines the relationship). Different choices of $g$ are related by the equivalence structure. $\square$

**The triangle identity.** The coherence $\tau$ is called a *triangle identity* — it says that the triangle in the adjunction diagram commutes:

$$f \xrightarrow{\eta_{f(x)}?} f \circ g \circ f \xrightarrow{\varepsilon_{f(x)}} f$$

The two ways around the triangle (via $\eta$ applied to $f$, or via $\varepsilon$ at $f$) agree.

## Definition 3: Contractible Fibers

The most elegant definition uses the notion of contractibility:

**Definition 1.5 (Fiber).** The *fiber* of $f : A \to B$ over $y : B$ is:
$$\mathsf{fib}_f(y) :\equiv \sum_{x : A}\, f(x) =_B y$$

An element of $\mathsf{fib}_f(y)$ is a pair $(x, p)$ where $x : A$ and $p : f(x) = y$ — a point in $A$ that maps to $y$, together with a specific path witnessing this.

**Definition 1.6 (Equivalence via contractible fibers).** A function $f : A \to B$ is an *equivalence* if all its fibers are contractible:
$$\mathsf{isEquiv}(f) :\equiv \prod_{y : B}\, \mathsf{isContr}(\mathsf{fib}_f(y))$$

**Intuition:** $f$ is an equivalence iff every $y : B$ has exactly one preimage. "Exactly one" is expressed as "contractible" — the fiber $\mathsf{fib}_f(y)$ has one element $(x_0, p_0)$ that all other elements are equal to.

**Theorem 1.7.** $\mathsf{isEquiv}(f)$ is a proposition.

*Proof.* $\mathsf{isEquiv}(f) = \prod_{y:B} \mathsf{isContr}(\mathsf{fib}_f(y))$. This is a product of propositions (each $\mathsf{isContr}(\mathsf{fib}_f(y))$ is a proposition by Theorem 1.9 of Section 1 of Chapter 17). A product of propositions is a proposition. $\square$

**Theorem 1.8 (All three definitions agree).** For any $f : A \to B$:
$$\mathsf{isBiInv}(f) \simeq \mathsf{isHAE}(f) \simeq \mathsf{isEquiv}(f)$$

Since all three are propositions (so the equivalence is automatic from any logical implication), it suffices to show that any one implies any other.

*Proof sketch.* 
- $\mathsf{isEquiv} \Rightarrow \mathsf{isBiInv}$: From contractible fibers, extract the center $(g(y), \varepsilon_y)$ of each fiber to get a right inverse $g$ with right homotopy $\varepsilon$. Extract a left inverse by a similar argument.
- $\mathsf{isBiInv} \Rightarrow \mathsf{qinv}$: A left and right inverse can be "merged" into a single quasi-inverse (take either the left or right inverse and derive the other homotopy from the two-sidedness).
- $\mathsf{qinv} \Rightarrow \mathsf{isHAE}$: From a quasi-inverse $(g, \eta, \varepsilon)$, modify $\varepsilon$ by conjugating with $f$-images of $\eta$ to get the coherence. $\square$

## The Type of Equivalences

**Definition 1.9.** The *type of equivalences* from $A$ to $B$ is:
$$A \simeq B :\equiv \sum_{f : A \to B}\, \mathsf{isEquiv}(f)$$

An element of $A \simeq B$ is a pair $(f, e)$ where $f : A \to B$ and $e : \mathsf{isEquiv}(f)$.

Since $\mathsf{isEquiv}(f)$ is a proposition, the type $A \simeq B$ is like a "subtype" of $A \to B$ consisting of the functions that happen to be equivalences.

**Projections:** From $(f, e) : A \simeq B$:
- The underlying function: $\mathsf{fun}(f, e) = f : A \to B$
- The proof of equivalence: $\mathsf{equiv}(f, e) = e : \mathsf{isEquiv}(f)$
- The inverse function: $\mathsf{inv}(f, e) = g : B \to A$ (extracted from the equivalence structure)

## Equivalences Form a Groupoid

**Theorem 1.10 (Identity equivalence).** $\mathsf{id}_A : A \simeq A$.

*Proof.* $\mathsf{isEquiv}(\mathsf{id}_A)$: the fiber $\mathsf{fib}_{\mathsf{id}}(a) = \sum_{x:A}(x = a)$ is contractible (the total path space from $a$, which we showed is contractible in Chapter 17). $\square$

**Theorem 1.11 (Inverse equivalence).** If $e : A \simeq B$, then $e^{-1} : B \simeq A$.

*Proof.* Extract the inverse function $g$ from the equivalence structure of $e$. Show $g$ has contractible fibers (by the symmetry of the equivalence). $\square$

**Theorem 1.12 (Composition of equivalences).** If $e_1 : A \simeq B$ and $e_2 : B \simeq C$, then $e_2 \circ e_1 : A \simeq C$.

*Proof.* The fiber of $g \circ f$ over $z$ is $\sum_{x:A}(g(f(x)) = z) \simeq \sum_{y:B}\mathsf{fib}_g(z) \times \mathsf{fib}_f(y)$... the correct argument: for each $z : C$, $\mathsf{fib}_{g \circ f}(z) \simeq \sum_{y:B} \mathsf{fib}_f(y) \times \{y' : B \mid g(y') = z\}$... use the two-out-of-three property. $\square$

**Theorem 1.13 (Two-out-of-three).** In a composable triple $A \xrightarrow{f} B \xrightarrow{g} C$: if any two of $f$, $g$, $g \circ f$ are equivalences, so is the third.

*Proof.* Use contractible fibers and fiber sequences. $\square$

## Equivalences as the Correct Notion of Sameness

The key insight is that equivalences, not bijections or functions with quasi-inverses, are the *correct* notion of sameness for types in HoTT.

**Why propositionality matters.** If $\mathsf{isEquiv}(f)$ is a proposition, then "being an equivalence" is a *property* of $f$ — either $f$ is an equivalence or it isn't, with no additional structure. This means:
- Two equivalences $(f, e)$ and $(f, e')$ with the same underlying function are automatically equal: $(f, e) = (f, e')$ because $e = e'$ (both prove the same proposition).
- The type $A \simeq B$ doesn't have "extra structure" beyond the underlying function.

This is crucial for Univalence: we want paths in the universe ($A = B$) to correspond to equivalences ($A \simeq B$), and this only works if the type of equivalences has "the right size" — which requires propositionality.

## Self-Equivalences and Automorphisms

**Definition 1.14.** The *automorphism group* of $A$ is:
$$\mathsf{Aut}(A) :\equiv A \simeq A$$

(the type of self-equivalences of $A$, viewed as a group under composition).

**Examples:**
- $\mathsf{Aut}(\mathbf{0}) = \mathbf{1}$ (only the identity)
- $\mathsf{Aut}(\mathbf{1}) = \mathbf{1}$ (only the identity)
- $\mathsf{Aut}(\mathsf{Bool}) = \mathbb{Z}/2\mathbb{Z}$ (identity and negation)
- $\mathsf{Aut}(\mathsf{Fin}(n)) \simeq S_n$ (the symmetric group on $n$ elements)

By Univalence (which we state in the next section), paths $A = A$ in the universe correspond to self-equivalences $A \simeq A$. So the loop space of the universe at $A$ is $\mathsf{Aut}(A)$: $\Omega(\mathsf{Type}, A) = \mathsf{Aut}(A)$.

This is the type-theoretic version of: the automorphism group of an object is the group of "symmetries" at that object.

## Equivalence Induction

Just as path induction (J) says: to prove something about all paths $p : a = b$, prove it for $p = \mathsf{refl}_a$ — there's an analogous principle for equivalences:

**Theorem 1.15 (Equivalence induction, informal).** To prove something about all equivalences $e : A \simeq B$, it suffices to prove it when $A = B$ and $e = \mathsf{id}_A$.

This is a consequence of Univalence (which we prove in the next section): since $A \simeq B$ corresponds to $A = B$, the J rule for paths applies and reduces to the reflexivity case.

**Corollary 1.16.** Any property of types that is stated in terms of equivalences is automatically preserved by equivalences. (You only need to check it for the identity equivalence, and then it propagates to all equivalences.)

This is a key principle of *univalent mathematics*: state all properties in terms of equivalences (not bijections or raw functions), and they'll automatically be equivalence-invariant.
