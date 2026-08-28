# When Are Two Things the Same?

Java offers two ways to ask whether two things are the same. They look almost
identical, and choosing wrongly gives you a program that passes on your test data
and fails on real data — the worst failure mode there is, since testing does not
catch it.

The difficulty is not the syntax. It is that "the same" is genuinely ambiguous,
and the language cannot settle the ambiguity for you, because the answer depends on
what you are modeling.

Three lessons, and the chapter's reason for existing.

`==` against `.equals` first: two operators asking two genuinely different
questions, and the damage done by using the first when you meant the second. Then
the `equals`/`hashCode` contract — a set of rules the collections depend on, which
Java cannot check and which is broken in real code constantly. Then immutability,
which is not a fourth topic but the strategy that makes most of the first two stop
mattering.
