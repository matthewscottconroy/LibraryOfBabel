# Exercises

---

**Exercise 7.1.** Determine whether the following terms in STLC can be typed, and if so, find their most general type. If they cannot be typed, explain which constraint fails:

(a) $\lambda f : A \to B. \lambda g : B \to C. \lambda x : A. g\, (f\, x)$

(b) $\lambda p : A \times B. (\mathsf{snd}\, p, \mathsf{fst}\, p)$

(c) $\lambda x. x\, x$ (self-application)

(d) $\lambda f : A \to A. \lambda n : \mathbb{N}. \mathsf{rec}_{\mathbb{N}}(\lambda\_. A, \text{base}, \lambda k. \lambda r. f\, r, n)$

---

**Exercise 7.2.** Reduce the following to normal form, showing each beta reduction step:

(a) $(\lambda x : A \to A. \lambda y : A. x\, (x\, y))\, (\lambda z : A. z)\, a$

(b) $\mathsf{fst}\, ((\lambda x : \mathbb{N}. (x, x))\, 3)$

(c) $\mathsf{case}\, (\mathsf{inl}\, 5 : \mathbb{N} + \mathbb{B})\, \mathsf{of}\, \mathsf{inl}(n) \Rightarrow n + 1 \mid \mathsf{inr}(b) \Rightarrow 0$

---

**Exercise 7.3.** Prove that $\lambda x. x\, x$ cannot be typed in STLC by contradiction: assume $x : A$ for some type $A$, and derive that $A$ would have to be a solution to $A = A \to B$ for some $B$. Show no such finite type $A$ exists.

---

**Exercise 7.4 (Church Encodings).** 

(a) Verify that Church booleans satisfy: $\mathsf{if}\, \mathsf{true}\, t\, f \to_\beta^* t$ and $\mathsf{if}\, \mathsf{false}\, t\, f \to_\beta^* f$.

(b) Write the Church encoding for $\mathsf{and} : \mathsf{Bool} \to \mathsf{Bool} \to \mathsf{Bool}$ and verify it gives the correct truth table.

(c) Write the Church encoding for $\mathsf{iszero} : \mathsf{Nat} \to \mathsf{Bool}$: true if $n = 0$, false otherwise.

---

**Exercise 7.5 (System F Types).** In System F, give the types of:

(a) The polymorphic swap: takes a pair and swaps components

(b) Church composition: $\forall \alpha \beta \gamma. (\beta \to \gamma) \to (\alpha \to \beta) \to (\alpha \to \gamma)$

(c) The flip function: takes $f : A \to B \to C$ and returns $f$ with arguments swapped

---

**Exercise 7.6.** The following types in System F correspond to logical propositions under the Curry-Howard correspondence. Identify the proposition and write a term inhabiting each type:

(a) $\forall \alpha. \alpha \to \alpha$

(b) $\forall \alpha. \alpha \to \alpha \to \alpha$ — what are all possible terms (up to beta-eta equivalence)?

(c) $\forall \alpha \beta. (\alpha \to \beta) \to \alpha \to \beta$ — what proposition is this?

(d) $\forall \alpha. (\alpha \to \alpha) \to \alpha \to \alpha$ — what is the connection to natural numbers?

---

**Exercise 7.7 (Parametricity).** The free theorem for $\forall \alpha \beta. (\alpha \to \beta) \to \mathsf{List}\, \alpha \to \mathsf{List}\, \beta$ says: any term of this type must behave like "map" — it applies a function to each element of a list.

(a) State the parametricity condition formally for this type.

(b) Explain why this means the function cannot "ignore" the first argument or "invent" new elements.

(c) Can such a function change the length of the list? Justify your answer from parametricity.

---

**Exercise 7.8.** In System Fω, define the type operator $\mathsf{Either} : \star \to \star \to \star$ as a type-level function. Show that $\mathsf{Either}\, A\, B$ (for $A, B : \star$) has kind $\star$.

---

**Exercise 7.9 (Lambda Cube).** The lambda cube has 8 corners corresponding to combinations of:
- $\lambda_\to$: terms depending on terms (STLC)
- $\lambda_2$: terms depending on types (System F)
- $\lambda_\omega$: types depending on types (Fω)
- $\lambda P$: types depending on terms (LF/dependent types)

(a) Draw the lambda cube and label each corner with its name and key feature.

(b) Which corner contains: Haskell's type system? Coq's core? MLTT? HoTT?

(c) Why is the "types depending on terms" corner (dependent types) the most powerful?

---

**Exercise 7.10 (Research Challenge).** Girard's paradox (1972) shows that the *impredicative* universe $\mathsf{Type} : \mathsf{Type}$ (type-in-type) is inconsistent.

(a) Look up the construction and explain the key idea: why does $\mathsf{Type} : \mathsf{Type}$ allow the construction of a term of type $\bot$?

(b) How does the stratified universe hierarchy (Chapter 8, Section 4) prevent this paradox?

(c) In Agda, try defining `bad : Set where bad = bad → bad` and observe the error. What does Agda complain about?
