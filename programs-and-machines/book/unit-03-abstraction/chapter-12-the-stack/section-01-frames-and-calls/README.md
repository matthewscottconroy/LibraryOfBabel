# Frames and Calls

You have been taking a great deal on faith.

That a method call leaves the caller's variables exactly where they were. That the
right value finds its way back to the right place, out of a method that may have
called four others on the way. That a method calling *itself* does not descend into
incoherence, with a hundred copies of `n` fighting over one name.

None of that is obvious. All of it has to be arranged by something.

Here is where the arrangement gets shown to you, and the pleasure of it is that
there is barely a design decision in sight. One data structure does the whole job,
and it is not chosen so much as *conceded* — the shape of the problem leaves almost
nothing else available.

We start with the packet of state belonging to a single execution of a single
method: its parameters, its locals, and the address it must return to when it is
finished. Then the rule for organizing those packets, which we will find is forced
rather than invented — given that calls nest, nothing else works. And finally what
happens when the packets run out of room, which is an error worth genuinely
understanding rather than merely recognizing on sight.
