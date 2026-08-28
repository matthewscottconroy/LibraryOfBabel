# Talking to Other Machines

Everything so far has assumed the other end of a conversation is on your machine
and cooperating.

Now it is not. It is on a machine you do not control, reachable over something that
can lose your message, and the hardest fact in this section is that when it stops
answering you cannot tell whether it is slow or dead. Every distributed system ever
built is a set of guesses about that question.

Two lessons.

Sockets and protocols: how two programs on different machines exchange bytes, and
why an agreement about what those bytes mean is the whole of the work. Then
blocking and waiting, which is where the network's two defining properties —
everything is slow, and anything can fail — turn into design decisions.
