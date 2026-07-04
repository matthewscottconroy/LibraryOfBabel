# Rings and Fields

Groups have one operation; rings have two, interlocked by distributivity. Rings are also where the logic of algebra turns dramatic: the first-order theory of the complex numbers is decidable, that of the reals is decidable — and that of the integers, seemingly the most elementary structure of all, is not.

## The Axioms

Fix the signature $\Sigma_{\mathrm{ring}} = (+,\, \cdot,\, -,\, 0,\, 1)$ with arities $2, 2, 1, 0, 0$. A **ring** is a $\Sigma_{\mathrm{ring}}$-structure satisfying:

$$
\begin{aligned}
&\text{(R1)} \quad && (R, +, -, 0) \text{ is an abelian group: } (x+y)+z = x+(y+z),\; x+y = y+x,\; x+0 = x,\; x+(-x) = 0\\
&\text{(R2)} && (R, \cdot, 1) \text{ is a monoid: } (x \cdot y) \cdot z = x \cdot (y \cdot z),\; 1 \cdot x = x,\; x \cdot 1 = x\\
&\text{(R3, distributivity)} && x \cdot (y + z) = x \cdot y + x \cdot z, \qquad (x + y) \cdot z = x \cdot z + y \cdot z
\end{aligned}
$$

A **commutative ring** additionally satisfies $x \cdot y = y \cdot x$. Once again the axioms are pure equations — rings and commutative rings are varieties in the sense of Section 4.

**Proposition.** $0 \cdot x = 0$ for all $x$.

*Proof.* $0 \cdot x \overset{\text{R1}}{=} (0 + 0) \cdot x \overset{\text{R3}}{=} 0 \cdot x + 0 \cdot x$; adding $-(0 \cdot x)$ to both sides (R1) gives $0 = 0 \cdot x$. $\square$

This tiny proof shows distributivity earning its keep: it is the only axiom linking $+$ to $\cdot$, and every interaction between the two operations factors through it.

## Domains and Fields

**Definition (Unit).** $u \in R$ is a **unit** if $uv = vu = 1$ for some $v$. The units form a group $R^{\times}$ under multiplication.

**Definition (Zero divisor).** $a \neq 0$ is a **zero divisor** if $ab = 0$ for some $b \neq 0$.

**Definition (Integral domain).** A commutative ring with $1 \neq 0$ and no zero divisors.

**Definition (Field).** A commutative ring with $1 \neq 0$ in which every nonzero element is a unit. Every field is an integral domain: if $ab = 0$ and $a \neq 0$, then $b = a^{-1}ab = 0$.

| Ring | Type | Notes |
|------|------|-------|
| $\mathbb{Z}$ | integral domain, not a field | $\mathbb{Z}^{\times} = \{1, -1\}$ |
| $\mathbb{Z}/6\mathbb{Z}$ | not a domain | $2 \cdot 3 = 0$: zero divisors |
| $R[x]$ (polynomials) | domain iff $R$ is | never a field: $x$ has no inverse |
| $\mathbb{Q}, \mathbb{R}, \mathbb{C}$ | fields | characteristic $0$ |
| $\mathbb{F}_p = \mathbb{Z}/p\mathbb{Z}$ | finite field | one field of order $p^k$ for each prime power |

A logical observation before proceeding: the field axiom "every nonzero element has an inverse" is
$$\forall x\, (x \neq 0 \to \exists y\; x \cdot y = 1),$$
which uses negation and an existential quantifier. This is not a defect of formulation — Section 4 proves that *no* equational axiomatization of fields exists.

## Ideals and Quotient Rings

**Definition (Ideal).** $I \subseteq R$ is an **ideal** if it is an additive subgroup and $rx \in I$, $xr \in I$ for all $r \in R$, $x \in I$. Examples: $n\mathbb{Z} \subseteq \mathbb{Z}$; the multiples of a fixed polynomial in $\mathbb{Q}[x]$.

Ideals play the role normal subgroups played for groups: they are exactly the kernels of **ring homomorphisms** (maps preserving $+$, $\cdot$, $0$, $1$), and they are exactly what one can quotient by. The **quotient ring** $R/I$ has elements $a + I$ with the induced operations, well defined because $I$ absorbs multiplication.

**Theorem (First isomorphism theorem for rings).** If $\varphi : R \to S$ is a ring homomorphism, then $\ker\varphi = \{a : \varphi(a) = 0\}$ is an ideal and $R/\ker\varphi \cong \operatorname{im}\varphi$. $\square$ (The proof mirrors the group case; Section 4 exposes both as one theorem about congruences.)

For example, $\mathbb{Z}/n\mathbb{Z}$ is literally the quotient of $\mathbb{Z}$ by the ideal $n\mathbb{Z}$.

## Prime Moduli

**Theorem.** $\mathbb{Z}/n\mathbb{Z}$ is a field if and only if $n$ is prime.

*Proof.* ($\Leftarrow$) Let $p$ be prime and $[a] \neq [0]$, so $p \nmid a$ and hence $\gcd(a, p) = 1$. By Bézout's identity (Chapter 8) there are integers $u, v$ with $au + pv = 1$; reducing mod $p$, $[a][u] = [1]$, so $[a]$ is a unit. ($\Rightarrow$) If $n = 1$ the ring is trivial ($1 = 0$), not a field. If $n = ab$ with $1 < a, b < n$, then $[a][b] = [0]$ with $[a], [b] \neq [0]$ — zero divisors — so $\mathbb{Z}/n\mathbb{Z}$ is not even a domain. $\square$

**Worked example.** Compute $5^{-1}$ in $\mathbb{F}_7$. The extended Euclidean algorithm gives $7 = 1 \cdot 5 + 2$ and $5 = 2 \cdot 2 + 1$, so $1 = 5 - 2 \cdot 2 = 5 - 2(7 - 5) = 3 \cdot 5 - 2 \cdot 7$. Hence $5^{-1} = 3$ in $\mathbb{F}_7$; check: $5 \cdot 3 = 15 \equiv 1 \pmod 7$.

## Characteristic

**Definition (Characteristic).** $\operatorname{char}(R)$ is the least $n \ge 1$ with $\underbrace{1 + \cdots + 1}_{n} = 0$, or $0$ if no such $n$ exists. Thus $\operatorname{char}(\mathbb{Z}/n\mathbb{Z}) = n$, $\operatorname{char}(\mathbb{Q}) = 0$, $\operatorname{char}(\mathbb{F}_p) = p$.

**Proposition.** The characteristic of an integral domain is $0$ or prime.

*Proof.* If $\operatorname{char}(R) = n = ab$ with $1 < a, b < n$, then $(a \cdot 1)(b \cdot 1) = n \cdot 1 = 0$, so one factor is $0$, contradicting minimality of $n$. $\square$

## Ordered and Real Closed Fields

**Definition (Ordered field).** A field with a linear order $\le$ such that $x \le y \Rightarrow x + z \le y + z$, and $0 \le x, y \Rightarrow 0 \le xy$. Examples: $\mathbb{Q}$, $\mathbb{R}$. Non-example: $\mathbb{C}$ — in an ordered field every square is $\ge 0$, but $i^2 = -1 < 0$.

**Definition (Real closed field).** An ordered field in which every positive element has a square root and every odd-degree polynomial has a root. $\mathbb{R}$ and the real algebraic numbers are real closed; the real closed fields are exactly the ordered fields elementarily equivalent to $\mathbb{R}$.

## The Decidability Map

Now the logical payoff. Write $\mathrm{Th}(\mathcal{M})$ for the set of first-order sentences true in $\mathcal{M}$.

**Algebraically closed fields.** Let $\mathrm{ACF}_p$ be the field axioms plus "every nonconstant polynomial has a root" (an axiom schema, one sentence per degree) plus the characteristic-$p$ axioms. Each $\mathrm{ACF}_p$ ($p$ zero or prime) is **complete and decidable**, and admits **quantifier elimination**. The completeness argument is a model-theoretic gem (Chapter 9): an algebraically closed field of given characteristic is determined up to isomorphism by its transcendence degree, so $\mathrm{ACF}_p$ is categorical in every uncountable cardinality; having no finite models, it is complete by the Łoś–Vaught test; being complete and recursively axiomatized, it is decidable. In particular $\mathrm{Th}(\mathbb{C}; +, \cdot) = \mathrm{ACF}_0$ is decidable. Quantifier elimination here is Chevalley's theorem in geometric clothing: projections of constructible sets are constructible.

**Real closed fields.** $\mathrm{Th}(\mathbb{R}; +, \cdot, \le) = \mathrm{RCF}$ is decidable (Tarski, *A Decision Method for Elementary Algebra and Geometry*, 1951), again via quantifier elimination. The title is exact: through Cartesian coordinates, all of elementary Euclidean geometry becomes a fragment of RCF, so there is an algorithm deciding every first-order geometric statement (Chapter 20). Practical algorithms came later (Collins's cylindrical algebraic decomposition, 1975) and power modern nonlinear-arithmetic solvers.

**The integers.** $\mathrm{Th}(\mathbb{Z}; +, \cdot)$ is undecidable — this is the Gödel–Church barrier (Chapter 10). Julia Robinson (1949) showed $\mathbb{Z}$ is first-order definable in $\mathbb{Q}$, so $\mathrm{Th}(\mathbb{Q}; +, \cdot)$ is undecidable too. Yet remove multiplication and decidability returns: $\mathrm{Th}(\mathbb{Z}; +, \le)$ is **Presburger arithmetic**, decidable (Presburger, 1929); remove addition instead and $\mathrm{Th}(\mathbb{N}; \cdot)$ (Skolem arithmetic) is also decidable. Addition and multiplication are each tame alone; together they encode sequences via Gödel's $\beta$-function, hence computation. The boundary is not "how complicated the structure looks": $\mathbb{C}$ and $\mathbb{R}$ are decidable while their subring $\mathbb{Z}$ is not.

**Hilbert's tenth problem.** Hilbert (1900) asked for an algorithm to decide whether a Diophantine equation $p(x_1, \dots, x_n) = 0$ has integer solutions — a *purely existential* question, far weaker than deciding all of $\mathrm{Th}(\mathbb{Z}; +, \cdot)$. Matiyasevich (1970), completing work of Davis, Putnam, and Julia Robinson, proved that every recursively enumerable set is Diophantine; since some r.e. sets are undecidable (Chapter 10), no such algorithm exists. Even the $\exists$-fragment of integer arithmetic is undecidable — while over $\mathbb{R}$ and $\mathbb{C}$ the *entire* first-order theory is decidable. Whether Hilbert's tenth problem is decidable over $\mathbb{Q}$ remains open.

## Exercises
See [problems/ch19_abstract_algebra/](../../../problems/ch19_abstract_algebra/)
