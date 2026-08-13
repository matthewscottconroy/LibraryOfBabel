# Chapter 45 — The People

This chapter has fewer named individuals than most, and the reason is worth stating: **WLAN
design is an engineering practice rather than a research field.** Its knowledge accumulated
in deployments, was refined by consultants who had to make buildings work, and was written
down mostly in certification material and vendor guides rather than in papers.

**Which does not make it less rigorous** — §45.3's capacity arithmetic is derivable from
Chapters 42–44 — but it does mean the credit is diffuse.

**Devin Akin.** Founder of the CWNP certification programme, and one of the people most
responsible for wireless design existing as a discipline rather than as folklore.

**His contribution was insisting on measurement.** Before the CWNA/CWNE material, wireless
"design" in most organisations meant placing access points where there was power and hoping.
**The survey methodology of §45.1 — predictive, passive, active, spectrum, each answering a
different question — is largely his framing**, and it is now standard.

**He has also been consistently blunt** about vendor marketing, particularly about the rate
figures of Chapter 44 §44.1 and about MU-MIMO's real-world benefit, which is a useful
corrective in a field where the datasheets are optimistic.

**Keith Parsons.** Wireless training and the **"more access points at lower power"** argument
of §45.3, which he has spent two decades explaining to people who find it counter-intuitive.

**The argument is genuinely hard to accept** — every instinct says that a coverage problem is
solved by more signal — and **the three compounding reasons are not obvious until laid out.**
Parsons's contribution is pedagogical persistence.

**Andrew von Nagy.** The **capacity-planning methodology** of §45.3 — devices per user,
active fraction, demand per application, and **the client-count constraint that binds before
throughput does.**

**His observation that client count rather than bandwidth determines access-point count** is
the single most consequential idea in modern WLAN design, and it is why lecture theatres and
stadiums look the way they do.

**Jim Vajda, Jennifer Minella, and the practitioner community.** Much of what this chapter
contains circulates as blog posts, conference talks and forum answers rather than as books.

**Minella's work on wireless security and on the intersection with enterprise policy** is
particularly worth following, and her book on network authentication is the practical
treatment of Chapter 44 §44.3's enterprise cases.

**The IEEE 802.11k, 802.11v and 802.11r task groups.** §45.2's three amendments, and they are
a good example of **standardising something the market had already solved badly.**

**Before 802.11r, every vendor had a proprietary fast-roaming mechanism** — Cisco's CCKM,
others' equivalents — **and none interoperated.** A voice handset from one vendor roamed
quickly on that vendor's infrastructure and slowly on anyone else's.

**802.11r's value is interoperability rather than novelty**, which is the same contribution
802.11 itself made (Chapter 43's notes on Vic Hayes) and is the recurring role of a standards
body in this book.

**And its adoption difficulty is instructive:** **some older clients refused to associate to
an SSID advertising 802.11r at all**, which meant deploying the standard broke devices that
worked before. **A correct improvement with a deployment cost**, and Chapter 28's incentive
argument applies — the organisation deploying it bears the compatibility risk, and the benefit
is to its own users, which is why adoption was eventually fine.

**The Ekahau, AirMagnet and Hamina developers.** Survey software, and it changed the practice
of the field.

**Before it, a survey meant walking with a laptop and writing numbers on a floor plan.**
Predictive modelling made it possible to estimate a deployment before building it, and
**the visual heat map made wireless coverage legible to people who are not radio
engineers** — which matters enormously for getting a deployment funded.

**The risk it introduced** is §45.1's: **a predictive survey is easy to produce and easy to
believe**, and a plan built on wrong wall data looks exactly as authoritative as a correct
one.

**The Wi-Fi Alliance's Voice-Enterprise certification.** An unglamorous programme that tests
whether a device actually implements 802.11k, v and r correctly and roams within the time
voice requires.

**Its value is that it is testable.** "Supports 802.11r" is a claim; **passing an
interoperability test against a suite of infrastructure is evidence** — and for anyone
deploying voice over WLAN, the certification list is more useful than the datasheets.
