# Documentation and Style

Chapter 11 said a method is a contract and that contracts live in comments.
Chapter 14 said comments can become wrong without telling you. Both are true, and
this appendix is about writing the comments that are worth having and formatting
the code around them.

## Javadoc

Java has a documentation format the tooling understands. A comment beginning
`/**` before a declaration is a **doc comment**:

```java
/**
 * Returns the largest value in the given array.
 *
 * <p>The array is not modified. Comparison uses natural ordering, so
 * for an array of all-negative values the result is the value closest
 * to zero.
 *
 * @param values a non-null array with at least one element
 * @return the largest element of {@code values}
 * @throws IllegalArgumentException if {@code values} is empty
 * @throws NullPointerException if {@code values} is null
 */
public static int largest(int[] values) { ... }
```

The `javadoc` tool turns these into HTML. The API documentation you look things
up in is generated exactly this way from the library's own source.

## The tags

| tag | for |
|---|---|
| `@param name` | one per parameter, describing what it must be |
| `@return` | what comes back; omit for `void` |
| `@throws Type` | each exception and the condition that causes it |
| `@see` | a related class or method |
| `@since` | the version it appeared in |
| `@deprecated` | why, and what to use instead |
| `{@code x}` | inline code, escaping angle brackets |
| `{@link Type#method}` | a cross-reference |

Conventions that matter:

**The first sentence is a summary.** It is extracted for index pages, so it must
stand alone. Start with a verb in the third person — *Returns the largest value*
— not *This method returns* and not *Return*.

**`@param` describes the contract, not the type.** The type is already in the
signature. `@param values a non-null array with at least one element` says
something; `@param values the array` says nothing.

**Document what you decided.** Chapter 11's rule: if you had to think about an
edge case, write down what you concluded. That is the part a reader cannot
recover from the code.

## What not to comment

The most common mistake is commenting *what* instead of *why*:

```java
// increment i
i++;

// loop through the array
for (int x : a) { ... }
```

These restate the code. They cost a line, they can drift out of date, and they
train readers to skip comments — which means the one comment that mattered gets
skipped too.

Comment the things the code cannot say:

```java
// The API returns results one page behind during a rebuild, so we
// retry once before treating an empty page as the end.
if (page.isEmpty() && !retried) { ... }
```

That is information. No amount of reading the code recovers it.

The test: **if the comment would still be true after a rewrite, it is probably
about intent and worth keeping. If it describes the current lines, delete it and
name things better.**

## Naming

Repeating Chapter 7, because it is the highest-value habit here.

```
variables, methods    camelCase          totalScore, findLargest
classes, interfaces   PascalCase         BankAccount, Comparable
constants             UPPER_SNAKE_CASE   MAX_ATTEMPTS
packages              lowercase          com.example.billing
type parameters       single capital     T, E, K, V
```

Beyond convention:

**Names should say what the thing is.** `daysUntilExpiry` beats `d`. You read code
more than you write it, and the reader lacks your context.

**Short names for short scopes.** `i` for a loop index over four lines is clearer
than `currentIndex`, because every programmer reads `i` instantly. The further a
name travels, the more it must carry.

**Booleans read as questions.** `isValid`, `hasNext`, `canRetry`. So that
`if (isValid(x))` is a sentence.

**Methods returning a value are named for the value; methods causing an effect
are named for the action.** Chapter 11's rule, and a `getBalance` that opens a
network connection betrays every reader.

**Avoid abbreviations that are not universal.** `req`, `res`, `tmp`, `idx` are
widely understood. `acctMgrSvc` is not.

## Formatting

The compiler does not care. Consistency matters, and being consistent with the
wider world matters more than any individual choice.

Two published guides are in common use: the Google Java Style Guide, and the
older Sun/Oracle Code Conventions. Pick one, configure your editor to apply it,
and stop thinking about the questions it settles.

Points where nearly everyone agrees:

**Braces on the same line.** `if (x) {`, not a brace on its own line. This is the
Java convention even though C and C# often differ.

**Always use braces**, even for one statement. Chapter 8 gave the reason and the
security defect that made it famous.

**One statement per line.**

**Indent consistently** — four spaces is the Java norm, and tabs versus spaces is
settled by your project, not by you.

**Keep lines under about 100 characters.** Long lines force horizontal scrolling
in side-by-side diffs, which is where a lot of code gets read.

**A blank line separates ideas.** Chapter 14 pointed out that these are also the
seams where a method wants to divide, which makes them doubly worth placing
deliberately.

## Let the tools do it

Modern practice is to stop arguing and automate.

**Formatters** — `google-java-format`, or your IDE's built-in — reformat source
mechanically. Run one on save and formatting stops being a topic in code review.

**Linters** — Checkstyle, PMD, SpotBugs, or an IDE's inspections — flag
suspicious patterns: unused variables, `==` on strings, missing `equals`
alongside `hashCode`. Chapter 18's `==` trap is exactly the sort of thing a
linter catches for free.

Both belong in a build so they run without anyone remembering.

## Why any of this matters

One argument, and it is Chapter 6's.

A language's constructs exist to extend what a person can hold in mind. So does
formatting, so does naming, so does a comment that says why. None of it changes
what the machine does. All of it changes how much of the program a human can keep
in view at once, and that is the constraint that actually binds.

The corollary is that these are not matters of taste dressed up as rules. A
misleading name costs someone an hour. A comment that has drifted out of date
costs someone a wrong assumption. Both are real, and both are cheap to prevent.
