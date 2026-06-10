# Important Figures

## Haskell Brooks Curry (1900–1982)
*Logician and mathematician who discovered the first form of the propositions-as-types correspondence.*

Haskell Curry received his doctorate in mathematics from Göttingen in 1930, though he had been working on combinatory logic for several years before that. He was drawn to the project of formalizing the foundations of logic without set theory, seeking a purely operational account of logical inference. His primary vehicle was combinatory logic — a system of basic "combinator" functions ($S$, $K$, $I$, $B$, $C$) from which all computable functions can be built without variables or binding.

In 1934, while studying the types of these combinators, Curry noticed something unexpected: the type of $K = \lambda x y. x$, which is $A \to B \to A$, is precisely the first axiom of Hilbert-style propositional logic. The type of $S = \lambda f g x. f x (g x)$, which is $(A \to B \to C) \to (A \to B) \to A \to C$, is the second axiom. This was the first observation that the type structure of a functional system mirrors the axiom structure of a logic. Curry called this "functionality in combinatory logic" and published it as a brief note in the Proceedings of the National Academy of Sciences. He did not have the vocabulary of "propositions as types" yet — natural deduction had not been invented — but the seed was planted.

Curry's later work, particularly the two-volume *Combinatory Logic* (with Feys, 1958; with Hindley and Seldin, 1972), remained foundational for the entire field. His name is attached to the correspondence itself, to the operation of "currying" a function (converting $f(x,y)$ to $f(x)(y)$, a concept he formalized), and to the Curry-Howard-Lambek triple correspondence. He spent most of his career at Penn State and later the University of Amsterdam.

---

## William Alvin Howard (1926–present)
*Logician who articulated the full isomorphism between natural deduction proofs and typed lambda terms.*

William Howard worked in mathematical logic, with expertise in proof theory and ordinal analysis. His central contribution to what we now call the Curry-Howard correspondence was made in a 1969 manuscript, "The Formulae-as-Types Notion of Construction," which he circulated informally but did not publish until 1980, when it appeared in a Festschrift for Curry. The paper is remarkable for its precision: Howard wrote down the exact translation table between natural deduction rules and typing rules, showing that every introduction rule corresponds to a constructor for the type, every elimination rule corresponds to a destructor, and every detour reduction in proof theory corresponds to a beta reduction in the lambda calculus.

Howard went further than identifying a pattern: he proved the correspondence for all connectives of intuitionistic propositional logic ($\wedge$, $\vee$, $\to$, $\bot$, $\top$) and showed that it extends to arithmetic (with the natural numbers corresponding to the induction principle). He was also the first to observe that the correspondence extends to predicate logic if you allow dependent types, anticipating the work that Martin-Löf would develop fully in the following decade. The phrase "formulae as types, proofs as programs" — often shortened to "propositions as types" or "the Curry-Howard correspondence" — captures Howard's insight. His 1969 manuscript, circulated for over a decade before publication, influenced every proof theorist and type theorist who encountered it, making it one of the most influential unpublished manuscripts in logic.

---

## Joachim Lambek (1922–2014)
*Algebraist and logician who discovered the categorical third leg of the correspondence.*

Joachim Lambek was born in Leipzig and immigrated to Canada in 1945, eventually spending his entire academic career at McGill University in Montreal. He worked in remarkably diverse areas: ring theory, module theory, categorical grammar, formal linguistics, and — most relevant here — the categorical foundations of logic and computation. His 1958 paper "The Mathematics of Sentence Structure" introduced the Lambek calculus, a type-logical grammar that anticipated much of linear logic. His series "Deductive Systems and Categories" (1969–1972) established the connection between proof theory and category theory that now bears his name.

Lambek's central insight was that natural deduction proofs in propositional logic can be organized into a category — with propositions as objects and equivalence classes of proofs as morphisms — and that this category is precisely a *cartesian closed category* (CCC). The composition of morphisms is cut elimination; the identity morphisms are the identity proofs; the function type $A \to B$ is the internal hom. Simultaneously, the simply typed lambda calculus provides the *internal language* of any CCC: terms are morphisms, types are objects, and beta-eta equality corresponds to morphism equality. The "Lambek correspondence" (or Curry-Howard-Lambek correspondence) thus identifies three different-looking mathematical structures: intuitionistic propositional logic, the simply typed lambda calculus, and cartesian closed categories. Each is a different presentation of the same underlying mathematical object. This categorical perspective has proved enormously fruitful in denotational semantics, linear logic (Lambek contributed early ideas here too), and the categorical semantics of dependent type theory.

---

## Per Martin-Löf (1942–present)
*Swedish logician and philosopher who extended the correspondence to predicate logic through dependent type theory.*

Per Martin-Löf is arguably the most important single figure in the history of dependent type theory. Trained as a mathematical logician with deep interests in the philosophy of mathematics, he sought to build a type theory that could serve as a complete foundation for constructive mathematics — one in which every proof is an explicit construction and every statement has computational content. His type theory, now called Martin-Löf Type Theory (MLTT), went through several formulations in the 1970s and 1980s; the version most commonly used today was presented in his 1980 Padova lectures, published as *Intuitionistic Type Theory* (Bibliopolis, 1984).

Martin-Löf introduced the four central judgments of type theory (A is a type; A and B are equal types; a is an element of type A; a and b are equal elements of type A), the dependent product type $\Pi(x:A).B(x)$ as a universal quantifier, the dependent sum type $\Sigma(x:A).B(x)$ as an existential quantifier, the identity type $\text{Id}_A(a,b)$ as the type of proofs that $a$ equals $b$, the universe hierarchy $U_0, U_1, U_2, \ldots$ to avoid paradox, and the inductive definition of $\mathbb{N}$, $\mathbb{B}$, and other types via their elimination rules. Each of these is an innovation; together they form a coherent foundational system. His philosophical writing — particularly "An Intuitionistic Theory of Types" (1975), "Truth of a Proposition, Evidence of a Judgement, Validity of a Proof" (1987), and "Analytic and Synthetic Judgements in Type Theory" (1994) — engages seriously with Kant, Brouwer, and the philosophy of constructivism in ways that go beyond mere technical development. The HoTT book explicitly builds on MLTT as its foundation, making Martin-Löf's framework the direct ancestor of Voevodsky's univalence axiom.

---

## Thierry Coquand (1961–present)
*French logician who developed the Calculus of Constructions and cofounded the Coq proof assistant.*

Thierry Coquand did his doctoral work under Gérard Huet at INRIA in the 1980s. His thesis introduced the *Calculus of Constructions* (CoC), a type theory that unifies System F, Martin-Löf type theory, and higher-order logic in a single elegant framework organized along the *lambda cube* — a diagram that captures all eight possible combinations of term-level and type-level abstraction. The 1988 paper "The Calculus of Constructions" (with Huet) established CoC as a coherent system with good meta-theoretic properties, including decidable type checking and strong normalization.

Coquand's CoC became the theoretical foundation for the Coq proof assistant (originally called CoC, renamed after the rooster that symbolizes France — and, homophonically, for Coquand). He also developed Coq's first implementation with Christine Paulin-Mohring. Beyond CoC, Coquand made fundamental contributions to the theory of inductive types in type theory, to cubical type theory (a computational interpretation of the univalence axiom), and to constructive set theory. His work with Marc Bezem and Simon Huber on cubical sets provided the first constructive model of HoTT's univalence axiom, a major open problem for over a decade after Voevodsky's original conjecture.

---

## Philip Wadler (1956–present)
*Computer scientist who brought the Curry-Howard correspondence to the programming language community and pioneered its applications in language design.*

Philip Wadler is a professor at Edinburgh who has worked at the intersection of type theory, functional programming, and logic throughout his career. His specific contributions to the Curry-Howard correspondence are both technical and expository. On the technical side, he applied the correspondence to derive *free theorems* — the observation (building on Reynolds's parametricity) that the type of a polymorphic function is a proposition, and that the unique way to implement it is a proof of that proposition. His 1989 paper "Theorems for Free!" showed that you can derive non-trivial program properties purely from the type signature, without examining the code. He also pioneered the use of monads in functional programming (connecting the comonad/monad structure of continuation semantics to proof-theoretic notions), and designed the language Featherweight Java.

On the expository side, Wadler's 2015 CACM article "Propositions as Types" is the single best-known introduction to the correspondence for a computer science audience. It tells the story of Curry, Howard, and Lambek with historical precision and genuine wit, introduces the "holy trinity" framing, and explains why the correspondence is deep rather than merely clever. The article won the SIGPLAN Programming Languages Software Award. Wadler's skill at communicating deep ideas accessibly — visible also in his contributions to the design of Haskell and his co-authorship of *Programming in Haskell* — has made the Curry-Howard correspondence a central part of the vocabulary of programming language theory.

---

## Gerhard Gentzen (1909–1945)
*German logician who invented natural deduction, providing the logical half of the Curry-Howard correspondence.*

Gentzen's brief career — he died in a Czech internment camp at age 35, shortly after the end of World War II — produced two of the most important systems in proof theory: natural deduction and sequent calculus. His 1935 paper "Investigations into Logical Deduction" introduced both. Natural deduction (the German *natürliches Schließen*) was designed to model how mathematicians actually reason: you make assumptions, you derive conclusions, and you eventually discharge assumptions by the rule for $\to$-introduction. Each connective has *introduction rules* (how to prove it) and *elimination rules* (how to use it). The symmetry between introduction and elimination is the formal embodiment of the idea that a logical connective is fully characterized by what it means to prove it and how to use a proof of it.

Gentzen also proved, in the same paper, the *Hauptsatz* (cut elimination theorem): every proof can be put in "normal form" without any "detours" (cuts). This is the proof-theoretic counterpart of beta reduction — precisely the identification that Howard made in 1969. Gentzen's work established the symmetries and dualities of proof theory (introduction vs. elimination, natural deduction vs. sequent calculus, cut elimination) that are still the organizing principles of the field. The $\wedge/\vee$ duality, the $\forall/\exists$ duality, and the observation that classical logic is natural deduction with the law of excluded middle as a special rule — all of these go back to Gentzen's 1935 paper.
