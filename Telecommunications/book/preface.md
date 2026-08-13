# Preface: A Fire on a Hill

Long before there were engineers there were signal fires, and long before there
was a theory of communication there was the problem it eventually solved. Watch
what a signal fire actually does and you will find, in miniature, nearly every
concept in this book.

Someone on a hilltop wants to tell someone on a distant hilltop that the enemy
fleet has been sighted. They cannot walk there in time; that is the whole
difficulty, and it is worth pausing on, because *distance* is the only reason
telecommunications exists. If everyone stood in the same room, the entire
apparatus of this book — the cables, the frames, the addresses, the protocols,
the seven layers and their arguments — would collapse into a shout.

So the watcher lights a fire. Immediately, and without any theory to guide them,
they have made a series of decisions that we will spend the next several hundred
pages formalising.

They have chosen an **encoding**: fire means *enemy sighted*, no fire means
*nothing to report*. That is one bit, though nobody would call it that for
another twenty-eight centuries. They have chosen a **medium**: light through
air, which is fast — effectively instantaneous over these distances — but which
is defeated by fog, and which cannot go around a mountain. They have accepted a
**bandwidth**: one bit, once, per night, which is a data rate so pitiful that
naming it seems cruel, and yet it was sufficient, because the *quantity* of
information required was also one bit. They have exposed themselves to **noise**:
a shepherd's cooking fire on the intervening ridge, a lightning strike, the moon
rising red through haze. And they have no way at all to detect the difference
between *no fire* and *fire that the receiver failed to see*, which is to say
they have built a system with no **acknowledgement** and therefore no
**reliability**.

Aeschylus, writing the *Agamemnon* around 458 BCE, has Clytemnestra describe a
chain of eight such beacons carrying the news of Troy's fall across the Aegean to
Argos in a single night. It is the earliest description we have of a **relay
network** — a set of intermediate nodes, each receiving and regenerating a
signal, extending a range that no single link could span. It is also, if you
read it with a suspicious modern eye, a description of a network with a
catastrophic failure mode: any single beacon that sleeps through its shift silently
severs the entire path, and no one at either end can tell the difference between
a working network reporting *nothing happened* and a broken network reporting
nothing at all.

We are going to keep meeting that problem. We will meet it in the ARP cache and
in the routing table, in the DNS resolver and in the TCP retransmission timer, in
the spanning tree that quietly blocks a port and in the firewall rule that
silently drops rather than rejects. *Distinguishing "nothing to report" from "the
path is broken"* turns out to be one of the deep recurring problems of the field,
and almost every protocol in this book contains some machinery — a keepalive, a
timeout, an acknowledgement, a hello packet — that exists purely to address it.

---

Between the beacon and the machine you are reading this on lies a chain of
inventions that this book will follow in order rather than in isolation. In 1837
Samuel Morse and Alfred Vail worked out that a code with variable-length symbols,
short for common letters and long for rare ones, would move English over a wire
faster than a fixed-length code would — a discovery that anticipated Shannon's
source coding theorem by a hundred and eleven years, made by people who had no
mathematics for why it worked and simply noticed that it did. In 1876 Alexander
Graham Bell's telephone made the signal *continuous* rather than discrete, which
was a gain in usability and, as we will see in Chapter 12, an enormous long-term
liability. In 1948, in a building in Murray Hill, New Jersey, Claude Shannon
published a paper that made the question *how much can this channel carry?*
answerable for the first time in the history of the species — before Shannon,
engineers argued about it; after Shannon, they computed it.

Then, in the 1960s and 1970s, a set of people who were mostly not telephone
engineers, and who were mostly regarded as slightly eccentric by the people who
were, decided that the right way to move data was to chop it into small labelled
chunks and let each chunk find its own way. This was, by the standards of the
telephone industry, an obviously terrible idea. It was also correct, and Unit III
is where we work out why.

---

There is a particular kind of understanding this book is trying to produce, and
it is worth being explicit about it, because it is not the kind that most
networking material aims at.

The common failure mode of networking education is a graduate who can recite that
DNS uses port 53 but who, when a user says *the website is down*, has no idea
what to check first. They have a vocabulary without a model. They can name the
seven layers and cannot use them to divide a problem in half. This is not the
student's fault. It is what happens when a field's terminology is taught before
its mechanisms, and it is a genuine pedagogical failure, not merely an aesthetic
one.

So the wager of this book is that if you understand *why ports have to exist* —
if you have felt the problem of a packet arriving at a machine running forty
programs and having no idea which one wanted it — then port 53 is a detail you
will absorb without effort and never quite forget, because it will have somewhere
to live. Whereas port 53 learned first, learned as a fact about the world, is a
fact you will look up again in six months.

Every chapter therefore does the same thing. It presents a problem you can feel.
It develops a mechanism that solves it, with real numbers, because a mechanism
you cannot compute with is a mechanism you do not really have. And then, only
then, it tells you what the industry calls the thing, in a box marked **Network+
note**, so that you can pass the exam and talk to your colleagues and read the
vendor documentation — but so that the vocabulary sits on top of the
understanding rather than in place of it.

---

A word on the historical material, since there is a lot of it and it is not
decoration.

Networks are one of the few engineering domains where the history is genuinely
load-bearing. The reason an Ethernet frame has a minimum size of 64 bytes is a
consequence of the speed of light and the maximum length of a coaxial cable
specified in 1980 — a cable that has not been manufactured in decades, whose
constraint is still enforced by the switch on your desk. The reason IPv4
addresses ran out is a decision made in 1981 by people who reasonably believed
they were building an experiment. The reason the modern Internet's routing system
can be hijacked by a single misconfigured router in a distant country is that BGP
was sketched, according to its authors, on the backs of two napkins in 1989
between people who all knew and trusted each other.

You cannot reason about these systems without knowing why they are shaped the
way they are, because they are not shaped the way a clean-sheet design would
shape them. They are shaped by accretion, constraint, accident, and the
occasional flash of genuine foresight. Learning the accidents is not nostalgia.
It is how you develop the instinct for which parts of a system are principled and
which are merely historical — and therefore which parts you may safely reason
from, and which parts you must simply look up.

---

One last thing before we start.

You will get more from this book if you have a terminal open beside it. Nearly
every claim here can be checked on the machine you already own, in seconds, for
free. When the book says a DNS resolution walks a hierarchy, you can watch it
walk. When it says TCP opens with a three-message handshake, you can capture the
three messages. When it says your packet's time-to-live decrements at every
router, you can watch it decrement across an ocean.

Very few fields offer that. Take it.

Light the fire.
