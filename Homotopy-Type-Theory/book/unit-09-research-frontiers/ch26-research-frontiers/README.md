# Chapter 26: Research Frontiers

## The Brunerie Moment

Guillaume Brunerie sat down in 2016 to compute π₄(S³). He knew the answer was ℤ/2ℤ — classical homotopy theory had established this in 1951, using the EHP long exact sequence and the Hopf invariant. The computation, by classical methods, takes a few pages once the stable machinery is assembled. Brunerie wanted to prove it in HoTT: not just assert the classical result, not just translate it, but derive it from the axioms, type-theoretically, constructively, in a proof assistant.

He succeeded. But something strange happened.

The proof required a specific integer n — a number defined by the proof term itself, not by fiat. The type theory predicted that n = ±2, but could not compute n. Not because of a bug. Not because of a missing library lemma. Because extracting the specific value of n required a computation that the proof assistant could not complete in any reasonable time: the term was too large, the reduction too deep, the computational graph too wide for a machine to traverse before exhausting memory or patience.

This is the Brunerie moment: the moment when type theory, which is supposed to be *computational* — which promises that proofs have extractable content, that every term reduces to a normal form — encountered a proof so large that its computational content was, in practice, inaccessible.

The Brunerie moment is not a failure. It is a diagnosis. It tells you exactly what kind of new mathematics is needed: proofs whose computational content is transparent, not just correct. It tells you what the field is organized around. And it tells you that the problem is open — not because nobody smart has worked on it, but because it is genuinely hard in a way that requires new ideas.

There are others.

---

## Chapter Structure

This chapter does not teach settled mathematics. Every previous chapter in this curriculum was about results that have been established, proved, formalized (or at least provable in principle). This chapter is about what has not been established, what has been proved but not made transparent, what has been formalized but not understood, and what is not yet even well-posed.

This is what frontier mathematics looks like from the inside.

**Section 01 — Open Problems** maps the specific technical questions that are currently driving research: Brunerie's number, canonicity for Book HoTT, the general syntax for HITs, directed univalence, π₅(S⁴), the sharpness of Blakers-Massey, stable homotopy theory synthetically. For each, what is known, what is not, and what a resolution would require.

**Section 02 — Formalization Frontiers** surveys the active library projects: what has been machine-verified, what gaps remain, and where a motivated reader could make a concrete contribution right now.

**Section 03 — Connections to Other Fields** follows the threads that lead outside HoTT proper: to algebraic K-theory, topological field theories, chromatic homotopy, condensed mathematics, quantum type theory, and Schreiber's M-theory program. Some of these connections are theorems; some are conjectures; some are research programs still being assembled.

**Section 04 — How to Contribute** is practical: the HoTT Zulip, the HoTTEST seminar, the arXiv, the conferences, the library issues, the research ladder from "fill a sorry in Cubical Agda" to "prove a new theorem." Specific steps, in order.

---

## A Note on Tone

This chapter is written in a different register than the preceding ones. Those chapters were written by someone who has already made the journey and is describing the terrain from above. This chapter is written by someone standing at the edge alongside you, pointing at what can be seen from here.

The open problems are real. The people working on them are named. The papers are citable. The library gaps are enumerated. This is not inspiration for its own sake — it is a map of a territory that exists, with specific features at specific coordinates, and with specific paths that lead into it from where you are now.

The Brunerie moment is both a cautionary tale and an invitation. It says: type theory is powerful enough to prove things that are genuinely hard, but not yet powerful enough to make their computational content transparent. Closing that gap is the work of the next decade. Some of that work will be done by people who learned HoTT from books like this one.

Begin.
