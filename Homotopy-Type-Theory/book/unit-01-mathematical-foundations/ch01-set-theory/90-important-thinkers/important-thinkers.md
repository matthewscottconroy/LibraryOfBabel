# Important Thinkers in Set Theory

## Georg Cantor (1845–1918)

Cantor invented set theory. He had one predecessor in spirit — Bolzano, who thought about infinite collections without the technical tools — and none in practice. Beginning in 1874, Cantor proved that the real numbers are uncountable, that there are strictly more reals than naturals, and that the power set of any set has strictly greater cardinality than the set itself. He introduced ordinal numbers to measure transfinite processes, cardinal numbers to measure transfinite size, and posed the Continuum Hypothesis — conjecturing that there is no cardinality between ℵ₀ and |ℝ|.

Cantor's work was bitterly contested. Kronecker called it a "mathematical disease," denied Cantor professorships in Berlin, and privately disparaged him throughout his career. Cantor suffered severe depression and spent extended periods in sanatoriums; the mental illness and the mathematical attacks are intertwined in his biography, though causation is debated. He died in a sanatorium in 1918, not living to see his work vindicated.

The vindication was complete. Hilbert's declaration — "No one shall expel us from the paradise that Cantor has created" — became the consensus of the mathematical community. Cantor's diagonal argument, his theory of ordinals and cardinals, and the questions he raised about infinite set theory define the research program of set theory to this day.

## Gottlob Frege (1848–1925)

Frege appears again in set theory because his *Grundgesetze der Arithmetik* was the most ambitious attempt to derive all of arithmetic from purely logical principles — and its destruction by Russell's paradox is the founding crisis of set theory as an axiomatic discipline.

Frege's Basic Law V — that every property defines a set — is the naive comprehension principle. Russell showed it is inconsistent. Frege's response was honest and devastating: he acknowledged the paradox fully, attempted a patch (restricting Basic Law V), and acknowledged that the patch might not work. He was right — it did not. He spent his last years exploring neo-logicist alternatives that he never completed to his satisfaction.

What survives is the conceptual framework. Frege identified the *extension* of a concept (the set of things satisfying a predicate) as the right set-theoretic notion, understood that mathematical objects should be defined by their functional role rather than their intrinsic nature, and formulated the *Caesar problem*: why is Julius Caesar not a number? The Caesar problem is Benacerraf's identity problem in an earlier form, and Frege understood it correctly: if numbers are objects, we need a criterion for which objects they are.

## Bertrand Russell (1872–1970)

Russell's paradox ended Frege's logicism and forced the development of axiomatic set theory. Russell's own response — the theory of types — was the first rigorous attempt to prevent self-referential paradoxes by a stratified type system.

The *Principia Mathematica* (1910–1913, with Whitehead) is the most technically demanding foundational work ever written. It builds arithmetic, real analysis, and significant portions of abstract mathematics from a ramified type-theoretic foundation with explicit axiom systems. The notation is idiosyncratic and the ramification unnecessarily complex (as Russell himself later acknowledged), but the achievement is enormous: a complete formal derivation of classical mathematics.

Russell's type theory is the direct ancestor of all type theories, including Martin-Löf Type Theory and HoTT. The idea that objects live at levels, that self-reference is prevented by type-checking, and that existence claims must satisfy type constraints — all of these originate with Russell's response to his own paradox. The modern type theories are cleaner and better understood, but they inherit Russell's central insight.

## Ernst Zermelo (1871–1953)

Zermelo saved Cantor's theory by axiomatizing it. His 1908 paper introduced the first axiomatic set theory: seven axioms (Extensionality, Empty Set, Pairing, Union, Power Set, Separation, Infinity) designed to formalize Cantor's work while blocking the paradoxes. Separation replaced naive comprehension with restricted comprehension, preventing Russell's paradox at the source.

Zermelo also proved (1904) that every set can be well-ordered, using the Axiom of Choice explicitly for the first time. His proof was controversial — critics objected to the non-constructive character of the choice function — and his response was to axiomatize Choice as one of his seven axioms. This explicit axiomatization of Choice, rather than treating it as obvious, was a significant contribution to the clarity of foundational discussions.

Zermelo preferred a second-order formulation of set theory (where quantifiers range over arbitrary subsets, not just elements of the domain) and was critical of Skolem's move to first-order ZFC. The tension between these formulations — second-order (categorical but proof-theoretically intractable) versus first-order (non-categorical but proof-theoretically rich) — is a permanent theme in set theory.

## Abraham Fraenkel (1891–1965)

Fraenkel identified the gap in Zermelo's axioms and provided the fix: the Axiom of Replacement. Zermelo's system cannot prove that the set {ω, 𝒫(ω), 𝒫(𝒫(ω)), ...} exists, because Separation only carves subsets of existing sets and cannot build new sets by applying a function to an existing set's elements.

Fraenkel's Replacement Axiom (independently discovered by Skolem) says: the image of any set under any definable function is a set. This dramatically increases the strength of the system and is necessary for the development of transfinite cardinal arithmetic and the construction of all ordinals as sets.

Fraenkel also gave the first informal argument for the independence of the Axiom of Choice from ZF, constructing a "permutation model" where Choice fails. This technique — constructing a model with specific properties by adding automorphisms — was the conceptual forerunner of Cohen's forcing.

## Paul Cohen (1934–2007)

Cohen invented forcing and proved that the Continuum Hypothesis is independent of ZFC — the most celebrated result in set theory since Gödel's work.

Gödel had shown (1938) that CH is consistent with ZFC: in the constructible universe L, CH holds. Cohen showed (1963) that ¬CH is also consistent: one can build a model of ZFC in which there are ℵ₂ real numbers, so 2^ℵ₀ = ℵ₂ ≠ ℵ₁. Together, these results established the complete independence of CH from ZFC.

Forcing is a technique for building new models of set theory by adding "generic" elements — elements that are in some sense "as undetermined as possible" consistent with the axioms. The method has been extended and refined by hundreds of set theorists and is now the dominant tool for proving independence results.

Cohen received the Fields Medal in 1966 — the only Fields Medal ever awarded for work in mathematical logic. He was 29 when he proved the independence of CH and essentially never returned to set theory, spending the rest of his career in harmonic analysis.

## Vladimir Voevodsky (1966–2017)

Voevodsky appears in this section because he is the founder of Homotopy Type Theory, which emerged directly from his work on foundations of mathematics and his frustration with the limitations of ZFC.

Voevodsky won the Fields Medal in 2002 for his proof of the Milnor conjecture in algebraic K-theory. During this work, he discovered that a proof he had published years earlier was incorrect — a fact not caught by the mathematical community for years. This experience convinced him that mathematically rigorous verification required computer formalization, and he turned to foundations.

His "Univalent Foundations" program, developed at the Institute for Advanced Study beginning around 2009, proposes to replace ZFC with a type-theoretic foundation where:
- Types correspond to spaces (in the homotopy-theoretic sense)
- Equality corresponds to paths
- Equivalence implies equality (the Univalence Axiom)
- Proof assistants can verify every step

The resulting system — Homotopy Type Theory — is the subject of this curriculum. Voevodsky's insight that the identity problem in ZFC (isomorphic structures should be equal) could be resolved by the topological notion of equivalence-as-path is the conceptual breakthrough. He died in 2017 at age 51, leaving the development of HoTT as one of the most active areas in mathematical foundations.
