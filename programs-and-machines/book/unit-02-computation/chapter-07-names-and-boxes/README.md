# Names and Boxes

The last chapter said a variable is a named piece of state. That is true and it
hides two ideas, both of which cause trouble.

The first is that a name and a value are different things. When you write
`x = 5`, there is a name `x`, there is a value 5, and there is a relationship
between them — and the relationship can change without either the name or the
value changing. This sounds like hair-splitting until Chapter 20, when two names
turn out to refer to the same object and changing one changes the other. The
groundwork is laid here.

The second is that `=` does not mean equals. It means *assign*, which is an
action rather than a claim, and the difference is the reason `n = n + 1` is
sensible rather than absurd. If you have any mathematics in your background, that
line is genuinely offensive on first sight, and getting clear about why it is not
a contradiction is worth the page it takes.

The chapter also covers where names live and how long they last, which sounds
like bookkeeping and is actually about limiting how much you have to think about
at once. A name that exists everywhere is a name that could be changed by
anything, and a program made of those is a program nobody can reason about.

**Naming a Value** covers what a variable actually is in terms of Chapter 6's
state, why assignment is not equality, and what a type declaration promises.

**Where a Name Lives** covers scope and lifetime, and then mutation — the fact
that a variable can change, which is the whole point of variables and also the
source of most of the difficulty in the rest of the book.

Chapters 6 was abstract. This one is concrete and mostly straightforward, and if
you have programmed before you will find much of it familiar.

Read the familiar parts anyway, and read them for the connection to Chapter 6 —
what state does this construct create, and what transition does it perform? That
framing is what makes Unit V's harder material tractable later, and it is
invisible if you only learn the syntax.
