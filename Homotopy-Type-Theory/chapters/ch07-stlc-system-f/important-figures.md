# Important Figures

## Alonzo Church (1903–1995)
*American logician and mathematician who invented the lambda calculus and the simply typed lambda calculus.*

Alonzo Church spent almost his entire academic career at Princeton, where he was a central figure in the mathematical logic group that included Gödel, Turing (who was his PhD student), and Kleene. His invention of the lambda calculus in the early 1930s — introduced in a series of papers beginning in 1932 — was motivated by the search for a foundation for logic that avoided set-theoretic paradoxes. The lambda calculus formalized the notion of a computable function as an explicit rule of substitution: $\lambda x. M$ denotes the function that maps $x$ to $M$, and application $M N$ evaluates by substituting $N$ for $x$ in $M$.

The untyped lambda calculus, Church quickly showed, could encode everything: the natural numbers (Church numerals), the booleans, pairs, lists, and — crucially — the Y combinator, which implements recursion. The system was Turing-complete. But Kleene and Rosser showed in 1935 that the untyped system, when treated as a logic, was inconsistent. Church's response was the 1940 paper "A Formulation of the Simple Theory of Types," which introduced typed lambda calculus. Types in this system are built from base types and function types ($A \to B$), and the typing rules ensure that self-application ($x\, x$) and the Y combinator cannot be typed. The typed system is consistent, strongly normalizing, and corresponds — as Howard would later show — to intuitionistic propositional logic. The simply typed lambda calculus introduced in that 1940 paper, together with the Church encodings for booleans and numerals, remains the starting point for every introductory treatment of type theory.

Church also formulated Church's thesis (independently of Turing) and proved the undecidability of the halting problem (1936, independently and slightly before Turing). His PhD students include Alan Turing, Stephen Kleene, and Michael Rabin, making his intellectual lineage extraordinary.

---

## Roger Hindley (1938–present)
*British logician who proved the principal types theorem, the mathematical foundation of Hindley-Milner type inference.*

Roger Hindley worked in proof theory and combinatory logic, and his 1969 paper "The Principal Type-Scheme of an Object in Combinatory Logic" is one of the most influential results in the theory of type inference. The theorem states: every typable term in STLC (or combinatory logic) has a *principal type* — a most general type from which every other valid type can be obtained by substituting specific types for type variables. For example, the term $\lambda x. x$ has principal type $\alpha \to \alpha$, from which $\mathtt{Int} \to \mathtt{Int}$, $(\mathtt{Bool} \to \mathtt{Bool}) \to (\mathtt{Bool} \to \mathtt{Bool})$, and every other valid type can be obtained by instantiation.

The practical consequence of this theorem is that type inference is well-defined: you don't need to guess a type for a term, because there is always a unique "most general" correct answer. Hindley's proof used a unification algorithm to construct principal types, and the key step was showing that the set of valid types for any typable term has a least element under the specialization ordering. Hindley proved this theorem in the context of combinatory logic, working independently of any programming language application. It was Milner who, eight years later, developed the practical algorithm (Algorithm W) based on Hindley's theorem and deployed it in ML. The resulting system is universally called "Hindley-Milner type inference," though the two men worked independently and nearly ten years apart. Hindley has graciously acknowledged that Milner's contribution was essential for turning the mathematical result into a practical tool.

---

## Robin Milner (1934–2010)
*British computer scientist who developed ML, Algorithm W for type inference, and the pi-calculus.*

Robin Milner was one of the most influential computer scientists of the twentieth century, with fundamental contributions spanning type theory, concurrency, and programming language design. His early career was at Stanford and then Edinburgh, where he founded the Laboratory for Foundations of Computer Science and led the development of the LCF (Logic for Computable Functions) proof assistant — a pioneering system that introduced the concept of a typed meta-language (ML, for "metalanguage") for writing tactics and proofs.

The 1978 paper "A Theory of Type Polymorphism in Programming" introduced let-polymorphism and Algorithm W. The central innovation was *generalization at let bindings*: when you write `let f = e in ...`, the type of `f` is generalized to a type scheme (universally quantifying all free type variables), so `f` can be used at different types within the body. This "let-polymorphism" is more restricted than System F's full impredicative polymorphism (which is not decidably inferable), but decidable and sufficient for practical purposes. Algorithm W is a form of constraint-based type inference using unification: it collects type equations and solves them by Martelli-Montanari unification, producing the principal type. The algorithm runs in nearly-linear time in practice (though worst-case exponential for contrived inputs). ML's type system, built on this algorithm, became the paradigm for statically typed functional languages — OCaml, Standard ML, F#, and (with significant extensions) Haskell all descend from it. Later, Milner developed the theory of bisimulation (in his process calculus work), the $\pi$-calculus for mobile computing, and Bigraphs. He received the Turing Award in 1991.

---

## Jean-Yves Girard (1947–present)
*French logician who independently discovered System F, proved its strong normalization, and invented linear logic.*

Jean-Yves Girard is the most technically original logician of the last half-century, with a style that combines extraordinary technical power with philosophical ambition. His 1972 Paris thesis introduced System F — the second-order lambda calculus — and proved its strong normalization by a semantic argument using *reducibility candidates* (a generalization of Tait's reducibility method). This proof is widely regarded as a tour de force: it establishes termination for a system where types can quantify over all types (including themselves), using a careful semantic argument that sidesteps the apparent circularity.

System F's type system allows $\forall \alpha. A$: quantification over type variables. The polymorphic identity $\Lambda \alpha. \lambda x : \alpha. x$ has type $\forall \alpha. \alpha \to \alpha$. The Church numeral for $n$ has type $\forall \alpha. (\alpha \to \alpha) \to \alpha \to \alpha$. All of first-order arithmetic can be encoded. The system corresponds to second-order intuitionistic propositional logic. Girard proved that it is strongly normalizing — every well-typed term reduces to a value — which established the consistency of second-order logic by the Curry-Howard correspondence.

In the 1980s, Girard invented linear logic by analyzing the structure of proofs and identifying two modalities ($!$ and $?$) that control the "reuse" and "discarding" of hypotheses. Linear logic has had enormous impact on the theory of programming languages, resource management, proof nets, and the semantics of concurrency. Girard's later work on *Ludics* and *Geometry of Interaction* seeks a deeper (if more idiosyncratic) foundation for logic itself. His writing is famously difficult and opinionated, and his influence on proof theory and type theory is pervasive.

---

## John C. Reynolds (1935–2013)
*American computer scientist who independently discovered System F and introduced parametric polymorphism.*

John Reynolds was a professor at CMU who worked throughout his career on the mathematical foundations of programming languages. He independently discovered System F in 1974 (calling it the "second-order typed lambda calculus") in a paper titled "Towards a Theory of Type Structure." Reynolds came to System F from the programming language perspective: he was studying how types could provide strong abstraction guarantees, ensuring that internal implementation details are not visible to users of an abstract type. The universal type $\forall \alpha. A$ provides exactly this: a function of that type must work uniformly for all types $\alpha$, which means it cannot make case distinctions based on the type — the type is truly abstract.

Reynolds formalized this intuition in his 1983 paper "Types, Abstraction and Parametric Polymorphism," which introduced the *relational model* of polymorphism (each type is modeled not just as a set but as a relation) and proved the *parametricity theorem*: any well-typed polymorphic term respects all type-indexed relations. Parametricity is the rigorous version of the principle "a polymorphic function behaves uniformly": if $f : \forall \alpha. \alpha \to \alpha$, then $f$ must be the identity. If $f : \forall \alpha. [\alpha] \to [\alpha]$, then `map g . f = f . map g` — no matter what `f` does, it commutes with `map`. These are "free theorems" (Wadler's term): theorems about programs that you get for free from the types, without examining the code. Reynolds also introduced defunctionalization, continuation-passing style transformations, and the concept of idealized Algol (a clean semantics for imperative programs with local variables).

---

## Peter Landin (1930–2009)
*British computer scientist who gave the lambda calculus operational semantics and pioneered the analysis of programming languages.*

Peter Landin was one of the first people to recognize that the lambda calculus could serve as the formal model for programming languages, and his work in the 1960s established the conceptual framework for operational semantics. His 1964 paper "The Mechanical Evaluation of Expressions" introduced the SECD machine — an abstract machine with a Stack, an Environment, a Control sequence, and a Dump — that specifies how to evaluate functional programs step by step. The SECD machine was the first formal operational semantics for a higher-order language and the ancestor of all abstract machines for functional programming languages (the Krivine machine, the Zinc abstract machine underlying OCaml, the G-machine underlying Haskell).

Landin's 1966 paper "The Next 700 Programming Languages" was even more influential in terms of ideas: it introduced ISWIM ("If you See What I Mean"), a notation for functional programs built directly on the lambda calculus, and argued that the lambda calculus should serve as the core of all programming languages. ISWIM influenced every functional language that followed — Scheme, ML, Miranda, Haskell. Landin also introduced the `where` clause (now universal in Haskell), `let`-expressions, and first-class functions as default programming style. His work on `J` (a first-class jump operator) anticipated continuations and Scheme's `call/cc`. Despite his enormous influence, Landin published relatively little compared to his impact on the field, and he remained somewhat outside the mainstream of academic computer science.

---

## Philip Wadler (1956–present)
*Computer scientist whose work on free theorems, monads, and typed programming has shaped modern functional language design.*

Philip Wadler's contributions span type theory, functional programming, and programming language design. His 1989 paper "Theorems for Free!" popularized Reynolds's parametricity result for the programming languages community. By showing concretely that you can derive properties of `reverse`, `map`, and `foldr` purely from their types — without looking at the code — Wadler made parametricity accessible to practicing programmers and type theorists who were not specialists in categorical semantics. The "free theorems" became a practical tool for equational reasoning in Haskell and other polymorphic languages.

Wadler's 1992 paper "The Essence of Functional Programming" introduced monads to the Haskell community as a unifying structure for handling effects (I/O, state, exceptions, nondeterminism) in a pure functional language. This transformed the design of Haskell and influenced every subsequent purely functional language. He co-designed GJ (Generics for Java), which became the basis for Java generics, bringing parametric polymorphism to Java in a form used by billions of programmers. His 2015 CACM paper "Propositions as Types" remains the best single introduction to the Curry-Howard correspondence for a general CS audience. Wadler's skill at connecting deep theoretical results to practical applications — at seeing what the mathematics means for programming — is exemplary, and his influence on the design of Haskell and on the pedagogy of type theory is profound.
