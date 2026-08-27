# Further Reading

**Joel Spolsky, "The Absolute Minimum Every Software Developer Absolutely,
Positively Must Know About Unicode and Character Sets" (2003).** Recommended in
Chapter 4 and now applicable. Read it again after Section 29.1.2; the mojibake
example will land differently once you have produced some.

**RFC 4180, "Common Format and MIME Type for Comma-Separated Values Files".** Six
pages, and it will take twenty minutes to read the thing you thought you already
knew. The newline-inside-a-quoted-field rule is the part that invalidates most
hand-written parsers.

**Martin Kleppmann, *Designing Data-Intensive Applications*.** The best book on
this list. Chapter 4, "Encoding and Evolution", is Section 29.2.2 at full length
and with real systems; chapter 7 on transactions is where Jim Gray's material
becomes usable. Read it when persistence starts being the hard part of your work.

**E. F. Codd, "A Relational Model of Data for Large Shared Data Banks" (1970).**
Eleven pages that reorganized an industry. The argument about data independence is
the interesting part, and it reads as an argument about abstraction that this book
has been making since Chapter 1.

**The SQLite documentation, particularly "Appropriate Uses For SQLite" and "How
SQLite Is Tested".** The first is a candid discussion of when a file, an embedded
database, and a server are each right. The second is one of the best things
written about software testing anywhere, and it is a database manual.

**Dan Luu, "Files are hard" (2015).** A survey of the ways filesystem writes fail
in practice, including several that defeat the atomic-rename pattern. Read it
after Section 29.2.2, and treat it as a reason to use a database sooner rather
than as a reason to despair.

**Kernighan and Pike, *The Practice of Programming*, chapter 9.** On notation and
data formats, from two people who worked with Thompson and Ritchie. Short, and it
argues for text formats better than this chapter had room to.
