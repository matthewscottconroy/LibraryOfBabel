# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## Truth tables

**8.1.** Write the truth table for `A && !B`.

**8.2.** Write truth tables for `!(A || B)` and `!A && !B` and confirm the
columns match. Which law is this?

**8.3. [carries forward]** Show that `A XOR B` equals `(A || B) && !(A && B)` by
truth table.

**8.4.** There are 16 possible two-input boolean operators. We named AND, OR,
XOR, and NAND. Write the truth tables for three others and describe each in
words.

**8.5.** Build OR from NAND alone and verify with a truth table. (Section 8.1.1
gives the formula; check it rather than trusting it.)

## Gates and arithmetic

**8.6.** Draw the half adder and trace all four input combinations, giving sum
and carry for each.

**8.7. [carries forward]** The full adder's carry-out is
`(A && B) || ((A XOR B) && carryIn)`. Build its truth table for all eight input
combinations and confirm it matches the carry you would produce on paper.

**8.8.** Chaining 32 full adders gives a 32-bit adder. Explain, in terms of that
circuit, why `Integer.MAX_VALUE + 1` is negative and why no exception is raised.

**8.9.** A multiplexer is `(S && B) || (!S && A)`. Confirm by truth table that it
outputs `A` when `S` is false and `B` when `S` is true.

## Rewriting conditions

**8.10.** Apply De Morgan to each, and simplify the comparisons:
- `!(x > 5 && y < 3)`
- `!(a == b || c != d)`
- `!(!p && q)`

**8.11.** Simplify `(A && B) || (A && !B)` using the laws, showing each step and
naming the law used.

**8.12.** Rewrite so no negation applies to a compound expression:
```java
if (!(user != null && user.isActive())) { deny(); }
```

**8.13.** A specification says: "reject applicants who are under 18, and
applicants without a license." Write the condition. Explain why the "and" in the
sentence becomes an `||` in the code.

## Java

**8.14.** Predict the output, then run:
```java
int x = 5;
if (x > 3)
    System.out.println("a");
    System.out.println("b");
```

**8.15.** Explain why this throws, and fix it by reordering:
```java
if (s.length() > 0 && s != null) { }
```

**8.16.** Predict which of `loud("A")` and `loud("B")` run in each line, where
`loud` prints and returns false:
```java
boolean p = loud("A") && loud("B");
boolean q = loud("A") | loud("B");
boolean r = loud("A") || loud("B");
```

**8.17.** Rewrite this with guard clauses:
```java
if (a != null) {
    if (a.isReady()) {
        if (a.count() > 0) {
            process(a);
        }
    }
}
```

**8.18.** Given the old-style `switch` in Section 8.2.3 with the `break` removed
from `case 1`, what is `name` when `day` is 1? Explain.

## Going further

**8.19.** Section 8.2.2 says `&&` is "control flow wearing a logical operator's
clothes". Boolean algebra says `A && B` equals `B && A`. Reconcile these: in what
sense is the law true, and in what sense does Java violate it?

**8.20.** Switching on an enum without a `default` stops compiling when a new
constant is added. Argue that this is a feature. Then argue the opposite. Which
do you find more convincing, and does your answer depend on the size of the
codebase?

**8.21.** Shannon's 1937 insight was that Boole's 1854 algebra describes
switching circuits. Explain what this bought that was not available before, in
terms of what an engineer could do with a circuit design.
