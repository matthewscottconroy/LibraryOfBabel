# People

Short profiles of the people whose decisions this chapter describes. These are
not decorations; in each case the idea makes more sense once you know what
problem the person was actually facing.

## Gottfried Wilhelm Leibniz (1646–1716)

Leibniz published a systematic account of binary arithmetic in 1703, in a paper
for the Paris Academy of Sciences titled *Explication de l'Arithmétique Binaire*.
He showed how to write any number using only 0 and 1, and how addition,
subtraction, multiplication and division work in that system.

He had no machine to run it on, and no expectation of one — his interest was
partly mathematical and partly theological, since he found significance in
everything arising from nothing and unity. What matters for us is that the
arithmetic was worked out and published two and a half centuries before there was
any hardware that needed it. When engineers eventually wanted a two-state number
system, the mathematics was waiting.

## Claude Shannon (1916–2001)

Shannon is responsible for two separate foundational results, either of which
would have been a career.

The first was his 1937 master's thesis at MIT, *A Symbolic Analysis of Relay and
Switching Circuits*. Shannon noticed that the algebra George Boole had developed
for logic in the 1850s described exactly the behavior of electrical relay
networks: a switch is open or closed, a proposition is false or true, and the
same algebra governs both. This is the observation that makes circuits designable
rather than merely tinkerable, and Chapter 8 is built on it.

The second was *A Mathematical Theory of Communication* (1948), which founded
information theory — and which is where the word "bit" first appears in print.
Shannon's definition of a bit is quantitative: it is the information gained from
resolving a question whose outcomes were equally likely. Unit VIII returns to
this, because it is what ultimately explains why compression works and how far it
can go.

## John Tukey (1915–2000)

A statistician at Bell Labs and Princeton, Tukey coined the word "bit" as a
contraction of "binary digit"; Shannon credits him in the 1948 paper. Tukey also
coined "software" in a 1958 article.

He is worth a mention here for a reason beyond vocabulary. Tukey's career was
built on the idea that the way you *represent* data determines what you can see
in it — his work on exploratory data analysis, and his part in the fast Fourier
transform, both turn on choosing a representation that makes structure visible.
That is this chapter's argument, arriving from a different direction.

## Nikolay Brusentsov (1925–2014)

Brusentsov led the team that built Setun at Moscow State University, completed in
1958. Setun was a working general-purpose computer that used three states rather
than two — balanced ternary, with digits −1, 0 and +1.

The design had real advantages: balanced ternary represents negative numbers
without a separate sign convention, and rounding is simpler. Around fifty
machines were built. It is included here because it is the clearest evidence that
binary's victory was an engineering outcome rather than a mathematical necessity
— someone built the alternative, and it ran.

## Danny Cohen (1937–2019)

An Israeli-American computer scientist who worked on early real-time networked
flight simulation and on the protocols that became Internet voice traffic.

In 1980 he circulated a note titled *On Holy Wars and a Plea for Peace*,
addressing the incompatible byte-ordering conventions then proliferating among
machine architectures. Borrowing the Lilliputian dispute over which end of an egg
to break from Swift's *Gulliver's Travels*, he named the two camps big-endian and
little-endian. The names stuck, and so did his argument: the choice is arbitrary,
what matters is that communicating parties agree.
