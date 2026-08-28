# Threads

This is the hardest material in the book, and it is worth saying why rather than
letting you find out.

Everything you know about reading a program assumes two things: that statements
happen in the order they are written, and that a value you read is the value that
was put there. With more than one thread, neither holds. What you get instead is a
class of bug that does not reproduce, does not appear under a debugger, passes
every test, and fails in production once a week.

Three lessons.

Why concurrency exists at all, which is a story about processors rather than about
software. Then shared state and races, demonstrated at a scale where the failure is
impossible to miss. Then locks and the alternatives, with the costs measured and
the two classic failures named.
