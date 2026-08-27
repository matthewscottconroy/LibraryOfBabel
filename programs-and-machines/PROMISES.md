# Promises Outstanding

Every forward reference made by the written chapters to a chapter that does not
yet exist. Each is a commitment the reader has already been given, and the units
that follow have to honor them.

Regenerate after any edit; `tools-lint.py` guarantees the references are in range,
not that they are kept.

**27 promises across 4 unwritten chapters.**

---

## Chapter 32 — Counting the Cost  *(Unit VIII: Limits and Cost)*

- **unit-02-computation/chapter-09-repeating/section-01-the-shape-of-a-loop/03-nested-loops.md**
  > …ast row is ten billion iterations, which on a modern machine is seconds to minutes rather than the microseconds you might have assumed from the fact that the code is six lines long. This is Chapter 32's subject, and the reason to raise it here is that nesting is where beginners first write something accidentally unusable. Code that is instant on the ten-item test data and unusable on the…
- **unit-02-computation/chapter-10-reading-a-programs-mind/section-02-when-it-goes-wrong/02-bisecting-a-bug.md**
  > …y you have halved the search. Repeat. Each observation halves what is left, so a program of a thousand steps is narrowed in about ten questions, and a million steps in about twenty. That is Chapter 32's logarithm, arriving in a practical setting. ## Doing it ```java int[] data = readData(); // is data what I expect here? int[] cleaned = clean(data); // is cleaned right? int[] sorted = so…
- **unit-03-abstraction/chapter-11-giving-a-process-a-name/section-01-the-method/03-returning-a-value.md**
  > …visible at the call site. **Split into two methods.** Frequently best when the two results are independently useful. Two passes over the data is usually a cost worth paying for clarity, and Chapter 32 will let you judge when it is not. ## Naming, again Return type and name should agree, and readers rely on it more than they realize. ```java int count(...) // returns a number boolean isVa…
- **unit-03-abstraction/chapter-13-recursion/further-reading.md**
  > … 4. Chapter 2 covers merge sort — the canonical divide-and-conquer recursion. Chapter 4 covers how to analyze the cost of a recursive algorithm, which is what Section 13.2.1 gestures at and Chapter 32 will need. ## On memoization and dynamic programming Bellman, R. (1957). *Dynamic Programming*. Princeton University Press. The origin of the technique that fixes the Fibonacci problem. Bel…
- **unit-03-abstraction/chapter-13-recursion/section-02-shapes-of-process/01-linear-and-tree-recursion.md**
  > … tree contains enormous duplication, and nothing remembers that a value has already been found. Roughly, the number of calls grows like the golden ratio to the *n*. That is exponential, and Chapter 32 will make the vocabulary precise; the practical point is available now: **a tree recursion that recomputes shared subproblems is unusable beyond small inputs.** ## Two fixes **Remember answ…
- **unit-03-abstraction/chapter-13-recursion/section-02-shapes-of-process/01-linear-and-tree-recursion.md**
  > …f tree recursion, worth naming because you will meet it constantly. Split the problem in half, solve both halves, combine. Merge sort and binary search do this, and so does the quicksort of Chapter 32. The branches do not overlap — the two halves are disjoint — so there is no recomputation. And because the input halves each time, the depth is about $\log_{2} n$ rather than *n*: a million…
- **unit-03-abstraction/chapter-14-designing-with-procedures/section-01-decomposition/01-finding-the-seams.md**
  > …wn. **Three passes instead of one.** The original walked `scores` three times too, so nothing changed here — but a decomposition *can* cost performance by preventing a single combined pass. Chapter 32 gives you the tools to judge when that matters. It usually does not. ## The judgment The signals tell you *where* a division is available. They do not tell you whether to take it, and that …
- **unit-04-compound-data/chapter-15-many-things-at-once/section-01-the-array/01-why-an-index-is-arithmetic.md**
  > … + i × elementSize ``` For element 2: 1000 + 2 × 4 = 1008. One multiplication, one addition, done. No searching, no comparison, no walking. That is **constant-time access**, written O(1) in Chapter 32's notation, and it is the property that makes arrays worth having. ## Why the constraints follow Look at what the formula requires and every "restriction" of arrays becomes a consequence. *…
- **unit-04-compound-data/chapter-15-many-things-at-once/section-02-arrays-of-arrays/03-traversal-patterns.md**
  > …in it, and this is part of why an `ArrayList` often outperforms a `LinkedList` even for operations where the linked structure has the better theoretical cost — Chapter 17 returns to it, and Chapter 32 gives the vocabulary for why "theoretical cost" and "measured cost" can disagree. ## Other patterns Some traversals you will write. **The diagonal**, where row and column advance together: …
- **unit-04-compound-data/chapter-15-many-things-at-once/section-02-arrays-of-arrays/03-traversal-patterns.md**
  > … consider(grid[r][c]); ``` Note `c = r + 1` — starting the inner loop from the outer index is the idiom for "each pair once", and it is worth recognizing on sight. It halves the work, which Chapter 32 will say does not change the complexity and does change the wall clock. **Neighbors**, for grids where cells interact: ```java for (int dr = -1; dr <= 1; dr++) { for (int dc = -1; dc <= 1; …
- **unit-04-compound-data/chapter-17-growing-collections/further-reading.md**
  > …repper, U. (2007). "What Every Programmer Should Know About Memory." LWN.net. Recommended in Chapter 15, and the explanation for why `LinkedList` loses cases that theory says it should win. Chapter 32 revisits the gap between predicted and measured cost.
- **unit-04-compound-data/chapter-17-growing-collections/important-researchers.md**
  > …t claims about how fast a program runs should be *derived* rather than asserted, with the arithmetic shown. The sum of powers of two in Section 17.1.2 is a small instance of that habit, and Chapter 32 will take it up properly. He is also the author of TeX, the typesetting system this book is set with, written because he was unhappy with the appearance of the second edition of his own boo…
- **unit-04-compound-data/chapter-17-growing-collections/section-02-generics-and-iteration/03-choosing-a-collection.md**
  > …n. Six types cover almost everything, and the remainder — `ArrayDeque`, `PriorityQueue`, the concurrent collections of Chapter 31 — announce themselves when you need them. ## Costs, roughly Chapter 32 gives the notation; the shape is useful now. | operation | `ArrayList` | `LinkedList` | `HashSet`/`HashMap` | `TreeSet`/`TreeMap` | |---|---|---|---|---| | access by position | constant | l…
- **unit-06-programs-as-data/chapter-24-languages-and-grammars/important-researchers.md**
  > …class of grammars could be parsed deterministically in one left-to-right pass. Knuth appears elsewhere in this book for analysis of algorithms and for *The Art of Computer Programming*, and Chapter 32 quotes his warning about premature optimization. The parsing result is the one that turned compiler construction from craft into engineering. Turing Award, 1974. **Frances Allen** (1932–202…
- **unit-06-programs-as-data/chapter-25-an-evaluator/exercises.md**
  > …at the first. **25.10** Instrument `eval` to count node visits. Report the counts for `fib(10)`, `fib(15)` and `fib(20)`. Then say what kind of growth that is, and check your answer against Chapter 32 when you get there. **25.11** *Longer.* Add closures. Allow `def` inside a procedure body, store the defining environment in the `Procedure` record, and use it as the parent in `apply`. The…
- **unit-06-programs-as-data/chapter-25-an-evaluator/exercises.md**
  > …Report every place that had to change, and then say what a type checker would be for. **25.13** [carries forward] Keep the interpreter. Chapter 27 uses it as the example for reflection, and Chapter 32 uses its measured costs. If you have added features of your own, keep those too.
- **unit-06-programs-as-data/chapter-25-an-evaluator/section-02-a-language-of-our-own/03-what-we-have-built.md**
  > …lications ``` Each step of `n` by 5 multiplies the work by about eleven — the growth is exponential, which is a property of the naive Fibonacci algorithm rather than of our interpreter, and Chapter 32 will name it. But every one of those 242,785 applications allocates a `HashMap`, and every node visit performs a type switch that could have been decided once. **A tree-walking interpreter …

## Chapter 33 — Information  *(Unit VIII: Limits and Cost)*

- **unit-01-representation/chapter-01-two-voltages-and-an-agreement/section-02-encoding-as-convention/01-agreements-not-facts.md**
  > …etween them is choosing an agreement. In **Chapter 25**, we will write an interpreter, and a data structure will become a program purely because our evaluator agreed to read it as one. In **Chapter 33**, Shannon will let us measure how much information an agreement can carry at all. Each of those will feel like a new topic when you reach it. It is worth noticing that it is the same topic…

## Chapter 34 — What No Program Can Do  *(Unit VIII: Limits and Cost)*

- **unit-02-computation/chapter-06-what-a-step-is/section-01-state-and-transition/03-tables-that-compute.md**
  > …able with four entries.** Nothing in the machine knows what parity is. The knowledge is in the arrangement, not in any part. You will meet this pattern repeatedly, at every scale. It is why Chapter 34 can argue about what programs can and cannot do without ever needing to ask whether they understand anything. ## Where finite states run out Finite state machines are not all-powerful, and …
- **unit-02-computation/chapter-06-what-a-step-is/section-02-two-famous-machines/01-the-turing-machine.md**
  > …other machine, plus its input, and determines whether that machine ever halts. He proved no such machine can exist — the **halting problem** is undecidable. We will do the proof properly in Chapter 34, because it deserves the room and because you will need Unit VI's idea of programs-as-data to feel its force. For now, hold the shape: some questions about programs cannot be answered by an…
- **unit-02-computation/chapter-09-repeating/exercises.md**
  > …he inductive step, and what plays the role of "for all n"? **9.20.** The Collatz conjecture is unproved. Explain what that means for the claim "this loop terminates", and connect it to what Chapter 34 will say about the halting problem in general.
- **unit-02-computation/chapter-09-repeating/important-concepts.md**
  > …riant plus variant is **total correctness**. **Termination can be genuinely hard.** The Collatz loop is six lines and whether it always halts is an open problem posed in 1937. This previews Chapter 34: no method decides halting for arbitrary programs. **Off-by-one is a failed boundary claim.** Not carelessness — the code looks right, which is why it was written. A range has two ends and …
- **unit-02-computation/chapter-09-repeating/section-02-what-a-loop-promises/02-termination.md**
  > …because it makes the point that termination is genuinely a separate question, and can be genuinely hard. That six-line loop's termination is an open problem in mathematics. It also previews Chapter 34. If deciding whether *this* loop halts is beyond us, you might guess that deciding it for arbitrary programs is beyond any method at all. That guess is correct, and Turing proved it in 1936…
- **unit-06-programs-as-data/chapter-24-languages-and-grammars/important-concepts.md**
  > … set of rules that produces exactly the members. **More languages than descriptions** — $\Sigma^*$ is countable but its subsets are not, so some languages have no grammar and no recognizer. Chapter 34 exhibits a natural one. **The Chomsky hierarchy** — regular, context-free, context-sensitive, unrestricted, each needing more machine than the last. Two matter here: words are regular, nest…
- **unit-06-programs-as-data/chapter-24-languages-and-grammars/section-01-what-a-language-is/01-strings-and-languages.md**
  > …xactly the strings in the language. That is a grammar, and it is the next lesson. Both descriptions are finite. The set need not be. ## Not every language has one Worth knowing now, because Chapter 34 collects the debt. $\Sigma^*$ is countably infinite — you can list all strings in order of length. But the *languages* over $\Sigma$ are all the subsets of $\Sigma^*$, and there are uncount…
- **unit-06-programs-as-data/chapter-24-languages-and-grammars/section-01-what-a-language-is/01-strings-and-languages.md**
  > …there are strictly more languages than there are finite descriptions, and therefore languages that no grammar generates and no program recognizes. This is not a curiosity about exotic sets. Chapter 34 exhibits a specific, extremely natural language — the set of programs that halt — and shows no recognizer exists. The counting argument above is why such a thing has to exist; the halting p…

## Chapter 35 — Where You Are Now  *(Unit VIII: Limits and Cost)*

- **unit-06-programs-as-data/chapter-25-an-evaluator/important-researchers.md**
  > …first course should treat programs as objects of study, and should culminate in writing an evaluator for the language you are learning — is the reason this unit exists in the shape it does. Chapter 35 returns to it.
