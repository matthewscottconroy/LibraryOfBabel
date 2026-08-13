# Chapter 12 — Further Reading

## Primary sources

**Erlang, A. K. (1909). "The Theory of Probabilities and Telephone Conversations."
*Nyt Tidsskrift for Matematik B* 20: 33–39**, and **(1917) "Solution of Some
Problems in the Theory of Probabilities of Significance in Automatic Telephone
Exchanges." *Post Office Electrical Engineers' Journal* 10: 189–197.**
Short, and remarkable for being a working engineer inventing the mathematics he
needed. Translations are freely available. Worth reading for how directly the
problem is stated.

**Oliver, B. M., Pierce, J. R. & Shannon, C. E. (1948). "The Philosophy of PCM."
*Proceedings of the IRE* 36(11): 1324–1331.**
The case for digitising voice, made before a system existed. The regeneration
argument in §III is Chapter 5 §5.1's, in its original form.

**Reeves, A. H. (1938). French Patent 852,183 / (1942) US Patent 2,272,070, "Electric
Signaling System."**
PCM, ten years before Bell Labs and thirty before it was practical.

**ITU-T Q.700 series, *Specifications of Signalling System No. 7*.**
The SS7 specifications. Q.700 is the introduction and gives the architecture in
about twenty pages, which is enough for the purposes of §12.3.

**Bell System Technical Journal, Vol. 61 No. 7 (1982), the SS7 issue.**
The design rationale, written by the people who did it. Freely available online, and
better on *why* than any textbook.

## Books

**Bellamy, J. C. (2000). *Digital Telephony*, 3rd ed. Wiley.**
The reference for §12.2. Chapters 3 and 4 cover PCM, companding, and the digital
hierarchies completely, including the robbed-bit and clear-channel material that
produces the 56/64 distinction.

**Russell, T. (2006). *Signaling System #7*, 5th ed. McGraw-Hill.**
The standard practitioner's book on SS7. Thorough, and useful for understanding
exactly what an SCP query does during call setup — which is what makes number
portability and toll-free service work.

**Freeman, R. L. (2004). *Telecommunication System Engineering*, 4th ed. Wiley.**
Chapter 1 covers traffic engineering with worked Erlang B and C examples and the
tables that practitioners actually use.

**Angus, I. (2001). *An Introduction to Erlang B and Erlang C.* Telemanagement
Press.**
Short, practical, and aimed at people who must size a trunk group this week. Free
versions circulate; it is the most accessible treatment of §12.4.

**Brooks, J. (1976). *Telephone: The First Hundred Years.* Harper & Row.**
The institutional history — how the Bell System grew, how it was regulated, and how
its engineering decisions were shaped by its monopoly. Context for why the network
looks the way it does.

**Rosenbush, S. & Wolf, M. (contributors), and more usefully Coe, L. (1995).
*The Telephone and Its Several Inventors.* McFarland.**
Good on Strowger and on the priority disputes around the telephone itself.

## Historical and popular

**Lapsley, P. (2013). *Exploding the Phone.* Grove Press.**
The definitive history of phreaking, based on FBI files and interviews. Careful
about who discovered what and when, which most accounts are not, and good on
Joybubbles and the blind community's role. Genuinely entertaining and technically
accurate.

**Rosenbaum, R. (1971). "Secrets of the Little Blue Box." *Esquire*, October 1971.**
The article that introduced phreaking to a general audience and, by Wozniak's own
account, prompted him and Jobs to build one. Worth reading as a primary document.

**Standage, T. (1998). *The Victorian Internet.* Walker.**
Context for the pre-telephone era and for why multiplexing mattered so much.

## Security

**Nohl, K. (2014). "Mobile self-defense." Chaos Communication Congress 31C3.**
And **Engel, T. (2008/2014), SS7 location tracking, 25C3 and 31C3.**
The public demonstrations that made SS7's weaknesses widely known. Recordings are
freely available. Watching a live interception demonstration is more persuasive than
reading about one.

**GSMA FS.11, *SS7 Interconnect Security Monitoring Guidelines*.**
The industry's response. Useful for understanding what mitigations exist and how
partial they are.

**NIST SP 800-63B, *Digital Identity Guidelines: Authentication*.**
Section 5.1.3's discussion of out-of-band authenticators is where SMS's weakness is
formally acknowledged in a standards document, and it is the citation to use when
arguing against SMS-based two-factor authentication.

## Tools

**Any online Erlang B calculator**, or six lines implementing the recurrence in
§12.4. Writing it yourself takes five minutes and makes the trunking-efficiency
effect immediate — compute the circuits needed for 1, 10 and 100 erlangs at 1%
blocking and look at the utilisations.

**A VoIP bandwidth calculator**, or Chapter 3 §3.1's arithmetic done by hand.
exercise B8 of Chapter 12's point is that per-call bandwidth is not the codec rate, and doing
it once fixes that permanently.

## For the certification-minded

N10-009 expects the circuit-switched versus packet-switched distinction, T1/E1
rates and DS0 (objectives 1.2, 1.6). SS7 and Erlang are not examined.

Three things from this chapter that are worth carrying anyway:

1. **The 56 kb/s origin.** It explains a figure that appears throughout older
   material and looks like an error.
2. **Signalling separated from media**, which is objective 1.4's SIP and RTP in
   modern dress.
3. **SMS is a weak authentication factor** (objective 4.1), and §12.3 explains the
   mechanism rather than merely asserting it — which is what makes the
   recommendation defensible when someone asks why.
