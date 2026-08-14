# Chapter 63 — The People

This chapter's discipline came from aviation, medicine and cognitive psychology. Chapter
53's and Chapter 55's people apply here too — Gawande, Cook, Dekker, Allspaw — and this
section covers the ones specific to diagnosis rather than to prevention.

Eric Moody and the crew of British Airways Flight 9, 24 June 1982.

A Boeing 747 flew into a volcanic ash cloud over Java and all four engines failed.

**There was no checklist.** Four-engine failure was considered so improbable that no procedure
existed, and the crew had eight thousand metres of altitude and about fifteen minutes.

What they did was systematic rather than inspired. They worked the problem in a defined
order — fly the aircraft first, then diagnose, then act — and they did not guess. They
applied the single-engine restart procedure four times, and after several attempts three
engines relit.

Moody's announcement to the passengers is the other reason the incident is remembered:

> "Ladies and gentlemen, this is your captain speaking. We have a small problem. All four
> engines have stopped. We are doing our damnedest to get them going again. I trust you are not
> in too much distress."

Which is, as a status update, better than most: it states what is known, what is being
done, and does not speculate. §63.1's communication requirement, under considerably more
pressure than a network incident supplies.

> The transferable lesson is not the heroism. It is that under conditions where no procedure
> existed, the crew applied the procedures they had, in a defined order, one at a time — and
> **that is exactly §63.1's discipline.**

**Daniel Kahneman (1934–2024) and Amos Tversky (1937–1996).** The cognitive biases §63.1
names.

Their work from 1974 onward established that human judgement under uncertainty follows
systematic, predictable patterns of error — not random mistakes, but consistent ones that
can be described and anticipated.

| Bias | In §63.1 |
|---|---|
| **Confirmation bias** | **you look at the firewall and find something** |
| **Availability heuristic** | **recency bias — the last three were DNS** |
| **Anchoring** | **the reporter's diagnosis** |
| **Representativeness** | **the interesting hypothesis feels more likely than the boring one** |

Kahneman's *Thinking, Fast and Slow* (2011) names the two systems — fast, automatic,
pattern-matching judgement, and slow, effortful, deliberate reasoning — and the practical
finding is that the fast system is what operates under time pressure and is exactly the one
subject to these biases.

> **Which is the argument for a method.** A checklist is a device for forcing the slow system
> to run when the fast one would otherwise decide, and it works because it does not depend on
> the practitioner recognising that they are biased — which they cannot reliably do.

Gary Klein (b. 1944) supplies the necessary counterweight, and it is worth having.

Klein's research on naturalistic decision making — firefighters, nurses, military
commanders — found that experts under time pressure make good decisions rapidly, by pattern
recognition, without comparing options.

**Which appears to contradict Kahneman entirely.** The two of them collaborated on the
question and published a joint paper (2009) resolving it, and the resolution is the useful
part:

> Expert intuition is reliable when the domain is regular enough for patterns to exist and the
> expert has had enough feedback to learn them. **It is unreliable when either condition
> fails.**

**Applied to network troubleshooting:**

| | |
|---|---|
| **Familiar network, common fault, years of experience** | **intuition is good, and the method is overhead** |
| **Unfamiliar system, novel symptom, high stakes** | **intuition is unreliable, and the method is essential** |

Which is §63.2's proportionality argument, with an evidential basis — and it explains why
experienced engineers resist the method and are frequently right to, and why they are
catastrophically wrong to when the system is new to them.

**Klein's "premortem" technique also belongs here:** before acting, imagine the action has
failed and explain why. It surfaces objections that a "does anyone have concerns?" question
does not, and it takes two minutes.

**Sherlock Holmes**, and the reason he is cited in every troubleshooting text.

The maxim usually quoted — *"when you have eliminated the impossible, whatever remains,
however improbable, must be the truth"* — is the weaker one. The better one is from
*A Scandal in Bohemia*:

> "It is a capital mistake to theorise before one has data. Insensibly one begins to twist
> facts to suit theories, instead of theories to suit facts."

Which is §63.1 in two sentences, written in 1891, and "insensibly" is the precise word —
the twisting is not deliberate and the practitioner does not notice it happening.

W. Edwards Deming (1900–1993) and the Plan–Do–Check–Act cycle.

Deming's cycle is the seven steps' ancestor, and his larger contribution to this chapter is
the insistence that most defects are properties of the system rather than of the people
operating it — which is Dekker's argument (Chapter 53's reading) arriving from
manufacturing forty years earlier.

And his observation that "without data you are just another person with an opinion" is
§63.1's evidence requirement, stated for a different audience.

**Richard Feynman (1918–1988)** and the Challenger commission, for one paragraph.

Feynman's contribution to the investigation was to demand the actual numbers — and, when
the engineering estimates of failure probability differed from management's by three orders of
magnitude, to publish that fact in an appendix he insisted be included.

> "For a successful technology, reality must take precedence over public relations, for nature
> cannot be fooled."

Which is §63.4's argument about honest incident records, and it applies to internal
documents as much as to public ones.

## What this history establishes

**The biases are systematic, not personal.** Kahneman and Tversky established that they are
predictable properties of human judgement, which means a method that anticipates them works
for everyone and is not an insult to anyone's competence.

**Expertise is real and conditional.** Klein established that expert intuition works where
patterns exist and feedback has been available, which means the method should be applied in
proportion to unfamiliarity rather than uniformly.

**And the discipline was borrowed, not invented.** Aviation's checklists, medicine's protocols
and manufacturing's cycles all predate networking's version of them by decades, and
networking is still catching up — Chapter 53 made the same observation about documentation.

> **The consolation is that the borrowing works.** These are not analogies; the cognitive
> failure modes are literally the same ones, in a domain where the consequences are smaller and
> the opportunity to practise is far greater.
