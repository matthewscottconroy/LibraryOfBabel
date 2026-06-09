# Part I — The Language of Mathematics
## Chapters 1–3: Logic, Sets, Relations, Functions, and Cardinality

---

### What This Part Establishes

Before any algebraic structure can be studied, mathematics needs a language — a way to form statements, reason about them, and build structures from nothing. This part establishes:

1. **How to write and verify a proof** — the syntax and semantics of mathematical argument
2. **What a set is** — the universe in which all algebraic objects live
3. **What a function is** — the maps between algebraic objects
4. **How to compare infinite sizes** — the tool for understanding existence proofs
5. **Zorn's Lemma** — the existence principle behind bases, closures, and maximal ideals

No prior mathematical knowledge is assumed beyond arithmetic.

---

### Internal Dependency Map

```
Chapter 1 (Logic and Proof)
    │
    ▼
Chapter 2 (Sets, Relations, Functions)
    │
    └──► Chapter 3 (Cardinality and Choice)
```

Everything in Parts II–XII depends on Part I.

---

## Chapter 1 — Logic and the Art of Proof

**What it establishes:** The formal language of mathematics and the methods of proof.

---

### 1.1 Propositional Logic

**What it establishes:** How to form mathematical statements and determine their truth.

**1.1.1 Propositions, Connectives, and Truth Tables**
A *proposition* is a statement that is definitively true or false. We introduce the five connectives ($\neg$,$\land$,$\lor$,$\Rightarrow$,$\Leftrightarrow$) and their truth tables. Every complex statement in mathematics is built from these. The truth table for$P \Rightarrow Q$ — counterintuitive at first — is analyzed in detail: a false hypothesis makes any implication true.

**1.1.2 Logical Equivalence and Tautologies**
Two statements are *logically equivalent* if they have the same truth value under every assignment. We establish the key equivalences: contrapositive ($P \Rightarrow Q \equiv \neg Q \Rightarrow \neg P$), double negation, De Morgan's laws. A *tautology* is true regardless — these become the rules of inference.

**1.1.3 Implication: Direct Proof, Contrapositive, Contradiction**
Three strategies for proving $P \Rightarrow Q$, chosen based on which makes the argument cleanest: assume$P$ and derive$Q$ (direct); assume$\neg Q$ and derive$\neg P$ (contrapositive); assume$P \land \neg Q$ and derive a contradiction. The equivalence of these strategies is logical, not a trick.

---

### 1.2 Predicate Logic and Quantifiers

**What it establishes:** How to make statements about all or some elements of a set.

**1.2.1 Predicates and the Universal and Existential Quantifiers**
A *predicate* $P(x)$ is a statement depending on a variable.$\forall x \in S, P(x)$ asserts$P$ holds for every element;$\exists x \in S, P(x)$ asserts it holds for at least one. The formal notation is introduced alongside the informal reading.

**1.2.2 Negating Quantified Statements**
The negation laws: $\neg(\forall x, P(x)) \equiv \exists x, \neg P(x)$ and$\neg(\exists x, P(x)) \equiv \forall x, \neg P(x)$. These are the engine of counterexample arguments and the foundation for understanding what it means to disprove a theorem.

**1.2.3 Nested Quantifiers and Order Dependence**
$\forall x \, \exists y, R(x,y)$ (for every input, some output exists) differs fundamentally from$\exists y \, \forall x, R(x,y)$ (a single output works for all inputs). The second is strictly stronger. This distinction drives the definitions of pointwise continuity vs. uniform continuity, pointwise convergence vs. uniform convergence, and appears throughout analysis.

---

### 1.3 Mathematical Proof Strategies

**What it establishes:** Four proof templates that cover essentially all mathematical arguments.

**1.3.1 Direct Proof and Chain Arguments**
Assume hypotheses; apply definitions and previously established facts; arrive at the conclusion. The key discipline: every step must be justified by a named principle. We see how algebraic manipulations, divisibility arguments, and geometric arguments all fit this pattern.

**1.3.2 Proof by Contradiction and Proof by Contrapositive**
*Contradiction:* Assume the negation of the conclusion; derive $\bot$ (a false statement). Used when the conclusion is a non-existence result (no solution, no such element) or when the negation has rich algebraic structure. *Contrapositive:* Often cleaner when the hypothesis is harder to use directly but the negation of the conclusion is concrete. The irrationality of$\sqrt{2}$ is the archetype.

**1.3.3 Mathematical Induction: Weak, Strong, and Structural Forms**
*Weak induction:* Base case + inductive step $P(k) \Rightarrow P(k+1)$ establishes$P(n)$ for all$n \geq n_0$. *Strong induction:* The inductive step assumes$P(j)$ for all$j \leq k$ — useful when$P(k+1)$ depends on multiple predecessors (e.g., prime factorization). *Structural induction:* Induction on a well-founded relation, not just$\mathbb{N}$; used for trees, derivation trees, and later for ordinals. The well-ordering principle of$\mathbb{N}$ is established as equivalent.

**1.3.4 Existence and Uniqueness Arguments**
*Existence* can be constructive (exhibit the object) or non-constructive (contradiction from non-existence). *Uniqueness:* Assume two objects satisfy the property; show they are equal. The $\exists!$ quantifier combines both. These templates recur throughout algebra: unique identity, unique inverse, unique representative of a coset.

---

### 1.4 The Axiomatic Method

**What it establishes:** The philosophical framework in which all of mathematics operates.

**1.4.1 Primitive Terms, Axioms, and Theorems**
Every axiomatic system begins with *undefined terms* and *axioms* — statements taken as true without proof. All other statements are *theorems* derived from these. In group theory: sets and elements are primitive; the group axioms are accepted; every property of groups is derived. This section motivates why we do this: to make reasoning reliable and to see which properties actually require which axioms.

**1.4.2 Consistency, Completeness, and Independence**
A system is *consistent* if no contradiction is derivable; *complete* if every true statement is provable; *independent* if no axiom follows from the others. We preview Gödel's theorems: no sufficiently powerful consistent system is complete. Independence is illustrated by the parallel postulate in geometry — a preview of the independence of the Axiom of Choice.

**1.4.3 Why Rigor Matters: Famous Failures of Intuition**
Several historical examples where intuition failed catastrophically: "proofs" of the four-color theorem with errors undiscovered for years; false conjectures about primes; the Banach–Tarski paradox. Rigor is not pedantry — it is the only reliable protection against errors that otherwise propagate through research for decades.

---

## Chapter 2 — Sets, Relations, and Functions

**What it establishes:** The foundational universe of mathematical objects and the structure of maps between them.

---

### 2.1 Naive Set Theory

**What it establishes:** How to build and manipulate collections of objects.

**2.1.1 Sets and Membership; Set-Builder Notation**
A *set* is a collection of objects; $x \in A$ asserts membership. Set-builder notation$\{x \mid P(x)\}$ defines sets by properties. The standard sets$\mathbb{N}$,$\mathbb{Z}$,$\mathbb{Q}$,$\mathbb{R}$,$\mathbb{C}$ are introduced as primitive examples. The empty set$\emptyset = \{\}$ has no elements.

**2.1.2 Operations: Union, Intersection, Difference, Complement**
The four basic operations on sets, with their algebraic laws: commutativity, associativity, distributivity, and De Morgan's laws (now for sets, mirroring propositional logic). Venn diagrams are useful for intuition; algebraic proofs are required for rigor. Indexed unions and intersections $\bigcup_{i \in I} A_i$ and$\bigcap_{i \in I} A_i$ are introduced.

**2.1.3 Power Sets and Cartesian Products**
The *power set* $\mathcal{P}(A)$ contains all subsets of$A$; if$|A| = n$ then$|\mathcal{P}(A)| = 2^n$. The *Cartesian product*$A \times B = \{(a,b) \mid a \in A, b \in B\}$ is the set of ordered pairs; generalized to$A_1 \times \cdots \times A_n$. These constructions generate most algebraic objects.

**2.1.4 Russell's Paradox and Its Lesson**
The "set of all sets not containing themselves" is not a set. This shows that the naive comprehension principle ($\{x \mid P(x)\}$ always forms a set) is inconsistent. The lesson: we cannot collect everything into a set freely. This motivates the restricted comprehension of ZFC (Chapter 55).

---

### 2.2 Relations

**What it establishes:** A uniform framework for all comparison, similarity, and ordering concepts.

**2.2.1 Relations as Subsets of Cartesian Products**
A *relation* from $A$ to$B$ is a subset$R \subseteq A \times B$. This definition subsumes all pairwise comparisons: equality, divisibility, congruence, "is a subgroup of," "is less than." Writing$aRb$ is shorthand for$(a,b) \in R$.

**2.2.2 Reflexivity, Symmetry, Antisymmetry, Transitivity**
The four key properties that a relation on $A$ may possess, with examples illustrating each combination. Reflexivity ($aRa$ always), symmetry ($aRb \Rightarrow bRa$), antisymmetry ($aRb$ and$bRa$ $\Rightarrow a = b$), transitivity ($aRb$ and$bRc$ $\Rightarrow aRc$). Non-examples are as important as examples.

**2.2.3 Equivalence Relations and the Partition Theorem**
A relation that is reflexive, symmetric, and transitive is an *equivalence relation*. The *equivalence class* of $a$ is$[a] = \{b \mid aRb\}$. The fundamental theorem: equivalence classes partition the set (pairwise disjoint, covering), and conversely every partition defines an equivalence relation. This bijection — partitions$\leftrightarrow$ equivalence relations — is used constantly in algebra (congruence classes, cosets, orbits).

**2.2.4 Partial Orders and Total Orders**
A relation that is reflexive, antisymmetric, and transitive is a *partial order* (poset). If every pair is comparable, it is a *total order*. Examples: divisibility on $\mathbb{N}$ (partial),$\leq$ on$\mathbb{R}$ (total), inclusion on$\mathcal{P}(S)$ (partial). Posets reappear throughout algebra: subgroups ordered by inclusion, ideals ordered by inclusion, prime ideals ordered by inclusion (the "spectrum").

---

### 2.3 Quotient Sets

**What it establishes:** How to construct new sets by identifying equivalent elements — the template for all quotient constructions.

**2.3.1 Equivalence Classes and the Canonical Projection**
The *equivalence class* $[a]_R$ of an element$a$ under$\sim$ is the set of all elements equivalent to$a$. The *canonical projection*$\pi: A \to A/{\sim}$ sends$a \mapsto [a]$; it is always surjective.

**2.3.2 The Quotient Set $A/{\sim}$**
The *quotient set* $A/{\sim} = \{[a] \mid a \in A\}$ has the equivalence classes as its elements. It is a new set, smaller than$A$, that "collapses" equivalent elements to a point. This is the universal construction that gives rise to$\mathbb{Z}/n\mathbb{Z}$, quotient groups, quotient rings, quotient vector spaces, and quotient modules.

**2.3.3 Well-Definedness: When Does a Map Descend to the Quotient?**
A map $f: A \to B$ *descends to the quotient* (defines a map$\bar{f}: A/{\sim} \to B$) precisely when$a \sim a' \Rightarrow f(a) = f(a')$. This is the key verification in every quotient construction. Violating it means the map is not well-defined — a common source of errors.

---

### 2.4 Functions

**What it establishes:** The precise notion of a deterministic rule from inputs to outputs.

**2.4.1 Functions as Special Relations; Domain, Codomain, Image**
A *function* $f: A \to B$ is a relation where each$a \in A$ has exactly one$b \in B$ with$(a,b) \in f$. The *domain* is$A$, the *codomain* is$B$, the *image* is$f(A) = \{f(a) \mid a \in A\} \subseteq B$. The distinction between codomain and image is essential: a function is surjective when image = codomain.

**2.4.2 Injections, Surjections, and Bijections**
A function is *injective* (one-to-one) if distinct inputs give distinct outputs; *surjective* (onto) if every codomain element is achieved; *bijective* if both. Bijections are the "sameness" morphisms for sets — they witness that two sets have the same elements up to renaming.

**2.4.3 Composition and Inverses**
The *composition* $(g \circ f)(a) = g(f(a))$ is associative. The *identity function*$\mathrm{id}_A(a) = a$ is the unit for composition.$f$ has a *left inverse* iff$f$ is injective; a *right inverse* iff$f$ is surjective (this requires the Axiom of Choice for infinite sets); a *two-sided inverse* iff$f$ is bijective.

**2.4.4 Direct and Inverse Images**
For $S \subseteq A$: the *direct image*$f(S) = \{f(s) \mid s \in S\}$. For$T \subseteq B$: the *inverse image*$f^{-1}(T) = \{a \in A \mid f(a) \in T\}$. The inverse image of a union is the union of inverse images (and similarly for intersections, complements) — but the direct image only distributes over unions, not intersections.

---

## Chapter 3 — Cardinality and the Axiom of Choice

**What it establishes:** A rigorous theory of infinite sizes; the powerful existence tool used throughout algebra.

---

### 3.1 Finite and Infinite Sets

**What it establishes:** The bijection-based definition of size that works for infinite sets.

**3.1.1 Equinumerosity: Bijections as the Measure of Size**
Two sets have the *same cardinality* if there is a bijection between them: $|A| = |B|$ iff$A \sim B$. For finite sets this recovers counting. The relation$\sim$ is an equivalence relation on sets.

**3.1.2 Finite Sets and Counting**
A set is *finite* if it is equinumerous with $\{1, 2, \ldots, n\}$ for some$n$; its cardinality is$n$. Key properties: subsets of finite sets are finite; finite unions of finite sets are finite. The pigeonhole principle is a direct consequence.

**3.1.3 Countable and Uncountable Sets**
A set is *countably infinite* if it is equinumerous with $\mathbb{N}$. *Countable* = finite or countably infinite. A set is *uncountable* if it is infinite but not countable. Countable sets can be listed as a sequence$a_1, a_2, a_3, \ldots$; uncountable sets cannot.

---

### 3.2 Cantor's Theory

**What it establishes:** The genuinely infinite hierarchy of infinite sets.

**3.2.1 Countability of $\mathbb{Z}$ and$\mathbb{Q}$: Diagonal Enumeration**
Despite $\mathbb{Z}$ and$\mathbb{Q}$ seeming "larger" than$\mathbb{N}$, both are countable. The proofs use diagonal enumeration: an explicit bijection listing all elements. The crucial insight: it is possible to be infinite in multiple "directions" yet still countable.

**3.2.2 Uncountability of $\mathbb{R}$: The Diagonal Argument**
Cantor's diagonal argument: any purported enumeration of reals in $(0,1)$ can be used to construct a real not on the list. Therefore no such enumeration exists and$\mathbb{R}$ is uncountable. This argument is reused in logic (Gödel's theorems) and in the proof that there are more functions$\mathbb{N} \to \{0,1\}$ than natural numbers.

**3.2.3 Cantor's Theorem: $|A| < |\mathcal{P}(A)|$ for Any$A$**
For any set $A$, there is no surjection$A \to \mathcal{P}(A)$. Proof: given any function$f: A \to \mathcal{P}(A)$, the set$D = \{a \in A \mid a \notin f(a)\}$ is not in the range of$f$ (diagonal construction). This gives a strictly increasing sequence$|\mathbb{N}| < |\mathcal{P}(\mathbb{N})| < |\mathcal{P}(\mathcal{P}(\mathbb{N}))| < \cdots$ — infinitely many infinite cardinalities.

**3.2.4 The Schröder–Bernstein Theorem**
If $|A| \leq |B|$ and$|B| \leq |A|$ (injections in both directions), then$|A| = |B|$ (a bijection exists). This is non-trivial: it says that to show two sets have equal cardinality, it suffices to find injections in both directions rather than an explicit bijection. The proof explicitly constructs the bijection.

---

### 3.3 The Axiom of Choice and Its Equivalents

**What it establishes:** The most powerful and most controversial axiom; the source of existence results throughout algebra.

**3.3.1 The Axiom of Choice: Statement and Motivation**
For any collection $\{A_i\}_{i \in I}$ of non-empty sets, there exists a *choice function*$f$ with$f(i) \in A_i$ for all$i$. For finite collections this is obvious (induction). For infinite collections it is not — there may be no rule to specify which element to pick from each set. The Axiom of Choice asserts such a selection always exists, even when no rule can be given.

**3.3.2 Zorn's Lemma: Statement and Template for Use**
*Zorn's Lemma:* If every chain (totally ordered subset) in a poset $P$ has an upper bound in$P$, then$P$ has a maximal element. Equivalent to AC. The standard template: (1) Define the poset of "partial solutions" ordered by extension; (2) Show every chain has an upper bound (take the union); (3) Conclude a maximal element exists; (4) Show maximality forces the element to be a "complete solution."

**3.3.3 The Well-Ordering Theorem**
Every set can be well-ordered (totally ordered so every non-empty subset has a minimum). Equivalent to AC. Implies that any two cardinalities are comparable. The well-ordering of $\mathbb{R}$ cannot be constructed explicitly.

**3.3.4 Using Zorn's Lemma in Algebra: Bases, Maximal Ideals, Algebraic Closures**
Four foundational applications: (1) Every vector space has a basis — the poset of independent sets, maximal = basis. (2) Every ring has a maximal ideal — the poset of proper ideals, maximal = maximal ideal. (3) Every field has an algebraic closure — more complex but same template. (4) Every group has a maximal proper subgroup (in the finitely generated case). These are the algebraic existence theorems that cannot be proved without AC.
