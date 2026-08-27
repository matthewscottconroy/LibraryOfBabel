# Further Reading

**Joshua Bloch, *Effective Java*, third edition.** Item 20, "Prefer interfaces to
abstract classes," argues Section 22.1.2's comparison more carefully than there
was room for here. Items 34 through 38 are the best short treatment of enums
anywhere — particularly Item 34 on why the `int` constant pattern is worse than it
looks, and Item 36 on `EnumSet` as the replacement for bit fields.

**Gamma, Helm, Johnson, and Vlissides, *Design Patterns* (1994).** Read the
introduction, which contains both principles this chapter uses, and then the
Template Method and Strategy chapters, which are the two patterns most directly
about the interface-versus-abstract-class choice. Treat the rest as a catalogue to
consult, not a curriculum.

**Brian Goetz, JEP 395 (Records) and JEP 409 (Sealed Classes).** Short, readable,
and unusually explicit about the reasoning. JEP 395's "records are not
boilerplate reduction" section is worth reading twice; it changes what you think
records are for.

**The Java Language Specification, chapter 9 (Interfaces).** For the exact rules on
default method inheritance, which get genuinely subtle when a class implements two
interfaces that both supply a default for the same method. You will not need this
often, and when you do nothing else will answer the question.

**Brian Goetz, "Java Language Architect" talks on Project Amber.** Recorded
conference sessions in which the sealed-plus-record-plus-pattern-matching design
is explained as a whole rather than as three features. The best available account
of where Java is going and why.

**Alan Kay, "The Early History of Smalltalk" (1993).** Recommended again from
Chapter 21, and now for a different reason: read it after Section 22.1.1 and the
message-passing argument will land differently.
