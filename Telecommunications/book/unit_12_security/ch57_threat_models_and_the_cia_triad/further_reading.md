# Chapter 57 — Further Reading

## The four to read first

**Saltzer, J. & Schroeder, M. (1975). "The Protection of Information in Computer Systems."**
*Proceedings of the IEEE*.
**The single highest-value item in this unit's reading.** **Sections 1 and 2 take twenty
minutes** and contain the eight principles that everything since restates. **Free.**

**Ware, W. (1970). *Security Controls for Computer Systems*.** RAND R-609.
**The report that named the discipline**, declassified in 1979. **Read the summary** for the
structural argument that security cannot be added afterwards.

**Schneier, B. — *Beyond Fear* (2003).**
**About risk and proportion rather than about computers**, and it is the best available
treatment of §57.3's argument. **His five-step framework for evaluating any security measure is
worth memorising.**

**Shostack, A. — *Threat Modeling: Designing for Security* (2014).**
**The practical book.** **STRIDE, and — more usefully — a procedure a team can actually follow.**
D1 is much easier after reading it.

## Foundations

**Kerckhoffs, A. (1883). *La Cryptographie Militaire*.**
**Historical, short, and the second principle is worth reading in the original framing.**

**Anderson, J. (1972). *Computer Security Technology Planning Study*, and (1980) *Computer
Security Threat Monitoring and Surveillance*.**
**The reference monitor, and the origin of intrusion detection.** **The three properties —
complete mediation, tamper-proof, verifiable — are a useful test for any control.**

**Denning, D. (1987). "An Intrusion-Detection Model."** *IEEE Transactions on Software
Engineering*.
**Anomaly detection, and unusually honest about the false-positive constraint** — which is
Chapter 54 §54.4's alert fatigue, predicted.

**Anderson, R. — *Security Engineering* (3rd ed., 2020).**
**Free online.** **The standard graduate text, and it is enormous and readable.** **Chapters 1–3
are the general framework; the chapters on physical security, on banking and on distributed
systems are each better than most books on their subject.** **If you read one long book in this
unit, this one.**

## Threat data, used rather than admired

**The Verizon Data Breach Investigations Report**, annually.
**F2 uses it.** **Read the "Results and Analysis" section and the incident classification
patterns**, and ignore the marketing. **The initial-access distribution is the number that
should shape your spending.**

**CISA's Known Exploited Vulnerabilities catalogue** (cisa.gov/kev).
**F6 uses it.** **A far smaller and more actionable set than "everything with a high CVSS
score"**, and the right trigger for Chapter 55 §55.3's emergency patching track.

**MITRE ATT&CK** (attack.mitre.org).
**A catalogue of what attackers actually do, by technique, with real-world references.**
**Consult rather than read.** **Its greatest value is as a coverage check**: for each technique
relevant to your environment, do you prevent it, detect it, or neither?

**National CERT annual reports**, and **insurers' claims analyses.**
**The insurers' data is under-used and unusually honest**, because they are paying.

**Honeynet Project** material, for F1.

## Frameworks, used as prompts

**NIST Cybersecurity Framework 2.0.**
**Identify, Protect, Detect, Respond, Recover, Govern.** **Six words, and its value is as a
completeness check** — most organisations are heavy on Protect and thin on Detect and Recover,
and the framework makes that visible.

**CIS Critical Security Controls (v8).**
**The single most useful prioritised list available**, and **Implementation Group 1 is
approximately §57.1's "defeat the opportunists" list.** **Start here if you need a programme and
have no time.**

**ISO/IEC 27001 and 27005**, and **NIST SP 800-30 (risk assessment).**
**Consult if you are obliged to.** **SP 800-30's threat and vulnerability taxonomies are useful
prompts for D3's register**, and it is free.

**OWASP Top Ten**, for the application row of §57.4 — **outside this book's scope and worth
knowing exists.**

## The human layer

**Cialdini, R. — *Influence*.**
**Not a security book.** **It is the mechanism behind every social engineering technique**, and
reading it changes how phishing looks.

**Hadnagy, C. — *Social Engineering: The Science of Human Hacking*.**
The practitioner's treatment.

**Sasse, M. A., Brostoff, S. & Weirich, D. (2001). "Transforming the 'Weakest Link'."**
**The paper that argued users are not the problem — the systems are** — and it is Saltzer and
Schroeder's psychological acceptability, with evidence.

## Practical work

**A honeypot** — `opencanary`, `cowrie`, or a cloud provider's equivalent. **F1 uses one.**
**Only on infrastructure you control and are permitted to expose.** **The results are more
persuasive than any statistic**, because they are yours.

**Shodan and Censys** — **F3 uses them.** **Passive, legal, and the fastest way to see your own
organisation as an opportunist sees it.** **Certificate transparency logs (crt.sh) are the other
half**, and they routinely reveal hostnames nobody meant to publish.

**The Spoofer project** (caida.org/projects/spoofer) — **F4 measures whether your provider
implements BCP 38.**

**A tabletop exercise.** **F7 needs a room and an hour.** **It is consistently the highest-value
security activity per hour spent**, and it requires no tooling.

## On the limits of all this

**Herley, C. (2009). "So Long, and No Thanks for the Externalities: The Rational Rejection of
Security Advice by Users."**
**An argument that users who ignore security advice are frequently correct**, because the
advice's cost to them exceeds the expected loss. **Uncomfortable and rigorous**, and it is
§57.3's proportionality applied to the person rather than the organisation.

**Anderson, R. (2001). "Why Information Security is Hard — An Economic Perspective."**
**The paper that introduced security economics.** **§57.4's BCP 38 argument — the cost falling
on a different party from the benefit — is this paper's central thesis**, and it explains more
of the Internet's remaining problems than any technical analysis.

## Where to look next

**Chapter 58** supplies the cryptography that §57.2 says is required for confidentiality and
integrity; **Chapter 59** covers the authentication that §57.2 classifies as a mechanism;
**Chapter 60** implements §57.1's segmentation; **Chapter 61** covers the tunnels; and
**Chapter 62** walks §57.4's table in detail, from the attacker's side.
