# Disquotationalism

Disquotationalism is the most austere form of deflationism about truth. It takes its name from the core idea that the truth predicate's sole function is to *disquote*: to move from mention to use, from "'Snow is white' is true" to "Snow is white." W. V. O. Quine is the most influential defender, though he treated his view as continuous with a broader deflationism rather than as a wholly distinctive thesis.

The disquotational account is framed in terms of sentences rather than propositions. For each sentence *s* of our language, the T-sentence — "'s' is true iff *s*" — exhaustively specifies the extension of the truth predicate for *s*. Truth is not a property that sentences share by virtue of some relation to the world; it is simply whatever is encoded in the totality of T-sentences.

Tarski's T-schema: T(⌜s⌝) ↔ s

Consider: T(⌜snow is white⌝) ↔ snow is white. This schema, applied to every sentence of the object language, yields a definition of truth for that language. The key insight is that we never need to appeal to any feature of the world beyond what is already captured in the right-hand side of the T-sentence. Quine formulated this in *Philosophy of Logic* (1970): "Disquotation is the whole of the concept." Truth is a device for semantic ascent — for talking about sentences rather than about the world — and for blind endorsement of claims we cannot enumerate individually. "Everything Newton said about motion is true" is equivalent to the (possibly infinite) conjunction of Newton's claims about motion, but we cannot state that conjunction explicitly; truth-talk lets us endorse it wholesale:

∀s(Aristotle-claimed(s) → T(s))

Without the truth predicate, this cannot be rendered as a first-order sentence, because "s" ranges over sentences (linguistic objects), not over what the sentences say. Truth lets us "mention" sentences in quantifiers and then "use" their content via the disquotational schema.

**Tarski's formal achievement**

The main technical achievement that anchors disquotationalism is Tarski's recursive definition of truth for formal languages in "The Concept of Truth in Formalized Languages" (1933/1956). The definition proceeds via *satisfaction*: a sentence is satisfied by a sequence of objects iff the objects appropriately make the sentence "come out true." Atomic satisfaction is defined by the reference of names and the extension of predicates; complex satisfaction is defined recursively by the clauses for logical connectives and quantifiers. The key theorem: a sentence *s* is true iff *s* is satisfied by all sequences. This definition entails all T-sentences as theorems.

Tarski himself was ambivalent about whether his definition captured a *real* property of truth, noting that it gave a definition "formally correct and materially adequate" — but what "materially adequate" means remained contested. Deflationists read Tarski as vindicating the T-schema-centric approach; realists read him as providing the formal structure of a correspondence theory (the recursive clauses define how the semantic properties of complex sentences are determined by the semantic properties of their parts, which are in turn determined by reference relations to the world).

**The sentence versus proposition problem**

Disquotationalism, by operating on sentences, faces a significant difficulty: the T-schema "'Snow is white' is true iff snow is white" gives a truth condition only for the English sentence "Snow is white." It tells us nothing directly about translations, about what ancient Greek speakers meant, or about whether propositions (if they exist as abstract entities) are true or false. Quine was prepared to accept this limitation: truth is essentially a property of the sentences of our own language, relativized to our interpretive practices.

Critics press that this parochialism is a serious defect. If "Caesar crossed the Rubicon" (Latin) was true, what makes it true? Not the English T-sentence for the English translation, which concerns a different sentence. The disquotationalist must either (a) restrict truth-talk to our own language, (b) extend it via translation, or (c) shift to propositions as the truth-bearers. Option (c) makes disquotationalism a variant of minimalism (Horwich), which takes propositions as the primary truth-bearers and formulates the T-schema for propositions. Paul Horwich's minimalism differs from Quine's disquotationalism precisely here:

- Quine: "Snow is white" is true iff snow is white.  (truth as predicate of sentences)
- Horwich: The proposition that snow is white is true iff snow is white.  (truth as predicate of propositions)

This difference matters because propositions, if they exist, are language-independent: the English sentence and its French translation express the same proposition, and both are true iff snow is white. Horwich's minimalism avoids the parochialism objection at the cost of requiring propositions as entities — which Quine rejected as obscure.

**The Liar paradox**

The T-schema has well-known paradoxical instances. The Liar sentence *L*: "This sentence is not true" generates contradiction when the T-schema is applied unrestrictedly:

- T(⌜L⌝) ↔ L   (by the T-schema)
- L ≡ ¬T(⌜L⌝)   (by definition of *L*)
- Therefore: T(⌜L⌝) ↔ ¬T(⌜L⌝)   (contradiction)

Tarski's solution was to distinguish object language from metalanguage: T-sentences for the object language are formulated in the metalanguage, and the Liar sentence is not a sentence of the object language (it refers to its own truth, crossing levels). This hierarchical solution requires a principled account of what makes a sentence a "legitimate" truth-bearer — and explaining those exclusions seems to require a substantive account of truth that goes beyond the T-schema.

Tarski's formal treatment introduced a crucial tool: the object language / metalanguage distinction. The truth predicate for a language *L* is always defined in a metalanguage *ML* that is richer than *L*:

- Object language L₀: contains no truth predicate
- Metalanguage L₁: contains the truth predicate for L₀, defined by the T-sentences for L₀
- Meta-metalanguage L₂: contains the truth predicate for L₁

This hierarchy is formally elegant but philosophically problematic: natural languages seem to contain their own truth predicates ("This sentence is true"), so natural language truth cannot be modeled by Tarski's hierarchy without significant restriction. Kripke's alternative (*Outline of a Theory of Truth*, 1975) uses partial logic and a fixed-point construction to model truth in a language with its own truth predicate, at the cost of denying that the Liar sentence has a truth value — truth-value gaps.

Disquotationalism also seems unable to account for why truth is a *norm* — why we should aim at truth rather than at coherence or utility. The T-schema tells us that "snow is white" is true iff snow is white, but it does not explain why the truth of our beliefs matters, or why true beliefs reliably guide action better than false ones. Stephen Leeds (1978) offered the most radical disquotationalist response: truth is an *empirical* notion, and its utility in our practice derives from the fact that human beings are reliable truth-trackers in certain domains by evolution and learning. The norm of truth follows from the fact that true beliefs reliably lead to successful action, not from any metaphysical story about the nature of truth. Critics argue this requires more than the T-schema to sustain.

Hartry Field's "pure disquotationalism" (*Truth and the Absence of Fact*, 2001) argues that the only notion of truth needed in a naturalistic theory of the world is the disquotational notion. Any appearance of truth playing an explanatory role beyond the T-schema can be fully captured by the disquotational notion together with other naturalistic facts about human psychology and behavior. Field's naturalism thus underpins the disquotationalist program with an empirical story about why truth-tracking creatures succeed — but, as the challenges to deflationism that follow show, not everyone finds this sufficient.
