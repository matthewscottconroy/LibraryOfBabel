# Chapter 65 — Exercises

## A. Recall

**A1.** Give the four interface state combinations and what each means.

**A2.** Why must `err-disabled` never be cleared without reading the cause?

**A3.** What is a split pair, why does a continuity tester miss it, and what are its three
symptoms?

**A4.** State the copper distance limits for 1000BASE-T and 10GBASE-T, and say what the figure
includes.

**A5.** Distinguish CRC errors accompanied by late collisions from CRC errors without them.

**A6.** List the eight causes of a fibre fault in probability order, and name the one that is
free to check.

**A7.** What is receiver overload, and what is the fix?

**A8.** Give three PoE failure modes and the command that investigates them.

**A9.** Why is "no IP address" very often a VLAN problem?

**A10.** Give the three trunk faults and the distinct signature of each.

**A11.** What does the spanning tree topology change counter tell you, and why is it the most
useful STP diagnostic?

**A12.** State the duplex rule in one sentence, and the two counters that prove a mismatch.

**A13.** What does a MAC flapping message mean, and why is it frequently ignored?

**A14.** Distinguish a mask that is too narrow from one that is too wide, by symptom.

**A15.** When does asymmetric routing become a fault?

**A16.** Why does an OSPF adjacency stuck at ExStart indicate an MTU mismatch?

**A17.** What is NAT hairpinning, and what is the better fix?

**A18.** Distinguish "connection refused" from "connection timed out".

**A19.** Why does shortening a DHCP lease not fix pool exhaustion?

**A20.** What does a consistently round delay — exactly 5 or exactly 30 seconds — indicate?

## B. Apply

**B1.** For each interface output, give the diagnosis and the next action:

(a) `Gi1/0/12 is up, line protocol is down`
(b) `Gi1/0/14 is administratively down`
(c) `Gi1/0/20 is down, line protocol is down (err-disabled)`
(d) `Gi1/0/8 is up, line protocol is up`, 0 errors, and the user cannot reach anything
(e) `Gi1/0/3 is up, line protocol is up`, 48,000 CRC errors, 0 collisions
(f) `Gi1/0/5 is up, line protocol is up`, 3,200 CRC errors, 1,900 late collisions

**B2.** A subnet is 10.20.4.0/22 and a host is configured 10.20.5.14/24.

(a) What does the host believe its network is?
(b) Is 10.20.6.9 reachable? By what path?
(c) Now the host is configured 10.20.5.14/16. Is 10.30.0.5 reachable? Explain.
(d) State which of the two errors produces a hard failure and which produces a subtle one.

**B3.** Read this transceiver output and give the diagnosis for each port, with the next action:

```
   Port     Tx Power   Rx Power   Sensitivity  Type
   Te1/0/1  -2.1 dBm   -18.4 dBm  -21 dBm      10GBASE-LR
   Te1/0/2  -2.3 dBm   -31.2 dBm  -21 dBm      10GBASE-LR
   Te1/0/3  -2.0 dBm    -1.8 dBm  -21 dBm      10GBASE-ER (40 km) on 200 m
   Te1/0/4  -2.2 dBm    no signal -21 dBm      10GBASE-LR
```

**B4.** A site's DHCP scope covers 10.20.5.100 to 10.20.5.200 with a 24-hour lease. The site has
grown to 145 devices.

(a) How many addresses are in the pool?
(b) By how many is the site short?
(c) An engineer proposes reducing the lease to 4 hours. Assess.
(d) Give two solutions that would work and state the consequence of each.

**B5.** For each symptom, state whether the cause is most likely Layer 1, 2, 3 or above, and
give the single command you would run first:

(a) A device shows 169.254.8.4
(b) Local hosts reachable, nothing else
(c) Everything works except one destination on the same subnet
(d) Throughput is 4 Mb/s on a 1 Gb/s link
(e) A link comes up and goes down every 90 seconds
(f) `ping` by IP works, by name does not
(g) The page loads and the images do not
(h) One VLAN works within a building and not between buildings

**B6.** An OSPF adjacency between two routers reaches ExStart and stays there.

(a) Give the most likely cause.
(b) Give two commands that would confirm it.
(c) Give three other causes of adjacency failure and how each would present differently.

**B7.** A web service is reachable from the Internet and not from inside the organisation.

(a) Explain the mechanism.
(b) Give two fixes.
(c) State which you would prefer and why.

**B8.** For each error, state the cause and whether it is a network fault:

(a) `curl: (60) unable to get local issuer certificate`
(b) `Connection refused`
(c) `Connection timed out`
(d) `SERVFAIL`
(e) `KRB_AP_ERR_SKEW`
(f) `169.254.x.x` on a single machine in an otherwise healthy VLAN

## C. Analyse

**C1.** The chapter states that most incidents end at Layer 4 or above. Analyse why the network
is nonetheless the first thing suspected, and what a network team should do about it
structurally rather than case by case.

**C2.** Analyse the mask-too-narrow case: it produces a working network with an extra hop and an
ICMP redirect. Under what circumstances does it become a real problem, and how long could it
persist undetected?

**C3.** Proxy ARP hides subnet mask errors. Analyse this as a general pattern — a mechanism that
compensates for a misconfiguration and thereby conceals it — and find two other examples in this
book.

**C4.** Analyse why "turn off IPv6" is the reflexive response to IPv6 faults, what it costs, and
how you would argue against it to a colleague under time pressure.

**C5.** The chapter says the most valuable contribution is frequently proving the network is not
at fault, and warns against triumph. Analyse the professional dynamics: why does this create
friction, and how should the finding be presented?

**C6.** Analyse the "exactly N seconds" signal. Why do timeouts produce round numbers, what does
each common value correspond to, and why is a consistent delay stronger evidence than an
inconsistent one?

**C7.** Duplex mismatch is described as rare and not extinct. Analyse where it still occurs,
why auto-negotiation did not eliminate it entirely, and what configuration practice would.

**C8.** Analyse the diagnostic value of the distinction between "refused" and "timed out". What
does each tell you about how far the packet travelled, and why is the distinction so frequently
ignored?

## D. Design

**D1.** Build a symptom-to-cause reference card for a service desk: one page, organised by what
the user says rather than by layer, with the first check for each. It must be usable by someone
who has not read this book.

**D2.** Design the standard first-response procedure for "a user cannot reach a service":
the questions, the commands, the decision points, and the escalation criteria. It should
determine the layer within five minutes.

**D3.** Design the monitoring that would detect each of this chapter's faults before a user
reports it. For each of ten faults, state the signal, the threshold and the alert (Chapter 54
§54.4's rules apply — every alert must be actionable).

**D4.** An organisation has recurring "no IP address" reports across several sites. Design the
investigation and the permanent remediation, addressing VLAN configuration, DHCP capacity,
monitoring and documentation.

**D5.** Design the physical-layer verification procedure for a new cabling installation: what is
tested, with what, to what standard, what is recorded, and what constitutes a failure. Include
what you would test for that a continuity tester cannot find.

## E. Troubleshoot

**E1.** A user's port shows `up/up`, they have a valid address, they can ping their gateway, and
they cannot reach anything else. Give your next four checks in order.

**E2.** An access point reboots each morning at about 08:30. It is healthy overnight. Diagnose.

**E3.** A newly-installed 10 Gb/s fibre link between two switches will not come up. Both
transceivers show normal Tx power and no Rx signal. Give three causes and how to distinguish
them.

**E4.** After a VLAN was added, devices in it can communicate within each building and not
between buildings. Diagnose and give the exact configuration to check.

**E5.** A file server is reachable and file transfers stall at about 90% every time. Diagnose,
and state which chapter treats the mechanism.

**E6.** Every user in one VLAN experiences a two-second outage roughly every ten minutes.
Diagnose and give the command that names the cause.

**E7.** A branch office reports that everything is slow. `iperf3` to the data centre shows
940 Mb/s. Users still complain. Give three explanations and how to test each.

**E8.** A partner's application works from your DMZ and fails from your internal network. Both
have routes and firewall rules. Give three possible causes.

**E9.** An engineer replaces a failed switch with an identical spare, restores the configuration,
and two ports do not work. Diagnose.

**E10.** Authentication fails for every user in one site, and works elsewhere. The RADIUS server
is healthy. Give the first thing you would check.

## F. Extend

**F1.** Deliberately create each of the following in a lab and record the exact symptom and the
counters produced: a duplex mismatch, a VLAN mismatch, a native VLAN mismatch, a wrong subnet
mask (both directions), and a missing route. Build a personal reference.

**F2.** Read the interface counters on every switch you have access to and identify every port
with a non-zero CRC, late collision or discard count. Investigate the three worst and report
what you found.

**F3.** Read the transceiver power on every fibre link you can reach and compare each against
the optic's documented sensitivity. Report the margin for each and identify any within 3 dB.

**F4.** Examine the spanning tree topology change counters across a switched estate. Identify the
port generating the most changes and determine why.

**F5.** Audit DHCP pool utilisation across every scope in an environment you administer. Report
the utilisation, the growth rate, and the date each scope would exhaust at current growth.

**F6.** Take a real application performance complaint and produce the three-part demonstration in
§65.4: capture at both ends, show the request arriving and the response being sent, and measure
the server-side time. Report the result and how it was received.

**F7.** Create an MTU black hole in a lab (a tunnel with ICMP filtered) and observe the symptom
from the application's side before diagnosing it from the network's. Write down what a user
would report.

**F8.** Read your organisation's last twenty incident records and classify each by the layer at
which the fault actually was. Compare the distribution with where the investigation began.
