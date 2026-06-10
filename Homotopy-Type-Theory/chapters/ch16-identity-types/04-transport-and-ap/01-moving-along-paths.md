# 4.1 Transport and ap: Moving Along Paths

## The Challenge of Dependent Types

In simple type theory, if $a = b$ then $f(a) = f(b)$ for any function $f : A \to B$. This is the substitution principle — replacing equal terms gives equal results.

In dependent type theory, we have type *families* $P : A \to \mathsf{Type}$. If $a = b$, then $P(a)$ and $P(b)$ are the same *type* (by Leibniz). But what does that mean for *elements*? If $x : P(a)$, we should get something in $P(b)$, but $P(a)$ and $P(b)$ are generally different types — there's no obvious way to "move" $x$ from one to the other.

The answer is *transport*: using the path $p : a = b$ as a guide, we can "move" elements of $P(a)$ to elements of $P(b)$.

## Transport

**Theorem 4.1 (Transport).** For any type family $P : A \to \mathsf{Type}$ and any path $p : a =_A b$, there is a function:
$$\mathsf{transport}^P(p, -) : P(a) \to P(b)$$

**Construction via J:** Apply J to $p$ with:
- Motive: $C(b, p) = P(a) \to P(b)$
- Base case: $C(a, \mathsf{refl}_a) = P(a) \to P(a)$, using the identity function $\mathsf{id}_{P(a)}$

So: $\mathsf{transport}^P(\mathsf{refl}_a, x) \equiv x$ for all $x : P(a)$.

**Computation rule:** $\mathsf{transport}^P(\mathsf{refl}_a, x) \equiv x$ (definitionally)

## Transport is an Equivalence

**Theorem 4.2.** For any path $p : a = b$, the transport function $\mathsf{transport}^P(p) : P(a) \to P(b)$ is an equivalence.

*Proof:* The inverse is $\mathsf{transport}^P(p^{-1}) : P(b) \to P(a)$.

For the composition $\mathsf{transport}^P(p^{-1}) \circ \mathsf{transport}^P(p) = \mathsf{id}_{P(a)}$: by J on $p$, reduce to the reflexivity case, where both transports are identity and the composition is identity. $\square$

**The fibration picture.** A type family $P : A \to \mathsf{Type}$ is a fibration (in the model-categorical sense). Transport is *parallel transport* along the path $p$. In differential geometry, parallel transport along a curve moves tangent vectors from one fiber to another; here, transport moves elements of $P(a)$ to $P(b)$ along the path $p$.

The fact that transport is an equivalence is the categorical statement that the fibers of a fibration are all "isomorphic" along paths — the fibration is locally trivial.

## Transport Computations

Let's compute transport for specific type families.

**Example 4.3 (Equality type).** $P(x) = (a = x)$ for a fixed $a : A$. Then:
$$\mathsf{transport}^{a=(-)}\!(p, q) = q \cdot p \quad \text{(or } p^{-1} \cdot q\text{, depending on conventions)}$$
for $q : a = b$ and $p : b = c$.

*Verification:* When $p = \mathsf{refl}_b$: $\mathsf{transport}^{a=(-)}(\mathsf{refl}_b, q) = q \cdot \mathsf{refl}_b = q$. ✓

**Example 4.4 (Constant family).** If $P(x) = B$ (a constant family), then $\mathsf{transport}^B(p, b) = b$ (unchanged), since there's no dependence on the path.

**Example 4.5 (Function type).** For $P(x) = (B(x) \to C(x))$ where $B, C : A \to \mathsf{Type}$:
$$\mathsf{transport}^{B \to C}(p, f) = \lambda y \mapsto \mathsf{transport}^C(p, f(\mathsf{transport}^B(p^{-1}, y)))$$

Move backward along $p$ in $B$, apply $f$, then move forward along $p$ in $C$.

**Example 4.6 (Vectors).** For $P(n) = \mathsf{Vec}\,A\,n$ (vectors of length $n$):
$$\mathsf{transport}^{\mathsf{Vec}\,A}(p, v) = \text{"reindex } v \text{ according to } p"$$

If $p : m = n$ (equality of natural numbers), then $\mathsf{transport}$ converts a vector of length $m$ to a vector of length $n$.

## The Dependent Action on Paths (apd)

For ordinary functions $f : A \to B$, we have $\mathsf{ap}_f : (a = b) \to (f(a) = f(b))$.

For dependent functions $f : \prod_{x:A} P(x)$, we need something more: given $p : a = b$, we need to compare $f(a) : P(a)$ and $f(b) : P(b)$. But these live in different types! The comparison must use transport.

**Definition 4.7 (apd — Dependent action on paths).** For $f : \prod_{x:A} P(x)$ and $p : a =_A b$:
$$\mathsf{apd}_f(p) : \mathsf{transport}^P(p, f(a)) =_{P(b)} f(b)$$

This says: if you transport $f(a)$ along $p$ to $P(b)$, you get the same thing as $f(b)$.

**Construction via J:** Apply J to $p$ with:
- Motive: $C(b, p) = (\mathsf{transport}^P(p, f(a)) = f(b))$
- Base case: $\mathsf{transport}^P(\mathsf{refl}_a, f(a)) = f(a)$, which holds by the computation rule for transport

So: $\mathsf{apd}_f(\mathsf{refl}_a) \equiv \mathsf{refl}_{f(a)}$.

**The topological picture.** For a section $f$ of a fibration $P \to A$ (a dependent function $f : \prod_{x:A} P(x)$), the fact that $f$ is a section means: if you parallel-transport $f(a)$ along any path $p$ to $b$, you get exactly $f(b)$. The section is "compatible" with the parallel transport. That's $\mathsf{apd}_f(p)$.

## The Non-Dependent Case: ap

For a non-dependent function $f : A \to B$ (equivalently, a dependent function where $P$ is the constant family $P(x) = B$), the dependent $\mathsf{apd}$ specializes to the ordinary $\mathsf{ap}$:

**Definition 4.8 (ap).** For $f : A \to B$ and $p : a = b$:
$$\mathsf{ap}_f(p) : f(a) =_B f(b)$$

**Construction via J:** Apply J to $p$ with:
- Motive: $C(b, p) = (f(a) = f(b))$
- Base case: $f(a) = f(a)$, using $\mathsf{refl}_{f(a)}$

Computation: $\mathsf{ap}_f(\mathsf{refl}_a) \equiv \mathsf{refl}_{f(a)}$.

**Theorem 4.9 (ap is a functor).** $\mathsf{ap}_f$ is a "groupoid homomorphism":
1. $\mathsf{ap}_f(p \cdot q) = \mathsf{ap}_f(p) \cdot \mathsf{ap}_f(q)$ (preserves concatenation)
2. $\mathsf{ap}_f(p^{-1}) = (\mathsf{ap}_f(p))^{-1}$ (preserves inversion)
3. $\mathsf{ap}_{\mathsf{id}}(p) = p$ (identity function preserves paths)
4. $\mathsf{ap}_{g \circ f}(p) = \mathsf{ap}_g(\mathsf{ap}_f(p))$ (composition is compatible)

*Proof of (1):* By J on $p$ first (base case: $\mathsf{ap}_f(\mathsf{refl}_a \cdot q) = \mathsf{ap}_f(q) = \mathsf{refl}_{f(a)} \cdot \mathsf{ap}_f(q)$). Then J on $q$. In the end, both sides reduce to $\mathsf{refl}_{f(a)}$. $\square$

**Interpretation.** The properties of $\mathsf{ap}$ say: every function in HoTT is "automatically continuous" — it preserves paths, path composition, path inversion, etc. There's no need to separately prove continuity; the type theory enforces it.

## Homotopies are Natural Transformations

**Definition 4.10 (Homotopy).** For $f, g : A \to B$, a homotopy $H : f \sim g$ is:
$$H : \prod_{a:A} f(a) =_B g(a)$$

A homotopy is a path from $f(a)$ to $g(a)$ for each $a$, varying continuously (in the sense that the path function is a dependent function).

**Theorem 4.11 (Naturality of homotopies).** For $H : f \sim g$ and $p : a = b$:
$$\mathsf{ap}_g(p) \cdot H(b) = H(a) \cdot \mathsf{ap}_f(p)$$

This says: the two ways to get from $f(a)$ to $g(b)$ agree — either apply $H(a)$ then $\mathsf{ap}_g(p)$, or apply $\mathsf{ap}_f(p)$ then $H(b)$.

*Proof:* By J on $p$. Reduces to $\mathsf{ap}_g(\mathsf{refl}_a) \cdot H(a) = H(a) \cdot \mathsf{ap}_f(\mathsf{refl}_a)$, i.e., $\mathsf{refl} \cdot H(a) = H(a) \cdot \mathsf{refl}$. Both sides equal $H(a)$ by the unit laws. $\square$

**The naturality square.** Naturality says that the square:

$$\begin{array}{ccc}
f(a) & \xrightarrow{H(a)} & g(a) \\
\mathsf{ap}_f(p) \downarrow & & \downarrow \mathsf{ap}_g(p) \\
f(b) & \xrightarrow{H(b)} & g(b)
\end{array}$$

commutes (up to a 2-path). This is the type-theoretic version of "natural transformations commute with morphisms" — every homotopy is "natural."

## Summary

| Operation | Type | Defined by | Computes on |
|---|---|---|---|
| $\mathsf{transport}^P(p)$ | $P(a) \to P(b)$ | J on $p$ | $\mathsf{refl} \mapsto \mathsf{id}$ |
| $\mathsf{ap}_f(p)$ | $f(a) = f(b)$ | J on $p$ | $\mathsf{refl} \mapsto \mathsf{refl}$ |
| $\mathsf{apd}_f(p)$ | $\mathsf{transport}(p, f(a)) = f(b)$ | J on $p$ | $\mathsf{refl} \mapsto \mathsf{refl}$ |
| Homotopy naturality | $\mathsf{ap}_g(p) \cdot H(b) = H(a) \cdot \mathsf{ap}_f(p)$ | J on $p$ | — |

Transport and ap are the two fundamental ways paths interact with the rest of type theory. Transport moves elements along paths in dependent types; ap shows that functions are "continuous." Together, they ensure that the entire type-theoretic universe is homotopy-coherent.
