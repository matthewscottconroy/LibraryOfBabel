# People

## Bob Bemer (1920–2004)

An IBM engineer who served on the committee that produced ASCII and is often
called its father. He is responsible for the escape character, for the backslash,
and for pushing to reserve space in the standard for other alphabets — an
argument he lost, and which took Unicode another twenty-five years to win.

Bemer also spent decades warning about the two-digit year problem, publishing on
it as early as 1971 and being largely ignored until the late 1990s made it
urgent. That is a representation problem in the purest form: a field too small
for the values it would eventually hold, chosen when the storage cost was real
and the future seemed far away.

## Joseph D. Becker

A researcher at Xerox who wrote the 1988 draft *Unicode 88*, which set out the
proposal that became the standard. The paper argued for a unified character set
covering all the world's writing systems, and made the case that the fragmented
code-page situation was not merely inconvenient but structurally unfixable.

The original proposal assumed 16 bits would be enough — 65,536 characters, which
seemed generous. That assumption is the direct ancestor of Java's 16-bit `char`
and of the surrogate-pair machinery that exists to work around it. It is a
useful example of how a reasonable estimate, encoded into a type, outlives the
reasoning behind it.

## Ken Thompson (born 1943) and Rob Pike (born 1956)

Both at Bell Labs; Thompson had earlier created Unix and the B language, and
shares the 1983 Turing Award for it.

In September 1992 the two of them designed UTF-8, reportedly sketching it on a
placemat in a New Jersey diner and implementing it over the following days. The
design brief was demanding: encode all of Unicode, keep ASCII files byte-for-byte
valid, never let an ASCII byte appear inside a multi-byte character, and allow a
reader to find character boundaries from any position. UTF-8 satisfies all of
them at once.

It is worth appreciating how unusual that is. Most standards satisfy their
constraints by compromise. UTF-8 satisfies its constraints by having a better
idea, and the result is that it now encodes the overwhelming majority of text on
the internet.

## Harry Nyquist (1889–1976)

A Swedish-born engineer at Bell Labs whose 1928 paper *Certain Topics in
Telegraph Transmission Theory* established the relationship between a channel's
bandwidth and the rate at which distinct signals can be sent through it.

Claude Shannon — whom we met in Chapter 1 and will meet again in Unit VIII —
formalized the sampling theorem that bears both their names in 1949. The result
is why 44,100 samples per second is enough to reproduce everything a human can
hear, and why sampling more often than that buys nothing for playback.

The theorem deserves a moment's respect. It says that discreteness is not
necessarily a loss: a continuous signal, sampled at instants, can be recovered
*exactly*, provided you know something about what it does not contain. That is a
much stronger statement than "close enough", and it is the reason digital audio
was possible at all.
