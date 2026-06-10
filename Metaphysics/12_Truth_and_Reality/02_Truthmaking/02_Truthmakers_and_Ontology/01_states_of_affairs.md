# States of Affairs as Truthmakers

The most influential answer to the question "what are truthmakers?" is states of affairs. A state of affairs is a complex entity consisting of an object (or objects) instantiating a property (or relation). The state of affairs that *a* is *F* exists just in case *a* instantiates *F*; it "obtains" or is "actual." States of affairs that do not obtain are merely possible: the state of affairs of the rose's being blue exists as a possibility but is not actual.

**Armstrong's argument**

D. M. Armstrong, the most systematic defender, argues that states of affairs are needed because neither particulars nor universals alone can serve as truthmakers. The particular rose cannot make "the rose is red" true, because the very same rose exists in worlds where it is not red. The universal redness cannot make it true, because redness exists in worlds where the rose lacks it. Only the state of affairs — the rose's-being-red — exists in exactly the worlds where the proposition is true.

More precisely:

- P1: A truthmaker for *p* must be an entity whose existence necessitates *p*.
- P2: Particulars alone do not necessitate their predicative truths: the rose exists in worlds where it is not red.
- P3: Universals alone do not necessitate their instantiations: redness exists in worlds where the rose lacks it.
- P4: The state of affairs (the rose's instantiating redness) exists in exactly the worlds where "the rose is red" is true.
- C: States of affairs are the appropriate truthmakers for atomic predicative truths.

This argument has an elegant structure. It shows why we need states of affairs rather than merely particulars or universals, by a process of elimination. The necessity condition on truthmaking (T ⊩ p iff □(T exists → p)) is the key: only the state of affairs satisfies it for atomic predicative propositions.

**The ontological status of states of affairs**

States of affairs have a distinctive ontological status: they are "thick" particulars, in Armstrong's terminology, containing both an object and a property (or relation) as constituents, but themselves being something more than the mere sum of their constituents. The state of affairs that *a* is *F* is not identical to {a, F} — the set containing *a* and *F*. If it were, the existence of *a* and *F* would entail the existence of the state of affairs, which is not so (*a* can exist without being *F*).

The standard formulation: the state of affairs [a is F] is distinct from the mereological sum a + F, because *a* and *F* can both exist without [a is F] existing (if *a* is not *F*), while [a is F] can exist without any further mereological addition — it just is the obtaining of the relation between *a* and *F*. Armstrong calls this the "tie" or "nexus" that binds constituents into states of affairs. Critics (Lewis, van Inwagen) find the nexus mysterious: what kind of entity is it, and why does it not itself require a further nexus to bind it to *a* and *F*?

This is Bradley's regress: if the state of affairs [a is F] requires a relation R to bind *a* and *F*, then [a R F] is itself a state of affairs — requiring a further relation R' to bind *a*, R, and *F*, and so on. Armstrong's response is that the binding in states of affairs is "non-relational" — the nexus is not a further relational entity but is internal to the structure of states of affairs. Whether this is satisfying is disputed.

**Armstrong's ontological system**

Armstrong's ontology culminates in a world of states of affairs: the world is the totality of obtaining states of affairs, not of things. This adapts rather than endorses Wittgenstein's *Tractatus* formulation. Laws of nature, for Armstrong, are second-order states of affairs relating universals — the nomic relation N(F, G) is a state of affairs consisting of the universals F and G standing in a nomic relation. This gives truthmakers for laws without reducing them to regularities (Humean regularity theory) or to mere possible-worlds talk. Specifically: the law "All Fs are Gs" is made true by the state of affairs N(F, G) — the universals F and G standing in the nomic necessitation relation. This is a higher-order state of affairs, but it is actual: the nomic relation between universals is part of the furniture of the actual world.

The system can be represented formally:

- Particulars: a, b, c, ...
- Universals: F, G, R, ...
- Atomic states of affairs: [Fa] (a instantiates F), [Rab] (a stands in R to b)
- Totality state of affairs: T_total (the world's completeness)
- Second-order states of affairs: [N(F,G)] (the nomic relation between universals F and G)

The truthmaker function TM: Propositions → States of Affairs:
- TM("Fa") = [Fa]
- TM("∃x Fx") = [Fa] for some a that instantiates F
- TM("¬Fa") = T_total (together with the absence of any [Fa]-type in T_total)
- TM("All Fs are Gs") = [N(F,G)]

The weaknesses — the totality state of affairs, the second-order nomic relation — are precisely the points at which critics object that the posits are ad hoc.

**Wittgenstein's Tractarian version and its contrast**

Wittgenstein's earlier formulation in the *Tractatus* is closely related but not identical. "The world divides into facts" (TLP 1.2) and facts are obtaining states of affairs (TLP 2). Objects are simple, uncombined entities; atomic states of affairs are the configurations of objects. True propositions "picture" obtaining states of affairs; false propositions "picture" non-obtaining states of affairs.

The key difference from Armstrong: Wittgenstein's objects are simples whose nature is exhausted by their possibilities of combination — they have no intrinsic properties beyond their combinatorial possibilities. Armstrong's particulars and universals have intrinsic natures. For Armstrong, a universal like CHARGE is a real entity with a nature, not merely a combinatorial possibility.

Lewis (1998) presses a deep objection: if states of affairs are needed as truthmakers, we should ask what makes the state of affairs exist. If the state of affairs [a is F] exists iff *a* instantiates *F*, then the state of affairs is grounded in the instantiation relation — and if instantiation is itself a relation, we have another state of affairs generating a regress. If instantiation is not a relation, it is unclear what it is. The nominalist alternative: there are no states of affairs, universals, or instantiation relations. Truthmakers for "the rose is red" are simply the rose itself, understood as a "thick" particular that includes all its qualitative character. This requires a different ontology — tropes, or bare particulars plus properties as sets — but avoids the state-of-affairs regress.

The state-of-affairs ontology has a clear historical trajectory: Russell's logical atomism (1918) introduced atomic facts as the basic truthmakers. Wittgenstein's *Tractatus* (1921) refined this into a full-scale metaphysics of facts, objects, and their combinatorial possibilities. The logical empiricists largely abandoned fact-talk in favor of verification conditions. The truthmaker framework was revived by C. B. Martin in Australian metaphysics (1980s), developed by Armstrong into a systematic ontological project, and extended by Mulligan, Simons, and Smith in their influential paper "Truth-Makers" (1984). Contemporary truthmaker theory — connecting states of affairs to grounding, essence, and the metaphysics of modality — represents the further development of this tradition in analytic metaphysics since 1990.
