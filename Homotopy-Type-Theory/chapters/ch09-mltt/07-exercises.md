# Exercises

---

**Exercise 9.1 (Path Induction from J).** Using only the J eliminator, prove the following. Give explicit terms — don't just describe the proof, write the actual term.

(a) **Symmetry:** If $p : a =_A b$, construct $p^{-1} : b =_A a$. (Hint: use motive $C(b', p') = b' =_A a$ and base $\mathsf{refl}_a$.)

(b) **Transitivity:** If $p : a =_A b$ and $q : b =_A c$, construct $p \cdot q : a =_A c$.

(c) **Inversion involution:** Prove $p^{-1}{}^{-1} = p$ for any $p : a = b$.

(d) **ap:** Given $f : A \to B$ and $p : a = b$, construct $\mathsf{ap}_f(p) : f(a) = f(b)$.

---

**Exercise 9.2 (Transport Computations).** Using the definition of transport from J:

(a) Show that $\mathsf{transport}^P(\mathsf{refl}_a, u) = u$ for any $u : P(a)$.

(b) If $P(x) = (a =_A x)$, compute $\mathsf{transport}^P(q, r)$ for $q : x_1 = x_2$ and $r : a = x_1$. (Answer should be $r \cdot q$.)

(c) If $P(x) = (x =_A c)$ for fixed $c$, compute $\mathsf{transport}^P(q, r)$ for $q : x_1 = x_2$ and $r : x_1 = c$.

(d) Show $\mathsf{transport}^P(p \cdot q) = \mathsf{transport}^P(q) \circ \mathsf{transport}^P(p)$.

---

**Exercise 9.3 (Groupoid Laws).** Prove the following groupoid laws by path induction (J). Show the motive and base case clearly.

(a) Right unit law: $p \cdot \mathsf{refl}_b = p$ for $p : a = b$.

(b) Left unit law: $\mathsf{refl}_a \cdot p = p$ for $p : a = b$.

(c) Why does the right unit law follow immediately from the computation rules, while the left unit law requires a non-trivial path induction?

---

**Exercise 9.4 (ap Properties).** Prove:

(a) $\mathsf{ap}_f(p \cdot q) = \mathsf{ap}_f(p) \cdot \mathsf{ap}_f(q)$. (Apply J to $q$.)

(b) $\mathsf{ap}_f(p^{-1}) = (\mathsf{ap}_f(p))^{-1}$.

(c) $\mathsf{ap}_g(\mathsf{ap}_f(p)) = \mathsf{ap}_{g \circ f}(p)$.

(d) $\mathsf{ap}_{\mathsf{id}_A}(p) = p$.

---

**Exercise 9.5 (Based vs. Unbased Path Induction).** 

(a) State the unbased path induction principle J'.

(b) Derive J' from J.

(c) Derive J from J'.

(d) Why do both versions produce the same theorems, even though they seem different?

---

**Exercise 9.6 (The Four Judgments).** For each of the following, identify which of the four MLTT judgments is being expressed, and whether it holds:

(a) $n : \mathbb{N} \vdash \mathsf{succ}(n) : \mathbb{N}$

(b) $\vdash (\lambda n. n + 0) = (\lambda n. n) : \mathbb{N} \to \mathbb{N}$ (definitional equality)

(c) $\vdash (\lambda n. n + 0) =_{\mathbb{N} \to \mathbb{N}} (\lambda n. n)$ (propositional equality — is this type inhabited?)

(d) $\vdash \mathsf{Vec}(A, 3)\ \mathsf{type}$ given $A : \mathsf{Type}$

(e) $n : \mathbb{N} \vdash \mathsf{Vec}(A, n) = \mathsf{Vec}(A, n)\ \mathsf{type}$ (definitional equality of types)

---

**Exercise 9.7 (UIP and Its Absence).** 

(a) Write out the statement of UIP as a type in MLTT.

(b) If UIP held, what would the identity type $p = q$ (for $p, q : a = b$) look like? (How many elements would it have?)

(c) Explain why the groupoid model of MLTT (types = groupoids, identity proofs = morphisms) provides a model where UIP fails. What is $\mathsf{base} =_{S^1} \mathsf{base}$ in the fundamental groupoid model?

(d) In Agda without `--with-K`, the following term has no type-correct definition:
```agda
uip : {A : Set} {x y : A} → (p q : x ≡ y) → p ≡ q
uip p q = {! ?!}  -- fill in the hole
```
Why does Agda's pattern matching reject any attempt to fill this hole?

---

**Exercise 9.8 (Intensional vs. Extensional).** 

(a) State the reflection rule for extensional MLTT.

(b) Show that funext (function extensionality) is provable in extensional MLTT.

(c) Explain why type checking becomes undecidable in extensional MLTT.

(d) In intensional MLTT, which of the following are provable without any additional axioms?
   - $\mathsf{transport}^P(p) : P(a) \to P(b)$ for $p : a = b$ ✓ or ✗?
   - funext: $(f \sim g) \to (f = g)$ ✓ or ✗?
   - Univalence: $(A \simeq B) \to (A = B)$ ✓ or ✗?
   - Axiom K: $p = \mathsf{refl}$ for $p : a = a$ ✓ or ✗?

---

**Exercise 9.9 (homotopies as Natural Transformations).** The naturality square for a homotopy $H : f \sim g$ (functions $A \to B$) says:

$$H(a_1) \cdot \mathsf{ap}_g(p) = \mathsf{ap}_f(p) \cdot H(a_2)$$

for $p : a_1 = a_2$.

(a) State this as a type: what is the type of the naturality proof?

(b) Prove it using path induction on $p$.

(c) In category theory, this is the naturality condition for a natural transformation. Explain the correspondence: what is $f$ and $g$ as functors, and what is $H$ as a natural transformation?

---

**Exercise 9.10 (Research: The Groupoid Interpretation).** Hofmann and Streicher (1994) constructed a *groupoid model* of intensional MLTT where UIP fails.

(a) Describe the groupoid model: what are types, what are terms, what are morphisms, and what does the identity type $a = b$ model?

(b) In this model, what is the type $\mathsf{base} =_{S^1} \mathsf{base}$ (the loop space of the circle)?

(c) Why does the groupoid model satisfy the J rule?

(d) Why does the groupoid model fail to satisfy UIP?

(e) How does Voevodsky's simplicial set model extend the groupoid model to give a model of HoTT (including Univalence)?
