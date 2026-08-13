# Chapter 18 — Exercises

## A. Recall

**A1.** State the one question ARP answers, in a single sentence.

**A2.** Why is an ARP request broadcast while the reply is unicast?

**A3.** What value appears in the target hardware address field of an ARP request,
and why?

**A4.** What EtherType identifies an ARP frame? What EtherType identifies IPv4?

**A5.** Name the four NDP message types and give each an IPv4 equivalent (or state
that it has none).

**A6.** What hop limit does every NDP message carry, and what does a receiver do with
one that arrives lower?

## B. Apply

**B1.** Host A is `10.20.30.40/24` with default gateway `10.20.30.1`. For each
destination, state **what address A ARPs for**, or state that no ARP is needed:

(a) `10.20.30.99`   (b) `10.20.31.99`   (c) `10.20.30.1`
(d) `8.8.8.8`   (e) `10.20.30.255`   (f) `127.0.0.1`

**B2.** Same host, but its mask has been misconfigured as `/16`. Answer (a)–(d)
again. Which answers changed, and what symptom does each change produce?

**B3.** Write out both frames of a complete ARP exchange in which host
`172.16.5.10` (`00:11:22:33:44:55`) resolves `172.16.5.1` (`00:aa:bb:cc:dd:ee`).
Give every Ethernet and ARP field.

**B4.** A host at `2001:db8:ac10:1::7:e5f1` needs to be reached by NDP. Compute:

(a) its solicited-node multicast address
(b) the destination MAC address of the frame carrying the solicitation

**B5.** A capture from host `192.168.1.50/24` shows repeated ARP requests for
`192.168.1.1`, `192.168.1.2`, `192.168.1.3`, … through `192.168.1.254`, in about
four seconds. Give two plausible explanations and one observation that would
distinguish them.

**B6.** A switch's MAC ageing timer is 300 seconds; a router's ARP timeout is 4 hours.
A server behind the switch is silent for ten minutes, then the router sends it a
packet. Trace what happens frame by frame and name the resulting symptom.

## C. Analyse

**C1.** ARP has no authentication. Explain concretely why adding it in 1982 would
have been difficult, and why the design was reasonable given its deployment context.
Then state what changed to make it unreasonable.

**C2.** A gratuitous ARP and an ARP spoof are the *same message*. Explain what
distinguishes them, and what this implies about defences that work by inspecting
message contents alone.

**C3.** Dynamic ARP Inspection depends on DHCP snooping. Explain the dependency, and
explain what happens to statically-addressed servers when DAI is enabled without
provision for them.

**C4.** IPv6 replaced broadcast address resolution with solicited-node multicast.
Quantify the benefit for a segment of 1,000 hosts in which each host resolves 20
neighbours, in terms of **CPU interrupts across all hosts**. State your assumptions.

**C5.** NDP's hop-limit-255 check prevents off-link attacks at zero cost. Explain the
mechanism, and explain precisely which attacks it does **not** prevent.

**C6.** Proxy ARP solves a real problem and is nonetheless deprecated. Identify the
problem it solved, explain why that problem no longer exists, and give the specific
troubleshooting symptom it produces when left enabled.

## D. Design

**D1.** You are given a /24 with 200 hosts and told to reduce broadcast traffic. You
may not add routers. What can you do, what would each measure gain, and what would you
recommend?

**D2.** Design the ARP-security posture for the Meridian Logistics network of the
semester project: which features on which ports, what the exceptions are, and what
you would monitor. Justify each choice against a specific attack.

**D3.** An IPv6 deployment is planned for a campus. Write the access-port
configuration policy for NDP-related switch features, and state the failure each item
prevents.

## E. Troubleshoot

**E1.** A user reports that the network "works for a few minutes then stops, then
works again". `arp -a` shows the gateway's mapping changing between two MAC addresses.
Give three candidate causes and the command that distinguishes them.

**E2.** `ping 192.168.10.55` fails from a host on `192.168.10.70/24`. `ip neigh`
shows `192.168.10.55 dev eth0 INCOMPLETE`. What has been ruled out, what has not, and
what is the next command?

**E3.** Same setup, but `ip neigh` shows a valid `REACHABLE` entry and `ping` still
fails. What does this tell you, and where do you look next?

**E4.** A newly-deployed IPv6 host has a link-local address but no global address.
`ping6 ff02::2%eth0` returns nothing. Give the two most likely causes and how to
confirm each.

**E5.** After a firewall change, IPv6 connectivity across the whole site fails while
IPv4 is unaffected. Hosts show `dadfailed` on some addresses and no default route.
What was almost certainly changed, and what is the correct policy?

**E6.** A host reaches a server on a different subnet without any route to it and
without a default gateway configured. Explain how this is possible, and give the
command that confirms your explanation.

## F. Extend

**F1.** Read RFC 826 in full — it is four pages. Identify one design decision that
made sense in 1982 and does not now, and one that has aged perfectly.

**F2.** Using `arping -D`, `ip neigh` and a packet capture, demonstrate a duplicate
address on a lab segment and document the symptom from three different hosts' points
of view. Explain why the reports disagree.

**F3.** SEND (RFC 3971) solves the authentication problem and was never deployed. Find
one other security mechanism in this book with the same history and explain what the
two cases have in common.

**F4.** Instrument a quiet lab segment for ten minutes and count ARP frames.
Extrapolate to 500 hosts and to 5,000 hosts, and compare with your answer to C4.
