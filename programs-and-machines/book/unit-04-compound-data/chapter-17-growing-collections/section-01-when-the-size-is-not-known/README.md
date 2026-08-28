# When the Size Is Not Known

Arrays have a fixed length, chosen when you create them. Almost nothing you
actually want to store has a length you know in advance.

That mismatch is the reason the collections library exists, and this section is
about the type that resolves it — plus, because it is worth seeing once, exactly
how it manages to grow something that cannot grow.

Three lessons.

`ArrayList` first: what it does and how to use it. Then how growth works, which is
worth a lesson because the mechanism explains a cost that would otherwise be
mysterious, and because it is a small piece of engineering worth admiring. Then
the three shapes — list, set, map — and the question each one answers.
