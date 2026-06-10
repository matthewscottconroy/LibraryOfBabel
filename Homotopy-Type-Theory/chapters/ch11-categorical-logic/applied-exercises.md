# Applied Exercises

Categorical logic is where the abstract machinery of category theory meets the concrete practice of formal reasoning. The subobject classifier $\Omega$ makes propositions into objects; the internal language makes logical deductions into morphisms; toposes model entire logical universes. The exercises below make these ideas concrete: they ask you to compute subobject classifiers in specific toposes, use the internal language to reason about a presheaf topos, encode dynamic logic as a Lawvere-Tierney topology, construct the CCC of Church encodings, and investigate what axioms hold in specific topos models.

---

## Exercise C.1: The Subobject Classifier in Sets and Presheaves
*Domain: Set Theory / Logic / Categorical Foundations*

**Setup:** In the topos $\mathbf{Set}$, the subobject classifier is $\Omega = \{\mathsf{true}, \mathsf{false}\}$: every subset $S \subseteq A$ corresponds to its characteristic function $\chi_S : A \to \{0, 1\}$, with $\chi_S(a) = 1$ iff $a \in S$. In a presheaf topos $\widehat{\mathcal{C}} = [\mathcal{C}^{op}, \mathbf{Set}]$, the subobject classifier is more interesting: $\Omega(c) = \{\text{sieves on } c\}$, where a sieve on $c$ is a collection $S$ of morphisms with codomain $c$ that is closed under precomposition (if $f : d \to c$ is in $S$ and $g : e \to d$, then $f \circ g : e \to c$ is also in $S$).

**Questions:**

1. **Sets.** Verify that $\Omega = \{0, 1\}$ in $\mathbf{Set}$ satisfies the universal property of the subobject classifier: for any injection $m : S \hookrightarrow A$, there is a unique characteristic function $\chi_m : A \to \{0, 1\}$ such that the square $S \to 1 \to \{0,1\}$ and $S \hookrightarrow A \to \{0,1\}$ is a pullback. Verify this for the inclusion $\{1,3\} \hookrightarrow \{1,2,3\}$ and the inclusion $\emptyset \hookrightarrow A$ (for any $A$). What are the characteristic functions in each case?

2. **Presheaves on the category $\bullet \to \bullet$ (the "walking arrow").** Let $\mathcal{C}$ be the category with two objects $0$ and $1$ and a single non-identity morphism $f : 0 \to 1$ (plus identities). A presheaf on $\mathcal{C}$ is a pair of sets $X_0$ and $X_1$ with a function $X_f : X_1 \to X_0$ (contravariant). Compute $\Omega$: for each object $c \in \{0, 1\}$, list all sieves on $c$ in $\mathcal{C}$. Since $c = 0$ has only the identity morphism into it (in this category), and $c = 1$ has both $\mathsf{id}_1$ and $f$, identify the sieves explicitly. Verify that $|\Omega(0)| = 2$ and $|\Omega(1)| = 3$.

3. **The characteristic morphism in $\widehat{\mathcal{C}}$.** Consider the subpresheaf $S \hookrightarrow X$ where $X = (\{a,b\}, \{a',b',c'\}, X_f)$ (with $X_f(a') = X_f(b') = a$ and $X_f(c') = b$) and $S = (\{a\}, \{a',b'\}, X_f|_{S_1})$. Compute the characteristic morphism $\chi : X \to \Omega$: for each object $c$, define $\chi_c : X_c \to \Omega(c)$ by sending each element to the sieve of morphisms $g : c' \to c$ such that $X_g(x) \in S_{c'}$. Verify that $\chi$ is natural (it commutes with the restriction maps).

4. (Extension) In the presheaf topos $\widehat{\mathcal{C}}$ for $\mathcal{C}$ the poset $\{0 \leq 1 \leq 2\}$, the subobject classifier $\Omega$ has $\Omega(i) = \{\text{upper sets of } \{j : j \leq i\}\}$ (since sieves on $i$ in a poset correspond to upper sets of the downward closure of $i$). Compute $\Omega(0), \Omega(1), \Omega(2)$ and the restriction maps. Show that $\Omega$ is a presheaf of Heyting algebras (each $\Omega(i)$ is a Heyting algebra under inclusion order) and identify the meet, join, and implication in $\Omega(2)$.

*Abstract concept illustrated: The subobject classifier $\Omega$ in $\mathbf{Set}$ and presheaf toposes; subobjects as generalizations of subsets; the Heyting algebra structure of $\Omega$.*

---

## Exercise C.2: Reasoning in the Internal Language of a Topos
*Domain: Intuitionistic Logic / Topos-Theoretic Semantics*

**Setup:** The internal language (Mitchell-Bénabou language) of a topos $\mathcal{E}$ is a higher-order intuitionistic type theory in which types are objects of $\mathcal{E}$, and propositions about $A$ are subobjects of $A$ (equivalently, morphisms $A \to \Omega$). Logical connectives are interpreted as: conjunction $P \wedge Q$ = pullback (intersection of subobjects), disjunction $P \vee Q$ = union, implication $P \Rightarrow Q$ = exponential in the Heyting algebra of subobjects, and quantifiers $\forall x:A, P$ and $\exists x:A, P$ as adjoints to substitution.

In this exercise, the topos is $\widehat{\mathcal{P}}$ — presheaves on the two-element poset $\mathcal{P} = \{0 \leq 1\}$ (the "category of stages" or "Kripke frames with two worlds"). A presheaf on $\mathcal{P}$ is a pair $(X_0, X_1, r_{01})$ with $r_{01} : X_1 \to X_0$ (the "restriction" from stage 1 to stage 0). This is the simplest nontrivial Kripke model for intuitionistic logic.

**Questions:**

1. **Interpreting propositions.** A proposition $P$ over an object $X = (X_0, X_1, r)$ is a subpresheaf $U \hookrightarrow X$, i.e., subsets $U_0 \subseteq X_0$ and $U_1 \subseteq X_1$ with $r(U_1) \subseteq U_0$. Interpret $P \wedge Q$ and $P \vee Q$ for two subpresheaves $P, Q \hookrightarrow X$. Show that the intersection $(P \wedge Q)_i = P_i \cap Q_i$ is the categorical pullback, and that the union $(P \vee Q)_i = P_i \cup Q_i$ satisfies the universal property of pushout of subobjects.

2. **Intuitionistic implication.** For $P, Q \hookrightarrow X$, the internal implication $P \Rightarrow Q$ is defined stagewide: $(P \Rightarrow Q)_i = \{x \in X_i : \forall j \geq i, \forall y \in X_j \text{ with } r^{j \to i}(y) = x, y \in P_j \Rightarrow y \in Q_j\}$ (Kripke semantics). Compute $(P \Rightarrow Q)_0$ and $(P \Rightarrow Q)_1$ for specific $P$ and $Q$ over $X = (\{a,b\}, \{a',b'\}, r)$ where $r(a') = r(b') = a$ and $P = (\{a\}, \{a'\})$, $Q = (\{a,b\}, \{a'\})$.

3. **Failure of LEM.** The law of excluded middle $P \vee \neg P = \mathsf{true}$ fails in $\widehat{\mathcal{P}}$ for non-trivial $P$. Using the $P$ from Question 2, compute $\neg P = (P \Rightarrow \bot)$ (where $\bot = (\emptyset, \emptyset)$) and then $P \vee \neg P$. Show that $(P \vee \neg P)_0 \neq X_0$, demonstrating that LEM fails. What is the stage at which LEM first fails, and what is the intuitive meaning in terms of "what is known at stage 0 vs. stage 1"?

4. (Extension) The double negation operator $\neg\neg$ defines a Lawvere-Tierney topology $j = \neg\neg$ on $\Omega$: verify that $j$ satisfies $j \circ \mathsf{true} = \mathsf{true}$, $j \circ j = j$, and $j(p \wedge q) = j(p) \wedge j(q)$. Show that the $j$-sheaves in $\widehat{\mathcal{P}}$ (presheaves satisfying $r_{01}(X_1) = X_0$, i.e., every stage-0 element is the restriction of some stage-1 element) form a subtopos where LEM *does* hold. This is the Boolean topos of "dense" presheaves.

*Abstract concept illustrated: The internal language of a topos; Kripke semantics as a presheaf model; failure of LEM; Lawvere-Tierney topologies and their sheaves.*

---

## Exercise C.3: Church Encodings in a Cartesian Closed Category
*Domain: Programming Language Theory / Type Theory*

**Setup:** In the simply typed lambda calculus (STLC), every inductive type can be encoded using only function types — the Church encoding. The natural number $n$ is encoded as the type-$\alpha$ term $\lambda f. \lambda z. f^n(z) : (\alpha \to \alpha) \to \alpha \to \alpha$ (the $n$-fold iteration of $f$). These encodings work in any CCC: the Church numeral for $n$ is a morphism from $1$ (the terminal object) to the exponential $((A \Rightarrow A) \Rightarrow (A \Rightarrow A))$.

**Questions:**

1. **Church numerals in a CCC.** Let $\mathcal{E}$ be any CCC with objects $A$ and terminal object $1$. Define the "Church numeral" for $n$ as the morphism $\ulcorner n \urcorner : 1 \to (A \Rightarrow A) \Rightarrow (A \Rightarrow A)$ corresponding, under the CCC isomorphism $\mathsf{Hom}(X, [A,B]) \cong \mathsf{Hom}(X \times A, B)$, to the $n$-fold iterate: $\text{ev}^{(n)} : ((A \Rightarrow A) \times A) \to A$ where $\text{ev}^{(0)} = \pi_2$ (projection) and $\text{ev}^{(n+1)} = \text{ev} \circ (\text{id} \times \text{ev}^{(n)})$. Work out $\ulcorner 0 \urcorner$, $\ulcorner 1 \urcorner$, and $\ulcorner 2 \urcorner$ explicitly in terms of CCC operations.

2. **Successor and addition.** Define successor $\mathsf{succ} : [A \Rightarrow A] \Rightarrow ([A \Rightarrow A] \Rightarrow A \Rightarrow A)$ by $\mathsf{succ}(n) = \lambda f. \lambda z. f(n\, f\, z)$ — applying $f$ one more time. Work out this definition as a morphism in a CCC (using the CCC isomorphisms for currying and uncurrying). Similarly, define addition $\mathsf{add}(m)(n) = \lambda f. \lambda z. m\, f\, (n\, f\, z)$ and verify that, in $\mathbf{Set}$, this corresponds to actual addition of natural numbers.

3. **Church booleans.** Define $\mathsf{true} = \lambda x. \lambda y. x$ and $\mathsf{false} = \lambda x. \lambda y. y$ as morphisms in a CCC. Show that the "if-then-else" function $\mathsf{ite} = \lambda b. \lambda x. \lambda y. b\, x\, y$ satisfies $\mathsf{ite}\, \mathsf{true}\, a\, b = a$ and $\mathsf{ite}\, \mathsf{false}\, a\, b = b$ as *definitional* equalities in the CCC (via $\beta$-reduction, which is the composition law). Define $\mathsf{and} : B \times B \to B$ in terms of $\mathsf{ite}$ and verify the truth table.

4. (Extension) The CCC / STLC correspondence says that a closed type in STLC is an object in the free CCC, and a closed term of that type is a morphism from the terminal object. Show that the type $\mathsf{Nat} = \forall \alpha. (\alpha \to \alpha) \to \alpha \to \alpha$ (in System F) is the "natural numbers object" of the free bicartesian closed category (a CCC with coproducts): the Church numeral $\ulcorner n \urcorner$ is the unique morphism from $1$ through which the $n$-fold iteration factors. In what sense does this fail in a plain CCC without initial algebras? (Hint: the Church numerals can represent all finite natural numbers but may not represent an actual natural numbers object in every CCC.)

*Abstract concept illustrated: CCCs model STLC; the CCC isomorphism $\mathsf{Hom}(C \times A, B) \cong \mathsf{Hom}(C, [A,B])$ is currying; the internal language of a CCC.*

---

## Exercise C.4: Encoding Dynamic Logic as a Lawvere-Tierney Topology
*Domain: Modal Logic / Programming Language Semantics*

**Setup:** Dynamic logic (a modal logic for programs) has a modality $[a]\varphi$ ("after every execution of program $a$, formula $\varphi$ holds") and $\langle a \rangle \varphi$ ("there exists an execution of $a$ after which $\varphi$ holds"). In categorical terms, program semantics is usually given by a monad $T$ (for nondeterministic programs) or a functor on a category of states, and the modalities correspond to operations on subobjects.

A Lawvere-Tierney topology $j : \Omega \to \Omega$ adds a modality to the internal logic of a topos: $j$-closed subobjects model propositions that are "stable" under some operation. This exercise asks you to model a simple dynamic logic using a Lawvere-Tierney topology on the presheaf topos $\widehat{\mathcal{P}}$ over a poset of "computation stages."

**Questions:**

1. **States and transitions.** Let $S = \{s_0, s_1, s_2\}$ be a set of program states. Define the category $\mathcal{T}$ with objects $S$ and a morphism $s \to t$ for each transition (say: $s_0 \to s_1$, $s_0 \to s_2$, $s_1 \to s_2$, and all identities). A presheaf $F : \mathcal{T}^{op} \to \mathbf{Set}$ assigns to each state a set of "local propositions" that hold at that state, with restriction maps going backward along transitions. Define a concrete presheaf $F$ where $F(s_i)$ is the set of atomic propositions holding at state $s_i$ (e.g., $F(s_0) = \{p, q\}$, $F(s_1) = \{p\}$, $F(s_2) = \{q\}$), and write down the restriction maps.

2. **The "always after" topology.** Define a Lawvere-Tierney topology $j$ on $\widehat{\mathcal{T}}$ corresponding to the "eventually holds" modality: $j(U)_s = \{x \in F(s) : \text{for all } t \text{ reachable from } s, x|_t \in U_t\}$ (i.e., $U$ holds at $x$ now and at all future states). Verify that $j$ satisfies the three Lawvere-Tierney axioms: (a) $j \circ \mathsf{true} = \mathsf{true}$, (b) $j \circ j = j$ (idempotency), (c) $j(P \wedge Q) = j(P) \wedge j(Q)$.

3. **Sheaves for the topology.** A $j$-sheaf for the above topology is a presheaf $F$ such that: whenever a proposition $U$ satisfies $j(U) = F$ (i.e., $U$ is "dense" in $F$), then $U = F$ itself. Show that in the context of dynamic logic, the $j$-sheaves are exactly the presheaves where every proposition that is "stable under all future transitions" is already a global proposition. Describe what the $j$-closed propositions are, and show that the sheaf subtopos models a modal logic with the $[a]$-modality for all transitions.

4. (Extension) The Kripke semantics for the modal logic $K$ (the minimal normal modal logic) uses a set of worlds $W$ with an accessibility relation $R \subseteq W \times W$, with $\square\varphi$ holding at $w$ iff $\varphi$ holds at all $w' \in W$ with $w R w'$. Show that Kripke frames for $K$ correspond to presheaves on the category associated to $(W, R)$ (with one object per world and one morphism $w \to w'$ whenever $w R w'$), and that the modality $\square$ corresponds to the right adjoint of the "substitution functor" induced by the "next state" projection. Identify the corresponding Lawvere-Tierney topology.

*Abstract concept illustrated: Lawvere-Tierney topologies; sheaves as models of modal logic; the internal logic of a subtopos; geometric morphisms and logical modalities.*

---

## Exercise C.5: Models of Type Theory in Toposes
*Domain: Foundations of Mathematics / Categorical Semantics*

**Setup:** Different toposes model different logical principles. In $\mathbf{Set}$, both LEM (Law of Excluded Middle) and AC (Axiom of Choice) hold internally. In the presheaf topos $\widehat{\mathcal{P}}$ (presheaves on the two-element poset), LEM fails but AC can hold. In the effective topos, Church's thesis holds (every function is computable). In Voevodsky's simplicial set model, Univalence holds.

This exercise asks you to determine which logical axioms hold in specific models, and to understand how the model choice affects what is provable.

**Questions:**

1. **LEM in $\mathbf{Set}$ vs. $\widehat{\mathcal{P}}$.** The proposition $\neg\neg P \Rightarrow P$ (double negation elimination, equivalent to LEM over an elementary topos) holds internally in $\mathbf{Set}$ because $\Omega = \{0, 1\}$ is Boolean. Show it fails in $\widehat{\mathcal{P}}$: find an explicit subpresheaf $P$ of some $X$ such that the double negation $\neg\neg P$ strictly contains $P$ (i.e., there is an element in $(\neg\neg P)_i$ that is not in $P_i$ for some stage $i$). Interpret this in terms of Kripke semantics: what does it mean for a proposition to be "doubly-negation-closed"?

2. **Markov's principle.** Markov's principle states: if a decidable predicate $P : \mathbb{N} \to \{0,1\}$ is not uniformly zero (i.e., $\neg(\forall n, P(n) = 0)$), then there exists an $n$ with $P(n) = 1$. This is a constructively non-trivial principle: it holds in the effective topos (every such decision procedure is computable, and if it's not uniformly zero, a search will terminate) but fails in the presheaf topos $\widehat{\mathcal{P}}$. Sketch why it holds in the effective topos (in terms of realizability) and fails in $\widehat{\mathcal{P}}$ (in terms of Kripke semantics).

3. **UIP in the groupoid model.** Hofmann and Streicher's *groupoid model* interprets types as groupoids (categories with all morphisms invertible) and identity proofs as groupoid morphisms. In this model, UIP (Uniqueness of Identity Proofs: all elements of $a =_A b$ are equal) fails: a type $A$ can have two distinct groupoid morphisms $p, q : a \to b$, and the statement $p = q$ is not provable. Construct an explicit groupoid $G$ (say, $\mathbb{Z}_2$ as a one-object groupoid) and an element $p : * =_G *$ (the non-trivial automorphism) and $q = \mathsf{id}$ such that $p \neq q$. Explain what this says about the unprovability of UIP in MLTT.

4. (Extension) The *Diaconescu theorem* says: in the internal logic of any elementary topos, the Axiom of Choice (every surjection splits) implies LEM. Sketch the proof: given a proposition $P$ (a subobject of $1$), form the two-element quotient set $Q = \{0,1\}$, use AC to get a choice function for the family parameterized by $P$, and derive $P \vee \neg P$. This theorem explains why constructive mathematics (which rejects LEM) must also reject unrestricted AC, and why the "type-theoretic AC" (provable in MLTT without axioms — see ch09 Exercise A.5) does not violate this: it is a weaker form that does not imply LEM.

*Abstract concept illustrated: Different toposes model different logical principles; UIP and its failure; the Diaconescu theorem; the model-dependence of type-theoretic axioms.*

---

## Exercise C.6: The CCC Semantics of the Simply Typed Lambda Calculus
*Domain: Programming Language Theory / Denotational Semantics*

**Setup:** The Lambek-Scott theorem states that the free CCC generated by a set $B$ of base types has, as its morphisms $A \to B$, the $\beta\eta$-equivalence classes of STLC terms of type $B$ in a context of type $A$. This gives a fully abstract denotational semantics for STLC: every program is interpreted as a morphism in a CCC, and two programs are equal as programs iff they are equal as morphisms.

**Questions:**

1. **Interpreting STLC in $\mathbf{Set}$.** Let $\llbracket - \rrbracket : \mathsf{STLC} \to \mathbf{Set}$ be the interpretation where base types are interpreted as sets (say $\llbracket \mathsf{Nat} \rrbracket = \mathbb{N}$, $\llbracket \mathsf{Bool} \rrbracket = \{0, 1\}$), function types as function sets ($\llbracket A \to B \rrbracket = \mathbf{Set}(\llbracket A \rrbracket, \llbracket B \rrbracket)$), and terms as functions. Interpret the terms $\mathsf{not} : \mathsf{Bool} \to \mathsf{Bool}$, $\mathsf{and} : \mathsf{Bool} \times \mathsf{Bool} \to \mathsf{Bool}$, and $\mathsf{add} : \mathsf{Nat} \times \mathsf{Nat} \to \mathsf{Nat}$ as set-functions. Verify that $\beta$-reduction corresponds to function application: $\llbracket (\lambda x. t)\, a \rrbracket = \llbracket t[a/x] \rrbracket$.

2. **The CCC isomorphism and currying.** In any CCC, the adjunction $(-) \times A \dashv (-)^A$ gives an isomorphism $\mathsf{Hom}(C \times A, B) \cong \mathsf{Hom}(C, B^A)$. In $\mathbf{Set}$, this is the bijection between functions $C \times A \to B$ and functions $C \to (A \to B)$ — currying. In STLC, this corresponds to the fact that a term $\Gamma, x : A \vdash t : B$ is the same as a term $\Gamma \vdash \lambda x. t : A \to B$ (lambda abstraction). Show that the CCC isomorphism is exactly the $\lambda$-abstraction rule: write out the bijection on both sides and identify each side with the corresponding typing judgment.

3. **The $\eta$-law as a CCC identity.** The $\eta$-law for functions states $\lambda x. f\, x = f$ (for $f : A \to B$). In CCC terms, this is the identity $\eta$-isomorphism of the adjunction: the counit $\epsilon : (B^A \times A) \to B$ (evaluation) satisfies $\epsilon \circ (\Lambda f \times \mathsf{id}_A) = f$ where $\Lambda$ is the currying bijection. Verify this in $\mathbf{Set}$ for a specific function $f : \mathbb{N} \to \mathbb{N}$ (say $f = \mathsf{double}$), and show that the $\eta$-law in STLC is precisely this CCC identity.

4. (Extension) In a presheaf topos $\widehat{\mathcal{C}}$, the exponential object $G^F$ for presheaves $F, G$ is computed as $G^F(c) = [\widehat{\mathcal{C}}](\mathbf{y}(c) \times F, G)$ (natural transformations from the product of the representable with $F$ to $G$). This means that $G^F$ at stage $c$ is the set of "functions from $F$ to $G$ that are natural in everything above $c$." Compute $G^F(c)$ for a small example: let $\mathcal{C} = \{0 \leq 1\}$, $F = \mathbf{y}(0)$ (the representable at $0$), and $G$ any presheaf. Show that $G^{F}$ recovers $G(1)$ at stage $1$ and $G(0) \times G(1)$ at stage $0$ (up to bijection), and interpret this as saying "at stage 0, a function from $\mathbf{y}(0)$ to $G$ is a pair of values, one for each stage."

*Abstract concept illustrated: The CCC / STLC correspondence (Lambek-Scott); currying as the CCC isomorphism; $\eta$-laws and adjunctions; exponentials in presheaf toposes.*
