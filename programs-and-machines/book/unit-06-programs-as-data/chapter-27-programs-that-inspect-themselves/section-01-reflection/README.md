# Reflection

JUnit finds your test methods. Jackson turns your objects into JSON. Spring works
out which constructor to call.

None of those tools has seen your classes. They were written years before you wrote
your code, they contain no reference to any of your type names, and they work
anyway. Whatever lets them do that is the subject of this section — and it is also
capable of reading the private fields you spent Chapter 19 protecting.

Two lessons.

The mechanism first: `Class` as an object you can hold, the field and method
objects it hands out, invoking and constructing by name, and the access checks
that turn out to be optional. Then the costs — performance, safety, tooling — and
the rule about where reflection belongs.
