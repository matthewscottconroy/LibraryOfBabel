# What Is a Proposition?

> *"The proposition is a picture of reality."*
> — Wittgenstein, *Tractatus Logico-Philosophicus*, 4.01

---

Before we can study logic — before we can define validity, construct proofs, or write a single Lean theorem — we need to identify what logic operates *on*. What is the subject matter of logical reasoning? The answer almost everyone gives is: **propositions**. But what, exactly, is a proposition?

This turns out to be a surprisingly deep question, and the different answers philosophers have given to it illuminate fundamental issues in logic, metaphysics, and the philosophy of language. More practically, how we answer it determines what kind of formal system we build. Let us work through the question carefully.

## The Simple Answer and Its Problems

Here is a first pass: a proposition is *what a declarative sentence expresses* — a claim that is either true or false. "The Eiffel Tower is in Paris" expresses a true proposition. "The Eiffel Tower is in London" expresses a false one. "Is the Eiffel Tower in Paris?" is not a proposition — it is a question. "Close the door" is not a proposition — it is a command.

This is essentially right, but it immediately raises a complication: *different sentences can express the same proposition*. Consider:

- "Snow is white."
- "La neige est blanche."
- "Schnee ist weiß."

These three sentences, in English, French, and German, express the same proposition. The proposition is not the sentence — it is what the sentence *says*, abstracted away from the particular words and their arrangement.

Now consider the complication in the other direction: the same sentence can express *different* propositions depending on context. "I am hungry" says something different depending on who "I" refers to. "It is raining" says something different depending on when and where it is uttered. These **indexical** sentences have context-dependent meaning: their propositional content shifts with the context of utterance.

## Frege's Decomposition: Sense, Reference, and Thought

Gottlob Frege gave the analysis that still shapes contemporary philosophy of language. In his 1892 paper "On Sense and Reference" (*Über Sinn und Bedeutung*), Frege distinguished between:

- **Reference** (*Bedeutung*): what an expression picks out in the world — the object or truth value it refers to.
- **Sense** (*Sinn*): the *mode of presentation* — how the expression presents its referent.

Frege's motivating example is this: "The morning star" and "the evening star" both refer to the same object — the planet Venus. But they differ in sense: they present Venus in different ways (as the last star visible in the morning versus the first star visible in the evening). This is why the sentence "The morning star is the evening star" is *informative*, expressing an empirical discovery, while "The morning star is the morning star" is trivially true.

For propositions, Frege identified them with *Thoughts* (Gedanken): abstract, mind-independent objects that are the senses of declarative sentences. A Thought is neither mental (it does not live in anyone's mind) nor physical (it is not in the world), but a third kind of entity in a "third realm." When you and I both think about the proposition that 2+2=4, we are both grasping the same Thought — not two psychological duplicates but a single abstract object.

This **Platonist** view of propositions — that they are abstract, mind-independent entities — is the default assumption of classical logic. It has the significant advantage of explaining how communication is possible: two people can understand the same mathematical theorem because they have access to the same abstract propositional content.

## Russell's Structured Propositions

Bertrand Russell took a different view. For Russell, propositions are not abstract senses but structured complexes that contain the actual objects, properties, and relations they are about. The proposition expressed by "Socrates is mortal" contains Socrates himself (the man, not a description of him) and the property of mortality.

This view — **Russellian propositions** or **direct reference theory** — makes propositions dependent on the world in a deeper way. It also generates puzzles. What happens to the proposition "The present king of France is bald" when France has no king? For Frege, such sentences fail to express a complete proposition (the subject term lacks a referent). For Russell, the sentence actually has a *logical form* different from its grammatical form: it says something like "there exists a unique present king of France, and that entity is bald" — an existential claim that is straightforwardly false, not meaningless.

The debate between Fregean and Russellian approaches remains active in philosophy of language. For our purposes, the important takeaway is that there are genuine, deep questions about what propositions are — and that these questions have formal consequences.

## The Liar's Challenge

Whatever propositions are, they are supposed to have determinate truth values. But consider: does "This proposition is false" have a truth value? If it is true, it is false; if it is false, it is true. This is the **Liar paradox** applied not to sentences but to propositions.

The Liar is evidence that not every declarative sentence expresses a proposition with a well-defined truth value. Formal logic sidesteps this by working only with well-formed formulas in formally specified languages — languages that are carefully designed to prevent self-reference of the troublesome kind.

> **A Deeper Puzzle**: Consider: "The proposition expressed by the English sentence 'The proposition expressed by this sentence is false' is false." Does *this* sentence express a proposition? (This version, due to Saul Kripke, is harder to dismiss than the original Liar. Kripke's 1975 paper "Outline of a Theory of Truth" is one of the most important papers in philosophical logic — well worth reading.)

## Propositions in Logic and Proof Assistants

For practical purposes in this textbook, we will adopt a working definition: **a proposition is any expression that has a determinate truth value relative to a specified interpretation**. We will not require that propositions exist as abstract Platonic entities; we only require that they be truth-apt — capable of being true or false.

In Lean 4 and Coq, this is made precise by the type system. Propositions are terms of type `Prop`. A proof of a proposition P is a term of type P. This is the **Curry-Howard correspondence** (Chapter 11): propositions are types, and proofs are programs. The deep philosophical question of "what is a proposition?" is answered, pragmatically, by "whatever has type `Prop` in Lean."

This answer is not philosophically neutral — it embeds a broadly constructive, type-theoretic picture of mathematics. But it is an extraordinarily productive answer: it is the foundation on which Lean's Mathlib library (containing hundreds of thousands of formally verified mathematical theorems) is built.

---

## Real-World Applications

**Knowledge representation in AI**: Automated reasoning systems represent knowledge as propositions (or more generally, as first-order sentences). A knowledge base in a system like Prolog or an OWL ontology is a structured collection of propositional content. The inference engine derives new propositions from existing ones using logical rules. The question "what is a proposition?" becomes, in this context, the engineering question "what is the data structure we use to represent a claim?"

**Natural language semantics**: The compositional semantics of natural language (as studied in formal linguistics, following Montague Grammar) assigns propositions to sentences by recursive rules. The proposition expressed by a sentence is a function from possible worlds to truth values — a **set of possible worlds**. This is the semantic framework that underlies much of formal linguistics and some branches of philosophy.

---

*Next: We examine what it means for a proposition to be true, and the different theories of truth that philosophers have proposed.*
