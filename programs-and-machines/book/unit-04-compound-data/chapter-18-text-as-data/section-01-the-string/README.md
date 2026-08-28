# The String

`String` is the type you have used most and thought about least, and it has a
property you have already tripped over: nothing you do to a string changes it.

`s.toUpperCase();` on its own line does nothing at all. That is not a bug, it is
not an oversight, and the reason behind it is worth four separate benefits — which
is why the type is designed that way despite the cost.

Three lessons.

Immutability first — what it means, what it costs, and the four distinct benefits
that make it worth the cost. Then building text, where immutability's cost is
real and `StringBuilder` is the answer, with a measurement showing the difference
is not small. Then comparison, which is where the `==` trap lives and where Chapter
4's Unicode material returns with consequences.
