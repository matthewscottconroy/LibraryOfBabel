# Text and Other Agreements

Numbers at least have the decency to be one thing. Text does not.

Ask what a letter is and the answer starts reasonable and gets worse. Is `A` the
same letter as `a`? Same letter, different case — so a program sorting names must
know they belong together, and a program checking a password must know they do
not. Is `é` one letter or two, an `e` with a mark added? Both answers are in use,
and text that means the same thing can be stored two different ways. Is the Greek
capital omega `Ω` the same as the ohm sign `Ω`? They look identical and mean
entirely different things.

None of this is a computing problem. It is a writing problem, several thousand
years old, which computing inherited and was obliged to make precise. This
chapter is about how that went — badly at first, then better, then complicated.

The first section, **Letters as Numbers**, follows the history because the
history explains the design. ASCII was built for English on 1960s hardware and
made assumptions that were reasonable then and are wrong now. Everything after it
is repair work. Unicode's central move — separating the *character* from its
*encoding* — is the idea worth carrying away, and once you have it, mojibake and
byte-order marks and the length of an emoji all stop being mysterious.

The second section, **Everything Else Is Also Bits**, widens the lens. Color,
sound, and images are the same question asked again: choose what to measure,
choose how finely, accept what you lose. Having watched it three more times, the
general shape of the representation question becomes visible, and that is what
Unit I has been building toward.

A warning about scope. Text encoding is genuinely deep — there are people whose
entire careers are Unicode normalization and collation — and this chapter does
not attempt completeness. What it attempts is enough understanding that when text
breaks, you know what kind of thing has gone wrong and where to look. That is a
realistic goal and a useful one.

One practical note. If you have ever had a program work perfectly on your machine
and produce garbage on someone else's, encoding is among the likeliest causes,
and it is the one people investigate last. By the end of this chapter it should
be the first thing you check.
