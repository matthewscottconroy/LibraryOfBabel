# Files and Streams

A file might be four bytes or four hundred gigabytes, and you frequently do not
know which until you open it.

That single awkwardness shapes the entire design of every I/O library in every
language — including the decision, which looks perverse at first, to offer you
*less* than a file can actually do.

Three lessons.

The stream abstraction first — a sequence of items arriving one at a time, which
is what every I/O library in every language is built on and why. Then text, where
encoding stops being theoretical. Then bytes and buffering, with a measurement
that explains an entire layer of the library.
