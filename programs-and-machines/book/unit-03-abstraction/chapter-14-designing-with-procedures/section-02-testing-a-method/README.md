# Testing a Method

You ran it, it printed the right thing, and you moved on. Then you changed
something nearby.

Is it still right? The honest answer, for most code most of the time, is a shrug —
and that shrug is why programs decay. This section is about writing the checking
down so the machine does the remembering.

Three lessons.

What a test actually is — an executable claim, which turns Chapter 11's contract
from a comment into something that fails when it stops being true. Then choosing
cases, which is where testing succeeds or fails, since a test suite that exercises
only the easy paths gives confidence without warrant. Then the observation that
tests are the most reliable documentation available, because unlike comments they
cannot quietly become wrong.
