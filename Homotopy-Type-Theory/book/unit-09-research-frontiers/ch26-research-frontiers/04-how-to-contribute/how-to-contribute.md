# 04 — How to Contribute

## A Field Small Enough to Enter

HoTT is, by the standards of modern mathematics, a small field. There are perhaps a few hundred active researchers worldwide who work primarily in HoTT or adjacent areas of dependent type theory and synthetic homotopy theory. The entire corpus of significant papers fits on a bookshelf. The researchers whose names appear in this curriculum are, most of them, accessible — they post on Zulip, they respond to email, they give talks at publicly accessible seminars.

This smallness is not a limitation. It is an invitation.

A student who works through a curriculum like this one, does the exercises seriously, and engages with the formalization infrastructure is not a peripheral consumer of HoTT. They are — by the standards of this field — a prepared participant. The community is small enough that a motivated person, arriving with genuine preparation, can contribute something real within a year or two. Not to the periphery. To the center.

What follows is a practical guide to actually doing this.

---

## The Community Infrastructure

**The HoTT Zulip: hott.zulipchat.com**

This is the central online community for HoTT research. Registration is free and open to anyone. The main streams are:

- **general**: Primary discussion channel. Current research, questions, announcements. Active researchers post here regularly, including the people whose papers appear in this curriculum.
- **announcements**: New papers, workshops, conference programs. Browsing this stream gives you a real-time picture of what the field is doing.
- **HoTTEST**: Discussion related to the HoTTEST electronic seminar.
- **Agda**: Cubical Agda questions, library discussions, formalization problems.
- **Lean**: Lean 4 and Mathlib questions.
- **jobs**: Postdoctoral and faculty positions. If you are looking for a research position, this stream is essential.

**How to participate productively.** Post specific questions. "I'm formalizing the Freudenthal suspension theorem in Cubical Agda and stuck on the connectivity argument — specifically, the `is-connected-Susp` lemma gives me n-connected suspension but I need to transport this across the equivalence `Susp X ≃ pushout f g`; is there a standard technique in the library for this?" is a good question. "I don't understand Freudenthal" is not.

When you have solved a problem or completed a formalization, post it too. The community benefits from knowing what has been done and what techniques worked.

**The HoTTEST Seminar**

HoTTEST (Homotopy Type Theory Electronic Seminar Talks) is a free online seminar series running since 2019. Talks are given by active researchers on current work: new results, open problems, research directions, and tools. The talks are recorded and available at:

    uwo.ca/math/faculty/kapulkin/seminars/hottest.html

The organizers (currently Kris Kapulkin and others) maintain an archive of all talks. Watching 5–10 recent talks is one of the fastest ways to understand what the frontier looks like from the inside — not just "what theorems have been proved" but "why do the researchers care, what are they excited about, what is hard?"

To get notifications of new talks: join the HoTT Zulip (the HoTTEST stream announces each talk) or sign up for the mailing list at the seminar website.

**The nLab**

The nLab (ncatlab.org) is a collaborative wiki for higher mathematics: category theory, homotopy theory, type theory, mathematical physics. It is not a formalization library — it is a reference written in informal mathematics — but it is often the best source for understanding how a specific concept fits into the broader landscape. When you encounter a term in a paper and the paper does not define it, the nLab is the first place to look.

Contributing to the nLab is open to anyone with an account. The bar for nLab pages is informal mathematical accuracy, not formalization. If you learn something that is not well-explained in any existing nLab page, adding an explanation or improving an existing page is a genuine contribution to the mathematical literature.

**The arXiv**

New HoTT results appear on the arXiv before (sometimes long before) journal publication. The relevant categories:

- **math.LO** (mathematical logic): foundations, set theory, type theory
- **math.AT** (algebraic topology): homotopy groups, spectra, chromatic homotopy
- **cs.LO** (logic in computer science): proof assistants, type theory, formalization

Set up arXiv email alerts for:
- "homotopy type theory"
- "cubical type theory"
- "simplicial type theory"
- "univalent foundations"
- Your own specific research areas

The arXiv digest emails (one per weekday) arrive early morning and list new preprints with titles and abstracts. Scanning these takes 5 minutes and keeps you current.

---

## Finding an Advisor and a Research Direction

**If you are a graduate student.** Finding an advisor who works in HoTT or closely adjacent areas (dependent type theory, algebraic topology, proof assistants) is the most direct path. The HoTT Zulip "jobs" stream posts faculty and postdoc openings. Researchers who are actively looking for students often say so in their talks or on their academic webpages.

Active HoTT advisors (as of 2025–2026) include:
- **Anders Mörtberg** (Stockholm University): Cubical Agda, synthetic homotopy theory, formalization
- **Emily Riehl** (Johns Hopkins University): Simplicial type theory, ∞-category theory
- **Ulrik Buchholtz** (University of Nottingham): Synthetic homotopy theory, modal HoTT
- **Egbert Rijke** (University of Ljubljana): Synthetic homotopy theory, HoTT Book-style mathematics
- **Mike Shulman** (University of San Diego): Modal HoTT, cohesive HoTT, foundations
- **Evan Cavallo** (University of Gothenburg): Cubical type theory, internal models
- **Tom de Jong** (University of Edinburgh): Domain theory in HoTT, order theory

**If you are not in a PhD program.** The community is accessible without institutional affiliation. Formalization contributions (to Cubical Agda, sHoTT, or UniMath) do not require academic credentials — the code is on GitHub, the community is on Zulip, and the result is a contribution to a public repository that anyone can see. Several significant contributions to the Cubical Agda library have been made by people working outside traditional academic positions.

The key, regardless of institutional situation, is engagement with the community before, during, and after your work. Post about what you are trying to do, ask for help when stuck, and share your results when you make progress.

---

## The Research Ladder: Specific Steps

Here is a concrete sequence of actions, ordered by difficulty and time investment.

**Weeks 1–4: Get the tools working**

1. Install Agda and the Cubical Agda library (instructions at the library README).
2. Load an existing file in the library interactively in your editor (Emacs with `agda-mode` or VS Code with the Agda extension).
3. Navigate to `Cubical/HITs/S1/Base.agda`. Read the definition of S¹ as a HIT. Check that you understand the recursion principle `rec` and the induction principle `elim`.
4. Write a small Agda file: define the constant function on S¹ and verify it satisfies the expected equation on `loop`.

This gives you a working environment and a minimal proof of concept.

**Months 1–3: Formalization practice**

1. Choose one theorem from Chapters 20–25 of this curriculum that has a clear statement but is not yet in the Cubical Agda library (check the library before choosing).
2. Formalize the statement as an Agda type. Check that it compiles with a hole for the proof.
3. Attempt the proof, using library lemmas where possible.
4. Post your progress on the HoTT Zulip Agda stream. Even if the proof is incomplete, a well-stated goal and a report on obstacles is valuable to the community.

**Months 3–9: First contribution**

1. Browse the Cubical Agda library issues (github.com/agda/cubical/issues). Find an issue labeled "wanted theorem" or "good first issue" that you understand.
2. Check that the theorem is not already proved (search the library).
3. Write the formalization. Use holes liberally; fill them in iteratively.
4. Submit a pull request. The review process will be collaborative: the maintainers will suggest improvements and the final result will be stronger.

**Months 9–18: Novel result**

After a successful library contribution, you understand the tools, you have navigated the community, and you know where the gaps are. From here:

1. Identify a theorem that is *not* in any library — something that requires either new mathematical content or new library infrastructure.
2. Write a mathematical sketch of the proof first. Check it with a human (advisor, colleague, Zulip). Only then formalize.
3. If the formalization reveals a gap or an error in the sketch, report this. A gap discovered during formalization is a research result.
4. Write up the result as a paper: a formalization paper reports what you proved, how, what was hard, and what the formalization revealed. Submit to arXiv immediately; then to ITP (Interactive Theorem Proving) or TYPES.

---

## What to Read and in What Order

**For immediate use after finishing this book:**

1. Brunerie's thesis introduction (arXiv:1606.05916, pages 1–20): to understand what synthetic homotopy theory can achieve and what its hard problems look like from the inside.
2. Ljungström-Mörtberg (LICS 2023): to understand the current state of the Brunerie number computation.
3. Riehl-Shulman (arXiv:1705.07442, introduction and Sections 1–3): to understand the central open problem (directed univalence) and the simplicial type theory program.
4. Browse the Cubical Agda library (github.com/agda/cubical/Cubical/Homotopy/): to see what has been formalized and at what level of sophistication.

**For deeper research:**

- Anel-Biedermann-Finster-Joyal (Journal of Topology 2020): the Blakers-Massey theorem in ∞-toposes. The proof technique (excisive functors, the orthogonal factorization system approach) is standard in current synthetic homotopy theory.
- Shulman (MSCS 2018): cohesive HoTT and the Brouwer fixed-point theorem. A model paper for how to write a cohesive HoTT result.
- Kudasov-Riehl-Weinberger (CPP 2024): the formalization of the Yoneda lemma in Rzk. The paper itself is a model of a formalization paper.
- Lumsdaine-Shulman (2020): the semantics of HITs. Essential for anyone working on the coherence problem.

---

## Conferences

The main venues for HoTT research, in rough order of importance:

**TYPES** (Types for Proofs and Programs): Annual European conference for dependent type theory. Informal proceedings (abstracts only). Good for early-stage work and for meeting the European research community. Accepting to TYPES means your work is interesting enough for a 20-minute talk; it does not have the publication weight of LICS.

**ITP** (Interactive Theorem Proving): The primary venue for formalization results. Selective and peer-reviewed. If you formalize a significant theorem, ITP is the right venue. Proceedings published by Springer or LIPIcs.

**LICS** (Logic in Computer Science): For foundational results — new type theories, canonicity proofs, semantics. Highly selective. The top venue for type theory that is primarily logic or CS theory rather than formalization.

**POPL** (Principles of Programming Languages): For PL-oriented results — type systems, semantics, programming languages. High visibility, strong refereeing.

**FSCD** (Formal Structures in Computation and Deduction): For term rewriting, proof theory, type theory. Less competitive than LICS, broader scope.

**CT** (International Category Theory conference): Annual, primarily pure mathematics. HoTT results that are primarily category-theoretic (simplicial type theory, synthetic ∞-category theory) fit here.

You do not need to attend conferences to do research. But attending at least one conference within your first two years of research work is enormously valuable. You will meet people whose papers you have read, understand the culture of the community, and come back with a clearer sense of what the field cares about.

---

## The Practical Reality

Research is mostly not breakthroughs. It is mostly: reading papers until you understand a small piece, writing code that fails and then figuring out why, posting a question and getting a response that clarifies one step while revealing three more obstacles, revising a proof sketch after realizing a step does not work, trying a different approach, and eventually — after weeks or months of this — having something that works.

This is not a discouraging description. It is an accurate one. The experience of doing this work is different from the experience of reading about the results. Results look clean because the cleaning happened before publication. The work itself is messy.

The appropriate response to this is not discouragement but preparation. You are better prepared for the mess if you have a specific problem (not "I want to learn HoTT" but "I am trying to formalize the inductive step of π_n(Sⁿ) = ℤ using Freudenthal"), a specific tool (Cubical Agda, with the library cloned and loading), and a specific community to ask (the HoTT Zulip, with a well-formulated question posted).

This curriculum has given you the preparation. The specific problem, tool, and community are available. What happens next is up to you.
