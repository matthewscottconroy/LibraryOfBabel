# 1.1 Contractible Types

## The Simplest Possible Type

What's the simplest type you can imagine? One with exactly one element. You know what the element is, you know what all the paths are (just reflexivity), and there's nothing interesting happening at any higher level.

This is a *contractible type*. The name comes from topology: a contractible space is one that can be "contracted" to a single point via a continuous deformation. The circle can't be contracted (it has a hole). A disc can be contracted (push every point toward the center). A single point is trivially contractible.

In type theory, contractibility is not about continuous deformations — it's about the existence of a "center" that everything is equal to.

## The Definition

**Definition 1.1 (Contractible type).** A type $A$ is *contractible* if there exists an element $c : A$ (the *center of contraction*) together with a path from $c$ to every other element:

$$\mathsf{isContr}(A) :\equiv \sum_{c:A}\, \prod_{x:A}\, (c = x)$$

An element of $\mathsf{isContr}(A)$ is a pair $(c, h)$ where $c : A$ is the center and $h : \prod_{x:A}(c = x)$ is the *contracting homotopy*.

Let's unpack this. The contracting homotopy $h$ gives, for each $x : A$, a path $h(x) : c = x$. So from the center $c$, there is a path to every element. This doesn't say there's only one element — it says every element is *connected* to $c$ by a path. Since paths are symmetric and composable, this means every element is connected to every other element by a path.

**The unit type is contractible.** $\mathsf{isContr}(\mathbf{1})$ is witnessed by $(*, \lambda x.\, \mathsf{refl}_*)$. The center is $* : \mathbf{1}$ (the only element), and the contracting homotopy sends $*$ to $\mathsf{refl}_*$. (Since $\mathbf{1}$ has only one element, there's nothing else to worry about.)

**Topological picture.** In the simplicial set model, a contractible type corresponds to a contractible Kan complex — one that deformation retracts to a single vertex. The contracting homotopy is literally a homotopy from the identity map to the constant map at $c$.

## Contractible vs. Nonempty

A quick sanity check: contractible is stronger than nonempty. "Nonempty" says $A$ has *some* element. "Contractible" says $A$ has *exactly one element up to paths*.

More precisely: $A$ is contractible iff it has an element AND that element is unique up to a *specified* path. The paths $h(x) : c = x$ are part of the data — not just an existence statement.

This matters because in HoTT, proof relevance is the norm. Just saying "there exists an element" and "any two elements are equal" doesn't make a type contractible — you need the specific contracting homotopy as data.

## Contractible Types are Equivalent to the Unit Type

The key theorem characterizing contractible types:

**Theorem 1.2.** $A$ is contractible if and only if $A \simeq \mathbf{1}$.

*Proof.*

($\Rightarrow$) Suppose $(c, h) : \mathsf{isContr}(A)$. Define:
- $f : A \to \mathbf{1}$ by $f(a) = *$ (the unique function to the unit type)
- $g : \mathbf{1} \to A$ by $g(*) = c$ (the constant function at the center)

Check the homotopies:
- $g \circ f \sim \mathsf{id}_A$: $(g \circ f)(a) = g(*) = c$ and $\mathsf{id}_A(a) = a$, and $h(a) : c = a$ witnesses the homotopy.
- $f \circ g \sim \mathsf{id}_\mathbf{1}$: $(f \circ g)(*) = f(c) = *$ and $\mathsf{id}_\mathbf{1}(*) = *$, witnessed by $\mathsf{refl}_*$.

So $f$ is an equivalence with inverse $g$.

($\Leftarrow$) Suppose $e : A \simeq \mathbf{1}$. Let $f : A \to \mathbf{1}$ and $g : \mathbf{1} \to A$ be the underlying functions. Take $c = g(*) : A$ as the center. For any $x : A$:
- $f(x) = *$ (since $\mathbf{1}$ has only one element, and $f$ lands there)
- $f(g(*)) = *$ as well
- The left homotopy $\eta : g \circ f \sim \mathsf{id}_A$ gives $\eta(x) : g(f(x)) = x$, i.e., $g(*) = x$, i.e., $c = x$.

So $\lambda x.\, \eta(x)$ is the contracting homotopy. $\square$

**Corollary 1.3.** Any two contractible types are equivalent to each other (since they're both equivalent to $\mathbf{1}$).

In particular, "there is at most one contractible type, up to equivalence." Contractibility is not just a property of the type — it uniquely determines the type up to equivalence.

## The Total Path Space is Contractible

One of the most important examples of contractible types:

**Theorem 1.4.** For any type $A$ and any $a : A$, the total path space $\sum_{x:A} (a = x)$ is contractible.

*Proof.* The center is $(a, \mathsf{refl}_a)$. For any $(x, p) : \sum_{x:A}(a = x)$, we need a path $(a, \mathsf{refl}_a) = (x, p)$.

By the Σ-path characterization (Chapter 16, Section 5.2), a path in $\sum_{x:A}(a=x)$ from $(a, \mathsf{refl}_a)$ to $(x, p)$ consists of:
1. A path $q : a = x$ in $A$
2. A path $\mathsf{transport}^{a=(-)}(q, \mathsf{refl}_a) = p$

Take $q = p$. Then $\mathsf{transport}^{a=(-)}(p, \mathsf{refl}_a) = \mathsf{refl}_a \cdot p = p$ (by the left unit law and the transport computation in path types from Section 4). So $(p, \mathsf{left\_unit}(p))$ is the required Σ-path. $\square$

**Why this matters.** This is the "contractibility interpretation of J" that we saw in Chapter 16. The J rule says: to prove something about all $(x, p) : \sum_{x:A}(a=x)$, prove it at the center $(a, \mathsf{refl}_a)$. Contractibility is exactly why this works — you can always retract to the center.

**The J rule as a consequence.** Conversely, the J rule implies that $\sum_{x:A}(a=x)$ is contractible: the J-map sends a proof at $(a, \mathsf{refl}_a)$ to a proof at any $(x, p)$, and this is the contracting homotopy.

So J and contractibility of the total path space are two perspectives on the same underlying fact.

## Closure Properties of Contractible Types

Contractible types are closed under several constructions:

**Theorem 1.5.** If $A$ is contractible and $B$ is any type, then $A \times B \simeq B$.

*Proof.* By Theorem 1.2, $A \simeq \mathbf{1}$, so $A \times B \simeq \mathbf{1} \times B \simeq B$ (since $\mathbf{1} \times B \simeq B$ by the first projection). $\square$

**Theorem 1.6.** If $A$ is contractible and $P : A \to \mathsf{Type}$ is any family, then $\sum_{x:A} P(x) \simeq P(c)$ where $c$ is the center of contraction.

*Proof.* The center of contraction gives a homotopy between the inclusion $P(c) \hookrightarrow \sum_{x:A} P(x)$ (sending $y \mapsto (c, y)$) and the identity, via transport along the contracting homotopy. Concretely: for any $(x, y) : \sum_{x:A}P(x)$, use $h(x) : c = x$ to get $\mathsf{transport}^P(h(x)^{-1}, y) : P(c)$, giving an inverse to the inclusion. $\square$

**Theorem 1.7.** If $A$ is contractible and $P : A \to \mathsf{Type}$ is a family such that $P(c)$ is contractible (where $c$ is the center), then $\sum_{x:A} P(x)$ is contractible.

This follows from the previous theorem: $\sum_{x:A} P(x) \simeq P(c) \simeq \mathbf{1}$.

**Theorem 1.8.** If $P : A \to \mathsf{Type}$ is a family such that each $P(x)$ is contractible, then $\prod_{x:A} P(x)$ is contractible.

*Proof.* Let $c_x : P(x)$ be the center of contraction of $P(x)$. Define $f : \prod_{x:A} P(x)$ by $f(x) = c_x$. For any $g : \prod_{x:A} P(x)$, we need $f = g$. By function extensionality, it suffices to show $f(x) = g(x)$ for all $x : A$. Since $P(x)$ is contractible with center $c_x$, we have $c_x = g(x)$ (by the contracting homotopy applied to $g(x)$). So $f = g$. $\square$

## Contractibility is a Proposition

Is $\mathsf{isContr}(A)$ itself a contractible type? Not quite, but something close:

**Theorem 1.9.** $\mathsf{isContr}(A)$ is a mere proposition: any two proofs of contractibility are equal.

*Proof.* Let $(c_1, h_1)$ and $(c_2, h_2)$ be two witnesses of contractibility. We need a path $(c_1, h_1) = (c_2, h_2)$ in $\mathsf{isContr}(A)$.

By the Σ-path characterization, we need:
1. A path $p : c_1 = c_2$: use $h_1(c_2)$ or $h_2(c_1)^{-1}$ (they're equal since $A$ is contractible).
2. A dependent path over $p$ from $h_1$ to $h_2$: For any $x : A$, we need $\mathsf{transport}(p, h_1(x)) = h_2(x)$. This follows from the fact that, in the contractible type $A$, all paths from $c_2$ to $x$ are equal (since $A$ has at most one path between any two points... wait, that's not quite right unless $A$ is a proposition, not just contractible).

Actually, the proof requires more care. The key insight is: in a contractible type, all path spaces are also contractible (since the type is h-level $-2$, the next level is also trivial). We prove this by induction:

**Lemma 1.10.** If $A$ is contractible, then for any $x, y : A$, the type $x = y$ is contractible.

*Proof.* Since $A$ is contractible with center $c$ and contracting homotopy $h$, we have paths $h(x) : c = x$ and $h(y) : c = y$. The path $h(x)^{-1} \cdot h(y) : x = y$ exists. Since $A$ is contractible, the function $\prod_{z:A}(x = z)$ (the contracting homotopy from $x$) gives contractibility of each $x = z$.

More precisely: the type $\sum_{y:A}(x = y)$ is contractible (by Theorem 1.4 applied to $x$), so each fiber $x = y$ over a fixed $y$ is... hmm, not directly contractible in general.

The correct argument: for any fixed $x, y : A$ with $A$ contractible, $x = y$ is contractible. Take the center $p_0 = h(x)^{-1} \cdot h(y) : x = y$. For any $q : x = y$, we need $p_0 = q$. 

By J on $q$, reduce to the case $x = y = a$ and $q = \mathsf{refl}_a$. Then $p_0 = h(a)^{-1} \cdot h(a) = \mathsf{refl}_a$ by the right inverse law. $\square$

Returning to Theorem 1.9: using Lemma 1.10, all paths in $A$ are contractible (all higher path types are contractible), which gives us the unique dependent path we need. $\square$

## Contractibility via Fibers

One more important perspective on contractibility:

**Theorem 1.11.** A function $f : A \to B$ is an equivalence if and only if all its fibers $\mathsf{fib}_f(b) = \sum_{a:A}(f(a) = b)$ are contractible.

This is a fundamental characterization of equivalences: a function is an equivalence iff every point in the codomain has exactly one preimage (the fiber is a single-element-up-to-paths thing, i.e., contractible).

We'll return to this in Chapter 18 when discussing Univalence. But it's worth noting now: contractibility is the correct way to say "exactly one" in a homotopy-coherent setting.

## The Intuition

Contractible types are the "trivial" types — they're trivial because everything in them is equal to a single point. But "trivial" doesn't mean "useless":

- The total path space being contractible is the reason J works.
- Fibers being contractible is the definition of equivalence.
- Products with contractible types simplify (absorb into the other factor).
- Pi-types over contractible types simplify (reduce to a single fiber).

Contractibility is the type-theoretic version of "there's only one thing here, and we know what it is." It's the starting point of the h-level hierarchy — everything richer than a single point will have more complex path structure.
