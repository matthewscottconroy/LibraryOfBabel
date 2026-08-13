# Chapter 22 — The OSI Model

Here is a fact that ought to be more disconcerting than it usually is: the most
universally used conceptual framework in computer networking describes a protocol
suite that nobody runs.

Between 1977 and 1984, the International Organization for Standardization
developed **Open Systems Interconnection** — a complete seven-layer reference model
together with a full stack of protocols to implement it. The effort was serious,
international, well-funded and, for a period in the late 1980s, officially
mandated: the US Government's GOSIP procurement profile required OSI protocol
support, and the European Community pushed the same way. Careers were built on it.
Textbooks were written for it. It was going to replace TCP/IP, which was widely
regarded as a research-grade stopgap.

It did not. By 1995 the OSI protocol stack was commercially dead, and TCP/IP —
designed by a much smaller group, with far less money, and with no mandate at all —
ran everything.

**And yet the model outlived its protocols so completely that it is now
inescapable.** Engineers who have never seen a single OSI protocol say "that's a
Layer 2 problem" fluently, several times a day. Vendors sell "Layer 7 firewalls"
and "Layer 4 load balancers." Every certification examines it. Every fault
escalation form has a field for it.

Why? The chapter's answer, which §22.4 develops, is that the model turned out to be
a far better *diagnostic instrument* than it was a design blueprint. Its lasting
value is that it gives you a disciplined way to divide a problem — and that value
is entirely independent of whether the protocols it describes ever shipped.

## Why OSI lost, briefly

Worth knowing, because the reasons are instructive rather than accidental.

**It was designed by committee before implementation.** TCP/IP was specified by
people who had already built it and wanted others to interoperate; the RFC series
began as literal *requests for comments* among implementers. OSI was specified
first and implemented afterwards, by which time the specification's ambiguities and
excesses were expensive to fix.

**It was too complicated and too expensive.** The full stack was large, licences for
implementations cost real money, and TCP/IP came free with Berkeley Unix, which
came effectively free with a VAX, which every university had.

**It was too late.** By the time OSI implementations were purchasable, TCP/IP had
the ARPANET, the entire academic world, and — decisively — the 1 January 1983
flag-day cutover behind it. Networks have overwhelming network effects; the value
of joining the one everyone is already on is not something a better design
overcomes.

**It insisted on features nobody needed.** Layers 5 and 6 — session and presentation
— have almost no counterpart in real deployed systems, because it turned out that
applications preferred to handle those concerns themselves. Two of the seven layers
were, in practice, wrong.

This is not a story about a good design losing to a bad one. It is a story about
the difference between a specification and a running system, and it is the reason
the IETF's informal motto — Dave Clark's 1992 line, *"We reject: kings, presidents
and voting. We believe in: rough consensus and running code"* — is quoted as often
as it is.

## The seven layers, and the mnemonic problem

| # | Layer | What it does | Chapters |
|---|---|---|---|
| 7 | Application | Provides network services to software | 39, 40, 41 |
| 6 | Presentation | Encoding, encryption, compression | 2 §2.4, 58 |
| 5 | Session | Establishing, managing, tearing down dialogues | 41 |
| 4 | Transport | End-to-end delivery between processes | 35–38 |
| 3 | Network | Logical addressing and routing between networks | 24–34 |
| 2 | Data Link | Framing and delivery on one link | 15–20 |
| 1 | Physical | Bits onto the medium | 5–10 |

Every course teaches a mnemonic. *Please Do Not Throw Sausage Pizza Away* going
up; *All People Seem To Need Data Processing* going down. Use one if it helps.

But notice what you already have. You spent Unit II on Layer 1 and Unit IV on
Layer 2, and you did so without being told the numbers. The numbers are a
convenient index into work you have already done, which is exactly the relationship
this book intends — and it is why the chapter can be relatively short.

The layers you have *not* yet built are 3 through 7, and each is a unit ahead of
you: Unit VI and VII are Layer 3, Unit VIII is Layers 4 through 7.

## The honest caveats

Three, stated plainly, because the model is routinely oversold.

**The mapping to real protocols is imperfect.** ARP is between 2 and 3. MPLS is
"2.5" by common consent and by nobody's specification. TLS is variously called 5,
6, or "between 4 and 7" depending on who is arguing. ICMP is carried in IP but is a
control protocol for IP, which makes it Layer 3 signalling about Layer 3.

**Layers 5 and 6 are largely vestigial.** In the TCP/IP world their functions were
absorbed into applications and libraries. Do not spend effort looking for a session
layer in a modern stack; you will not find one, and its absence is not an oversight.

**Real systems violate it constantly and deliberately**, per Chapter 21 §21.4.

None of these undermine the model's use. A map that is imperfect at the coastline is
still the right thing to navigate by, provided you know where it is imperfect. The
purpose of stating the caveats is so that when you meet ARP and cannot place it, you
recognise the situation as a known limitation of the map rather than a gap in your
understanding.

## What this chapter does

§22.1 covers the history: the committee, the mandate, the protocols, and a fair
account of why it lost.

§22.2 covers Layers 1–3, mapped explicitly onto the mechanisms of Units II and IV
so that the model is attached to things you have built.

§22.3 covers Layers 4–7, mapped forward onto Unit VIII, with an honest treatment of
5 and 6.

§22.4 is the payoff: OSI as a diagnostic instrument. The layered method — establish
which layers are working, bisect, isolate — worked through several real symptom
sets, and the correspondence with the troubleshooting methodology formalised in
Chapter 63.

## By the end you will be able to

- Name the seven layers in order, in both directions, and state each one's
  responsibility in a sentence.
- Place any protocol or device you meet at its layer, and recognise the ones that
  do not fit cleanly.
- Explain why OSI's protocols failed while its model survived.
- Use the layered method to bisect a described fault, stating which layers the
  available evidence exonerates.
- Explain what a "Layer 3 switch" and a "Layer 7 firewall" actually are, and why
  the marketing names are accurate.
