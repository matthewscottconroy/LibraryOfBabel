# Identity and Equality

Two `Account` objects, same owner, same balance. Are they the same account?

The question sounds like philosophy and it is the most practical thing in the
unit. Get it wrong and a `HashSet` silently contains two copies of what you
believe is one item, a lookup returns null for a key that is demonstrably in the
map, and a change in one part of the program appears in another part that has no
connection to it.

This chapter is the one I would nominate as the hardest in the book. Not because
any single idea is difficult, but because there are two distinct questions that
English answers with the same word, and separating them takes effort.

**Identity** — are these the same object? One thing, or two?

**Equality** — do these two objects count as the same for my purposes?

Java answers the first with `==` and the second with `.equals`, and Chapter 18
already showed the damage when they are confused: `==` on strings works for
literals and fails for anything built at run time, so the bug passes every test.

It is hard because the right answer depends on what you are modelling, and the language
cannot know.

Two banknotes with the same serial number are a forgery — identity matters. Two
£5 notes are interchangeable — equality is what matters. Two people with the same
name are different people; two spellings of the same name are the same string.
Whether two objects with identical fields are "the same" is a question about your
domain, and you have to answer it.

What Java supplies is a mechanism for recording your answer, plus a contract you
must satisfy for the collections to work. This chapter is that mechanism and that
contract, and then the strategy — immutability — that makes most of the problem go
away.

Two sections. **References** is the groundwork: what a variable of object type holds, what
aliasing is, and the difference between a shallow and a deep copy. Chapter 12
introduced all three and this is where they stop being a preview.

**When Are Two Things the Same?** is `==` against `.equals`, the `equals`/
`hashCode` contract that hash collections depend on, and the argument that an
object which cannot change is an object about which most of these questions do not
arise.
