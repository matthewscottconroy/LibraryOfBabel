# Failure as a Value

Every method you have written so far has worked. Not because you are careful — the
inputs came from you, the data was in memory, and nothing outside the program was
consulted.

That ends here. A method that reads a file can be handed a file that is not there,
and it has to say something about that. This section is about what it says, and why
the three obvious answers all fail in the same way.

Three lessons.

First the problem: the ways of reporting failure that do not use exceptions, and
exactly how each one fails. Then the mechanism — throwing, catching, `finally`,
the stack trace, and chaining. Then Java's checked exceptions, which are unique
among mainstream languages and worth understanding as a decision with two sides.
