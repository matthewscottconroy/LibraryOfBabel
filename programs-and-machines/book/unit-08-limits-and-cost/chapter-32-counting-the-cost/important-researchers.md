# Important Researchers

**Donald Knuth** (born 1938) returns from Chapter 24, and this is his subject. *The
Art of Computer Programming*, begun in 1962 and still unfinished, established that
algorithms could be analyzed mathematically rather than merely timed — that you
could state what a program costs as a function of its input and prove it. He
popularized big-O notation for this purpose, borrowing it from analytic number
theory, and the practice of deriving costs rather than measuring them is largely
his doing.

He also wrote the sentence this chapter and Chapter 18 both circle: *premature
optimization is the root of all evil*. It is quoted so often and so partially that
the surrounding argument is worth recovering — he was making the case that the
three percent of code where efficiency matters should be optimized carefully, and
that identifying which three percent requires measurement. It is an argument for
measuring, not against optimizing. Turing Award, 1974.

**Paul Bachmann** (1837–1920) and **Edmund Landau** (1877–1938) invented the
notation. Bachmann introduced $O$ in 1894 in a book on number theory; Landau
popularized it, which is why it is sometimes called Landau notation. Neither had
any interest in computation; they were describing the growth of error terms in
analytic estimates, and the notation waited seventy years for Knuth to find a
second use for it.

**Tony Hoare** (born 1934) appears a third time, for quicksort — which he invented
in 1959, at twenty-five, while a visiting student in Moscow working on machine
translation of Russian. He needed to sort words to look them up in a dictionary,
and the algorithm he devised is still, sixty-five years later, the basis of the
sorting routine in most standard libraries. It is a good example of a fundamental
algorithm arriving as a side effect of an applied problem.

**John von Neumann** (1903–1957) wrote merge sort in 1945, in the same period as
the stored-program report of Chapter 6, and it is thought to be the first sorting
algorithm written for a stored-program computer. That divide-and-conquer, the
stored-program architecture, and a good deal of the mathematics underlying both
came from one person in one decade is a fact worth pausing over.

**Robert Tarjan** (born 1948) developed amortized analysis into a formal technique
in the 1970s and 1980s, giving the accounting and potential methods that make
Section 32.2.2's `ArrayList` argument rigorous for structures more complicated than
a doubling array. He also invented or co-invented a remarkable number of the graph
algorithms that are now standard. Turing Award, 1986.

**Tim Peters** (born 1968) wrote TimSort in 2002, for Python, by observing that
real data is rarely random — it usually contains long runs that are already
ordered — and building a merge sort that finds and exploits them. It is stable,
it is $O(n \log n)$ worst case and $O(n)$ on nearly sorted input, and Java adopted
it for `Arrays.sort` on objects. It is a good illustration of Section 32.2.3's
point: the win came from a fact about real inputs that no worst-case analysis can
express.
