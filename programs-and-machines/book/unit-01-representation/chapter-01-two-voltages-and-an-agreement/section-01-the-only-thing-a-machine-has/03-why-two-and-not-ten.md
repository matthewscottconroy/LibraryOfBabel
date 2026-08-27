# Why Two and Not Ten

We count in tens. Every calculator you have used displays tens. So the obvious
design for a computer is one that works in tens — ten voltage levels, ten
symbols, no conversion needed, and no student ever again has to learn binary.

This is not a naive idea. It is what the first electronic computers actually did,
and the people who built them were not fools. It is worth understanding why the
approach lost, because the reasons are still operating today.

## People built these

ENIAC, completed at the University of Pennsylvania in 1945, was a decimal
machine. It stored numbers using ring counters — assemblies of ten vacuum-tube
flip-flops in which exactly one was on at a time, the position of the lit one
indicating the digit. Each flip-flop took a pair of triodes, so a single decimal
digit cost around twenty tubes, and one ten-digit accumulator ran to over five
hundred. ENIAC had about seventeen thousand tubes in total.

A little later, in the Soviet Union, Nikolay Brusentsov built Setun at Moscow
State University in 1958 — a machine that was not binary or decimal but
*ternary*, using three states. It worked, and by several accounts worked well.

So the alternatives were tried by capable people. Something pushed the industry
to two anyway.

## The noise argument again

Recall the noise margin from two sections ago. We agreed to read below 1 volt as
0 and above 2 volts as 1, leaving a full volt of forbidden middle ground. Noise
has to be enormous to push a signal across that gap.

Now build the decimal version. Suppose the circuit runs on 5 volts and you need
ten distinguishable levels. You get half a volt per level: 0.0 volts is a zero,
0.5 is a one, 1.0 is a two, and so on. To read the value you must decide which
half-volt band the signal is in.

Your noise margin has collapsed from a volt to a quarter of a volt — and that is
the *best* case, assuming your levels are perfectly spaced and your thresholds
perfectly placed. A disturbance that a binary circuit would shrug off now turns a
seven into a six.

It gets worse, because the problem is not only reading but *writing*. A binary
circuit must produce two output levels, and it can do that by driving as hard as
it can in one direction or the other — slam the switch fully on or fully off. A
decimal circuit must produce 0.5 volts *accurately*, and hold it, across
temperature changes and manufacturing variation and whatever the neighbouring
circuits are doing. Precision is expensive. Extremes are cheap.

That asymmetry is the heart of it. Two states are not merely fewer; they are the
only arrangement where you never have to be accurate about anything.

## The information you are buying

There is a fair counter-argument: surely a ten-state signal carries more, so
maybe the extra fragility is worth it?

Let us price it. One decimal digit distinguishes 10 possibilities. How many bits
would you need for the same job? Three bits give 8 — not enough. Four give 16 —
more than enough. So a decimal digit is worth somewhere between three and four
bits; the exact figure is about 3.32.

So by moving from two levels to ten, you have multiplied the information in one
signal by roughly 3.3. And in exchange you have divided your noise margin by
about 4.5, made every output stage a precision analogue circuit, and made every
input stage a nine-way comparison instead of a single threshold test.

Meanwhile the alternative — just use four wires and stay binary — costs you four
cheap circuits instead of one expensive one, and four cheap circuits is a
bargain, because cheap circuits are the one thing semiconductor manufacturing is
spectacularly good at producing in quantity.

## Where the real answer is

Here is the deepest version of the argument, and it is the one I would like you
to leave with.

A binary circuit does not have to *measure* anything.

A decimal circuit must answer "which of ten bands is this voltage in", which is
a measurement, and measurement is an analogue operation with analogue
sensitivities. A binary circuit answers "is this above or below the threshold",
which is a comparison — and a comparison can be built from a device that
falls over in one direction or the other. Transistors are extremely good at
falling over. That is essentially their nature.

So the industry converged on two states not because two is mathematically
special, but because two is the number of states you can distinguish *without
measuring*. Ternary is not absurd; Brusentsov's machine ran. It just never
offered enough to overcome the manufacturing advantage of a component that only
has to be able to slam shut.

## What about ten inside the machine?

You will occasionally meet decimal arithmetic living inside binary hardware, and
it is worth knowing why.

Money is the usual reason. As Chapter 3 will show in painful detail, a binary
fraction cannot represent 0.1 exactly — the same way a decimal fraction cannot
represent one third exactly. For scientific work the tiny error is irrelevant.
For a bank ledger totalling millions of transactions, an error that compounds is
unacceptable, and "unacceptable" here can mean "illegal".

So financial systems often use decimal representations built *on top of* binary
hardware: encodings where each decimal digit is stored in its own group of bits,
and arithmetic is performed digit by digit in software. It is slower. It is used
anyway, because correctness in the last cent is worth more than speed. Java
provides `BigDecimal` for exactly this, and Chapter 3 will tell you when to reach
for it.

Which is a nice illustration of where we started: the hardware is binary, and
decimal is an agreement layered on top. All the way up, it is agreements.

## The upshot

Two states won because two is the smallest number of states that can carry
information, and the smallest number is the most robust: maximum noise margin,
no precision requirements, and circuits that only need to be able to switch
rather than to measure.

Everything from here on is built on that decision. In the next chapter we start
building — beginning with how a *sequence* of these two-state signals can be made
to mean a number.
