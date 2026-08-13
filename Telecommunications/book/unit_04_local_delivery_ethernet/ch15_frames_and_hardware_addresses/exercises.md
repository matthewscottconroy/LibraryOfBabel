# Chapter 15 — Exercises

## A. Recall

**A1.** Give three independent reasons why data is chopped into frames.

**A2.** Decode `00:1b:44:11:3a:b7`: which bytes are the OUI, is it unicast or
multicast, and is it universally or locally administered?

**A3.** State the minimum and maximum Ethernet frame sizes, and the maximum with
an 802.1Q tag.

**A4.** What EtherType values correspond to IPv4, ARP, IPv6 and an 802.1Q tag?

**A5.** What does a CRC detect, and what does it not do?

## B. Apply

**B1.** A 60 kB transfer over a link with a bit error rate of 5 × 10⁻⁷. Compute the
probability of an error-free transfer as (a) one continuous transmission and
(b) 1,500-byte frames. For (b), compute the expected number of frames requiring
retransmission and the expected retransmitted volume. Compare with (a).

**B2.** Derive Ethernet's 64-byte minimum from a 2,500 m segment at 2 × 10⁸ m/s and
10 Mb/s. Then compute what the minimum would need to be at 100 Mb/s over the same
distance, and explain why 100BASE-TX's maximum segment length is 100 m rather than
2,500 m.

**B3.** Compute the total bytes on the wire for a frame carrying a 46-byte payload
and for one carrying 1,500 bytes, including interframe gap, preamble and SFD. Give
the efficiency in each case.

**B4.** For each of these first bytes, state whether the address is unicast or
multicast and whether it is universally or locally administered: `00`, `01`, `02`,
`33`, `ff`, `4a`.

**B5.** Decode this frame header by hand:

```
0x0000:  ffff ffff ffff 001c 2312 34ab 0806 0001
0x0010:  0800 0604 0001 001c 2312 34ab c0a8 010a
```

Identify: destination, source, EtherType, and what the frame is. Look up the source
OUI if you can.

**B6.** A switch reports, on one interface over 24 hours: 1,204,882 input packets;
CRC errors 412; runts 0; giants 1,882; output drops 14,203. Diagnose each non-zero
counter separately, and state which indicates a physical fault, which indicates a
configuration problem, and which indicates congestion.

**B7.** Compute the efficiency gain of jumbo frames: 9,000-byte payload versus
1,500-byte, including all overhead. Then state the specific failure that results if
one device on the path does not support them, and the symptom a user reports.

## C. Analyse

**C1.** Explain why flat addressing cannot scale globally, using the aggregation
argument. Then explain why it is nonetheless the right choice for a local segment,
and state the general principle about what flat and hierarchical addressing each
buy and cost.

**C2.** A CRC detects all burst errors up to the length of the CRC itself.
Explain why that guarantee is particularly well matched to real channels, referring
to Chapter 6's impairments. Then construct an error pattern that CRC-32 would fail
to detect, and compute the probability of an arbitrary corruption escaping.

**C3.** Ethernet detects errors and does not correct them; deep-space links
correct heavily. Derive the decision rule from the cost of retransmission, and use it
to explain why 400GBASE-R added forward error correction after twenty years of
Ethernet not having it.

**C4.** A store-and-forward switch verifies the FCS and recomputes it on egress.
Explain the integrity gap this creates, why it is not hypothetical, and how the
end-to-end argument applies. Then explain why TCP's checksum exists despite
Ethernet's CRC being far stronger.

**C5.** MAC randomisation broke DHCP reservations, MAC filtering, captive portal
persistence and Wi-Fi analytics. For each, state what the organisation was actually
trying to achieve and what the correct mechanism is. Then evaluate the response of
instructing users to disable randomisation.

## D. Design

**D1.** You are specifying the access-layer configuration for a building with:

- 180 workstations (managed, domain-joined)
- 34 wireless access points
- 22 IP cameras
- 51 IP telephones
- Visitor devices on a guest SSID, unmanaged, with randomised MAC addresses

The security team has proposed MAC-based access control for all wired ports and MAC
filtering for the guest wireless.

Evaluate the proposal for each device class. For each, state what identity mechanism
you would use instead and why, addressing the devices that cannot support 802.1X.
Then state what MAC addresses *are* useful for in this design, and what your
frame-size and MTU policy should be given the presence of tagged frames on trunk
links.

## E. Troubleshoot

**E1.** A user reports that a file transfer to a server "takes forever" — about
18 Mb/s on a gigabit link. Investigation shows:

- Both ends negotiated 1000/full.
- `ping` to the server: min 0.28 ms, avg 0.31 ms, max 0.9 ms, mdev 0.06 ms, 0% loss.
- Link uptime 71 days, no flaps.
- The user's switch port: input packets 41,203,884; **CRC errors 8,204,551**;
  runts 0; giants 0; output drops 0.
- The server's port shows no errors.
- Utilisation on the user's port peaks at 4%.
- The cable was installed six years ago and passed certification then.

Compute the frame error rate. Using Chapter 3 §3.3's Mathis relation with the
measured RTT, estimate the expected TCP throughput and compare with the observed
18 Mb/s.

Then explain: why `ping` shows zero loss despite the error rate; why the server's
port shows no errors; why utilisation is a red herring; and what single physical
measurement you would take next. State what should have alerted someone six weeks
ago and why nobody noticed.
