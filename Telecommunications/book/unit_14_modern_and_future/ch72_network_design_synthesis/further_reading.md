# Chapter 72 — Further Reading

## Read these first

Oppenheimer, P. — *Top-Down Network Design* (3rd ed.).
The book that established that design begins with requirements. **Read Part I** — the
requirements and characterisation chapters — which is §72.1 and §72.2 done thoroughly.

White, R. & Donohue, D. — *The Art of Network Architecture* (2014).
The closest thing to a book about reasoning rather than about building. The complexity /
optimisation / surface argument is worth internalising, and it is the most useful design
heuristic in this chapter's reading.

Parnas, D. & Clements, P. (1986). "A Rational Design Process: How and Why to Fake It."
Eight pages, and it will change how you write a design document. The argument that the
documentation is a rational reconstruction rather than a history is liberating and correct.

Simon, H. — *The Sciences of the Artificial*, chapters on satisficing and on the architecture
of complexity.
Why a designer does not optimise, and why complex systems that work are hierarchical —
which is Chapters 21, 27 and 67, independently derived.

## Design practice

White, R., Zinin, A. & others — *Optimal Routing Design*, and White's later writing at
`rule11.tech`.
The routing design material is the most rigorous available, and White's blog is the ongoing
version of the book.

**Cisco's, Juniper's and Arista's validated design guides.**
Read them for the derivation rather than the topology. The better ones state why, and
those are the ones worth the time; the others are bills of materials with diagrams.

The Cisco Press *Designing for Cisco Internetwork Solutions* material, used with the same
caution as any vendor curriculum: the method transfers and the product recommendations do
not.

Ivan Pepelnjak's design material (`ipSpace.net`) — recommended throughout this book, and
his "how do I design X?" material is consistently the most sceptical and the most concrete.
The recurring "what problem are you solving?" is §72.4's over-engineering test.

The NANOG and RIPE design and operations tracks — operators presenting designs they built
and what they got wrong, which is the material that does not appear in books.

## The wider design literature

Alexander, C. — *Notes on the Synthesis of Form*, and *A Pattern Language*.
Not about networks, and *Notes* is the more relevant of the two — the argument that a
design resolves conflicting forces rather than choosing between them.

Petroski, H. — *To Engineer Is Human*, and *Design Paradigms*.
Failure as the source of engineering knowledge, and *Design Paradigms* is a catalogue of
design errors across disciplines that reads uncomfortably familiar.

Vincenti, W. — *What Engineers Know and How They Know It*.
A philosopher of technology on aeronautical engineering, and the best available account of
how engineering knowledge is actually produced — by iteration, by failure, and by
parameter-variation experiments rather than by deduction from theory.

Ferguson, E. — *Engineering and the Mind's Eye*.
On the non-verbal, visual component of design reasoning, which is why the three diagrams of
Chapter 53 §53.1 do work that prose cannot.

Brooks, F. — *The Design of Design*, and *The Mythical Man-Month*.
Brooks on design process, and the second book's arguments about conceptual integrity and about
the second-system effect — the latter being precisely §72.4's over-engineering warning, named
in 1975.

## Requirements and stakeholders

Robertson, S. & Robertson, J. — *Mastering the Requirements Process*.
**The rigorous treatment**, and the Volere template is more than most network projects need and
useful to have read.

Gause, D. & Weinberg, G. — *Exploring Requirements: Quality Before Design*.
Shorter and better for this purpose. The material on ambiguity and on what clients
actually mean is directly §72.1's argument.

Any book on interviewing technique, used deliberately. §72.1's "walk me through a day" is
an ethnographic technique, and it is more effective than a questionnaire for the same reason it
is in that literature.

## Cost and business argument

The TCO material from any of the major analysts, used for its structure rather than its
figures.

**Your own organisation's finance function.** **Genuinely** — §72.4's argument is that a design
is defended in the client's terms, and understanding how capital and operational expenditure
are actually treated in your organisation changes how a design should be presented.

Chapter 51's TeleGeography pricing reports and Chapter 69's cloud pricing pages — for the
actual numbers, which is what makes a cost argument credible.

## Worked examples to study

**Published network designs with their reasoning:**

The Internet2, GÉANT and JISC network architectures — published, detailed, and with the
design rationale documented, which is rare.

The RIPE and NANOG "how we built it" presentations — and specifically the ones that include
what was changed afterwards.

Google's, Meta's and Microsoft's data centre papers (Chapters 67 and 71's reading) — and
read them for the constraints and the rejected alternatives rather than for the architecture,
which does not transfer.

**Post-incident reports** (Chapters 55, 56, 63) — read as design critiques. An outage report
frequently names a design decision that was reasonable and turned out to be load-bearing in an
unanticipated way, which is the most instructive material available.

## The book's own apparatus

Appendix D's Network+ crosswalk, for anyone taking the certification.

**The `project/` directory** — the semester project is this chapter, staged, and D1 to D5 are
its assessment.

**The `labs/` directory** — fifteen labs, and the debrief is what is assessed, which is
§72.4's argument applied to teaching.

And the `tools/` directory — `netcalc.py` for §72.3's address plans, `perfcalc.py` for
§72.2's capacity arithmetic, and `simnet.py` for the statistical multiplexing argument that
underlies the headroom figures.

## Where to look next

**Nowhere. This is the last chapter.**

The reading that continues is the operational kind: the RFCs for the protocols you deploy,
the post-incident reports of the failures you have not yet had, the vendors' release notes for
the equipment you own, and the operator community's presentations — and the discipline is
Chapter 63 §63.4's: write down what you learn, because the next occurrence is cheaper if you
did.

And the question the book opened with is the one to keep asking:

> How do we get information from one process on one computer to another process on another
> computer — reliably, efficiently, securely, and at scale?

**Every chapter answered part of it.** The answers will change; the question will not.
