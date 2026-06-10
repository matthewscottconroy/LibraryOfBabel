# 3.1 How to Begin Contributing

## Reading the Literature

Research begins with reading. But the HoTT literature is dense and the papers presuppose each other in non-obvious ways. Here is a recommended reading path — not the only path, but one that builds dependencies in the right order.

### The Essential Seven

**1. The HoTT Book (2013)**

*Univalent Foundations Program, "Homotopy Type Theory: Univalent Foundations of Mathematics."*

This is the primary text — it is to HoTT what Hartshorne is to algebraic geometry or Bredon to algebraic topology. Chapters 1–6 are essential: identity types, h-levels, the fundamental theorem of identity types, equivalences, univalence, and HITs. Chapters 7–10 are the research content: homotopy theory, set theory, real numbers, categorical logic.

Don't read the whole book before doing anything else. Read through Chapter 6, then dip into later chapters as you need them.

**2. Awodey-Warren, "Homotopy-Theoretic Models of Identity Types" (2009)**

The paper that first showed why HoTT works: the groupoid model gives an interpretation of identity types where Martin-Löf's J rule holds but the uniqueness of identity proofs (UIP) does not. This is the conceptual foundation for all of HoTT.

Read this alongside Chapter 1 of the HoTT Book. It is short (15 pages) and clarifies why the identity type behaves the way it does.

**3. Cohen-Coquand-Huber-Mörtberg, "Cubical Type Theory: A Constructive Interpretation of the Univalence Axiom" (2015)**

The CCHM paper introducing the cubical type theory that underlies Cubical Agda. This is the technical foundation for Chapter 23 of this curriculum. Read it after you understand Book HoTT; the contrast between "univalence as axiom" and "univalence as theorem via Glue" becomes clear.

The key insight: paths are functions $I \to A$ on the De Morgan interval, and univalence follows from the *Glue* type constructor that glues partial type equivalences into a global type.

**4. Riehl-Shulman, "A Synthetic Theory of ∞-Categories in Homotopy Type Theory" (2017)**

The foundational paper for simplicial type theory and synthetic ∞-category theory. Read this after Chapter 24. The paper introduces the two-interval framework, Segal types, the Yoneda lemma, and the beginnings of the theory of (co)cartesian fibrations.

Warning: this paper is long (100+ pages). The introduction and Sections 1–5 give the essential ideas; the later sections are for reference.

**5. Brunerie, "On the Homotopy Groups of Spheres in Homotopy Type Theory" (2016, PhD thesis)**

The proof of $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ entirely within HoTT. Dense but essential for anyone interested in synthetic homotopy theory. The thesis constructs the Hopf invariant, the EHP sequence, and the Brunerie number.

Don't try to read this linearly. Read the introduction and Chapter 1, then pick up individual chapters as needed. The conceptual outline is in the introduction; the proofs fill in later.

**6. Anel-Biedermann-Finster-Joyal, "A Generalized Blakers-Massey Theorem" (2017)**

The synthetic Blakers-Massey theorem, proved in an arbitrary ∞-topos (not just spaces). The paper introduces the technique of *excisive* functors and shows that the Blakers-Massey connectivity bound follows from a formal argument about pushouts.

This paper represents the state of the art in synthetic homotopy theory and is essential for understanding current research directions.

**7. Shulman, "Brouwer's Fixed-Point Theorem in Real-Cohesive Homotopy Type Theory" (2018)**

The flagship application of cohesive HoTT (Chapter 25). The paper proves the Brouwer fixed-point theorem synthetically — without coordinates or analysis — using the shape modality. It demonstrates that cohesive HoTT can do serious mathematics cleanly.

This is also a model of how to write a cohesive HoTT paper: the cohesion axioms are clearly stated, the proof is self-contained, and the mathematical content is at the forefront.

### How to Read Papers

The trick is not to read papers from top to bottom. The abstract tells you the result. The introduction tells you the proof strategy and where it fits in the literature. The body contains the proofs. For most purposes — understanding what the paper does and whether it's relevant to you — the abstract and introduction are enough.

When you want the proof of a specific result, go directly to that section. Read the statement, check the definitions it uses, then read the proof. Don't read the parts you don't need.

Mark everything you don't understand the first time. Some things become clear on a second pass; others require looking up background. Learn to distinguish between "I don't understand this yet" (background is missing) and "this is unclear" (the writing is bad). The former is fixed by studying; the latter is fixed by finding another reference.

For HoTT papers specifically: always keep the HoTT Book within reach. Most papers cite the Book for standard results. When a paper says "by Lemma 3.4.3 of [HoTT]," find that lemma and understand it before continuing.

### Staying Current

The field moves fast. New results appear on arXiv regularly. The best way to stay current:

- **arXiv**: cs.LO (logic in computer science) and math.LO and math.AT. Subscribe to the cs.LO daily digest.
- **HoTT Zulip** (hott.zulipchat.com): announcements stream posts new papers as they appear.
- **Google Scholar alerts**: set up alerts for "homotopy type theory," "cubical type theory," "simplicial type theory."
- **Conference proceedings**: LICS, TYPES, ITP, POPL. Browse the proceedings every year.

---

## Finding Your Problem

The right problem is one you can actually work on. That sounds obvious, but it rules out a lot. "Prove the cobordism hypothesis in simplicial type theory" is not a good first problem. "Formalize $\pi_n(S^n) = \mathbb{Z}$ for all $n$ in Cubical Agda" is.

### What Makes a Good First Problem

A good first research problem has three properties:

**It is clearly stated.** You know exactly what you are trying to prove. There should be no ambiguity about the statement, only difficulty about the proof.

**It is connected to existing work.** The infrastructure is there — definitions, library code, prior results — and your contribution is filling a specific gap rather than building a new theory from scratch.

**It is at the edge of your current ability.** If it is too easy, it's not research. If it is too hard, you'll get stuck indefinitely. The sweet spot is a problem where you understand all the pieces but haven't put them together.

### Where to Find Problems

**The Cubical Agda library issues.** Go to github.com/agda/cubical/issues and filter by labels "wanted theorem," "enhancement," or "good first issue." These are problems identified by the library maintainers as valuable contributions that haven't been done. Some are straightforward; some are hard. Read several and identify which ones you understand well enough to attempt.

**The "future work" sections of papers.** Every paper ends with "future work" or "open questions." These are real problems — the authors couldn't solve them and are telling you so. After reading a paper, write down all the open problems it mentions. Some of them will be accessible given your background.

**The HoTT Zulip.** The "jobs" stream (and others) sometimes post specific research problems. The "general" stream discusses ongoing work. Reading a few weeks of conversation will give you a sense of what people are working on and where the gaps are.

**The HoTT Book exercises.** Many exercises are explicitly marked as "open research problems." These are canonical problems that the community cares about.

### The Research Ladder

Here is a concrete ladder of problems in increasing difficulty:

**Step 1 (months 1-3): Formalization practice**
- Formalize a theorem from the HoTT Book that isn't in the Cubical Agda library yet
- Complete an exercise from this curriculum formally in Cubical Agda
- Fix a small bug or add a missing lemma to the Cubical library

**Step 2 (months 3-9): First contribution**
- Formalize $\pi_n(S^n) = \mathbb{Z}$ for all $n$ using Freudenthal (accessible, library infrastructure is there)
- Formalize the Mayer-Vietoris sequence for pushouts in Cubical Agda
- Prove the Seifert-van Kampen theorem in Lean 4 (classical, no HITs required)

**Step 3 (months 9-18): Novel result**
- Extend a result from a recent paper to a case not covered by the paper
- Formalize a result from algebraic topology not yet in any proof assistant
- Find a cleaner proof of a known result using HoTT methods

**Step 4 (year 2+): Research paper**
- Prove something new: a result not known before, or a new proof of a known result with better properties
- Identify an open problem, work on it, report progress

Don't skip steps. The step 1 work is not just preparation for later steps — it is how you discover what is actually hard and what is actually easy. Many things that look hard are easy once you know the tools; many things that look easy are hard for non-obvious reasons. You only learn this by doing.

---

## Engaging with the Community

HoTT is a small field — roughly a few hundred active researchers worldwide. This is a feature, not a bug. You can realistically interact with the people whose papers you read. The community is welcoming to newcomers who have done serious preparation.

### The HoTT Zulip

The HoTT Zulip (hott.zulipchat.com) is the central online community. It has streams for:
- **general**: Main discussion, current research, questions
- **jobs**: Postdoctoral and faculty positions
- **announcements**: New papers, workshops, talks
- **HoTT Electronic Seminar**: Discussion related to the HoTTEST seminar series
- **Lean**: Lean 4 and Mathlib questions
- **Agda**: Cubical Agda and related

When you have a question — about a paper, a formalization, an open problem — post it here. Include enough context that someone who hasn't been thinking about your problem for a week can understand the question. Be specific: "I'm trying to formalize Freudenthal and stuck on the connectivity argument — the `is-connected-Susp` lemma in the library gives n-connectivity of the suspension but I need to transport this to the loop space; is there a standard technique?" is a good question. "I don't understand Freudenthal" is not.

### The HoTTEST Seminar

HoTTEST (Homotopy Type Theory Electronic Seminar Talks) is a free online seminar series. Talks are given by active researchers on current work, including open problems, recent results, and research directions. Watching these talks is one of the best ways to see what the field looks like from the inside — not just what results have been proved, but why they matter and what questions they leave open.

The recordings are available online. The talks from 2019 onward are a good survey of recent research.

### Conferences

The main conference venues for HoTT research, in rough order of centrality:

**TYPES** (Types for Proofs and Programs): The main European conference for dependent type theory. Annual. Informal proceedings (abstracts only); good for early work.

**LICS** (Logic in Computer Science): Broader CS logic. Competitive. For foundational results (new type theories, canonicity proofs, semantics).

**ITP** (Interactive Theorem Proving): The main venue for formalization results. If you formalize a significant theorem, ITP is where to report it.

**POPL** (Principles of Programming Languages): For PL-oriented results. Higher visibility than TYPES.

**FSCD** (Formal Structures in Computation and Deduction): For term rewriting, proof theory, type theory.

You don't need to attend conferences to contribute — much of the real conversation happens on Zulip and arxiv. But attending at least one conference is enormously useful for understanding the field's culture and making personal connections.

### Working with an Advisor

If you are a graduate student, finding an advisor who works in HoTT or related areas (dependent type theory, algebraic topology, proof assistants) is the most direct path. The HoTT community has researchers at many universities; the "jobs" stream on Zulip posts openings regularly.

If you are not in a PhD program, the community is still accessible. Many people do significant HoTT work outside of academia (formalization contributions, in particular, don't require institutional affiliation). The key is engaging with the community, posting your work, and responding to feedback.

---

## Writing and Publishing

### What to Write

**A formalization paper** reports a significant formalized proof: what theorem was proved, in which system, using which techniques, what was hard, and what new ideas the formalization required. The mathematical content is the theorem. The computer science content is the formalization itself — the code, available at a public repository (GitHub), is part of the paper.

Formalization papers answer the question: *what did you learn by doing this?* The formalized proof itself doesn't convey this; the paper does.

**A type theory paper** presents new theory: a new type theory, a new proof technique, a new model, a new connection between type theory and mathematics. These papers are denser and more technical. They require understanding of the existing literature and a clear sense of what is new.

**A problem paper** is unusual but valuable: a paper that clearly states an open problem, explains why it is hard, surveys what is known, and offers partial results or conjectures. These are rare in formal verification but common in mathematics. If you understand an open problem deeply, writing up what is known is a real contribution.

### Where to Publish

**ArXiv first.** Post every preprint to arxiv.org. Use the math.LO or cs.LO categories (math.AT for topology-heavy results). ArXiv submission is immediate and free; it establishes priority and makes the work available to the community right away. This is standard practice in the field.

**Formalization results:**
- ITP (Interactive Theorem Proving): The primary venue. Selective but not impossible for strong formalization work.
- Agda Workshop (associated with ICFP/PLDI): Less formal, good for early Cubical Agda results.
- Lean Together / Mathlib workshops: For Lean 4 contributions.

**Type theory / foundations:**
- LICS: Tier 1 for foundational results.
- POPL: Tier 1 for PL-oriented results.
- FSCD: Good for more technical type theory.
- TYPES proceedings: Informal; good for preliminary results.
- MSCS (Mathematical Structures in Computer Science): Journal; good for mature results.

**For mathematics:**
- Journal of Pure and Applied Algebra (for algebraic K-theory, algebraic topology)
- Algebraic & Geometric Topology (for homotopy theory)
- Advances in Mathematics (for significant results)

**The timeline.** Conference papers are reviewed in 3 months. Journal papers can take 1-3 years. ArXiv establishes priority; don't wait for the conference to post.

### Writing the Paper

The structure of a formalization paper:
1. **Introduction**: What theorem was proved, in which system, why it matters, what was hard
2. **Background**: What the reader needs to know (can be brief if citing standard references)
3. **Main results**: The theorem, its statement, the structure of the proof
4. **Formalization**: What was hard to formalize, what new infrastructure was needed, what the code structure looks like
5. **Conclusion / future work**: What remains, what open questions the paper raises

Keep it concrete. Concrete is better than abstract. Specific is better than general. "We formalize the Brunerie number by computing it as the degree of the Hopf map in dimension 4" is better than "We contribute to the growing body of work on synthetic homotopy theory."

---

## The Longer View

### Why HoTT Matters

This curriculum has covered a lot of ground. It may not always be clear why it all matters — why anyone would spend years developing a type-theoretic foundation for algebraic topology when classical methods work perfectly well.

Here is the honest answer.

For *mathematics*, HoTT matters because it gives a foundation where mathematical practice is formally correct. Mathematicians routinely identify isomorphic structures ("let $G$ be *any* group with property $P$; all such groups are isomorphic so WLOG..."), and this is formally valid in HoTT (univalence) but only informally valid in ZFC. As computer-verified mathematics becomes standard practice — not if, but when — HoTT is the foundation where practice and formalization align.

For *computer science*, HoTT is the rigorous version of the intuitions behind parametric polymorphism, quotient types, and abstract data structures. The Curry-Howard correspondence (propositions as types) was always the right idea; HoTT completes it by adding homotopy groups as identity types and univalence as the formal version of "two things with the same interface are the same thing."

For *physics*, the cohesive HoTT program (Chapter 25) provides a synthetic foundation for gauge theory, topological field theory, and string theory that classical mathematics approaches by cobbling together differential geometry, sheaf theory, and ∞-category theory. The synthetic approach is cleaner — the geometry is built into the type theory — and it opens the possibility of machine-verified physics calculations.

For *foundations*, HoTT is a competitor to ZFC that is arguably more expressive (it can reason about higher-dimensional structures that ZFC must encode indirectly) and more aligned with how mathematicians actually think.

None of this means HoTT is destined to dominate or that other foundations are wrong. It means HoTT is solving real problems, and the solutions are new enough that many of the most important theorems are still being proved.

### The Next Decade

Looking out from 2026:

**Near term**: The computational improvements to Cubical Agda are already paying off — proof checking times are decreasing and the library is growing rapidly. Within two or three years, there will be a mature Lean 4 HoTT library alongside Mathlib, and the canonicity problem for simplicial type theory will likely be solved (or significantly clarified).

**Medium term**: Synthetic algebraic K-theory is within reach — the definitions are in place, the library infrastructure is growing, and the classical theorems to be formalized are known. Within a decade, someone will formalize the Quillen K-theory of a ring in Cubical Agda. The cobordism hypothesis is a longer target, but the simplicial type theory framework is the right language for it.

**Long term**: The long-term arc is toward proof assistants that can do graduate-level mathematics automatically — not by brute-force search, but because the representations (types, paths, homotopies) are close to how mathematicians actually think. HoTT is part of making this possible.

### Your Place in the Story

The HoTT Book was published in 2013. The CCHM cubical paper was 2015. Simplicial type theory was 2017. Modal HoTT was being developed at the same time. Many of the researchers who wrote those papers are in their 30s and 40s now. The students who read the HoTT Book when it came out are now faculty.

You are entering a field that is still establishing its foundations — not in the sense that it is shaky, but in the sense that the fundamental structure is there and the edifice is being built. The Freudenthal suspension theorem is in the library. The Brunerie number is computed. The Hopf fibration is formalized. The Yoneda lemma is proved in simplicial type theory.

What isn't there yet is vast. $\pi_n(S^n)$ for all $n$ is not formalized. Algebraic K-theory is absent. The cobordism hypothesis is unproven in HoTT. Canonicity for simplicial type theory is open. Directed univalence is open.

These aren't obscure technical details. They are the things the field is organized around. Progress on any of them would be noticed.

This curriculum gave you the background. What comes next — reading the papers, finding a problem, engaging with the community, doing the work — is up to you.

The next result could be yours.
