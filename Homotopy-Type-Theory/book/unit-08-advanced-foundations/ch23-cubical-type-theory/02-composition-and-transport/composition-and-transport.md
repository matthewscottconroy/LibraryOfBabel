# 23.2 Composition and Transport

## The Problem Paths-as-Functions Create

When paths are functions from the interval, you gain computational power — but you also face a new problem. Concatenating two paths $p : a =_A b$ and $q : b =_A c$ into a path $a =_A c$ cannot be done by simple function combination. The interval $\mathbb{I}$ has no "if-then-else" for interior points, no split point that would let you say "use $p$ on the first half and $q$ on the second." The interval is not $[0,1]$ with a total order you can bisect.

The solution is *composition* — the operation that fills open boxes. Path concatenation is a special case of filling a 2-dimensional open box: you have the bottom, left, and right faces, and you want the top. The composition operation `hcomp` fills the missing face. This operation is not just a convenience for concatenation; it is the Kan condition — the computational heart of why cubical type theory is a homotopy theory.

## Partial Elements, Revisited

Before writing down `hcomp`, we need the precise notion of the data it receives.

An *open box* in dimension 2 is a square with one face missing. Concretely, in context with dimension variables $i, j : \mathbb{I}$, an open box consists of:

- A *tube*: for each position $j : \mathbb{I}$, a partial element of $A$ defined on the faces $(i = 0)$ and $(i = 1)$. This gives the left and right walls of the box.
- A *base*: an element at $j = 0$ (the bottom).

The missing face is at $j = 1$ (the top). The composition operation fills it.

More generally, the data for `hcomp` is:
- A face formula $\phi$ (specifying which faces are provided)
- A *tube*: a function $(j : \mathbb{I}) \to [\phi \vdash A]$ — at each $j$, a partial element defined on the face $\phi$
- A *base*: an element $a_0 : A$ such that $a_0 = u(0)$ whenever $\phi = 1$

The coherence condition on the base is essential: the tube at $j = 0$ must agree with the base on the face $\phi$.

## The `hcomp` Operation

**Definition.** Given $A : \mathsf{Type}$, $\phi : \mathbb{F}$, $u : (j : \mathbb{I}) \to [\phi \vdash A]$, and $a_0 : A$ with $a_0 = u(0)$ whenever $\phi = 1$:

$$\mathsf{hcomp}^A_\phi(u, a_0) : A$$

satisfying:
- **Face condition**: When $\phi = 1$: $\mathsf{hcomp}^A_\phi(u, a_0) = u(1)$
- **The "h" stands for "homogeneous"**: the type $A$ does not change across the interval. For heterogeneous composition (type changes), we use `comp`, which combines `hcomp` with `transp`.

**Path concatenation** is the canonical example. For $p : a =_A b$ and $q : b =_A c$:

$$p \cdot q \;:\equiv\; \lambda i. \;\mathsf{hcomp}^A_{i=0}\!\left(\lambda j. \;[i = 0 \mapsto p(j)],\; q(i)\right)$$

Read this as: at each point $i$, we fill an open box. The tube at $j$ is empty unless $i = 0$, in which case it is $p(j)$ (running along the path $p$). The base is $q(i)$.

Check at $i = 0$: the face condition gives $\mathsf{hcomp}^A_{0=0}(\ldots) = u(1) = p(1) = b$. But also $q(0) = b$. So the path starts correctly.

Check at $i = 1$: the tube is everywhere empty ($i \neq 0$), so $\phi = 0$, and `hcomp` with an empty tube returns the base: $q(1) = c$. Correct endpoint.

## The Kan Condition and Type Formers

`hcomp` must be defined for every type in the theory. This requires specifying, for each type former, how composition works on it. This is the Kan condition: every open box in every type has a filler.

**For $\Pi$-types** ($\Pi_{x:A} B(x)$, including function types as the non-dependent case):

$$\mathsf{hcomp}^{\Pi_{x:A} B(x)}_\phi(u, f_0)(a) = \mathsf{hcomp}^{B(a)}_\phi\!\left(\lambda j. \;u(j)(a),\; f_0(a)\right)$$

Apply `hcomp` pointwise: to compose a tube of functions at argument $a$, just compose the values at $a$.

**For $\Sigma$-types** ($\Sigma_{x:A} B(x)$):

$$\mathsf{hcomp}^{\Sigma_{x:A} B(x)}_\phi(u, (a_0, b_0)) = \left(\mathsf{hcomp}^A_\phi(u.1, a_0),\; \mathsf{hcomp}^{B(\hat{a})}_\phi(u.2, b_0)\right)$$

where $\hat{a}$ is the composed first component. The second component is transported over the path induced by composing the first.

**For inductive types** ($\mathbb{N}$, $\mathsf{Bool}$, etc.): The composition is trivial when $\phi = 0$ (return the base) or $\phi = 1$ (return $u(1)$). For these types, `hcomp` is essentially a case analysis that pushes the composition through the constructors. Since $\mathbb{N}$ has no parameters depending on the interval, composition is in fact always trivial: $\mathsf{hcomp}^{\mathbb{N}}_\phi(u, n_0) = n_0$ when $\phi = 0$.

**For the universe** $\mathsf{Type}$: This is the hard case. Composition in the universe requires the Glue type (Section 3), because composing a tube of types along equivalences produces a new type.

## Filling and the Kan Condition Geometrically

The Kan condition is easiest to understand geometrically. Imagine working in a simplicial or cubical set. The *Kan condition* says: any open box (a cube with one face missing) can be filled. The missing face always exists.

For a 1-cube (a path) with endpoint data: trivially, the endpoint is just the point.

For a 2-cube (a square) with three sides given (left, right, bottom): the top side can always be constructed. This is exactly `hcomp` in dimension 2.

For a 3-cube with five faces given: the sixth face exists. This is `hcomp` in dimension 3.

All of these are instances of the same operation with the same type, just with different numbers of dimension variables in scope. The recursive structure of the computation rules ensures consistency across dimensions.

In the categorical semantics, types in CCHM are interpreted as *fibrant cubical sets* — cubical sets satisfying the Kan condition. The `hcomp` operation is what witnesses fibrancy. Every type in CCHM is automatically fibrant because composition is a primitive that must be provided.

## The `transp` Operation

Composition (`hcomp`) works in a *fixed* type. But HoTT requires transporting along *paths of types* — moving an element from type $A(0)$ to type $A(1)$ when $A : \mathbb{I} \to \mathsf{Type}$ is a changing family.

**Definition.** Given a path of types $A : \mathbb{I} \to \mathsf{Type}$, a face formula $\phi$ (indicating when $A$ is constant), and an element $a_0 : A(0)$:

$$\mathsf{transp}^A_\phi(a_0) : A(1)$$

satisfying:
- **Triviality when constant**: When $\phi = 1$: $\mathsf{transp}^A_\phi(a_0) = a_0$
- **Type-specific computation**: When $\phi = 0$: `transp` uses the structure of the type former to compute the transported element

The $\phi$ parameter specifies "regions where $A$ is already constant, so transport is trivial." This is needed for the composition rules: when computing `hcomp` for a type family, you need to transport along partial paths, and in regions where the type is constant, transport is the identity.

**For $\mathbb{N}$**: $\mathsf{transp}^{\lambda i. \mathbb{N}}_\phi(n) = n$. The natural numbers don't depend on the interval, so transport is always the identity.

**For function types** $\Pi_{x:A(i)} B(i,x)$: Transport forward in $B$, after transporting the argument backward in $A$:

$$\mathsf{transp}^{\Pi_{x:A} B(x)}_\phi(f)(b) = \mathsf{transp}^{B(1, \cdot)}_\phi\!\left(f\!\left(\mathsf{transp}^{A^{-}}_\phi(b)\right)\right)$$

where $A^-(i) = A(1 - i)$ is the reversed path of types. To apply a transported function to an argument in $A(1)$, you first pull the argument back to $A(0)$ using transport in the reverse direction, apply the function there, then push the result forward.

**For $\Sigma$-types**: Transport each component, using the transported first component to adjust the type of the second:

$$\mathsf{transp}^{\Sigma_{x:A} B(x)}_\phi((a_0, b_0)) = \left(\mathsf{transp}^A_\phi(a_0),\; \mathsf{transp}^{\lambda i. B(i, a_i)}_\phi(b_0)\right)$$

where $a_i = \mathsf{transp}^{A|_{[0,i]}}_\phi(a_0)$ is the partial transport of $a_0$ up to time $i$.

## The `comp` Operation: Combining Both

The full *composition* operation `comp` combines `hcomp` and `transp` into a single primitive for heterogeneous boxes — open boxes where the type changes across the box:

$$\mathsf{comp}^A_\phi(u, a_0) : A(1)$$

where $A : \mathbb{I} \to \mathsf{Type}$ is a path of types, $u : (j : \mathbb{I}) \to [\phi \vdash A(j)]$, and $a_0 : A(0)$ with $a_0 = u(0)$ when $\phi = 1$.

The relation: $\mathsf{comp}^A_\phi(u, a_0) = \mathsf{hcomp}^{A(1)}_\phi\!\left(\lambda j. \;\mathsf{transp}^{\lambda k. A(j \vee k)}_\phi(u(j)),\; \mathsf{transp}^A_\phi(a_0)\right)$

In practice, `comp` is the most useful single operation: it handles the general case of filling an open heterogeneous box.

## Groupoid Coherences via Cube Algebra

With `hcomp` available, the groupoid laws for paths become computable objects. Crucially, some laws hold definitionally (by computation) and some hold propositionally (as paths, not definitional equalities). Understanding which is which is important for working in Cubical Agda.

**Left unit**: $\text{refl} \cdot p \sim p$ (propositional, witnessed by the connection $\lambda i\, j. \, p(i \wedge j)$)

**Right unit**: $p \cdot \text{refl} \sim p$ (propositional, witnessed by $\lambda i\, j. \, p(i \vee j)$)

**Associativity**: $(p \cdot q) \cdot r \sim p \cdot (q \cdot r)$ (propositional, witnessed by a 3-dimensional composition)

**Inverse**: $p \cdot \text{sym}(p) \sim \text{refl}$ (propositional, requires a 2-cube using the complement)

The key insight: these witnesses are *explicit functions* out of $\mathbb{I}^2$ (or $\mathbb{I}^3$), not abstract elements of an identity type that you produce via J-induction. The proof of $\text{refl} \cdot p = p$ in Book HoTT requires path induction. In Cubical Agda, it is the bivariate function $\lambda i\, j. \, p(i \wedge j)$. Period.

## The Connection Types in Detail

The *connection lemmas* are the most elegant computational witnesses in cubical type theory.

**Left connection**: For $p : a =_A b$, the term $\lambda i\, j. \, p(i \wedge j)$ is a 2-cube with:
- Face $i = 0$: $\lambda j. \, p(0 \wedge j) = \lambda j. \, p(0) = \lambda j. \, a$ — constant at $a$
- Face $i = 1$: $\lambda j. \, p(1 \wedge j) = \lambda j. \, p(j) = p$ — the path $p$
- Face $j = 0$: $\lambda i. \, p(i \wedge 0) = \lambda i. \, p(0) = \lambda i. \, a$ — constant at $a$
- Face $j = 1$: $\lambda i. \, p(i \wedge 1) = \lambda i. \, p(i) = p$ — the path $p$

This 2-cube witnesses: the square has $\text{refl}_a$ on two adjacent sides and $p$ on the other two. From this, we extract a path from $\text{refl} \cdot p$ to $p$ by filling the appropriate face.

**Right connection**: $\lambda i\, j. \, p(i \vee j)$ — the dual construction for the right unit law.

These are not just clever tricks. They are instances of a general principle: the De Morgan algebra on $\mathbb{I}$ encodes enough algebraic structure to represent all the coherences of an $\infty$-groupoid as explicit polynomial expressions in the interval variables.

## Canonicity: The Theorem That Makes Everything Worth It

**Theorem (Canonicity for CCHM Cubical Type Theory).** Every closed term $t : \mathbb{N}$ is definitionally equal to a numeral $\overline{n}$ for some $n \geq 0$.

This theorem is the payoff for all the work of designing `hcomp`, `transp`, and the specific computation rules for each type former. It says the theory is *computationally complete* in the strongest sense: every element of the natural numbers that can be proved to exist can be evaluated to an explicit number.

The proof proceeds by *normalization*: showing that the reduction system — all the `hcomp` and `transp` rules, the $\beta$ and $\eta$ rules for each type former, the interval algebra — is strongly normalizing and confluent. Every sequence of reductions terminates in a unique normal form. For closed terms of type $\mathbb{N}$, the only normal forms are the numerals $0, 1, 2, \ldots$

The Brunerie number computation is the most spectacular demonstration: a term of type $\mathbb{Z}$ defined by a complex chain of synthetic homotopy computations involving $\pi_4(S^3)$, which in Book HoTT is stuck but in Cubical Agda evaluates to $-2$ in a few seconds. The machine runs the proof.
