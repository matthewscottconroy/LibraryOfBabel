# Polymorphism

Here is the payoff for the last section, and it is worth stating before the
machinery arrives.

You can write a loop today that correctly handles a kind of thing that will not be
invented until next year, by someone who has never read your loop. That is not a
figure of speech. It is a concrete property of how method calls are resolved, and
this section is about how it is possible and what it costs.

Three lessons.

First the idea: one name that means different things depending on what it is
called on, and why that removes the branching you would otherwise write. Then the
mechanism — how the JVM finds the right method at run time, and what it costs.
Then Liskov's principle, which is the test for whether a hierarchy is honest, and
a square that fails it.
