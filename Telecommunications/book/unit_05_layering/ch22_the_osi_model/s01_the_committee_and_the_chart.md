# 22.1 The Committee and the Chart

The OSI model is the most successful failure in the history of computing. Its protocols
were specified in exhaustive detail, mandated by governments, backed by every major
computer manufacturer, and are used by essentially nobody. Its **model** is on the wall
of every network operations centre on Earth.

Understanding how both of those are true is the point of this section.

## 1977

The problem was real and urgent. Every computer manufacturer had its own networking:
IBM's **SNA** (1974), DEC's **DECnet** (1975), Burroughs, Honeywell, Univac, each
complete, each proprietary, each incompatible with all the others.

For a customer this was intolerable. Buying IBM meant buying IBM forever, because the
network was the lock-in. A company that had bought DEC could not connect to a supplier
who had bought IBM without buying a third thing to sit between them.

**Open Systems Interconnection** was the response — the word *open* in the name is the
entire political programme. The International Organization for Standardization
convened the work in 1977; the reference model was published as **ISO 7498** in 1984.

The goal was correct, important, and pursued by serious people. Nothing about the OSI
effort was foolish.

## The seven layers

```
   ┌───┬──────────────┬───────────────────────────────────┐
   │ 7 │ Application  │  services to the user's program   │
   ├───┼──────────────┼───────────────────────────────────┤
   │ 6 │ Presentation │  syntax, encoding, encryption     │
   ├───┼──────────────┼───────────────────────────────────┤
   │ 5 │ Session      │  dialogue control, checkpointing  │
   ├───┼──────────────┼───────────────────────────────────┤
   │ 4 │ Transport    │  end-to-end delivery              │
   ├───┼──────────────┼───────────────────────────────────┤
   │ 3 │ Network      │  routing between networks         │
   ├───┼──────────────┼───────────────────────────────────┤
   │ 2 │ Data Link    │  frames on one link               │
   ├───┼──────────────┼───────────────────────────────────┤
   │ 1 │ Physical     │  signals on a medium              │
   └───┴──────────────┴───────────────────────────────────┘
```

The mnemonics, which you will meet regardless of your feelings about mnemonics:

**Bottom up:** *Please Do Not Throw Sausage Pizza Away*
**Top down:** *All People Seem To Need Data Processing*

## Why seven?

There is no derivation. The number was **negotiated**.

The committee's stated principles were reasonable — create a boundary where the level
of abstraction changes, where the function is manifestly different, where existing
practice suggests a division, where a boundary minimises information flow across it.
Sensible criteria, and they do not yield a number.

What produced seven was a committee in which different national bodies and different
companies had different existing systems to accommodate. **Layers 5 and 6 in particular
exist because they were somebody's requirement**, and in the protocol stack that
actually won they have no independent existence at all (Chapter 23 §23.2).

This is not a scandal. It is how standards get made, and knowing it prevents the
common error of treating the number seven as though it had been derived from
something.

## What the committee got right

Substantial things, and they are why the model outlived the protocols.

**Layers 1–3 are excellent.** Physical, data link, network — signals, frames, packets.
The decomposition is clean, the boundaries are where the abstraction genuinely changes,
and everything built in Units I–IV maps onto them exactly. There is no serious
alternative proposal for these three.

**The service/protocol distinction** (§21.2). Zimmermann's formulation, and it remains
the clearest way to talk about what a layer does versus how.

**The vocabulary.** "Layer 3 device", "Layer 2 problem", "a Layer 7 firewall" — this
is how the industry speaks, worldwide, across vendors and languages. Chapter 23's
four-layer model is more accurate and **nobody says "internet-layer device"**. The
seven-layer numbering is the lingua franca.

**Connectionless service.** OSI initially specified only connection-oriented network
service, on the reasoning that telecoms networks were connection-oriented and this was
a telecoms matter. **CLNP** — the connectionless alternative — was added after
sustained argument, and it was the right addition. Its cousin **IS-IS** (Chapter 31)
survives and routes a large fraction of the world's service-provider networks today.

## What the committee got wrong

**It was slow.** Design by international committee, with national bodies and dozens of
companies, produces documents in years rather than months. The IETF's counter-culture —
"we reject kings, presidents and voting; we believe in rough consensus and running
code" — was a direct response to watching this.

**It specified before implementing.** OSI protocols were written to be correct, then
implemented. TCP/IP was implemented, found wanting, fixed, and specified. **The second
approach discovers the problems that only appear in practice**, and the first
discovers them after the specification is frozen.

**It was complex.** The full OSI stack is enormous. Comparing sizes:

| | Roughly |
|---|---|
| OSI transport (TP0–TP4), session, presentation | ~1,000 pages |
| TCP (RFC 793) | 85 pages |
| IP (RFC 791) | 45 pages |

A student can read RFC 791 and RFC 793 in a weekend and implement a working stack. The
OSI equivalent is a career.

**It was expensive.** ISO standards cost money. RFCs are free, and always have been. To
a graduate student in 1985 — the people who actually built things — this alone decided
it. **You cannot implement what you cannot read**, and TCP/IP could be read at no cost
from any Internet-connected machine, which was itself the argument.

**It arrived late.** By the time OSI implementations were purchasable, TCP/IP was
running the ARPANET, was in BSD Unix, was free, and worked. The window closed while the
committee was still meeting.

## GOSIP, and the mandate that failed

The seriousness of the effort deserves emphasis, because retrospect makes it look
inevitable that TCP/IP would win, and it did not look that way at the time.

**The US government mandated OSI.** GOSIP — the Government OSI Profile, 1990 — required
federal agencies to procure OSI-compliant products. The European equivalents did the
same. This was the full weight of the largest customers on Earth.

It failed anyway. Agencies bought OSI-compliant equipment, sought waivers, and ran
TCP/IP on it. GOSIP was quietly withdrawn in 1995.

> **A government mandate, universal vendor support, and a technically thorough
> specification lost to free software that already worked.**

That sentence is worth carrying beyond networking. Chapter 23 §23.2 gives the fuller
account, and the pattern — deployability beating specification quality — appears again
with SEND (Chapter 18), with IPsec versus TLS (Chapter 61), and with IPv6's thirty-year
transition.

## What survived

Not nothing. The OSI work left more behind than the story usually admits:

| Survives | Where |
|---|---|
| **The seven-layer vocabulary** | universal, daily |
| **IS-IS** | routing a large share of service-provider backbones |
| **X.500 / LDAP** | every enterprise directory, including Active Directory |
| **X.509 certificates** | **every TLS connection on the Internet** |
| **ASN.1** | SNMP, LDAP, 3G/4G/5G signalling, and much else |
| **CLNP concepts** | connectionless network service, generally |

**Every HTTPS connection you make uses an X.509 certificate**, which is an OSI
specification, encoded in ASN.1 DER, which is another one. The protocols lost; the data
formats and the naming work are everywhere.

## Why still teach it?

Four honest reasons.

**1. It is the vocabulary.** Every vendor, every colleague, every job advertisement,
every certification uses the numbering. Not knowing it is a communication failure, not
an intellectual position.

**2. It is a diagnostic instrument.** §22.4 is the payoff, and it is the most
effective general troubleshooting method in this book. The seven layers give an ordered
checklist that reduces an unbounded problem to seven bounded ones.

**3. Layers 1–3 are correct.** They describe what was built in Units I–IV, accurately.

**4. It is examined.** Network+ objective 1.1 is the OSI model, and it is examined
heavily. This is the least intellectually satisfying reason and it is a real one.

## What breaks here

**Treating the model as a specification of what exists.** It describes a stack that
was never widely built. What runs is Chapter 23's.

**Forcing everything into a layer.** ARP, MPLS, ICMP, QUIC — some things do not fit,
and §21.4 explains why. Insisting on an answer produces confident nonsense.

**Assuming layers 5 and 6 exist as separate implementations.** In practice they are
library functions inside applications: TLS, JPEG, character encodings.

**Dismissing it because the protocols failed.** The model outlived them for reasons
that are about human communication, not about protocol design.

> **Network+ note.** Objective 1.1 **is** the OSI model, and it is among the most
> heavily examined objectives on the entire test. Know the seven layers in both
> directions, know what belongs at each, know the PDU names (Chapter 23 §23.3), and be
> able to place any given device or protocol. Expect several questions.
