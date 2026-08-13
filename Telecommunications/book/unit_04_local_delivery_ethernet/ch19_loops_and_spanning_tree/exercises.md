# Chapter 19 — Exercises

## A. Recall

**A1.** State the three properties of Ethernet that together make a switching loop
catastrophic rather than merely inefficient.

**A2.** Name the three simultaneous failures a loop causes, and give the log message
that indicates the second.

**A3.** What are the two components of a bridge ID, in order of comparison, and what
is the default value of the first?

**A4.** List the five STP tie-breakers in order.

**A5.** Give the classic 802.1D port states with their durations, and the RSTP states
they collapse into.

**A6.** What are the four RSTP port roles?

**A7.** Which LACP mode combinations form a bundle, and which does not?

## B. Apply

**B1.** Four switches with these bridge IDs and 1 Gb/s links (cost 4) throughout:

```
        SW-W (32768, 00:00:0c:11:11:11)
        /                    \
    SW-X (32768, …:22:22:22)  SW-Y (24576, …:33:33:33)
        \                    /
        SW-Z (32768, …:44:44:44)
```
plus a direct SW-X–SW-Y link.

(a) Which switch is root, and why?
(b) Identify every port's role.
(c) Which port blocks?

**B2.** Same topology, but the SW-W–SW-Y link is 100 Mb/s (cost 19). Redo (b) and (c).

**B3.** A switch's cost to the root is 8. It receives a BPDU on another port
advertising a root path cost of 4, and that port is 1 Gb/s. Should the port become the
new root port? Show the arithmetic.

**B4.** A network has a diameter of 5 switches. Compute the worst-case classic-STP
convergence time after a root failure, and the equivalent under RSTP. State your
assumptions.

**B5.** A four-member 1 Gb/s bundle hashes on source+destination IP. Two servers, A
and B, run a single 40 GB backup between them.

(a) What throughput will the transfer achieve?
(b) What throughput would eight parallel connections achieve, assuming an even hash?
(c) What if the hash were source-MAC only?

**B6.** An MSTP deployment has instance 1 → VLANs 1–100, instance 2 → VLANs 101–200,
region name `CAMPUS`, revision 3. A new switch is configured identically except that
VLAN 150 is mapped to instance 1. Describe precisely what happens.

## C. Analyse

**C1.** Explain why RSTP's proposal/agreement handshake is *safe* — that is, why it
cannot create a transient loop despite skipping the timers. Your answer must identify
the specific action that provides the guarantee.

**C2.** A duplex mismatch causes RSTP to revert to 802.1D timing. Explain the causal
chain, and explain why this makes duplex mismatches unusually hard to diagnose.

**C3.** PortFast without BPDU Guard is described as "a loop waiting to happen".
Construct the specific sequence of events, naming the physical action a user takes.

**C4.** Loop Guard exists because a blocked port that stops receiving BPDUs would
otherwise start forwarding. Explain why the original designers made silence mean "no
loop", and why that inference is wrong on fibre.

**C5.** Compare STP and link aggregation as answers to the parallel-links problem.
Under what circumstances would you deliberately choose STP blocking over aggregation?

**C6.** Aggregation hashes rather than round-robins, at the cost of single-flow
throughput. Explain what would break if it round-robined, being specific about which
protocol and which mechanism.

**C7.** Perlman later argued that Layer 2 networks should be routed rather than
bridged, and the industry eventually agreed. Explain what routing provides that
spanning tree cannot, using two specific mechanisms.

## D. Design

**D1.** Design the STP configuration for a three-tier campus: two core switches, four
distribution switches, twenty access switches. Specify priorities, which guards go on
which ports, and justify each choice against a named failure.

**D2.** A pair of core switches connects to a pair of distribution switches with four
links total. Design the connectivity using aggregation and/or STP, state what fails
under each single-component failure, and give the convergence time in each case.

**D3.** For the Meridian Logistics network of the semester project, write the access-
port template. Every line must be justified by a failure it prevents or a delay it
removes.

**D4.** Your organisation is building a new data centre and asks whether to run a
large bridged fabric with MSTP or a routed leaf-spine. Write the recommendation, with
the trade-offs stated honestly.

## E. Troubleshoot

**E1.** All ports on two switches show solid LEDs, no host has connectivity, and you
cannot SSH to either switch. State exactly what you do, in order, and what equipment
you need.

**E2.** The logs show `MACFLAP_NOTIF` for one MAC address flapping between Gi0/5 and
Gi0/12, but the network is otherwise working. What is happening, and why is it not a
storm?

**E3.** A user reports that their computer takes about 30 seconds after boot before it
can reach anything. Everything works afterwards. Diagnose, and give the configuration
line that fixes it and the one that must accompany it.

**E4.** A port went `err-disabled` at 09:14. What almost certainly happened, what do
you do before re-enabling it, and why is `errdisable recovery` the wrong first move?

**E5.** After a fibre uplink was repaired, a broadcast storm began. The fibre tests
fine on one strand. Explain, and name the two features that would have prevented it.

**E6.** A four-link bundle shows only two active members. Give the three most likely
causes in order and the command that distinguishes them.

**E7.** Traffic between two switches is severely imbalanced across a bundle: 900 Mb/s
on one member, under 50 Mb/s on the other three. Nothing is misconfigured. Explain,
and give two options.

## F. Extend

**F1.** Read Perlman's *Algorhyme* alongside the 802.1D specification and identify
which line of the poem corresponds to which clause. Note anything in the standard the
poem omits.

**F2.** Build a loop in a lab with two switches and two cables, with STP disabled.
Capture the first 200 milliseconds and plot frame count against time. Then enable STP
and repeat. Document the difference.

**F3.** Investigate TRILL (RFC 6325) and explain what it adds to Layer 2 that spanning
tree lacks. Then explain why VXLAN largely displaced it.

**F4.** Perlman has said she considers spanning tree a mistake in the sense that
bridging should never have scaled as far as it did. Argue for or against, using
evidence from this chapter and Chapter 17.
