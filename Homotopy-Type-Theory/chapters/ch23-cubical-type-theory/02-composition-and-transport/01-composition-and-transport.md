# 2.1 Composition and Transport

## The Need for Composition

If paths are functions $\mathbb{I} \to A$, then path *reversal* (using $\sim$) and *evaluation at a point* are immediate. But path *concatenation* is not — you can't just glue two functions together without some additional structure.

Consider: $p : a =_A b$ and $q : b =_A c$. We want a path $a =_A c$. Naively, you might try:

$$\lambda i. \text{if } i < 0.5 \text{ then } p(2i) \text{ else } q(2i - 1)$$

but this doesn't make sense — the interval $\mathbb{I}$ is not $[0,1]$ with a linear order, and there's no "if-then-else" for interval values.

The correct approach is *composition* — the operation that fills an *open box* in a type. Concatenation is a special case of composition.

## Partial Elements

Before defining composition, we need the notion of a *partial element*: a term that is only defined on part of the cube.

**Partial types.** For a face formula $\phi$ and a type $A$, write $[\phi \vdash A]$ for the type of partial elements — terms of type $A$ defined only when $\phi = 1$.

More concretely: a partial element $u : [\phi \vdash A]$ is a term in context extended with the constraint $\phi = 1$. Outside the face $\phi$, $u$ is not defined.

**Notation.** We write partial elements using case syntax:
$$u = [i = 0 \mapsto a, i = 1 \mapsto b]$$
for an element defined at both endpoints of dimension $i$, sending $i = 0$ to $a$ and $i = 1$ to $b$.

**Example.** A path $p : a =_A b$ can be viewed as a "full" element (defined everywhere on $\mathbb{I}$), but we can also extract partial elements:

- The partial element $[i = 0 \mapsto a]$: defined only at the left endpoint
- The partial element $[i = 0 \mapsto p(0), i = 1 \mapsto p(1)] = [i = 0 \mapsto a, i = 1 \mapsto b]$: defined at both endpoints

## The `hcomp` Composition Operation

The *composition* operation $\mathsf{hcomp}$ is the primitive that solves the box-filling problem.

**Intuition.** Imagine an open box in dimension 2: a square missing its top face. You have:
- Left face: a path $l : p(0) =_A q(0)$
- Right face: a path $r : p(1) =_A q(1)$
- Bottom face: a path $p : a =_A b$

The composition $\mathsf{hcomp}$ fills the missing top face: it produces a path $q : a' =_A b'$ for suitable endpoints.

**Formal definition.** Given:
- $A : \mathsf{Type}$ (the type we're working in)
- $\phi$ : a face formula
- $u : (i : \mathbb{I}) \to [\phi \vdash A]$ : the "tube" (partial element for each $i$)
- $a_0 : A$ with $a_0 = u(0)$ whenever $\phi = 1$ (the "base")

The composition produces:
$$\mathsf{hcomp}_\phi^A(u, a_0) : A$$

satisfying:
- *Face condition*: When $\phi = 1$, $\mathsf{hcomp}_\phi^A(u, a_0) = u(1)$
- *Base condition*: The term $a_0$ is the "bottom" and the composition gives the "top"

**Path concatenation.** For $p : a =_A b$ and $q : b =_A c$:

$$p \cdot q \;:\equiv\; \lambda i. \mathsf{hcomp}_{i=0}^A(\lambda j. [i = 0 \mapsto p(j)], q(i))$$

This says: at each point $i$ in the interval, fill a 1-dimensional open box. The "tube" is empty at all points except $i = 0$, where it runs along $p$. The base at $j = 0$ is $q(i)$, which runs along $q$.

Let's check the endpoints:
- At $i = 0$: $\mathsf{hcomp}_{i=0}^A(\lambda j. [i=0 \mapsto p(j)], q(0))$. Since $i = 0$, $\phi = 1$, so the result is $u(1) = p(1) = b$. ✓
- At $i = 1$: $\mathsf{hcomp}_{i=0}^A(\lambda j. [i=0 \mapsto p(j)], q(1))$. Since $i = 1 \neq 0$, $\phi = 0$, and the tube is empty. The composition is just the base $q(1) = c$. ✓

So concatenation is $\lambda i. (p \cdot q)(i)$ starting at $a$ and ending at $c$.

## Kan Filling

The *Kan condition* for a type $A$ says that all open boxes in $A$ can be filled. This is exactly the condition that `hcomp` is always defined.

In classical simplicial homotopy theory, a Kan complex has the property that horn inclusions $\Lambda^n_k \hookrightarrow \Delta^n$ lift. The Kan condition in cubical type theory is the analogous condition for cubical horns.

**The composition operation for each type former.** For `hcomp` to be well-defined on all types, we need to specify how it acts on each type constructor:

**For function types $A \to B$:**
$$\mathsf{hcomp}_\phi^{A \to B}(u, f_0)(a) = \mathsf{hcomp}_\phi^B(\lambda i. u(i)(a), f_0(a))$$
Apply $\mathsf{hcomp}$ pointwise.

**For $\Sigma$-types $\Sigma_{x:A} B(x)$:**
$$\mathsf{hcomp}_\phi^{\Sigma_{x:A} B(x)}(u, (a_0, b_0)) = (\mathsf{hcomp}^A_\phi(u.1, a_0), \mathsf{hcomp}^{B(...)}_\phi(u.2, b_0))$$
Compose each component separately (with some care about the type of the second component depending on the first).

**For $\Pi$-types $\Pi_{x:A} B(x)$:** Similar to function types — pointwise composition.

**For the universe $\mathsf{Type}$:**
$$\mathsf{hcomp}_\phi^\mathsf{Type}(u, A_0)$$
produces a new type that "glues together" the partial family $u$ with the base $A_0$. This is done using the `Glue` type constructor (Section 3).

**For inductive types (like $\mathbb{N}$, $\mathsf{Bool}$):**
If $\phi = 0$ (no face constraint), then $\mathsf{hcomp}^A_0(u, a_0) = a_0$.
If $\phi = 1$ (full constraint), then $\mathsf{hcomp}^A_1(u, a_0) = u(1)$.
For intermediate cases, the composition uses the rigid structure of inductive types: since constructors are injective and disjoint, the composition must stay within the constructor applied to composed components.

## Transport

Transport along a path of types is a primitive in cubical type theory:

$$\mathsf{transp}^A_\phi(a_0) : A(1)$$

where:
- $A : \mathbb{I} \to \mathsf{Type}$ is a path of types
- $\phi$ is a formula saying when $A$ is constant (when transport should be trivial)
- $a_0 : A(0)$ is the element to transport

**Computation rules for `transp`:**

When $\phi = 1$ (the type family is constant):
$$\mathsf{transp}^A_1(a_0) = a_0$$

This is the "transport along a constant path is the identity" rule.

**For specific type formers:**

**Function types:**
$$\mathsf{transp}^{A \to B}_\phi(f)(b) = \mathsf{transp}^B_\phi(\lambda i. B(i), f(\mathsf{transp}^{A(\sim \cdot)}_\phi(b)))$$
Transport forward in $B$, after transporting the argument backward in $A$.

**$\Sigma$-types:**
$$\mathsf{transp}^{\Sigma_{x:A} B(x)}_\phi((a_0, b_0)) = (\mathsf{transp}^A_\phi(a_0), \mathsf{transp}^{B(\mathsf{transp}^A_\phi(...))}_\phi(b_0))$$
Transport the first component, then transport the second along the path induced by the first.

**Identity type:**
$$\mathsf{transp}^{p =_A q}_\phi(r_0)$$
for paths $p, q : \mathbb{I} \to A$ and $r_0 : p(0) =_{A(0)} q(0)$. This is more complex and requires 2-dimensional composition.

**Inductive types ($\mathbb{N}$, etc.):**
$$\mathsf{transp}^{\mathbb{N}}_\phi(n) = n$$
Since $\mathbb{N}$ doesn't depend on any universe (it has no parameters), transport is the identity — there's nothing to transform.

## The Relationship Between `hcomp` and `transp`

The two primitives `hcomp` and `transp` are related:

- `transp` transports along a path of *types* (from $A(0)$ to $A(1)$)
- `hcomp` fills an open box within a *fixed* type

Every transport can be expressed as a composition:
$$\mathsf{transp}^A_\phi(a_0) = \mathsf{hcomp}^{A(1)}_{(i \text{ const or } \phi)}(\lambda j. \mathsf{transp}^{A_{...}}_\phi(a_0), a_0)$$
(this is not quite the right formula, but the idea is that transporting along a path of types is a special case of filling a box where the type is changing).

In Cubical Agda, both `transp` and `hcomp` are primitives, and the library defines higher-level operations in terms of them.

## Path Groupoid Laws

With `hcomp` and `transp`, we can prove all the groupoid laws for paths. Crucially, these are *definitional equalities* for the axioms and *path equalities* for the coherences.

**Left unit:** $\mathsf{refl} \cdot p = p$

In cubical type theory, using the meet $\wedge$:
$$(\mathsf{refl} \cdot p)(i) = \mathsf{hcomp}_{i=0}^A(\lambda j. [i=0 \mapsto a], p(i))$$
At $i = 0$: tube is full, result is $a = p(0)$.
At $i = 1$: tube is empty, result is $p(1) = b$.
But this doesn't immediately give $p(i)$ for intermediate $i$.

In fact, $\mathsf{refl} \cdot p$ is *homotopic* to $p$ but not *definitionally equal* to $p$. The homotopy is given by:
$$H = \lambda i j. p(i \wedge j) : (i j : \mathbb{I}) \to A$$
Check:
- $H(0, j) = p(0 \wedge j) = p(0) = a$ ✓
- $H(1, j) = p(1 \wedge j) = p(j)$ ✓
- $H(i, 0) = p(i \wedge 0) = p(0) = a$ ✓
- $H(i, 1) = p(i \wedge 1) = p(i)$ ✓

So $H$ gives a path $\mathsf{refl} \cdot p \Rightarrow p$, i.e., `refl ∙ p ≡ p` in Cubical Agda.

**Associativity:** Similar 2-dimensional argument.

**Inverses:** $p \cdot \mathsf{sym}(p) = \mathsf{refl}$. Uses the De Morgan complement and 2-dimensional `hcomp`.

The beautiful aspect of cubical type theory: these groupoid coherences, which are complex axioms in Book HoTT, become *computable operations* in cubical type theory. The proofs are explicit functions out of the cube $\mathbb{I}^2$, not abstract elements of an identity type.

## Connection Types

Using meet and join, cubical type theory can define *connection* types — 2-dimensional cubes that witness specific coherences:

**Left connection:** For $p : a =_A b$:
$$\mathsf{lConn}(p) = \lambda i j. p(i \wedge j) : a =_{a =_A b} p$$

This is a 2-cube with:
- $\mathsf{lConn}(p)(0, j) = p(0 \wedge j) = p(0) = a$
- $\mathsf{lConn}(p)(1, j) = p(j)$
- $\mathsf{lConn}(p)(i, 0) = p(i \wedge 0) = p(0) = a$
- $\mathsf{lConn}(p)(i, 1) = p(i)$

So it witnesses that the constant path at $a$ followed by $p$ is homotopic to $p$.

**Right connection:** $\lambda i j. p(i \vee j)$, using join.

Connection types are the computational witnesses for the left and right unit laws. They replace the abstract path induction arguments of Book HoTT with concrete cubical constructions.

## Canonicity

With these computation rules for `hcomp` and `transp`, every closed term of type $\mathbb{N}$ can be evaluated:

**Theorem (Canonicity).** In CCHM cubical type theory, every closed term $t : \mathbb{N}$ is definitionally equal to a numeral $\overline{n}$ for some $n \geq 0$.

The proof is by *normalization*: show that the reduction system for cubical type theory (including all the `hcomp` and `transp` rules for each type former) is strongly normalizing and confluent. Then closed terms of type $\mathbb{N}$ reduce to unique normal forms, which must be numerals.

This theorem validates the computational interpretation: proofs in cubical type theory can be extracted as programs, and those programs run correctly.
