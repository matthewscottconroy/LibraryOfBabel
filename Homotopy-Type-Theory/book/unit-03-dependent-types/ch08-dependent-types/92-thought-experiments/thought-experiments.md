# Thought Experiments: Chapter 8

## Experiment 1: The Typechecker as Oracle

Imagine you have a fully implemented dependent type checker for MLTT. Call it the Oracle. You can ask the Oracle any question of the form "does this term have this type?" and it will answer yes or no in finite time.

**The first surprise:** The Oracle is solving a problem that is undecidable in most systems. In STLC and System F, type-checking is decidable. In dependent type theory with intensional equality, type-checking is also decidable (though EXPTIME-hard in the worst case). In extensional MLTT (Section 06 of Chapter 9), type-checking becomes undecidable. The Oracle can exist — but only if we are careful about which variant of dependent type theory we are using.

**The second surprise:** The Oracle is also a *theorem prover*. Asking "does this term have this type?" is the same as asking "is this proposition provable?" by Curry-Howard. So the Oracle proves mathematical theorems, automatically, by type-checking.

**The thought experiment:** What would it mean to give the Oracle the type:

$$\prod_{n:\mathbb{N}} \mathsf{IsPrime}(n) + \neg\mathsf{IsPrime}(n)$$

This is the type of a decision procedure for primality. An element of this type is a function that, given any n, returns either a proof that n is prime or a proof that it is not. The Oracle cannot produce such an element automatically — that would solve primality — but it can *verify* one if you provide it.

What would the Oracle say about: Π(n:ℕ).IsEven(n) + ¬IsEven(n)? This one is easy — you can define the decision procedure by recursion. The Oracle accepts it.

What about Goldbach's conjecture? Here the Oracle is useless until someone provides a proof. The type exists. Populating it remains open.

## Experiment 2: The Universe as a Space

In HoTT, the universe Type₀ is itself a type — and therefore a space. Paths in Type₀ are, by the univalence axiom, equivalences between types.

Consider: what does the universe "look like" as a space? Each point is a type. A path from A to B is an equivalence A ≃ B. A homotopy between two paths (two equivalences from A to B) is a natural isomorphism between them.

**What is the connected component of ℕ in Type₀?** It consists of all types equivalent to ℕ — all countably infinite discrete types. Every countably infinite set (in the homotopy-theoretic sense) is in this component. The fundamental group of this component, πι(Type₀, ℕ), would be the group of self-equivalences of ℕ — which is the group of bijections ℕ → ℕ, which is the symmetric group on a countable set.

**What is the connected component of 𝟙?** Just the contractible types — types with exactly one element up to homotopy. The component of 𝟘 is the empty types — types with no element. These components do not talk to each other (there is no path from 𝟙 to 𝟘 in Type₀, because that would be an equivalence between them, impossible since 𝟙 is inhabited and 𝟘 is not).

**The thought experiment:** If you were standing inside Type₀, walking along paths from type to type, what would it look like? You would move between equivalent types by following equivalences. The landscape would be richly structured — some components have non-trivial fundamental groups, some are simply connected, some (like the component of the circle S¹) have interesting higher homotopy groups. The universe, as a space, has the topology of the "moduli space" of all homotopy types.

## Experiment 3: The Axiom of Choice and Its Friends

We proved that the Axiom of Choice is a theorem in MLTT. But this is not the classical AC.

Consider two versions:
1. AC_MLTT: Π(x:A).Σ(b:B(x)).C(x,b) → Σ(f:Π(x:A).B(x)).Π(x:A).C(x,f(x))
2. AC_class: For any family of nonempty sets, there exists a choice function.

These look similar, but AC_MLTT is a theorem while AC_class is an axiom. Why?

**The key difference:** In AC_MLTT, the hypothesis Π(x:A).Σ(b:B(x)).C(x,b) already contains explicit witnesses — for each x, a specific b(x) with C(x,b(x)). The "choice" is just extraction. There is nothing non-constructive.

In AC_class, you are told that for each x there *exists* some b, without specifying which. If the existence is proved non-constructively (by contradiction or classical excluded middle), there may be no procedure for extracting the witness.

**The thought experiment:** If you add propositional truncation (squash types), what happens? The MLTT-style AC with truncated hypotheses:

$$\|Π(x:A).Σ(b:B(x)).C(x,b)\| → Σ(f:Π(x:A).B(x)).Π(x:A).C(x,f(x))$$

This says: if we merely know (proof-irrelevantly) that witnesses exist, can we extract a choice function? This is not provable in HoTT. In fact, whether this holds is related to classical axioms. This is the genuine AC, and it requires non-constructive principles.

The distinction reveals something deep: MLTT with Σ types builds choice in by construction. The apparent "theorem" is really a definitional fact about how Σ types work, not a substantive mathematical principle. Real choice — extracting from propositional existence — is a genuine mathematical assumption.

## Experiment 4: Girard's Paradox in Slow Motion

Imagine you are building a type theory and you want to allow Type : Type. Walk through what goes wrong.

**Step 1:** You define the type U of "small types" as U = Π(X:Type).(P(P(X)) → X) → P(P(X)), where P(X) = X → Type. Since Type : Type, U : Type.

**Step 2:** You define τ : P(P(U)) → U. This requires a function that takes a "family of families" and produces an element of U. You write τ(t) = λX. λf. λp. t(U)(λx. p(f(τ(x)))).

**Step 3:** You observe that τ creates self-referential types — U mentions itself in its own definition, and τ feeds U back into itself.

**Step 4:** From this self-reference, you derive an element of the empty type, showing the system is inconsistent.

The lesson: the inconsistency arises from the ability of types to refer to themselves. Without the universe hierarchy — without separating Type₀ from Type₁ — you cannot prevent a type from appearing in its own formation rule. The hierarchy is the minimal surgical intervention that prevents this self-reference while retaining the expressive power of quantifying over types.

**The question to sit with:** Why does the hierarchy work? What property does "A : Type_i implies A is not a term in Type_i" have that prevents the paradox? The answer involves well-foundedness: the hierarchy is well-founded (no infinite descending chains), and the paradox requires an infinite regress of self-reference to generate a contradiction.

## Experiment 5: What Would Proof-Irrelevant Dependent Types Look Like?

Suppose we decided that all types would be proof-irrelevant: any two terms of the same type are automatically equal (definitionally). This is the setoid model, or the world of extensional MLTT.

**Consequence for Σ types:** Σ(x:A).P(x) would have elements (a, p) where p : P(a) — but since all p's of type P(a) are equal, the Σ type would collapse to just {a : A | P(a) holds}, a genuine subset type. The second component becomes meaningless (it's just a certificate, indistinguishable from any other certificate).

**Consequence for identity types:** a = b would be proof-irrelevant: any two proofs of a = b are equal. This is UIP — Uniqueness of Identity Proofs. The identity type would be "flat" — at most one element per pair (a, b).

**Consequence for the universe:** If the universe were proof-irrelevant, two types would be provably equal iff they are equivalent — but the equality would carry no information about *how* they are equivalent. This collapses the rich structure of equivalences to a mere equivalence relation, destroying the content of univalence.

**The point:** Proof-irrelevance is logically convenient (you do not have to track proofs), but it destroys the higher-dimensional structure that makes HoTT interesting. The decision to take proof-relevance seriously — to allow identity types to have multiple distinct elements — is the decision that opens the door to the homotopy interpretation. And HoTT, it turns out, is what you get when you follow that opening all the way.
