# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## Writing methods

**11.1.** Write `static int cube(int n)` and call it from `main` with three
values.

**11.2.** Write `static boolean isEven(int n)`. Then write `isOdd` in one line
that uses it.

**11.3. [carries forward]** Write `static int countDigits(int n)` returning the
number of decimal digits in a positive `n`. State its precondition. What does
your method do for 0, and is that what you want?

**11.4.** Write `static String repeat(String s, int times)` without using
`String.repeat`. What should it do when `times` is 0? Negative?

**11.5.** Write `static int smallest(int[] a)` as the counterpart to `largest`.
Give its contract in the two-line Requires/Ensures form.

## Reading contracts

**11.6.** Write Requires/Ensures for each:
- `static double divide(int a, int b)`
- `static char firstLetter(String s)`
- `static int[] sorted(int[] a)`

**11.7.** `static int indexOf(int[] a, int target)` returns −1 when the target is
absent. Is −1 an acceptable sentinel here? Justify using Section 11.2.2's rule.

**11.8. [carries forward]** A method is documented as *Requires: the array is
sorted*. Give two reasons it might demand this rather than checking it, and one
situation where checking would be the better choice.

**11.9.** Section 11.2.1 says a precondition is "a way of not handling a case".
Explain this using `largest` and the empty array.

## Judgment

**11.10.** For each, say whether it should be one method or several, and why:
- `readFileAndParseAndValidate`
- `printReport`
- `calculateTotalAndUpdateDatabase`
- `isPrime`

**11.11.** Apply the one-sentence-without-"and" test to three methods you have
written. Rewrite any that fail.

**11.12.** A method takes six parameters, four of them `int`. Name two distinct
problems this creates, and two ways to improve it.

**11.13.** Which of these are pure? For the impure ones, say what the effect is.
```java
static int square(int n)
static void print(String s)
static int nextRandom()
static String upper(String s)
static boolean deleteFile(String path)
```

**11.14. [carries forward]** Section 11.1.3 advises against a method that both
causes an effect and returns an interesting value. Find such a method in a
library you use, and say what makes it awkward.

## Going further

**11.15.** Rewrite this as three methods with good names, and say what each one's
contract is:
```java
static void run(int[] data) {
    int total = 0;
    for (int x : data) total += x;
    double mean = (double) total / data.length;
    double sq = 0;
    for (int x : data) sq += (x - mean) * (x - mean);
    System.out.println(Math.sqrt(sq / data.length));
}
```

**11.16.** Section 11.1.1 says a wrong abstraction is worse than none. Construct
a small example: a method whose name is subtly inaccurate, and a bug that results
from a reader trusting the name.

**11.17.** Chapter 9 had loop invariants; this chapter has contracts. State what
the two have in common in one sentence, and predict what the third form —
mentioned as coming in Unit IV — will be a claim about.

**11.18.** Assertions are disabled by default. Argue that this is correct, then
argue that it is a mistake. Which position do you hold, and does it depend on
what the assertion is checking?
