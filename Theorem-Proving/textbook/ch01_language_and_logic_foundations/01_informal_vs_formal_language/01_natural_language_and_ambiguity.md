# Natural Language and Ambiguity

> *"The limits of my language are the limits of my world."*
> — Ludwig Wittgenstein, *Tractatus Logico-Philosophicus*

---

Consider this sentence, lifted verbatim from an actual court case:

> "No vehicles in the park."

Simple enough, right? Now answer these questions: Does it prohibit a baby's pram? An ambulance rushing to an emergency? A decommissioned WWII tank installed as a monument? A toy car? The very process of trying to answer these questions reveals something deep and unsettling about natural language: even our clearest utterances carry hidden ambiguities, edge cases, and interpretive gaps that only become visible when we push them.

This is not a quirk of legal English. It is a fundamental feature of human language, and it has profound consequences for anyone who wants to *reason rigorously*. Logic is the discipline of reasoning without error. Proof is the act of establishing truth beyond reasonable doubt. Neither is possible if the language we use to formulate our claims can be silently understood in multiple incompatible ways. Before we can build the machinery of formal logic, we need to understand what we are escaping — and why escape is necessary.

## The Three Faces of Ambiguity

Linguists distinguish several sources of ambiguity in natural language. We will focus on the three most philosophically important.

### Lexical Ambiguity: One Word, Many Meanings

The most familiar kind is **lexical ambiguity**: a single word that maps to multiple unrelated concepts. When someone tells you "I went to the bank," you need context to know whether they withdrew money or skipped stones. The word *bank* is simply overloaded — two different concepts happen to share a label.

In everyday conversation this is harmless. Context, tone, and shared knowledge quietly resolve the ambiguity before you even notice it was there. But consider what happens when we try to build a formal argument. If a premise contains an ambiguous word, the argument might be valid under *one* reading of the word and invalid under another. We can be tricked into accepting a conclusion that does not actually follow, because we unconsciously shifted interpretations midway through.

Medieval logicians called this the *fallacy of equivocation*, and they considered it one of the most insidious errors in reasoning. Here is a classic example:

> Nothing is better than perfect happiness.
> A ham sandwich is better than nothing.
> Therefore, a ham sandwich is better than perfect happiness.

The word "nothing" shifts meaning between the first premise (where it means "no thing that exists") and the second (where it means "the state of having nothing"). Both premises sound reasonable under their respective readings. But the argument is nonsense — and the nonsense is hidden in plain sight.

### Syntactic Ambiguity: One Sentence, Multiple Structures

More treacherous still is **syntactic ambiguity**: sentences that are grammatically well-formed but whose structure admits multiple parsings, each with a different meaning.

Consider: *"I saw the man with the telescope."* Did I use a telescope to see him, or was he carrying a telescope? The words are identical; the grammar permits both readings equally. A constituency parser — the kind of program that tries to build a tree structure from a sentence — will cheerfully produce both and have no principled way to choose between them without context.

Now consider an example where the ambiguity has mathematical teeth:

> "Every student read a book."

This sentence is ambiguous between two very different claims:
- **Reading 1**: Each student read at least one book (possibly a different book for each student).
- **Reading 2**: There is a specific book that every student read.

In the formal notation of first-order logic (which we will develop in Chapter 3), these are:

$$\forall s \, \exists b \; \text{Read}(s, b) \qquad \text{vs.} \qquad \exists b \, \forall s \; \text{Read}(s, b)$$

These are genuinely different statements. The first could be true in a library where each student picks their own book; the second requires a single shared text. The order in which the quantifiers ∀ (*for all*) and ∃ (*there exists*) appear determines which reading we intend — and natural language does not tell us which order applies.

This is not a pedantic concern. The history of philosophy is littered with arguments that appeared to establish deep truths but actually exploited exactly this kind of quantifier ambiguity. Aristotle's logic, brilliant as it was, lacked the formal machinery to distinguish them cleanly. It took until Gottlob Frege's *Begriffsschrift* in 1879 for this ambiguity to be resolved once and for all — and that resolution is what made modern mathematics possible.

### Pragmatic Ambiguity: What Is Said vs. What Is Meant

The third kind is **pragmatic ambiguity**: the gap between what a sentence literally says and what a speaker means by uttering it. When your friend says "Nice haircut," you might hear a sincere compliment or dripping sarcasm. When a colleague asks "Can you help me move this sofa?", they are technically asking whether you are physically capable, but what they mean is "Will you help me?"

Pragmatic ambiguity arises from the fact that language is a social tool, not just a representational one. Utterances carry implicatures, presuppositions, speech acts. The sentence "It's cold in here" is, on the surface, a meteorological report; in the right context it is a request to close the window.

For the purposes of logic and mathematics, we eliminate pragmatic ambiguity by adopting a convention: we always interpret sentences as *literal* and *context-independent*. A mathematical proof is not sarcastic. It does not hint or imply. Every sentence says exactly what it says, no more and no less.

## Vagueness: A Different Problem Entirely

It is worth pausing to distinguish ambiguity from **vagueness**, because they are often conflated. An ambiguous term has multiple *distinct* meanings — "bank" means either financial institution or riverbank, and there is no middle ground. A *vague* term has a single meaning but an indeterminate extension: cases where there is genuinely no fact of the matter about whether the term applies.

"Tall" is the canonical example. Is someone who is 5'11" tall? What about 5'10"? 5'9"? There is a clear center (7 feet is definitely tall) and a clear periphery (4 feet is definitely not tall), but a broad penumbra where our concept simply does not deliver a verdict. This is not because we lack information — it is because the concept itself has no sharp boundary.

The ancient Greeks discovered this through the **Sorites paradox** (*sorites* = heap, from the Greek *soros*). Suppose you have a heap of sand. Remove one grain: still a heap. Remove another: still a heap. Surely removing a single grain can never transform a heap into a non-heap. Yet if you apply this reasoning ten thousand times, you are left with a single grain of sand — which is not a heap. Something has gone wrong, but what?

The Sorites paradox is genuinely difficult. Philosophers have proposed: (1) Epistemic theories — there *is* a sharp boundary, we just cannot know it; (2) Supervaluationism — the sentence is "super-true" if true on all ways of sharpening the vague concept, "super-false" otherwise; (3) Degree theories — truth comes in degrees between 0 and 1; (4) Just-so theories — vague predicates simply do not have precise extensions, and classical logic breaks down in the penumbra.

For our purposes, the key point is this: formal languages eliminate both ambiguity *and* vagueness simultaneously, by fiat. When we define a formal predicate `Prime(n)`, we mean precisely: n > 1 and n has no positive divisors other than 1 and itself. No borderline cases. No interpretive flexibility. The price of this precision is that our formal language is much weaker in expressive richness than natural language. But in mathematics and logic, that is a price we are delighted to pay.

## Why This Matters: The Fatal Flaw of Informal Proof

Here is a sobering historical fact. In the nineteenth century, Augustin-Louis Cauchy — one of the greatest mathematicians who ever lived — published a "proof" of the following theorem: *The limit of a convergent sequence of continuous functions is continuous.*

The proof was wrong. It confounded the order of quantifiers in a subtle way, conflating pointwise convergence (∀x∀ε∃δ) with uniform convergence (∀ε∃δ∀x). Every working mathematician at the time missed the error. It was Niels Abel who first noticed it, commenting darkly that there were "so many failures and paradoxes" in the analysis of the day.

The root cause was not computational — it was *linguistic*. The informal language of nineteenth-century mathematics lacked the precision to make the quantifier order explicit. The ambiguity was built into the prose.

This is why we care about formal language. Not as a pedantic exercise, but because the history of mathematics is full of brilliant people making errors that formal precision would have caught instantly. The type-checker in Lean or Coq is not a bureaucratic nuisance — it is a mathematical safety net woven from the lessons of two centuries of near-misses.

## What Formal Languages Achieve

A **formal language** strips language down to bare essentials: an alphabet of symbols, a grammar that specifies exactly which strings are well-formed, and a semantics that assigns precise meanings. Nothing is left implicit. Nothing is resolved by context.

When you write `∀x(Prime(x) → ∃y(Prime(y) ∧ y > x))` in first-order logic, there is no ambiguity about quantifier scope (the ∀ has widest scope, then the →, then the ∃), no ambiguity about what Prime means (you defined it), and no implicit reliance on context. Every reader — human or machine — sees exactly the same structure.

> **A Thought Experiment**: Imagine trying to write a formal proof in English. Every sentence you write must be *completely* unambiguous — no pronouns that could refer to multiple antecedents, no implicit assumptions, no scope left open to interpretation. Try it with any mathematical theorem you know. How many words do you need? How many clarifying parentheticals? You will find that you are essentially reinventing formal notation, awkwardly and imperfectly, in natural language. This is not a coincidence.

The tools in this textbook — Lean 4, Coq, Tarski's World / Carnap — embody this insight. They are not just calculators for logic. They are *formal languages with teeth*: languages that refuse to accept anything ambiguous, that force you to be precise, and that reward precision with machine-checked certainty.

In Lean 4, for instance, every term has a unique type, and the elaborator rejects any expression whose type is ambiguous. In Coq, every logical step must be justified by an explicit inference rule. These are not limitations — they are the point. The discomfort you may feel when the proof assistant demands more precision than you thought was necessary is exactly the discomfort of having your ambiguities surfaced and corrected.

## The Cost of Precision

There is a genuine trade-off here, and it would be intellectually dishonest not to acknowledge it. Natural language is extraordinarily expressive precisely *because* of its flexibility. Metaphor, irony, implicature, pragmatics — these are features, not bugs. Poetry cannot be formalized. Love letters should not be.

Mathematical proofs sit at one extreme of the expressiveness-precision spectrum, and for good reason: in a proof, an ambiguity is not a stylistic flourish but a potential error. We are not writing poetry; we are building bridges over chasms of uncertainty, and those bridges need to hold.

As you work through this textbook, you will develop a bilingual competence: the ability to think in the rich, flexible medium of natural language, and to translate those thoughts into the crystalline precision of formal notation. This translation skill — knowing both what to formalize and *how* to formalize it — is one of the deepest skills a logician or mathematician can possess.

---

## Tool Connections

**Tarski's World / Carnap (carnap.io)**: Tarski's World makes the syntax/semantics gap *visible*. You write a sentence in the formal language of blocks — `∀x(Cube(x) → Large(x))` — and immediately see whether it is true or false in a specific arrangement of blocks. The same sentence is true in some worlds and false in others, and you cannot hide behind ambiguity to avoid this verdict.

**Lean 4 and Coq**: These proof assistants are, among other things, formal language processors. They refuse to typecheck ambiguous expressions. When you write `h.1` to extract the left component of a conjunction `h : P ∧ Q`, the system knows exactly what you mean because the type of `h` is unambiguous. The "type error" messages you will inevitably encounter are the system's way of telling you that you have left something ambiguous.

**Python (NLTK, spaCy)**: Natural language processing tools make linguistic ambiguity *computational*. A constituency parser will produce multiple parse trees for an ambiguous sentence, making the hidden structure visible as data. This is useful not just for understanding language, but for understanding why formal languages are designed differently.

---

## Real-World Applications

**Legal language**: The history of law is a history of ambiguity. Contract disputes, statutory interpretation, constitutional law — the central act of legal reasoning is deciding what ambiguous language means in a specific case. Centuries of common law can be understood as an incremental project to clarify the meaning of words through accumulated precedent. Legal drafters who have studied logic write clearer documents and generate less litigation.

**Software engineering**: Every programming language is a formal language, carefully designed to be unambiguous. But the *specifications* of software — what the program is supposed to do — are often written in natural language, and natural language specifications are notoriously ambiguous. "The system shall process requests in a timely manner." What counts as timely? Countless software projects have failed because the specification and the implementation were both internally consistent but mutually inconsistent. Formal specification languages (TLA+, Alloy, Z) address this by bringing formal language into the requirements phase.

**Artificial intelligence**: One of the hardest problems in AI is understanding natural language — precisely because of ambiguity. A language model that has read the entire internet still cannot be said to "understand" a sentence in the way a logician can: as a formal object with a definite truth condition in a given world. The gap between surface form and logical form is the gap between pattern matching and genuine comprehension.

---

*Next: We examine how formal languages are constructed from scratch — the idea of a grammar as a precise, recursive recipe for building well-formed expressions.*
