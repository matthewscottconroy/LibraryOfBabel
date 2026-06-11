# Chapter 10 Overview: Computability and Incompleteness

---

## Central Question

Is there a limit to what formal systems can prove? Is there a limit to what algorithms can compute? These two questions, raised in the 1930s, turned out to have identical answers — and the technique for answering both revealed a profound connection between proof and computation.

---

## Why This Chapter Matters

Gödel's incompleteness theorems (1931) shattered Hilbert's program: no consistent formal system rich enough to express arithmetic can prove all arithmetical truths. Church and Turing (1936) independently proved the undecidability of the halting problem, establishing the limits of computation. These results define the boundary of the formally and computationally achievable, and are among the most important intellectual achievements of the twentieth century.

---

## Key Definitions

**Turing machine.** A Turing machine (TM) consists of: a finite alphabet $\Sigma$ (including a blank symbol $\sqcup$), a finite set $Q$ of states (including a start state $q_0$ and halt states), and a transition function $\delta: Q \times \Sigma \to Q \times \Sigma \times \{L, R\}$. The machine reads and writes symbols on an infinite tape, moving left or right, until it halts.

**Computable function.** A partial function $f: \mathbb{N}^k \to \mathbb{N}$ is computable (Turing-computable) if there is a TM that, on input $(n_1, \ldots, n_k)$, halts with $f(n_1, \ldots, n_k)$ on the tape iff $f$ is defined on that input, and runs forever otherwise.

**Decidable set.** A set $S \subseteq \mathbb{N}$ is decidable (recursive) if its characteristic function $\chi_S$ is computable: there is a TM that always halts, outputting 1 if $n \in S$ and 0 if $n \notin S$.

**Computably enumerable (c.e. / r.e.).** A set $S$ is c.e. (recursively enumerable) if there is a TM that halts on input $n$ iff $n \in S$. Equivalently: $S$ is the domain of a partial computable function, or $S$ is the range of a total computable function.

**Halting problem.** $K = \{e : \phi_e(e) \downarrow\}$ — the set of Turing machine indices $e$ such that machine $e$ halts on input $e$. This is the canonical undecidable set.

**Gödel numbering.** An effective encoding of syntactic objects (formulas, proofs) as natural numbers. Under such a coding, "this formula is provable" becomes an arithmetic statement.

---

## Main Theorems

### Theorem: The Halting Problem is Undecidable

**Theorem (Turing 1936).** The set $H = \{(e, n) : \phi_e(n) \downarrow\}$ (machine $e$ halts on input $n$) is not decidable.

**Proof.** By diagonalisation. Suppose $H$ were decidable by TM $D$. Define:

$$g(n) = \begin{cases} 1 & \text{if } D(n, n) = 0 \ (\text{i.e., machine } n \text{ does not halt on } n) \\ \text{undefined} & \text{if } D(n, n) = 1 \end{cases}$$

$g$ is computable; let $e$ be its index. Then $g(e) \downarrow \iff D(e, e) = 0 \iff g(e) \uparrow$. Contradiction. $\square$

### Rice's Theorem

**Theorem (Rice 1953).** For any non-trivial property $P$ of partial computable functions (non-trivial meaning neither all functions have $P$ nor no function has $P$), the set $\{e : \phi_e \text{ has property } P\}$ is undecidable.

**Proof.** By reduction from the halting problem. Fix $f_0$ not having $P$ and $f_1$ having $P$. For any input $e$ to the halting problem, construct a machine $e'$ that: simulates $\phi_e(e)$; if that halts, behaves like $f_1$; otherwise behaves like $f_0$. Then $\phi_{e'}$ has property $P$ iff $\phi_e(e) \downarrow$. So deciding $P$ would decide halting — contradiction. $\square$

**Significance:** Rice's theorem shows that essentially *any* interesting question about the behaviour of programs is undecidable: "does this program terminate?", "does this program produce output 0 on all inputs?", "is this program equivalent to that one?" — all undecidable.

### The Recursion Theorem (Kleene's Fixed-Point Theorem)

**Theorem (Kleene 1938).** For any total computable function $h: \mathbb{N} \to \mathbb{N}$, there exists an index $e$ such that $\phi_e = \phi_{h(e)}$ (the program $e$ computes the same function as the program that $h$ maps $e$ to).

**Proof.** Define $g(n, x) = \phi_{h(\phi_n(n))}(x)$ (compute the index $\phi_n(n)$, apply $h$, run the result on $x$). Let $e_0$ be an index for $g$. Then $\phi_{e_0}(n) = g(n, n)$ ... wait, let's be precise. Let $d$ be an index for the function $n \mapsto h(\phi_n(n))$. Then $\phi_d$ is total; $\phi_{\phi_d(d)}$ is the desired fixed point. $\square$

**Applications:** Programs that print themselves (quines) are guaranteed to exist by the recursion theorem. Many undecidability proofs use the recursion theorem to construct self-referential programs.

### Gödel's First Incompleteness Theorem

**Theorem (Gödel 1931).** Let $T$ be any consistent, effectively axiomatisable theory extending Peano Arithmetic (or any theory strong enough to represent computable functions). Then $T$ is incomplete: there exists a sentence $G_T$ (the "Gödel sentence") such that:
1. $T \not\vdash G_T$
2. $T \not\vdash \neg G_T$

Moreover, $G_T$ is true (in the standard model $\mathbb{N}$) — it is a true sentence that $T$ cannot prove.

**Proof sketch.** 

*Step 1: Gödel numbering.* Assign a natural number $\ulcorner\phi\urcorner$ to each formula $\phi$. The property "the formula with Gödel number $n$ is provable in $T$" is expressible in arithmetic (since $T$ is effectively axiomatised, provability is a c.e. property, hence arithmetic).

*Step 2: Diagonal lemma.* For any formula $\phi(x)$ (with one free variable), there exists a sentence $\psi$ such that $T \vdash \psi \leftrightarrow \phi(\ulcorner\psi\urcorner)$. (Proof: by the diagonal construction — define $\psi$ as $\phi$ applied to its own Gödel number.)

*Step 3: Construct $G_T$.* Let $\phi(x)$ express "the formula with Gödel number $x$ is not provable in $T$." By the diagonal lemma, there is a sentence $G_T$ such that $T \vdash G_T \leftrightarrow \neg Pr_T(\ulcorner G_T\urcorner)$. In words: "$G_T$ says: 'I am not provable in $T$.'"

*Step 4: Show $T \not\vdash G_T$.* If $T \vdash G_T$, then $Pr_T(\ulcorner G_T\urcorner)$ is true, so $G_T$ is false (since $G_T$ says it's not provable), so $T \vdash \neg G_T$ — contradicting consistency.

*Step 5: Show $T \not\vdash \neg G_T$ (for $\omega$-consistent $T$).* If $T \vdash \neg G_T$, then $G_T$ is false, so $T \vdash G_T$ — contradiction. (For this direction, Gödel required $\omega$-consistency; Rosser (1936) strengthened it to ordinary consistency.) $\square$

### Gödel's Second Incompleteness Theorem

**Theorem.** Under the same hypotheses, $T \not\vdash Con(T)$, where $Con(T)$ is the sentence expressing "T is consistent."

**Proof sketch.** Working inside $T$, formalise the proof of the first incompleteness theorem: "$G_T$ is unprovable in $T$ if $T$ is consistent" becomes a provable implication in $T$. So $Con(T) \to \neg Pr_T(\ulcorner G_T\urcorner) \to G_T$. If $T \vdash Con(T)$, then $T \vdash G_T$ — contradicting the first theorem. $\square$

**Significance:** No consistent formal system can prove its own consistency. Hilbert's program — prove mathematics consistent by finitary means, expressible within mathematics — is impossible for sufficiently strong systems.

---

## Historical Context

**David Hilbert (1928)** posed the Entscheidungsproblem and the formal consistency programme.

**Kurt Gödel (1906–1978)** proved the completeness theorem (1930, his dissertation) and the incompleteness theorems (1931, at age 25). The incompleteness results appeared in a journal paper that is one of the most consequential mathematical papers ever written.

**Alonzo Church (1936)** defined the lambda calculus and proved the undecidability of the Entscheidungsproblem using it.

**Alan Turing (1936)** independently defined Turing machines and proved halting undecidability. His paper "On Computable Numbers, with an Application to the Entscheidungsproblem" also defined the concept of a universal computer.

**The Church-Turing thesis** (not a theorem, but a thesis): every intuitively computable function is Turing-computable. This is supported by the equivalence of all known models of computation (lambda calculus, Turing machines, recursive functions, random access machines).

---

## Connections to Other Chapters

- **Chapter 4** proves the completeness of FOL, which is presupposed in Gödel's incompleteness argument.
- **Chapter 7** provides the recursion theorem (the computability version of structural recursion) used in these proofs.
- **Chapter 13** is directly motivated by incompleteness: proof assistants exist precisely because humans need machine-verified proof in domains where intuition fails.
