# Groups

A group is the simplest algebraic structure with a genuinely deep theory — and it appears everywhere: symmetries of geometric objects, permutations, number systems, cryptography, quantum mechanics. For this book, groups play a second role. The group axioms form a small, purely *equational* first-order theory, so group theory is a laboratory for logic itself: what is provable, what is decidable, and where exactly decidability breaks all have sharp, instructive answers here.

## The Axioms as an Equational Theory

Fix the signature $\Sigma_{\mathrm{grp}} = (\cdot,\, {}^{-1},\, e)$: a binary operation, a unary operation, a constant. The **theory of groups** is the set of universal closures of five equations:

$$
\begin{aligned}
&\text{(G1, associativity)} \quad && (x \cdot y) \cdot z = x \cdot (y \cdot z)\\
&\text{(G2, identity)} && e \cdot x = x, \qquad x \cdot e = x\\
&\text{(G3, inverses)} && x^{-1} \cdot x = e, \qquad x \cdot x^{-1} = e
\end{aligned}
$$

**Definition (Group).** A group is a $\Sigma_{\mathrm{grp}}$-structure satisfying G1–G3. It is **abelian** (commutative) if additionally $x \cdot y = y \cdot x$.

Notice what is *absent*: no $\exists$, no $\to$, no $\neg$ — only universally quantified equations. Traditional textbooks say "there exists an identity element such that..."; putting $e$ and ${}^{-1}$ directly into the signature Skolemizes those existentials away (Chapter 3). The payoff is substantial: equational classes are exactly the *varieties* of Section 4, closed under subalgebras, products, and homomorphic images, and their equational consequences can be derived in Birkhoff's five-rule calculus of equational logic.

## Examples

| Group | Operation | Identity | Order | Abelian? |
|-------|-----------|----------|-------|----------|
| $(\mathbb{Z}, +)$ | addition | $0$ | $\infty$ | Yes |
| $(\mathbb{Q}^{*}, \times)$ | multiplication | $1$ | $\infty$ | Yes |
| $(\mathbb{Z}/n\mathbb{Z}, +)$ | addition mod $n$ | $0$ | $n$ | Yes |
| $S_n$ (permutations of $n$ letters) | composition | $\mathrm{id}$ | $n!$ | No ($n \ge 3$) |
| $D_4$ (symmetries of the square) | composition | $\mathrm{id}$ | $8$ | No |
| $GL_n(\mathbb{R})$ (invertible matrices) | matrix product | $I_n$ | $\infty$ | No ($n \ge 2$) |

$D_4$ consists of four rotations and four reflections; a rotation followed by a reflection differs from the reflection followed by the rotation, so non-commutativity is visible in an eight-element group.

## First Theorems, Proved Equationally

The theorems below are chains of replacements, each licensed by a named axiom — they are literally derivations in equational logic (Section 4).

**Theorem (Uniqueness of identity).** If $e'$ satisfies $e' \cdot x = x$ and $x \cdot e' = x$ for all $x$, then $e' = e$.

*Proof.* $e' \overset{\text{G2}}{=} e' \cdot e \overset{\text{hyp}}{=} e$, using G2 for $e$ (right identity) and the left-identity property of $e'$. $\square$

**Theorem (Uniqueness of inverses).** If $a \cdot b = e$ and $b \cdot a = e$, then $b = a^{-1}$.

*Proof.*
$$
b \overset{\text{G2}}{=} e \cdot b \overset{\text{G3}}{=} (a^{-1} \cdot a) \cdot b \overset{\text{G1}}{=} a^{-1} \cdot (a \cdot b) \overset{\text{hyp}}{=} a^{-1} \cdot e \overset{\text{G2}}{=} a^{-1}. \qquad \square
$$

**Theorem (Cancellation laws).** If $a \cdot b = a \cdot c$ then $b = c$; if $b \cdot a = c \cdot a$ then $b = c$.

*Proof (left cancellation).*
$$
b \overset{\text{G2}}{=} e \cdot b \overset{\text{G3}}{=} (a^{-1} \cdot a) \cdot b \overset{\text{G1}}{=} a^{-1} \cdot (a \cdot b) \overset{\text{hyp}}{=} a^{-1} \cdot (a \cdot c) \overset{\text{G1}}{=} (a^{-1} \cdot a) \cdot c \overset{\text{G3}}{=} e \cdot c \overset{\text{G2}}{=} c.
$$
Right cancellation is symmetric. $\square$

**Theorem (Socks–shoes).** $(a \cdot b)^{-1} = b^{-1} \cdot a^{-1}$.

*Proof.* We verify that $b^{-1} \cdot a^{-1}$ is a two-sided inverse of $a \cdot b$:
$$
(a \cdot b) \cdot (b^{-1} \cdot a^{-1}) \overset{\text{G1}}{=} a \cdot \big((b \cdot b^{-1}) \cdot a^{-1}\big) \overset{\text{G3}}{=} a \cdot (e \cdot a^{-1}) \overset{\text{G2}}{=} a \cdot a^{-1} \overset{\text{G3}}{=} e,
$$
and symmetrically $(b^{-1} \cdot a^{-1}) \cdot (a \cdot b) = e$. By uniqueness of inverses, $(a \cdot b)^{-1} = b^{-1} \cdot a^{-1}$. $\square$

## Subgroups, Homomorphisms, Quotients

**Definition (Subgroup).** $H \le G$ if $H \subseteq G$ contains $e$ and is closed under $\cdot$ and ${}^{-1}$. Then $H$ is itself a group — universal axioms are inherited by substructures.

**Definition (Homomorphism, kernel).** A map $\varphi : G \to H$ with $\varphi(a \cdot b) = \varphi(a) \cdot \varphi(b)$. It follows (by cancellation) that $\varphi(e) = e$ and $\varphi(a^{-1}) = \varphi(a)^{-1}$. The **kernel** is $\ker \varphi = \{a \in G : \varphi(a) = e\}$.

**Definition (Normal subgroup).** $N \trianglelefteq G$ if $g n g^{-1} \in N$ for all $g \in G$, $n \in N$.

Kernels are always normal, and normality is exactly what makes the **quotient group** $G/N = \{gN : g \in G\}$ well defined under $(gN)(hN) = (gh)N$.

**Theorem (First isomorphism theorem).** If $\varphi : G \to H$ is a homomorphism, then $G/\ker\varphi \cong \operatorname{im}\varphi$.

*Proof sketch.* Set $K = \ker\varphi$ and define $\bar\varphi(gK) = \varphi(g)$. This is well defined and injective because $\varphi(g) = \varphi(h) \iff \varphi(h^{-1}g) = e \iff h^{-1}g \in K \iff gK = hK$; it is surjective onto $\operatorname{im}\varphi$ by construction, and a homomorphism by the definition of the coset product. $\square$

## Lagrange's Theorem

**Theorem (Lagrange).** If $G$ is a finite group and $H \le G$, then $|H|$ divides $|G|$.

*Proof.* Define $a \sim b \iff a^{-1}b \in H$. This is an equivalence relation: reflexive since $a^{-1}a = e \in H$; symmetric since $(a^{-1}b)^{-1} = b^{-1}a \in H$; transitive since $(a^{-1}b)(b^{-1}c) = a^{-1}c \in H$. The class of $a$ is the left coset $aH = \{ah : h \in H\}$. The map $h \mapsto ah$ is a bijection $H \to aH$: surjective by definition, injective by left cancellation. So the cosets partition $G$ into blocks of equal size $|H|$, and $|G| = [G : H]\,|H|$, where $[G : H]$ is the number of cosets. $\square$

**Corollary.** The order of any element $a$ (the least $n \ge 1$ with $a^n = e$) divides $|G|$; hence $a^{|G|} = e$. Applied to $(\mathbb{Z}/p\mathbb{Z})^{*}$ this yields Fermat's little theorem (Chapter 8).

**Worked example.** In $\mathbb{Z}/12\mathbb{Z}$, Lagrange restricts subgroup orders to $1, 2, 3, 4, 6, 12$ — and each occurs exactly once: $\{0\}$, $\langle 6\rangle$, $\langle 4\rangle$, $\langle 3\rangle$, $\langle 2\rangle$, $\langle 1\rangle$. The converse of Lagrange fails in general: $A_4$ has order $12$ but no subgroup of order $6$.

**Theorem (Cayley).** Every group is isomorphic to a subgroup of a symmetric group: $g \mapsto (x \mapsto gx)$ embeds $G$ into $\operatorname{Sym}(G)$, injectively by cancellation. Every group is a permutation group in disguise.

## The Logic of Groups

How much of group theory can an algorithm settle? The answer splits along two axes.

**Undecidability of the full first-order theory.** The set of first-order sentences true in all groups is undecidable (Tarski, announced 1946; Tarski–Mostowski–Robinson, *Undecidable Theories*, 1953). Mal'cev extended the method, showing undecidability for many restricted classes, including finite, nilpotent, and solvable groups. The mechanism is the one from Chapter 10: enough arithmetic can be interpreted inside suitably chosen groups.

**Decidability for abelian groups.** By contrast, the first-order theory of *abelian* groups is decidable (Szmielew, 1955). Every abelian group is determined, as far as first-order sentences can see, by a family of numerical invariants (the Szmielew invariants), and every sentence reduces effectively, via quantifier elimination in an enriched language, to a Boolean combination of statements about them.

Why does this boundary matter? Commutativity is a single equation, yet it separates classifiable structure from computational universality: where a class of structures admits classification by invariants, a decision procedure typically follows; where the class is rich enough to simulate computation, its theory is undecidable. The same watershed reappears for rings in the next section (Presburger arithmetic versus full arithmetic).

**Theorem (Novikov–Boone).** There is a finitely presented group $\langle S \mid R \rangle$ whose word problem — given a word $w$ in the generators, decide whether $w = e$ follows from the relations — is undecidable (Novikov 1955; Boone 1958).

This is orthogonal to the above: the word problem asks only about *equations*, but relative to extra relations $R$, and the relations can encode a Turing machine (Chapter 10). Without extra relations the situation is benign — equality of words in a free group is decidable by normal-form rewriting, a fact Section 4 will systematize as Knuth–Bendix completion.

## Simple Groups and Classification

A group whose only normal subgroups are $\{e\}$ and $G$ is **simple**; simple groups are the atoms from which all finite groups are built by extensions. The Classification of Finite Simple Groups (announced 1983, roughly 10,000 journal pages) lists them all: cyclic groups $\mathbb{Z}/p\mathbb{Z}$, alternating groups $A_n$ ($n \ge 5$), sixteen families of Lie type, and twenty-six sporadic groups including the Monster. A proof of this scale strains human refereeing — one major motivation for the formalization programme of Section 5, which begins with the Feit–Thompson odd-order theorem.

## Exercises
See [problems/ch19_abstract_algebra/](../../../problems/ch19_abstract_algebra/)
