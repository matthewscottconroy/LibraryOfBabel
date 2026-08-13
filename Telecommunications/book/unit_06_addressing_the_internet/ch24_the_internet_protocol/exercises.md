# Chapter 24 — Exercises

## A. Recall

**A1.** List six things IP does not guarantee, and the one thing it does.

**A2.** Give five reasons a router legitimately drops a packet.

**A3.** State the size of the minimum and maximum IPv4 header, and the field that
determines it.

**A4.** Give the IP protocol numbers for ICMP, TCP and UDP.

**A5.** What does the DF flag do, and which mechanism depends on it?

**A6.** In what units is the fragment offset expressed, and what constraint does that
impose?

**A7.** What does TTL actually count, and what is its initial value on Linux, Windows
and Cisco devices?

## B. Apply

**B1.** Decode this IPv4 header completely, giving every field's value:

```
45 00 05 dc 8b 42 20 00 3f 11 00 00 0a 14 1e 05 c0 a8 64 0a
```

**B2.** A 5000-byte IP packet must cross a link with MTU 1500. Give the complete
fragment table: for each fragment, the payload size, offset value, MF flag and total
length.

**B3.** Repeat B2 for an MTU of 576.

**B4.** A link loses 2% of packets independently. Compute the effective packet loss for
an unfragmented packet, and for one split into 4 fragments.

**B5.** A path contains: Ethernet (1500) → PPPoE (1492) → IPsec tunnel (−56) → VXLAN
(−50). Compute the effective end-to-end MTU and the largest TCP payload that fits.

**B6.** A `ping` reply shows `ttl=245`. Give the two most likely initial values and the
hop count implied by each. Which is more likely and why?

**B7.** Draw the traceroute exchange for a 4-hop path, showing the TTL of each probe,
which device responds, and what ICMP type it returns.

## C. Analyse

**C1.** Explain why a network that guarantees delivery cannot serve both live voice and
file transfer well. Be specific about what the guarantee forces.

**C2.** Explain why the IP header checksum covers only the header. Give both the
performance reason and the architectural reason.

**C3.** IPv6 removed the header checksum entirely. Justify this using the end-to-end
argument, and state what is lost.

**C4.** Give six reasons IPv4 fragmentation is considered a mistake. For each, say
whether IPv6's approach avoids it.

**C5.** Explain the PMTUD black hole completely: the sequence of events, why the symptom
is "small works, large hangs", and why it looks like an application problem.

**C6.** MSS clamping is described as ugly and universal. Explain what makes it a layer
violation, and why it is used anyway.

**C7.** "TTL does not prevent loops; it makes them survivable." Explain the distinction
and why it is the fundamental difference between Layer 2 and Layer 3 forwarding.

**C8.** Explain why high latency at an intermediate traceroute hop, followed by low
latency at the destination, is normal.

## D. Design

**D1.** Design the MTU policy for a data centre running VXLAN over a routed underlay.
Specify the underlay MTU, the tenant MTU, and justify the headroom.

**D2.** A site connects over an IPsec VPN and users report that file transfers hang.
Write the diagnostic procedure and the three candidate fixes in order of preference,
with the trade-offs of each.

**D3.** Write the firewall policy for ICMP at an Internet edge: which types and codes
must be permitted inbound and outbound, and the failure each permission prevents.

**D4.** For the semester project's network, specify the MTU at every link and identify
any point where encapsulation could reduce the effective path MTU below 1500.

## E. Troubleshoot

**E1.** A user can SSH to a server and log in, but `scp` of a 4 MB file hangs after
transferring a few kilobytes. Diagnose, giving the exact command that confirms it.

**E2.** A website loads its HTML but no images. `ping` and DNS both work. Same diagnosis
as E1 — explain why the symptom differs.

**E3.** `traceroute` to a server shows `* * *` at hops 3, 4 and 5, then normal replies
at 6 and 7, and the destination is reachable. What is happening at hops 3–5?

**E4.** `traceroute` returns nothing at all past the first hop, but `curl` to the
destination works. Give the cause and the flag that works around it.

**E5.** `traceroute` output shows hops alternating between `10.0.0.1` and `10.0.0.2`
for fifteen hops, then `!H`. Diagnose precisely.

**E6.** After enabling jumbo frames on a server, it can reach some hosts and not others.
Explain, and state what must be verified.

**E7.** DNSSEC validation fails for one zone while ordinary DNS works everywhere.
Connect this to the chapter.

**E8.** A firewall was configured to drop all fragments. Which of the following break,
and why: DNS, NFS over UDP, IPsec, HTTP, VoIP?

## F. Extend

**F1.** Capture a packet, print it in hex, and decode the IP header entirely by hand
before checking against Wireshark. Repeat until it takes under three minutes.

**F2.** Use `tracepath` and manual `ping -M do -s` binary search to find the path MTU to
three different destinations. Explain any that are below 1500.

**F3.** Read RFC 8900 (*IP Fragmentation Considered Fragile*) and summarise its
recommendations in one page. Identify which of §24.3's six problems it emphasises most.

**F4.** Read RFC 3514 (the evil bit). Explain the serious point it makes, and find one
real proposed security mechanism that makes the same error.

**F5.** Run `mtr` to a distant destination for ten minutes and analyse the output.
Distinguish hops that are lossy from hops that merely rate-limit ICMP, and explain how
you can tell.
