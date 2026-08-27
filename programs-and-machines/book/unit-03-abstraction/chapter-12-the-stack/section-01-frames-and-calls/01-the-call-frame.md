# The Call Frame

When a method is called, the machine needs somewhere to keep that call's
information. That place is a **call frame**, sometimes called a stack frame or an
activation record.

## What is in one

A frame holds everything belonging to *one execution* of *one method*:

**Parameters**, holding copies of the arguments.

**Local variables** declared inside the method.

**The return address** — where in the caller to resume.

**Working space** for intermediate values, which on the JVM is the operand stack
of Chapter 5.

Chapter 6 called all of this state. A frame is the state belonging to one call,
and the "where to resume" part is the program counter's value, saved.

## One execution, not one method

That phrase matters and is easy to slide past.

If `largest` is called three times, there are three frames — one after another,
or several at once if the calls are nested. Each has its own parameters and its
own locals, and they do not interact.

This is why Chapter 7 said a local variable is fresh on every call and remembers
nothing:

```java
static void count() {
    int n = 0;
    n++;
    System.out.println(n);      // always 1
}
```

Each call gets a new frame with a new `n`. The previous frame is gone.

It is also what makes recursion possible. In Chapter 13 a method will call
itself, and the two executions will need separate variables despite being the same
method — which works because frames belong to executions.

## Watching it happen

```java
public static void main(String[] args) {
    int x = 5;
    int y = triple(x);
    System.out.println(y);
}

static int triple(int n) {
    int result = n * 3;
    return result;
}
```

Before the call, one frame:

```
┌─────────────────────────┐
│ main                    │
│   args = ...            │
│   x = 5                 │
│   y = (not yet set)     │
└─────────────────────────┘
```

Calling `triple(x)` creates a second frame on top. `n` receives a *copy* of 5:

```
┌─────────────────────────┐
│ triple                  │
│   n = 5                 │
│   result = (not set)    │
│   return to: main, line 3│
├─────────────────────────┤
│ main                    │
│   x = 5,  y = ?         │
└─────────────────────────┘
```

`triple` runs, setting `result` to 15. Then `return result` does two things: it
hands 15 back, and it **discards the frame**.

```
┌─────────────────────────┐
│ main                    │
│   x = 5,  y = 15        │
└─────────────────────────┘
```

`n` and `result` no longer exist. They were not saved, moved, or cleaned up
later; the space they occupied is available for the next call.

## Why the frame is discarded

This is the part worth appreciating.

Method calls **nest**: if `a` calls `b` and `b` calls `c`, then `c` finishes
first, then `b`, then `a`. There is no way for `b` to outlive `a` — it was called
by `a` and returns to it.

Last in, first out. Which is exactly the discipline of a stack, and it is not a
design choice so much as a recognition: given that calls nest, a stack is the
structure the situation already has.

That is why local variables cannot outlive their method. The space is reclaimed
the instant the frame is popped, and anything still pointing at it would be
pointing at space about to be reused. Languages that let you take the address of a
local — C among them — make it possible to hold such a pointer, and the resulting
bug is spectacular and hard to find. Java removes the possibility by not letting
you have addresses at all.

## Where objects live instead

A preview, because it explains something otherwise puzzling.

If frames are discarded on return, how can a method create an object and return
it? The object was made during the call; should it not vanish with the frame?

Java uses two regions. The **stack** holds frames — small, fixed-size, discarded
in order. The **heap** holds objects, and is not tied to call structure at all. A
local variable in a frame holds a *reference* to a heap object, and returning that
reference lets the object outlive the frame that made it.

An object is freed when nothing refers to it any more, which is the garbage
collector's job and has nothing to do with any method returning. That is Chapter
7's remark that scope and lifetime diverge for objects, now with a mechanism
attached.

Next, the discipline itself, and what a stack trace has been telling you all
along.
