# References and Primary Sources

## Foundational Texts

**L.E.J. Brouwer, *Over de Grondslagen der Wiskunde* (On the Foundations of Mathematics, doctoral dissertation, 1907).** Brouwer's foundational manifesto, in which he rejects Hilbert's formalism and the classical use of the law of excluded middle, arguing that mathematical truth is a mental construction and that logic is secondary to mathematics, not primary. The dissertation introduces intuitionism as a philosophical position and lays the groundwork for all of Brouwer's later mathematical work. A German translation appears in his collected works; partial English translations exist in various anthologies.

**Arend Heyting, *Mathematische Grundlagenforschung: Intuitionismus, Beweistheorie* (1934).** Heyting's systematic exposition of Brouwer's intuitionism in formal terms, introducing the first formal system for intuitionistic logic. This monograph is the source of what we now call *Heyting arithmetic* (intuitionistic Peano arithmetic) and the Heyting algebra structure for intuitionistic propositional logic. Heyting also articulates the proof-conditional semantics later called the BHK interpretation.

**Errett Bishop, *Foundations of Constructive Analysis* (McGraw-Hill, 1967).** Bishop's landmark book, which showed that a large portion of classical mathematical analysis — continuity, differentiation, integration, measure theory, functional analysis — can be developed constructively with no appeal to the law of excluded middle or the axiom of choice in its classical form. Bishop's work demonstrated that constructive mathematics is not a fragment of classical mathematics but a mathematically rich subject in its own right.

**Michael Dummett, *Elements of Intuitionism*, 2nd ed. (Oxford University Press, 2000).** The definitive philosophical and technical treatment of intuitionistic logic and mathematics by one of the 20th century's leading philosophers of language. Dummett provides a rigorous development of the formal system, Kripke semantics, the double-negation translation, and a defense of intuitionism as the correct philosophy of mathematics, grounded in a theory of meaning.

**Anne Sjerp Troelstra and Dirk van Dalen, *Constructivism in Mathematics: An Introduction*, 2 vols. (North-Holland, 1988).** The comprehensive reference work for constructive mathematics, covering intuitionistic logic, formal systems, realizability, Kripke semantics, constructive set theory, and the relationships between the major constructive schools (Brouwer, Bishop, Markov). An indispensable reference for the research-level treatment of topics in this chapter.

---

## Seminal Papers

**L.E.J. Brouwer, "Intuitionism and Formalism," *Bulletin of the American Mathematical Society* 20 (1913), pp. 81–96.** Brouwer's English-language introduction to his philosophical position, distinguishing intuitionism from formalism. Directly accessible and gives the reader a clear sense of Brouwer's view that mathematics is a languageless mental activity and that formal logic is derivative.

**Arend Heyting, "Die intuitionistische Grundlegung der Mathematik" (The Intuitionistic Foundations of Mathematics), *Erkenntnis* 2 (1931), pp. 91–121.** Heyting's paper presenting the first formal axiom system for intuitionistic propositional and predicate logic. The axioms include all the classical tautologies except the law of excluded middle and double negation elimination. This system is the one studied in Section 2 of this chapter.

**Andrey Kolmogorov, "Zur Deutung der intuitionistischen Logik" (On the Interpretation of Intuitionistic Logic), *Mathematische Zeitschrift* 35 (1932), pp. 58–65.** Kolmogorov gives an independent interpretation of intuitionistic logic in terms of *problems and solutions*: each proposition is a problem, and a proof is a solution to that problem. A conjunction is a pair of solutions; an implication is a procedure transforming a solution to the hypothesis into a solution to the conclusion. This is essentially the BHK interpretation but framed in computational terms, predating the Curry-Howard correspondence by decades.

**Saul Kripke, "Semantical Analysis of Intuitionistic Logic I," in: *Formal Systems and Recursive Functions* (North-Holland, 1965), pp. 92–130.** Introduces Kripke frames for intuitionistic logic: a set of "possible worlds" with a partial order (representing stages of knowledge), and a monotone forcing relation specifying which propositions hold at each world. Kripke proves soundness and completeness of intuitionistic propositional logic with respect to this semantics. This paper is the foundation for Section 3 of this chapter.

**Kurt Gödel, "Zur intuitionistischen Arithmetik und Zahlentheorie" (On Intuitionistic Arithmetic and Number Theory), *Ergebnisse eines mathematischen Kolloquiums* 4 (1933), pp. 34–38.** Gödel proves that classical propositional logic is interpretable in the intuitionistic propositional logic via the double-negation translation: $\varphi$ is classically provable if and only if $\varphi^N$ (the Gödel-Gentzen negative translation of $\varphi$) is intuitionistically provable. This is the foundation for Section 4 of this chapter.

**Gerhard Gentzen, "Über das Verhältnis zwischen intuitionistischer und klassischer Arithmetik" (On the Relationship Between Intuitionistic and Classical Arithmetic, 1933, published 1974).** Gentzen independently discovers the double-negation translation in the same year as Gödel. The paper was submitted in 1933 but not published until 1974; together with Gödel's note, it establishes the classical-intuitionistic translation as a fundamental tool.

**Per Martin-Löf, "An Intuitionistic Theory of Types: Predicative Part," in: *Logic Colloquium '73* (North-Holland, 1975), pp. 73–118.** The first published presentation of Intuitionistic Type Theory as a constructive foundation for mathematics. The system includes dependent function types ($\Pi$-types) and dependent pair types ($\Sigma$-types) corresponding to universal and existential quantifiers; identity types are introduced. This paper is the direct intellectual predecessor of HoTT.

---

## Textbooks and Modern Treatments

**Anne Sjerp Troelstra, *Metamathematical Investigation of Intuitionistic Arithmetic and Analysis* (Springer Lecture Notes in Mathematics, 1973).** The classic reference for the metamathematics of intuitionistic systems, including realizability, the disjunction property, and the existence property. More technical than Dummett's book; essential for research-level work on intuitionistic arithmetic.

**Dirk van Dalen, *Logic and Structure*, 5th ed. (Springer, 2013).** A clear undergraduate-to-graduate textbook covering both classical and intuitionistic propositional and predicate logic. The treatment of natural deduction, Kripke semantics, and the double-negation translation is thorough and well-paced. A good primary text for the material in this chapter.

**Philip Wadler, "Propositions as Types," *Communications of the ACM* 58:12 (2015), pp. 75–84.** An accessible survey of the Curry-Howard correspondence from the perspective of programming languages research. Covers the BHK interpretation, the correspondence between natural deduction and typed $\lambda$-calculus, and its implications for programming language design. Freely available.

**Thierry Coquand and Gérard Huet, "The Calculus of Constructions," *Information and Computation* 76:2–3 (1988), pp. 95–120.** Introduces the Calculus of Constructions (CoC), the type theory underlying Coq, which combines dependent types with polymorphism in a constructive system. The Coq proof assistant implements an extension of CoC and is directly relevant to Section 5 of this chapter on the computational payoff of constructive proofs.

---

## Online Resources and Lecture Notes

**Frank Pfenning and Rowan Davies, "A Judgmental Reconstruction of Modal Logic," *Mathematical Structures in Computer Science* 11 (2001), pp. 511–540.** A technically precise treatment of how to extend the BHK interpretation and natural deduction to modal logics. Useful for the Kripke semantics material in Section 3. Freely available.

**Per Martin-Löf, *Intuitionistic Type Theory* (Bibliopolis, 1984; freely available at multiple repositories).** The Padova lectures, the definitive self-contained presentation of MLTT. Short (90 pages), extremely dense, and rewarding. Every serious student of constructive foundations should read this.

**The HoTT Book, Chapters 1 and 3 (homotopytypetheory.org/book).** Chapter 1 develops the dependent type theory that is the formalization of the BHK interpretation; Chapter 3 discusses the relationship between propositions and types in the HoTT setting (the distinction between propositions as types vs. propositions as (-1)-truncated types).

**nLab, entries on "Intuitionistic Logic," "BHK Interpretation," "Kripke-Joyal Semantics," "Constructive Mathematics," and "Law of Excluded Middle."** Precise mathematical treatments with references. The entry on "Kripke-Joyal semantics" connects the Kripke semantics of Section 3 to topos theory, which is directly relevant to this curriculum's later chapters.

**Thierry Coquand, "Type Theory" (Stanford Encyclopedia of Philosophy, 2022).** An authoritative survey of type theory from a constructive perspective, covering MLTT, the Calculus of Constructions, and HoTT. Freely available at plato.stanford.edu.

---

## Historical Context

The roots of intuitionistic logic lie in a foundational dispute that erupted in the early 20th century. Brouwer's 1907 dissertation was a direct attack on the logical foundations of mathematics as practiced by Hilbert and Russell. Brouwer argued that mathematics is a *languageless* mental activity — a sequence of mental constructions — and that logic and formal systems are at best approximate descriptions of mathematical thought, not its foundation. From this view, the law of excluded middle (LEM, $P \vee \neg P$) is not a logical truth but an unjustified assumption: to assert $P \vee \neg P$, you must have decided which holds. For a finite structure, LEM may be verified by explicit check; but for propositions ranging over infinite domains (like number theory), it is an assumption without constructive warrant.

Brouwer's philosophical position was controversial and his mathematical development of intuitionism (including the *creating subject* and bar induction) was difficult to formalize. The crucial step toward a mainstream formal treatment came with Arend Heyting, Brouwer's student, who in 1930–31 wrote down the first axiom system for intuitionistic propositional and predicate logic. Heyting's axioms are precisely classical logic with LEM and double negation elimination removed. This led to the question: what is the *semantics* of this logic? The classical truth-functional semantics (propositions are true or false) is obviously inadequate. The *BHK interpretation* (propositions as problems, proofs as solutions) provided an informal semantics; Kolmogorov's 1932 paper gave an equivalent formulation. But a rigorous model theory was lacking until Kripke's 1965 paper introduced the possible-worlds semantics for intuitionistic logic.

The discovery that classical mathematics could be translated into intuitionistic mathematics via the double-negation translation (Gödel and Gentzen, 1933) defused some of the philosophical controversy: the two systems are not as different as they appear, and every classical theorem is "constructively provable" in a weakened form. The constructive movement gained new momentum with Bishop's 1967 book, which demonstrated — by developing substantial mathematics — that constructive proofs are not curiosities but productive mathematics. The connection to computation, via the Curry-Howard correspondence (Curry 1934, Howard 1969) and Martin-Löf's Type Theory (1975), made constructive logic practically important: constructive proofs are programs, and a constructive proof of a theorem produces a certified algorithm. This is the direct line to Coq, Agda, and ultimately HoTT.
