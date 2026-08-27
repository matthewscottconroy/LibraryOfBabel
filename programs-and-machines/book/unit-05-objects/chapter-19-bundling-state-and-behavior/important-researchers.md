# People

## Ole-Johan Dahl (1931–2002) and Kristen Nygaard (1926–2002)

Two Norwegians at the Norwegian Computing Center who, between 1962 and 1967,
invented objects.

Simula was built to write simulations — ships in a harbour, customers in a queue —
and the insight came from the problem domain. A simulation is full of things that
have their own state and their own behavior and act over time, and writing that
as a pile of arrays and procedures loses the correspondence. Simula 67 gave them
classes, objects, inheritance, and virtual methods: essentially the whole
vocabulary this unit uses.

Two things are worth registering. First, objects were invented to *model* — to
make the program's structure resemble the situation it was about — and only later
became a general organizing principle. Second, they arrived complete. Alan Kay
and Bjarne Stroustrup both took Simula as their starting point, which makes 1967
the origin of most of what came after.

They shared the Turing Award in 2001, a year before both died.

## Alan Kay (born 1940)

The designer of Smalltalk at Xerox PARC in the 1970s, and the person who coined
"object-oriented programming".

His conception was different from Simula's and different again from Java's. For
Kay the essential thing was **messaging** — objects as independent entities that
communicate by sending messages, each deciding for itself how to respond, with no
shared state and no direct access to one another's internals. He has said several
times that he regrets the term, because it put the emphasis on objects rather than
on messages.

The relevance here is that Java's `private` is a weak version of an idea Kay
wanted taken much further. In his framing an object should be unreachable except
through messages, and the possibility of a public field would not arise.

He also has a remark worth carrying past this chapter: that he made up the term
and did not have C++ in mind.

## Barbara Liskov (born 1939)

Liskov's fifth appearance, and the one that connects this chapter to Unit IV.

Chapters 11 and 16 credited her with abstract data types — the claim that a
program should be built from units defined by what they promise, and that a
*language* should enforce the separation rather than leaving it to discipline.

That second half is the whole of Section 19.2.1. Before CLU, you could choose to
program through an interface and nothing stopped anyone reaching around it.
Making the representation genuinely unreachable converts a convention into a
guarantee, and a guarantee is what lets you reason about which code could break an
invariant.

She returns in Chapter 21, where the principle governing inheritance carries her
name.

## James Gosling (born 1955)

Gosling's third appearance, for two decisions this chapter rests on.

**Four access levels rather than two.** C++ had public, private and protected;
Java added package-private as the *default*, so that a field with no modifier is
visible to its package and no further. Making the more restrictive option the
default is a small thing that shapes a great deal of code.

**No standalone functions.** Every method belongs to a class, which is why `main`
must be static and why utility classes like `Math` exist as bags of static methods
with no instances. This is widely regarded as awkward, and it is the reason
Section 19.2.3 has to explain that `static` means something different from
everything else in the chapter.

Whether that uniformity was worth its cost is arguable. What it produced is a
language where the answer to "where does this code live" is always "in a class",
which is at least easy to teach.
