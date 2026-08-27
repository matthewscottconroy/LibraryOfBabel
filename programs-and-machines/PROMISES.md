# Promises Outstanding

Every forward reference made by the written chapters to a chapter that does not
yet exist. Each is a commitment the reader has already been given, and the units
that follow have to honor them.

Regenerate after any edit; `tools-lint.py` guarantees the references are in range,
not that they are kept.

**18 promises across 3 unwritten chapters.**

---

## Chapter 33 — Information  *(Unit VIII: Limits and Cost)*

- **unit-01-representation/chapter-01-two-voltages-and-an-agreement/section-02-encoding-as-convention/01-agreements-not-facts.md**
  > …etween them is choosing an agreement. In **Chapter 25**, we will write an interpreter, and a data structure will become a program purely because our evaluator agreed to read it as one. In **Chapter 33**, Shannon will let us measure how much information an agreement can carry at all. Each of those will feel like a new topic when you reach it. It is worth noticing that it is the same topic…
- **unit-08-limits-and-cost/chapter-32-counting-the-cost/section-02-cost-in-practice/01-searching-and-sorting.md**
  > …sorting, and the way past a lower bound is always to change the problem. That structure — count the possibilities, count what the algorithm can distinguish, conclude — is worth remembering. Chapter 33 uses it to prove that no compressor shrinks everything, and Chapter 34's argument is a relative of it. ## In practice **Do not write a sort.** `Arrays.sort` and `Collections.sort` are imple…
- **unit-08-limits-and-cost/chapter-32-counting-the-cost/section-02-cost-in-practice/03-measuring-honestly.md**
  > …is what that looks like when it works. The bubble-sort row above is what it looks like when it does not, and reporting the second is as much a part of the discipline as reporting the first. Chapter 33 turns from how long a program takes to how much a message contains, and the counting arguments start proving things impossible.
- **unit-08-limits-and-cost/intro.md**
  > …s you will meet. Then the practical half — searching and sorting, space as a cost, and the gap between what theory predicts and what a machine does, which is wider than the theory admits. **Chapter 33 — Information.** How much is in a message. Entropy, which measures surprise, and compression, which is what you can do about it. It ends with a counting argument proving that no compressor …

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
- **unit-08-limits-and-cost/chapter-32-counting-the-cost/section-02-cost-in-practice/01-searching-and-sorting.md**
  > …e, and it doubles with every doubling of $n$. Section 32.2.3 has one more row of this table, and it does not behave. ## The lower bound A genuine impossibility result, and a mild preview of Chapter 34. **No comparison sort can be faster than $O(n \log n)$ in the worst case.** The argument is a counting one. A sorting algorithm's behavior is determined by the answers to its comparisons, e…
- **unit-08-limits-and-cost/chapter-32-counting-the-cost/section-02-cost-in-practice/01-searching-and-sorting.md**
  > …. That structure — count the possibilities, count what the algorithm can distinguish, conclude — is worth remembering. Chapter 33 uses it to prove that no compressor shrinks everything, and Chapter 34's argument is a relative of it. ## In practice **Do not write a sort.** `Arrays.sort` and `Collections.sort` are implemented by specialists, tuned over decades, and better than what you wil…
- **unit-08-limits-and-cost/intro.md**
  > …ssion, which is what you can do about it. It ends with a counting argument proving that no compressor can shrink everything — the first impossibility result in the book, and a gentle one. **Chapter 34 — What No Program Can Do.** The halting problem. Undecidability. Kolmogorov complexity, and the observation that most strings cannot be described more briefly than by writing them out. Turi…
- **unit-08-limits-and-cost/intro.md**
  > …g problem's diagonal argument is not hard, but it is meaningless if you have never seen a program take another program as data. You have. Chapter 25 was, among other things, preparation for Chapter 34. A note on the arithmetic. This unit has more mathematics in it than the others, and none of it is beyond what a first-year course assumes: logarithms, exponents, sums, and one limit. Where…

## Chapter 35 — Where You Are Now  *(Unit VIII: Limits and Cost)*

- **unit-06-programs-as-data/chapter-25-an-evaluator/important-researchers.md**
  > …first course should treat programs as objects of study, and should culminate in writing an evaluator for the language you are learning — is the reason this unit exists in the shape it does. Chapter 35 returns to it.
- **unit-08-limits-and-cost/intro.md**
  > …Kolmogorov complexity, and the observation that most strings cannot be described more briefly than by writing them out. Turing's 1936 argument, in full, and it is shorter than you expect. **Chapter 35 — Where You Are Now.** What was actually learned, what the through- line was, and what to read next. Why is this the last unit? Because these are questions about programs in general, and an…
