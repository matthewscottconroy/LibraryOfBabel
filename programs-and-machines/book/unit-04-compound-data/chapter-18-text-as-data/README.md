# Text as Data

Text has been present since Chapter 4, where we established what a character is
and how it becomes bytes. Since then strings have appeared in every program and we
have never treated them as data to be worked with.

This chapter does, and it is a good place for the unit to end, because a `String`
turns out to be a worked example of nearly everything the unit has said.

It is a **collection of values** — characters — with a fixed size once created,
so it is the array of Chapter 15. It is an **abstract data type**: its
representation changed underneath everyone in Java 9 without a line of source
changing anywhere, which is the strongest instance of Chapter 16's argument there
is. It is **immutable**, a design decision this chapter defends and Chapter 20
generalizes. And building one efficiently requires exactly the growth-by-doubling
of Chapter 17.

Text deserves a chapter because most programs are mostly text handling, and most
people learn it as a collection of remembered methods rather than as a subject
with a shape.

There is a shape. The chapter's two halves are it.

**The String** is about the type itself: why it cannot be changed, what that costs
and buys, how to build text without paying the cost, and how to compare text —
which is harder than it looks and where the `==` trap of Chapter 16 does its most
frequent damage.

**Parsing and Formatting** is about the two directions of a boundary. Text arrives
from files, users, and networks, and must become structured data. Structured data
must become text for display and for storage. Both are conversions between
representations, and Chapter 1's warning applies: a conversion is where information
is lost and where wrong assumptions surface.

`String` is **immutable**. Once created, its contents never change.

Every method that appears to modify a string returns a new one:

```java
String s = "hello";
s.toUpperCase();
System.out.println(s);      // hello — unchanged
```

That is the single most common early mistake with strings, and it follows directly
from immutability rather than being an arbitrary rule. The method could not have
modified `s`; there is no operation that modifies a string.

The rest of Section 18.1.1 is about why the language made that choice, and it is a
better decision than it first appears.
