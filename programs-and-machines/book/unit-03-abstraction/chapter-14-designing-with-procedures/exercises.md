# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## Decomposition

**14.1.** Take `report` from Section 14.1.1 and mark every seam signal you can
find. For each, say which signal it is.

**14.2. [carries forward]** Decompose this into named methods. State each one's
contract.
```java
static void run(String[] words) {
    int longest = 0;
    for (String w : words) if (w.length() > longest) longest = w.length();
    int total = 0;
    for (String w : words) total += w.length();
    System.out.println("Longest: " + longest);
    System.out.println("Average: " + (double) total / words.length);
}
```

**14.3.** For each, say whether it does one job. If not, say how you would split
it:
- `validateAndSave`
- `calculateTax`
- `printReportAndClearBuffer`
- `findLargestPrimeFactor`
- `updateUserAndSendEmailAndLog`

**14.4.** Find a method in `processOrder` (Section 14.1.2) that operates at the
wrong level of abstraction. Extract it and say what improved.

**14.5.** Section 14.1.1 lists what was *lost* by decomposing `report`. Give a
situation where those costs would outweigh the benefits.

## Command and query

**14.6.** Classify each as command, query, or both:
```java
boolean isEmpty()
void clear()
int size()
Item removeFirst()
boolean add(Item x)
void setName(String n)
```

**14.7.** `removeFirst()` both changes the collection and returns a value.
Explain why that is a command–query violation, and why it is nonetheless
universal in collection libraries.

**14.8.** Why can a query be called twice safely while a command cannot? Answer in
terms of what each does to the state.

## Testing

**14.9.** Write the equivalence classes for `static boolean isValidPassword(String
s)` where a valid password is 8–20 characters with at least one digit.

**14.10. [carries forward]** Write tests for `mean(int[] a)` from Section 14.1.1.
Include at least one case that would fail on the current implementation, and say
what it reveals.

**14.11.** For `static String substring(String s, int from, int to)`, list ten
test cases. At least six should be boundary cases.

**14.12.** This method has 100% line coverage from one test. Give a test that
finds a bug it misses.
```java
static int percent(int part, int whole) { return part * 100 / whole; }
```

**14.13.** Explain why high coverage is necessary but not sufficient. Construct a
test suite with full coverage that checks nothing useful.

**14.14.** Write tests for `largest` that would have caught the all-negative bug
from Chapter 10, and the empty-array bug.

## Tests as documentation

**14.15.** A comment says a method returns 0 for empty input; the method throws.
Explain why the comment is worse than no comment at all.

**14.16.** Rewrite these test names so a failure report identifies the problem:
`test1`, `testLargest`, `testEmpty`, `testStuff`.

**14.17. [carries forward]** Take a method from a library you use. Find its tests
if they are public, or write the tests you would expect. What did you learn about
the method that its documentation did not tell you?

## Going further

**14.18.** Section 14.2.3 claims difficulty testing something is evidence about
its design. Take a method that would be hard to test — one that reads the system
clock, say — and restructure it so it is easy. What did you change?

**14.19.** Argue for writing tests before the code. Then argue against. State
which you would do for: a script you will run once; a library others will depend
on; a bug fix in unfamiliar code.

**14.20.** Unit III's closing paragraph says a method is a contract, the stack is
the mechanism, recursion is induction as code, and decomposition is judgment.
Write a paragraph connecting all four to Chapter 6's claim that a language's job
is to extend what a person can hold in mind.
