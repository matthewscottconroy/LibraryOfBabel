# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## Tracing

**10.1.** Build the state table for:
```java
int x = 2, y = 5, z = 0;
z = x + y;
x = z - x;
y = z - y;
```
What does it do?

**10.2. [carries forward]** Trace this with `n = 10`, one row per iteration.
State what it computes, and give the invariant.
```java
int n = 10, count = 0;
while (n > 0) { if (n % 2 == 1) count++; n = n / 2; }
```

**10.3.** Trace with `a = {4, 4, 4}`. Does the result surprise you? What does the
loop actually count?
```java
int c = 0;
for (int i = 0; i < a.length; i++)
    for (int j = 0; j < a.length; j++)
        if (a[i] == a[j]) c++;
```

**10.4.** Trace the swap without a temporary and show, in the table, the exact
row where information is lost.

## Desk checking

**10.5.** Desk check `largest` from Section 10.1.2 on: an all-negative array, a
one-element array, and an empty array. For each, say what happens and whether it
is acceptable.

**10.6. [carries forward]** Desk check this on `{}`, `{5}`, and `{3, 1}`. Find
two distinct defects.
```java
static double average(int[] a) {
    int total = 0;
    for (int i = 1; i < a.length; i++) total += a[i];
    return total / a.length;
}
```

**10.7.** Section 10.1.2 warns that you read what you meant rather than what you
wrote. Describe a way of tracing that reduces this, and say why it helps.

## Reading errors

**10.8.** For this trace, name the exception type, the offending values, the line,
and the call path. Then say what single character is probably wrong.
```
Exception in thread "main" java.lang.ArrayIndexOutOfBoundsException: Index 5 out of bounds for length 5
	at Stats.mean(Stats.java:22)
	at Stats.report(Stats.java:14)
	at Stats.main(Stats.java:6)
```

**10.9.** A stack trace is forty lines and only three name files you wrote. Which
frame do you look at first, and why?

**10.10.** For each, say whether it is a compile-time or a run-time failure, and
what kind of mistake it indicates:
- `cannot find symbol`
- `NullPointerException`
- `';' expected`
- `ArithmeticException: / by zero`
- `incompatible types: String cannot be converted to int`

## Bisecting

**10.11.** A pipeline of six stages produces a wrong answer. Describe, in order,
the checks you would make, and say how many you expect to need.

**10.12.** A program worked at commit 1 and fails at commit 128. How many
checkouts are needed to find the breaking commit by bisection? Show the
arithmetic.

**10.13. [carries forward]** A bug appears with a 10,000-line input file.
Describe how you would minimize it, and say what you would do when a removal
makes the bug disappear.

**10.14.** Why does bisection require you to have an expectation at each
checkpoint? Give an example where the absence of one makes the technique useless.

## Going further

**10.15.** Section 10.2.2 says bisection, minimization, and desk checking are the
same move. State that move in one sentence, and give a fourth setting where it
applies.

**10.16.** Chapter 6 said an intermittent bug is a machine behaving
deterministically on a state larger than you thought. Take a concrete
intermittent failure — a test that passes alone and fails in a suite — and list
four things that might be in the hidden state.

**10.17.** Section 10.2.3 says a debugger changes concurrent programs by
observing them. Explain the mechanism, and say what that implies about the kinds
of bug a debugger cannot help with.

**10.18.** Write the worst debugging session you can imagine, as a short
narrative — someone using the unproductive method for two hours. Then annotate it
with the point at which each technique from this chapter would have ended it.
