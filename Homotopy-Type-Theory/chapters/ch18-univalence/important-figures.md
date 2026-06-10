# Important Figures

## Vladimir Voevodsky (1966–2017)
*Discoverer of the univalence axiom; creator of the univalent foundations program; Fields Medalist.*

Vladimir Voevodsky was born in Moscow in 1966 and showed exceptional mathematical talent from an early age, studying at Moscow State University before completing his PhD at Harvard in 1992 under David Kazhdan. His early career was in algebraic geometry and algebraic K-theory, where he developed the theory of motivic cohomology and the motivic cohomology operations — work for which he received the Fields Medal in 2002, one of the most prestigious recognitions in mathematics. The citation noted his proof of the Milnor conjecture (1996), which related algebraic K-theory to Galois cohomology, and his development of the $\mathbb{A}^1$-homotopy theory of algebraic varieties, a powerful new framework combining algebraic geometry with homotopy theory.

Yet in the 2000s, troubled by discovering errors in his own earlier published proofs and convinced that the mathematical community had no reliable method for catching such mistakes, Voevodsky pivoted sharply toward the foundations of mathematics and computer-verified proof. He began working with the Coq proof assistant and quickly realized that the existing foundational frameworks were poorly suited to how mathematicians actually think about mathematical structures — isomorphic objects are treated as interchangeable, but no formal system encoded this as a theorem. By around 2006 he had formulated the univalence axiom: the statement that $\mathsf{idToEquiv} : (A =_{\mathsf{Type}} B) \simeq (A \simeq B)$ is an equivalence, meaning that equality of types is the same as equivalence of types. He proved the consistency of this axiom by constructing the simplicial set model, in which types are interpreted as Kan complexes and the universe is the Kan complex of small Kan complexes. He announced this work publicly in his 2010 lecture "Univalent Foundations of Mathematics" at Carnegie Mellon University.

Voevodsky spent his remaining years at the Institute for Advanced Study, where he developed the `UniMath` library in Coq — the first large-scale formalization of mathematics based on univalent foundations — and organized the 2012–2013 IAS special year in Univalent Foundations that produced the HoTT Book. His influence on the field is hard to overstate: the univalence axiom, the program of formalizing mathematics so that equality means isomorphism, and the specific technical notion of h-levels (Chapter 17) all originate with him. He died unexpectedly in 2017 at age 51, but the foundations program he initiated continues to shape research in type theory, proof assistants, and the philosophy of mathematics.

---

## Steve Awodey (1959–present)
*Category theorist and logician; co-discoverer of the homotopy interpretation of identity types.*

Steve Awodey is a professor of philosophy at Carnegie Mellon University, where he works at the intersection of logic, category theory, and the foundations of mathematics. He is best known among a broad audience for his textbook *Category Theory* (Oxford, 2006), which became a standard reference for mathematicians and computer scientists. His technical research focuses on categorical logic — the study of logical systems through their category-theoretic semantics — and he has contributed to the theory of locally cartesian closed categories, fibered categories, and the categorical interpretation of dependent type theory.

Awodey's central contribution to the foundations of HoTT is the 2009 paper "Homotopy theoretic models of identity types," co-authored with Michael A. Warren, which established that the identity types of Martin-Löf type theory have models in groupoids and, more generally, in categories with a suitable notion of path (weak factorization systems). This was the first rigorous proof that the identity types have higher-dimensional homotopy-theoretic meaning, validating Voevodsky's intuition that the identity type $a =_A b$ should be thought of as the type of paths from $a$ to $b$. The specific construction uses the Quillen model structure on the category of groupoids, where the path object of a groupoid $G$ is the groupoid of arrows in $G$. This "groupoid model" of identity types predated and inspired the full simplicial set model of HoTT.

Awodey was one of the key organizers of the 2012–2013 IAS special year in Univalent Foundations and contributed substantially to the HoTT Book, particularly the chapters on type theory and the philosophical framing of the project. He has continued to work on the categorical semantics of HoTT, including work on natural models of type theory (a cleaner categorical framework for understanding the substitution structure of dependent type theories) and on the relationship between HoTT and elementary $\infty$-topos theory.

---

## Michael A. Warren (dates not public)
*Type theorist; co-author of the first homotopy-theoretic model of identity types.*

Michael Warren was a PhD student of Steve Awodey at Carnegie Mellon University, and his dissertation (2008) introduced the idea that identity types in Martin-Löf type theory can be modeled by the path objects in any category equipped with a weak factorization system. This observation, developed jointly with Awodey in the paper "Homotopy theoretic models of identity types" (2009), was the pivotal technical contribution that established the homotopy-theoretic interpretation of identity types on a firm footing. Warren showed that the J rule (the elimination rule for identity types) corresponds exactly to the lifting property of trivial cofibrations against fibrations in a Quillen model structure — a deep and beautiful connection between two independently developed bodies of theory.

Warren's contribution to this chapter is specifically to the foundational semantics: his work explains *why* the univalence axiom is meaningful, since it makes precise what "identity type in the universe" means in a homotopy-theoretic model. In the groupoid model, a path $A = B$ in the universe is a functor between $A$ and $B$ that is an equivalence of groupoids, matching the categorical notion of equivalence. This makes the univalence axiom semantically natural: it says that the identity type in the universe behaves as it should in the model. Warren subsequently worked on weak $\omega$-categories and higher-dimensional type theory before moving away from academic research.

---

## Chris Kapulkin (dates not public)
*Type theorist; author of the rigorous proof of the simplicial set model of univalent foundations.*

Chris Kapulkin received his PhD from the University of Western Ontario and worked at Carnegie Mellon University before joining the faculty at the University of Western Ontario, where he works on the semantics of type theory and its connections to higher category theory. His central contribution to HoTT is the paper "The simplicial model of univalent foundations (after Voevodsky)," co-authored with Peter LeFanu Lumsdaine, which appeared as a preprint in 2012 and was published in the *Journal of the European Mathematical Society* in 2021.

Voevodsky had sketched the proof that Kan complexes model univalent type theory, but the full verification required substantial technical work that he left incomplete. Kapulkin and Lumsdaine carried out this verification rigorously, establishing that there is a model of MLTT with univalence in the Quillen model structure on simplicial sets, with the universe interpreted as the Kan complex of small Kan complexes. This proof is the definitive consistency result for the univalence axiom. The technical heart involves showing that the Kan complex of equivalences $A \simeq B$ is exactly the path space between $A$ and $B$ in the universe, which is what the univalence axiom demands. Kapulkin has also worked on the $(\infty,1)$-categorical semantics of univalent type theory, clarifying the relationship between the model-categorical and $\infty$-categorical perspectives.

---

## Peter LeFanu Lumsdaine (dates not public)
*Type theorist; contributor to the simplicial model, HITs semantics, and the structural theory of identity types.*

Peter LeFanu Lumsdaine completed his PhD at Dalhousie University under Peter Selinger and Michael Makkai, and has held positions at Stockholm University and elsewhere. He is one of the most technically accomplished researchers in the semantics of dependent type theory, with contributions spanning the simplicial model, the semantics of higher inductive types, and the theory of weak $\omega$-categories. His breadth — spanning both the categorical foundations and the proof-theoretic details of type theory — makes him one of the central figures in the field.

For this chapter, Lumsdaine's contribution to the simplicial set model (with Kapulkin) is primary: his technical precision and command of both model category theory and type theory were essential to completing the proof that Voevodsky had sketched. He also contributed to understanding the *structure identity principle* (SIP): the theorem that structured types (groups, rings, categories) are equal iff they are isomorphic when formalized in univalent type theory. The SIP says that the "correct" definition of a group in HoTT is a type together with its operations and axioms packaged as a $\Sigma$-type, and that with univalence, equality in this type of groups is exactly group isomorphism. Lumsdaine has also co-authored key results on the semantics of HITs (Chapter 19) and contributed to the HoTT Book.

---

## Thierry Coquand (1961–present)
*Type theorist; creator of Coq (with Huet); inventor of cubical type theory, which makes univalence computational.*

Thierry Coquand is a professor at the University of Gothenburg and one of the most influential type theorists of the past forty years. He co-developed the Calculus of Constructions with Gérard Huet in the mid-1980s, which became the theoretical foundation for the Coq proof assistant. His work spans constructive mathematics, constructive algebra (where he proved Hilbert's basis theorem and the Hermite normal form theorem constructively), dependent type theory, and the metatheory of type theories. He received the Alonzo Church Award in 2019 for the development of Coq.

Coquand's contribution most directly relevant to this chapter is his development of cubical type theory, in the 2015 paper "Cubical Type Theory" co-authored with Cohen, Huber, and Mörtberg. Cubical type theory provides a computational interpretation of the univalence axiom: instead of adding `ua` as an axiom that simply asserts the existence of a function $A \simeq B \to A = B$ with no computational content, cubical type theory introduces the interval type $\mathbb{I}$ and the Kan operations (composition and filling), from which `ua` can be *derived* as a theorem with a concrete normal form. The key construction is the `Glue` type: $\mathsf{Glue}(A, \varphi, B, e)$ glues a type $B$ onto $A$ along the partial equivalence $e$, and from this one constructs `ua`. The result is that `ua` and `transport` along `ua` both reduce to normal forms, making univalence fully computational. This resolved a longstanding open problem — that univalence was an axiom, not a theorem — and is the basis for Cubical Agda (Chapter 22).

---

## Benedikt Ahrens (dates not public)
*Type theorist and category theorist; developer of the structure identity principle in univalent foundations.*

Benedikt Ahrens completed his PhD at the University of Nice Sophia-Antipolis and has held positions at the University of Birmingham and Delft University of Technology. His research focuses on the formal verification of mathematics and computer science using proof assistants, particularly in the UniMath library, and on the categorical semantics of type theory. He is one of the primary developers of UniMath and has contributed extensively to the formalization of category theory in a univalent setting.

Ahrens's most directly relevant contribution to this chapter is the 2015 paper "Univalent categories and the Rezk completion" (with Kapulkin and Shulman), which works out the structure identity principle for categories in detail. This paper defines *univalent categories* — categories in which the identity type between objects is equivalent to the type of isomorphisms — and shows that every category can be completed to a univalent one (the Rezk completion, defined as a HIT). This is the paradigmatic instance of the SIP: two univalent categories are equal iff they are equivalent as categories. The paper demonstrates concretely how univalence transforms the foundations of category theory, making the informal practice of treating equivalent categories as identical into a theorem. This work has been influential in developing the general theory of how mathematical structures should be formalized in HoTT to take full advantage of univalence.
