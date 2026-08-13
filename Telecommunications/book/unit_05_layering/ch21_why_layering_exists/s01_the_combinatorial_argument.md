# 21.1 The Combinatorial Argument

Twenty chapters in, and no layer model has been drawn. This is deliberate, and it is
unusual: most networking books open with the OSI chart on page four.

The reason for the delay is that a layer model shown to someone with nothing to
organise is a piece of received vocabulary. Shown to someone who has built signalling,
media, medium sharing and a working switched network, it is a description of what they
already have. **The model is the answer to a question, and the question has to come
first.**

Here is the question.

## Counting

Suppose you must connect **m** kinds of application to **n** kinds of physical medium.

Applications, say: file transfer, electronic mail, remote terminal, printing, voice.
Media, say: twisted pair, coaxial cable, fibre, radio, satellite.

**The direct approach.** Write each application to drive each medium directly. File
transfer over twisted pair; file transfer over coax; file transfer over fibre; and
onward.

$$\text{implementations} = m \times n$$

Five by five is **25**. Manageable, apparently.

Now add a medium — say, a sixth. **Five new implementations**, one per application,
each written by people who understand file transfer or electronic mail and must now
also understand the electrical characteristics of the new cable.

Add an application instead. **Six new implementations**, one per medium.

The cost of any addition is proportional to the size of the *other* set. And since both
sets grow, the total grows as their product:

| m | n | implementations |
|---|---|---|
| 5 | 5 | 25 |
| 10 | 10 | 100 |
| 20 | 20 | 400 |
| 50 | 50 | 2,500 |

At 2,500 the arrangement has failed. Not because 2,500 is a large number, but because
**every one of them must be maintained, tested and updated independently**, by people
who must understand both ends of a pairing that has no intellectual coherence.

## The layered approach

Insert a single common intermediary. Every application talks to it; it talks to every
medium.

```
   Direct:  m × n                    Layered:  m + n

   App1 ─┬─ Medium1                  App1 ─┐          ┌─ Medium1
   App2 ─┼─ Medium2                  App2 ─┤          ├─ Medium2
   App3 ─┼─ Medium3                  App3 ─┼─ [IP] ───┼─ Medium3
   App4 ─┼─ Medium4                  App4 ─┤          ├─ Medium4
   App5 ─┴─ Medium5                  App5 ─┘          └─ Medium5

   25 implementations                10 implementations
```

$$\text{implementations} = m + n$$

Five plus five is **10**. And the marginal cost changes character entirely:

> **Adding a medium costs one implementation, regardless of how many applications
> exist. Adding an application costs one implementation, regardless of how many media
> exist.**

| m | n | direct (m×n) | layered (m+n) | ratio |
|---|---|---|---|---|
| 5 | 5 | 25 | 10 | 2.5× |
| 10 | 10 | 100 | 20 | 5× |
| 20 | 20 | 400 | 40 | 10× |
| 50 | 50 | 2,500 | 100 | **25×** |

**The advantage grows with scale**, which is the property that matters. A saving of
2.5× would be a nice optimisation. A saving that increases without bound as the system
grows is the difference between a network that can be extended by strangers and one
that cannot.

## What was actually bought

The arithmetic understates it, because the real gain is not fewer lines of code.

**Independent evolution.** Wi-Fi was designed in the 1990s. Every application written
before it — email from 1971, FTP from 1971, the Web from 1990 — worked over it on the
day it shipped, unmodified. Nobody wrote a Wi-Fi version of email. **The application
authors were dead, retired or busy, and their software worked anyway.**

**Independent expertise.** Radio engineering and mail-server design have nothing in
common. The layer boundary means that a person can be excellent at one without knowing
anything about the other, which is the only way a field of this size can employ
anybody.

**Independent failure.** When Wi-Fi is broken, it is broken for everything, and you
know to look at Wi-Fi. When email is broken and the Web works, the fault is in email.
Chapter 22 §22.4 makes this the basis of a troubleshooting method, and it is the
strongest practical argument for layering that this book will make.

**Substitutability.** You can replace a layer's implementation entirely without
touching anything above or below, provided the interface holds. The transition from
100 Mb/s to gigabit Ethernet required no application anywhere to change.

## The single point of leverage

The layered picture has one property the arithmetic does not show: **the middle is
special**.

There are many applications and many media. There is **one** IP. Every application
depends on it and every medium must support it, which means:

- It must be **simple enough to implement on anything**, including a device with a
  few kilobytes of memory
- It must be **stable**, because everything depends on it
- It must be **minimal**, because every feature it has is a feature every medium must
  accommodate
- And **changing it is extraordinarily hard**, because everything depends on it

The last point is IPv6's whole story (Chapter 28). A change to an application affects
its users. A change to a medium affects that medium's users. **A change to IP affects
everyone, simultaneously, and cannot be made incrementally** — which is why a
transition begun in 1998 is not finished.

This shape is the **hourglass**, and Chapter 23 §23.4 treats it properly. Note it here:
the thing that makes layering powerful — one universal intermediary — is also the thing
that makes the intermediary nearly impossible to change. **The narrow waist is both the
achievement and the constraint.**

## Why not more layers, or fewer?

If one intermediary is good, are seven better?

The counting argument does not say. It says *at least one* intermediary is
enormously valuable; it says nothing about the ideal number. Additional layers are
justified only when they factor out something genuinely reusable:

| Layer | What it factors out | Reused by |
|---|---|---|
| Link | driving a specific medium | everything above |
| Internet | getting across many networks | everything above |
| Transport | reliability, ordering, multiplexing | every application that wants them |
| Application | the actual task | — |

Each earns its place by serving many things above with one implementation below. **A
layer used by exactly one thing is not a layer; it is a subroutine with delusions.**

And each layer costs something (§21.3): a header, a copy, a boundary that cannot be
optimised across, and a place where information is lost. Chapter 22's seven layers are
a model for discussion; Chapter 23's four are what was actually built, and the
difference between those numbers is a real argument about where boundaries pay for
themselves.

## What we have already been doing

Look back at what is built:

| Chapter | What it does | Depends on | Provides |
|---|---|---|---|
| 5–8 | signals over a medium | physics | bits between two points |
| 9–10 | sharing one medium | bits | a shared channel |
| 15–16 | framing and addressing | a shared channel | frames on one link |
| 17–20 | switching and segmentation | frames | **a working local network** |

Each row uses the row below without knowing how it works, and provides something to
the row above without knowing what it is for. A switch does not know what its frames
carry. Ethernet does not know whether it is running on copper or fibre.

**The layers were built before they were named**, which is the honest history of the
subject and the reason this unit comes at Chapter 21 rather than Chapter 1.

## What breaks here

**Treating the model as the reality.** The layers are a decomposition, chosen because
it is useful. Chapter 22 §22.3 and §21.4 both cover cases where reality declines to
cooperate.

**Assuming layer count is principled.** It is not. Four layers, five layers, seven
layers — all are defensible groupings of the same functions.

**Missing the marginal-cost argument.** The point is not that layering saves work
today; it is that it makes the cost of *change* independent of the system's size.

> **Network+ note.** The certification does not examine the combinatorial argument
> directly, and understanding it makes every layer question easier — because you can
> derive what belongs at a layer instead of recalling it. **A layer exists to serve
> many things above with one implementation below**, and any function that does not do
> that is misplaced.
