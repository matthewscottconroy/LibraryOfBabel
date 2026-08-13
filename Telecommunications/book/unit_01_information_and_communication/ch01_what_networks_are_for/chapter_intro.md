# Chapter 1 — What Networks Are For

On the morning of 16 August 1858, Queen Victoria sent a message to President
James Buchanan. It was ninety-eight words long. It took sixteen hours and
thirty-odd minutes to arrive.

This was celebrated, correctly, as one of the great triumphs of the age. Cannons
were fired in New York; the city hall caught fire during the celebrations. The
alternative — a ship — took ten days if the weather was kind. The Atlantic
Telegraph Company had laid 2,500 nautical miles of cable across the floor of an
ocean nobody had properly mapped, and had gotten it to work, and the world had
become, in a way that was immediately obvious to everyone alive, smaller.

The cable failed three weeks later. The chief electrician, Wildman Whitehouse,
had been applying two thousand volts to it in an attempt to push the signal
through faster, and had cooked the insulation. Whitehouse was a surgeon by
training who had appointed himself an expert in submarine telegraphy. William
Thomson — later Lord Kelvin — had told him, with mathematics, that the cable's
capacitance would smear each pulse across its neighbours, that the fix was a more
sensitive receiver rather than a more brutal transmitter, and that the voltage
would destroy the line. Whitehouse ignored him. Whitehouse was wrong, publicly,
expensively, and in a manner that took eight years and a second cable to repair.

We start here, and not with a diagram of a switch, because that story contains
the whole subject. There is a **channel** with physical properties that constrain
what can be sent through it. There is a **rate** that people want and a rate the
physics permits, and the gap between them is where all the engineering happens.
There is a **failure** whose cause was not obvious from the symptom. And there is
a man who tried to solve a signal-integrity problem by turning up the power,
which — as anyone who has ever seen someone respond to Wi-Fi complaints by buying
a higher-gain antenna will recognise — is a mistake with an extremely long
half-life.

## What this chapter does

We are going to build the frame that the rest of the book hangs on.

First we establish *why* — what problem networks solve, and why distance is the
only reason any of this exists. Then we take apart a communication system into
its six pieces, following the diagram Claude Shannon drew in 1948, which is still
correct and which will let us locate every technology in this book on a single
map. Then we introduce the three nouns the field is built from — **hosts**,
**links**, and **protocols** — and see that a network is nothing more than these,
composed. And finally we state, in one sentence, the question that organises the
next seventy-one chapters, and show how each unit of this book attaches to a
different clause of it.

## By the end you will be able to

- Explain what a communication system is, in terms general enough to cover a
  signal fire, a telegraph cable, and a fibre-optic link, and specific enough to
  locate a fault in any of them.
- Identify the six components of Shannon's model in any real system put in front
  of you, including systems you have never seen.
- Distinguish a **host** from a **node** from an **endpoint**, and say why the
  distinction matters when reading a standards document.
- State the recurring question of this book and explain which part of it a given
  technology is trying to answer.
- Explain why "turn up the power" is usually the wrong answer to a communication
  problem, in terms that will still be true in Chapter 42 when we say it again
  about radios.

## Where this sits in the argument

This chapter is the root of the dependency tree. Nothing before it; everything
after. In particular, Chapter 2 will take the vague word *information* used here
and make it countable, and Chapter 4 will take the vague word *channel* used here
and give it a number.

Keep Whitehouse in mind as we go. He is going to reappear in this book under many
names.
