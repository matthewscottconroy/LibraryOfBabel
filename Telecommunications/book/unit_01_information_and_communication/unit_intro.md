# Unit I — The Signal and the Symbol

There is a temptation, in a book about networks, to begin with a network: to draw
a box labelled *switch*, connect three boxes labelled *PC* to it, and start
naming things. We are going to resist that temptation for four chapters, because
the boxes are not the beginning. The beginning is a much older and much stranger
question, and if we skip it we will spend the rest of the book unable to answer
the only questions that ultimately matter: *how fast can this go, and why not
faster?*

The question is this. Suppose you have something to say, and someone far away who
needs to hear it. Between you there is nothing but the physical world — copper,
glass, air, vacuum — none of which has any interest in your meaning. The world
carries voltages and photons and pressure waves. It does not carry *the enemy
fleet has been sighted*. Somewhere, someone must convert one into the other, and
someone at the far end must convert it back, and the whole edifice of
telecommunications is a two-hundred-year-long argument about how to do that
conversion well.

This unit is about that conversion and the ceiling on it.

Chapter 1 sets up the problem and gives us the vocabulary of a communication
system — source, transmitter, channel, noise, receiver, destination — a diagram
first drawn by Claude Shannon in 1948 and which has not needed correcting since.
Chapter 2 builds the bit from nothing: what it means for information to be
*counted*, why binary is not an arbitrary convention, and why hexadecimal exists
even though nobody has sixteen fingers. Chapter 3 gives us the measurements —
bandwidth, throughput, latency, jitter, loss — and, more importantly, teaches us
to stop confusing them, because the most common error in network
diagnosis is treating a latency problem as a bandwidth problem and buying a
bigger pipe that changes nothing.

And Chapter 4 is where the unit earns its keep. In 1948 Shannon proved that every
channel has a hard, computable, physically enforced upper bound on the rate at
which information can cross it, and that no amount of cleverness, funding, or
marketing can exceed it. That is an extraordinary thing for a theorem to say. Most
engineering disciplines do not have a law that tells you *this is the best that
can ever be done*. Networking does, and the whole subsequent history of the field
— from the 56 kb/s modem ceiling to the modulation schemes in Wi-Fi 7 — is a
history of engineers climbing asymptotically toward a line that Shannon drew
before any of them were born.

By the end of this unit you will not yet have built a network. You will,
however, be able to look at any link that anybody ever proposes to build, ask
four questions about it — what is the bandwidth, what is the noise, what is the
distance, what is the required rate — and know within a factor of two whether it
is possible. That turns out to be a more durable skill than knowing any protocol
in this book.
