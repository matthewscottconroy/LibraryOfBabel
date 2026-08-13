# Chapter 57 — Exercises

## A. Recall

**A1.** Name the five threat actor categories, and rank them by how likely a typical
organisation is to encounter each.

**A2.** State the four controls that defeat opportunistic attacks, in order of effect.

**A3.** Give the eight steps of the ransomware business model, and identify which one is the
network's step.

**A4.** Why is "we have backups" not a sufficient answer to ransomware? Give two reasons.

**A5.** Derive the CIA triad from the three verbs, and state why the list is exhaustive.

**A6.** Why can a checksum not provide integrity against an adversary? State the requirement it
lacks.

**A7.** What does encryption not protect? List four things.

**A8.** Why is availability the hardest of the three properties to defend?

**A9.** Explain why a 40 Gb/s attack against a 1 Gb/s circuit cannot be filtered by your
firewall.

**A10.** Distinguish security ends from security mechanisms, and classify authentication,
authorisation, non-repudiation and logging.

**A11.** Give the four responses to a risk, and say which is most under-used and why.

**A12.** Define SLE, ARO and ALE, and state what the arithmetic is bad at.

**A13.** State the proportionality test in one sentence.

**A14.** What four parts must a documented risk acceptance have?

**A15.** Name three data link layer attacks and the control for each.

**A16.** What is BCP 38, why does it work, and why is it not universally deployed?

## B. Apply

**B1.** For each organisation, identify the two most relevant threat actors and the single
control you would fund first:

(a) A 40-person architecture practice
(b) A regional hospital trust
(c) A defence subcontractor with 200 staff
(d) A municipal water utility
(e) An online retailer with 3 million customers

**B2.** For each system, rank confidentiality, integrity and availability and justify:

(a) A railway signalling network
(b) A law firm's document management system
(c) A public transport departure board
(d) A bank's transaction ledger
(e) A network's own management plane
(f) A university's research computing cluster

**B3.** Compute ALE for each risk and rank them:

| Risk | SLE | ARO |
|---|---|---|
| Ransomware | £750,000 | 0.06 |
| WAN circuit failure, one day | £9,000 | 3 |
| Laptop lost or stolen | £2,500 | 20 |
| Insider data theft | £180,000 | 0.1 |
| DDoS, 4 hours | £30,000 | 0.5 |

(a) Give the ALE for each and the ranking.
(b) Which would you fund first, and does the ranking agree with your instinct?
(c) One of these is poorly served by ALE. Identify it and explain.

**B4.** A control costs £60,000 a year and is estimated to reduce the ransomware risk in B3 by
75%.

(a) Compute the net value.
(b) Recompute if the ARO is actually 0.12.
(c) The control also reduces the insider risk by 50% and the DDoS risk by 20%. Recompute the
net value counting all three.
(d) State what this exercise demonstrates about how controls are usually costed.

**B5.** Classify each as mitigate, transfer, accept or avoid, and say whether you agree:

(a) Buying cyber insurance
(b) Decommissioning an unused FTP server
(c) Deploying MFA on the VPN
(d) Documenting that an EOL industrial controller will remain in service until 2029
(e) Moving email to a hosted provider
(f) Switching off a public API nobody uses

**B6.** Rewrite each of these register entries as a scenario rather than a missing control:

(a) "No network segmentation"
(b) "Weak passwords"
(c) "Unpatched firewalls"
(d) "No DDoS protection"
(e) "Single point of failure in the core"

**B7.** For a described network — an office of 150 people, one comms room, a firewall pair,
Internet access, Wi-Fi, a server room and a site-to-site VPN to a branch — enumerate the attack
surface layer by layer using §57.4's table. For each entry, state whether the exposure is
present, what control addresses it, and how you would detect exploitation.

**B8.** An organisation's public IP range is scanned continuously. Design an experiment (safe
and legal on your own infrastructure) to measure how quickly a newly exposed service is found
and what is attempted. State what you would record and what you would conclude from each
outcome.

## C. Analyse

**C1.** The chapter claims the three verbs are exhaustive. Test this: attempt to construct a
fourth thing an adversary can do to a channel, and either succeed or explain why your candidate
reduces to one of the three.

**C2.** Analyse the claim that most organisations under-invest in defeating opportunists while
over-investing elsewhere. Why does this misallocation occur? Consider incentives, marketing,
visibility and how security budgets are approved.

**C3.** Segmentation is described as the specific countermeasure to ransomware. Analyse this:
what exactly does it prevent, what does it not prevent, and what does it cost operationally?
Is it the best available control, or merely the network's contribution?

**C4.** Analyse the conflicts between the three properties. Choose two conflicts and, for each,
describe a real system where the resolution differs, and say what determines the resolution.

**C5.** Fail-open versus fail-closed. Analyse the choice for: a firewall, an 802.1X-protected
switch port, a certificate validation check, and an industrial safety interlock. State the
principle that determines the answer.

**C6.** Analyse the incentive structure that has prevented BCP 38's universal deployment.
Identify two other problems in this book with the same structure, and propose what could change
the outcome.

**C7.** The chapter states that ALE handles the tail badly. Analyse this limitation
mathematically and practically, and propose how an organisation should treat existential risks
in a register that otherwise uses expected values.

**C8.** "Compliance is not security." Analyse the relationship honestly: what does compliance
achieve that risk assessment does not, where does it actively harm, and how should an engineer
allocate effort between them?

**C9.** Analyse why every Layer 2 protocol in this book authenticates nothing. Was this a
mistake, a reasonable decision in context, or an unavoidable consequence of something? What
would it have cost to do otherwise in 1985?

## D. Design

**D1.** Produce a threat model for a described organisation of your choosing (or your own):
assets, actors, the three properties ranked per asset class, the attack surface, and the five
controls you would fund first with reasons. Keep it to three pages.

**D2.** Design the argument you would make to a finance director for a £120,000 security
programme, using the proportionality framework. Include the arithmetic, the assumptions, the
sensitivity of the conclusion to those assumptions, and the three things you would say when the
numbers do not support the spend but you believe it is right anyway.

**D3.** Design a risk register for a network team: the fields, three fully worked example
entries, the review cadence, and the process by which an entry is added, changed or closed.
State how you would prevent it becoming a shopping list.

**D4.** Design the physical security controls for a comms room and a server room, given a
£4,000 budget. Justify each item against a specific attack from §57.4's physical row, and state
what you deliberately did not buy.

**D5.** An organisation asks you to "make the network secure." Write the one-page response that
converts this into a tractable programme: what you would ask, what you would assess, what you
would deliver, and in what order.

## E. Troubleshoot

**E1.** An organisation has a firewall, an IPS, endpoint protection and annual penetration
testing, and is compromised through a reused password on a VPN with no MFA. Analyse where the
threat model failed.

**E2.** Ransomware encrypts a file server. The backups, which ran successfully every night, are
also encrypted. Explain the mechanism and state the three design changes required.

**E3.** A network is fully segmented and an attacker still reaches every system. Give three
mechanisms by which this can happen despite the segmentation.

**E4.** A DDoS attack of 12 Gb/s is directed at a service behind a 500 Mb/s circuit. Describe
what happens, what the on-premises firewall can do, and what the actual remedy is.

**E5.** Traffic between two internal systems is encrypted, and an investigator nonetheless
determines which systems communicated, when, and roughly how much data moved. Explain, and say
what would have prevented it.

**E6.** An attacker replays a correctly authenticated management command and it is executed.
Explain what the protocol lacked and name the three mechanisms that would prevent it.

**E7.** A helpdesk resets an MFA factor for someone impersonating an employee, and the account
is compromised. Analyse this as a control failure: which control failed, and is the fix
technical?

**E8.** A risk was accepted three years ago and has now materialised. The acceptance is
documented but nobody can find an owner or a review date. Analyse the consequences and state
what the acceptance record should have contained.

## F. Extend

**F1.** Set up a honeypot on a public address (a low-interaction one such as `cowrie` or
`opencanary`, and only on infrastructure you control and are permitted to expose). Record the
time to first scan, the time to first exploit attempt, and the distribution of what is
attempted over a week. Report the results against §57.1's claims.

**F2.** Obtain a current breach or incident report (the Verizon DBIR, an insurer's claims
analysis, or a national CERT's annual report) and extract the distribution of initial access
vectors. Compare with §57.1's claims and note any divergence.

**F3.** Perform an exposure assessment of an organisation you are authorised to assess: what is
reachable from the Internet, on what ports, running what versions. Use only passive sources
(Shodan, Censys, certificate transparency logs, DNS) unless you have explicit permission to
scan. Report what an opportunist would find.

**F4.** Read RFC 2827 (BCP 38) and RFC 3704. Determine whether your own provider implements
source address filtering — the Spoofer project provides a client that measures this — and report
the result.

**F5.** Build a risk register for a system you know well, with at least eight entries stated as
scenarios. Compute ALE for each. Present the ranking and identify at least one case where the
arithmetic disagrees with the organisation's actual spending.

**F6.** Read the CISA Known Exploited Vulnerabilities catalogue and determine how many entries
affect equipment classes present in a network you know. Compare the count with the number of
high-CVSS vulnerabilities for the same equipment, and comment on the difference in
actionability.

**F7.** Conduct a tabletop exercise for a ransomware scenario with colleagues: the initial
alert, the decisions in the first hour, who has authority, and what information is needed.
Record every point at which the answer was "we would have to find out."
