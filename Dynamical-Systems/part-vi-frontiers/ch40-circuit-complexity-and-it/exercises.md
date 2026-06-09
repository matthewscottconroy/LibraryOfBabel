# Exercises — Chapter 40

These exercises develop the key techniques: counting arguments, Fourier analysis, and the switching lemma. Exercise 40.4 requires identifying a proof that escapes the Razborov-Rudich barrier.

---

**Exercise 40.1.** (Counting Lower Bound) Show that the function $f: \{0,1\}^{20} \to \{0,1\}$ with the highest circuit complexity requires at least $c \cdot 2^{20}/20$ gates for some constant $c$. Compute the bound explicitly.

**Exercise 40.2.** (Fourier Analysis) Compute the Fourier coefficients of the majority function $\text{MAJ}_3(x_1, x_2, x_3) = 1$ iff $x_1 + x_2 + x_3 \geq 2$. What is the total influence $I(\text{MAJ}_3)$?

**Exercise 40.3.** Verify the switching lemma for a width-2 DNF with 4 clauses. Apply a random restriction with $p = 1/4$ and compute the probability that the restricted function requires a depth-2 decision tree.

**Exercise 40.4.** (Research) Identify one complexity lower bound proof that is *not* a natural proof (i.e., violates at least one of the constructivity, largeness, or usefulness conditions). Explain why it avoids the Razborov-Rudich barrier.
