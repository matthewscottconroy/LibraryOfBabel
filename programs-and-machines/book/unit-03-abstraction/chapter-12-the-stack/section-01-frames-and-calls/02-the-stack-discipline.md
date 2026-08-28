# The Stack Discipline

You are reading a sentence in a book and it mentions a word you do not know. You
stop, look the word up, and the dictionary entry uses a second word you do not
know. You look *that* up. Now finish something.

You cannot finish the sentence first. You cannot even finish the first dictionary
entry first. The only thing you can finish is the second lookup — the one you
started most recently — and finishing it lets you finish the one before it, and
so on back to the sentence.

Nobody taught you that rule. It falls out of the situation: interruptions nest,
and nested things must be completed inside out. Method calls are interruptions of
exactly this kind, which is why the machine can store them in the simplest
possible structure.

A **stack** is a collection with one rule: the last thing added is the first thing
removed. Add is called **push**, remove is called **pop**.

The physical image is a stack of plates. You add to the top and take from the
top. There is no operation for reaching the middle.

## Why calls need exactly this

Because calls nest, and nesting is a last-in-first-out pattern.

If `main` calls `report`, and `report` calls `mean`, and `mean` calls `sum`, then:

```
main starts
  report starts
    mean starts
      sum starts
      sum finishes      ← first to finish
    mean finishes
  report finishes
main finishes           ← last to finish
```

The most recently started is always the first to finish. Push a frame on call,
pop it on return, and the frames are always in the right order automatically. No
searching, no bookkeeping — the structure of the problem supplies the structure of
the solution.

## The stack at a moment

While `sum` is running:

```
┌─────────────────┐  ← top
│ sum             │
├─────────────────┤
│ mean            │
├─────────────────┤
│ report          │
├─────────────────┤
│ main            │
└─────────────────┘  ← bottom
```

Only the top frame is executing. The others are **suspended** — each stopped at a
call, each holding its variables and its return address, each waiting for the one
above to finish.

That is the picture to keep. At any instant, one method is running and the rest of
the stack is a record of how the program got there.

## A stack trace is the stack

Now Chapter 10's error messages make complete sense:

```
Exception in thread "main" java.lang.ArrayIndexOutOfBoundsException: Index 3 out of bounds for length 3
	at Boom.sumTo(Boom.java:9)
	at Boom.main(Boom.java:5)
```

That is a **printout of the stack** at the moment of failure. Top line is the top
frame — the method that was executing. Below it, its caller, and so on to `main`
at the bottom.

Which is why Chapter 10's reading rule works: the topmost frame is where it broke,
and each line below is a suspended call waiting for the one above. The trace is
not a summary someone wrote; it is the data structure, printed.

And clicking up a frame in a debugger to inspect the caller's variables works
because those variables are still there — the frame is suspended, not gone.

## Why the stack is fast

Allocating a frame is moving one number.

The machine keeps a **stack pointer** — a register holding the address of the top.
To push a frame, subtract its size from the pointer. To pop, add it back. That is
one arithmetic operation per call, with no searching for free space and no
bookkeeping about what is in use.

Compare the heap, where objects live: allocating there means finding a suitable
free region, and freeing means eventually determining that nothing refers to a
region any more. Both are far more work.

This is why method calls are cheap enough to use freely, and it is worth knowing
when someone tells you that breaking code into small methods is slow. It is not,
and Chapter 5's JIT compiler frequently removes the call entirely by inlining it.

## The stack is finite

Fast allocation comes from a fixed region reserved in advance. Reserved in
advance means bounded.

The default on a typical JVM is around half a megabyte to a megabyte per thread.
Frames are small — tens of bytes — so you get room for something in the order of
ten thousand nested calls. On the machine this book was written on, an
empty-bodied infinite recursion reached a depth of about 22,000 before running
out.

For ordinary code this is enormous. Call depth in normal programs is tens, not
thousands.

For recursion it is a real limit, and it is the subject of the next lesson.

## Two regions, two purposes

Worth tabulating, because the distinction organizes a lot:

| | stack | heap |
|---|---|---|
| holds | call frames | objects |
| lifetime | tied to calls | until nothing refers to it |
| allocation | move a pointer | find space |
| freeing | move a pointer back | garbage collection |
| size | fixed, small | large, growable |
| ordering | strict last-in-first-out | none |

A local variable of primitive type lives on the stack. An object lives on the
heap, and the local variable holds a reference to it.

Which is the fact the whole next section turns on.
