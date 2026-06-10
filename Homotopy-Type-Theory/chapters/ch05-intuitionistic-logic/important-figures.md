# Important Figures

## L.E.J. Brouwer (1881–1966)
*Founder of intuitionism; rejected classical logic and the law of excluded middle on philosophical grounds.*

Luitzen Egbertus Jan Brouwer was born in Overschie, Netherlands, and studied mathematics at the University of Amsterdam, where he later spent his entire career. He was a mathematician of exceptional range: before his foundational work, he made major contributions to topology, including the Brouwer fixed-point theorem (every continuous map from a closed ball to itself has a fixed point), the Jordan curve theorem (for arbitrary dimension), and the proof of topological invariance of dimension. These classical results were proved early in his career using methods he later rejected as non-constructive.

Brouwer's 1907 doctoral dissertation introduced intuitionism as a foundational position: mathematics is a languageless mental activity; mathematical objects are mental constructions; a proposition is true only when a construction for it exists. From this standpoint, the law of excluded middle ($P \vee \neg P$) is not a logical axiom but an unwarranted claim about all propositions, since we have no general method of either proving or refuting an arbitrary proposition. Brouwer was prepared to follow this logic wherever it led: he rejected large portions of classical mathematics, including much of classical analysis and set theory, and developed *intuitionistic analysis* as an alternative with its own principles (notably the *continuity theorem*: every total function from Baire space to the natural numbers is continuous, contradicting classical results about discontinuous functions).

Brouwer's influence was enormous but indirect: his philosophy was difficult to formalize and his mathematical results were hard to read. It was Heyting who formalized the logic, and Bishop who showed that constructive mathematics could be developed without Brouwer's more radical principles. Nevertheless, Brouwer is the founding figure: without his philosophical challenge, there would be no intuitionistic logic, no constructive type theory, and no HoTT.

---

## Arend Heyting (1898–1980)
*Formalized intuitionistic logic; introduced Heyting algebras; made Brouwer's intuitionism mathematically accessible.*

Heyting studied under Brouwer in Amsterdam and remained a faithful interpreter of his teacher's ideas while making the crucial step of formalization that Brouwer himself resisted. He spent his career at the University of Amsterdam. He was a gentle and precise writer, and his work — unlike Brouwer's — was accessible to mathematicians without a philosophical background.

In 1930, Heyting published the first formal axiom system for intuitionistic propositional and predicate logic, presenting it at a conference on the foundations of mathematics. His 1931 paper "Die intuitionistische Grundlegung der Mathematik" (The Intuitionistic Foundations of Mathematics) is the canonical source. The axioms are classical logic with the law of excluded middle and double negation elimination removed; the resulting system is exactly the system studied in Section 2 of this chapter. Heyting also articulated the *BHK interpretation* (though he did not use this acronym): for each connective, a specification of what counts as a proof. The algebraic semantics of intuitionistic logic — *Heyting algebras* — bears his name: these are lattices with an implication operation satisfying $a \wedge b \leq c$ if and only if $a \leq b \to c$ (the adjunction), generalizing Boolean algebras by dropping the requirement that $a \vee \neg a = 1$. Every Lindenbaum algebra of intuitionistic propositional logic is a Heyting algebra, and every complete Heyting algebra gives a model of intuitionistic logic.

Heyting's formalization made it possible to ask and answer model-theoretic questions about intuitionistic logic: what are its models, when is it complete, how does it differ from classical logic? Kripke's semantics and the Gödel-Gentzen translation both presuppose the formal system Heyting defined.

---

## Andrey Kolmogorov (1903–1987)
*Gave the problem-interpretation of intuitionistic logic; foundational contributor to probability theory and computability.*

Kolmogorov is one of the towering figures of 20th-century mathematics, with foundational contributions to probability theory (his 1933 axiomatization), ergodic theory, turbulence, topology, and algorithmic information theory (Kolmogorov complexity). His work on the foundations of intuitionistic logic is less well known but independently important.

In his 1932 paper "Zur Deutung der intuitionistischen Logik" (On the Interpretation of Intuitionistic Logic), Kolmogorov proposed interpreting propositions as *problems* and proofs as *solutions*: a conjunction is a pair of solutions; a disjunction requires indicating which problem is solved; an implication is a procedure transforming a solution to the hypothesis into a solution to the conclusion. This *problem interpretation* is equivalent to the BHK interpretation and anticipates the Curry-Howard correspondence: the "procedures" in Kolmogorov's interpretation are precisely the computational procedures (functions, programs) of the $\lambda$-calculus. Kolmogorov's framing makes the computational character of constructive proofs transparent.

Kolmogorov also proved in the same paper that the double-negation of every classical tautology is intuitionistically provable — an early result pointing toward the Gödel-Gentzen translation. His work in algorithmic information theory (Kolmogorov complexity, developed independently of Solomonoff and Chaitin in the 1960s) connects to the concept of *realizability*: a proposition is realizable if there is a short program that witnesses it.

---

## Errett Bishop (1928–1983)
*Developed Bishop-style constructive mathematics; showed that substantial classical mathematics can be proved constructively.*

Bishop was an American mathematician who earned his doctorate from the University of Chicago (under Paul Halmos) and spent his career at the University of California San Diego. Before his foundational work, he made major contributions to several areas of function theory and operator algebras, using entirely classical methods.

His 1967 book *Foundations of Constructive Analysis* was a transformative event in constructive mathematics. Bishop showed — in detail, for a large swath of analysis — that classical theorems about continuous functions, integration, measure theory, and functional analysis have constructive proofs. His approach differed from Brouwer's: he used only *neutral* constructive principles (neither Brouwer's continuity theorem nor Markov's principle), making his results valid in the widest variety of constructive systems, including Martin-Löf Type Theory and (with appropriate setup) HoTT. Bishop-style constructive mathematics uses the same mathematical language as classical mathematics — real numbers, Cauchy sequences, measurable functions — but requires that all objects be explicitly constructed and all proofs be constructive.

For this chapter, Bishop's significance is in demonstrating that the Section 5 discussion of constructive schools is not merely philosophical: the mathematics works. His notion of a *set* (a collection together with an equality relation, both defined constructively) presages the setoid interpretation of types in MLTT and the notion of h-sets in HoTT. His construction of the real numbers constructively — as Cauchy sequences of rationals with an explicit modulus of convergence — is the version of real analysis constructively compatible with HoTT.

---

## Per Martin-Löf (born 1942)
*Developed Intuitionistic Type Theory (MLTT); formalized the BHK interpretation as a type-theoretic foundation; connected constructive logic to computer science.*

Martin-Löf is a Swedish logician and philosopher of mathematics who has spent his career at Stockholm University. He began his career in probability theory (the foundations of statistical testing) before turning to mathematical logic and the constructive foundations of mathematics. His Intuitionistic Type Theory is the direct ancestor of every modern proof assistant — Coq, Agda, Lean — and the foundation of HoTT.

MLTT makes the BHK interpretation precise in a formal system: propositions are types; proofs are terms; to assert $P \wedge Q$ constructively is to give a term of type $P \times Q$; to assert $\exists x : A, P(x)$ is to give a term of type $\Sigma_{x:A} P(x)$ (a dependent pair: a witness $a : A$ and a proof $P(a)$). The dependent function type $\Pi_{x:A} B(x)$ formalizes constructive universal quantification. This system, developed in a series of papers from 1971 onwards and systematized in the 1984 Bibliopolis monograph *Intuitionistic Type Theory*, is the framework in which the Curry-Howard correspondence is not an analogy but a foundation.

For this chapter specifically, Martin-Löf's 1975 paper introducing the predicative version of MLTT, and his philosophical papers on meaning-theoretic justification of logical laws (particularly his lecture "On the Meanings of the Logical Constants and the Justifications of the Logical Laws," 1983), are most relevant. He argues, following Dummett, that the introduction rules of natural deduction are the *meaning-conferring* rules — they determine what a connective means — and the elimination rules are derived by a *harmony* condition. This justifies the constructive restriction of logic on purely semantic grounds, without appeal to Brouwer's philosophy of mind.

---

## Dana Scott (born 1932)
*Developed domain theory and denotational semantics; contributed to the foundations of Kripke-Joyal semantics.*

Dana Scott was born in Berkeley, California, studied under Church at Princeton, and held positions at Oxford, Carnegie Mellon, and other institutions. He is best known in computer science for co-inventing domain theory with Christopher Strachey (the mathematical semantics of programming languages using partially ordered sets of "approximations"), and in logic for Scott-continuous functions and Scott topologies.

In the context of this chapter, Scott's most relevant contribution is his work on sheaf models and topological semantics for intuitionistic logic, which extends Kripke semantics and connects to Grothendieck toposes. A *Kripke-Joyal* or *topos-theoretic* semantics interprets intuitionistic propositions as *sieves* (or open sets) in a topological space or category; a proposition is "true at a world $w$" if it holds in an open neighborhood of $w$. This generalizes Kripke frames from partial orders to arbitrary categories and provides the foundation for sheaf models of type theory. Scott's semantic work on function types ($D_\infty$ domains solving the equation $D \cong [D \to D]$) also influenced the denotational semantics of MLTT and, through it, the categorical semantics of HoTT in terms of $\infty$-groupoids.

For Section 3 of this chapter, Scott's contribution is in the enrichment of Kripke semantics to handle the topological and categorical structure that makes semantics of dependent type theory work: the key is that the "forcing" relation of Kripke semantics is a *sheaf condition*, and the categorical logic literature on Heyting algebras as internal logics of toposes descends directly from Scott's semantic work.

---

## Michael Dummett (1925–2011)
*Provided the philosophical foundation for intuitionism; argued for anti-realism and verificationism as the basis for constructive logic.*

Dummett was a British philosopher, Professor at Oxford, and one of the leading analytic philosophers of the late 20th century. His work spanned the philosophy of language, philosophy of mathematics, metaphysics, and the history of analytic philosophy. He was also a committed social activist — his 1978 book on race relations in Britain was based on his anti-racism work.

Dummett's *Elements of Intuitionism* (1977, 2nd ed. 2000) is the definitive modern treatment of intuitionistic logic from a philosophical perspective, providing a rigorous formal development alongside a philosophical defense of the constructivist position. His philosophical argument for intuitionism does not rest on Brouwer's mentalism but on a general theory of meaning: to understand a proposition is to know what would count as a proof of it, not to know its "truth condition" in a possibly mind-independent world. This *verificationist* or *proof-conditional* semantics entails the rejection of LEM for propositions we cannot decide. Dummett connects this argument to Wittgenstein's philosophy of language and Frege's theory of sense.

For this chapter, Dummett's significance is twofold. First, his careful statement of the disjunction property and the existence property as *proof obligations* that any adequate formal system must satisfy — and his proof that intuitionistic logic satisfies them but classical logic does not — gives formal content to the philosophical position. Second, his work on *anti-realism* and the meaning of logical constants is the background against which Martin-Löf's type-theoretic semantics is best understood: both are arguing that proof, not truth, is the primary notion in mathematical semantics.
