# 17.2 The MAC Address Table

The whole algorithm fits in three sentences, and everything in enterprise switching
is elaboration on it.

> **Learn** the source address against the arrival port.
> **Forward** by the destination address, out that port only.
> **Flood** when the destination is unknown.

This section works it through, because tracing it by hand once is worth more than
reading it ten times.

## The table

Also called the **CAM table** (content-addressable memory, after the hardware that
implements it), the **forwarding database**, or the **bridge table**. Same thing.

Each entry:

| Field | Purpose |
|---|---|
| MAC address | the key |
| Port | where it was last seen |
| VLAN | which broadcast domain (Chapter 20) |
| Age / type | when it was last refreshed; dynamic or static |

```
Switch# show mac address-table
   Vlan   Mac Address       Type     Ports
   ----   -----------       ----     -----
     10   001b.4411.3ab7    DYNAMIC  Gi1/0/3
     10   0050.5601.2345    DYNAMIC  Gi1/0/7
     10   a4bb.6d02.19cc    DYNAMIC  Gi1/0/24
     20   0018.71ab.5599    DYNAMIC  Gi1/0/12
```

Note the VLAN column: the table is **per VLAN**, so the same address can appear
twice in different VLANs, and a lookup is keyed on the pair.

## The algorithm, traced

Four stations, one switch, table initially empty.

```
   A (aa:aa)──┐                  ┌──C (cc:cc)
              ├──[ SWITCH ]──────┤
   B (bb:bb)──┘                  └──D (dd:dd)
     port 1      2    3    4        port 3, 4
```

Ports: A→1, B→2, C→3, D→4.

**Frame 1: A sends to C.**

- **Learn:** source `aa:aa` seen on port 1. Table: `aa:aa → 1`.
- **Look up destination** `cc:cc`. **Not in the table.**
- **Flood:** send out ports 2, 3, 4 — everything except the arrival port.
- B and D receive a frame not addressed to them and discard it. C accepts it.

**Frame 2: C replies to A.**

- **Learn:** `cc:cc` on port 3. Table: `aa:aa → 1`, `cc:cc → 3`.
- **Look up** `aa:aa`. **Known, port 1.**
- **Forward to port 1 only.** B and D see nothing.

**Frame 3: A sends to C again.**

- **Learn:** refresh `aa:aa → 1` (resets its age).
- **Look up** `cc:cc`. **Known, port 3.**
- **Forward to port 3 only.**

The network has learned both addresses from **two frames**, and thereafter this
conversation is invisible to B and D.

**Frame 4: B sends a broadcast.**

- **Learn:** `bb:bb → 2`.
- Destination is `ff:ff:ff:ff:ff:ff`. **Always flooded**, by construction — there is
  no entry to look up and there never will be.
- Sent out ports 1, 3, 4. Every station processes it.

That last case is why **a switch does not break up a broadcast domain** (§17.3).

## Why learning works at all

The insight worth naming: **the switch learns from traffic it was going to have to
handle anyway**, at no cost.

Every frame carries a source address, and a frame arriving on port *n* is proof that
its sender is reachable through port *n*. The switch extracts this for free, from
frames it is forwarding regardless.

There is no protocol, no exchange, no configuration, and no cooperation required
from the stations. A switch dropped into a working network learns it within seconds.

**And there is no alternative.** Chapter 15 §15.2 established that MAC addresses are
flat — no rule summarises a set of them, so nothing can be computed. Learning by
observation is the only mechanism available, and the finite table it produces is a
direct consequence of flat addressing.

## Ageing

Entries expire, by default after **300 seconds** of silence. The timer is
configurable and 300 is near-universal.

**Why age at all?**

- **Devices move.** A laptop unplugged from port 12 and plugged into port 30 must be
  reachable at port 30, and the stale entry must go.
- **The table is finite.** Departed devices would otherwise consume entries forever.

**The consequence:** a device that is silent for five minutes is forgotten, and the
next frame addressed to it is **flooded** until it speaks again.

This is usually harmless. Occasionally it is the explanation for a puzzling traffic
pattern — a quiet server whose inbound traffic is periodically flooded to every port
in its VLAN, visible in a capture and mysterious until you know the mechanism.

The related phenomenon, **unicast flooding**, has a more troublesome cause:
**asymmetric routing**. If traffic to a device arrives on the switch but the device's
replies leave by another path, the switch never sees frames *from* it, never learns
its address, and floods every frame *to* it, permanently. Chapter 65's diagnosis is
that persistent unicast flooding to a live host means the switch is not seeing that
host's transmissions.

## Static entries

An address can be configured manually rather than learned. Static entries do not age
and take precedence.

Uses: pinning a critical server's address to a port; part of port-security
configuration; and occasionally forcing traffic for a device that is silent by
design.

Mostly a niche. Configuring statically what the switch learns automatically is
maintenance work with no benefit, and stale static entries cause faults that look
inexplicable — because the switch is forwarding to a port the device left months ago,
and no ageing will correct it.

## Table size, and exhaustion

The table lives in fast, expensive hardware, and it is **finite**:

| Class | Typical capacity |
|---|---|
| Small access switch | 8,000 |
| Enterprise access | 16,000–32,000 |
| Distribution / core | 64,000–128,000 |
| Data centre | 256,000+ |

**When the table fills, the switch cannot learn.** It does not stop working — it
**floods** everything it cannot look up, which means it behaves as a hub.

Two ways to reach that state:

**Legitimately**, on a very large flat network. A single broadcast domain with more
devices than the switch's table capacity — including every virtual machine's
addresses, which multiply quickly — degrades all of them.

**Deliberately.** Chapter 62 §62.1's **MAC flooding**: an attacker generates frames
with hundreds of thousands of fabricated source addresses, the table fills within
seconds, and the switch floods everything — restoring the eavesdropping capability
that switching removed.

Notice the shape of that attack: **it does not break the switch, it degrades it to
an earlier, weaker design.** A surprising number of network attacks have this form,
and recognising it is a useful instinct.

The mitigation is **port security**: limit the number of addresses learnable on a
port. Two or three on an access port covers a workstation, a daisy-chained IP
telephone, and one virtual machine, while making flooding impossible.

```
interface GigabitEthernet1/0/12
 switchport port-security
 switchport port-security maximum 3
 switchport port-security violation restrict
```

The violation modes matter: `shutdown` err-disables the port (safe, and a support
call), `restrict` drops the offending frames and increments a counter (usually the
right choice), `protect` drops silently (no visibility, rarely right).

## Reading a real table

Practical uses, and this is one of the highest-value diagnostic commands available:

**Find where a device is.** Given a MAC address, the table gives the port — and
following it across switches locates the device physically. This is how you find the
machine generating a broadcast storm.

**Confirm a device is present.** An address in the table means the device has
transmitted recently.

**Spot a duplicate address.** The same address appearing on two ports, or flapping
between them, means two devices share it — a cloned virtual machine, a spoof, or a
loop.

**Spot a loop.** During a broadcast storm the table thrashes, with addresses moving
between ports rapidly. Most platforms log this as a MAC flap or MAC move, and it is
the clearest single indicator of a Layer 2 loop (Chapter 19 §19.1).

**Count devices on a port.** More than one or two on an access port means a hub, an
unmanaged switch, or a virtualisation host — which may or may not be expected.

## What breaks here

**Persistent unicast flooding to a live host.** Asymmetric routing, so the switch
never learns that host's address. Check whether return traffic takes a different
path.

**MAC flapping between ports.** A loop, a duplicate address, or a device that is
genuinely moving. The logs name the two ports, which usually identifies the loop.

**A full table.** Flooding everywhere, degraded performance, and eavesdropping
becomes possible. Either the broadcast domain is too large or someone is attacking
it.

**A stale static entry.** Traffic forwarded to a port the device left, with no ageing
to correct it. Invisible unless you look for it.

**Ageing shorter than a device's silence.** Periodic flooding of that device's
inbound traffic — usually harmless, occasionally the explanation for something odd.

> **Network+ note.** Objective 1.2 expects switch operation, and objective 5.1's
> troubleshooting expects you to use the MAC address table to locate a device. The
> three-sentence algorithm is worth memorising verbatim, and `show mac
> address-table` is worth using until it is automatic.
