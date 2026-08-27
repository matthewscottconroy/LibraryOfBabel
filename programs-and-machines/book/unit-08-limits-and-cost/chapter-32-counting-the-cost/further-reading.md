# Further Reading

**Cormen, Leiserson, Rivest and Stein, *Introduction to Algorithms*.** Universally
CLRS, and the standard reference. Comprehensive, rigorous, and heavy. Chapters 1
through 4 cover this chapter's material properly, including the recurrence-solving
that Chapter 13 gestured at; chapter 17 is amortized analysis. Use it as a
reference rather than reading it through.

**Jon Bentley, *Programming Pearls*, second edition.** The best available
antidote to treating this material as abstract. Short essays, each starting from a
real problem, and the column on the $O(n^2)$-to-$O(n)$ improvement in a sorting
problem is the clearest illustration of why the class matters. Bentley also
measures constantly, which is the habit this chapter is arguing for.

**Donald Knuth, "Structured Programming with go to Statements" (1974).** The paper
containing the premature-optimization sentence. Read the surrounding two
paragraphs; the argument is not the one the quotation is usually used to make.

**Ulrich Drepper, "What Every Programmer Should Know About Memory" (2007).**
Recommended in Chapters 15 and 17 and finally fully relevant. It is the explanation
for Section 32.2.3's memory-hierarchy gap, and for why the `LinkedList`
measurements disagree with the theory. Long, and the first thirty pages are the
ones to read.

**Steven Skiena, *The Algorithm Design Manual*, third edition.** More practical
than CLRS and better for the question "what algorithm should I use here". The
catalogue in the second half is genuinely useful, and the war stories are the best
argument in print that analysis pays off on real problems.

**Robert Sedgewick and Kevin Wayne, *Algorithms*, fourth edition.** Java
throughout, which makes it the natural companion to this book, with good
visualizations and an emphasis on empirical measurement alongside the analysis.
Their Coursera courses cover the same material.

**Tim Peters's original TimSort description**, in the Python source's
`listsort.txt`. A working programmer explaining a real algorithm, including why
each decision was made and what was measured. Rare and worth reading.
