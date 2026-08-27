# Agreements, Not Facts

Here is a pattern of eight bits:

```
01000001
```

What is it?

Sit with the question before reading on. If you already know some binary you will
have an answer ready, and I want you to notice how quickly it arrived and how
confident it felt.

## Five correct answers

That pattern is the number 65.

That pattern is the letter `A`.

That pattern is the number 1.010001 times 2 to the something, if we are reading
it as a very small floating-point format.

That pattern is a moderately dark gray, if it is one channel of an eight-bit
color value.

That pattern is a machine instruction, on some processor somewhere.

Every one of these is right. Not "arguably right" or "right from a certain point
of view" — right, in the same full sense. The pattern does not have a hidden true
meaning that the other readings are approximating. It has no meaning at all until
a convention is applied, and different conventions yield different meanings with
equal legitimacy.

If you answered "65", notice what happened: you did not read the pattern, you
applied a convention you had already internalised so thoroughly that it felt like
perception. That is exactly the reflex this section is trying to interrupt —
not because the convention is wrong, but because you need to be able to see it as
a convention in order to notice when a different one is in force.

## An encoding is a function

Let us be precise about what a convention is, because precision here pays off
for the rest of the book.

An **encoding** is a rule that assigns meanings to patterns. Formally it is a
function: give it a pattern, it gives back a value. `01000001` goes in, `65`
comes out — under the unsigned-binary encoding. Under the ASCII encoding, `A`
comes out.

Two things follow immediately, and both matter.

**First: the encoding must be agreed in advance by both sides.** The program that
writes the pattern and the program that reads it must be using the same function,
or the reader gets a different value than the writer intended. There is no
mechanism by which the pattern can announce its own encoding, because any such
announcement would itself be a pattern needing an encoding. It is agreements all
the way down; somewhere, a human decided.

**Second: a mismatch is silent.** This is the part that costs people days.

If you write `01000001` intending the letter `A` and I read it as unsigned
binary, I do not get an error. I get 65. My program continues, cheerfully, with a
number that means nothing. Nothing was violated — I applied a perfectly valid
encoding to a perfectly valid pattern. The mistake was in the agreement, and the
machine has no way to detect it, because the machine has no access to what you
*meant*.

This is why the `café` becoming `cafÃ©` example from the unit introduction is
not a malfunction. Some program wrote those bytes under one agreement about how
to encode accented letters, and another read them under a different one. Both
behaved correctly. The bytes were never wrong. The agreement was.

## The mojibake experience

That garbling has a name, borrowed from Japanese: **mojibake**, roughly "character
transformation". Once you know the term you will start seeing it everywhere — in
badly imported spreadsheets, in old web pages, on restaurant menus printed from a
system that did not know what to do with an accent.

It is worth looking at closely, because it is the cleanest everyday example of an
encoding mismatch and you can work out exactly what happened.

The character `é` in the UTF-8 encoding is stored as two bytes:
`11000011 10101001`. If a program reads those two bytes under an older
single-byte encoding — one where every byte is its own character — it does not
see one character. It sees two. The first byte comes out as `Ã` and the second as
`©`, and so `café` displays as `cafÃ©`.

Nothing failed. Two bytes went in, two characters came out, exactly as the
reading convention specifies. The information loss happened because the reader
was told there was one convention in force when there was another.

Notice also that you can *reverse* this if you know what happened. The bytes
survived; only the interpretation was wrong. That is a genuinely useful thing to
know when you meet a corrupted file, and it is only available to someone who
thinks in terms of patterns and agreements rather than in terms of text.

## Why this is the central idea

I promised in the preface that one claim would recur through the whole book.
This section is that claim in its purest form, so let me point out where it is
going to come back.

In **Chapter 2**, the same bits will mean one number under an unsigned agreement
and a different, negative number under two's complement. Same pattern, two
readings, both correct.

In **Chapter 16**, an abstract data type will turn out to be an agreement about
what a collection of values means, enforced by making the underlying
representation unreachable.

In **Chapter 20**, `==` and `.equals` will turn out to be asking two different
questions about sameness, and choosing between them is choosing an agreement.

In **Chapter 25**, we will write an interpreter, and a data structure will become
a program purely because our evaluator agreed to read it as one.

In **Chapter 33**, Shannon will let us measure how much information an agreement
can carry at all.

Each of those will feel like a new topic when you reach it. It is worth noticing
that it is the same topic.

## A habit worth forming

When something in a program is behaving inexplicably, there is a question that
resolves a surprising fraction of cases, and it is a question you can now ask:

*What does this pattern mean, and who decided?*

Not "what is this value" — that presumes the answer. What does it mean, under
which agreement, and is the code on the other end using the same one?

We turn next to something the agreement has to settle before anything else: how
wide the pattern is.
