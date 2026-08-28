# Interfaces

Chapter 21 got substitutability by inheriting, which takes the implementation
along with the type. Often you want only the first half.

`String` and `LocalDate` can both be sorted. They share no ancestor worth speaking
of and not a line of code, and yet one sorting method works on both. Whatever makes
that possible cannot be inheritance in Chapter 21's sense, and it is the most
important construct in this chapter.

Three lessons.

The interface itself: a set of method signatures with no bodies, a type that
several unrelated classes can share, and what Java 8's default methods added.
Then abstract classes, which supply some implementation and leave the rest open,
and the question of which to reach for. Then the convention that governs how you
declare variables, which is Chapter 17's `List<String> names = new ArrayList<>()`
finally explained.
