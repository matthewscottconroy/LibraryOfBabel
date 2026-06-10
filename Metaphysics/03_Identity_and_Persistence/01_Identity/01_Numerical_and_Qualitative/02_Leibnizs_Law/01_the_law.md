# Leibniz's Law

*The formal statement and philosophical significance of the principle governing identity and property-sharing.*

---

Leibniz's Law is often stated as a biconditional: a = b if and only if a and b share all their properties. But the two halves of this biconditional are not on equal footing, and keeping them distinct is essential. The first half — that identity implies property-sharing — is a logical truth. The second — that property-sharing implies identity — is a contested metaphysical principle. Much philosophical mischief results from treating both as unproblematic.

The *Indiscernibility of Identicals* (II) states: if a = b, then for every property P, Pa ↔ Pb. This direction is universally accepted as trivially true: numerically identical things cannot differ in properties because they are *one* thing. The *Identity of Indiscernibles* (PII) states: if a and b share all their properties, then a = b. This says qualitative indistinguishability implies numerical identity — a claim that is both substantive and contested.

Both principles can be rendered in second-order logic, where quantification over predicates (properties) is permitted:

- **(II)** a = b → ∀F(Fa ↔ Fb)
- **(PII)** ∀F(Fa ↔ Fb) → a = b
- **Full Leibniz's Law**: a = b ↔ ∀F(Fa ↔ Fb)

In first-order logic, (II) appears as an axiom schema: for each open formula φ(x), the axiom a = b → (φ(a) ↔ φ(b)) is included. This restricts the universal second-order quantification over properties to the particular first-order predicates available in the language.

## Historical Provenance

Leibniz formulated the biconditional in his *Discourse on Metaphysics* (§9) and in correspondence with Arnauld. For Leibniz, PII was a necessary metaphysical truth grounded in the principle of sufficient reason: there is no sufficient reason for God to create two numerically distinct but qualitatively identical things, since their diversity would be a distinction without a difference. The Indiscernibility of Identicals, meanwhile, is a logical truth: numerical identity is the relation of an individual to itself, and an individual cannot differ from itself.

The formulation entered logic through Frege and Russell. In *Principia Mathematica*, Russell defines identity in terms of shared properties: x = y ↔ ∀F(Fx ↔ Fy). This attempts to reduce identity to a second-order equivalence, though Quine later argued this definition is inadmissible in first-order logic without identity.

## The Law as a Diagnostic Tool

The philosophical power of (II) lies in its use as a *test for non-identity*. If I can find a property that one thing has but another lacks, then the two things are not identical. The argument form is:

- P1. a = b → ∀F(Fa ↔ Fb). [Leibniz's Law]
- P2. Fa ∧ ¬Fb. [a has a property b lacks]
- C. a ≠ b. [by modus tollens on P1 with P2]

We can see this test at work across the major debates. In the mind-body debate, mental states like pain have the property "has a qualitative feel (qualia)" that brain states, as described by neuroscience, appear to lack under physical descriptions. By Leibniz's Law, if this property-difference is genuine, pain ≠ C-fiber firing. In mereology, the statue would cease to exist if squashed while the clay would not; by Leibniz's Law, the statue ≠ the clay. In philosophy of language, Hesperus and Phosphorus seemed to have different modal properties — but on careful analysis, the differences are *de dicto*, not *de re*, and so do not establish genuine non-identity. The test is indispensable, and understanding where it applies cleanly is itself a philosophical achievement.

## The Role of PII

PII figures in several important debates, even if its status as a necessary truth is disputed. Black's two-sphere universe appears to provide a model in which two distinct individuals share all properties, which would refute PII if the scenario is genuinely possible. Quantum mechanics complicates matters further: bosons (e.g., photons) can occupy the same quantum state, and some philosophers have argued that quantum statistics show PII fails at the subatomic level. Bundle theorists, who identify objects with their qualitative bundles, are committed to PII by definition: if two "objects" have the same bundle, they are one object. The failure of PII, if genuine, is a decisive refutation of bundle theory.

The key discipline required to apply Leibniz's Law legitimately is distinguishing genuine property-differences — those that hold independently of how an object is described or conceptualized — from apparent ones that arise from intensional contexts, time-indexing, or description-sensitivity. A difference that disappears when we move from *de dicto* to *de re* readings is not a genuine difference in the objects. A difference that survives that move is genuine evidence of non-identity. Making this distinction is one of the central skills of analytic metaphysics.
