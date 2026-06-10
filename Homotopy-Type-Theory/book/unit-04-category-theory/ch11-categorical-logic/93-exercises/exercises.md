# Exercises: Categorical Logic

## Section 1: CCCs and STLC

**Exercise 1.** In a CCC $\mathcal{C}$:

(a) Show that $[A \times B, C] \cong [A, [B, C]]$ naturally in all variables. (This is currying at the object level.)
(b) Show that $[1, A] \cong A$ for any object $A$.
(c) Show that $[A, A] \cong [A \times A, A]^{\Delta}$ is not right — what is the correct statement?

**Exercise 2.** Interpret the following STLC terms as morphisms in a CCC. What is the explicit morphism in $\mathbf{Set}$ that interprets each term?

(a) $f : A \to B, x : A \vdash f\, x : B$
(b) $\vdash \lambda f : A \to B. \lambda g : B \to C. \lambda x : A. g\, (f\, x) : (A \to B) \to (B \to C) \to A \to C$
(c) $\vdash \lambda x : A. \lambda y : B. x : A \to B \to A$ (the $K$ combinator)

**Exercise 3.** In a CCC, prove:

(a) $(A \times B) \times C \cong A \times (B \times C)$ (associativity of products, using universal properties)
(b) $A \times 1 \cong A$ (unit law)
(c) $[A + B, C] \cong [A, C] \times [B, C]$ (in a bicartesian CCC)

**Exercise 4.** Let $\mathcal{C}$ be a CCC and $f : A \to B$ a morphism. Define the "precomposition" morphism $[f, C] : [B, C] \to [A, C]$ using the universal property of the exponential. Show this is functorial: $[\mathsf{id}_A, C] = \mathsf{id}_{[A,C]}$ and $[f \circ g, C] = [g, C] \circ [f, C]$.

**Exercise 5.** Prove the soundness of the CCC interpretation of STLC: the $\beta$-reduction $(\lambda x. t)\, s \to t[s/x]$ is interpreted as an equality of morphisms in any CCC. (Use the counit equation of the product-exponential adjunction.)

## Section 2: LCCCs and Dependent Types

**Exercise 6.** In the category $\mathbf{Set}$, show that for any function $f : A \to B$:

(a) The substitution functor $f^* : \mathbf{Set}/B \to \mathbf{Set}/A$ sends each $p : C \to B$ to the pullback of $p$ along $f$.
(b) The left adjoint $\Sigma_f : \mathbf{Set}/A \to \mathbf{Set}/B$ sends $(q : D \to A)$ to $(f \circ q : D \to B)$.
(c) The right adjoint $\Pi_f : \mathbf{Set}/A \to \mathbf{Set}/B$ sends $(q : D \to A)$ to the function $b \mapsto$ the set of sections of $q$ over $f^{-1}(b)$.

**Exercise 7.** Verify the Frobenius law in $\mathbf{Set}$: for $f : A \to B$, $C \in \mathbf{Set}/A$, and $D \in \mathbf{Set}/B$:

$$\Sigma_f(C \times f^*(D)) \cong \Sigma_f(C) \times D$$

Interpret this as the type-theoretic statement $\sum_{a:A} (C(a) \times D(f(a))) \simeq (\sum_{a:A} C(a)) \times D(f(a))$.

**Exercise 8.** The Beck-Chevalley condition: For the commutative square

$$\begin{array}{ccc} A & \xrightarrow{u} & B \\ \downarrow_v & & \downarrow_f \\ C & \xrightarrow{g} & D \end{array}$$

(a pullback), verify that $u_! \circ v^* \cong f^* \circ g_!$ (for $\Sigma$) and $u_* \circ v^* \cong f^* \circ g_*$ (for $\Pi$) in $\mathbf{Set}$, where $(-)_! = \Sigma_{(-)}$ and $(-)_* = \Pi_{(-)}$.

**Exercise 9.** In dependent type theory, the substitution lemma says: if $\Gamma, x:A \vdash B : \mathcal{U}$ and $\Gamma \vdash a : A$, then $\Gamma \vdash B[a/x] : \mathcal{U}$. Interpret this substitution categorically as a pullback in the appropriate slice category.

**Exercise 10.** The currying equivalence for dependent types says: $\Pi_{(x:A)} (B(x) \to C) \simeq ((\Sigma_{(x:A)} B(x)) \to C)$ (when $C$ does not depend on $x$). Prove this as a natural isomorphism of functors, using the adjunction $\Sigma_A \dashv \pi_A^* \dashv \Pi_A$.

## Section 3: Fibered Categories

**Exercise 11.** Let $p : \mathcal{E} \to \mathcal{B}$ be a functor. Prove that a morphism $\phi : X \to Y$ in $\mathcal{E}$ is cartesian over $f = p(\phi)$ if and only if the canonical map $\mathcal{E}(Z, X) \to \mathcal{E}(Z, Y) \times_{\mathcal{B}(p(Z), p(Y))} \mathcal{B}(p(Z), p(X))$ is a bijection for every $Z \in \mathcal{E}$.

**Exercise 12.** Show that isomorphisms in $\mathcal{E}$ are cartesian morphisms. Are cartesian morphisms always isomorphisms? Give a counterexample or prove they are.

**Exercise 13.** The *codomain fibration* $\mathsf{cod} : \mathcal{C}^\to \to \mathcal{C}$ (where $\mathcal{C}^\to$ is the arrow category of $\mathcal{C}$) sends each morphism $f : A \to B$ to its codomain $B$. Show this is a fibration if and only if $\mathcal{C}$ has pullbacks.

**Exercise 14.** For a fibration $p : \mathcal{E} \to \mathcal{B}$, define the *fiber* $\mathcal{E}_B = p^{-1}(B)$ for each $B \in \mathcal{B}$. Show that each reindexing functor $f^* : \mathcal{E}_B \to \mathcal{E}_A$ (for $f : A \to B$) is well-defined up to isomorphism, using the existence and uniqueness (up to isomorphism) of cartesian lifts.

## Section 4: Toposes

**Exercise 15.** In $\mathbf{Set}$, verify that $\Omega = \{0, 1\}$ is a subobject classifier: for every injective function $m : B \hookrightarrow A$, there is a unique characteristic function $\chi_m : A \to \{0,1\}$ such that $B = m^{-1}(\{1\})$.

**Exercise 16.** In the topos $[\mathcal{C}^{op}, \mathbf{Set}]$ of presheaves on a small category $\mathcal{C}$:

(a) Show that the subobject classifier $\Omega$ is the presheaf $\Omega(C) = \{S : S \text{ is a sieve on } C\}$ where a sieve on $C$ is a set of morphisms with codomain $C$ closed under precomposition.
(b) Compute $\Omega$ explicitly for the poset $\mathcal{C} = \{0 \to 1\}$ (two objects, one non-identity morphism).

**Exercise 17.** In any topos $\mathcal{E}$:

(a) Show that the power object $P(A) = \Omega^A$ classifies subobjects of $A$: $\mathsf{Hom}(B, P(A)) \cong \mathsf{Sub}(B \times A)$ naturally in $B$.
(b) Show that $P$ gives a contravariant functor: for $f : A \to B$, there is a morphism $P(f) : P(B) \to P(A)$ (inverse image).

**Exercise 18.** Prove that the internal logic of any topos is intuitionistic (satisfies all intuitionistic tautologies). Where does the proof of LEM break down in a non-Boolean topos? (Hint: trace through what $\phi \vee \neg \phi$ means in the internal logic, using the operations $\vee, \neg : \Omega \to \Omega$.)

## Section 5: Identity Types Categorically

**Exercise 19.** In $\mathbf{Set}$, the diagonal map $\Delta : A \to A \times A$ (sending $a$ to $(a,a)$) factors as $A \xrightarrow{r} A^I \xrightarrow{(s,t)} A \times A$ where $A^I = \{(a,b,\gamma) : \gamma : a =_A b\}$ for some appropriate "path" notion. In $\mathbf{Set}$, what is $A^I$? (Every path is a path.) What does this factorization say about identity in sets?

**Exercise 20.** In the groupoid model:

(a) What is the identity type $a =_G b$ for a groupoid $G$ with objects $a, b$?
(b) If $G = \mathbf{B}\mathbb{Z}$ (the delooping of $\mathbb{Z}$), what is the identity type of the single object $*$ with itself?
(c) Does UIP hold in this model? Show a specific pair of non-equal identity proofs.

**Exercise 21.** Prove that in the groupoid model, the $J$ eliminator holds: given a type family $P : \prod_{x,y:G} (x =_G y) \to \mathcal{U}$ and a proof $d : \prod_{a:G} P(a,a,\mathsf{id}_a)$, there is a term $J(P,d) : \prod_{x,y:G} \prod_{f:x=y} P(x,y,f)$.

(Hint: the functor $P$ assigns to each morphism $f : x \to y$ a "groupoid" $P(x,y,f)$. Define $J(P,d)(x,y,f)$ by ... the groupoid transport along $f$.)

**Exercise 22.** Prove that the Univalence Axiom fails in the set model. That is: find two types (sets) $A$ and $B$ that are isomorphic (as sets) but for which the canonical map $(A =_\mathcal{U} B) \to (A \simeq B)$ is not an equivalence.

(Hint: In the set model, $A =_\mathcal{U} B$ means literal equality of sets, which is much more restrictive than bijection.)
