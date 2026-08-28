# Many Things at Once

Suppose I hand you a hundred numbers and ask you to keep hold of them.

You know how to keep hold of one. A name, a type, a value. Do it a hundred times
and you have `score0` through `score99`, which is not a solution so much as a
confession — because the moment I ask for the average you need to mention all
hundred names in one expression, and the moment I make it a thousand numbers you
have to go back and edit the program. A program that must be rewritten when the
amount of data changes is barely a program at all.

What you want is a single name for the whole collection, and a way of saying
*which one* that is itself computable. Say the forty-seventh, without having
written the word forty-seven anywhere in advance.

That is an **array**: a fixed-size sequence of values of the same type, stored
consecutively, and reachable by position.

Every clause in that sentence is load-bearing, and this chapter takes them one at a
time — why fixed size, why the same type, why consecutive, and what "by position"
costs as well as what it buys.

If you take one idea from this chapter, take this: **an array index is
arithmetic.**

When you write `scores[47]`, the machine does not search. It does not compare 47
against anything, or walk along counting. It multiplies:

```
address of scores[47]  =  address of scores[0]  +  47 × (size of one element)
```

One multiplication and one addition, and the element is there. The same cost for
element 47 as for element 4,700,000.

That property — **constant-time access by position** — is the reason arrays exist,
and it is bought by the two constraints people find restrictive. Elements must be
the same type, so they are the same size, so the multiplication works. And the
storage must be consecutive, so the addition works.

Chapter 1 said fixed width buys constant-time addressing, and Chapter 15 is where
that promise is collected.

**The Array** covers declaring, filling, and accessing; the index-as-arithmetic
argument; and bounds checking — a run-time check Java performs on every access,
which costs something and prevents an entire category of security defect.

**Arrays of Arrays** covers two-dimensional data, which in Java is not really
two-dimensional, and the traversal patterns you will use constantly.

A word for people who have programmed before. Arrays are usually taught early and
mechanically: here is the syntax, here is a loop, remember that indices start
at 0.

That treatment leaves you able to use them and unable to reason about them. Why
*do* indices start at 0? Why can an array not grow? Why is `int[]` fast and
`ArrayList<Integer>` slower? Why does copying an array of arrays not copy the
inner arrays?

Every one of those has an answer that follows from the paragraph above, and
knowing the answers is what makes Chapter 17's choices between collection types
comprehensible rather than arbitrary.
