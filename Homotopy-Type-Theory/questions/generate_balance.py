#!/usr/bin/env python3
"""
Generate balancing questions for chapters with skewed difficulty or type distributions.

Chapters needing MORE BEGINNER questions: 11, 12, 23, 24, 25, 26
Chapters needing MORE ADVANCED questions: 13, 16, 17, 21
Chapters needing MORE TF/BLANK questions: 12, 20, 21, 24, 25, 26

Run from the project root: python3 questions/generate_balance.py
"""
import json, uuid, os
from pathlib import Path

QUESTIONS_DIR = Path(__file__).parent

CHAPTER_DIRS = {
    11: "ch11-categorical-logic",
    12: "ch12-higher-categories",
    13: "ch13-topology",
    16: "ch16-identity-types",
    17: "ch17-h-levels",
    21: "ch21-lean4",
    23: "ch23-cubical-type-theory",
    24: "ch24-simplicial-type-theory",
    25: "ch25-modal-hott",
    26: "ch26-research-frontiers",
}

CHAPTER_PHASES = {
    11: 3, 12: 3, 13: 4, 16: 5, 17: 5,
    21: 6, 23: 7, 24: 7, 25: 7, 26: 8,
}

def q(chapter, kind, text, choices, answer, explanation, tags, difficulty):
    return {
        "question_id": str(uuid.uuid4()),
        "chapter": chapter,
        "phase": CHAPTER_PHASES[chapter],
        "kind": kind,
        "text": text,
        "choices": choices,
        "answer": answer,
        "explanation": explanation,
        "tags": tags,
        "difficulty": difficulty,
        "generated": False,
    }

def mc(chapter, text, choices, answer, explanation, tags, difficulty):
    return q(chapter, "mc", text, choices, answer, explanation, tags, difficulty)

def tf(chapter, text, answer_bool, explanation, tags, difficulty):
    return q(chapter, "tf", text, ["True", "False"],
             0 if answer_bool else 1, explanation, tags, difficulty)

def blank(chapter, text, synonyms, explanation, tags, difficulty):
    return q(chapter, "blank", text, synonyms, synonyms[0].lower(),
             explanation, tags, difficulty)


# ── Chapter 11: Categorical Logic — 10 BEGINNER questions ─────────────────────

CH11_BEGINNER = [
    mc(11,
       "A category is called Cartesian closed (CCC) if it has finite products and:",
       ["all equalizers",
        "exponential objects A^B for any two objects A, B",
        "a zero object and biproducts",
        "all pushouts"],
       1,
       "A CCC has finite products and internal hom objects A^B (exponentials). "
       "This structure models the simply typed lambda calculus via the Curry-Howard-Lambek correspondence.",
       ["ccc", "categorical-logic"], "beginner"),

    mc(11,
       "In categorical semantics, the conjunction P ∧ Q of two propositions is modeled by:",
       ["the coproduct P + Q",
        "the product P × Q",
        "the exponential P^Q",
        "the equalizer of P and Q"],
       1,
       "Conjunction = product, disjunction = coproduct, implication = exponential. "
       "This is the core of the logic–category correspondence.",
       ["categorical-logic", "ccc"], "beginner"),

    tf(11,
       "The internal logic of every elementary topos satisfies the law of excluded middle.",
       False,
       "Topos logic is intuitionistic by default. LEM holds only when the subobject classifier Ω is Boolean "
       "(i.e., Ω ≅ 1 + 1), which is not required in general.",
       ["topos", "categorical-logic"], "beginner"),

    mc(11,
       "In a slice category C/A, the objects are:",
       ["subobjects of A",
        "morphisms f: X → A for any object X in C",
        "functors from C to A",
        "morphisms from A to other objects"],
       1,
       "C/A (C over A) has objects = morphisms into A, and morphisms = commutative triangles over A. "
       "Slice categories model dependent type contexts categorically.",
       ["lccc", "categorical-logic"], "beginner"),

    mc(11,
       "The Yoneda lemma says that natural transformations C(-, A) ⟹ F are in bijection with:",
       ["morphisms from A to any object",
        "elements of F(A)",
        "subobjects of A",
        "functors from C to Set"],
       1,
       "Nat(C(-,A), F) ≅ F(A), naturally in A and F. This is one of the most fundamental results "
       "in category theory and is used everywhere in categorical logic.",
       ["yoneda", "category-theory"], "beginner"),

    tf(11,
       "Every Cartesian closed category (CCC) can serve as a model of the simply typed lambda calculus.",
       True,
       "The Curry-Howard-Lambek correspondence: types = objects, terms = morphisms, "
       "→ = exponential, × = product. Every CCC gives a sound and complete model of STLC.",
       ["ccc", "stlc"], "beginner"),

    mc(11,
       "A subobject classifier Ω in an elementary topos is an object such that:",
       ["every subobject of X corresponds uniquely to a morphism X → Ω",
        "Ω is the terminal object 1",
        "Ω classifies all endofunctors",
        "Ω is the initial object 0"],
       0,
       "The characteristic morphism χ_S: X → Ω of a subobject S ↪ X is the categorical analog "
       "of the indicator function. This makes Ω the 'object of truth values'.",
       ["topos", "subobject-classifier"], "beginner"),

    mc(11,
       "LCCC stands for:",
       ["left-closed Cartesian category",
        "locally Cartesian closed category",
        "limit-coheight Cartesian complex",
        "linearly closed category construction"],
       1,
       "A locally Cartesian closed category (LCCC) has all slice categories C/A being CCCs. "
       "LCCCs model dependent type theory (Seely-Hofmann-Dybjer).",
       ["lccc", "dependent-types"], "beginner"),

    mc(11,
       "In categorical logic, universal quantification ∀x. P(x) is modeled by:",
       ["the left adjoint to the pullback functor",
        "the right adjoint to the pullback functor (dependent product)",
        "the equalizer of P",
        "the subobject classifier Ω"],
       1,
       "∀ = right adjoint (Π) to substitution/pullback; ∃ = left adjoint (Σ) to substitution/pullback. "
       "This is the Beck-Chevalley / hyperdoctrine structure.",
       ["lccc", "categorical-logic", "quantifiers"], "beginner"),

    blank(11,
          "The Curry-Howard-Lambek correspondence identifies Cartesian closed categories with "
          "intuitionistic propositional logic and the ___.",
          ["simply typed lambda calculus", "STLC"],
          "The three-way correspondence: CCCs ↔ STLC ↔ intuitionistic prop. logic. "
          "Objects = types/propositions, morphisms = terms/proofs.",
          ["ccc", "stlc", "categorical-logic"], "beginner"),
]


# ── Chapter 12: Higher Categories — 10 BEGINNER questions ──────────────────────

CH12_BEGINNER = [
    mc(12,
       "A groupoid is a category in which:",
       ["all objects are isomorphic",
        "every morphism is an isomorphism",
        "there is exactly one morphism between any two objects",
        "all hom-sets are groups"],
       1,
       "A groupoid is a category where every morphism f: A → B has an inverse f⁻¹: B → A. "
       "The fundamental groupoid Π₁(X) of a space is the prototypical example.",
       ["groupoids", "higher-categories"], "beginner"),

    tf(12,
       "The fundamental groupoid Π₁(X) of a topological space has objects = points of X "
       "and morphisms = homotopy classes of paths.",
       True,
       "Π₁(X) generalizes the fundamental group: instead of loops based at one point, "
       "we allow paths between any two points, with homotopy classes as morphisms.",
       ["groupoids", "homotopy-hypothesis"], "beginner"),

    mc(12,
       "Grothendieck's homotopy hypothesis conjectures that ∞-groupoids correspond to:",
       ["chain complexes",
        "simplicial sets",
        "homotopy types (topological spaces up to weak homotopy equivalence)",
        "strict ω-categories"],
       2,
       "The homotopy hypothesis: ∞-Gpd ≃ Top (as (∞,1)-categories). This is the bridge "
       "between abstract higher category theory and classical algebraic topology.",
       ["homotopy-hypothesis", "infinity-groupoid"], "beginner"),

    mc(12,
       "A quasi-category (in the sense of Joyal) is a simplicial set in which every "
       "___ has a filler.",
       ["outer horn Λⁿ₀ or Λⁿₙ",
        "inner horn Λⁿₖ (0 < k < n)",
        "boundary ∂Δ[n]",
        "standard simplex Δ[n]"],
       1,
       "Quasi-categories (∞-categories) require fillers for inner horns only. "
       "Kan complexes (∞-groupoids) additionally require fillers for outer horns.",
       ["quasi-categories", "kan-complex"], "beginner"),

    mc(12,
       "In a bicategory (weak 2-category), the associativity and unit laws for composition hold:",
       ["strictly (as equalities)",
        "up to coherent isomorphisms (2-morphisms)",
        "only when the objects form a set",
        "not at all — they are replaced by axioms"],
       1,
       "Bicategories weaken strict 2-categories: associativity (a∘b)∘c ≅ a∘(b∘c) and "
       "units id∘a ≅ a ≅ a∘id hold up to coherent invertible 2-morphisms.",
       ["bicategory", "higher-categories"], "beginner"),

    mc(12,
       "In HoTT, every type A carries the structure of an ∞-groupoid where the morphisms are:",
       ["the elements of A",
        "the paths a = b (identity types)",
        "the functions A → A",
        "the propositions about A"],
       1,
       "Types are ∞-groupoids: paths (1-morphisms), paths-of-paths (2-morphisms), etc. "
       "This is the core of the HoTT interpretation of type theory.",
       ["hott", "infinity-groupoid"], "beginner"),

    tf(12,
       "Every Kan complex is a quasi-category.",
       True,
       "A Kan complex fills all horns (outer and inner), so it certainly fills inner horns. "
       "Kan complexes are ∞-groupoids; quasi-categories are ∞-categories (not necessarily groupoids).",
       ["kan-complex", "quasi-categories"], "beginner"),

    mc(12,
       "Morphisms between morphisms in a 2-category are called:",
       ["natural transformations only",
        "2-morphisms (or 2-cells)",
        "adjunctions",
        "bimodules"],
       1,
       "A 2-category has objects, 1-morphisms (between objects), and 2-morphisms (between 1-morphisms). "
       "Examples: Cat (categories, functors, natural transformations), 2-Grp.",
       ["2-category", "higher-categories"], "beginner"),

    mc(12,
       "The homotopy category Ho(C) of an ∞-category C is obtained by:",
       ["restricting to the 0-simplices only",
        "inverting all morphisms",
        "taking objects and homotopy classes of 1-morphisms",
        "truncating all higher path data above dimension 1"],
       2,
       "Ho(C) has the same objects as C, but morphisms are π₀(Map_C(x,y)) "
       "— homotopy classes of morphisms. This is the '1-categorical shadow' of C.",
       ["homotopy-category", "infinity-categories"], "beginner"),

    blank(12,
          "The nerve N(C) of a 1-category C is a simplicial set whose n-simplices are "
          "composable chains of ___ morphisms.",
          ["n", "n composable"],
          "N(C)_n = {f₁, …, fₙ composable morphisms in C}. The nerve embeds Cat into sSet, "
          "and a simplicial set is the nerve of a category iff it satisfies the Segal condition.",
          ["nerve", "simplicial-set", "segal-condition"], "beginner"),
]


# ── Chapter 13: Topology — 10 ADVANCED questions ──────────────────────────────

CH13_ADVANCED = [
    mc(13,
       "Tychonoff's theorem (an arbitrary product of compact spaces is compact) is equivalent to:",
       ["the axiom of choice",
        "Zorn's lemma only in the finite case",
        "the continuum hypothesis",
        "König's lemma"],
       0,
       "Kelley (1950) showed Tychonoff ↔ AC. The proof uses AC to construct ultrafilters; "
       "the converse uses Tychonoff to well-order any set.",
       ["compactness", "axiom-of-choice"], "advanced"),

    mc(13,
       "The Urysohn metrization theorem states that a topological space is metrizable if it is:",
       ["compact and Hausdorff",
        "second-countable, regular, and T₁",
        "locally compact and σ-compact",
        "paracompact and Hausdorff"],
       1,
       "Urysohn: second-countable + regular + T₁ ⟹ metrizable. Second-countable + regular ⟹ normal "
       "(Urysohn's lemma applies); Urysohn's embedding then gives a metric.",
       ["metrizability", "urysohn"], "advanced"),

    tf(13,
       "A connected topological space is necessarily path-connected.",
       False,
       "The topologist's sine curve {(x, sin(1/x)) : x > 0} ∪ {(0,y) : -1≤y≤1} is connected "
       "but not path-connected — the limit line cannot be reached by a path from the oscillating part.",
       ["connectedness"], "advanced"),

    mc(13,
       "The van Kampen theorem computes π₁(X ∪_C Y) when C = X ∩ Y is path-connected as:",
       ["π₁(X) × π₁(Y)",
        "π₁(X) ∗ π₁(Y) (free product)",
        "π₁(X) ∗_{π₁(C)} π₁(Y) (amalgamated free product)",
        "π₁(X) / π₁(Y)"],
       2,
       "Van Kampen: π₁(X ∪_C Y) ≅ π₁(X) *_{π₁(C)} π₁(Y). When C is simply connected, "
       "this reduces to the free product π₁(X) * π₁(Y). Critical for computing π₁ of CW complexes.",
       ["van-kampen", "fundamental-group"], "advanced"),

    mc(13,
       "The fundamental group of the Klein bottle K is:",
       ["ℤ × ℤ/2ℤ",
        "⟨a, b | abab⁻¹ = 1⟩",
        "⟨a, b | aba⁻¹b⁻¹ = 1⟩",
        "ℤ * ℤ/2ℤ"],
       1,
       "π₁(K) = ⟨a,b | abab⁻¹⟩ — one generator for each 1-cell, one relation for the 2-cell. "
       "This is non-abelian and non-isomorphic to π₁(T²) = ℤ².",
       ["fundamental-group", "cw-complex"], "advanced"),

    mc(13,
       "A CW complex is called n-dimensional if:",
       ["it has exactly n cells",
        "it has cells of dimension at most n and at least one n-cell",
        "its nth homotopy group is non-trivial",
        "it is homeomorphic to Sⁿ"],
       1,
       "Dimension of a CW complex = the highest dimension of any of its cells. "
       "The n-skeleton Xⁿ contains all cells of dimension ≤ n.",
       ["cw-complex"], "advanced"),

    tf(13,
       "Every covering space of a path-connected, locally path-connected, semi-locally simply connected "
       "space X corresponds (up to isomorphism) to a subgroup of π₁(X).",
       True,
       "The Galois correspondence for covering spaces: {covering spaces of X} ↔ {subgroups of π₁(X,x₀)}. "
       "The universal cover corresponds to the trivial subgroup {1}.",
       ["covering-spaces", "fundamental-group"], "advanced"),

    mc(13,
       "The long exact sequence of homotopy groups for a fibration p: E → B with fiber F is:",
       ["⋯ → πₙ(F) → πₙ(E) → πₙ(B) → πₙ₋₁(F) → ⋯",
        "⋯ → πₙ(B) → πₙ(E) → πₙ(F) → ⋯",
        "πₙ(E) ≅ πₙ(F) × πₙ(B) for all n",
        "⋯ → πₙ(F) → πₙ(B) → πₙ(E) → ⋯"],
       0,
       "LES: ⋯ → πₙ(F) →i* πₙ(E) →p* πₙ(B) →∂ πₙ₋₁(F) → ⋯ → π₀(E) → π₀(B). "
       "Connecting map ∂ comes from lifting a loop in B to a path in E.",
       ["fibration", "homotopy-groups"], "advanced"),

    mc(13,
       "Which of the following is NOT a topological invariant (can change under homeomorphism)?",
       ["the fundamental group π₁",
        "the number of path components",
        "the diameter in a fixed metric",
        "compactness"],
       2,
       "The diameter depends on the choice of metric, not just the topology. "
       "A homeomorphism between metric spaces need not be an isometry.",
       ["homeomorphism", "invariants"], "advanced"),

    blank(13,
          "A continuous map f: X → Y such that every open cover of Y pulls back to an open cover "
          "of X with a finite subcover is called a ___ map.",
          ["proper", "proper map"],
          "Proper maps: preimages of compact sets are compact. Equivalent to the pullback property above. "
          "Proper maps are closed, and proper bijections between Hausdorff spaces are homeomorphisms.",
          ["proper-map", "compactness"], "advanced"),
]


# ── Chapter 16: Identity Types — 10 ADVANCED questions ────────────────────────

CH16_ADVANCED = [
    mc(16,
       "The encode-decode method for computing πₙ(A) proceeds by constructing a family "
       "code: A → Type such that:",
       ["code(x) ≃ (a = x) for all x, via a section of decode",
        "code(x) is always contractible",
        "code(a) = ℕ and code(x) = 𝟘 for x ≠ a",
        "decode is a surjection onto the loop space"],
       0,
       "Encode: (a=x) → code(x); decode: code(x) → (a=x). Show encode ∘ decode = id and "
       "decode ∘ encode = id (by path induction). Used to prove π₁(S¹) = ℤ.",
       ["encode-decode", "identity-types"], "advanced"),

    mc(16,
       "The fundamental theorem of identity types states: given R: A → Prop with r: R(a), "
       "if f: Π(x:A). R(x) → (a=x) and encode: Π(x:A). (a=x) → R(x) are mutual inverses, then:",
       ["A is a set",
        "(Σ x:A, R(x)) is contractible",
        "R(x) is a proposition for all x",
        "A is contractible"],
       1,
       "FTIT: if R with r₀: R(a) satisfies R(x) ≃ (a=x), then (Σx:A, R(x)) is contractible "
       "(center = (a, r₀)). This characterizes identity types via contractible total spaces.",
       ["identity-types", "fundamental-theorem"], "advanced"),

    tf(16,
       "In HoTT, the J eliminator (path induction) can be derived from the fact that "
       "the type Σ(x:A), a=x is contractible.",
       True,
       "The contractibility of Σ(x:A), a=x with center (a, refl) means that "
       "any two elements (b, p) are equal to (a, refl), giving exactly the J eliminator.",
       ["j-eliminator", "identity-types", "contractible"], "advanced"),

    mc(16,
       "The action on paths ap_f(p) for f: A → B and p: a₁=a₂ satisfies:",
       ["ap_f(p ∙ q) = ap_f(p) ∙ ap_f(q)  (functoriality)",
        "ap_f(p) = p  (identity)",
        "ap_f(p⁻¹) = ap_f(p)  (ignores inversion)",
        "ap_f(refl) = p"],
       0,
       "ap_f is functorial: ap(id) = id, ap(g∘f) = ap(g)∘ap(f), ap_f(p∙q) = ap_f(p)∙ap_f(q), "
       "ap_f(p⁻¹) = (ap_f(p))⁻¹. This makes every function a functor between groupoids.",
       ["ap", "functoriality", "identity-types"], "advanced"),

    mc(16,
       "Whiskering a 2-path α: p=q (where p,q: a=b) on the right by r: b=c gives:",
       ["a 2-path α ▷ r : (p ∙ r) = (q ∙ r)",
        "a path in A",
        "a 2-path r ▷ α : (r ∙ p) = (r ∙ q)",
        "the horizontal composite α ∙ₕ refl_r"],
       0,
       "Right whiskering: α ▷ r : p ∙ r = q ∙ r. Left whiskering: l ▷ α : l ∙ p = l ∙ q. "
       "These are used to define horizontal composition of 2-paths.",
       ["higher-paths", "whiskering", "identity-types"], "advanced"),

    mc(16,
       "The Eckmann-Hilton argument shows that for 2-loops α, β: refl = refl in ΩΩA:",
       ["α ∙ β = β ∙ α  (π₂(A) is abelian)",
        "α = β  (all 2-loops are equal)",
        "α ∙ β = refl  (all 2-loops are trivial)",
        "α and β cannot be composed"],
       0,
       "Eckmann-Hilton: the two compositions (vertical and horizontal) on ΩΩA agree and are both "
       "commutative. This proves π₂(A) is abelian — the first instance of commutativity from higher structure.",
       ["eckmann-hilton", "higher-paths", "homotopy-groups"], "advanced"),

    tf(16,
       "Transport along a path p: a=b in a family B: A → Type is always an equivalence "
       "B(a) ≃ B(b).",
       True,
       "transport(p, -): B(a) → B(b) is an equivalence with inverse transport(p⁻¹, -): B(b) → B(a). "
       "This is because p⁻¹ is the inverse path and transport is functorial.",
       ["transport", "identity-types", "equivalence"], "advanced"),

    mc(16,
       "For dependent functions f, g: Π(x:A). B(x), a homotopy h: f ~ g is:",
       ["a path f = g in Π(x:A). B(x)",
        "a term of type Π(x:A). f(x) = g(x)",
        "an equivalence between the total spaces",
        "a section of the fibration B → A"],
       1,
       "A homotopy between dependent functions is Π(x:A). f(x) = g(x). "
       "Function extensionality (funext) says this implies f = g (they are equal as functions).",
       ["homotopy", "function-extensionality"], "advanced"),

    mc(16,
       "The type of based paths from a:A (the 'path space' based at a) is Σ(x:A), a=x. "
       "This type is:",
       ["a set (h-level 0)",
        "always empty",
        "contractible (h-level -2)",
        "a mere proposition"],
       2,
       "Σ(x:A), a=x is contractible with center (a, refl_a). "
       "This contractibility is equivalent to the J eliminator.",
       ["contractible", "identity-types", "path-space"], "advanced"),

    blank(16,
          "The operation that takes p: a=b in A and f: A → B to a proof f(a) = f(b) in B "
          "is called ___ (written ap_f(p)).",
          ["action on paths", "ap", "functorial action"],
          "ap_f(p) or f[p]: A=(a=b) → B=(f(a)=f(b)). Makes every function into a functor "
          "between the associated groupoids. Fundamental operation in HoTT.",
          ["ap", "identity-types"], "advanced"),
]


# ── Chapter 17: H-Levels — 10 ADVANCED questions ──────────────────────────────

CH17_ADVANCED = [
    mc(17,
       "Hedberg's theorem states: if A has decidable equality (∀x,y:A. (x=y)+(x≠y)), then:",
       ["A is contractible",
        "A is a mere proposition",
        "A is a set (h-level 0, all identity types are mere propositions)",
        "A has decidable membership"],
       2,
       "Hedberg (1998): decidable equality → UIP → h-set. The proof constructs a constant "
       "function on loops using the decision procedure, then applies Streicher's theorem.",
       ["hedberg", "h-sets", "decidability"], "advanced"),

    mc(17,
       "For n ≥ -2, a type A is an n-type if and only if:",
       ["πₖ(A) is trivial for k > n",
        "all identity types a=b in A are (n-1)-types",
        "A has at most n+3 elements",
        "A is n-connected"],
       1,
       "The recursive definition: (-2)-types = contractible; (n+1)-types = all identity types a=b are n-types. "
       "This gives the h-level hierarchy: contr (-2), prop (-1), set (0), groupoid (1), …",
       ["h-levels", "n-types"], "advanced"),

    tf(17,
       "The propositional truncation ‖A‖₋₁ of A satisfies: ‖A‖₋₁ is inhabited if and only if A is inhabited.",
       True,
       "‖A‖₋₁ comes with |·|: A → ‖A‖₋₁, so if A is inhabited, so is ‖A‖₋₁. "
       "Conversely, ‖A‖₋₁ is a mere proposition, so to map out of it into a proposition P, "
       "you need a map A → P (not an element of A directly).",
       ["propositional-truncation", "h-levels"], "advanced"),

    mc(17,
       "The axiom of unique choice in HoTT states that if R: A → B → Prop is total (∀a.∃b. R(a,b)) "
       "and functional (∀a,b,b'. R(a,b) → R(a,b') → b=b'), then:",
       ["the axiom of choice gives a function A → B",
        "there exists a unique function f: A → B with ∀a. R(a,f(a))",
        "B must be a set",
        "this requires propositional resizing"],
       1,
       "Unique choice: ∀a. ∃!b. R(a,b) → ∃ f: A→B. ∀a. R(a,f(a)). Works constructively "
       "because the uniqueness condition (h-prop) lets us extract from ‖Σb.R(a,b)‖.",
       ["axiom-of-choice", "h-sets"], "advanced"),

    mc(17,
       "The Eckmann-Hilton argument shows that π₂(A, a) is abelian. The key step is showing that "
       "for 2-loops α, β: refl_a = refl_a, the two compositions ⋆ (vertical) and ⋆ₕ (horizontal) satisfy:",
       ["α ⋆ β = α  and  α ⋆ₕ β = β",
        "α ⋆ β = α ⋆ₕ β = β ⋆ α",
        "α ⋆ β = refl and α ⋆ₕ β = refl",
        "the two compositions are unrelated"],
       1,
       "Both compositions agree on 2-loops (by interchange) and both are unital, "
       "so α⋆β = α⋆ₕβ = β⋆α. The same argument shows all πₙ for n≥2 are abelian.",
       ["eckmann-hilton", "homotopy-groups", "h-levels"], "advanced"),

    mc(17,
       "A mere proposition P satisfies: Π(x,y:P). x=y. An equivalent characterization is that:",
       ["P is contractible",
        "P → P is contractible",
        "P ≃ 𝟙 whenever P is inhabited",
        "P is a retract of 𝟙"],
       2,
       "P is an h-prop iff: (a) all elements are equal, or equivalently (b) P ≃ 𝟙 when inhabited. "
       "h-props are the propositions of HoTT — they have 'at most one proof'.",
       ["h-propositions", "h-levels"], "advanced"),

    tf(17,
       "The type of all mere propositions (h-props) is itself a set (h-level 0).",
       True,
       "Two h-props P, Q: hProp are equal iff P ≃ Q (by univalence), and the type of equivalences "
       "between h-props is itself an h-prop (being an h-prop is a property). So hProp is a set.",
       ["h-propositions", "univalence", "h-sets"], "advanced"),

    mc(17,
       "The set-quotient A/R (for a relation R: A → A → Prop) in HoTT can be constructed as:",
       ["the image of the map [·]: A → A/R",
        "the pushout A ∪_{A×A} A (one copy per related pair)",
        "a higher inductive type with point constructor [·]: A → A/R "
        "and path constructor q: ∀x,y. R(x,y) → [x]=[y]",
        "the propositional truncation ‖A‖₋₁"],
       2,
       "A/R as a HIT: point constructor [·]: A → A/R and path constructor q: R(x,y)→[x]=[y], "
       "plus the 0-truncation condition. The universal property: maps A/R → B (B a set) "
       "correspond to R-respecting functions A → B.",
       ["quotient-types", "hit", "h-sets"], "advanced"),

    mc(17,
       "The n-th Postnikov section P_n(A) of a type A is characterized by:",
       ["πₖ(P_n(A)) ≅ πₖ(A) for k ≤ n and πₖ(P_n(A)) = 0 for k > n",
        "P_n(A) is the n-th loop space of A",
        "P_n(A) is the n-fold suspension of A",
        "P_n(A) has the same 0-cells as A but no higher cells"],
       0,
       "Postnikov sections truncate homotopy groups above n. P_n(A) is an n-type with "
       "a map A → P_n(A) inducing isomorphisms on πₖ for k≤n. These assemble into the Postnikov tower.",
       ["postnikov-tower", "n-types", "h-levels"], "advanced"),

    blank(17,
          "A type A is n-connected if πₖ(A) is trivial for k ≤ n. A (-1)-connected type is "
          "simply called ___.",
          ["inhabited", "merely inhabited", "non-empty"],
          "(-1)-connected = merely inhabited (= ‖A‖₋₁ is contractible = A is non-empty). "
          "0-connected = path-connected. 1-connected = simply connected.",
          ["connectivity", "h-levels"], "advanced"),
]


# ── Chapter 21: Lean 4 — 10 ADVANCED questions (includes TF and blank) ────────

CH21_ADVANCED = [
    mc(21,
       "In Lean 4's kernel, Prop is a sort that differs from Type in that:",
       ["Prop is a universe with no universe polymorphism",
        "proof terms in Prop are definitionally equal (proof irrelevance holds)",
        "Prop has no inductive types",
        "Prop is always the bottom of the universe hierarchy"],
       1,
       "Lean 4 has proof irrelevance for Prop: any two proofs p q: P are definitionally equal "
       "when P: Prop. This allows large eliminations from Type into Prop but not vice versa.",
       ["Lean4", "prop", "proof-irrelevance"], "advanced"),

    tf(21,
       "In Lean 4, `simp` is guaranteed to terminate on any goal.",
       False,
       "simp applies simp lemmas (rewrite rules) until no more apply. If lemmas form a cycle "
       "(e.g. a↔b and b↔a), simp will loop. Use `simp?` to inspect what is applied.",
       ["Lean4", "tactics"], "advanced"),

    mc(21,
       "The `apply?` tactic in Lean 4 / Mathlib:",
       ["applies all applicable lemmas and closes the goal automatically",
        "searches the library for lemmas whose conclusion unifies with the current goal",
        "rewrites the goal using all available simp lemmas",
        "converts the goal to its normal form"],
       1,
       "apply? (like exact?) is a 'search tactic' that suggests library lemmas. "
       "apply? finds f such that applying f transforms the goal; exact? finds a term that closes it.",
       ["Lean4", "tactics", "Mathlib"], "advanced"),

    tf(21,
       "In Lean 4, `#eval` and `#reduce` always produce the same output for a well-typed term.",
       False,
       "#eval compiles and runs the term using the native code generator (fast); "
       "#reduce kernel-reduces (slow, pure type theory). They agree on values but differ in speed "
       "and sometimes in what they can evaluate (e.g., axioms).",
       ["Lean4", "computation"], "advanced"),

    mc(21,
       "The `omega` tactic in Lean 4 / Mathlib decides:",
       ["all first-order arithmetic goals",
        "polynomial equalities over ℝ",
        "linear arithmetic goals over ℤ and ℕ",
        "propositional logic goals only"],
       2,
       "omega decides linear arithmetic (Presburger arithmetic): goals involving +, -, "
       "scalar multiplication, ≤, =, ∀, ∃ over ℤ and ℕ. Polynomial goals need ring/norm_num.",
       ["Lean4", "tactics", "omega"], "advanced"),

    mc(21,
       "In Lean 4, a `structure` definition generates:",
       ["only an inductive type with one constructor",
        "an inductive type with one constructor, plus projection functions and dot-notation support",
        "a typeclass with no default implementations",
        "a namespace with arbitrary definitions"],
       1,
       "structure S where field1: T1; field2: T2 generates: constructor S.mk, projections "
       "S.field1 and S.field2, and dot notation (s.field1). Also inherits from parent structures.",
       ["Lean4", "structures"], "advanced"),

    tf(21,
       "In Lean 4, `instance` declarations are searched globally by the elaborator, "
       "so adding an instance in one file affects typeclass resolution in all files that import it.",
       True,
       "Lean 4 uses open-world typeclass resolution: instances accumulate across imports. "
       "This is why Mathlib instances (e.g., Fintype for specific types) are available automatically.",
       ["Lean4", "typeclass"], "advanced"),

    mc(21,
       "To define a recursive function in Lean 4 that Lean cannot automatically prove terminating, you use:",
       ["a `partial` declaration (disables termination checking) or a well-founded recursion proof",
        "the `unsafe` keyword",
        "a `noncomputable` declaration",
        "a mutual block"],
       0,
       "Lean 4 requires termination proofs for recursive functions. `partial` opts out (no termination guarantee). "
       "Alternatively, provide a termination_by clause with a measure that decreases.",
       ["Lean4", "termination"], "advanced"),

    mc(21,
       "In Lean 4 / Mathlib4, the `CategoryTheory` library models morphisms as:",
       ["elements of a set Hom(X,Y) with a definitional associativity law",
        "bundled structures (category as a typeclass over an object type, with Hom as a field)",
        "functions between types only",
        "propositions about objects"],
       1,
       "Mathlib4 CategoryTheory: `Category C` is a typeclass on C: Type with `Hom: C → C → Type`, "
       "`comp`, `id`, and associativity/unit as proof fields. This allows universe-polymorphic categories.",
       ["Lean4", "Mathlib", "category-theory"], "advanced"),

    blank(21,
          "In Lean 4, the tactic that splits a goal of the form P ∧ Q into two subgoals P and Q is ___.",
          ["constructor", "And.intro", "exact ⟨_, _⟩"],
          "`constructor` applies the first constructor of an inductive type to the goal. "
          "For P ∧ Q (= And P Q), this splits into two goals: P and Q.",
          ["Lean4", "tactics"], "advanced"),
]


# ── Chapter 23: Cubical Type Theory — 10 BEGINNER questions ───────────────────

CH23_BEGINNER = [
    mc(23,
       "In CCHM cubical type theory, the interval I is equipped with:",
       ["only two points: 0 and 1",
        "a De Morgan algebra structure: ∧, ∨, and complement ~",
        "a group structure",
        "a linear order"],
       1,
       "The CCHM interval has i0, i1, meet (∧), join (∨), and complement (~i). "
       "This De Morgan structure is what gives cubical paths their computational power.",
       ["interval", "cchm", "cubical-type-theory"], "beginner"),

    tf(23,
       "In cubical type theory (CCHM), the univalence axiom is a theorem, not a postulated axiom.",
       True,
       "Uniquely among type theories, CCHM makes univalence computable: it follows from the "
       "Glue type and the interval. There is no need to postulate it separately.",
       ["univalence", "cubical-type-theory", "cchm"], "beginner"),

    mc(23,
       "A path in cubical type theory between a and b in type A is:",
       ["an element of the identity type a = b",
        "a function p: I → A with p(i0) = a and p(i1) = b",
        "a homotopy between a and b",
        "a proof of a ≡ b (definitional equality)"],
       1,
       "Cubical paths are functions from the interval: PathP (λ_. A) a b ≡ Π(i:I). A "
       "with the constraint that the function reduces to a at i0 and b at i1.",
       ["path-type", "cubical-type-theory"], "beginner"),

    mc(23,
       "What does 'canonicity' mean for a type theory?",
       ["every type has at most one element",
        "every closed term of a base type (e.g., ℕ) reduces to a constructor (a numeral)",
        "all proofs of the same proposition are definitionally equal",
        "the theory has a decision procedure for type checking"],
       1,
       "Canonicity: closed terms of ground types normalize to constructor forms. "
       "CCHM achieves canonicity, whereas Book HoTT with the univalence axiom does not (open problem).",
       ["canonicity", "cubical-type-theory"], "beginner"),

    mc(23,
       "The transp operation in cubical type theory is used for:",
       ["composing two paths",
        "transporting an element along a path in a type",
        "inverting a path",
        "computing the action of a function on paths"],
       1,
       "transp (λi. A i) φ a₀: A i1, where A: I → U is a type path, φ is a constraint "
       "under which a₀ is already in A i1, and a₀: A i0. This replaces transport.",
       ["transp", "transport", "cubical-type-theory"], "beginner"),

    mc(23,
       "The Glue type in CCHM is primarily used to construct:",
       ["the propositional truncation",
        "the proof that univalence holds (ua: A≃B → A=B)",
        "higher inductive types",
        "the dependent product Π"],
       1,
       "Glue φ A B f: a family over I that is A on the face φ and B elsewhere, "
       "with equivalence f: A ≃ B on φ. This is the key construction for proving ua.",
       ["glue-type", "univalence", "cubical-type-theory"], "beginner"),

    tf(23,
       "Cartesian cubical type theory (as in Agda's --cubical mode) uses the same interval "
       "as CCHM (with complement ~).",
       False,
       "Cartesian cubical TT (Angiuli et al.) uses a simpler interval without complement or connections. "
       "Cubical Agda uses the CCHM interval with De Morgan operations.",
       ["cartesian-cubical", "cchm", "cubical-type-theory"], "beginner"),

    mc(23,
       "In Cubical Agda, the command `PathP (λ i → A i) a b` defines:",
       ["a path from a to b in a fixed type A",
        "a heterogeneous path from a: A i0 to b: A i1 lying over a type path A: I → U",
        "a proof that A is contractible",
        "a transport of a to the type A i1"],
       1,
       "PathP generalizes Path: Path A a b ≡ PathP (λ_. A) a b. Heterogeneous paths are needed "
       "for the eliminator of HITs where the type changes along the path.",
       ["path-type", "cubical-type-theory", "cubical-agda"], "beginner"),

    mc(23,
       "hcomp (homogeneous composition) in cubical TT takes as input:",
       ["two paths and returns their concatenation directly",
        "a system of paths on the boundary of a cube and produces the remaining face",
        "a type and two terms and produces a path",
        "a path and a point and lifts the point"],
       1,
       "hcomp fills open boxes: given partial elements agreeing on a system of faces, "
       "it produces the remaining face. This generalizes both path composition and transport.",
       ["hcomp", "cubical-type-theory"], "beginner"),

    blank(23,
          "In cubical type theory, the two endpoints of the interval I are named ___ and ___.",
          ["i0 and i1", "0 and 1"],
          "The interval I has i0 (the 'left' endpoint, like 0∈[0,1]) and i1 (the 'right' endpoint). "
          "A path p: I→A with p i0 = a and p i1 = b witnesses a = b.",
          ["interval", "cubical-type-theory"], "beginner"),
]


# ── Chapter 24: Simplicial Type Theory — 10 BEGINNER questions ────────────────

CH24_BEGINNER = [
    mc(24,
       "In Riehl-Shulman simplicial type theory, the hom type hom_A(a, b) represents:",
       ["the identity type a = b",
        "the type of directed morphisms from a to b (not necessarily invertible)",
        "the function type A → B",
        "the loop space of A at a"],
       1,
       "hom_A(a,b) is the type of 'arrows' from a to b in the ∞-category A. "
       "Unlike a = b, morphisms in hom_A are not automatically invertible.",
       ["hom-type", "simplicial-type-theory"], "beginner"),

    mc(24,
       "A Segal type in synthetic (∞,1)-category theory is a type A where:",
       ["all morphisms are invertible",
        "the Segal condition holds: every composable pair uniquely composes",
        "A has decidable equality",
        "A is a mere proposition"],
       1,
       "The Segal condition: the natural map hom(a,b) × hom(b,c) → hom(a,c) has contractible fibers. "
       "This corresponds to inner horn filling in the simplicial model.",
       ["segal-condition", "simplicial-type-theory"], "beginner"),

    tf(24,
       "In a Rezk type (complete Segal space), isomorphic objects are equal.",
       True,
       "Completeness = the Rezk condition: isomorphisms a ≅ b imply equality a = b. "
       "This is the synthetic analog of univalence for (∞,1)-categories.",
       ["complete-segal-space", "simplicial-type-theory"], "beginner"),

    mc(24,
       "Rzk is:",
       ["an extension of Lean 4 for directed type theory",
        "a proof assistant based on Riehl-Shulman's simplicial type theory",
        "a tactic in Cubical Agda",
        "a library for Coq/Rocq"],
       1,
       "Rzk implements simplicial/directed type theory from scratch, following the "
       "Riehl-Shulman paper. It is used to formalize results in synthetic (∞,1)-category theory.",
       ["rzk", "simplicial-type-theory"], "beginner"),

    mc(24,
       "In simplicial type theory, a functor between Segal types A and B is:",
       ["a special morphism in the hom type",
        "a plain function f: A → B (automatically preserving composition up to homotopy)",
        "a proof that A ≃ B",
        "a natural transformation from A to B"],
       1,
       "A key feature of synthetic category theory: functors = plain functions between Segal types. "
       "Preservation of composition follows from the Segal condition automatically.",
       ["functors", "simplicial-type-theory", "segal-condition"], "beginner"),

    tf(24,
       "Every type in simplicial type theory is a Segal type.",
       False,
       "Discrete types (where hom(a,b) ≃ a=b) are not Segal in the directed sense. "
       "Being Segal is an additional structure imposed on a type, not held by all types.",
       ["segal-condition", "discrete", "simplicial-type-theory"], "beginner"),

    mc(24,
       "The key difference between ordinary HoTT and simplicial type theory is:",
       ["simplicial TT has no univalence",
        "simplicial TT has directed morphisms (hom types) alongside undirected paths (identity types)",
        "simplicial TT uses a different interval",
        "simplicial TT is classical"],
       1,
       "HoTT has only symmetric paths (a=b). Simplicial TT adds directed morphisms "
       "hom_A(a,b) that need not be invertible, enabling synthetic (∞,1)-category theory.",
       ["simplicial-type-theory", "directed"], "beginner"),

    mc(24,
       "The synthetic Yoneda lemma in Segal type theory states that for a Segal type A:",
       ["every Segal type is equivalent to a hom type",
        "natural transformations from hom(a,-) to F are equivalent to elements of F(a)",
        "every functor has a right adjoint",
        "A is complete if and only if it is a Kan fibration"],
       1,
       "Synthetic Yoneda: Nat(hom(a,-), F) ≃ F(a), naturally. This follows from the Segal condition "
       "and the fact that hom(a,a) has a canonical element (the identity).",
       ["yoneda", "simplicial-type-theory"], "beginner"),

    mc(24,
       "A natural transformation between functors F, G: A → B (Segal types) in simplicial TT is:",
       ["a proof that F = G",
        "an element of the hom type hom_{A→B}(F, G) — a directed path in the function type",
        "a family of morphisms F(a) → G(a) plus coherence proofs",
        "an equivalence F ≃ G"],
       1,
       "In synthetic category theory, natural transformations = directed paths in function types. "
       "This collapses the definition: α: hom_{A→B}(F,G) is a term of the hom type, "
       "and components α(a): hom_B(F(a), G(a)) are its evaluations.",
       ["natural-transformation", "simplicial-type-theory"], "beginner"),

    blank(24,
          "A Segal type where every equivalence is an equality of objects is called a ___ type.",
          ["Rezk", "complete Segal"],
          "Rezk types = complete Segal spaces in the synthetic setting. "
          "The Rezk condition is the directed analog of the univalence axiom.",
          ["complete-segal-space", "rezk", "simplicial-type-theory"], "beginner"),
]


# ── Chapter 25: Modal HoTT — 10 BEGINNER questions ────────────────────────────

CH25_BEGINNER = [
    mc(25,
       "A modality in HoTT is:",
       ["a modal operator from classical modal logic (□, ◇)",
        "an idempotent monadic operation on types: a reflective localization",
        "a higher inductive type",
        "a universe level annotation"],
       1,
       "A modality is an operation ○: Type → Type with a unit η: A → ○A that is universal "
       "(○-modal types are exactly the local types). Truncations and the flat/sharp modalities are examples.",
       ["modality", "modal-hott"], "beginner"),

    mc(25,
       "The flat modality ♭A in cohesive HoTT represents:",
       ["the suspension of A",
        "the type A with all higher path structure collapsed",
        "the 'discrete' or 'locally constant' version of A",
        "the propositional truncation of A"],
       2,
       "♭A is the 'underlying discrete type': it picks out the constant/locally-constant elements of A. "
       "In real-cohesive HoTT, ♭A corresponds to the set of locally constant maps into A.",
       ["flat-modality", "cohesive-hott"], "beginner"),

    tf(25,
       "The adjunction ∫ ⊣ ♭ ⊣ ♯ means that shape is left adjoint to flat, and flat is left adjoint to sharp.",
       True,
       "In cohesive HoTT: shape ∫ ⊣ flat ♭ ⊣ sharp ♯. These three modalities capture the "
       "cohesive structure (differential-geometric intuition) of the types.",
       ["cohesive-hott", "modality"], "beginner"),

    mc(25,
       "The shape modality ∫A (or 'shape of A') in cohesive HoTT represents:",
       ["the propositional truncation of A",
        "the underlying homotopy type of A, forgetting smooth/differential structure",
        "the loop space of A",
        "the discrete set of connected components"],
       1,
       "∫A is the 'geometric realization' or 'fundamental ∞-groupoid' of A. "
       "In real-cohesive HoTT, ∫A for a smooth space A is its classical homotopy type.",
       ["shape-modality", "cohesive-hott"], "beginner"),

    mc(25,
       "A type A is ♭-modal (or 'discrete') in cohesive HoTT if:",
       ["A has no higher paths",
        "the unit A → ♭A is an equivalence",
        "A is contractible",
        "A is a mere proposition"],
       1,
       "A type is ○-modal for a modality ○ when the unit η_A: A → ○A is an equivalence. "
       "Discrete types (♭-modal) have no non-trivial smooth structure.",
       ["flat-modality", "discrete", "cohesive-hott"], "beginner"),

    tf(25,
       "The propositional truncation ‖A‖₋₁ is an example of a modality.",
       True,
       "Truncations are lex modalities. ‖-‖₋₁ is (-1)-truncation; ‖-‖₀ is 0-truncation (set quotient), etc. "
       "More generally, n-truncation is a lex modality for each n ≥ -2.",
       ["propositional-truncation", "modality"], "beginner"),

    mc(25,
       "A lex modality is a modality that additionally:",
       ["is involutive (○○A ≃ ○A)",
        "preserves finite limits (products and pullbacks)",
        "turns every type into a set",
        "satisfies the axiom of choice"],
       1,
       "Lex (= left exact) modalities preserve pullbacks. The n-truncations are lex modalities; "
       "the propositional truncation ‖-‖₋₁ is lex. Non-lex modalities include ♭ in some models.",
       ["lex-modality", "modal-hott"], "beginner"),

    mc(25,
       "Real cohesive HoTT is a variant of modal HoTT where cohesion is defined over:",
       ["the integers ℤ",
        "the real numbers ℝ, enabling synthetic differential geometry",
        "a Grothendieck topos",
        "the Boolean algebra 2"],
       1,
       "Real-cohesive HoTT (Shulman) uses ℝ-cohesion to do synthetic differential geometry: "
       "smooth functions, de Rham cohomology, and Chern-Weil theory all become synthetic.",
       ["cohesive-hott", "real-cohesion"], "beginner"),

    mc(25,
       "The sharp modality ♯A in cohesive HoTT represents:",
       ["the suspension of A",
        "the 'codiscrete' or 'indiscrete' version of A (all maps into it are smooth)",
        "the loop space of A",
        "the propositional truncation"],
       1,
       "♯A is codiscrete: every type maps uniquely into ♯A up to homotopy, "
       "but maps out of ♯A are rigid. ♯ is right adjoint to ♭.",
       ["sharp-modality", "cohesive-hott"], "beginner"),

    blank(25,
          "The three cohesive modalities are arranged in an adjoint triple: "
          "___ ⊣ flat ♭ ⊣ sharp ♯.",
          ["shape ∫", "∫"],
          "The adjoint triple ∫ ⊣ ♭ ⊣ ♯ is the defining structure of cohesion. "
          "Shape ∫ is left adjoint to flat ♭; flat ♭ is left adjoint to sharp ♯.",
          ["cohesive-hott", "modality"], "beginner"),
]


# ── Chapter 26: Research Frontiers — 10 BEGINNER questions ────────────────────

CH26_BEGINNER = [
    mc(26,
       "Brunerie's number refers to:",
       ["the Euler characteristic of S⁴",
        "the value of π₄(S³) computed via a formalized HoTT proof in Agda",
        "the number of non-trivial path components of the universe",
        "the first Betti number of a certain HIT"],
       1,
       "Guillaume Brunerie formalized a proof that π₄(S³) = ℤ/2ℤ in Agda/HoTT. "
       "The proof extracted a natural number 'n'; computing n=2 required significant effort.",
       ["brunerie", "synthetic-homotopy", "pi4-s3"], "beginner"),

    mc(26,
       "2-Level Type Theory (2LTT) is designed to have:",
       ["two universe levels (Type 0 and Type 1)",
        "both a fibrant layer (with univalence) and a strict layer (without univalence) simultaneously",
        "two different notions of equality",
        "a classical and an intuitionistic mode"],
       1,
       "2LTT (Annenkov-Capriotti-Kraus-Sattler) allows reasoning about both HoTT (fibrant) "
       "and strict metatheory in the same system — needed for synthetic approaches to model theory.",
       ["2ltt", "research-frontiers"], "beginner"),

    tf(26,
       "Canonicity for Book HoTT (that every closed term of ℕ reduces to a numeral) has been fully proved.",
       False,
       "Canonicity for Book HoTT (with the univalence axiom as a postulate) remains open. "
       "Cubical type theory achieves canonicity by making univalence computational.",
       ["canonicity", "research-frontiers"], "beginner"),

    mc(26,
       "The Blakers-Massey theorem in HoTT (proved by Anel-Biedermann-Finster-Joyal 2017) is a:",
       ["result about the fundamental group of the circle",
        "connectivity bound for the comparison map between homotopy pushouts and pullbacks",
        "proof that all higher homotopy groups of spheres are finitely generated",
        "formalization of the Hopf fibration"],
       1,
       "Blakers-Massey: if f: A→C and g: B→C are m- and n-connected, then the comparison map "
       "A×_C B → A×B is (m+n)-connected. In HoTT, this was first proved using lex modalities.",
       ["blakers-massey", "research-frontiers", "synthetic-homotopy"], "beginner"),

    mc(26,
       "The agda/cubical library is:",
       ["a library for classical mathematics in Agda",
        "the main library for Cubical Agda, containing formalized HoTT results",
        "a library of simplicial set algorithms",
        "a Lean 4 package for homotopy theory"],
       1,
       "The agda/cubical library (github.com/agda/cubical) formalizes: the circle and π₁(S¹)=ℤ, "
       "set quotients, Eilenberg-MacLane spaces, and much more, all in Cubical Agda.",
       ["cubical-agda", "formalization", "research-frontiers"], "beginner"),

    mc(26,
       "π₄(S³) equals:",
       ["ℤ",
        "ℤ/2ℤ",
        "ℤ/4ℤ",
        "0"],
       1,
       "π₄(S³) = ℤ/2ℤ, the cyclic group of order 2. This is Brunerie's theorem, formalized in Agda. "
       "The generator is related to the Hopf fibration S¹ → S³ → S² via the EHP sequence.",
       ["pi4-s3", "brunerie", "homotopy-groups"], "beginner"),

    tf(26,
       "Directed HoTT aims to extend HoTT to handle ∞-categories (with non-invertible morphisms).",
       True,
       "Directed/simplicial HoTT (Riehl-Shulman) adds directed morphisms (hom types) alongside "
       "undirected paths. This enables synthetic (∞,1)-category theory inside type theory.",
       ["directed", "research-frontiers"], "beginner"),

    mc(26,
       "XTT (Extended Cubical Type Theory) differs from standard cubical TT by:",
       ["adding classical logic",
        "removing univalence",
        "adding a strict (definitionally proof-irrelevant) equality type alongside the path type",
        "using a non-computational interval"],
       2,
       "XTT (Sterling-Angiuli-Gratzer) adds a 'boundary separator' / strict equality. "
       "This gives definitional proof irrelevance for the strict equality, useful for practical formalization.",
       ["xtt", "cubical-type-theory", "research-frontiers"], "beginner"),

    mc(26,
       "Which of the following is an open problem in HoTT as of 2025?",
       ["whether π₁(S¹) = ℤ (already proved)",
        "coherence for general HITs (that arbitrary HIT elimination rules are consistent)",
        "whether univalence is consistent (already proved via simplicial set model)",
        "function extensionality (already follows from univalence)"],
       1,
       "General HIT coherence: it is not fully proved that all HITs with arbitrary path constructors "
       "can be realized in all models. Specific HITs (circle, pushouts, truncations) are fine.",
       ["hit", "research-frontiers", "open-problems"], "beginner"),

    blank(26,
          "The proof assistant designed for synthetic (∞,1)-category theory based on "
          "Riehl-Shulman's framework is called ___.",
          ["Rzk"],
          "Rzk (rzk-lang.github.io) implements simplicial/directed type theory. "
          "It is used to formalize results like the Yoneda lemma and adjoint functor theorem synthetically.",
          ["rzk", "simplicial-type-theory", "research-frontiers"], "beginner"),
]


# ── Write all files ────────────────────────────────────────────────────────────

def write_questions(chapter: int, questions: list[dict], start: int = 50):
    ch_dir = QUESTIONS_DIR / CHAPTER_DIRS[chapter]
    ch_dir.mkdir(exist_ok=True)
    for i, q_data in enumerate(questions):
        filename = ch_dir / f"{start + i:03d}.json"
        filename.write_text(json.dumps(q_data, indent=2, ensure_ascii=False))
    print(f"  ch{chapter:02d}: wrote {len(questions)} files "
          f"({start:03d}–{start+len(questions)-1:03d})")


def main():
    print("Writing balancing questions...")
    write_questions(11, CH11_BEGINNER)
    write_questions(12, CH12_BEGINNER)
    write_questions(13, CH13_ADVANCED)
    write_questions(16, CH16_ADVANCED)
    write_questions(17, CH17_ADVANCED)
    write_questions(21, CH21_ADVANCED)
    write_questions(23, CH23_BEGINNER)
    write_questions(24, CH24_BEGINNER)
    write_questions(25, CH25_BEGINNER)
    write_questions(26, CH26_BEGINNER)
    print("Done.")


if __name__ == "__main__":
    main()
