# Gödel's First Incompleteness Theorem

> "If the proof that the system is consistent is carried out within the system itself, then it proves its own consistency, which is impossible for a consistent system."
> — John von Neumann, after hearing Gödel speak in 1930

## The Year Mathematics Changed

In September 1930, at a conference in Königsberg, a 24-year-old Austrian mathematician named Kurt Gödel announced, almost as an aside, a result that would transform the foundations of mathematics forever. David Hilbert — the most influential mathematician of the early 20th century, and the architect of a grand program to formalize and "complete" all of mathematics — was in the same city, giving a speech on the perfection of mathematics. He did not hear Gödel's announcement.

By 1931, Gödel's paper was published. Hilbert's program was over.

## Hilbert's Dream

Hilbert believed that mathematics could be:
1. **Complete**: Every mathematical statement is either provable or disprovable
2. **Consistent**: No contradictions can be derived
3. **Decidable**: There is a mechanical procedure to determine, for any statement, whether it is provable

This was the **Hilbert Program** — to put mathematics on an absolutely secure, mechanical, formal foundation. Hilbert's famous rallying cry: *"Wir müssen wissen, wir werden wissen"* — "We must know, we will know."

Gödel showed that goals 1 and 3 are impossible for any sufficiently powerful consistent system. (Turing independently showed the impossibility of goal 3 in a different form — the halting problem — in 1936.)

## Gödel Numbering: Making Arithmetic Talk About Itself

The key technical innovation is the **Gödel numbering** — a way to encode logical formulas and proofs as natural numbers, so that arithmetic can talk about its own syntax.

The idea: assign a unique number to each symbol, then encode sequences of symbols (formulas) and sequences of formulas (proofs) using prime factorization:

- Assign numbers to symbols: $\ulcorner\neg\urcorner = 1$, $\ulcorner\wedge\urcorner = 2$, $\ulcorner\forall\urcorner = 3$, $\ulcorner 0\urcorner = 4$, $\ulcorner S\urcorner = 5$, etc.
- Encode a sequence $(s_1, s_2, \ldots, s_n)$ as $2^{s_1} \cdot 3^{s_2} \cdot 5^{s_3} \cdots p_n^{s_n}$ (where $p_n$ is the $n$-th prime)
- The Gödel number of a formula $\varphi$ is written $\ulcorner \varphi \urcorner$

**Critical consequence**: Since provability is about manipulating formulas (which are now numbers) according to inference rules (which are now number-theoretic operations), provability itself becomes an arithmetically definable relation. There is an arithmetic formula $\text{Prf}(m, n)$ meaning "$m$ is the Gödel number of a proof of the formula with Gödel number $n$," and therefore an arithmetic formula $\text{Provable}(n) = \exists m\, \text{Prf}(m, n)$ meaning "the formula with Gödel number $n$ is provable."

This is the machinery that allows arithmetic to talk about its own proofs.

## The Diagonal Lemma (Self-Reference)

The **Diagonal Lemma** (also called the Fixed Point Lemma) is the technical heart:

**Lemma**: For any formula $\varphi(x)$ with one free variable, there is a sentence $\psi$ such that:
$$T \vdash \psi \leftrightarrow \varphi(\ulcorner \psi \urcorner)$$

In other words: for any property $\varphi$, there is a sentence that *says about itself* that it has property $\varphi$.

This is the formal version of "This sentence is false" — we can construct sentences that refer to themselves.

**Proof sketch**: Let $\text{sub}(m, n)$ be the Gödel number of the formula obtained by substituting the numeral $\bar{n}$ for the free variable in the formula with Gödel number $m$. This is an arithmetically computable function, so it is representable in $T$.

Let $\theta(x) = \varphi(\text{sub}(x, x))$ and let $k = \ulcorner \theta \urcorner$. Set $\psi = \theta(\bar{k})$.

Then $\psi = \theta(\bar{k}) = \varphi(\text{sub}(k, k)) = \varphi(\ulcorner \theta(\bar{k}) \urcorner) = \varphi(\ulcorner \psi \urcorner)$. $\square$

(This is the same diagonal trick as Cantor's: construct something by applying it to itself.)

## Constructing the Gödel Sentence

Apply the Diagonal Lemma to $\varphi(x) = \neg \text{Provable}(x)$:

There is a sentence $G_T$ such that $T \vdash G_T \leftrightarrow \neg \text{Provable}(\ulcorner G_T \urcorner)$.

$G_T$ says: **"I am not provable in $T$."**

This is the **Gödel sentence** for $T$.

## The Proof of Incompleteness

**Theorem (Gödel 1931)**: If $T$ is a consistent, recursively axiomatizable theory that contains Robinson Arithmetic (a weak fragment of Peano Arithmetic), then:

1. $T \nvdash G_T$ (the Gödel sentence is not provable in $T$)
2. $T \nvdash \neg G_T$ (the negation is not provable either)
3. $G_T$ is true in the standard model $\mathbb{N}$

**Proof of (1)**: Suppose $T \vdash G_T$. Since $T$ correctly represents provability, this means $T \vdash \text{Provable}(\ulcorner G_T \urcorner)$. But $G_T$ says $\neg \text{Provable}(\ulcorner G_T \urcorner)$. So $T$ proves both $\text{Provable}(\ulcorner G_T \urcorner)$ and $\neg \text{Provable}(\ulcorner G_T \urcorner)$ — that is, $T$ is inconsistent. Contradiction. $\square$

**Proof of (2)**: This requires the notion of $\omega$-consistency (or can be proved with simple consistency using the Second Incompleteness Theorem). The argument: if $T \vdash \neg G_T$, then $G_T$ is false (under $\omega$-consistency assumptions), meaning $G_T$ *is* provable in $T$. But then $T$ proves $G_T$ and $\neg G_T$ — inconsistency. $\square$

**Proof of (3)**: $G_T$ says it is not provable. We showed $T \nvdash G_T$. So $G_T$ is indeed not provable — which is exactly what $G_T$ says. In the standard model, $G_T$ is true. $\square$

## The Second Incompleteness Theorem

**Theorem**: If $T$ is consistent, then $T \nvdash \text{Con}(T)$, where $\text{Con}(T)$ is the arithmetic sentence asserting $T$'s consistency ("$T$ does not prove $\bot$").

**Proof sketch**: Show that $\text{Con}(T)$ implies $G_T$ is unprovable (formalize the proof of incompleteness *inside* $T$). So if $T \vdash \text{Con}(T)$, then $T \vdash G_T$ — but we showed $T \nvdash G_T$ (assuming $T$ is consistent). Contradiction. $\square$

**Significance**: Hilbert wanted to prove the consistency of strong theories like ZFC from weaker, more "obviously secure" methods. The Second Incompleteness Theorem rules this out: any system strong enough to express its own consistency proof cannot be that proof's tool. ZFC cannot prove ZFC is consistent. Peano Arithmetic cannot prove PA is consistent.

## What Incompleteness Does NOT Mean

Several common misunderstandings:

**Myth 1**: "Gödel proved mathematics is unreliable."
False. Gödel showed that any formal system for mathematics is *incomplete* — there are truths it cannot prove. This does not mean mathematics is inconsistent or that proofs are unreliable. Working mathematicians routinely prove things in ZFC that ZFC can verify.

**Myth 2**: "Gödel showed human intuition transcends computation."
This is a contested philosophical claim (associated with Penrose's "Gödelian argument"). The argument is: human mathematicians can see $G_T$ is true even though no formal system can prove it. But to see this, we must be working in a stronger system — and that system has its own unprovable Gödel sentence. There is no end to this, but there is also no clear sense in which humans "transcend" the formal limitations.

**Myth 3**: "Most important mathematical questions are undecidable."
False. The vast majority of mathematics is provable in ZFC. Gödel sentences are "artificial" — constructed specifically to be unprovable. Some genuinely interesting questions (like the Continuum Hypothesis) are independent of ZFC, but these are exceptional.

## Decidable Theories

Not everything is hopeless. Several important theories ARE complete and decidable:

| Theory | Decidable? |
|--------|-----------|
| Propositional logic | Yes (truth tables) |
| Presburger arithmetic (addition over ℕ, no multiplication) | Yes (Presburger 1929) |
| Real closed fields (ℝ with +, ×, <) | Yes (Tarski 1948) |
| Algebraically closed fields (ℂ) | Yes (Tarski) |
| Euclidean geometry | Yes (Tarski) |
| First-order arithmetic (with ×) | No (Gödel) |
| Set theory (ZFC) | No |

The distinction: multiplication introduces enough power to encode Gödel numbering. Without it (Presburger arithmetic), the theory is too weak to talk about its own provability.

## Connection to Computability

Gödel's incompleteness and Turing's halting problem are deeply related — in fact, they are two faces of the same phenomenon.

The halting problem: no algorithm can decide whether program $P$ halts on input $x$.

Gödel incompleteness: no consistent formal theory $T$ (that is computably axiomatizable and strong enough) can prove all true arithmetic sentences.

**The connection**: A sentence of arithmetic is provable in $T$ iff there is a halting computation that generates the proof. So undecidability of the halting problem implies there are arithmetic sentences whose provability cannot be decided — Gödel incompleteness.

More precisely: the set of theorems of any consistent, computably axiomatizable theory is computably enumerable but not computable (if the theory is complete, it would be computable by enumerating all proofs — but a computable complete consistent theory cannot exist by Gödel).

## Exercises
See [problems/ch10_computability/03_incompleteness_exercises.md](../../../problems/ch10_computability/03_incompleteness_exercises.md)
