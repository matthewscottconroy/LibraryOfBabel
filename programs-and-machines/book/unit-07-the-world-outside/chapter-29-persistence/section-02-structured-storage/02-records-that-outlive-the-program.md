# Records That Outlive the Program

Your program writes a file. Between the first byte and the last, the process can be
killed, the machine can lose power, or the disk can fill.

Where does that leave the file? Not the old contents and not the new — the old data
gone and the new data half-written. For a configuration file that means the program
does not start next time.

The fix is two lines, and knowing it is the difference between a program that
survives being switched off and one that does not.

Behind that, and behind everything else in this lesson, is one constraint: the data
you write today will be read by a version of your program that does not exist yet.

## The interrupted write

```java
Files.writeString(target, newContents);
```

One line, and a window inside it. Between the truncation of `target` and the last
byte written, the file on disk is neither the old contents nor the new — and
anything that stops the program in that window leaves it that way permanently.

For a data file that is silent loss. For a configuration file it is worse: the
program will not start next time, and the thing that would have told you why is the
file that is now half written.

The fix is two lines and it is standard practice:

```java
Path tmp = target.resolveSibling(target.getFileName() + ".tmp");
Files.writeString(tmp, newContents);
Files.move(tmp, target, StandardCopyOption.ATOMIC_MOVE,
                        StandardCopyOption.REPLACE_EXISTING);
```

Verified:

```
after atomic replace: version=2
temp file gone: true
```

**Write to a temporary file, then rename over the target.** A rename within a
filesystem is atomic: the target is either the old file or the new one, never half
of either. An interruption during the write leaves a stray temporary file and the
original intact, which is a recoverable state.

Two caveats worth knowing. The rename must be on the same filesystem — across
filesystems it becomes a copy and the atomicity is lost, which is why the
temporary file goes beside the target rather than in a system temp directory. And
atomicity against a *power failure* additionally requires forcing the file to disk
before the rename, because the operating system may still be holding your bytes in
its own cache. The rename alone is enough for the common case of a killed process.

This pattern is everywhere once you know it. It is how editors save, how package
managers install, and how databases update their metadata.

## Versioning

Your format will change. Plan for it in the first version, because retrofitting is
much harder.

The cheapest thing that works: **put a version in the file.**

```
# books v2
title,author,year,isbn
```

A reader can then check, and can handle old files deliberately rather than by
accident.

The rules that make change survivable:

**Adding a field should not break old readers.** With a header-driven CSV reader,
a new column at the end is ignored. With JSON, an unknown key is ignored. With
position-based parsing, it is not — which is an argument for headers.

**Removing or reordering a field breaks things.** Do it in a new version, and keep
the reader for the old one until you are certain no old files remain. "Certain" is
a stronger condition than people assume; files are found in backups years later.

**Never change the meaning of a field.** Changing `year` from publication year to
acquisition year, without renaming it, produces data that parses perfectly and is
wrong. Add a new field and deprecate the old one.

That last rule is the one that costs the most to violate, because the failure is
undetectable by any check.

## Self-description

A format should say what it is.

**A header line**, naming the fields, so a reader can find a column rather than
counting to it.

**A version**, so incompatibility is detected rather than inferred.

**A stated encoding**, in a specification if not in the file. Section 29.1.2's
point.

**A comment convention** — lines beginning with a hash — so that a human can leave
a note in a file another human will open. Costs two lines in the reader.

The test to apply: **could someone who has never seen this format read a file and
work out what it means?** If not, you have written something only your program can
read, and your program will change.

## Absent values

The question every format must answer and many answer by accident: how do you
represent a field with no value?

The candidates: an empty field, a literal `NULL`, the text `N/A`, a sentinel like
$-1$ or `9999`.

The trap is that all of them are ambiguous against real data. An empty author
might mean unknown, or might mean the author's name is genuinely empty. `N/A` is
a legal book title.

So: **decide, write it down, and prefer the empty field**, which at least cannot
be confused with a value someone typed. And note this is Section 28.1.1's sentinel
problem, returned in a format instead of a return value — the same defect for the
same reason.

## What to store

Two decisions that cause disproportionate trouble.

**Store timestamps in UTC, in ISO 8601, with the offset.**

```
2026-08-27T14:30:00Z
```

Not `27/08/2026`, which is ambiguous with American ordering. Not a local time
without an offset, which is unrecoverable. Not a Unix epoch integer unless size
matters, since it is unreadable and its resolution is a guess.

**Store money in integer minor units.** Chapter 3 established that a `double`
cannot represent one tenth exactly; storing prices as floating point means the
file itself is wrong, and no amount of care in the reader recovers it. Store cents
as a `long` and record the currency.

Both of these are Chapter 1's argument in the end: the representation is an
agreement, and a file is where an agreement becomes permanent.

## The general habit

**A file format is an interface.** It has consumers you do not control, it must
change without breaking them, and every field is a commitment.

If a class's public methods deserve the thought Chapter 23 asked for, a file
format deserves more, because you can refactor a class and you cannot refactor a
file somebody already has.

Next: when files stop being enough.
