# Finding the Seams

Here is a method that works:

```java
static void report(int[] scores) {
    int total = 0;
    for (int s : scores) total += s;
    double mean = (double) total / scores.length;

    int max = scores[0];
    for (int i = 1; i < scores.length; i++)
        if (scores[i] > max) max = scores[i];

    int min = scores[0];
    for (int i = 1; i < scores.length; i++)
        if (scores[i] < min) min = scores[i];

    System.out.println("Count: " + scores.length);
    System.out.println("Mean:  " + mean);
    System.out.println("Range: " + min + " to " + max);
}
```

Nothing is wrong with it. It compiles, it runs, it prints the right numbers, and
if you found it in a codebase you would have no complaint.

It is also asking to be divided — and here is the part I want you to notice — it is
*telling you where*. Go back and look at it before reading on. Decide where you
would cut, and hold on to your answer.

## The signals it is giving you

**Blank lines.** Look at where they fall. This is almost certainly where you cut,
and it is worth understanding why you were right.

You did not put those blank lines there for decoration. You put them there while
writing, to separate ideas, because your hand knew the ideas were separate before
your attention did. Each group a blank line separates is a candidate method. It is
the most reliable signal available and it costs nothing whatever to read.

**Comments that label a section.** A comment saying `// compute the mean` above
five lines is a name looking for a method. Replace it with `double mean =
mean(scores);` and the comment becomes unnecessary, which is the point.

**Repetition.** The `max` and `min` loops here are the same loop with one operator
changed. Repetition is not always worth removing, but it is always worth noticing.

**Indentation depth.** Three levels of nesting usually means the inner part is a
separate operation. Chapter 9 said this about nested loops and it generalizes.

**Difficulty naming.** If you cannot name a method without using the word "and",
it is doing several things. That is Chapter 11's test — run backwards, as a
divining rod for where the seams are.

**Comments explaining how.** A comment explaining *what* a section does is a
method name. A comment explaining *why* is genuine information and should stay.

## Applying them

The blank lines divide `report` into four parts: mean, maximum, minimum, and
printing. Three of them are computations with obvious names.

```java
static double mean(int[] a) {
    int total = 0;
    for (int x : a) total += x;
    return (double) total / a.length;
}

static int largest(int[] a) {
    int max = a[0];
    for (int i = 1; i < a.length; i++)
        if (a[i] > max) max = a[i];
    return max;
}

static int smallest(int[] a) {
    int min = a[0];
    for (int i = 1; i < a.length; i++)
        if (a[i] < min) min = a[i];
    return min;
}

static void report(int[] scores) {
    System.out.println("Count: " + scores.length);
    System.out.println("Mean:  " + mean(scores));
    System.out.println("Range: " + smallest(scores) + " to " + largest(scores));
}
```

That is more lines than we started with. Count them if you like: the file grew.

It is better anyway, and I would rather be precise about why than assert it
at you, because "smaller methods are better" is advice that has done real damage
when followed without a reason.

**`report` now reads as what it does.** Three lines, each naming an idea. A reader
who wants to know how the mean is computed can go and look; a reader who wants to
know what the report contains is finished.

**The pieces are independently testable.** `mean` can be checked against known
inputs without producing a report. Section 14.2 does exactly that.

**The pieces are reusable**, which is the least important benefit and the one most
often cited.

**Each piece can be fixed once.** The empty-array bug in `largest` — Chapter 10's
`a[0]` on an empty array — now has one place to be fixed rather than being spread
through a longer method.

## What was lost

Honesty about the cost.

**There are four things to find instead of one.** In one file this is trivial;
across a large codebase it is a real navigational burden.

**The relationship is less visible.** In the original, you could see that all three
statistics walk the same array. Now that is implied rather than shown.

**Three passes instead of one.** Check this one before you believe it: the
original walked `scores` three times as well, so nothing actually changed here. But
a decomposition *can* cost you performance by preventing a single combined pass,
and it is worth knowing that the cost is real even when this example does not pay
it. Chapter 32 gives you the tools to judge when it matters. It usually does
not.

## The judgment

The signals tell you *where* a division is available. They do not tell you whether
to take it, and that is a separate question.

The test I find most useful: **would a reader of the calling code be better off
seeing a name, or seeing the steps?**

For `mean`, a name is better — everyone knows what a mean is, and the loop is
noise at the call site.

For a three-line fragment that does something specific to this one situation, the
steps may be clearer than a name nobody will recognize. Extracting
`incrementCounterAndCheckThreshold` helps nobody.

Both failures are real, and you should be equally afraid of them. Under-decomposed
code is a wall of statements you have to read all of. Over-decomposed code is a
maze of tiny methods in which answering any question means following a chain of
four calls to find the two lines that matter.

You will be warned about the first constantly and about the second almost never. I
have seen the second do just as much damage.

Next: what "one job" means.
