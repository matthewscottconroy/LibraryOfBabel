# When Are Two Things the Same?

Three lessons, and the chapter's reason for existing.

`==` against `.equals` first: two operators asking two genuinely different
questions, and the damage done by using the first when you meant the second. Then
the `equals`/`hashCode` contract — a set of rules the collections depend on, which
Java cannot check and which is broken in real code constantly. Then immutability,
which is not a fourth topic but the strategy that makes most of the first two stop
mattering.
