# Important Researchers

**Ken Thompson** (born 1943) and **Dennis Ritchie** (1941–2011) made the decision
this chapter rests on: in Unix, **a file is an uninterpreted sequence of bytes**.
Before Unix, operating systems had file *types* — record-oriented files, indexed
files, files with a declared structure the system enforced — and a program had to
negotiate with the system about what shape its data was. Unix removed all of it.
The consequence is the pipe, the fact that the same tools work on any file, and
the idea that structure is the program's business. It is also Chapter 1's argument
implemented as an operating system. They shared the Turing Award in 1983.

**Edgar F. Codd** (1923–2003) proposed the relational model in a 1970 paper, "A
Relational Model of Data for Large Shared Data Banks", and the argument was about
independence: an application should state what data it wants without knowing how
the data is arranged, so that the arrangement can change. IBM, where he worked,
was reluctant — it had a successful hierarchical product — and the model took a
decade to reach a shipping system. Codd spent much of the rest of his career
objecting that commercial systems had implemented his idea incompletely, which was
largely fair. Turing Award, 1981.

**Jim Gray** (1944–2007) returns from Chapter 28 as the person who made
transactions work. He defined the properties later named ACID, invented the
two-phase locking and write-ahead logging that essentially every database still
uses, and wrote *Transaction Processing: Concepts and Techniques*, which remains
the reference. The measure of the work is that atomicity and durability are now
things you ask for in one keyword. Turing Award, 1998.

**Michael Stonebraker** (born 1943) built the systems. Ingres and then Postgres at
Berkeley in the 1970s and 1980s, both of which became long-lived open-source
databases — PostgreSQL is the direct descendant of the second. His later argument,
that one database architecture cannot serve every workload well, drove the
specialization into column stores, stream processors and time-series databases
that the last two decades produced. Turing Award, 2014.

**D. Richard Hipp** (born 1961) wrote SQLite in 2000, for a project that needed a
database on a ship with no administrator. It is public domain, it is a single
file, and it is now the most widely deployed piece of database software in
existence, present in every phone and browser. It is also unusually well tested —
the test suite is many times the size of the code — and the project is a good
argument that a small, complete, carefully verified thing can matter more than a
large ambitious one.

**Douglas Crockford** (born 1955) did not invent JSON so much as notice it: he
observed that JavaScript's object literal syntax was a usable data interchange
format, wrote a two-page specification, and declined to add features. The
restraint is the contribution — JSON has no comments, no dates, and no schema,
each an omission Crockford defended, and it displaced XML largely because it was
small enough to implement in an afternoon.
