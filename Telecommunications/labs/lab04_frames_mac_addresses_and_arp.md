# Lab 04 — Frames, MAC Addresses, and ARP

**Corresponds to:** Chapters 15, 16, 17, 18
**Week:** 4
**Time:** 90 minutes

---

## Objectives

- Decode an Ethernet frame field by field from a hex dump, without tooling.
- Look up an OUI and identify a device's manufacturer from a capture.
- Trace an ARP exchange and predict, correctly, which address a host will ARP for.
- Watch a switch learn, age out, and flood.
- Recognise the diagnostic signature of an ARP failure.
- Perform and detect an ARP spoof in a controlled environment.

---

## You will need

- Three hosts on one segment, from Lab 03.
- Wireshark and `tcpdump`.
- A router or a host acting as a gateway, so that off-subnet traffic exists.
- Optionally `arping` and, for Part 5, `arpspoof` or `ettercap` — **on an isolated
  lab network only**.

**Fallback:** everything except Part 4 runs on two hosts and a virtual bridge.

---

## Procedure

### Part 1 — The frame, by hand

**1.** Capture five frames with full hex, without Wireshark's help:

```bash
sudo tcpdump -i <iface> -n -x -c 5 icmp
```

**2.** Take one frame and identify, by byte offset and without looking at
Wireshark's decode:

| Bytes | Field |
|---|---|
| 0–5 | Destination MAC |
| 6–11 | Source MAC |
| 12–13 | EtherType |
| 14 | IP version and IHL |
| 22 | TTL |
| 23 | Protocol |
| 26–29 | Source IP |
| 30–33 | Destination IP |

**3.** Convert the source IP bytes from hex to dotted decimal by hand and confirm.

**4.** Take the first three bytes of the source MAC and look up the OUI (the IEEE
registry is public and searchable). Record the manufacturer. Does it match the
machine you are sitting at? If the machine is a virtual machine, the OUI will
identify the hypervisor vendor, which is itself a useful thing to recognise.

**5.** Examine the destination MAC of a broadcast frame. Confirm it is
`ff:ff:ff:ff:ff:ff`. Then find a multicast frame and check bit 0 of the first
octet — it is 1 for multicast and 0 for unicast.

---

### Part 2 — ARP, watched

**6.** Clear the ARP cache and start a capture filtered to ARP:

```bash
sudo ip neigh flush all
sudo tcpdump -i <iface> -n arp
```

**7.** In another terminal, ping a host **on your own subnet**. Record the ARP
request and reply: who asked, what they asked for, who answered, and what the
answer was.

**8.** Clear the cache again. Now ping a host **off your subnet** — something on
the Internet.

**9.** Record which address was ARPed for this time. **Before looking, predict
it.** This is the case Chapter 18 §18.2 says students most often get wrong.

**10.** Examine the cache and the states:

```bash
ip neigh
```

Note the `REACHABLE`, `STALE` and `DELAY` states and what triggers each.

---

### Part 3 — Watching a switch learn

**11.** On a managed switch, clear the MAC table and display it:

```
clear mac address-table dynamic
show mac address-table
```

**12.** Ping between two hosts. Redisplay the table. Record what appeared, on
which ports, and reason about *which frame* taught the switch each entry.

**13.** Start a capture on a third, uninvolved host. Now clear the switch's table
again and immediately ping A to B. **Watch for the flooded frame** on host C
before the switch has learned.

**14.** Leave a host silent for longer than the ageing timer (typically 300 s).
Check the table. Then send one frame to it and check what host C sees.

---

### Part 4 — ARP failure signatures

**15.** Ping an address on your subnet that **no host holds**:

```bash
ping -c 3 192.0.2.199
ip neigh | grep 192.0.2.199
```

Record the exact error message and the cache entry state.

**16.** Compare with pinging an address **outside** your subnet with no route:

```bash
ping -c 3 203.0.113.99
```

Record the different error.

**17.** Now break ARP deliberately: add a **static, wrong** ARP entry.

```bash
sudo ip neigh replace 192.0.2.11 lladdr 00:11:22:33:44:55 dev <iface>
ping -c 3 192.0.2.11
```

Record what happens, and where in the stack the failure occurs. Remove it
afterwards with `sudo ip neigh del 192.0.2.11 dev <iface>`.

---

### Part 5 — Spoofing (isolated network only)

**18.** With instructor supervision on an isolated segment, have host C claim to
be the gateway:

```bash
sudo arpspoof -i <iface> -t <victim-ip> <gateway-ip>
```

**19.** On the victim, watch the ARP cache change:

```bash
watch -n 1 ip neigh
```

**20.** On host C, enable forwarding so the victim keeps working, and capture:

```bash
sudo sysctl -w net.ipv4.ip_forward=1
sudo tcpdump -i <iface> -n host <victim-ip> and not arp
```

**21.** Stop the spoof and watch the cache recover.

---

## Expected observations

- **The EtherType is `0800`** for IPv4 and `0806` for ARP. The ARP frames carry
  `0806` and are *not* inside IP.
- **For an on-subnet destination, the host ARPs for the destination.** For an
  off-subnet destination, it **ARPs for the gateway**, and the IP destination in
  the resulting frame is still the far host. If you predicted otherwise in step 9,
  reread Chapter 25 §25.3.
- **The switch learns from the source address**, so the first frame in either
  direction populates one entry. Host C sees the *first* frame flooded and nothing
  afterwards.
- **A failed ARP produces a locally generated "Destination Host Unreachable"** and
  an `INCOMPLETE` or `FAILED` cache entry. A missing route produces "Network is
  unreachable" instead — a different message from a different layer.
- **The wrong static ARP entry produces silent failure.** Frames go out addressed
  to a MAC that nobody holds; nothing replies; no error is generated by ARP,
  because ARP has no failure notification.
- **The spoof takes effect in seconds** and the victim notices nothing.

---

## Break it

Already integrated — steps 15 through 21 are the break-it section, because ARP is
best understood through its failures.

One more, if time allows: **set two hosts to the same IP** (as in Lab 03) and
watch the ARP cache oscillate between two MAC addresses. Compare that signature
with the static-wrong-entry signature from step 17.

---

## Debrief

**1.** Give the byte offsets of the source MAC, EtherType and destination IP in an
Ethernet II frame carrying IPv4. State how you would know, from bytes 12–13 alone,
whether the payload was IPv4, IPv6 or ARP.

**2.** In step 9, which address did the host ARP for, and why? State the exact
computation the host performed to decide. Then predict what would change if the
subnet mask were widened to include the "off-subnet" destination.

**3.** Host C saw exactly one flooded frame and then nothing. Explain the
mechanism, and state what would make host C see flooded frames *continuously*.
(Two answers: one benign, one an attack.)

**4.** You produced three distinct failure signatures: failed ARP, missing route,
and wrong static ARP entry. Tabulate them — symptom, error message, cache state —
and state which single command distinguishes them fastest.

**5.** ARP has no authentication. Explain why that was a defensible decision in
1982 and is not now, and name the two switch features that mitigate it. Then
explain why those features are *add-ons* rather than protocol changes, and what
that tells you about how security arrives in this field.

**6.** During the spoof, the victim had full connectivity and noticed nothing.
State two observations available to the victim that would have revealed it, and
one that would be available to the network administrator but not the victim.
