# Lab 05 — VLANs, Trunks, and Spanning Tree

**Corresponds to:** Chapters 19, 20
**Week:** 5
**Time:** 120 minutes

---

## Objectives

- Configure access ports and a trunk, and verify VLAN separation empirically.
- Decode an 802.1Q tag in a capture.
- Determine the spanning tree root, port roles and blocked ports, and predict
  them before checking.
- Set the root bridge deliberately and observe the topology change.
- Diagnose the four classic VLAN faults from their distinct symptoms.
- Cause and recover from a broadcast storm safely.

---

## You will need

- Two managed switches supporting 802.1Q and a spanning tree protocol.
- Four hosts, two per switch.
- A router or Layer 3 switch for the inter-VLAN routing section.
- Wireshark, with the capture point on a **trunk** link — a SPAN port mirroring
  the trunk, or a tap.

**Fallback:** Packet Tracer or GNS3 models all of this faithfully, including
802.1Q tagging visible in its packet inspector, and is a legitimate substitute
for the whole lab. What it cannot reproduce is the physical experience of a
storm, so if you have hardware, do Part 5 on it.

---

## Procedure

### Part 1 — Two VLANs

**1.** On switch 1, create VLANs 10 and 20:

```
vlan 10
 name STAFF
vlan 20
 name GUEST
```

**2.** Put host A in VLAN 10 and host B in VLAN 20:

```
interface <port-A>
 switchport mode access
 switchport access vlan 10
interface <port-B>
 switchport mode access
 switchport access vlan 20
```

**3.** Give A and B addresses **in the same IP subnet** — `192.0.2.10/24` and
`192.0.2.11/24`. This is deliberate and the point of step 4.

**4.** Ping A to B. Record the result.

**5.** Explain the result before continuing. Two hosts with addresses in the same
subnet, on the same physical switch, cannot reach each other. Say why.

---

### Part 2 — The trunk and the tag

**6.** Connect switch 1 to switch 2 and configure the link as a trunk at both
ends:

```
interface <uplink>
 switchport mode trunk
 switchport trunk allowed vlan 10,20
```

**7.** Put host C on switch 2 in VLAN 10, with `192.0.2.12/24`.

**8.** Ping A to C. Record the result. A and C are on **different switches** and
the **same VLAN**.

**9.** Capture on the trunk. Ping A to C again and find the frames.

**10.** In the capture, locate the 802.1Q tag. Record: the TPID (should be
`0x8100`), the PCP bits, and the VLAN ID. Confirm the VLAN ID matches.

**11.** Record the frame size of a tagged full-size frame. Compare with 1,518.

---

### Part 3 — Inter-VLAN routing

**12.** Give VLAN 10 and VLAN 20 **different subnets** — readdress B to
`198.51.100.11/24` and configure a gateway for each VLAN, either as
router-on-a-stick subinterfaces or as SVIs on a Layer 3 switch.

**13.** Set each host's default gateway accordingly. Ping A to B again.

**14.** Trace the path:

```bash
traceroute -n 198.51.100.11
```

Record the hop count and explain it.

---

### Part 4 — Spanning tree

**15.** Add a **second** link between the two switches. Before enabling it,
predict what will happen.

**16.** Enable it. Examine spanning tree state on both switches:

```
show spanning-tree
```

**17.** Record: which switch is root, why (read the bridge IDs), which ports are
root/designated/blocking, and the path cost.

**18.** **Predict which switch would be root** if you had not looked. Chapter 19
§19.2 says it is the lowest MAC address, which usually means the oldest device.
Was it?

**19.** Now set the root deliberately:

```
spanning-tree vlan 10,20 root primary
```

on the switch you *want* to be root. Re-examine. Record which ports changed role.

**20.** Unplug the active inter-switch link during a continuous ping and record
how long connectivity is lost before the blocked port takes over. Repeat with
RSTP if the switches support both, and compare.

---

### Part 5 — The storm (hardware only, briefly)

**21.** Disable spanning tree on both switches. **Have a plan to unplug.**

**22.** With both inter-switch links connected, send one broadcast from any host.

**23.** Watch the port LEDs. Attempt to ping anything. Attempt to open the switch
console.

**24.** Unplug one inter-switch link. Record how long recovery takes.

**25.** Re-enable spanning tree.

---

## Expected observations

- **Step 4: A cannot reach B**, despite being in the same IP subnet on the same
  switch. A VLAN is a broadcast domain; the ARP request never reaches B.
- **Step 8: A can reach C**, on a different switch, because the trunk carries
  VLAN 10 between them. Physical separation is irrelevant; VLAN membership is
  everything.
- **Step 10: the tag is present**, TPID `0x8100`, with the VLAN ID in the low
  12 bits of the following two bytes.
- **Step 11: a tagged maximum frame is 1,522 bytes**, four more than 1,518.
- **Step 14: one hop.** Traffic between VLANs is routed, even when both VLANs
  live on the same physical switch.
- **Step 17: the root is whichever switch has the lower bridge ID**, and with
  default priorities that is the lower MAC address.
- **Step 20: classic STP takes 30–50 seconds** to converge. RSTP takes a few.
- **Step 23: the network becomes completely unusable within about a second**, the
  switch CPUs saturate, and the console may be unresponsive.

---

## Break it

The four classic VLAN faults. Introduce each, observe, diagnose, and only then
read the explanation.

**A. Wrong access VLAN.** Move host C to VLAN 20 without telling your partner.
Symptom: C has link, gets no DHCP or cannot reach its subnet peers, and looks
perfectly configured from the host side.

**B. Native VLAN mismatch.** Set the native VLAN to 1 at one end of the trunk and
99 at the other. Symptom: **no error**, and connectivity that should not exist
between two segments. This is the one that is worst because nothing announces it.

**C. VLAN missing from the trunk's allowed list.** Remove VLAN 20 from
`switchport trunk allowed vlan`. Symptom: VLAN 20 works within each switch and
not between them — a confusing partial failure.

**D. Missing inter-VLAN routing.** Remove the SVI or subinterface for VLAN 20.
Symptom: hosts reach their own VLAN and nothing else, including hosts on the same
switch.

For each: record the symptom *as a user would report it*, then the first three
commands you would run.

---

## Debrief

**1.** In step 4, two hosts in the same IP subnet on the same switch could not
communicate. Explain the mechanism precisely, at the level of what happened to
the ARP request. Then state the general principle in one sentence.

**2.** A tagged frame is 1,522 bytes. Explain what happens at a boundary where
the next device does not expect tagged frames, name the counter that increments,
and say why this fault appears specifically at administrative boundaries.

**3.** Which switch became root with default priorities, and why? Explain the
operational consequence of leaving it to default in a real building — what
device is likely to win, and why that is undesirable.

**4.** You measured convergence time after unplugging the active link. State the
figure. Explain, mechanistically, why RSTP is faster — what assumption did the
classic timers make that RSTP discards?

**5.** In Break-It B, the native VLAN mismatch produced no error. Explain why this
is worse than an outage, and describe the double-tagging attack that the same
provision enables. Give the three configuration changes that prevent both.

**6.** Tabulate your four VLAN faults: the symptom as a user reports it, the
symptom as an engineer observes it, and the single command that confirms it. This
table is directly reusable in the week 13 gauntlet, so make it good.
