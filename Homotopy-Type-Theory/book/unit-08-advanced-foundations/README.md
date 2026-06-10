# Unit 08: Advanced Foundations
## From Computable Univalence to Synthetic Higher Categories and Cohesive Geometry

There is a moment in every deep mathematical story where the foundations stop being a convenience and start being the point. This unit is about that moment, repeated three times in rapid succession, each time revealing something you would not have predicted from the story so far.

The first discovery: the Univalence Axiom, beautiful as it is, leaves computation broken. Paths introduced by `ua` cannot reduce; they just sit there, opaque, and the machine that was supposed to be doing mathematics simply stops. Cohen, Coquand, Huber, and Mörtberg fixed this in 2016 by doing something audacious: they dissolved the axiom into the fabric of the type theory itself, replacing it with an interval, a De Morgan algebra, a composition operation, and a type constructor called Glue. The result is that univalence is no longer postulated — it is *computed*. Every closed term of type ℕ normalizes to a numeral. The program runs.

The second discovery: ordinary HoTT, for all its geometric sophistication, gives every path an automatic reverse. This is right for homotopy theory, where paths are undirected. But mathematics is full of directed structure — functors that go one way, morphisms that have no inverse, natural transformations with a source and a target. Riehl and Shulman asked what would happen if you added a *second* interval to HoTT, one without complement, one that models a directed edge. The answer is simplicial type theory: a setting where types can be ∞-categories, where the Yoneda lemma is provable without ever mentioning a simplicial set, where the distinction between a groupoid and a category is written into the very shape of the interval you use.

The third discovery: differential geometry — the mathematics of curvature, connection, and smooth structure — need not be done in coordinates. Urs Schreiber and Michael Shulman found that all of it, including gauge theory and string theory, lives naturally in a type theory extended with three modalities: shape ∫, flat ♭, and sharp ♯. These are not abbreviations for existing constructions. They are genuinely new structure, capturing the difference between a smooth space and its underlying homotopy type, between a space and its discrete shadow. The result, cohesive HoTT, is mathematics at the frontier: a single foundation that contains synthetic homotopy theory, synthetic differential geometry, and the rudiments of mathematical physics.

---

### What This Unit Demands

This is not introductory material. The reader who arrives here should be comfortable with:

- The identity type and path induction (Chapter 2)
- Transport, ap, and the path groupoid (Chapter 5)
- Equivalences and the statement of Univalence (Chapter 13)
- Basic higher inductive types (Chapter 17)
- The intuition for ∞-groupoids (Chapter 20)
- Elementary category theory: functors, natural transformations, adjunctions

The treatment here will not slow down for review. If a concept from earlier in the book appears, it appears at full speed. These are frontier topics, and they deserve to be engaged at the frontier.

---

### The Three Chapters

**Chapter 23 — Cubical Type Theory.** The CCHM interval, face formulas, partial elements, composition (hcomp), transport (transp), the Glue type, and the proof that univalence is a theorem. Variations: Cartesian cubical type theory (cooltt), XTT, 2-level type theory. Implementation: Cubical Agda.

**Chapter 24 — Simplicial Type Theory.** The directed interval 𝟚, extension types, the hom type, Segal types as synthetic ∞-categories, Rezk completeness as directed univalence, functors as functions, natural transformations as directed paths, and the synthetic Yoneda lemma. Implementation: Rzk.

**Chapter 25 — Modal HoTT and Cohesive Geometry.** Modalities as idempotent monads, lex modalities, the cohesion adjoint triple ∫ ⊣ ♭ ⊣ ♯, real cohesion, synthetic differential geometry via de Rham forms, principal bundles with connection, and Schreiber's formalization of physics. Implementation: Cubical Agda with `--cohesion`.

---

### A Note on Tools

Each chapter has a corresponding proof assistant implementation:

- **Cubical Agda** (`agda --cubical`): the primary implementation of CCHM cubical type theory, with a mature library (cubical-agda) covering homotopy theory, algebra, and cohomology.
- **Rzk** (`rzk-lang`): a proof assistant implementing simplicial type theory, designed specifically for synthetic ∞-category theory. Young, fast-moving, and directly tied to the Riehl-Shulman research program.
- **Cubical Agda with `--cohesion`**: an experimental extension implementing the flat modality ♭. The cohesion axioms are not yet fully integrated; this is active research territory.

The exercises in this unit include proof development tasks in both Cubical Agda and Rzk. Engaging with the actual code — running it, modifying it, watching it fail and understanding why — is not optional enrichment. It is the only way to understand what "computable" means in this context.

---

### The Bigger Picture

These three chapters are not three separate topics. They are three angles on a single question: *what is the right computational foundation for mathematics that takes homotopy and geometry seriously?*

Cubical type theory answers: make paths functions, make univalence a theorem, make everything compute.

Simplicial type theory answers: add direction, make ∞-categories native, make the Yoneda lemma a tautology.

Cohesive HoTT answers: add modalities that capture the difference between discrete and continuous, and watch differential geometry emerge from the adjoint triple.

The day these three programs are unified — a single type theory that is cubical and simplicial and cohesive — will be a significant day in the history of foundations. We are not there yet. But all three research programs are active, all three have working implementations, and the researchers who created them are still refining them. You are reading this at the right time.
