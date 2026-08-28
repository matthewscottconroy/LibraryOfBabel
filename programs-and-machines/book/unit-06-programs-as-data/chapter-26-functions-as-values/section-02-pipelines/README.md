# Pipelines

Look back at the loops you have written in this book. Nearly all of them do one of
three things: transform each element, keep some elements, or boil everything down
to a single value.

Three shapes, and a large fraction of all the loops anyone writes. They have names,
they were named in Lisp in the 1960s, and learning to see them is worth more than
the Java syntax for them — which is why this section does them by hand first.

Three lessons.

Map, filter and reduce first, as concepts, with hand-written loops — because the
operations are worth understanding before the syntax arrives. Then Java's streams,
which is that syntax plus laziness, short-circuiting, and collectors. Then the
counterweight: when a loop is better, with measurements rather than opinions.
