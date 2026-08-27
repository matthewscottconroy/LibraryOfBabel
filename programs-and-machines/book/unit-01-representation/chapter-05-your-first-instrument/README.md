# Your First Instrument

Four chapters in, Java finally arrives.

I want to be clear about what it is arriving *as*. In most introductions, the
first program is a milestone — you made the machine say something, welcome to
programming. That framing is fine but it sets the wrong expectation for this
book, because it makes the language the subject.

Here, Java is an instrument. A microscope. You have spent four chapters making
claims about what is inside a machine — that 214 is `11010110`, that −5 is stored
as a pattern that looks like a large positive number, that one tenth is a
repeating fraction rounded off at the 53rd bit, that `A` is 65. Every one of
those claims you have taken on my authority.

By the end of this chapter you will have written a program that shows you all of
them directly, and you will not have to take my word for anything again.

That is a better first program than "Hello, world", and we will still write
"Hello, world" first, because you have to start somewhere and because there are
about six things in it that need explaining.

The first section, **From Source to Running**, is about what actually happens
between typing a file and seeing output. Java's answer is unusual — it compiles,
but not to machine code — and understanding the two-step arrangement explains a
great deal about the language's behavior, its portability, and why error messages
arrive at two different times.

The second section, **Looking at the Bits**, is where the instrument gets built.
We meet Java's primitive types, which are precisely the fixed-width boxes of
Chapters 2 and 3 with names attached, and we write the program that displays
their contents.

## What I am not going to do

I am not going to explain every word of `public static void main(String[] args)`
in this chapter. I am going to explain some of it, tell you honestly which parts
you should accept for now, and name the chapter where each remaining piece gets
its explanation.

I think the alternative is worse. The full explanation requires classes, access
control, static members, arrays, and the JVM's launch protocol — five things you
have no context for yet. Books that insist on explaining it all on page one
produce a page that nobody understands and everybody skips.

So: some things will be provisional. I will always tell you when they are, and
where the debt gets paid.
