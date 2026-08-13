# 18.2 The ARP Exchange

RFC 826, *An Ethernet Address Resolution Protocol*, was written by David Plummer at
MIT and published in November 1982. It is **four pages long**, it has never been
revised, and it is one of the most consequential specifications in the Internet
suite.

Reading it is a genuinely pleasant twenty minutes, and it is worth doing to see how
much can be specified in four pages when the problem is stated precisely.

## The exchange

Two messages. That is the entire protocol.

**Request** — broadcast to everyone on the link:

> *Who has 192.168.10.1? Tell 192.168.10.70.*

**Reply** — unicast, back to the asker:

> *192.168.10.1 is at rr:rr:rr:rr:rr:rr.*

The request goes to `ff:ff:ff:ff:ff:ff`, so every station on the segment receives it.
Every station examines the target address; the one that recognises its own IP address
replies; **everyone else discards it silently**.

Note the asymmetry: the **request is broadcast** because the asker does not know
whom to ask, and the **reply is unicast** because the replier does know — it learned
the asker's MAC address from the request's source field. Broadcasting the reply would
interrupt every station for no reason.

## The packet format

ARP is deliberately generic. It was designed to resolve *any* protocol address to
*any* hardware address, and the header says so:

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-------------------------------+-------------------------------+
|     Hardware type (HTYPE)     |     Protocol type (PTYPE)     |
+---------------+---------------+-------------------------------+
|  HLEN (6)     |  PLEN (4)     |         Operation (OPER)      |
+---------------+---------------+-------------------------------+
|              Sender hardware address (6 bytes)                |
+---------------------------------------------------------------+
|      … continued              |  Sender protocol address      |
+-------------------------------+-------------------------------+
|      … continued              |  Target hardware address      |
+-------------------------------+-------------------------------+
|              … continued (6 bytes)                            |
+---------------------------------------------------------------+
|              Target protocol address (4 bytes)                |
+---------------------------------------------------------------+
```

| Field | Typical value | Meaning |
|---|---|---|
| HTYPE | `0x0001` | Ethernet |
| PTYPE | `0x0800` | IPv4 — **the same value as the EtherType** |
| HLEN | `6` | MAC address length in bytes |
| PLEN | `4` | IP address length in bytes |
| OPER | `1` = request, `2` = reply | |
| Sender HW / protocol | the asker's | |
| Target HW / protocol | what is being asked about | |

The genericity was used: ARP resolved addresses for Chaosnet, DECnet and others. In
practice today it means IPv4 over Ethernet and nothing else, and the HTYPE/PTYPE
fields are constants.

**In a request, the target hardware address is zero** — that is the field being
asked about. In the reply it is filled in.

## A worked exchange

Host A (`192.168.10.70`, `aa:aa:aa:aa:aa:aa`) needs the router at `192.168.10.1`.

**Frame 1 — the request**

| Layer | Field | Value |
|---|---|---|
| Ethernet | Destination | `ff:ff:ff:ff:ff:ff` |
| Ethernet | Source | `aa:aa:aa:aa:aa:aa` |
| Ethernet | EtherType | `0x0806` |
| ARP | Operation | 1 (request) |
| ARP | Sender HW | `aa:aa:aa:aa:aa:aa` |
| ARP | Sender IP | `192.168.10.70` |
| ARP | **Target HW** | `00:00:00:00:00:00` ← the question |
| ARP | Target IP | `192.168.10.1` |

**Frame 2 — the reply**

| Layer | Field | Value |
|---|---|---|
| Ethernet | Destination | `aa:aa:aa:aa:aa:aa` ← **unicast** |
| Ethernet | Source | `rr:rr:rr:rr:rr:rr` |
| Ethernet | EtherType | `0x0806` |
| ARP | Operation | 2 (reply) |
| ARP | Sender HW | `rr:rr:rr:rr:rr:rr` ← **the answer** |
| ARP | Sender IP | `192.168.10.1` |
| ARP | Target HW | `aa:aa:aa:aa:aa:aa` |
| ARP | Target IP | `192.168.10.70` |

Host A caches the result and sends its actual data frame. **Two frames of overhead,
once, and then nothing for several minutes.**

## Everyone learns

A detail with large consequences.

RFC 826 specifies that a host receiving **any** ARP packet — request or reply —
should update its cache with the sender's mapping **if it already has an entry for
that address**. And it must add an entry if it is the target.

So a broadcast request teaches every station on the link the *asker's* mapping, not
merely the replier's. In a network where hosts talk to a common gateway, everyone
learns everyone's mapping quickly, from traffic they were receiving anyway.

**This is also the vulnerability.** A station that accepts an unsolicited mapping
accepts it from whoever sent it, and §18.3 develops what that permits.

## The critical case: off-subnet

The case students most often get wrong, and it is worth being explicit.

**Host A (`192.168.10.70/24`) sends to `192.168.10.99`:**

Same subnet. A ARPs for **`192.168.10.99`**, receives that host's MAC, and sends the
frame directly.

**Host A sends to `8.8.8.8`:**

Different subnet. A does **not** ARP for `8.8.8.8` — it will never receive a reply,
because nothing on the local link holds that address and ARP requests are not
forwarded by routers.

Instead A consults its routing table, finds the default gateway `192.168.10.1`, and
**ARPs for the gateway**. The frame it then sends has:

- **Destination MAC:** the gateway's
- **Destination IP:** `8.8.8.8`

**A host never ARPs for an address outside its own subnet.** If you see one in a
capture, the sending host has a wrong subnet mask — which is exactly Chapter 25
§25.3's selective-connectivity symptom, visible from a different angle.

This makes an ARP capture a diagnostic for mask errors: **what a host ARPs for tells
you what it believes its subnet is.**

## The chicken-and-egg question

If a host must ARP before sending, and the ARP request is itself a frame, how does
the first one get sent?

**Because it is broadcast.** A broadcast needs no resolution — the destination
`ff:ff:ff:ff:ff:ff` is known in advance and requires no lookup. So the bootstrap
works: broadcast the question, receive a unicast answer, and everything afterward is
unicast.

The same trick appears in DHCP (Chapter 40 §40.2), where a host with no address at
all broadcasts from `0.0.0.0`. **Broadcast is the bootstrap mechanism for every
protocol that must work before configuration exists.**

## The cost

Small, and worth quantifying because it is the argument for bounding broadcast
domains.

Each resolution costs **two frames**, once per cache lifetime (typically 60–300
seconds). For a host talking to one gateway, that is two frames every few minutes —
negligible.

The problem is scale. In a broadcast domain of *n* hosts, **every ARP request is
processed by all *n***, and in the worst case each host may need to resolve each
other host:

$$\text{worst-case requests} \propto n^2$$

At 200 hosts this is unnoticeable. At 2,000 it is measurable. At 20,000 it is a
significant fraction of every host's CPU spent discarding requests for addresses it
does not hold.

This is one of the two arguments for keeping broadcast domains small (Chapter 17
§17.3), and it is why Chapter 20's VLANs are a capacity mechanism as well as a
security one.

## What breaks here

**A host ARPing for an off-subnet address.** Wrong subnet mask. The capture shows it
immediately, and it is one of the fastest diagnoses available.

**No reply to a request.** The target is absent, powered off, on a different VLAN,
or blocked. The requester's cache entry stays `INCOMPLETE` and §18.3 covers the
symptom.

**Two replies with different MAC addresses.** Duplicate IP address, or an ARP spoof.
The cache flaps and connectivity becomes intermittent and unpredictable.

**A flood of ARP requests for sequential addresses.** Something is scanning the
subnet — a discovery tool, a monitoring system, or malware enumerating targets.
Distinctive in a capture and worth recognising.

> **Network+ note.** Objective 1.4 expects ARP's operation, and objective 5.5
> expects the `arp` command. The examinable content is the request/reply exchange;
> the operationally valuable content is **what a host ARPs for reveals what it
> believes its subnet mask is**, which turns a capture into a mask diagnosis.
