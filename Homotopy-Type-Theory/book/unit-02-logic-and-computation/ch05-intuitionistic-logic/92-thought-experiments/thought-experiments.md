# Thought Experiments: Intuitionistic Logic

## 1. The Twin Prime Oracle

Suppose you had access to an oracle that could answer the Twin Prime Conjecture: either "there are infinitely many twin primes" or "there are only finitely many." You learn the answer: say, "there are infinitely many."

Now ask: is $P \vee \neg P$ — where $P$ is "there are infinitely many twin primes" — proved? Classically, yes: you now know the answer. But constructively, what do you actually have? You have an oracle's testimony, not a proof. You know $P$ holds, but you don't have a constructive proof of $P$ (a procedure generating infinitely many twin pairs). And even if the oracle had told you $\neg P$, you would know finitely many exist, but not their exact number.

This thought experiment illustrates that in constructive mathematics, knowing the *truth* of a statement is not the same as having a *proof* of it. Proofs carry computational content that mere knowledge of truth does not.

Now suppose the oracle gave you a complete proof of $P$, including an explicit procedure for generating twin primes. Does this change the constructive status of $P \vee \neg P$? Yes — now you have a left injection $\mathsf{inl}(\text{proof of P})$, and $P \vee \neg P$ is proved.

What does this tell you about the relationship between truth and provability in constructive mathematics?

## 2. The Universe of Mathematical Ignorance

Consider a Kripke model where the worlds are "stages of mathematical knowledge" — actual stages in the history of mathematics. In 1850, we know the Fundamental Theorem of Algebra but not the Riemann Hypothesis. In 1900, we know the Prime Number Theorem. In 2000, we know Andrew Wiles's proof of Fermat's Last Theorem.

The accessibility relation is chronological: $w \leq v$ if $v$ is a later stage. The forcing relation: $w \Vdash P$ if $P$ is a theorem known at stage $w$.

In this model, LEM fails for the Riemann Hypothesis: at any current stage, we have neither a proof nor a disproof. The proposition "there are infinitely many twin primes OR there are finitely many" is not forced at the current world, because forcing a disjunction requires knowing which disjunct holds.

Now ask: what happens to this model as time progresses? If we prove the twin prime conjecture, we add a new world where the disjunction is forced (via the left disjunct). The Kripke model grows. The monotonicity condition ensures that what was forced before is still forced after.

This temporal reading of Kripke semantics captures Brouwer's idea that mathematical truth unfolds through time, through the activity of mathematical construction. Is this a useful picture of mathematical practice?

## 3. The Constructive Existence Paradox

Here is a classical existence proof that feels constructive but isn't. Consider: "There exist two irrational numbers $x$ and $y$ such that $x^y$ is rational."

Classical proof: consider $\sqrt{2}^{\sqrt{2}}$. Either it is rational — done, take $x = y = \sqrt{2}$ — or it is irrational. If irrational, take $x = \sqrt{2}^{\sqrt{2}}$ (irrational) and $y = \sqrt{2}$. Then $x^y = (\sqrt{2}^{\sqrt{2}})^{\sqrt{2}} = \sqrt{2}^2 = 2$, which is rational.

This proof uses LEM on the proposition "$\sqrt{2}^{\sqrt{2}}$ is rational." It is non-constructive: it does not tell you *which* pair $(x, y)$ works. You know such a pair exists, but you cannot say which.

(Incidentally, $\sqrt{2}^{\sqrt{2}}$ is irrational — this follows from the Gelfond-Schneider theorem — so the second case is the actual answer. But you would need to prove Gelfond-Schneider to know this constructively.)

How would you prove this constructively? Is there a pair $(x, y)$ with explicit values and a direct verification? Can you construct an algorithm that produces the pair without case-splitting on an undecided proposition?

## 4. Markov's Principle as a Bet

Here is an operational interpretation of Markov's principle. Suppose you are running a search program: you iterate $n = 0, 1, 2, \ldots$ and check whether $P(n)$ holds, where $P$ is a decidable predicate. The search has not found a witness yet.

Now you are told: "It is not the case that the search will never terminate." (This is $\neg\neg \exists n, P(n)$.) Should you continue running the search, confident it will eventually terminate?

Markov's principle says: yes. If you have been told (and proved) that the search is not permanently non-terminating, you can conclude it terminates — and therefore, keep running it.

The constructivist objection: you have no bound on how long the search takes. You cannot extract a specific $n$ from the double-negation — you only know that no $n$ witnessing termination will be missed if you search long enough. The search might take $10^{100}$ steps, or $10^{10^{100}}$ steps. Markov's principle asserts it terminates, but gives no computational bound.

Is Markov's principle justified constructively? Is "eventually terminates" a constructive property if you have no bound on "eventually"?

## 5. The Interpolant and the Lemma

Craig's interpolation theorem (a consequence of cut elimination) says: if $A \vdash B$, there is an interpolant $I$ using only vocabulary common to $A$ and $B$, with $A \vdash I$ and $I \vdash B$.

This is a classical theorem about classical logic. But what is its constructive status? A constructive proof of Craig interpolation would exhibit, given a proof of $A \vdash B$, an actual interpolant $I$ and proofs of $A \vdash I$ and $I \vdash B$.

In fact, Craig interpolation does hold constructively for intuitionistic logic (proved by various methods including Maehara's proof via sequent calculus and cut elimination). The constructive proof exhibits the interpolant by following the structure of the cut-free proof.

Now the philosophical question: if two facts $A$ and $B$ are "connected" (one implies the other), there is always an intermediate fact $I$ built from their common vocabulary that connects them. Does this mean mathematical facts are never entirely isolated — that any implication must pass through a common intermediary?

## 6. Brouwer's Fan Theorem vs. Classical Analysis

Brouwer proved a theorem that is classically trivial but constructively non-trivial: the *Fan Theorem* (a weak form of König's Lemma). It states that every finitary spread (a certain kind of infinite binary tree) is bounded — any infinite path through a tree with only finitely many extensions at each node must have a bound on how long the paths get.

Classically, this follows immediately from König's Lemma and compactness. Constructively, it requires careful argument because "infinite path" and "bounded" require explicit algorithms.

The Fan Theorem, combined with other intuitionistic principles, implies: every function $f : [0,1] \to \mathbb{R}$ that is pointwise continuous is *uniformly* continuous. Classically, this fails: pointwise continuity does not imply uniform continuity ($f(x) = x \sin(1/x)$ on $(0,1]$). But constructively, pointwise continuous functions on a compact domain must be uniformly continuous, because "pointwise continuous" already requires an algorithm that works for all points.

This is a case where constructive mathematics proves *more* than classical mathematics — a stronger conclusion — because the constructive hypothesis is richer than the classical one. The "same" hypothesis (continuity) has stronger constructive content.

What does this tell us about the relationship between constructive and classical mathematics? When is the constructive version of a classical statement stronger, and when is it weaker?

## 7. Realizability as a Semantics for Programs

Kleene's realizability assigns to each natural number a potential "witness" for a proposition. Number $e$ realizes proposition $P$ if the Turing machine with code $e$, given any realizer for the hypotheses of $P$, produces a realizer for the conclusion of $P$.

Under this interpretation, the axiom of choice is realizable (for countable domains) because it just says: if for each $n$ you have a procedure giving a witness, compose the procedures. And Church's thesis — that all functions are recursive — is consistent with realizability, because the "functions" in the BHK interpretation are already taken to be recursive.

But in HoTT, the "functions" in the BHK interpretation are not required to be recursive — they are elements of the type theory, which includes much more than recursive functions (it can encode non-computable types, if non-constructive axioms are added). The realizability semantics and the type-theoretic semantics agree when we restrict to the constructive core, but diverge when extensions are added.

What is the right notion of "function" for the BHK interpretation? Is it recursive functions, computable functions in some broader sense, or any mathematical function? The answer determines which principles (Markov's principle, Church's thesis, LEM) are consistent with the interpretation.
