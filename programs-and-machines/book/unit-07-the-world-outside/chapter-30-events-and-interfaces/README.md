# Events and Interfaces

Every program in this book has had the same shape: it started, it did what it was
told in the order it was written, and it finished. `main` was in charge.

A program with a user interface is not like that, and the difference is
structural rather than cosmetic.

The user might click anything, at any time, in any order, or nothing at all for
ten minutes. Your program cannot ask "what will they do next" — it has to be ready
for all of it. So the arrangement inverts: instead of your code calling the
toolkit, the toolkit calls your code, whenever something happens.

That is **inversion of control**, and it is the chapter's real subject. Section
30.1 covers the event loop that makes it work, the listeners you register with it,
and the one rule that governs everything: **do not block the loop.**

Section 30.2 builds something. Components and layout, drawing, and then the
separation that matters more than any toolkit detail — keeping what your program
*is* apart from how it is *shown*.

Two things this chapter is deliberately not.

**It is not a Swing tutorial.** Java's GUI toolkits have changed three times —
AWT, Swing, JavaFX — and the desktop is no longer where most interfaces are built.
Memorizing widget APIs is a poor use of your attention.

**It is not about making things pretty.** Interface design is a real discipline
with its own literature and this chapter has nothing to contribute to it.

What is worth learning is the *shape*, and the shape transfers completely. A web
page in JavaScript, an Android app, an iOS app, a game loop, a web server handling
requests, a message queue consumer — every one of them is an event loop
dispatching to handlers you registered, with the same rule about not blocking and
the same pressure toward separating state from presentation.

Once you can see that, the specific toolkit is documentation.

One connection worth flagging before we start. The `() ->` you will write for
every listener is Chapter 26's lambda, and this is the use case that motivated
adding them. Before Java 8 every button handler was Chapter 22's five-line
anonymous class, and the resulting noise is the reason the notation exists.
