# Lab 03 — Building a LAN; Hubs, Switches and Sharing

**Corresponds to:** Chapters 9, 11, 13, 17
**Week:** 3
**Time:** 90 minutes

---

## Objectives

- Build a working local network from components and verify it.
- Demonstrate empirically the difference between a hub and a switch, in
  bandwidth, in collisions and in privacy.
- Count collision domains and broadcast domains in a real topology.
- Observe statistical multiplexing gain in a measurement rather than in a
  formula.
- Distinguish physical from logical topology in a network you built yourself.

---

## You will need

- Three or more hosts.
- A switch. **A hub if your institution still has one** — they are worth
  borrowing, and §Part 2 explains what to do if you cannot get one.
- Wireshark or `tcpdump` on each host.
- `simnet.py` from [tools/](../tools/).

**Fallback for no hub:** a switch with a SPAN/mirror port configured to copy all
traffic to one port reproduces the *observability* half of hub behaviour, though
not the bandwidth sharing. Packet Tracer models hubs faithfully and is the better
substitute for Part 2. Say which you used in your debrief.

---

## Procedure

### Part 1 — Build it

**1.** Connect three hosts to the switch. Assign static addresses in one subnet —
`192.0.2.10/24`, `.11`, `.12` — with no gateway. Verify each can ping the others.

**2.** Draw the physical topology. Then draw the logical topology. Note whether
they differ and be prepared to defend your answer.

**3.** On host A, examine the ARP cache before and after pinging B:

```bash
ip neigh flush all
ip neigh
ping -c 1 192.0.2.11
ip neigh
```

Record what appeared and where it came from.

**4.** If the switch is managed, view its MAC address table:

```
show mac address-table
```

Record which addresses are on which ports, and how they got there.

---

### Part 2 — Hub versus switch

**5.** Start a capture on host C in promiscuous mode:

```bash
sudo tcpdump -i <iface> -n not arp
```

**6.** With everything on the **switch**, ping from A to B. Record what host C
sees.

**7.** Replace the switch with a **hub** (or move to the Packet Tracer model).
Repeat. Record what host C sees now.

**8.** With the hub in place, run simultaneous transfers:

```bash
iperf3 -s              # on host C
iperf3 -c <C> -t 30    # on A and on B at the same time
```

Record each host's throughput and the total.

**9.** Restore the switch and repeat step 8. Record again.

---

### Part 3 — Domains

**10.** Draw your hub topology and your switch topology. For each, count and mark:

- the number of **collision domains**
- the number of **broadcast domains**

**11.** Now add a second switch, uplinked to the first, with two more hosts.
Recount both figures.

**12.** Predict, before testing, what host C sees when host D (on the second
switch) broadcasts. Then send a broadcast and check:

```bash
ping -b 192.0.2.255       # may need -b and may be blocked by default
arping -b 192.0.2.99      # an ARP for an address nobody holds
```

---

### Part 4 — The multiplexing argument

**13.** Run the statistical multiplexing calculation for your lab:

```bash
python3 simnet.py statmux --users 30 --rate 100 --activity 0.05 --link 1000
```

**14.** Now measure something analogous. Have all hosts run `iperf3` against one
server *intermittently* — 5 seconds on, 60 seconds off, staggered. Watch the
switch's uplink utilisation, or the server's aggregate.

**15.** Compute what capacity would have been needed to give every host a
dedicated, guaranteed 100 Mb/s, and compare with what the shared link actually
carried.

---

## Expected observations

- **On the switch, host C sees nothing** of A-to-B traffic except broadcasts. On
  the **hub, host C sees everything**, in plain view, with no special privilege.
- **On the hub, A and B share the bandwidth** — roughly 45 Mb/s each on a 100 Mb/s
  hub, less once collisions bite. **On the switch they each get the full rate**,
  because the switch forwards both conversations simultaneously between different
  port pairs.
- **Collision domains:** one for the whole hub; one per port on the switch.
- **Broadcast domains:** one, in every configuration in this lab, including with
  two switches. Nothing here breaks a broadcast domain — that requires a router
  or a VLAN, and Lab 05 does it.
- **Statistical multiplexing:** the shared link carries the aggregate comfortably
  at a fraction of the dedicated-capacity figure.

---

## Break it

**A. Loop the switch.** Connect two ports of the same switch to each other with a
patch cable. **Do this last, briefly, and be ready to unplug it.** Watch the port
LEDs and any host's ping. Record how long it takes for the network to become
unusable.

If the switch has spanning tree enabled it will block one port and nothing
dramatic will happen — which is itself the observation, and is Lab 05's subject.
If it does not, you have reproduced Chapter 19 §19.1 and should unplug promptly.

**B. Duplicate an IP address.** Set host C to `192.0.2.11`, the same as host B.
Ping both from A, repeatedly, and record what happens over 60 seconds. Examine
A's ARP cache.

**C. Unplug the uplink between the two switches** during a ping and record the
failure and recovery behaviour.

---

## Debrief

**1.** On the hub, host C could read A and B's traffic with one command and no
special access. Explain why, in terms of what a hub does with an incoming signal.
Then explain what changed when you fitted the switch, and why network monitoring
became harder in the 1990s as a *side effect* of a performance improvement.

**2.** State your measured throughputs for the two simultaneous transfers on the
hub and on the switch. Explain the difference in terms of collision domains, and
state what the total across both hosts was in each case.

**3.** Count collision and broadcast domains for your two-switch topology. State
the rule that produced each count. Then state what device or configuration would
be required to increase the number of broadcast domains, and why.

**4.** In Break-It B you gave two hosts the same address. Describe exactly what
you observed, then explain it using the ARP cache contents. Why is this fault so
much harder to diagnose than a wrong address, and what does that suggest about
the discipline in Chapter 27 §27.4?

**5.** Your `statmux` calculation gave a multiplexing gain. State the number,
state the assumption it depends on most sensitively, and describe a realistic
situation in your institution where that assumption would fail — and what the
consequence would be.

**6.** You drew a physical and a logical topology in step 2. If they were the
same, explain under what change they would diverge. If they differed, explain
which properties of the network follow from which diagram. (Chapter 11 §11.3.)
