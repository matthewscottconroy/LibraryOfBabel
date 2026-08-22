# Applied Exercises

Constructive logic is not a philosophical curiosity but a practical discipline. Every time a programmer writes a function that computes a value from an input, they are — whether they know it or not — constructing a proof in the sense of the BHK interpretation: the function is the proof of $\forall x, \exists y, f(x) = y$. The distinction between classical and intuitionistic reasoning becomes concrete in programming when you ask: does this existence proof give me an algorithm? Does this software guarantee give me a verified behavior or only an asserted one? The exercises below trace these connections carefully, from the BHK interpretation through Kripke semantics to the practical logic of verified software.

---

## Exercise C.1: Extracting Programs from Constructive Proofs
*Domain: Functional Programming / Program Synthesis*

**Setup:** The Curry-Howard correspondence says that a constructive proof of a proposition $P$ is a program of type $P$ (under the types-as-propositions identification). The *program extraction* paradigm makes this precise: given a constructive proof in Coq or Agda, you can mechanically extract a functional program that computes the content of the proof. This is the direct application of the BHK interpretation: a proof of $\forall n : \mathbb{N}, \exists m : \mathbb{N}, m > n$ is a function $f : \mathbb{N} \to \mathbb{N}$ with $f(n) > n$ for all $n$.

**Questions:**

1. Consider the proposition $\forall a \, b : \mathbb{N}, \exists q \, r : \mathbb{N}, a = b \cdot q + r \wedge r < b$ (the division algorithm: every pair of naturals $a, b$ with $b > 0$ has a unique quotient and remainder). A constructive proof of this proposition, under the BHK interpretation, is a function that takes $a$ and $b$ and returns $q$, $r$, a proof that $a = b \cdot q + r$, and a proof that $r < b$. Write (in mathematical pseudocode or Haskell) a function `divmod :: Nat -> Nat -> (Nat, Nat)` that computes the quotient and remainder by iterated subtraction. Identify which part of your program corresponds to the *witness* of the existential quantifier and which part corresponds to the *proof* of the formula.

2. A classically valid statement that has no direct constructive computational content: "For every continuous function $f : [0,1] \to \mathbb{R}$ with $f(0) < 0 < f(1)$, there exists $c \in [0,1]$ with $f(c) = 0$" (the Intermediate Value Theorem). The classical proof is non-constructive (it uses LEM to decide, at each step of a bisection, which half contains a sign change). The constructive version gives a *rational approximation* to $c$ within any prescribed $\varepsilon$, given an explicit modulus of continuity for $f$. Describe the structure of this constructive approximation algorithm. What additional input (beyond $f$, $f(0)$, and $f(1)$) is required? What type does the constructive proof have, compared to the classical existential statement?

3. A proof in classical logic of "there exists a non-computable function $f : \mathbb{N} \to \{0, 1\}$" is given by a simple cardinality argument (the computable functions are countable, but $\{0,1\}^{\mathbb{N}}$ is uncountable). Does this proof extract a program? Why not? What is the BHK-interpretation reason that this classical existence proof carries no computational content? (Hint: the existence proof does not exhibit a specific $f$; it argues by contradiction from the assumption that all such $f$ are computable.) Connect this to Section 6 of this chapter: is the property "is a non-computable function" decidable?

*Abstract concept illustrated: The BHK clause for $\exists$: a proof of $\exists x, P(x)$ contains a witness $x$ and a proof of $P(x)$; classical existence proofs by contradiction do not contain witnesses; the Curry-Howard correspondence as a mechanism for program extraction.*

---

## Exercise C.2: Avoiding Classical Reasoning in Verified Software
*Domain: Software Verification / Formal Methods*

**Setup:** Modern proof assistants (Coq, Agda, Lean) are based on constructive type theories. By default, users cannot assume the law of excluded middle: a proof of $P \vee \neg P$ must be constructive. In practice, this means that some classical proof patterns are unavailable unless LEM is explicitly postulated as an axiom. A key engineering question: which proofs in a verification project require classical axioms, and which can be done constructively?

**Questions:**

1. In Coq, the following classical axioms are sometimes added: (a) `excluded_middle : forall P : Prop, P \/ ~P`, (b) `proof_irrelevance : forall P : Prop, forall p q : P, p = q`, (c) `functional_extensionality : forall A B (f g : A -> B), (forall x, f x = g x) -> f = g`. None of these is provable constructively. For each, give an example of a program verification task where it would be needed: (a) a property whose verification seems to require deciding at each step whether a condition holds, (b) a case where two proofs of the same property need to be identified as equal, (c) a case where two functions must be proved equal by checking pointwise. For each, explain why the constructive version of the proof is actually stronger: what *extra* information does the constructive proof provide that the classical proof does not?

2. A critical section of software: "If the mutex is available, acquire it and enter; otherwise, wait." This has the form $P \vee \neg P$ where $P$ is "mutex is currently available." A classical programmer simply writes `if (mutex.tryAcquire()) {...} else {...}`, implicitly invoking LEM. But in a verified concurrent system, the *mutex state* is determined by a computable predicate (the mutex has a concrete state register). Explain why LEM is constructively valid for *decidable* predicates: if $P$ is decidable (meaning there is an algorithm that returns either a proof of $P$ or a proof of $\neg P$), then $P \vee \neg P$ is constructively provable. State this as a formal theorem and give its constructive proof. How does this connect Section 6 of this chapter (decidability) to practical software verification?

3. The following is a classical theorem: "Every finite graph either has a perfect matching or has no perfect matching." This is an instance of LEM for a decidable property (graph matching is decidable by polynomial-time algorithms). Now consider: "Every graph has either a perfect matching or no perfect matching." For infinite graphs, this may not be decidable, and a constructive proof would require a decision algorithm or an explicit obstruction. Write the formal constructive statement that corresponds to "there is an algorithm deciding matching": $\forall G, (G \text{ has a perfect matching}) \vee (G \text{ has no perfect matching})$, where this disjunction carries computational content. Contrast this with the classical statement and explain what a constructive existence proof of "a graph with a perfect matching" must provide.

*Abstract concept illustrated: Decidable vs. semidecidable propositions; LEM for decidable predicates is constructively provable; the disjunction property (a constructive proof of $P \vee Q$ specifies which holds); classical proofs without witnesses vs. constructive proofs with witnesses.*

---

## Exercise C.3: Kripke Semantics and Concurrent Systems
*Domain: Distributed Computing / Concurrency Theory*

**Setup:** Kripke frames for intuitionistic logic consist of a partial order $(W, \leq)$ of "worlds" and a monotone forcing relation $w \Vdash P$ (read: "world $w$ forces proposition $P$"). The monotonicity condition says: if $w \Vdash P$ and $w \leq v$, then $v \Vdash P$ — once a fact is established, it cannot become un-established in more advanced worlds. This models *accumulation of knowledge*: as we move to more advanced worlds (more information is gathered), propositions can only become more established, not less.

This is a precise model of *knowledge in a distributed or concurrent system*: worlds are *states of knowledge* in a distributed computation, $\leq$ represents the information order ("$v$ knows at least as much as $w$"), and $w \Vdash P$ means "at the current state of knowledge $w$, proposition $P$ has been established."

**Questions:**

1. Consider a distributed key-value store with three replicas $R_1, R_2, R_3$. A write of value $v$ to key $k$ is acknowledged only after two replicas confirm. Model the knowledge states of the system as a Kripke frame: each world $w$ is a tuple $(S_1, S_2, S_3)$ where $S_i$ is the set of key-value pairs known to replica $i$; $w \leq v$ means each $S_i(w) \subseteq S_i(v)$. Define $w \Vdash (k = v)$ as "a majority of replicas in $w$ agree that $k$ maps to $v$." Verify the monotonicity condition: if the system knows $k = v$ at state $w$ and learns more information (moves to state $v \geq w$), does it still know $k = v$? Why or why not?

2. In Kripke semantics, the intuitionistic implication $P \to Q$ at world $w$ means: "for every world $v \geq w$, if $v \Vdash P$ then $v \Vdash Q$." In the distributed system, this says: "for every future state of knowledge $v$ where $P$ holds, $Q$ also holds." This is a *stability guarantee*: once $P$ is established, $Q$ will be as well in any consistent future. Reformulate the eventual consistency condition for the key-value store in terms of Kripke forcing: "the store is eventually consistent" means $w \Vdash \neg\neg(k = v)$ for some appropriate world — "it is not the case that $k = v$ will never be established." Explain why $\neg\neg(k = v)$ is strictly weaker than $k = v$ in intuitionistic logic but the same in classical logic. What is the operational difference in the distributed system?

3. Kripke completeness says: $\vdash_\mathsf{IPC} \varphi$ if and only if $w \Vdash \varphi$ for all Kripke frames and all worlds $w$ in those frames. Use this to prove that $\neg\neg P \to P$ (double negation elimination, DNE) is *not* intuitionistically valid, by constructing a Kripke frame and a world $w$ where $w \Vdash \neg\neg P$ but $w \not\Vdash P$. (Hint: take $W = \{w_0, w_1\}$ with $w_0 < w_1$, and let $P$ hold at $w_1$ but not $w_0$. Verify $w_0 \Vdash \neg\neg P$ by tracing through the definition.) What does this mean for the distributed system: what is the *operational* content of a world that knows $\neg\neg P$ but not $P$?

*Abstract concept illustrated: Kripke semantics for intuitionistic logic; monotonicity of forcing as accumulation of knowledge; the failure of DNE in Kripke models; the relationship between Kripke semantics and distributed systems' knowledge states.*

---

## Exercise C.4: Constructive Real Arithmetic and Exact Computation
*Domain: Computer Algebra / Numerical Computation*

**Setup:** Standard floating-point arithmetic (IEEE 754) is not exact: rounding errors accumulate, and the result of a long computation may have no guaranteed relationship to the true mathematical answer. *Exact real computation* is a research area that implements computable real numbers as infinite streams of approximations with guaranteed precision: a real number $x$ is represented as a function $f_x : \mathbb{N} \to \mathbb{Q}$ such that $|f_x(n) - x| < 2^{-n}$ for all $n$. All arithmetic operations are defined on these streams, preserving the precision guarantee.

This is precisely Bishop's constructive real number: a Cauchy sequence of rationals with an explicit modulus of convergence. The constructive requirement — that every real number comes with a modulus of convergence, not just a Cauchy sequence — is not optional; it is what makes exact computation tractable.

**Questions:**

1. Define addition of exact reals: given $f_x : \mathbb{N} \to \mathbb{Q}$ with $|f_x(n) - x| < 2^{-n}$ and $f_y : \mathbb{N} \to \mathbb{Q}$ with $|f_y(n) - y| < 2^{-n}$, define $f_{x+y}(n) = f_x(n+1) + f_y(n+1)$. Verify that $|f_{x+y}(n) - (x+y)| < 2^{-n}$. Now implement multiplication: what precision of $f_x$ and $f_y$ is needed to compute $f_{x \cdot y}(n)$ to precision $2^{-n}$? (Your answer should involve bounding $|x|$ and $|y|$, which requires knowing that exact reals come with bounds.)

2. The classical Intermediate Value Theorem says: if $f : [a, b] \to \mathbb{R}$ is continuous and $f(a) < 0 < f(b)$, then there exists $c \in [a, b]$ with $f(c) = 0$. The constructive version requires: $f$ comes with a *modulus of uniform continuity* $\omega : \mathbb{Q}_{>0} \to \mathbb{Q}_{>0}$ such that $|x - y| < \omega(\varepsilon)$ implies $|f(x) - f(y)| < \varepsilon$. Given this, the bisection algorithm computes exact-real approximations to $c$. Implement the bisection algorithm for exact reals: at each step, compute $f$ at the midpoint to within $\varepsilon/2$ precision, determine the sign, and recurse. What is the type signature of this constructive IVT? Compare it to the classical statement: what extra data does the constructive version require as input?

3. The classical statement "every real number is either rational or irrational" ($\forall x : \mathbb{R}, x \in \mathbb{Q} \vee x \notin \mathbb{Q}$) is an instance of LEM and not constructively valid. Give an explicit reason: what would a constructive proof of this disjunction look like, and why can't we have one? (Hint: consider a real number $x$ defined by a series converging very slowly, where we don't know whether $x$ is rational or not.) Contrast with: "every *computable* real number is either computable-rational or computable-irrational" — is this decidable? (This connects to the remark in Section 6 about decidable vs. semidecidable properties.)

*Abstract concept illustrated: Bishop-style constructive real numbers as Cauchy sequences with explicit moduli; the computational content of constructive existence proofs; LEM for decidable properties vs. its failure for undecidable ones; the disjunction property in the context of exact real computation.*

---

## Exercise C.5: Decidable vs. Semidecidable Properties in Algorithms
*Domain: Algorithms / Programming Language Theory*

**Setup:** A property $P$ of natural numbers is *decidable* if there is a total algorithm that, for every input $n$, halts and outputs either "yes, $P(n)$" or "no, $\neg P(n)$." It is *semidecidable* (or recursively enumerable) if there is an algorithm that halts with "yes" when $P(n)$ holds, but may run forever when $\neg P(n)$. The constructive significance is this: a decidable property corresponds to a proof of $P(n) \vee \neg P(n)$ for each $n$ — an instance of LEM for that specific $n$; a semidecidable property corresponds to a proof of $\neg\neg P(n) \to P(n)$ is not available in general, but $P(n)$ may be provable by running the algorithm.

**Questions:**

1. Classify the following properties as decidable, semidecidable but not decidable, or neither, and justify your classification:
   - (a) "The Turing machine $M$ halts on empty input within $k$ steps" (for given $M$ and $k$).
   - (b) "The Turing machine $M$ halts on empty input" (for given $M$).
   - (c) "The Turing machine $M$ runs forever on empty input" (for given $M$).
   - (d) "The polynomial $p(x_1, \ldots, x_n)$ with integer coefficients has an integer root" (for given $p$).
   
   For each, identify the corresponding statement in intuitionistic logic: is it $P \vee \neg P$ (decided constructively), $P$ (semidecidable), $\neg P$ (co-semidecidable), or none?

2. The *type-checking* problem for a given typed programming language is typically decidable: given a program $e$ and a type $\tau$, determine whether $e$ has type $\tau$. Explain why type checking being decidable is a constructive virtue: it means the type system provides a *proof* of the typing judgment $\vdash e : \tau$, and this proof can be *mechanically verified*. By contrast, *type inference* (finding a $\tau$ such that $\vdash e : \tau$, or reporting that none exists) is semidecidable for dependent type theories (it may run forever searching for a type). What constructive property does type inference fail to have that type checking enjoys?

3. A program property $P$ is *h-decidable* at level $n$ (in the terminology of HoTT) if the proposition $P$ is an $n$-type: for $n = -1$ it is a proposition (proof-irrelevant), for $n = 0$ it is a set (with decidable equality), etc. The *decidability* property from Section 6 of this chapter says $P$ is decidable if $P + \neg P$ is inhabited (note: a coproduct, not a mere disjunction). Explain how the *sets* in HoTT (h-sets, or 0-types) correspond to the constructive notion of "types with decidable equality": $A$ is a set if $\forall x \, y : A, (x =_A y) + \neg(x =_A y)$. Give two examples of types in HoTT that are sets with decidable equality and two that are not. (Examples might include $\mathbb{N}$, $\mathbb{B}$, arbitrary function types, and the circle $S^1$.)

*Abstract concept illustrated: The constructive/intuitionistic distinction between decidable ($P + \neg P$) and semidecidable ($P$ without $\neg P$) properties; LEM for decidable propositions; h-levels and decidability in HoTT; the disjunction property of intuitionistic logic as a computational constraint.*

---

## Exercise C.6: The BHK Interpretation in a Typed Functional Language
*Domain: Programming Languages / Type Theory*

**Setup:** The Curry-Howard correspondence says that the introduction and elimination rules of intuitionistic natural deduction correspond exactly to the type formation and computation rules of the simply-typed $\lambda$-calculus (and, for dependent types, of MLTT). The following table summarizes the correspondence for propositional logic:

| Logic | Type Theory |
|-------|------------|
| $P \wedge Q$ | Product type $P \times Q$ |
| $P \vee Q$ | Sum type $P + Q$ |
| $P \to Q$ | Function type $P \to Q$ |
| $\top$ | Unit type $\mathbf{1}$ |
| $\bot$ | Empty type $\mathbf{0}$ |
| $\neg P$ | Function type $P \to \mathbf{0}$ |

Under this correspondence, the BHK clauses are exactly the typing rules of the type theory.

**Questions:**

1. Translate the following constructive proofs into Haskell (or typed pseudocode), using the above correspondence. For each, identify the BHK clause being instantiated:
   - (a) A proof of $P \wedge Q \to P$: the first projection.
   - (b) A proof of $P \to P \vee Q$: the left injection.
   - (c) A proof of $(P \to Q) \to (Q \to R) \to (P \to R)$: composition.
   - (d) A proof of $\neg\neg P \to P$ is *not* available; what goes wrong when you try to write the corresponding Haskell function? (The Haskell type is `((P -> Void) -> Void) -> P` for the empty type `Void`. Why can't this be implemented?)

2. Peirce's law, $(( P \to Q) \to P) \to P$, is a classical tautology not provable in intuitionistic logic. The corresponding Haskell type is `((P -> Q) -> P) -> P`. Show that this type is uninhabited for all `P` and `Q` in a purely functional language with no recursion and no bottom (`undefined`). (Hint: no pure function of this type can be written; any attempt requires knowing whether `P` is inhabited, which is not available as a function argument.) Under what extension of the language (adding control operators like `call/cc`, which corresponds to adding classical logic to the type theory) does this become implementable?

3. The *disjunction property* of intuitionistic propositional logic says: if $\vdash P \vee Q$, then $\vdash P$ or $\vdash Q$. Translate this into a statement about typeable closed programs: if there is a closed program of type $P + Q$ (a sum type), then there is a closed program of type $P$ or there is a closed program of type $Q$. Verify this for the sum type `Bool = True | False`: programs of type `Bool` are either `True` or `False` (or $\bot$, which we exclude). Now consider the classical extension: in a language with `callcc`, there may be programs of type `P + Q` that don't obviously reduce to either `inl p` or `inr q` — they may produce a value via a control effect. This is the type-theoretic expression of why LEM violates the disjunction property.

*Abstract concept illustrated: The Curry-Howard correspondence as a formal statement of the BHK interpretation; the disjunction property and existence property as consequences of constructivity; Peirce's law and LEM as classical principles without constructive content; the correspondence between adding classical axioms and adding control operators (call/cc).*
