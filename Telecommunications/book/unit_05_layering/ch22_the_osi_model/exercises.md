# Chapter 22 — Exercises

## A. Recall

**A1.** List the seven OSI layers from 1 to 7 and from 7 to 1, with a mnemonic for
each direction.

**A2.** Give the PDU name at layers 1, 2, 3 and 4.

**A3.** Name the two sublayers of Layer 2 and the standard that defines each.

**A4.** Give the layer of each: hub, switch, router, Layer 3 switch, wireless access
point, repeater, media converter.

**A5.** Give the layer of each: HTTP, TCP, IP, Ethernet, TLS, ARP, ICMP, UDP, DNS,
802.1Q.

**A6.** Name the three troubleshooting approaches from §22.4 and state when each is
appropriate.

**A7.** What is the difference between receiving a RST and receiving nothing, after
sending a SYN?

## B. Apply

**B1.** For each symptom, name the most likely layer and the first command you would
run:

(a) No link light
(b) `ping` to the gateway fails; `arping` to it succeeds
(c) `ping 8.8.8.8` works; `ping google.com` does not
(d) A browser reports a certificate error
(e) `nc -zv host 443` hangs with no response
(f) Two hosts on the same switch, same subnet, cannot reach each other
(g) Large file transfers hang; small ones work
(h) Everything worked yesterday; nothing was changed; nothing works today

**B2.** A user's email works and the intranet does not. List everything this proves
about layers 1–4, and state where you would start.

**B3.** Using divide-and-conquer, write the decision tree for diagnosing "I can't reach
the file server", with at most four tests to reach any leaf.

**B4.** Work the following through the §22.4 method, showing each step and inference:
a host that can ping its gateway, cannot ping `8.8.8.8`, and can ping another host on
its own subnet.

**B5.** Map every chapter of Units I–IV onto its OSI layer. Identify any that span two,
and explain.

## C. Analyse

**C1.** OSI's protocols failed and its model survived. Give three specific reasons for
the failure and two for the survival.

**C2.** "A government mandate, universal vendor support, and a technically thorough
specification lost to free software that already worked." Identify two other cases in
this book with the same shape and say what they have in common.

**C3.** Explain why layers 5 and 6 have no separate implementation in practice, and
why nothing is lost by the TCP/IP model omitting them.

**C4.** TLS is the standard exam answer for Layer 6 and does not fit there. Make the
case for placing it at 4, at 5, at 6 and at 7, then explain what the disagreement
reveals about the model.

**C5.** The diagnostic method's power is stated as: "a successful test at layer *n*
proves layers 1 through *n* are functioning." Prove this from the structure of
layering, and identify one case where it fails.

**C6.** Compute how many tests bottom-up and divide-and-conquer require in the worst
case for a seven-layer stack, and explain why the advantage grows with the size of the
problem space.

**C7.** OSI left behind X.509, ASN.1, LDAP and IS-IS. Explain why these survived when
the protocols did not, and what the pattern suggests about which parts of a standard
are durable.

## D. Design

**D1.** Write a one-page troubleshooting checklist for a helpdesk, organised by layer,
with the specific command for each check. It must be usable by someone with three
months' experience.

**D2.** Design the escalation criteria for a two-tier support organisation, expressed
in terms of layers: what tier 1 handles, what triggers escalation, and what
information must accompany it.

**D3.** For the semester project's network, write the layer-by-layer verification
procedure you would run after the initial build, with the expected output of each step.

**D4.** Your organisation's incident reports say "network issue" for 60% of outages.
Design a taxonomy based on this chapter that would make the reports useful, and explain
what you would learn from six months of data.

## E. Troubleshoot

**E1.** A host has a link light, a valid IP address and correct gateway, and cannot
ping anything including the gateway. `arping` to the gateway also fails. What layer,
and what are the three most likely causes?

**E2.** A server accepts connections on port 80 from inside the data centre and not
from the office. `nc -zv` from the office returns nothing at all. What does the silence
tell you, and what would a RST have told you instead?

**E3.** All users in one building report intermittent slowness. Some transfers hang
entirely. The switch shows no errors. Give a layer-ordered plan and state what you
expect at each step.

**E4.** A user reports "the internet is slow". Working top-down, give the first four
questions you would ask and what each eliminates.

**E5.** After a firewall change, one application broke and everything else works. State
which layers this eliminates, and why.

**E6.** `ping` works to a server; `curl https://server` fails with a timeout after the
TCP connection is established. Which layer, and what are two causes?

## F. Extend

**F1.** Read ISO 7498 (or a good summary) and find the committee's stated principles
for creating a layer boundary. Apply them to the TCP/IP four-layer model and see
whether it satisfies them.

**F2.** Find a current job advertisement for a network role and count the OSI layer
references. Then find a protocol specification from the last five years and do the
same. Explain the difference.

**F3.** Take three faults you have personally experienced, classify them by layer, and
determine which troubleshooting method would have found each fastest.

**F4.** Argue that the OSI model should no longer be taught, then argue that it must
be. Decide, and identify what would have to change for your answer to change.
