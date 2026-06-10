# Chapter 26 Exercises: Research Frontiers

---

## Section 1: Open Problems

**Exercise 1.1.** Look up the current state of Brunerie's problem. The key reference is the Cubical Agda library entry for $\pi_4(S^3)$ and the 2022-2023 papers by Ljungström and Mörtberg.

1. What is the current state of the formalization in Cubical Agda?
2. Is there a computationally checkable proof? What does "computationally checkable" mean here — i.e., what computes?
3. The original Brunerie thesis defined an integer $n$ (the "Brunerie number") by the proof term. What is $n$?
4. What is the key difficulty in giving a "human-readable" proof? Where does the conceptual transparency break down?

**Exercise 1.2.** The problem of formalizing $\pi_n(S^n) = \mathbb{Z}$ for all $n$ uses the Freudenthal suspension theorem. The Freudenthal theorem is already in the Cubical Agda library (in `Cubical.Homotopy.FreudenthalSuspension`).

1. State the Freudenthal suspension theorem precisely: what is the connectivity assumption, and what is the conclusion?
2. Explain how Freudenthal gives the inductive step: if $\pi_n(S^n) = \mathbb{Z}$, how does Freudenthal give $\pi_{n+1}(S^{n+1}) \cong \pi_n(S^n)$?
3. What is the base case? (The $n = 1$ case is formalized; state it.)
4. What is the main obstacle to completing the full induction in Cubical Agda?

**Exercise 1.3.** Canonicity is the statement that every closed term of type $\mathbb{N}$ reduces to a numeral.

1. Explain why canonicity holds for cubical type theory but remains open for Book HoTT.
2. Give an example of a closed term of type $\mathbb{N}$ in Book HoTT that is "stuck" — i.e., that cannot be reduced further because of univalence.
3. What is "homotopy canonicity" (Shulman's weaker result), and how does it differ from full canonicity?
4. What would a proof of canonicity for Book HoTT require? Why is it hard?

---

## Section 2: Formalization Frontiers

**Exercise 2.1.** Browse the open issues on the Cubical Agda library (github.com/agda/cubical/issues). Filter by labels such as "wanted theorem," "enhancement," or "good first issue."

1. Identify two issues that you understand well enough (given this curriculum) to attempt. State each issue precisely.
2. For each issue, describe: what theorem needs to be proved, what library infrastructure already exists, and what the main difficulty is.
3. Estimate (informally) which one would be harder, and why.

**Exercise 2.2.** Read the sHoTT library documentation and source (rzk-lang/sHoTT on GitHub). The Yoneda lemma is formalized there.

1. Identify one result from Riehl-Shulman (2017) that is not yet in the sHoTT library. State the result precisely.
2. What Rzk syntax would be needed to state this result? (Use the Rzk syntax from Chapter 24.)
3. What is the main obstacle to formalizing this result in the current sHoTT library?

**Exercise 2.3.** The Seifert-van Kampen theorem is covered in Chapter 20 (stated in HoTT as a HIT result). Lean 4 / Mathlib does not have the HoTT version, but it does have classical covering space theory.

Outline the steps needed to formalize the Seifert-van Kampen theorem in Lean 4:
1. What definitions are needed? (Fundamental group of a type, pushout, etc.)
2. What lemmas need to be proved before the main theorem?
3. What is the main proof strategy? (Compare: encode-decode, or van Kampen via categorical universal property.)
4. Where in Mathlib would this contribution live?

**Exercise 2.4.** Algebraic K-theory assigns groups $K_n(R)$ to a ring $R$. In HoTT, $K_0(R)$ is the group completion of the monoid of isomorphism classes of finitely-generated projective $R$-modules.

1. Define $K_0(R)$ as a type in HoTT (using the group completion HIT or a direct construction).
2. Show that $K_0(\mathbb{Z}) \cong \mathbb{Z}$ (using the fact that every projective $\mathbb{Z}$-module is free).
3. What would be needed to define $K_1(R)$ in HoTT? (Hint: $K_1(R) = \pi_1(BGL(R)^+)$.)

---

## Section 3: Community Engagement

**Exercise 3.1.** Register for the HoTT Zulip (hott.zulipchat.com).

1. Browse the "general" stream and find a conversation about an open problem. Summarize the problem and the current state of the discussion in one paragraph.
2. Find a conversation where someone is stuck on a formalization question. What is the question, and what answer (if any) did they receive?
3. Identify one researcher who appears to be working on a problem related to your interests. What are they working on?

**Exercise 3.2.** Watch one talk from the HoTTEST seminar series (recordings available at homotopytype.theory or the HoTT YouTube channel).

1. State the main result of the talk.
2. What open problems does the talk mention?
3. What background (from this curriculum) do you need to understand the talk? What do you still need to learn?

---

## Section 4: Literature

**Exercise 4.1.** Choose one paper from the essential reading list (Section 3.1) that you have not yet read. Read the abstract and introduction.

1. State the main result of the paper.
2. What are the key ideas in the proof (as described in the introduction)?
3. What background is presupposed? What is the first term you encounter that you don't immediately recognize?
4. What open problems does the paper raise?

**Exercise 4.2.** The Anel-Biedermann-Finster-Joyal paper (2017) on Blakers-Massey works in an arbitrary ∞-topos rather than just in HoTT types.

1. What is an "∞-topos"? (Give an informal definition; don't worry about full precision.)
2. Why does proving Blakers-Massey in an arbitrary ∞-topos make it stronger than the HoTT version?
3. What does "working in an arbitrary ∞-topos" mean for the proof — what changes when you move from a specific type theory to a general categorical setting?

---

## Section 5: Research Projects

**Exercise 5.1 (Formalization project).** Choose one of the following formalization projects and attempt it in Cubical Agda or Lean 4. Spend at least 10 hours on it.

**Option A:** Formalize the inductive step of $\pi_n(S^n) = \mathbb{Z}$ using Freudenthal. The base case and the Freudenthal theorem are in the Cubical library; your task is to set up the induction cleanly.

**Option B:** Formalize the Mayer-Vietoris sequence for reduced homology of pushouts. The key input is the long exact sequence of a pair (partially in the Cubical library) and the excision property.

**Option C:** Define a new HIT that is not yet in the Cubical library — for example, the real projective plane $\mathbb{RP}^2$, the Moore space $M(\mathbb{Z}/2\mathbb{Z}, 1)$, or the classifying space $BG$ for a group $G$ defined as a quotient. State its elimination principle and verify one basic property.

Document:
- What worked in your formalization attempt
- What was harder than expected
- What new infrastructure (lemmas, definitions) you needed
- Whether you succeeded, and if not, what the specific obstacle was

**Exercise 5.2 (Open problem survey).** Choose one open problem from Section 1 (Chapter 26: Open Problems). Write a 3-5 page survey:

1. State the problem precisely.
2. Explain the background: what is known, what approaches have been tried, what the obstacles are.
3. Describe at least one partial result or relevant recent development.
4. State your own assessment: do you think the problem is approachable? What would be the first step?

This is the kind of writeup you would give a colleague who is new to the problem but technically sophisticated. Writing it clearly is more important than having all the answers.
