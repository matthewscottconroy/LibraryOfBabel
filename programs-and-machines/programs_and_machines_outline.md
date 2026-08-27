# Programs and Machines — Outline

**A First Course in Computer Science, in Java**

This file is the structural contract for the book. Units, chapters, sections and
lesson files are fixed here first; everything downstream refers back to it. It is
excluded from the built PDF.

## The argument

The book has one claim, made eight times in eight different registers: *a computer
does nothing but hold patterns and change them, and everything else — numbers,
text, objects, programs, meaning — is an agreement we layer on top.* Unit I
establishes the claim for data. Unit II establishes it for process. Units III–V
build the tools that make large agreements survivable. Unit VI turns the claim on
programs themselves by writing an interpreter. Units VII–VIII ask what it costs
and where it stops.

Java is the instrument throughout, not the subject. A Java feature is introduced
only at the point where the foundational idea underneath it has already been
built, so that the syntax arrives as the answer to a question the reader is
already asking.

## Audience

No prior programming is assumed. Comfort with high-school algebra is assumed.
A reader who finishes the book should find the concepts in a second-semester
Java course — arrays and ArrayList, inheritance and polymorphism, text
processing, wrapper classes, file I/O, exceptions, GUIs, recursion — already
familiar, because each will have been derived rather than announced.

## Conventions

- Directories are kebab-case. Every level carries an orienting file: `intro.md`
  for a unit, `README.md` for a chapter and for a section.
- Heading depth is produced by directory depth, with `lift_intros = true`, giving
  part → chapter (`##`) → section (`###`) → lesson (`####`) → lesson subsection
  (`#####`). Unit intros and chapter overviews are written as flowing prose
  without their own subheadings, so that `###` always means "a section".
- Every chapter carries four back-matter files: `exercises.md`,
  `further-reading.md`, `important-concepts.md`, `important-researchers.md`.
- Exercises that a later chapter depends on are marked **[carries forward]**.
- US spelling. Code is Java 17+; every program compiles and runs as printed.
- Citations are real. No invented authors, papers, years, or figures.
- Front matter (`preface.md`) marks every heading `{.unnumbered}`. Without this
  its subheadings consume the LaTeX chapter counter, and Chapter 1 prints as
  "Chapter 7" — silently invalidating every cross-reference in the book.
- Exponents are written as LaTeX math (`$2^{n}$`), never as Unicode superscript
  letters, which the PDF's serif font cannot render.
- A chapter runs 7,000–9,500 words across 12–13 files. Units I–IV average 7,800;
  do not let later units drift below that, or the back half of the book will
  feel thinner than the front.
- Back-references are necessary and easy to overdo. `, which is …` and
  `Chapter N's …` are this book's two habitual constructions; keep their
  combined density under about four per thousand words in any one file, and vary
  the phrasing rather than repeating one appositive.

## Enforcement

`tools-lint.py` parses this outline and checks the tree against it, so the
contract above is enforced rather than merely described. It also checks the
failure modes that have each cost a rebuild at least once: overview files
carrying `##` headings, Unicode superscripts, bare backslash commands in prose,
glyphs no installed font carries, inline math pandoc will not close, missing
back matter, unmarked exercise sets, out-of-range chapter references, and the
voice spec's banned words. Every check is verified by fault injection.

```bash
python3 tools-lint.py          # must print "clean"
python3 tools-fix.py           # repairs the mechanical ones
```

Run it before every commit. The heading rule above was fixed once by hand and
then reintroduced in fourteen files, which is why it is now a check rather than
an intention.

---

## Unit I — Representation

*What a machine can hold. Before a single line of Java, we settle what a bit is, why an encoding is an agreement rather than a fact, and what that agreement costs.*

`book/unit-01-representation/intro.md`

### Chapter 1. Two Voltages and an Agreement

A machine stores voltage. Meaning is something we add.

`chapter-01-two-voltages-and-an-agreement/README.md` — chapter overview

- **The Only Thing a Machine Has** — `section-01-the-only-thing-a-machine-has/README.md`
  - `01-voltage-and-meaning.md` — Voltage and Meaning
  - `02-the-bit-as-a-choice.md` — The Bit as a Choice
  - `03-why-two-and-not-ten.md` — Why Two and Not Ten
- **Encoding as Convention** — `section-02-encoding-as-convention/README.md`
  - `01-agreements-not-facts.md` — Agreements, Not Facts
  - `02-fixed-width-and-its-price.md` — Fixed Width and Its Price
  - `03-reading-a-pattern-many-ways.md` — Reading a Pattern Many Ways

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 2. Numbers That End

Positional notation, binary, and the arithmetic of a finite machine.

`chapter-02-numbers-that-end/README.md` — chapter overview

- **What a Numeral Means** — `section-01-what-a-numeral-means/README.md`
  - `01-positional-notation.md` — Positional Notation
  - `02-counting-in-two.md` — Counting in Two
  - `03-hexadecimal-as-shorthand.md` — Hexadecimal as Shorthand
- **Arithmetic in a Box** — `section-02-arithmetic-in-a-box/README.md`
  - `01-adding-bit-by-bit.md` — Adding Bit by Bit
  - `02-negative-numbers-and-twos-complement.md` — Negative Numbers and Two's Complement
  - `03-overflow-is-not-a-bug.md` — Overflow Is Not a Bug

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 3. Numbers That Do Not End

Fractions in finite space, and why 0.1 + 0.2 is not 0.3.

`chapter-03-numbers-that-do-not-end/README.md` — chapter overview

- **Fractions in Finite Space** — `section-01-fractions-in-finite-space/README.md`
  - `01-binary-fractions.md` — Binary Fractions
  - `02-the-floating-point-bargain.md` — The Floating-Point Bargain
- **Living with Approximation** — `section-02-living-with-approximation/README.md`
  - `01-why-point-one-is-not-point-one.md` — Why 0.1 Is Not 0.1
  - `02-comparing-and-accumulating.md` — Comparing and Accumulating
  - `03-when-to-refuse-floating-point.md` — When to Refuse Floating Point

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 4. Text and Other Agreements

Characters, Unicode, and the long argument about what a letter is.

`chapter-04-text-and-other-agreements/README.md` — chapter overview

- **Letters as Numbers** — `section-01-letters-as-numbers/README.md`
  - `01-ascii-and-its-assumptions.md` — ASCII and Its Assumptions
  - `02-unicode-and-code-points.md` — Unicode and Code Points
  - `03-encodings-on-the-wire.md` — Encodings on the Wire
- **Everything Else Is Also Bits** — `section-02-everything-else-is-also-bits/README.md`
  - `01-color-sound-and-images.md` — Color, Sound, and Images
  - `02-the-representation-question.md` — The Representation Question

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 5. Your First Instrument

Java arrives, not as a subject but as a tool for looking at bits.

`chapter-05-your-first-instrument/README.md` — chapter overview

- **From Source to Running** — `section-01-from-source-to-running/README.md`
  - `01-what-javac-actually-does.md` — What javac Actually Does
  - `02-the-jvm-as-a-machine.md` — The JVM as a Machine
  - `03-your-first-program.md` — Your First Program
- **Looking at the Bits** — `section-02-looking-at-the-bits/README.md`
  - `01-printing-and-observing.md` — Printing and Observing
  - `02-the-primitive-types.md` — The Primitive Types
  - `03-an-instrument-for-the-rest-of-the-book.md` — An Instrument for the Rest of the Book

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`


## Unit II — Computation

*State and the step. What it means for a machine to do one thing, then another, and how sequencing, choice, and repetition are built from that single idea.*

`book/unit-02-computation/intro.md`

### Chapter 6. What a Step Is

State, transition, and the smallest useful machine.

`chapter-06-what-a-step-is/README.md` — chapter overview

- **State and Transition** — `section-01-state-and-transition/README.md`
  - `01-a-machine-is-its-state.md` — A Machine Is Its State
  - `02-the-step-as-a-function.md` — The Step as a Function
  - `03-tables-that-compute.md` — Tables That Compute
- **Two Famous Machines** — `section-02-two-famous-machines/README.md`
  - `01-the-turing-machine.md` — The Turing Machine
  - `02-the-stored-program-idea.md` — The Stored-Program Idea
  - `03-where-java-sits.md` — Where Java Sits

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 7. Names and Boxes

Variables, assignment, and the difference between a name and a value.

`chapter-07-names-and-boxes/README.md` — chapter overview

- **Naming a Value** — `section-01-naming-a-value/README.md`
  - `01-what-a-variable-is.md` — What a Variable Is
  - `02-assignment-is-not-equality.md` — Assignment Is Not Equality
  - `03-types-as-promises.md` — Types as Promises
- **Where a Name Lives** — `section-02-where-a-name-lives/README.md`
  - `01-scope-and-lifetime.md` — Scope and Lifetime
  - `02-mutation-and-its-hazards.md` — Mutation and Its Hazards

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 8. Choosing

Boolean logic, from truth tables to gates to if-statements.

`chapter-08-choosing/README.md` — chapter overview

- **The Logic Underneath** — `section-01-the-logic-underneath/README.md`
  - `01-truth-tables.md` — Truth Tables
  - `02-gates-and-circuits.md` — Gates and Circuits
  - `03-boolean-algebra.md` — Boolean Algebra
- **Choice in a Program** — `section-02-choice-in-a-program/README.md`
  - `01-if-and-else.md` — if and else
  - `02-short-circuit-evaluation.md` — Short-Circuit Evaluation
  - `03-switch-and-dispatch.md` — switch and Dispatch

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 9. Repeating

Loops as machines, and the invariant as the thing you actually prove.

`chapter-09-repeating/README.md` — chapter overview

- **The Shape of a Loop** — `section-01-the-shape-of-a-loop/README.md`
  - `01-while-as-the-primitive.md` — while as the Primitive
  - `02-for-as-an-abbreviation.md` — for as an Abbreviation
  - `03-nested-loops.md` — Nested Loops
- **What a Loop Promises** — `section-02-what-a-loop-promises/README.md`
  - `01-the-loop-invariant.md` — The Loop Invariant
  - `02-termination.md` — Termination
  - `03-off-by-one-as-a-failed-proof.md` — Off-by-One as a Failed Proof

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 10. Reading a Program's Mind

Tracing, state tables, and debugging as disciplined observation.

`chapter-10-reading-a-programs-mind/README.md` — chapter overview

- **Tracing by Hand** — `section-01-tracing-by-hand/README.md`
  - `01-the-state-table.md` — The State Table
  - `02-desk-checking.md` — Desk Checking
- **When It Goes Wrong** — `section-02-when-it-goes-wrong/README.md`
  - `01-reading-an-error.md` — Reading an Error
  - `02-bisecting-a-bug.md` — Bisecting a Bug
  - `03-the-debugger.md` — The Debugger

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`


## Unit III — Abstraction by Procedure

*Giving a process a name. Methods, the call stack, recursion, and the contract as the unit of design.*

`book/unit-03-abstraction/intro.md`

### Chapter 11. Giving a Process a Name

Methods, parameters, return values, and why naming is the whole art.

`chapter-11-giving-a-process-a-name/README.md` — chapter overview

- **The Method** — `section-01-the-method/README.md`
  - `01-why-name-a-process.md` — Why Name a Process
  - `02-parameters-and-arguments.md` — Parameters and Arguments
  - `03-returning-a-value.md` — Returning a Value
- **The Contract** — `section-02-the-contract/README.md`
  - `01-preconditions-and-postconditions.md` — Preconditions and Postconditions
  - `02-a-method-you-can-trust.md` — A Method You Can Trust

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 12. The Stack

How a machine remembers where it was.

`chapter-12-the-stack/README.md` — chapter overview

- **Frames and Calls** — `section-01-frames-and-calls/README.md`
  - `01-the-call-frame.md` — The Call Frame
  - `02-the-stack-discipline.md` — The Stack Discipline
  - `03-stack-overflow.md` — Stack Overflow
- **What Gets Passed** — `section-02-what-gets-passed/README.md`
  - `01-values-and-references.md` — Values and References
  - `02-java-passes-by-value-always.md` — Java Passes by Value, Always
  - `03-overloading.md` — Overloading

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 13. Recursion

Self-reference as a tool, and the two shapes a recursive process can take.

`chapter-13-recursion/README.md` — chapter overview

- **Self-Reference** — `section-01-self-reference/README.md`
  - `01-the-base-case.md` — The Base Case
  - `02-trusting-the-recursion.md` — Trusting the Recursion
  - `03-recursion-and-induction.md` — Recursion and Induction
- **Shapes of Process** — `section-02-shapes-of-process/README.md`
  - `01-linear-and-tree-recursion.md` — Linear and Tree Recursion
  - `02-recursive-process-vs-recursive-procedure.md` — Recursive Process vs. Recursive Procedure
  - `03-when-recursion-is-wrong.md` — When Recursion Is Wrong

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 14. Designing with Procedures

Decomposition, and the first real taste of engineering judgement.

`chapter-14-designing-with-procedures/README.md` — chapter overview

- **Decomposition** — `section-01-decomposition/README.md`
  - `01-finding-the-seams.md` — Finding the Seams
  - `02-one-job-per-method.md` — One Job per Method
- **Testing a Method** — `section-02-testing-a-method/README.md`
  - `01-what-a-test-is.md` — What a Test Is
  - `02-choosing-cases.md` — Choosing Cases
  - `03-tests-as-documentation.md` — Tests as Documentation

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`


## Unit IV — Compound Data

*Many things at once. Arrays, collections, text, and the invariant that makes a heap of values into a structure.*

`book/unit-04-compound-data/intro.md`

### Chapter 15. Many Things at Once

Arrays: contiguous storage and the index as arithmetic.

`chapter-15-many-things-at-once/README.md` — chapter overview

- **The Array** — `section-01-the-array/README.md`
  - `01-why-an-index-is-arithmetic.md` — Why an Index Is Arithmetic
  - `02-declaring-and-filling.md` — Declaring and Filling
  - `03-bounds-and-what-they-protect.md` — Bounds and What They Protect
- **Arrays of Arrays** — `section-02-arrays-of-arrays/README.md`
  - `01-two-dimensional-data.md` — Two-Dimensional Data
  - `02-ragged-arrays.md` — Ragged Arrays
  - `03-traversal-patterns.md` — Traversal Patterns

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 16. Contracts for Data

The abstract data type, and the wrapper classes as a first case study.

`chapter-16-contracts-for-data/README.md` — chapter overview

- **The Abstract Data Type** — `section-01-the-abstract-data-type/README.md`
  - `01-separating-use-from-implementation.md` — Separating Use from Implementation
  - `02-the-representation-invariant.md` — The Representation Invariant
- **Boxes Around Primitives** — `section-02-boxes-around-primitives/README.md`
  - `01-wrapper-classes.md` — Wrapper Classes
  - `02-autoboxing-and-its-traps.md` — Autoboxing and Its Traps
  - `03-null-and-the-billion-dollar-mistake.md` — null and the Billion-Dollar Mistake

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 17. Growing Collections

ArrayList, the collections framework, and generics as a promise about content.

`chapter-17-growing-collections/README.md` — chapter overview

- **When the Size Is Not Known** — `section-01-when-the-size-is-not-known/README.md`
  - `01-the-arraylist.md` — The ArrayList
  - `02-how-growth-actually-works.md` — How Growth Actually Works
  - `03-list-set-and-map.md` — List, Set, and Map
- **Generics and Iteration** — `section-02-generics-and-iteration/README.md`
  - `01-generics-as-a-promise.md` — Generics as a Promise
  - `02-iterating-safely.md` — Iterating Safely
  - `03-choosing-a-collection.md` — Choosing a Collection

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 18. Text as Data

Strings, immutability, and turning characters into meaning.

`chapter-18-text-as-data/README.md` — chapter overview

- **The String** — `section-01-the-string/README.md`
  - `01-immutability-and-why.md` — Immutability, and Why
  - `02-building-text-efficiently.md` — Building Text Efficiently
  - `03-comparing-text.md` — Comparing Text
- **Parsing and Formatting** — `section-02-parsing-and-formatting/README.md`
  - `01-splitting-and-scanning.md` — Splitting and Scanning
  - `02-patterns-and-regular-expressions.md` — Patterns and Regular Expressions
  - `03-formatting-for-humans.md` — Formatting for Humans

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`


## Unit V — Objects, State, and Identity

*Bundling state with behavior. Classes, identity, inheritance, and polymorphism, treated as design decisions rather than syntax.*

`book/unit-05-objects/intro.md`

### Chapter 19. Bundling State and Behavior

The class as a way of keeping an invariant.

`chapter-19-bundling-state-and-behavior/README.md` — chapter overview

- **The Class** — `section-01-the-class/README.md`
  - `01-objects-as-little-machines.md` — Objects as Little Machines
  - `02-fields-and-constructors.md` — Fields and Constructors
  - `03-methods-that-guard-state.md` — Methods That Guard State
- **Encapsulation** — `section-02-encapsulation/README.md`
  - `01-what-private-is-for.md` — What private Is For
  - `02-the-public-surface.md` — The Public Surface
  - `03-static-and-the-class-itself.md` — static and the Class Itself

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 20. Identity and Equality

Two objects, one object, and the question of sameness.

`chapter-20-identity-and-equality/README.md` — chapter overview

- **References** — `section-01-references/README.md`
  - `01-the-reference-model.md` — The Reference Model
  - `02-aliasing.md` — Aliasing
  - `03-copying-shallow-and-deep.md` — Copying, Shallow and Deep
- **When Are Two Things the Same?** — `section-02-when-are-two-things-the-same/README.md`
  - `01-double-equals-vs-equals.md` — == vs. equals
  - `02-the-equals-hashcode-contract.md` — The equals/hashCode Contract
  - `03-immutability-as-a-strategy.md` — Immutability as a Strategy

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 21. Families of Types

Inheritance and polymorphism, and the substitution principle underneath them.

`chapter-21-families-of-types/README.md` — chapter overview

- **Inheritance** — `section-01-inheritance/README.md`
  - `01-sharing-by-extension.md` — Sharing by Extension
  - `02-overriding-and-super.md` — Overriding and super
  - `03-the-object-class.md` — The Object Class
- **Polymorphism** — `section-02-polymorphism/README.md`
  - `01-one-name-many-behaviors.md` — One Name, Many Behaviors
  - `02-dynamic-dispatch.md` — Dynamic Dispatch
  - `03-the-substitution-principle.md` — The Substitution Principle

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 22. Contracts Without Implementation

Interfaces, abstract classes, enums, and records.

`chapter-22-contracts-without-implementation/README.md` — chapter overview

- **Interfaces** — `section-01-interfaces/README.md`
  - `01-a-promise-with-no-body.md` — A Promise with No Body
  - `02-abstract-classes.md` — Abstract Classes
  - `03-programming-to-an-interface.md` — Programming to an Interface
- **Restricted Shapes** — `section-02-restricted-shapes/README.md`
  - `01-enums-as-closed-sets.md` — Enums as Closed Sets
  - `02-records-as-plain-data.md` — Records as Plain Data

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 23. Designing Object Systems

Responsibility, composition, and drawing before coding.

`chapter-23-designing-object-systems/README.md` — chapter overview

- **Assigning Responsibility** — `section-01-assigning-responsibility/README.md`
  - `01-who-should-know-this.md` — Who Should Know This?
  - `02-composition-over-inheritance.md` — Composition over Inheritance
  - `03-coupling-and-cohesion.md` — Coupling and Cohesion
- **Drawing the Design** — `section-02-drawing-the-design/README.md`
  - `01-uml-as-a-sketch.md` — UML as a Sketch
  - `02-a-worked-design.md` — A Worked Design

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`


## Unit VI — Programs as Data

*The centre of the book. Grammars, parsing, and an evaluator written in Java that runs a language of our own.*

`book/unit-06-programs-as-data/intro.md`

### Chapter 24. Languages and Grammars

Syntax as a formal object.

`chapter-24-languages-and-grammars/README.md` — chapter overview

- **What a Language Is** — `section-01-what-a-language-is/README.md`
  - `01-strings-and-languages.md` — Strings and Languages
  - `02-grammars.md` — Grammars
  - `03-ambiguity.md` — Ambiguity
- **From Text to Tree** — `section-02-from-text-to-tree/README.md`
  - `01-tokenising.md` — Tokenising
  - `02-the-syntax-tree.md` — The Syntax Tree
  - `03-a-recursive-descent-parser.md` — A Recursive-Descent Parser

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 25. An Evaluator

We build an interpreter, and the distinction between a program and its meaning becomes concrete.

`chapter-25-an-evaluator/README.md` — chapter overview

- **Evaluation** — `section-01-evaluation/README.md`
  - `01-the-eval-apply-loop.md` — The eval/apply Loop
  - `02-environments.md` — Environments
  - `03-evaluating-arithmetic.md` — Evaluating Arithmetic
- **A Language of Our Own** — `section-02-a-language-of-our-own/README.md`
  - `01-adding-names-and-definitions.md` — Adding Names and Definitions
  - `02-adding-procedures.md` — Adding Procedures
  - `03-what-we-have-built.md` — What We Have Built

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 26. Functions as Values

Lambdas, higher-order methods, and abstraction over process.

`chapter-26-functions-as-values/README.md` — chapter overview

- **Passing Behavior** — `section-01-passing-behavior/README.md`
  - `01-functional-interfaces.md` — Functional Interfaces
  - `02-lambda-expressions.md` — Lambda Expressions
  - `03-higher-order-methods.md` — Higher-Order Methods
- **Pipelines** — `section-02-pipelines/README.md`
  - `01-map-filter-reduce.md` — Map, Filter, Reduce
  - `02-streams-in-java.md` — Streams in Java
  - `03-when-not-to.md` — When Not To

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 27. Programs That Inspect Themselves

Reflection, erasure, and the price of looking in the mirror.

`chapter-27-programs-that-inspect-themselves/README.md` — chapter overview

- **Reflection** — `section-01-reflection/README.md`
  - `01-a-class-as-an-object.md` — A Class as an Object
  - `02-what-reflection-costs.md` — What Reflection Costs
- **The Limits of the Type System** — `section-02-the-limits-of-the-type-system/README.md`
  - `01-erasure.md` — Erasure
  - `02-annotations.md` — Annotations

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`


## Unit VII — The World Outside the Program

*Failure, persistence, events, and concurrency: everything that happens when a program stops being a pure function and meets a world that does not cooperate.*

`book/unit-07-the-world-outside/intro.md`

### Chapter 28. When Things Go Wrong

Exceptions as a control-flow mechanism and a design statement.

`chapter-28-when-things-go-wrong/README.md` — chapter overview

- **Failure as a Value** — `section-01-failure-as-a-value/README.md`
  - `01-why-not-just-return-minus-one.md` — Why Not Just Return -1
  - `02-throwing-and-catching.md` — Throwing and Catching
  - `03-checked-and-unchecked.md` — Checked and Unchecked
- **Designing for Failure** — `section-02-designing-for-failure/README.md`
  - `01-where-to-handle.md` — Where to Handle
  - `02-resources-and-cleanup.md` — Resources and Cleanup
  - `03-failing-loudly.md` — Failing Loudly

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 29. Persistence

Files, streams, and the fact that storage outlives the process.

`chapter-29-persistence/README.md` — chapter overview

- **Files and Streams** — `section-01-files-and-streams/README.md`
  - `01-the-stream-abstraction.md` — The Stream Abstraction
  - `02-reading-and-writing-text.md` — Reading and Writing Text
  - `03-bytes-and-buffers.md` — Bytes and Buffers
- **Structured Storage** — `section-02-structured-storage/README.md`
  - `01-delimited-data.md` — Delimited Data
  - `02-records-that-outlive-the-program.md` — Records That Outlive the Program
  - `03-a-glimpse-of-databases.md` — A Glimpse of Databases

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 30. Events and Interfaces

The event loop, and why a GUI inverts control.

`chapter-30-events-and-interfaces/README.md` — chapter overview

- **Inversion of Control** — `section-01-inversion-of-control/README.md`
  - `01-the-event-loop.md` — The Event Loop
  - `02-listeners-and-callbacks.md` — Listeners and Callbacks
- **Building a Window** — `section-02-building-a-window/README.md`
  - `01-components-and-layout.md` — Components and Layout
  - `02-drawing.md` — Drawing
  - `03-separating-model-from-view.md` — Separating Model from View

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 31. Many Things at Once, Really

Concurrency, and the return of state as the central difficulty.

`chapter-31-many-things-at-once-really/README.md` — chapter overview

- **Threads** — `section-01-threads/README.md`
  - `01-why-concurrency.md` — Why Concurrency
  - `02-shared-state-and-races.md` — Shared State and Races
  - `03-locks-and-their-cost.md` — Locks and Their Cost
- **Talking to Other Machines** — `section-02-talking-to-other-machines/README.md`
  - `01-sockets-and-protocols.md` — Sockets and Protocols
  - `02-blocking-and-waiting.md` — Blocking and Waiting

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`


## Unit VIII — Limits and Cost

*What programs cost and what no program can do. Complexity, information, and undecidability close the argument the first unit opened.*

`book/unit-08-limits-and-cost/intro.md`

### Chapter 32. Counting the Cost

Complexity as a way of comparing algorithms without a stopwatch.

`chapter-32-counting-the-cost/README.md` — chapter overview

- **Growth** — `section-01-growth/README.md`
  - `01-counting-operations.md` — Counting Operations
  - `02-big-o.md` — Big-O
  - `03-the-usual-classes.md` — The Usual Classes
- **Cost in Practice** — `section-02-cost-in-practice/README.md`
  - `01-searching-and-sorting.md` — Searching and Sorting
  - `02-space-as-a-cost.md` — Space as a Cost
  - `03-measuring-honestly.md` — Measuring Honestly

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 33. Information

Entropy, compression, and the return of the encoding question.

`chapter-33-information/README.md` — chapter overview

- **How Surprising Is a Message?** — `section-01-how-surprising-is-a-message/README.md`
  - `01-information-as-surprise.md` — Information as Surprise
  - `02-entropy.md` — Entropy
- **Compression** — `section-02-compression/README.md`
  - `01-variable-length-codes.md` — Variable-Length Codes
  - `02-why-you-cannot-compress-everything.md` — Why You Cannot Compress Everything

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 34. What No Program Can Do

The halting problem, undecidability, and Kolmogorov complexity.

`chapter-34-what-no-program-can-do/README.md` — chapter overview

- **The Halting Problem** — `section-01-the-halting-problem/README.md`
  - `01-a-program-that-reads-programs.md` — A Program That Reads Programs
  - `02-the-contradiction.md` — The Contradiction
  - `03-what-it-does-and-does-not-mean.md` — What It Does and Does Not Mean
- **Descriptions** — `section-02-descriptions/README.md`
  - `01-kolmogorov-complexity.md` — Kolmogorov Complexity
  - `02-randomness-as-incompressibility.md` — Randomness as Incompressibility

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`

### Chapter 35. Where You Are Now

What was actually learned, and the shape of the road ahead.

`chapter-35-where-you-are-now/README.md` — chapter overview

- **The Through-Line** — `section-01-the-through-line/README.md`
  - `01-representation-revisited.md` — Representation Revisited
  - `02-abstraction-revisited.md` — Abstraction Revisited
- **Going On** — `section-02-going-on/README.md`
  - `01-what-to-read-next.md` — What to Read Next
  - `02-a-last-word.md` — A Last Word

  Back matter: `exercises.md`, `further-reading.md`, `important-concepts.md`, `important-researchers.md`


## Appendices

- `appendices/appendix-a-the-toolchain.md` — **Appendix A — The Toolchain**: javac, java, classpath, jar, and build tools.
- `appendices/appendix-b-testing.md` — **Appendix B — Testing with JUnit**: Writing and running automated tests.
- `appendices/appendix-c-documentation-and-style.md` — **Appendix C — Documentation and Style**: Javadoc, naming conventions, and readable code.
- `appendices/appendix-d-version-control.md` — **Appendix D — Version Control**: Git for people who have never used it.
- `appendices/appendix-e-syntax-reference.md` — **Appendix E — Java Syntax Reference**: A compact reference for everything used in this book.

---

## Build

```bash
python3 tools/build_book.py programs-and-machines --pdf
python3 tools/build_book.py programs-and-machines --check
python3 tools/validate.py --book programs-and-machines
```
