# Applied Exercises

Set theory provides the mathematical language for describing collections, correspondences, and cardinality — concepts that appear with surprising concreteness across computer science and engineering. A relational database table is a set of tuples. A type system assigns a set of valid values to each variable. A cryptographic key space is a set whose size determines security. A sorting algorithm's complexity depends on whether its domain can be well-ordered. The exercises below make these connections explicit, using the set-theoretic tools of this chapter — relations, functions, cardinality, Cantor's theorem, the Axiom of Choice, and ordinal induction — to analyze real problems in concrete domains.

---

## Exercise B.1: Database Schemas as Relational Set Theory
*Domain: Database Systems / Software Engineering*

**Setup:** A relational database is built on the mathematical notion of a relation. Each *table* with columns of types $D_1, D_2, \ldots, D_n$ is a finite subset of the Cartesian product $D_1 \times D_2 \times \cdots \times D_n$. Each row is a tuple $(d_1, d_2, \ldots, d_n)$ with $d_i \in D_i$. This is not a metaphor — it is the definition given by Codd in his original 1970 paper that created relational databases.

Consider a database with the following schema:
- `Student`: a subset of $\mathsf{ID} \times \mathsf{Name} \times \mathsf{Year}$
- `Course`: a subset of $\mathsf{CID} \times \mathsf{Title} \times \mathsf{Dept}$
- `Enrollment`: a subset of $\mathsf{ID} \times \mathsf{CID} \times \mathsf{Grade}$

**Questions:**
1. A *functional dependency* is a set-theoretic property: $X \to Y$ (read: $X$ determines $Y$) holds in a relation $R \subseteq D_1 \times \cdots \times D_n$ if for all tuples $t_1, t_2 \in R$, $\pi_X(t_1) = \pi_X(t_2)$ implies $\pi_Y(t_1) = \pi_Y(t_2)$ (where $\pi_X$ is the projection onto columns $X$). Explain why a *primary key* is precisely a minimal set of columns $K$ such that $K \to \{1, \ldots, n\}$ (i.e., $K$ determines all columns). Express this in the set-theoretic language of functions: what function does a primary key define, and what property must it have?

2. A *join* of two relations $R \subseteq A \times B$ and $S \subseteq B \times C$ (joined on the shared column $B$) is:
   $$R \bowtie S = \{(a, b, c) \mid (a, b) \in R \wedge (b, c) \in S\}$$
   Show that this is the set-theoretic *relational composition* $R \circ S$ (as defined in Section 1 of this chapter) up to reordering of components. What does the order of composition correspond to in a chain of SQL joins?

3. Third Normal Form (3NF) is a constraint on database schemas that prevents redundancy. One way to state it: every non-key attribute is determined by a key, not by another non-key attribute. Formalize this constraint as a property of the functions defined by functional dependencies. Construct a small example of a relation that violates 3NF and show how to decompose it into a pair of relations in 3NF using the set-theoretic notion of projection.

*Abstract concept illustrated: Relations as subsets of Cartesian products; functions as special relations; projection and composition of relations (Section 1 of this chapter, naive set theory).*

---

## Exercise B.2: Type Systems as Set-Theoretic Models
*Domain: Programming Language Theory / Software Engineering*

**Setup:** In a typed programming language, each expression is assigned a *type*, and types can be thought of as sets of values. The type `Bool` is the set $\{\mathtt{true}, \mathtt{false}\}$; the type `Nat` is $\{0, 1, 2, \ldots\}$; the type `Int` is $\mathbb{Z}$. More complex types are built by type constructors that correspond to set-theoretic operations:
- Product type $A \times B$ is the Cartesian product.
- Sum type $A + B$ (disjoint union) is $(\{0\} \times A) \cup (\{1\} \times B)$ (the tags distinguish the two copies).
- Function type $A \to B$ is the set of all functions from $A$ to $B$, i.e., $B^A$.

**Questions:**
1. Compute (or carefully describe) the *cardinality* of each of the following types, assuming $|A| = m$ and $|B| = n$ (finite). Identify which set-theoretic operation each type constructor corresponds to, and use the cardinal arithmetic rules from Section 1:
   - $A \times B$
   - $A + B$
   - $A \to B$
   - $(A \to B) \times (A \to C)$, and compare to $A \to (B \times C)$ — are they equal?

2. A *subtype* relationship $A <: B$ in a type system means that any value of type $A$ can be used wherever a value of type $B$ is expected. In set-theoretic terms, this corresponds to $A \subseteq B$ (inclusion). The function type constructor is *contravariant* in the argument type: $B <: B'$ and $A' <: A$ implies $(A \to B) <: (A' \to B')$. Explain this contravariance using the definition of functions as sets: why does making the argument type *smaller* correspond to making the function type *larger*?

3. In Haskell or Lean, the `Option A` type (written `Maybe A` in Haskell) is a type with values $\{\mathtt{None}\} \cup (\{\mathtt{Some}\} \times A)$, i.e., it is $1 + A$ where $1 = \{\mathtt{None}\}$ is a one-element set. More precisely, `Option A` is (in set-theoretic terms) the disjoint union $\{*\} \sqcup A$. What is $|\mathtt{Option}\, A|$? An *injective function* $f : A \to \mathtt{Option}\, B$ is often called a "partial function" from $A$ to $B$. Using the set-theoretic definition of partial functions (as relations that are functional but not necessarily total), show that partial functions $A \rightharpoonup B$ are in bijection with injections $A \to (1 + B)$.

*Abstract concept illustrated: Cardinal arithmetic; Cantor's theorem applied to function types; the set-theoretic models of type constructors (Sections 1 and 3).*

---

## Exercise B.3: Cryptographic Key Spaces and Cardinality
*Domain: Cryptography / Information Security*

**Setup:** The security of a cryptographic system often depends on the size of its *key space* — the set of all possible keys. If a key space has $N$ elements, a brute-force attack requires on average $N/2$ trials. Modern symmetric encryption (e.g., AES-256) uses keys drawn from a set of size $2^{256}$; public-key systems work in group-theoretic structures whose size must be chosen carefully. Cardinality arguments from set theory underlie the security analysis.

**Questions:**
1. An AES-256 key is a sequence of 256 bits — a function $f : \{1, \ldots, 256\} \to \{0, 1\}$. Using the set-theoretic definition of a function as a set of ordered pairs, compute $|\{0,1\}^{256}|$ (the set of all such functions). Write the answer in the form $2^n$ and explain why this equals the number of 256-bit strings.

2. A *one-time pad* key for encrypting a message of $n$ bits is a uniformly random $n$-bit string. Two messages of lengths $n_1$ and $n_2$ are encrypted with independent keys. The total key space is the Cartesian product of the two individual key spaces. If $n_1 = 128$ and $n_2 = 64$, what is the total number of key pairs? Express this as a power of 2 and verify using cardinal arithmetic: $|A \times B| = |A| \cdot |B|$.

3. The *pigeonhole principle* — a consequence of the set-theoretic definition of injection — states: if $|A| > |B|$, there is no injection $A \hookrightarrow B$. Apply this to hash functions: a cryptographic hash function $h : \mathsf{Messages} \to \{0,1\}^{256}$ maps an effectively infinite domain (all possible messages) to a finite set of $2^{256}$ hash values. Prove, using the pigeonhole principle, that $h$ cannot be injective (i.e., collisions must exist). Does the existence of collisions necessarily mean $h$ is insecure? What additional properties (beyond the set-theoretic structure) does security require?

4. (Extension) Cantor's theorem says $|\mathcal{P}(A)| > |A|$ for any set $A$, including infinite sets. In the context of symmetric encryption, the set of all possible cryptographic algorithms over key space $K$ can be thought of as a subset of functions $\{0,1\}^* \to \{0,1\}^*$ — a much larger set than $K$ itself. Does Cantor's theorem have any practical implication for the impossibility of a single master key that "encodes" all possible encryption algorithms? Argue informally.

*Abstract concept illustrated: Functions as sets; cardinal arithmetic; the pigeonhole principle as a consequence of the definition of injection; Cantor's theorem (Sections 1 and 2).*

---

## Exercise B.4: Countability Arguments in Computational Complexity
*Domain: Theoretical Computer Science / Algorithm Analysis*

**Setup:** The distinction between countable and uncountable sets has concrete consequences in computer science. The set of all programs (in any fixed programming language) is countable — programs are finite strings over a finite alphabet. The set of all mathematical functions $f : \mathbb{N} \to \mathbb{N}$ is uncountable (by Cantor's diagonal argument). This gap between computable functions (countably many) and all functions (uncountably many) is the set-theoretic heart of the fact that most functions are not computable.

**Questions:**
1. Let $\Sigma = \{0, 1\}$ be a binary alphabet and $\Sigma^* = \bigcup_{n=0}^{\infty} \Sigma^n$ be the set of all finite binary strings. Prove that $\Sigma^*$ is countably infinite by constructing an explicit bijection $f : \mathbb{N} \to \Sigma^*$. (*Hint:* order strings by length, then lexicographically within each length. What is $f(0)$, $f(1)$, $f(2)$, $f(3)$, $f(4)$?)

2. The set of all Java programs is a subset of $\Sigma^*$ and hence countable. The set of all languages over $\{0,1\}$ is $\mathcal{P}(\{0,1\}^*)$. Use Cantor's theorem — $|\mathcal{P}(A)| > |A|$ for any set $A$ — to prove that $|\mathcal{P}(\{0,1\}^*)| > |\{0,1\}^*|$, and conclude that there are more languages than there are programs. What does this imply about the existence of languages that no program (Turing machine) can decide?

3. The *Halting Problem* can be phrased as a cardinality argument. Define $H \subseteq \mathbb{N} \times \mathbb{N}$ by $(e, n) \in H$ if the $e$-th Turing machine halts on input $n$. Suppose for contradiction that $H$ were decidable — there is a total computable function $\chi_H : \mathbb{N} \times \mathbb{N} \to \{0,1\}$ with $\chi_H(e,n) = 1$ iff $(e,n) \in H$. Define $D(n) = 1 - \chi_H(n,n)$. Identify the diagonalization step in this argument and relate it explicitly to Cantor's 1891 diagonal argument for $|\mathbb{R}| > |\mathbb{N}|$.

*Abstract concept illustrated: Countable and uncountable sets; Cantor's diagonal argument (Section 2 of this chapter); cardinality as a tool for proving existence and non-existence results.*

---

## Exercise B.5: Function Composition and Software Architecture
*Domain: Software Architecture / Functional Programming*

**Setup:** In functional programming and software architecture, *composition* is a fundamental operation: if $f : A \to B$ and $g : B \to C$, then $g \circ f : A \to C$ is defined by $(g \circ f)(x) = g(f(x))$. This is the set-theoretic composition of functions. Properties of composition — associativity, identity, injectivity, surjectivity — correspond directly to properties that software architects care about when chaining components.

**Questions:**
1. Using the set-theoretic definitions of injectivity ($f$ is injective iff $f(x) = f(y)$ implies $x = y$) and surjectivity ($f$ is surjective iff for every $b \in B$ there exists $a \in A$ with $f(a) = b$), prove the following composition laws:
   - If $f : A \to B$ and $g : B \to C$ are both injective, then $g \circ f : A \to C$ is injective.
   - If $f : A \to B$ and $g : B \to C$ are both surjective, then $g \circ f : A \to C$ is surjective.
   - If $g \circ f$ is injective, must $f$ be injective? Must $g$ be injective? Prove or give counterexamples.

2. A *middleware pipeline* in a web server is a sequence of functions $f_1, f_2, \ldots, f_n$ where each $f_i : \mathsf{Request} \to \mathsf{Request}$ (or possibly $\mathsf{Request} \to \mathsf{Response}$). The pipeline computes $f_n \circ \cdots \circ f_2 \circ f_1$. Associativity of function composition ($(h \circ g) \circ f = h \circ (g \circ f)$) means the order of evaluation does not matter when grouping the composition. Prove associativity directly from the set-theoretic definition: for all $x \in A$, $((h \circ g) \circ f)(x) = (h \circ (g \circ f))(x)$.

3. The *identity function* $\mathsf{id}_A : A \to A$ defined by $\mathsf{id}_A(x) = x$ is the identity element for composition: $f \circ \mathsf{id}_A = f$ and $\mathsf{id}_B \circ f = f$ for $f : A \to B$. The collection of all sets together with all functions between them, with composition as the operation, forms a *category* (specifically, the category **Set**). Identify the four axioms of a group (from Chapter 2) and check which of them are satisfied by the collection of functions $A \to A$ under composition. Is this a group? If not, what structure is it?

*Abstract concept illustrated: Functions, injectivity, surjectivity, and composition (Section 1); the structure of function sets as an algebraic object (connection to Chapter 2).*

---

## Exercise B.6: The Axiom of Choice in Algorithm Design
*Domain: Algorithm Design / Combinatorics*

**Setup:** The Axiom of Choice (AC) asserts that for any collection of non-empty sets $\{A_i \mid i \in I\}$, there exists a function $f : I \to \bigcup_{i \in I} A_i$ with $f(i) \in A_i$ for each $i$. Algorithmically, this sounds trivial: just pick an element from each set. The subtlety is that AC is needed only when there is no *definable rule* for making the choices — when we cannot write an explicit algorithm. This distinction between "exists a choice function" (AC) and "has a computable choice function" (constructive mathematics) has concrete implications for what can be implemented.

**Questions:**
1. The *Well-Ordering Theorem* (equivalent to AC) states that every set can be well-ordered. A *well-ordering* on a set $S$ is a total order on $S$ in which every non-empty subset of $S$ has a least element. Explain why having a well-ordering on an infinite set is the set-theoretic prerequisite for defining a greedy algorithm on that set. (A greedy algorithm at each step selects the "minimum" remaining element satisfying some criterion — this requires the existence of a minimum, which is the well-ordering property.) Identify a specific algorithm (e.g., Dijkstra's, Kruskal's, Prim's) and pinpoint where well-orderedness is implicitly used.

2. Zorn's Lemma (equivalent to AC) states: if every chain (totally ordered subset) in a partially ordered set $(P, \leq)$ has an upper bound in $P$, then $P$ has a maximal element. Use Zorn's Lemma to prove that every vector space has a basis. (*Hint:* consider the poset of all linearly independent subsets of the vector space, ordered by inclusion.) This result is not constructively provable — it does not give an algorithm for finding the basis. What does this mean for a programmer who needs a basis for a specific computation?

3. The following statement is equivalent to a weak form of AC (the Axiom of Dependent Choice): "every infinite tree in which every node has finitely many children has an infinite path." This is König's Lemma. Use König's Lemma to prove that if a context-free grammar generates infinitely many strings, then at least one derivation tree is infinite (a tree where each non-terminal expands to finitely many symbols). This has a direct application in parsing theory — what does it imply about the existence of infinite derivations in CFGs that generate only finite strings?

*Abstract concept illustrated: The Axiom of Choice and its equivalents — Well-Ordering Theorem, Zorn's Lemma, König's Lemma (Section 4); the distinction between existence proofs and constructive proofs (Section 5 and connection to type theory).*
