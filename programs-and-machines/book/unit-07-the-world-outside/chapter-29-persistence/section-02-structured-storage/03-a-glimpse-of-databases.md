# A Glimpse of Databases

You have a program with a CSV file, an index kept in memory to make lookups fast, a
locking scheme so two copies do not corrupt each other, and a routine that takes
backups.

You have reimplemented SQLite, badly, and probably without noticing when it
happened. This lesson is about recognising the point where files stop being the
right answer — which is a specific list of five things, not a matter of scale.

At some point a file stops being the right answer. This lesson is about
recognizing that point, and about what you get in exchange for the complexity.

## Where files run out

Five problems, and any one of them is enough.

**Concurrent writers.** Two processes writing the same file interleave, and the
result is corruption. The atomic-rename trick of Section 29.2.2 handles a single
writer replacing a file; it does nothing for two writers appending. There is file
locking, and it is awkward, advisory on some systems, and unreliable over network
filesystems.

**Partial updates.** Changing one record in a million-record CSV means rewriting
the whole file. That is slow, and it is a window during which the file is being
rewritten.

**Queries.** "All books published after 1980, by author, sorted by title" over a
file means reading everything and filtering in memory. Fine for a thousand
records; not fine for ten million.

**Atomicity across records.** Moving money between two accounts is two changes
that must both happen or neither. Files give you no way to say that.

**Concurrent readers during a write.** A reader that opens the file mid-rewrite
sees a partial state.

Those five are, almost exactly, the list of things a database provides.

## What a database is

A program that owns your data and mediates access to it, offering four guarantees
usually named together as **ACID**:

**Atomicity.** A transaction happens completely or not at all. The money moves
from both accounts or neither.

**Consistency.** Declared rules — a balance cannot be negative, an order must
reference a real customer — are enforced by the database, not merely by every
program that writes to it. That is Chapter 19's encapsulation applied to data, and
for the same reason: a constraint anyone can bypass is not a guarantee.

**Isolation.** Concurrent transactions do not see each other's partial work.
Chapter 31's problem, solved once by people who specialize in it.

**Durability.** Once a transaction commits, it survives a crash. This is the
`force()` question from Section 29.1.3 taken seriously, with a write-ahead log so
that an interrupted commit can be replayed.

Every one of those is something you could implement over files. Jim Gray, from
Chapter 28, largely invented how, and it took the field two decades.

## SQL, briefly

The relational model — Codd, 1970 — says data is a set of tables, and a query
describes *what* you want rather than how to get it:

```sql
SELECT title, author FROM books
WHERE year > 1980
ORDER BY title;
```

Nothing there says how to find the rows. The database's query planner decides
whether to scan the table or use an index, in which order to join, and what to
compute first — and it decides using statistics about the actual data, which it
updates as the data changes.

That is a genuinely striking piece of abstraction: **you state the result and the
system chooses the algorithm.** It is the only widely used declarative language
most programmers meet, and the reason SQL has outlived every technology it was
supposed to be replaced by.

## From Java

```java
try (Connection c = DriverManager.getConnection(url);
     PreparedStatement s = c.prepareStatement(
         "SELECT title, author FROM books WHERE year > ?")) {
    s.setInt(1, 1980);
    try (ResultSet rs = s.executeQuery()) {
        while (rs.next())
            System.out.println(rs.getString("title") + " by " + rs.getString("author"));
    }
}
```

Two things in that snippet matter more than the API.

**try-with-resources on everything.** A connection is a scarce server-side
resource, and a leaked one is worse than a leaked file handle — the pool exhausts
and the whole application stops. Section 28.2.2's material, at its most consequential.

**A `PreparedStatement` with a `?`, not string concatenation.** Writing

```java
"... WHERE year > " + userInput
```

is **SQL injection**, and it is not a subtle vulnerability. If `userInput` is
`0; DROP TABLE books;--`, that text becomes part of the query. It has been the
top-ranked web vulnerability for most of two decades, and the fix is to never
build a query by concatenation — the placeholder sends the value separately, so it
cannot become syntax.

That distinction is Chapter 24's, exactly. Concatenation puts user data into the
*grammar*; a parameter keeps it as a *value*. Once you have written a parser, the
vulnerability is obvious in a way it was not before.

## SQLite, and the middle ground

The step between a file and a database server is smaller than it used to be.

**SQLite** is a database that lives in a single file, requires no server, and is
one dependency. It has real transactions, real indexes, and real SQL. It is the
most widely deployed database in the world — it is in every phone, browser, and
operating system — and it is the right answer for a great many programs that
currently use a hand-rolled file format.

The reasonable ladder:

**A file** for configuration, for data written once and read whole, for anything a
human should be able to open in an editor.

**SQLite** as soon as you want queries, indexes, incremental updates, or
transactions, and there is one process.

**A database server** — PostgreSQL, MySQL — when several processes or machines
need concurrent access, or when the data outgrows one machine's attention.

The mistake to avoid is the middle one skipped: a program that has grown a CSV
file, an index kept in memory, a locking scheme, and a backup routine has
reimplemented SQLite badly, and usually without noticing that that is what
happened.

## What to take from this

You will meet databases properly elsewhere, and this chapter's contribution is the
framing rather than the syntax.

**Persistence is a specialty.** Atomicity, isolation and durability are hard
enough that they are the subject of careers. When you need them, use something
that has them.

**The abstraction is the point.** A query says what, not how, and the system
chooses. That is the same move as every abstraction in this book, taken further
than any of them.

**And the failure modes are the ones you have already met.** Concurrent writers is
Chapter 31. Partial writes is Section 29.2.2. Injection is Chapter 24. Leaked
connections is Chapter 28. A database is not a new subject; it is the subjects you
have been reading about, solved carefully by people who did it full time.

Chapter 30 turns from data that outlives the program to a user who interrupts it.
