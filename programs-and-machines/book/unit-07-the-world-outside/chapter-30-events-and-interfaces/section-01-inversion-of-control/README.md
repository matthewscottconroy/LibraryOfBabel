# Inversion of Control

Every program in this book has had the same shape: it started, did what it was
told in the order it was written, and finished. `main` was in charge.

A program with a window is not like that, and the difference is structural rather
than cosmetic. The user might click anything, at any time, in any order, or nothing
at all for ten minutes — and your program cannot ask what they will do next. It has
to be ready for all of it, which turns the arrangement inside out.

Two lessons.

The event loop first — what it is, why it exists, and why blocking it is the one
mistake that matters. Then listeners and callbacks, which is how your code gets
attached to it, and what changes about a program when its methods are called by
something else.
