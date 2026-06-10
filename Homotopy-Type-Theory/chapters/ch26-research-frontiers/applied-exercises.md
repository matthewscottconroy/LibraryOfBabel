# Applied Exercises

The exercises in this chapter are different from those in all previous chapters. They are not exercises *about* research frontiers — they are exercises *in* research. There are no worked solutions, because the goal is to develop genuine research skills: reading the literature, working a proof assistant, engaging with the community, and mapping the problem space. Some of these exercises will take hours; a few could take months. The point is not to finish them quickly but to develop the habits of work that make a research career possible.

Complete at least two of the following six exercises before moving on.

---

## Exercise R.1: Finding a Contribution in the Cubical Agda Library
*Domain: Proof Assistant Development / Library Contribution*

**Setup:** The Cubical Agda library (github.com/agda/cubical) is an actively maintained library of machine-verified mathematics in Cubical Agda. It is the primary formalization library for HoTT in any proof assistant. The library maintainers track desired contributions through GitHub issues. This exercise is a genuine research task: identifying an approachable contribution and making progress on it.

**Tasks:**

1. Clone or browse the Cubical Agda library repository. Navigate to the Issues tab and filter by label "wanted theorem" or "enhancement." Read through at least ten issues. For each, assess: (a) what theorem is needed, (b) what it builds on, (c) whether you have the background to attempt it given this curriculum. Write a one-paragraph assessment for each of the three most promising issues.

2. For one of the three issues you assessed as most promising: find the relevant files in the library (the folder where the surrounding theory lives). Read the existing definitions and theorems in those files carefully. Identify what definitions you need to understand before you can state the target theorem. Write a list of five to ten definitions or lemmas from the library that you need to understand.

3. State the target theorem formally — either as a type in Agda syntax or in the HoTT Book's notation. Check that your statement matches what the issue is asking for. If there is ambiguity (e.g., multiple reasonable interpretations), write out each interpretation and identify which is the "right" one by consulting the existing library code and the relevant literature.

4. Attempt a sketch of the proof. Even if you cannot complete it in Agda, write a mathematical proof sketch in English: what are the key steps, what lemmas do you need, what is the induction structure? Note where you get stuck.

5. *Extension:* Make an actual contribution: submit a PR with a partial formalization, or open an issue documenting a precise obstacle you encountered. This is the most valuable outcome, but it is not required to "complete" the exercise — even a well-documented failed attempt is informative.

*Research skills developed: Reading library code, identifying gaps, stating theorems formally, navigating an active open-source mathematical library.*

---

## Exercise R.2: Reading a Recent arXiv Paper
*Domain: Literature Survey / Critical Reading*

**Setup:** The HoTT research literature moves fast. Papers appear on arXiv before (sometimes long before) journal or conference publication. Staying current means reading arXiv preprints, not waiting for published versions. This exercise develops the skill of extracting the essential content from a new paper.

**Tasks:**

1. Browse arxiv.org in the cs.LO category for papers from 2024 or 2025 with "homotopy type theory," "cubical type theory," or "simplicial type theory" in the title or abstract. Select one paper that: (a) connects to topics you have studied in this curriculum, (b) is not longer than 40 pages, and (c) has a clearly stated main theorem in the abstract.

2. Read the abstract and introduction carefully. After reading the introduction, answer these questions *before* reading the rest of the paper: (a) What is the main result? (b) What problem does it solve? (c) What technique does it use that is new? (d) What are the authors' stated open problems? Write down your answers.

3. Read the main theorem statement and its proof sketch (usually in a "main results" section, before the technical development). Map the proof sketch onto the background you have from this curriculum: which steps use technology you know (identity types, HITs, univalence, cubical intervals), and which steps use technology you have not seen before?

4. Identify one specific definition, lemma, or construction in the paper that you do not understand, and trace it to its source: follow the citations back to the paper(s) that introduced the concept. Read the relevant section of the source paper. Write a one-paragraph explanation, in your own words, of what the concept is and why the current paper needs it.

5. Write a one-page summary of the paper suitable for posting to the HoTT Zulip "announcements" stream: title, authors, main result, technique, significance, and one question you have after reading it. (You do not have to post it, but write it as if you would.)

*Research skills developed: Selective reading, extracting main results, tracing bibliographic chains, formulating questions about new material.*

---

## Exercise R.3: A Small Formalization in Cubical Agda
*Domain: Interactive Theorem Proving*

**Setup:** The gap between "understanding a theorem" and "formalizing it" is substantial. This exercise asks you to formalize a small but non-trivial result in Cubical Agda, discovering for yourself where the work actually lies.

**Tasks:**

1. Set up Cubical Agda locally (follow the instructions at the Cubical Agda library README). Verify that the library compiles and that you can load files interactively in your editor (Emacs or VS Code with the Agda mode).

2. Find the file `Cubical/HITs/S1/Base.agda` (or the analogous file in the current library structure). Read it carefully: understand how $S^1$ is defined as a HIT (the base point `base` and the loop `loop`), and how the recursion principle `rec` and the induction principle `elim` are stated.

3. Formalize the following lemma, which should not yet be in the library (check first): for any $n : \mathbb{Z}$, the map $\text{loop}^n : S^1 \to S^1$ (the $n$-fold loop) is a based map. Here "loop$^n$" means the element of $\pi_1(S^1) = \mathbb{Z}$ corresponding to $n$, viewed as a map $S^1 \to S^1$ via the suspension structure. State the lemma precisely as an Agda type before attempting the proof.

4. Alternatively (if the above is too difficult for a first formalization): formalize the following basic lemma about path spaces: for any $A : \text{Type}$ and $x : A$, the loop space $\Omega(A, x) = (x = x)$ has a group structure where the identity is `refl`, the group operation is path concatenation, and the inverse is path reversal. This requires proving associativity, left/right unit, and left/right inverse, all using path-over-path reasoning. Most of this is in the library, but assembling it into a group instance is instructive.

5. Document your formalization: write a comment block at the top of your Agda file explaining (a) what theorem you proved, (b) what was hard, (c) what you had to look up, and (d) one thing you would do differently next time. This documentation is as important as the proof itself.

*Research skills developed: Working with a proof assistant on non-trivial content; understanding the gap between mathematical intuition and formal proof; learning to use library code.*

---

## Exercise R.4: Reading Brunerie's Thesis Introduction
*Domain: Synthetic Homotopy Theory*

**Setup:** Guillaume Brunerie's PhD thesis "On the Homotopy Groups of Spheres in Homotopy Type Theory" (2016) is the canonical landmark in synthetic homotopy theory. Its introduction is one of the best-written expositions of what the HoTT approach to homotopy theory requires and achieves. This exercise develops your understanding of what makes $\pi_4(S^3)$ hard.

**Tasks:**

1. Download Brunerie's thesis (freely available at his website or arXiv:1606.05916). Read the introduction (pages 1–15) carefully and completely. After reading, answer: What is the Brunerie number $n$? Why does showing $n = \pm 2$ imply $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$? What are the three main ingredients of the proof?

2. Read Section 2.1 of the thesis ("The Hopf fibration"). The Hopf fibration $\eta : S^3 \to S^2$ is defined synthetically using the join construction. The key property is that the fibers are circles: $\text{fib}_\eta(x) \simeq S^1$ for each $x : S^2$. Explain, in your own words, what the Hopf fibration is geometrically (as a map from the 3-sphere to the 2-sphere where each fiber is a circle), and how Brunerie's synthetic construction captures this.

3. The EHP long exact sequence is the key technical tool: it relates $\pi_n(S^m)$, $\pi_n(S^{2m-1})$, and $\pi_n(\Sigma S^m)$. Read the section of the thesis that introduces this sequence (approximately Section 2.2). What does the EHP sequence say about $\pi_4(S^3)$ and $\pi_3(S^2)$? What additional input is needed to complete the computation?

4. The Brunerie number is an integer defined within the proof. The challenge of "computing" it is showing that it equals $\pm 2$. Read Brunerie's description of where this computation arises. Identify the specific type whose inhabitant *is* the Brunerie number. Is the Brunerie number a natural number, an integer, or an element of some more general type?

5. After reading these sections, write an outline of the proof: a list of five to ten steps, each a specific mathematical fact that needs to be established, in the order Brunerie establishes them. Annotate each step with (a) whether it has since been formalized in Cubical Agda, and (b) whether it is the kind of step that classical homotopy theory would handle differently.

*Research skills developed: Reading a long technical document selectively; understanding the structure of a large proof; identifying the key conceptual steps in a synthetic homotopy theory argument.*

---

## Exercise R.5: Mapping the Problem Space
*Domain: Research Planning*

**Setup:** One of the most useful things you can do early in a research career is to understand how open problems relate to each other: which problems must be solved before which others can be attacked, which problems require the same techniques, and which problems are genuinely independent. This exercise builds that map for the open problems of Chapter 26.

**Tasks:**

1. List the eight open problems from Section 1.1 of Chapter 26 (the Brunerie problem, canonicity for Book HoTT, general HIT syntax, directed univalence, canonicity for STT, formalization of $\pi_n(S^n)$, Blakers-Massey sharpness, stable homotopy in HoTT). For each problem, write a one-sentence statement of what it asks.

2. Draw a dependency graph: put each problem as a node, and draw an arrow from problem $A$ to problem $B$ if solving $A$ would be a significant step toward solving $B$ (not necessarily required, but helpful). For example: directed univalence (Problem 4) would likely help with canonicity for STT (Problem 5). Identify the three most connected nodes (those with the most arrows in or out).

3. For each problem, rate its accessibility on a scale: (1) accessible to a motivated student who has completed this curriculum, (2) requires graduate-level research experience, (3) major open problem requiring deep new ideas. Defend your ratings with one sentence each.

4. Find two problems that seem independent (no arrow between them in your graph) but that might actually share technical methods. Explain what the shared method would be and why working on one might generate insight for the other.

5. Choose one of the problems you rated (1) or (2). Write a one-page research plan: (a) What background beyond this curriculum do you need? (b) What papers would you read first? (c) What would a "partial result" look like? (d) What would count as success? Be specific: name specific theorems, specific library files, specific papers.

*Research skills developed: Structural thinking about a problem space; identifying entry points; planning research incrementally.*

---

## Exercise R.6: Joining the Community
*Domain: Research Community Engagement*

**Setup:** HoTT is a small, active, and welcoming community. The most effective way to accelerate your learning is to engage with it directly. This exercise guides you through that engagement in a structured way.

**Tasks:**

1. Register for the HoTT Zulip (hott.zulipchat.com). Read the last two weeks of posts in the "general" stream and the "announcements" stream. For each distinct research thread or topic that appears: (a) identify what problem or question is being discussed, (b) identify which researchers are active in the discussion, and (c) note whether the topic connects to material from this curriculum. Write a log of what you find.

2. Find a recent question in the "general" stream that you understand well enough to answer, at least partially. Draft a response (you do not have to post it). If you cannot find such a question, find one you partially understand and draft a response to the part you do understand, clearly marking what you are uncertain about. This exercise is not about performing expertise; it is about developing the habit of thinking about other people's questions.

3. Watch one HoTTEST seminar talk from 2023 or later (available at the seminar archive). After watching: (a) what was the main result, (b) what open problem does it address or make progress on, (c) what background from this curriculum was relevant, and (d) what one question would you ask the speaker?

4. Find a researcher whose recent work (2023–2025) is directly relevant to something in Chapters 20–26 of this curriculum. Read their academic webpage and the abstracts of their recent papers. Write a brief (one-paragraph) explanation of what research program they are pursuing and how it connects to the open problems in Chapter 26.

5. *Extension:* Post a question on the HoTT Zulip. It should be a genuine question — something you actually want to know, arising from your study of this curriculum or the exercises in this chapter. Follow the guidelines from Section 3.1 of Chapter 26: be specific, include context, and indicate what you already know. Report back (to yourself, in writing) what you learned from any responses.

*Research skills developed: Community navigation; identifying active researchers and their programs; developing the habit of asking questions; understanding the social structure of a research community.*
