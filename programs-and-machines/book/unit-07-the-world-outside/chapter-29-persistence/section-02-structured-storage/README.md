# Structured Storage

Bytes on a disk are not data until something agrees how to read them, and unlike
everything else in this book, the *something* is not necessarily you and not
necessarily now.

That is what makes this section different. A class can be refactored. A file that
somebody already has cannot.

Three lessons.

Delimited data, where a format everybody thinks is trivial turns out to have a
genuine parsing problem in it. Then the design of a format meant to be read after
you have forgotten writing it, including the two-line fix for interrupted writes.
Then databases — what they give you that a file does not, and how to tell when you
need one.
