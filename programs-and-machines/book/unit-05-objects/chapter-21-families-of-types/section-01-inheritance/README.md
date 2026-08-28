# Inheritance

Ask a programmer what object orientation is and there is a good chance the answer
involves a dog that is a kind of animal. Inheritance has become the public face of
the subject, and this section is going to teach it carefully and then spend the
next two chapters arguing that its fame is undeserved.

Both halves are meant. The mechanism is genuinely useful and genuinely
over-applied, and you cannot judge which is which until you know what it does.

Three lessons.

Extension first: what `extends` does, what a subclass gets, and what it may add.
Then overriding and `super` — replacing a behavior while keeping access to the
one you replaced. Then the class every class inherits from, which is where
`toString`, `equals` and `hashCode` came from and why Chapter 20 had to override
them rather than invent them.
