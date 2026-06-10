# Important Terms and Ideas: Modal Logic

**Kripke Frame**
A structure ⟨W, R⟩ where W is a non-empty set of possible worlds and R is a binary accessibility relation on W. Different constraints on R (reflexivity, transitivity, symmetry) correspond to different modal axioms and capture different conceptions of what necessity requires.

**Accessibility Relation**
A relation R between worlds such that wRv means world v is accessible from world w—roughly, v is a genuine possibility relative to w. A proposition is necessary at w just in case it is true at every world accessible from w.

**Modal Axioms (T, 4, B, 5)**
Key axioms that distinguish modal systems: T (□P → P, necessity implies truth), 4 (□P → □□P, necessities are necessarily necessary), B (P → □◇P, actuality is necessarily possible), and 5 (◇P → □◇P, possibilities are necessarily possible). Systems T, S4, and S5 combine these axioms with increasing strength.

**Barcan Formula**
◇∃xFx → ∃x◇Fx: if it is possible that something is F, then there is something that is possibly F. Its converse is ∃x◇Fx → ◇∃xFx. These formulas raise questions about whether the domain of quantification is fixed across worlds (necessitism) or varies (contingentism).

**Completeness**
A modal logic is complete with respect to a class of frames if every formula that is valid on all frames in the class is a theorem of the logic. Kripke's semantics enabled completeness proofs for standard systems, giving modal logic the same mathematical respectability as classical logic.

**Normal Modal Logic**
A modal system is normal if it contains all tautologies, the K axiom (□(P→Q) → (□P → □Q)), and is closed under modus ponens and necessitation (if ⊢P then ⊢□P). K is the weakest normal modal logic; S5 is the strongest standard system.

**Quantified Modal Logic**
The extension of modal propositional logic to include quantifiers. Raises questions about possibilist vs. actualist quantification (whether ∃x ranges over merely possible objects), variable domain semantics, and the proper treatment of identity and existence across worlds.
