# Unit IV — Local Delivery

We are now going to build a working network.

Not a diagram of one, not a model of one — an actual arrangement in which one
machine can send data to another machine and be confident it arrived. Everything
in Units I through III was preparation: we know what information is, how fast it
can move, how to put it on a wire, and why the wire must be shared. This unit
spends that preparation.

The problem is deliberately narrowed. Both machines are on the same physical
segment — the same cable, the same switch, the same room. The destination is
reachable directly, with no intermediate network in the way. That restriction is
what makes the problem solvable in six chapters, and lifting it is what Unit VI
and Unit VII are for.

## The five questions

Narrow as it is, "send data to that machine over there" decomposes into five
questions, and the chapters answer them in order.

**Where does one message end and the next begin?** A wire carries a continuous
stream of voltage transitions. Something must mark boundaries, or the receiver
cannot tell a message from the silence around it or from its successor. That is
**framing**, and Chapter 15 builds it.

**Which machine is this for?** If three machines can hear the transmission, the
message must say who it is for. That is **addressing**, and the MAC address is
where it starts — a 48-bit identifier burned into hardware at the factory, flat and
globally unique, with all the advantages and one enormous disadvantage that flatness
implies.

**Who gets to talk?** On a shared medium, two machines transmitting at once
produce a collision and neither is heard. Somebody must arbitrate. Chapter 16's
CSMA/CD is Ethernet's answer, and its story — from a radio network in Hawaii to a
memo at Xerox PARC to 400 gigabits per second — is the best single illustration in
this book of how a standard survives by changing everything except its interface.

**How does the network avoid sending everything everywhere?** Early Ethernet
delivered every frame to every station, which is simple and wasteful and
insecure. Chapter 17's switch learns which addresses live where and forwards
accordingly, and this one mechanism — a table, populated by observation — is the
most important device behaviour in local networking.

**And then two questions the first four create.** Chapter 18's ARP exists because
we will shortly have *two* address systems that must be reconciled. Chapter 19's
spanning tree exists because redundant links, which we want for resilience, create
loops, which in a network without a hop count are catastrophic rather than merely
inefficient. Chapter 20's VLANs exist because one physical network usually needs to
be several logical ones.

## Why Ethernet, and only Ethernet

A reasonable objection: this unit is almost entirely about one technology. Token
Ring, FDDI, ATM LANE, ARCNET and several others existed, some with genuine
technical advantages, and they are dead.

The concentration is deliberate and it reflects reality. Ethernet won the LAN so
completely that "the LAN" and "Ethernet" are now nearly synonymous, and — more
interestingly — it then went on to win the metropolitan and wide-area markets it
was never designed for, and the data-centre fabric, and the storage network, and
substantial parts of the industrial control and automotive markets. Chapter 16
asks why, because the answer is not "it was better."

The short version, developed properly in §16.3: Ethernet was cheap, it was
sufficient, and — crucially — its *interface* stayed stable while its
*implementation* was replaced wholesale, several times. The frame format
standardised in 1983 is still the frame format in 2026. The shared coaxial cable,
the collision detection, the Manchester coding, the hub, and the half-duplex
operation have all been removed. What survives is a header layout and a
willingness to be replaced.

That is a lesson about interfaces which applies well beyond networking, and it is
worth carrying into Unit V, where we make the argument about layering explicitly.

## A note on order

You may notice that we build a functioning network for six chapters before
introducing the OSI model in Unit V, which inverts the conventional order.

This is the book's central pedagogical bet, and Unit IV is where it pays off or
fails. By the end of Chapter 20 you will have encountered: a physical signalling
problem, a framing problem, a local addressing problem, a medium arbitration
problem, a topology problem, and the beginning of a global addressing problem —
each solved by a different mechanism, each replaceable independently. When
Chapter 21 then asks *what if every application had to solve all of these itself*,
the question will have force, because you will have watched six distinct problems
being solved by six distinct mechanisms and will already, informally, have been
thinking in layers for some time.

Presented in the other order, the seven layers are a chart to be memorised. Here,
they should arrive as a name for something you have already built.
