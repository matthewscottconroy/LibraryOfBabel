# How to Read This Book

This book has one question in it, asked seventy-two times in seventy-two
different registers:

> **How do we get information from one process on one computer to another
> process on another computer — reliably, efficiently, securely, and at scale?**

Everything here exists because it answers some part of that question. Nothing
here is introduced because a syllabus expects it. If you ever find yourself
reading a term and wondering *why am I being told this*, the chapter has failed
and you should write to whoever assigned it.

## Who This Is For

You need no networking background. If you have plugged in an Ethernet cable and
wondered, even briefly, what the cable was doing, you have the required
curiosity. If you have never converted a number to binary, Chapter 2 does it
with you from nothing, and nothing before Chapter 2 assumes it.

Three kinds of reader are expected:

- **The student** meeting networks for the first time in a semester course. Read
  in order. The order is the argument.
- **The certification candidate** preparing for CompTIA Network+ (N10-009).
  Every objective on that exam is mapped to a chapter in
  [Appendix D](appendices/appendix_d_network_plus_crosswalk.md). But read the
  derivations before the vocabulary — that is the entire pedagogical wager of
  this book, and the reason it is longer than a cram guide.
- **The working technician** who learned the commands and never the reasons.
  Start wherever you feel the gap. Units V, VII, and XIII are the ones that most
  often supply the missing floor under a working knowledge.

Assumed background: comfort with a command line, high-school algebra, and the
patience to convert a metaphor into a calculation.

## The Shape of the Argument

The book is ordered by dependency, not by tradition:

```
information → signals → media → sharing a medium → local delivery (Ethernet)
   → the idea of layers → global addressing (IP) → subnetting → routing
   → transport (TCP/UDP) → services (DNS/DHCP/HTTP) → wireless → wide area
   → operations → security → troubleshooting → design
```

Two departures from convention are deliberate and worth naming up front.

**The OSI model does not appear until Unit V.** Most books open with the seven
layers. That is backwards. Layering is a solution, and a solution presented
before its problem is just a list to be memorised. By Unit V you will have
personally hit four distinct problems that layering solves, and the model will
arrive as a relief rather than an initiation rite.

**Troubleshooting is a thread, not a unit.** Every layer we build generates new
ways to fail, and every chapter's final section ends with *what breaks here*.
Unit XIII formalises a method you will already have been using for twelve units.

## The Apparatus

Every chapter carries the same six kinds of file:

| File | What it is |
|---|---|
| `chapter_intro.md` | The opening scene, the problem, and what you will be able to do by the end |
| `sNN_*.md` | The lesson prose |
| `exercises.md` | Problems graded *Warm-up → Working → Challenge → Design*, plus a *Diagnose This* scenario |
| `important_concepts.md` | Every marked term, defined, with the section that derives it |
| `important_researchers.md` | The people, with real dates and real contributions |
| `further_reading.md` | Primary sources (RFCs, IEEE standards, original papers) and accessible secondary reading, annotated |

Two typographic conventions recur throughout:

> **Network+ note.** Vocabulary boxes like this one appear *after* a derivation,
> never before it. They give the industry's word for the thing you have just
> understood, and flag where CompTIA's N10-009 phrasing differs from the
> engineering literature's.

**What breaks here.** Boxes like this close a chapter's final section with the
failure modes that the chapter's mechanism makes possible, and the observable
symptom each one produces.

## Notation Fixed Once

| Convention | Choice |
|---|---|
| Data rate | bits per second, lowercase `b` (`Mb/s`) — never `MB/s` unless bytes are genuinely meant |
| Storage | bytes, uppercase `B` |
| Prefixes | decimal SI for rates (`1 Gb/s = 10⁹ b/s`), binary IEC (`GiB`) where memory is meant |
| IPv4 | dotted decimal with CIDR prefix: `192.168.10.70/27` |
| IPv6 | lowercase, RFC 5952 compression: `2001:db8::1` |
| MAC | colon-separated lowercase hex: `00:1b:44:11:3a:b7` |
| Layer names | *frame* at L2, *packet* at L3, *segment* (TCP) / *datagram* (UDP) at L4 — held strictly |
| Decibels | `dB` for ratios, `dBm` for absolute power referenced to 1 mW |
| Byte order | network byte order (big-endian) throughout |

Addresses in examples use the documentation ranges reserved by RFC 5737
(`192.0.2.0/24`, `198.51.100.0/24`, `203.0.113.0/24`) and RFC 3849
(`2001:db8::/32`) wherever a public address is needed, and RFC 1918 space
elsewhere. No example in this book contains a real routable address belonging to
someone else.

## Companion Assets

The prose is the book, but the book was written to be *done*, not only read:

- **[labs/](../labs/)** — fifteen hands-on labs, one per course week, each with
  objectives, a procedure, expected observations, and debrief questions.
- **[tools/](../tools/)** — runnable Python: a subnet calculator and practice
  generator, a CSMA simulator, encoding and modulation visualisers, a link-budget
  calculator, a longest-prefix-match demonstrator, and more.
- **[project/](../project/)** — the semester-long Network Design and Technical
  Justification project, in seven staged deliverables with rubrics.
- **[instructor/](../instructor/)** — a fifteen-week schedule, three exam
  blueprints, retrieval-quiz banks, and the Network+ crosswalk.

## A Warning About Certainty

Networking is a field with an unusually high ratio of confident folklore to
established fact. "Never use channel 12." "Cat6 is faster than Cat5e." "Jumbo
frames make everything better." Each of those is true in some circumstance,
false in others, and repeated as gospel in all of them.

Where this book states something as fact, it is because a standard says so, a
measurement showed it, or arithmetic requires it — and the chapter says which.
Where a claim is contested, the book says that too. If you take one habit from
this book, take that one: ask what kind of claim you are being handed before you
decide how hard to hold it.
