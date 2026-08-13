# Chapter 63 — Further Reading

## Read these first

**Kahneman, D. — *Thinking, Fast and Slow* (2011).**
**The biases in §63.1, established rather than asserted.** **Long; parts I and II carry the
argument**, and the chapters on the availability heuristic and on anchoring are directly this
chapter.

**Kahneman, D. & Klein, G. (2009). "Conditions for Intuitive Expertise: A Failure to
Disagree."** *American Psychologist*.
**Eight pages, and it is the resolution.** **When expert intuition is reliable and when it is
not** — which is §63.2's proportionality argument, with evidence.

**Cook, R. — "How Complex Systems Fail."**
**Recommended in Chapters 53 and 55, and points 10 through 14 are about diagnosis
specifically.** **Four pages.**

**Doyle, A. C. — *A Scandal in Bohemia* (1891).**
**For the paragraph, and because it is enjoyable.** "Insensibly one begins to twist facts to
suit theories."

## Method and practice

**Agans, D. — *Debugging: The 9 Indispensable Rules for Finding Even the Most Elusive Software
and Hardware Problems*.**
**Short, practical and directly transferable.** **"Quit thinking and look", "divide and
conquer", "change one thing at a time" and "keep an audit trail" are §63.1 to §63.4 in four
rules.** **The best single practical book on debugging anything.**

**Zeller, A. — *Why Programs Fail: A Guide to Systematic Debugging*.**
**More rigorous, and the chapters on hypothesis formation and on delta debugging are the
systematic version of §63.3's bisection.** **Delta debugging — automatically bisecting a change
set — is worth knowing about.**

**Limoncelli, T. et al. — *The Practice of System and Network Administration*, the
debugging chapter.**
**Practical, opinionated, and honest about what happens under pressure.**

**Beyer, B. et al. — *Site Reliability Engineering*, "Effective Troubleshooting" and "Managing
Incidents"** (free at sre.google).
**The incident command structure of §63.1 is developed properly here**, and the "Managing
Incidents" chapter is what D1's roles section should be based on.

**PagerDuty's Incident Response documentation** (response.pagerduty.com).
**Free, specific and well organised** — roles, severity definitions, communication templates.
**The best available model for D4.**

## Certification and the seven steps

**CompTIA's Network+ N10-009 objectives, section 5.1.**
**The seven steps are examined by name and in order**, and the exam tests which activity belongs
to which step. **Read the objective text itself rather than a summary** — the wording of each
step is what is examined.

**Any Network+ study guide's troubleshooting chapter**, used for the step wording and not for
the reasoning. **§63.2 supplies the reasoning; the guides supply the exam's phrasing.**

## Human factors and incident review

**Dekker, S. — *The Field Guide to Understanding "Human Error"*** (Chapter 53's reading).
**The reframing that makes §63.4's review productive.**

**Allspaw, J. — "Blameless PostMortems and a Just Culture"**, and the **Learning From
Incidents** community (learningfromincidents.io).

**Klein, G. — *Sources of Power: How People Make Decisions*.**
**The naturalistic decision making research.** **The firefighter chapters are the ones that
explain why experienced engineers do not follow methods and are frequently right.**

**Klein's premortem technique** — a two-page description is easy to find, **and it takes two
minutes to run and finds objections that "any concerns?" does not.**

**Deming, W. E. — *Out of the Crisis*.**
**Long, and the argument that defects are properties of systems rather than of people is
foundational.** **A summary is adequate for this chapter's purposes.**

## Post-mortems worth reading as examples

**Any major provider's public incident reports** — **AWS, Google Cloud, Cloudflare, GitHub,
Fastly.**
**F6 uses one.** **Read them as examples of §63.4's standard**: what they record, what they
attribute, what actions they commit to, and — in the better ones — **where they document their
own diagnostic errors.**

**Cloudflare's are consistently the most detailed and the most honest about the diagnosis
process**, including how long it took to find the cause and what the team believed in the
meantime.

**The `danluu.com/postmortem-lessons` collection** and similar aggregations — **for the patterns
across many incidents rather than the detail of one.**

**Aviation accident reports** — the AAIB, NTSB and equivalents publish them freely. **BA Flight
9's report is worth reading**, and **any report involving a diagnostic error under time pressure
will feel uncomfortably familiar.**

## Tools that support the method

**A scratch file.** **The single most valuable tool in this chapter** (§63.4), and it is a text
editor.

**`script`** (Unix) — **records an entire terminal session to a file, including timestamps with
`-t`.** **Run it at the start of an incident** and the record of what you did in what order
writes itself.

**`tmux` with logging enabled**, for the same purpose across multiple panes.

**Ring-buffer capture** — `tcpdump -W -C -w`, or a dedicated appliance. **F2 uses it**, and
Chapter 64 §64.3 covers the syntax.

**A ticketing or incident system with a timeline** — **the specific feature that matters is
timestamped entries**, so the reconstruction is automatic rather than remembered.

**A change log that can be searched by time** (Chapter 55 §55.2) — **because "what changed
between 09:00 and 09:40?" must be a query rather than an investigation.**

## Where to look next

**Chapter 64** supplies the tools the method uses; **Chapter 65** is the layered catalogue
§63.3's checklist summarises; **Chapter 66** is the specific case of performance complaints,
which resist this method more than any other symptom; and **Chapter 53 §53.4** is where the
documentation of §63.4 actually lives.
