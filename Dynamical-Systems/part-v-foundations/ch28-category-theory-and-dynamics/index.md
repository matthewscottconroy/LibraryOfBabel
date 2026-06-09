# Chapter 28 — Category Theory and Dynamical Systems

> *A dynamical system is an object in a category. A factor map is a morphism. Conjugacy is isomorphism. The study of dynamical systems is the study of a category — and category theory tells us what questions are meaningful.*

**Prerequisites:** Chapter 6 (topological dynamics), Chapter 7 (ergodic theory), Chapter 12 (symbolic dynamics). Some familiarity with categories helpful.

---

Category theory is often dismissed as "abstract nonsense" by those who haven't seen it in action. This chapter is an argument for the other view: that category theory is a precision tool for organizing the questions you already want to ask about dynamical systems, and that it sometimes generates new mathematics by forcing you to be precise about what "the same" means.

The central move is this: dynamical systems aren't just objects to study in isolation — they form a *category*, where the morphisms are the maps that respect the dynamics (factor maps, semiconjugacies, conjugacies). Once you have a category, you can ask: what are the products? The coproducts? The limits? How does this category relate to other categories — like Hilbert spaces, or C*-algebras? These categorical questions turn out to have dynamically meaningful answers, and the answers reveal structure you wouldn't see otherwise.

We'll see this pattern repeatedly. The Koopman functor translates dynamics into operator theory; the orbit groupoid translates dynamics into algebra; the crossed product construction translates dynamics into C*-algebras. Each translation preserves something important and discards something else, and category theory tells you precisely what is preserved and what is lost.

One more surprise: entropy itself is a functor. Leinster's 2011 result shows that Shannon entropy is uniquely characterized by being a morphism from the category of finite probability spaces (with the right notion of coarse-graining) to the nonneg reals. Entropy isn't just a formula — it's the unique functor with certain properties.

---

## Sections

- [28.1 — Categories of Dynamical Systems](categories-of-dynamical-systems.md)
- [28.2 — Functors Between Dynamics and Algebra](functors-between-dynamics-and-algebra.md)
- [28.3 — Topoi and Dynamical Systems](topoi-and-dynamical-systems.md)
- [28.4 — Categorical Entropy](categorical-entropy.md)
- [28.5 — Operator Algebras and Dynamics](operator-algebras-and-dynamics.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
