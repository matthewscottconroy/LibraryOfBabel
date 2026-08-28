# Designing for Failure

The syntax was the easy part. What decides whether a program is bearable to operate
at three in the morning is a judgment nothing checks: *where* a failure gets
handled.

The answer is nearly always further up than it feels like it should be, and this
section is mostly about why catching an exception is more often a mistake than
not catching one.

Three lessons, and they are about judgment rather than syntax.

Where a failure should be caught, which is almost always further up than it is.
How to guarantee that a resource is released when something has gone wrong. And
the argument for failing immediately and audibly rather than continuing in a state
you do not understand.
