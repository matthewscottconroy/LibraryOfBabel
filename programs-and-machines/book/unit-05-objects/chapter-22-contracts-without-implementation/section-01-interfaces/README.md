# Interfaces

Chapter 21 got substitutability by inheriting, which takes the implementation
along with the type. Often you want only the first half.

`String` and `LocalDate` can both be sorted. They share no ancestor worth speaking
of and not a line of code, and yet one sorting method works on both. Whatever makes
that possible cannot be inheritance in Chapter 21's sense, and it is the most
important construct in this chapter.

We begin with the interface itself — a set of method signatures with no bodies at
all, a type that wholly unrelated classes can share, and what Java 8 changed when
it allowed some of those signatures to come with code after all.

Then abstract classes, which take a middle position: some implementation supplied,
the rest left open, and a genuine question about when to reach for which.

And then a convention about how you declare your variables that has been sitting in
front of you, unexplained, since Chapter 17. You have written
`List<String> names = new ArrayList<>()` and wondered why the two halves of that
line disagree with each other. By the end of the section they will not look like a
disagreement.
