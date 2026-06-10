# Important Figures

## Per Martin-Löf (1942–present)
*Logician and philosopher of mathematics; creator of Martin-Löf Type Theory (MLTT), including the identity type, the J elimination rule, and the full dependent type theory that underlies HoTT.*

Per Martin-Löf was born in Sweden and spent his career primarily at Stockholm University, where he is now professor emeritus. He received his doctorate in mathematical logic and worked initially in probability theory before turning to the foundations of mathematics and constructivism. His intellectual trajectory was shaped by his engagement with Brouwer's intuitionism, Curry-Howard's proposition-as-types correspondence, and Gentzen's natural deduction — synthesizing them into a coherent dependent type theory with a rich philosophical interpretation.

The identity type is Martin-Löf's answer to a fundamental question: how should equality be treated in a constructive type theory? His 1975 paper introduced the identity type $\mathsf{Id}_A(a, b)$ (written $a =_A b$ in modern notation) as an inductive type: the only constructor is $\mathsf{refl}_a : a =_A a$, and the eliminator — the J rule — says that to prove a property $C(b, p)$ for all $b : A$ and $p : a =_A b$, it suffices to prove $C(a, \mathsf{refl}_a)$. This is path induction. Martin-Löf gave a philosophical justification: $\mathsf{refl}$ is the "trivial" proof of equality, and every property of equality should follow from how it behaves on trivial equalities — an instance of his broader principle that constructive existence means canonical witnesses.

Martin-Löf's broader framework — the four forms of judgement (type formation, term introduction, term elimination, computation), the notion of a type as a problem and a term as a proof, the universe hierarchy $\mathcal{U}_0 : \mathcal{U}_1 : \cdots$, and the constructive reading of all logical connectives — forms the backbone of modern proof assistants including Agda, Coq (Gallina), and Lean. The identity type is the centerpiece: it is the only inductive type that cannot be reduced to simpler inductive types, and its eliminator is the only one that talks about elements of a type in a genuinely dependent way. Martin-Löf's philosophical writings on type theory, including "On the Meanings of the Logical Constants and the Justifications of the Logical Laws" (1983), remain important reading for understanding what the J rule is meant to mean.

---

## Thomas Streicher (1958–present)
*Category-theorist and type theorist; co-discoverer (with Martin Hofmann) of the groupoid model of MLTT, proving that UIP is independent of the theory.*

Thomas Streicher studied mathematics and computer science in Germany and has held a professorship at the Technische Universität Darmstadt since 1994. His work spans categorical logic, semantics of type theory, realizability, and constructive mathematics. He is known for his clarity of exposition and his willingness to look at foundational questions from multiple perspectives simultaneously.

Streicher's most influential contribution to the theory of identity types is the Groupoid Model, developed jointly with Martin Hofmann and announced in 1994. The key insight — initially Streicher's, by his own account — was that Martin-Löf's identity type has non-trivial structure above dimension 0. In the groupoid model, a type $A$ is interpreted as a groupoid (a category in which every morphism is invertible), a term $a : A$ as an object of the groupoid, and an identity proof $p : a = b$ as a morphism $a \to b$. Two identity proofs $p, q : a = b$ are equal only if there is a 2-cell between them (in a 2-groupoid). Streicher showed that the K axiom $\prod_{A,a}\prod_{p:a=a} p = \mathsf{refl}_a$ fails in this model: the groupoid $\mathbf{B}\mathbb{Z}/2$ (the one-object groupoid with automorphism group $\mathbb{Z}/2$) has a non-trivial automorphism of its single object, giving a loop that is not equal to reflexivity. This single construction destroyed twenty years of received wisdom that UIP was "obviously true."

Streicher also made important contributions to the semantics of dependent types more broadly, including the "context comprehension" / "categories with attributes" framework for modeling dependent type theory categorically (detailed in his 1991 Habilitationsschrift "Semantics of Type Theory"), the study of universes in type theory, and realizability semantics. His work on "intensionality, extensionality, and proof irrelevance in modal type theory" explores the design space around identity types in more recent type systems.

---

## Martin Hofmann (1966–2011)
*Type theorist and computer scientist; co-creator of the groupoid model of MLTT; made fundamental contributions to the semantics of dependent types and logical frameworks.*

Martin Hofmann studied mathematics and computer science in Germany and held professorships at Edinburgh and then Munich (Ludwig-Maximilians-Universität), where he died in a mountaineering accident in 2011 at 44. He was known as a precise and creative thinker who worked at the intersection of logic, type theory, and programming language semantics.

Hofmann's joint work with Streicher on the groupoid model (1994) is described above; it is his most cited and most foundational contribution. But Hofmann also made important independent contributions to the theory of identity types. His 1995 paper "Conservativity of Equality Reflection over Intensional Type Theory" showed that adding the reflection rule for identity types (making the theory extensional) is a conservative extension in a certain sense — a result clarifying the relationship between intensional and extensional identity. His work with Thorsten Altenkirch and Giovanni Amato on "A Normalization Proof for Martin-Löf's Theory with One Universe" established normalization for a significant fragment of MLTT. The textbook chapter "Syntax and Semantics of Dependent Types" (1997) remains one of the clearest accounts of how to model Martin-Löf type theory categorically.

Hofmann's 1997 paper "Extensional Concepts in Intensional Type Theory" (his Edinburgh doctoral thesis, published as a book by Springer) is a comprehensive study of what extensional reasoning is available in intensional type theory, including the identity type. This work directly addresses the practical question of how programmers and mathematicians can use identity types for equational reasoning without having full extensionality. His influence on modern HoTT is profound: the groupoid model he developed is the direct ancestor of the simplicial set model, and his careful delineation of what the J rule can and cannot prove prefigures the detailed structural analysis in the HoTT Book.

---

## Michael Hedberg (fl. 1990s)
*Type theorist; proved Hedberg's theorem, connecting decidable equality to the uniqueness of identity proofs.*

Michael Hedberg worked at Chalmers University in Göteborg, Sweden, in the 1990s. His primary contribution to the literature is a single paper, "A Coherence Theorem for Martin-Löf's Type Theory" (*Journal of Functional Programming*, 1998), which proved what is now universally called Hedberg's theorem: if a type $A$ has decidable equality, then $A$ is an h-set (UIP holds for $A$).

The proof is ingenious and uses a technique now standard in HoTT: the "constant endofunction trick." If $A$ has decidable equality, then for each pair $a, b : A$ we can define a constant function $f_{a,b} : (a = b) \to (a = b)$ — one that ignores its input and returns a canonical path (if $a = b$) or anything (if $a \neq b$). By a general lemma, a constant function on a type makes the type a proposition: for any $p, q : a = b$, we have $p = f(p) = f(q) = q$ (using the constancy of $f$ twice and the observation that $f(p)$ and $p$ are connected by a higher path derived from the J rule). The details require care, but the technique is elegant.

Hedberg's theorem has several important consequences developed further in HoTT. First, it gives a large supply of h-sets: $\mathbb{N}$, $\mathbb{Z}$, $\mathsf{Bool}$, any finitely presented type with decidable equality — all h-sets. Second, it motivates the study of types without decidable equality, which may or may not be h-sets. Third, the constant-endofunction technique generalizes: Nicolai Kraus, Martin Escardó, Thierry Coquand, and Thorsten Altenkirch showed in 2017 that the condition "every fiber of a function has a constant endofunction" characterizes when a map is a "mere proposition" — generalizing Hedberg's theorem from equality to arbitrary maps.

---

## Peter Dybjer (1957–present)
*Computer scientist and type theorist; introduced inductive families, the general framework in which identity types live, and developed the theory of pattern matching in dependent types.*

Peter Dybjer has been a professor at Chalmers University of Technology since 1987. His research spans type theory, proof theory, category theory, and dependent type systems. He is one of the central figures in the Swedish school of constructive mathematics and proof assistants.

Dybjer's 1994 paper "Inductive Families" in *Formal Aspects of Computing* introduced the notion of an inductive family: an indexed family of types defined by a collection of constructors that can mention the index. The canonical example is the identity type $\mathsf{Id}_A : A \to A \to \mathsf{Type}$, an inductive family indexed by two elements of $A$, with a single constructor $\mathsf{refl}_a : \mathsf{Id}_A(a, a)$. Dybjer's framework gives a uniform account of the J rule as the induction principle for this family: $\mathsf{J}$ is just the recursor for the inductive family $\mathsf{Id}_A$. This makes the identity type a special case of a general pattern rather than a bespoke construction, and it clarifies why the J rule has the shape it does.

Dybjer's subsequent work on "pattern matching" in type theory (with Anton Setzer) developed the theory of allowing programs to "match" on elements of identity types — effectively using the fact that the only constructor is $\mathsf{refl}$ to simplify pattern-matching expressions. This is the basis of the `rewrite` and `with`-abstraction mechanisms in Agda. His work on setoids (types equipped with an explicit equivalence relation, used as a substitute for extensional equality) connects to the structural questions about identity types that HoTT later resolved using univalence. Dybjer's broader contributions to type theory include the development of Martin-Löf's type theory as a programming language (with Chalmers colleagues), significant foundational work on categorical semantics of type theory, and the theory of "quotient inductive-inductive types."

---

## Steve Awodey (1959–present)
*Category theorist and logician; co-discoverer (with Michael Warren) of the homotopy-theoretic interpretation of identity types; central figure in the early development of HoTT.*

Steve Awodey is a professor at Carnegie Mellon University, where he works in category theory, logic, and philosophy of mathematics. His category theory textbook (*Category Theory*, Oxford University Press, 2006/2010) is one of the most popular introductions to the subject. He was one of the key organizers of the 2012–2013 IAS special year on Univalent Foundations.

Awodey and Michael Warren's 2009 paper "Homotopy Theoretic Models of Identity Types" provided a categorical framework — based on weak factorization systems and path objects — for interpreting identity types homotopy-theoretically. Their key observation: the axioms for a weak factorization system on a category correspond exactly to the structural rules for identity types (the J rule corresponds to the "path lifting property," and reflexivity corresponds to the inclusion of constant paths). This gave a clean categorical semantics that generalized the Hofmann-Streicher groupoid model to a homotopy-theoretic setting, working for any category equipped with a weak factorization system (in particular, any model category).

Awodey has also contributed to the philosophy of HoTT, arguing for "structuralism" in the philosophy of mathematics: the HoTT perspective that mathematics is about structure (types up to equivalence) rather than about sets with a fixed notion of membership. His philosophical essays on Univalent Foundations and structuralism, including "Structuralism, Invariance, and Univalence" (*Philosophia Mathematica*, 2014), have influenced how HoTT is understood and presented to non-specialists.
