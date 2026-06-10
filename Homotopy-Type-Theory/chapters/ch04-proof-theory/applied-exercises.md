# Applied Exercises

The formal machinery of proof theory — sequent calculus, cut elimination, natural deduction, normalization — is not confined to the philosophy of mathematics. It is the foundation of automated reasoning, the theory behind SAT and SMT solvers, the explanation for why some theorems have only very long proofs, and the guarantee of correctness in proof-carrying code. The exercises below situate the abstract theory of Section 4 in these concrete engineering and theoretical contexts, asking you to trace the connection from formal proof rules to working systems and real complexity phenomena.

---

## Exercise B.1: Resolution and Cut Elimination in Automated Theorem Proving
*Domain: Formal Verification / Automated Theorem Proving*

**Setup:** Most modern automated theorem provers for first-order logic implement *resolution*, a refutation procedure: to prove a goal $G$ from axioms $\Gamma$, negate $G$ to get $\neg G$, add it to $\Gamma$, convert everything to conjunctive normal form (a conjunction of clauses, each a disjunction of literals), and repeatedly apply the *resolution rule*:

$$\frac{(A \vee C_1) \quad (\neg A \vee C_2)}{C_1 \vee C_2}$$

Resolution derives the empty clause $\square$ (representing $\bot$) if and only if $\Gamma \cup \{\neg G\}$ is unsatisfiable, i.e., if and only if $\Gamma \vdash G$. The connection to proof theory is direct: the resolution rule is precisely the *cut rule* of sequent calculus applied to the literal $A$, followed by simplification.

**Questions:**

1. Translate the following sequent calculus proof into a resolution derivation. Consider the sequent $P \to Q, P \Rightarrow Q$ (from $P \to Q$ and $P$, derive $Q$). The sequent calculus proof uses the $\to L$ rule. In clause form: the axioms are $\neg P \vee Q$ (for $P \to Q$) and $P$; the negated goal is $\neg Q$. Show how a single resolution step derives the empty clause $\square$.

2. Cut elimination says that every proof with cuts can be transformed into a cut-free proof. In the resolution setting, this means every resolution proof can be reorganized so that no intermediate lemma (cut formula) is introduced. The resulting proof is an *input resolution* or *unit resolution* derivation. Show by example that this reorganization can cause an exponential blowup: find a family of formulas $F_n$ (indexed by $n$) such that $F_n$ has a resolution proof with $O(n)$ resolution steps using intermediate lemmas, but any proof without intermediate lemmas requires at least $2^{O(n)}$ steps. (Hint: consider formulas that encode the pigeon-hole principle: $n+1$ pigeons cannot be placed in $n$ holes without a collision.)

3. Modern SAT solvers use *conflict-driven clause learning* (CDCL), which is closely related to proof search with cut: when the solver reaches a contradiction, it "learns" a new clause (the negation of the conflict) and adds it to the database. Interpret this learning step as a cut introduction: the learned clause is a lemma proved by the conflict, and it will be used (as a cut formula) in the rest of the search. Why does learning make search dramatically faster in practice, even though cut-elimination guarantees that learned clauses are in principle unnecessary? Relate your answer to the exponential blowup from part (2).

*Abstract concept illustrated: The cut rule as lemma use; cut elimination as the proof that lemmas are eliminable; the unavoidable cost of cut-free proofs (proof complexity); the Hauptsatz and its limits.*

---

## Exercise B.2: Proof Length and the Complexity of Cut-Free Proofs
*Domain: Computational Complexity / Proof Complexity*

**Setup:** Proof complexity studies the length of formal proofs in various proof systems, seeking to understand which theorems have short proofs and which do not. The central conjecture of the field (whose resolution would likely resolve $\mathsf{P}$ vs. $\mathsf{NP}$) is that there is no proof system in which all tautologies have polynomial-length proofs.

The *pigeon-hole principle* $\mathrm{PHP}_n$ asserts: "If $n+1$ pigeons are placed in $n$ holes, two pigeons share a hole." This is obviously true, but proving it in certain formal systems requires exponentially long proofs. Formally, $\mathrm{PHP}_n$ can be expressed as a propositional tautology with $O(n^2)$ variables ($x_{ij}$ meaning "pigeon $i$ is in hole $j$").

**Questions:**

1. Write out $\mathrm{PHP}_2$ explicitly as a propositional formula (3 pigeons, 2 holes): for each pigeon $i \in \{1, 2, 3\}$, at least one hole is occupied; for each hole $j \in \{1, 2\}$, at most one pigeon uses it. The formula says: (each pigeon is in some hole) $\wedge$ (no two pigeons share a hole). Verify that $\mathrm{PHP}_2$ is a tautology by truth-table.

2. Haken (1985) proved that $\mathrm{PHP}_n$ has no polynomial-size proofs in the *resolution* proof system. Intuitively, why does this make sense? A resolution refutation of $\neg\mathrm{PHP}_n$ must "discover" that every assignment puts two pigeons in the same hole. Describe informally why any resolution proof must "search" through an exponential number of cases. (You do not need to reproduce Haken's full proof; a convincing informal argument suffices.)

3. The *Frege proof system* (essentially, Hilbert-style propositional logic with modus ponens) can prove $\mathrm{PHP}_n$ in polynomial size using the counting principle: there are $n+1$ pigeons but only $n$ holes, so by counting, two must share. Write out the logical structure of this short proof (it uses the principle "if $|A| > |B|$, the map $A \to B$ is not injective"). Why does this short proof rely on a high-level mathematical fact (counting) rather than a case analysis? In the language of cut elimination: what is the "cut formula" (the lemma used) in this short proof, and why does it have a high complexity?

*Abstract concept illustrated: Cut elimination can cause exponential blowup in proof length; the subformula property is a restriction that makes proofs longer; the relationship between proof complexity and computational complexity ($\mathsf{P}$ vs. $\mathsf{NP}$).*

---

## Exercise B.3: Constructing and Analyzing the Gödel Sentence
*Domain: Mathematical Logic / Foundations of Mathematics*

**Setup:** Gödel's incompleteness proof constructs, for any sufficiently strong consistent theory $T$, a sentence $G_T$ that asserts its own unprovability: $G_T \equiv \neg\mathrm{Prov}_T(\ulcorner G_T \urcorner)$, where $\mathrm{Prov}_T(n)$ is a formula in the language of arithmetic expressing "the natural number $n$ codes a theorem of $T$." The construction requires two ingredients: (1) Gödel numbering — an encoding of formulas as natural numbers — and (2) the *diagonal lemma* (self-reference lemma): for any formula $\varphi(x)$, there is a sentence $\psi$ such that $T \vdash \psi \leftrightarrow \varphi(\ulcorner \psi \urcorner)$.

**Questions:**

1. The diagonal lemma has a direct analogue in type theory and in the $\lambda$-calculus: the *fixed-point combinator* $Y$ satisfies $Y f \to_\beta f(Y f)$, i.e., $Y f$ is a fixed point of $f$. Explain the analogy: how does the diagonal lemma in logic correspond to fixed-point combinators in computation? (In both cases, a "function on codes/terms" is applied to its own code/term.) What is the analogue of the Gödel sentence in the $\lambda$-calculus? (Hint: consider non-terminating $\lambda$-terms like $\Omega = (\lambda x. x x)(\lambda x. x x)$.)

2. The *Löbian sentence* $L_T$ is: "$L_T$ is provable in $T$." Formally: $L_T \equiv \mathrm{Prov}_T(\ulcorner L_T \urcorner)$. By Löb's theorem (a strengthening of the second incompleteness theorem), $T \vdash L_T$. Trace through the Löb's theorem argument: (a) assume $T \vdash \mathrm{Prov}_T(\ulcorner \varphi \urcorner) \to \varphi$ for any $\varphi$; (b) apply the diagonal lemma to get $L_T \equiv (\mathrm{Prov}_T(\ulcorner L_T \urcorner) \to L_T)$; (c) use the provability axioms (the Hilbert-Bernays-Löb derivability conditions) to derive $T \vdash L_T$. Compare this argument to the proof that the $Y$ combinator produces a fixed point.

3. The second incompleteness theorem says $T \not\vdash \mathrm{Con}(T)$ (assuming $T$ is consistent and contains arithmetic). But Gentzen *did* prove the consistency of PA — using transfinite induction up to $\varepsilon_0$. The apparent contradiction is resolved by observing that Gentzen's proof takes place in a *stronger* system that accepts $\varepsilon_0$-induction. Explain what $\varepsilon_0$ is (the least ordinal greater than $\omega$, $\omega^\omega$, $\omega^{\omega^\omega}$, etc.) and why accepting transfinite induction up to $\varepsilon_0$ is stronger than what PA proves. What is the proof-theoretic ordinal of PA, and what does it mean for a system's "strength"?

*Abstract concept illustrated: The diagonal lemma and self-reference; Gödel numbering as the encoding of syntax in arithmetic; the relationship between the provability predicate and the halting problem; ordinal analysis of proof-theoretic strength.*

---

## Exercise B.4: Program Termination and the Halting Problem
*Domain: Software Engineering / Program Verification*

**Setup:** A core task in software verification is proving that a program terminates — that it doesn't run forever. Turing's 1936 proof that the halting problem is undecidable says there is no single algorithm that decides termination for all programs. But this does not mean that termination is unprovable in specific cases: many practical programs can be proved to terminate, using well-founded orderings and ranking functions.

The connection to proof theory is the Curry-Howard correspondence: a *termination proof* for a program corresponds to a *normalization proof* for a term in a typed system. Section 3 of this chapter proves normalization for natural deduction; this corresponds exactly to the fact that well-typed $\lambda$-terms in the simply typed $\lambda$-calculus always terminate.

**Questions:**

1. Consider the following Python-style pseudocode:
   ```
   def collatz(n):
       while n != 1:
           if n % 2 == 0:
               n = n // 2
           else:
               n = 3 * n + 1
   ```
   The Collatz conjecture says this terminates for all positive integers $n$. Explain why you cannot simply run the program to verify termination (even for a specific $n$, a computation that runs for $10^{100}$ steps doesn't prove termination). What is a *ranking function* for termination proofs, and why does one exist for simple loops (e.g., a loop with `n = n - 1`) but not obviously for `collatz`? Connect the Collatz conjecture to Gödel's incompleteness: it is consistent with Peano Arithmetic that Collatz terminates, but also consistent that there is a counterexample — our inability to prove termination may be a logical obstacle, not merely a mathematical one.

2. The *simply typed $\lambda$-calculus* has the property that every typeable term terminates (the normalization theorem). Formally: if $\Gamma \vdash t : A$ is a valid typing judgment, then the sequence of $\beta$-reductions starting from $t$ terminates. The proof uses *reducibility candidates* (Tait's method): for each type $A$, define a set $\mathrm{Red}(A)$ of "strongly normalizing terms of type $A$," and prove by induction on types that every well-typed term belongs to $\mathrm{Red}(A)$. Trace through the key case: show that if $t \in \mathrm{Red}(A \to B)$ (a function) and $s \in \mathrm{Red}(A)$ (an argument), then $t s \in \mathrm{Red}(B)$ (the application). How is this structurally similar to the proof that a well-founded order on inputs implies termination?

3. The Curry-Howard correspondence says: proofs in natural deduction correspond to programs in the $\lambda$-calculus; normalization of proofs corresponds to execution of programs; and a "non-terminating proof" (a proof with an infinite reduction sequence) would be a proof of $\bot$ — which cannot exist in a consistent system. Explain why *inconsistency* in a type theory corresponds to *non-termination* in the corresponding programming language. (Hint: if $\bot$ is inhabited, then by the elimination rule for $\bot$, any type is inhabited, including the type of a non-terminating term.) Why do programming languages with unrestricted recursion (like Haskell or OCaml without restrictions) correspond logically to *inconsistent* type theories?

*Abstract concept illustrated: Normalization theorem for natural deduction; Tait's reducibility method; the Curry-Howard correspondence between termination and consistency; undecidability of the halting problem versus decidability for restricted classes of programs.*

---

## Exercise B.5: Proof-Carrying Code and Certified Software
*Domain: Operating Systems / Security / Formal Verification*

**Setup:** *Proof-carrying code* (PCC), introduced by Necula and Lee (1996), is a framework for safe code distribution: when untrusted code is delivered to a host system, it is accompanied by a formal proof that the code satisfies a safety policy (e.g., memory safety, no buffer overflows, type safety). The host verifier checks the proof — a mechanical, efficient process — rather than trusting the code or re-executing it in a sandbox.

The connection to proof theory is direct: the "proof" is a formal derivation in a sequent calculus or natural deduction system, and proof checking is the mechanical verification that each inference rule was applied correctly. Proof normalization guarantees that proofs can be put in a canonical form that is easy to verify.

**Questions:**

1. Suppose the safety policy is "no out-of-bounds array access." A simple formalization: if $a$ is an array of length $n$ and $i$ is an index, then $a[i]$ is safe if and only if $0 \leq i < n$. A PCC proof of safety would include, at each array access $a[i]$, a derivation of the sequent $\Gamma \Rightarrow 0 \leq i < n$, where $\Gamma$ contains the known facts at that program point. Model a simple loop:
   ```
   for i in range(n):
       a[i] = 0
   ```
   What would the sequent calculus derivation of "array access is safe" look like at each iteration? What is the "cut formula" in this proof (the loop invariant)?

2. Proof checking (verifying that a given proof is valid) is efficient: it takes time polynomial in the size of the proof, because each rule application can be verified in constant time. Proof *search* (finding a proof from scratch) can be much harder — even undecidable in general. Explain why this asymmetry is important for PCC: the code producer does the hard work of finding the proof; the host only needs to check it. In the language of proof theory: what property of sequent calculus (related to the subformula property and the structure of cut-free proofs) makes proof checking tractable?

3. The *BellCore* and later *seL4* operating system verification projects proved, using Isabelle/HOL and Coq, that the kernel's implementation correctly implements its specification with a proof containing tens of thousands of lemmas. These proofs are machine-checked using the same principles as PCC. A key technique is *certified compilation*: the CompCert project (Leroy, INRIA) produces a formally verified C compiler with a machine-checked proof that the compiled binary always has the same behavior as the source. Identify the proof-theoretic concept at the heart of this: why is the *simulation theorem* (if source computes result $r$, then compiled code also computes $r$) a statement in natural deduction, and what does its proof correspond to under Curry-Howard?

*Abstract concept illustrated: Sequent calculus as a framework for formal proof certificates; the subformula property and proof checking; proof normalization as a tool for canonical proof representation; the Curry-Howard view of programs and proofs as the same object.*
