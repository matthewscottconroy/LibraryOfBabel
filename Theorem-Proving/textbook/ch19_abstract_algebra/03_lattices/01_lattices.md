# Lattices and Boolean Algebras

Lattices are the algebraic distillation of order theory — and Boolean algebras are the algebraic structure of classical logic itself. This section closes the chapter's loop: having used logic to study algebra, we now watch logic *become* an algebra, with completeness reappearing as a representation theorem.

## One Structure, Two Definitions

**Definition (Lattice, order form).** A lattice is a poset $(L, \le)$ in which every pair $a, b$ has a greatest lower bound (**meet**) $a \wedge b$ and a least upper bound (**join**) $a \vee b$.

**Definition (Lattice, algebraic form).** A lattice is an algebra $(L, \wedge, \vee)$ satisfying, for all $a, b, c$:

$$
\begin{aligned}
&\text{(L1, commutativity)} \quad && a \wedge b = b \wedge a, \qquad a \vee b = b \vee a\\
&\text{(L2, associativity)} && (a \wedge b) \wedge c = a \wedge (b \wedge c), \qquad (a \vee b) \vee c = a \vee (b \vee c)\\
&\text{(L3, absorption)} && a \wedge (a \vee b) = a, \qquad a \vee (a \wedge b) = a
\end{aligned}
$$

Idempotence is derivable: $a \vee a \overset{\text{L3}}{=} a \vee (a \wedge (a \vee a)) \overset{\text{L3}}{=} a$, and dually $a \wedge a = a$.

**Theorem (Equivalence of the two definitions).** The definitions are equivalent, by mutually inverse constructions.

*Proof.* Given a poset with all binary infima and suprema, L1–L3 are immediate from the defining properties of $\inf$ and $\sup$ (e.g., $\inf(a, \sup(a,b)) = a$ since $a \le \sup(a,b)$).

Conversely, given $(L, \wedge, \vee)$ satisfying L1–L3, define $a \le b \iff a \wedge b = a$. First note $a \wedge b = a \iff a \vee b = b$: if $a \wedge b = a$ then $a \vee b = (a \wedge b) \vee b = b$ by L1, L3; the converse is dual. Now $\le$ is a partial order: *reflexive* by idempotence; *antisymmetric* since $a \wedge b = a$ and $b \wedge a = b$ give $a = b$ by L1; *transitive* since $a \wedge b = a$, $b \wedge c = b$ give $a \wedge c = (a \wedge b) \wedge c = a \wedge (b \wedge c) = a \wedge b = a$. Finally $a \wedge b$ is the infimum: $(a \wedge b) \wedge a = a \wedge b$ (L1, L2, idempotence), so $a \wedge b \le a$, and similarly $a \wedge b \le b$; and if $c \le a$, $c \le b$ then $c \wedge (a \wedge b) = (c \wedge a) \wedge b = c \wedge b = c$, so $c \le a \wedge b$. Dually $a \vee b = \sup(a,b)$. The two constructions invert each other. $\square$

The moral: an *order* can be traded for *equations*. Lattices form a variety (Section 4); "posets with suprema" do not even have an algebraic signature.

Examples: $(\mathcal{P}(X), \subseteq)$ with $\cap, \cup$; $(\mathbb{N}, \mid)$ with $\gcd, \operatorname{lcm}$; propositions ordered by provable implication, with $\wedge, \vee$ — the germ of the Lindenbaum–Tarski algebra below; the subgroups of a group under inclusion, with $H \wedge K = H \cap K$ and $H \vee K = \langle H \cup K \rangle$.

## Distributive and Modular Lattices

**Definition (Distributive lattice).** $a \wedge (b \vee c) = (a \wedge b) \vee (a \wedge c)$ for all $a,b,c$ (the dual law then follows).

**Definition (Modular lattice).** $a \le c \implies a \vee (b \wedge c) = (a \vee b) \wedge c$. Every distributive lattice is modular.

Two five-element lattices govern everything — the diamond $M_3$ and the pentagon $N_5$:

```
      ⊤              ⊤
    / | \           / \
   a  b  c         a   \
    \ | /          |    c
      ⊥            b   /
                    \ /
  M_3 (diamond)      ⊥    N_5 (pentagon)
```

**Worked example.** The Klein four-group $V = \mathbb{Z}/2 \times \mathbb{Z}/2$ has exactly three subgroups of order 2; its subgroup lattice is $M_3$. Distributivity fails there: with $a, b, c$ the three atoms, $a \wedge (b \vee c) = a \wedge \top = a$, but $(a \wedge b) \vee (a \wedge c) = \bot \vee \bot = \bot$.

**Theorem (Dedekind).** A lattice is modular iff it has no sublattice isomorphic to $N_5$.

**Theorem (Birkhoff).** A lattice is distributive iff it has no sublattice isomorphic to $M_3$ or $N_5$.

(Statements only; the proofs are careful case analyses.) The *normal* subgroups of any group always form a modular lattice; $M_3$ above shows they need not form a distributive one.

## Complete Lattices and Fixed Points

**Definition (Complete lattice).** A poset in which *every* subset $S$ has an infimum $\bigwedge S$ and a supremum $\bigvee S$. Examples: $\mathcal{P}(X)$; $[0,1]$; every finite lattice; the congruence lattice of any algebra (Section 4).

**Theorem (Knaster–Tarski).** Let $L$ be a complete lattice and $f : L \to L$ monotone ($x \le y \Rightarrow f(x) \le f(y)$). Then $f$ has a least fixed point $\mu f = \bigwedge \{x : f(x) \le x\}$ and a greatest fixed point $\nu f = \bigvee \{x : x \le f(x)\}$; indeed the set of all fixed points is itself a complete lattice.

*Proof (least fixed point).* Let $P = \{x : f(x) \le x\}$ (the *prefixed points*; $\top \in P$, so $P \neq \emptyset$) and $p = \bigwedge P$. For each $x \in P$: $p \le x$, so $f(p) \le f(x) \le x$ by monotonicity. Thus $f(p)$ is a lower bound of $P$, so $f(p) \le p$, i.e., $p \in P$. Applying $f$ again, $f(f(p)) \le f(p)$, so $f(p) \in P$, hence $p \le f(p)$. Therefore $f(p) = p$. Any fixed point $q$ satisfies $f(q) \le q$, so $q \in P$ and $p \le q$: $p$ is least. The claim for $\nu f$ is dual; the lattice-of-fixed-points claim is an exercise. $\square$

This theorem is a workhorse across the book: the $\mu$-calculus fixpoints that model checkers compute are exactly $\mu f$ and $\nu f$ over the complete lattice $\mathcal{P}(\text{States})$ (Chapter 14); Kripke's fixed-point construction of a self-referential truth predicate applies it to a monotone jump operator (Chapter 18); dataflow analyses in compilers compute least fixed points over finite lattices of program facts.

## Boolean Algebras

**Definition (Boolean algebra).** A distributive lattice with bottom $\bot$, top $\top$, and a complement operation $\neg$ satisfying $a \wedge \neg a = \bot$ and $a \vee \neg a = \top$. All axioms are equations: Boolean algebras form a variety.

Examples: $\mathcal{P}(X)$ with $\cap, \cup$, set complement; the clopen subsets of a topological space; and the two-element algebra $\mathbb{B} = \{0, 1\}$ — the truth values of classical logic, with $\wedge, \vee, \neg$ the truth tables of Chapter 2.

## The Lindenbaum–Tarski Algebra

Let $\mathrm{Form}$ be the propositional formulas over variables $P$, and define $\varphi \equiv \psi \iff\; \vdash \varphi \leftrightarrow \psi$. Provable equivalence is a *congruence* for the connectives (Section 4's notion): if $\varphi \equiv \varphi'$ and $\psi \equiv \psi'$ then $\varphi \wedge \psi \equiv \varphi' \wedge \psi'$, and so on. So the operations descend to equivalence classes.

**Theorem (Lindenbaum–Tarski).** $B = \mathrm{Form}/{\equiv}$, with $[\varphi] \wedge [\psi] = [\varphi \wedge \psi]$, $\neg[\varphi] = [\neg \varphi]$, $\top = [p \to p]$, $\bot = [\neg(p \to p)]$, is a Boolean algebra, in which $[\varphi] = \top \iff\; \vdash \varphi$ and $[\varphi] \le [\psi] \iff\; \vdash \varphi \to \psi$. $\square$

Provability has literally become an algebraic fact. This pays off immediately:

**Theorem (Completeness of propositional logic, algebraic proof).** If $\vDash \varphi$ then $\vdash \varphi$.

*Proof sketch.* Suppose $\nvdash \varphi$. Then $[\varphi] \neq \top$, so $[\neg\varphi] \neq \bot$. Every nonzero element of a Boolean algebra belongs to some **ultrafilter** $U$ (a maximal proper filter; existence via Zorn's lemma — the Boolean prime ideal theorem). Define a valuation by $v(p) = 1 \iff [p] \in U$. Ultrafilter properties ($[\alpha] \wedge [\beta] \in U$ iff both are in $U$; $\neg[\alpha] \in U$ iff $[\alpha] \notin U$) give, by induction on formulas, $\hat v(\psi) = 1 \iff [\psi] \in U$. Since $[\neg\varphi] \in U$, we get $\hat v(\varphi) = 0$, so $\nvDash \varphi$. $\square$

The same ultrafilter construction, applied to an arbitrary Boolean algebra, yields:

**Theorem (Stone representation, 1936).** Every Boolean algebra $B$ is isomorphic to a field of sets: $b \mapsto \{U \in \mathrm{Ult}(B) : b \in U\}$ embeds $B$ into $\mathcal{P}(\mathrm{Ult}(B))$. Topologized, $B$ is the algebra of clopen sets of its *Stone space*, and the correspondence is a full duality between Boolean algebras and compact totally disconnected Hausdorff spaces.

Abstract axioms, concrete models — the prototype of representation theorems throughout mathematics.

## Heyting Algebras and Intuitionistic Logic

**Definition (Heyting algebra).** A bounded lattice with an operation $\to$ satisfying the residuation law: $c \le (a \to b) \iff c \wedge a \le b$. Define $\neg a = a \to \bot$.

The canonical example: the open sets of a topological space $X$, with $U \to V = \operatorname{int}((X \setminus U) \cup V)$ and $\neg U = \operatorname{int}(X \setminus U)$. Take $X = \mathbb{R}$ and $U = \mathbb{R} \setminus \{0\}$: then $\neg U = \emptyset$ and $\neg\neg U = \mathbb{R} \neq U$, while $U \vee \neg U = U \neq \top$. Double negation elimination and excluded middle fail — exactly as in intuitionistic logic (Chapters 5 and 11). Heyting algebras are to intuitionistic logic what Boolean algebras are to classical: the Lindenbaum–Tarski algebra of intuitionistic propositional logic is the free Heyting algebra, and $\varphi$ is intuitionistically provable iff valid in every Heyting algebra (equivalently, in the opens of every topological space).

| Logic | Algebraic model |
|-------|----------------|
| Classical propositional | Boolean algebra |
| Intuitionistic propositional | Heyting algebra |
| Modal S4 | interior algebra |
| Linear logic | $*$-autonomous structures |

## Exercises
See [problems/ch19_abstract_algebra/](../../../problems/ch19_abstract_algebra/)
