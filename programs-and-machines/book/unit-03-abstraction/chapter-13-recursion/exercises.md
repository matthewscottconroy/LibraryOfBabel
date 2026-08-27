# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## Base cases

**13.1.** For each, name the smallest input and the answer for it:
summing an array; reversing a string; counting nodes in a tree; raising *x* to
the power *n*.

**13.2.** What is wrong with each?
```java
static int f(int n) { return n * f(n - 1); }
static int g(int n) { if (n == 0) return 1; return n * g(n); }
static int h(int n) { if (n == 0) return 1; return n * h(n + 1); }
```

**13.3. [carries forward]** Write `static int power(int base, int exp)`
recursively. State the base case, and say what your method does for a negative
exponent.

**13.4.** `factorial(13)` returns 1932053504, which is wrong. Explain why, and
say whether the fault is in the recursion. What would you change?

## Trusting

For each, write the correctness argument in two sentences — one for the base
case, one assuming the recursive call is correct. Do not trace.

**13.5.**
```java
static int count(int[] a, int from, int target) {
    if (from == a.length) return 0;
    return (a[from] == target ? 1 : 0) + count(a, from + 1, target);
}
```

**13.6.**
```java
static boolean isPalindrome(String s) {
    if (s.length() <= 1) return true;
    if (s.charAt(0) != s.charAt(s.length() - 1)) return false;
    return isPalindrome(s.substring(1, s.length() - 1));
}
```

**13.7. [carries forward]** Write a recursive method that returns the largest
element of an array, and give its two-sentence argument.

**13.8.** Section 13.1.2 suggests imagining the recursive call as a colleague's
correct method. Apply that framing to `reverse`, in writing.

## Induction

**13.9.** Prove by induction that 1 + 3 + 5 + … + (2*n*−1) = $n^{2}$. Then write
the corresponding recursive method.

**13.10.** Fill in the correspondence for `power` from 13.3: what is the base
case, the induction hypothesis, and the inductive step?

**13.11.** `gcd(a, b)` calls `gcd(b, a % b)`. The second argument does not
decrease by one. Explain why termination still holds, and which form of induction
this corresponds to.

## Cost

**13.12.** `fib(30)` makes 2,692,537 calls. Estimate the calls for `fib(35)`,
given that each increment multiplies the count by roughly 1.6.

**13.13.** Explain why `size(tree)` makes two recursive calls per node and is
still efficient, while `fib(n)` makes two and is not.

**13.14. [carries forward]** Rewrite `fib` to be linear, in two different ways:
by memoizing, and by working upwards. Which uses less space?

**13.15.** For each, say whether the process is recursive or iterative, and why:
```java
static int a(int n) { return n <= 1 ? 1 : n * a(n - 1); }
static int b(int n, int acc) { return n <= 1 ? acc : b(n - 1, n * acc); }
static int c(int n) { return n <= 1 ? 1 : c(n - 1) + c(n - 2); }
```

**13.16.** Java does not eliminate tail calls, so `b` above still uses *n* frames.
Given that, is there any reason to write it? Answer honestly.

## Judgment

**13.17.** For each, say whether you would use recursion or a loop, and why:
summing a list; walking a directory tree; finding a maximum; parsing nested
brackets; computing a running average; solving a maze.

**13.18.** Section 13.2.3 calls the naive Fibonacci "a small scandal" as a
teaching example. Argue for keeping it; argue for dropping it. Which do you find
more convincing?

**13.19.** Write a recursive method that prints every file under a directory.
Then describe, without writing it, what the iterative version would need — and
what data structure you would have to maintain yourself.
