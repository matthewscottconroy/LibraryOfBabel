# Important Concepts

**Count operations, not seconds** — seconds depend on the machine, the JIT, and
the cache; counts are machine-independent, predict behavior at sizes you have not
run, and isolate the algorithm from its implementation.

**What to count** — the operation the program does most of and that is expensive.
Comparisons for a search, I/O for anything touching disk or network. Counting the
wrong thing is a subtler error than counting incorrectly.

**Best, worst, average** — the worst case is a guarantee and the default; the
average requires knowing the input distribution, which is usually a guess. Where an
adversary chooses the input, the worst case is not hypothetical.

**Big-O** — $f$ is $O(g)$ if beyond some $n_0$, $f(n) \le c \cdot g(n)$. Constant
factors and small inputs are deliberately excluded.

**Drop constants and lower-order terms** — $n(n-1)/2$ is $O(n^2)$. Which is why
halving the work of a nested loop is a real improvement and invisible to the
notation.

**$O$, $\Omega$, $\Theta$** — upper bound, lower bound, tight bound. Almost
everyone writes $O$ meaning $\Theta$.

**Sequential work adds, nested work multiplies** — so three passes over an array
are still $O(n)$, which is why Chapter 14's decomposition cost nothing.

**Hidden costs in method calls** — `list.contains` inside a loop is $O(n \cdot m)$;
string concatenation in a loop is $O(n^2)$. Both look linear.

**Logarithms have no base in big-O** — different bases differ by a constant factor,
which is discarded.

**The seven classes** — constant, logarithmic, linear, linearithmic, quadratic,
exponential, factorial. The useful reading is what doubling $n$ does to each.

**$O(1)$ means unchanging, not fast** — a constant-time operation taking a
millisecond loses to a linear one taking a nanosecond per element, up to a million.

**$O(\log n)$ is effectively free** — ten million elements in 24 comparisons, a
billion in 30.

**$O(n)$ is optimal for anything that must read all its input** — a real lower
bound, not a limitation.

**Quadratic is fine to about ten thousand and painful past a hundred thousand.
Exponential is never fine past about forty**, and a faster machine buys one more
element.

**The measured signatures** — quadratic growth multiplying by exactly 4.00 per
doubling; naive Fibonacci's calls growing by a factor of 1.618 per step, the golden
ratio, confirming Chapter 13's prediction to three digits.

**Changing the class beats improving the constant** — past some size, and no amount
of tuning changes the class. The standard moves: a hash set for a nested scan,
sorting once then binary searching, memoization, and exploiting structure.

**The comparison-sort lower bound** — $k$ comparisons distinguish at most $2^k$
orderings and there are $n!$ of them, so $k \ge \log_2(n!) = \Theta(n \log n)$. A
statement about every comparison sort that could ever be written.

**Counting and radix sort are $O(n)$** because they do not compare. The way past a
lower bound is always to change the problem.

**Stability** — equal elements keep their relative order. Java's `Arrays.sort` is
stable on objects (TimSort) and not on primitives (dual-pivot quicksort).

**Insertion sort is $O(n^2)$ and the right choice under about fifty elements** —
which is why real sort implementations contain one.

**Space complexity** — the same notation counting memory. Recursion depth is space,
so a recursion proportional to the input has an input-size limit far below what
memory suggests; a $\log n$-deep recursion is fine forever.

**Measured space** — 4.2 bytes per element for `int[]`, 20.3 for
`ArrayList<Integer>`. Chapter 16's boxing in bytes: an object header, the value,
padding, and a reference.

**The time-space trade** — a cache or an index spends space to save time;
recomputing or streaming spends time to save space.

**Amortized cost** — `ArrayList.add` is $O(1)$ amortized because doubling to
capacity $n$ copies $n - 1$ elements in total over $n$ additions. Growing by a
constant instead would make it $O(n)$ amortized and building a list quadratic.

**Four things big-O cannot see** — the memory hierarchy, constant factors, branch
prediction, and the JIT. The first is the largest, and it is why an `ArrayList`
beats a `LinkedList` at operations the theory says the list should win.

**The row that did not behave** — bubble sort at 64,000 elements grew by a factor
of 12 rather than the predicted 4, reproducibly, and this chapter does not
establish why. A model with a residual is still a model; a chapter showing only the
rows that fit would teach something false.

**Analyse to predict how it scales; measure to know what it costs.** Both, for
different questions. Analysis without measurement optimizes the wrong constant;
measurement without analysis ships something that fails when the data grows.
