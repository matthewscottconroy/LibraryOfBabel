# 47.2 Zigbee, Thread and Matter

Three technologies at three layers, frequently discussed as competitors and mostly not.
Zigbee and Thread are networks; Matter is an application layer that runs over either.

Getting that right is most of understanding the smart-home landscape.

## The shared foundation: 802.15.4

Both Zigbee and Thread use IEEE 802.15.4 as their physical and MAC layer — the low-rate
wireless personal area network standard, and the counterpart to 802.11 and 802.3 at the
bottom of this chapter's stacks.

| | |
|---|---|
| Bands | **2.4 GHz** (worldwide), 868 MHz (EU), 915 MHz (Americas) |
| Rate | **250 kb/s** at 2.4 GHz |
| Range | **10–100 m** |
| **Frame size** | **127 bytes maximum** |
| Access | CSMA/CA |
| Power | **very low** — years on a battery |

The 127-byte frame is the constraint that shapes everything above it, and §47.2's
6LoWPAN discussion is entirely a consequence of it.

Its 2.4 GHz channels are 11–26, and — usefully — channels 15, 20, 25 and 26 sit between
Wi-Fi's 1, 6 and 11 (Chapter 43 §43.2). Choosing one of those is the most effective
thing you can do for Zigbee or Thread reliability in a building with Wi-Fi.

## Zigbee

The older of the two, from 2003, and the one with the largest installed base.

| | |
|---|---|
| Network layer | **Zigbee's own** — not IP |
| Topology | **mesh**, with coordinator, routers and end devices |
| Addressing | 16-bit short addresses |
| Application | **Zigbee Cluster Library** |
| Security | AES-128 |

**Its mesh is the selling point.** Mains-powered devices — bulbs, switches, plugs — act as
routers, so the network extends itself as more devices are added. Battery devices are end
devices and do not route, because routing requires listening continuously.

> A Zigbee network of twenty bulbs is twenty repeaters, which is why coverage in a house
> improves as you add devices.

Its problem is interoperability, and it has been Zigbee's persistent weakness.

Zigbee is not one protocol but a family of profiles — Home Automation, Light Link, Green
Power, and vendor extensions — and devices from different manufacturers frequently did not
work together despite both being "Zigbee". Zigbee 3.0 (2016) unified the profiles and
improved matters substantially, and the reputation persists because a decade of devices
predates it.

**And it is not IP.** A Zigbee network reaches the Internet only through a gateway that
translates, which means:

- A vendor-specific hub for each ecosystem, frequently
- A translation layer that can lose semantics
- **A cloud dependency** in many products — the hub talks to the vendor's cloud, and the
  lights stop working when the vendor's service does

## Thread

2014, from Nest/Google and others, and its central decision is the one Zigbee did not
make.

> **Thread devices are IPv6 hosts.** Every device has an IPv6 address and is directly
> addressable. **There is no protocol translation.**

| | Zigbee | **Thread** |
|---|---|---|
| PHY/MAC | 802.15.4 | **802.15.4** |
| Network | Zigbee's own | **IPv6 (6LoWPAN)** |
| Gateway | **translating hub** | **Border Router — routes, does not translate** |
| Single point of failure | **the coordinator** | **no — self-healing, multiple leaders** |
| Cloud dependency | often | **not architecturally** |
| Application | ZCL | **anything over IP** — usually Matter |

**Two consequences of being IP:**

**A Border Router routes rather than translates.** It connects the Thread mesh to the home
Wi-Fi or Ethernet network at the IP layer, so a phone can address a Thread device directly.
No semantic translation, so nothing is lost.

**No single coordinator.** Thread elects leaders dynamically and the network heals when a
node fails — where a Zigbee coordinator's loss takes the network down.

### 6LoWPAN — fitting IPv6 into 127 bytes

The mechanism that makes Thread possible, and it is a nice piece of engineering.

**The problem:** an IPv6 header is **40 bytes** (Chapter 28), the minimum IPv6 MTU is
**1,280 bytes** (Chapter 24 §24.3), and an 802.15.4 frame is 127 bytes with about 102 of
payload after the MAC header and security.

So a bare IPv6 packet does not fit, and the required MTU is ten times the frame size.

**6LoWPAN's answers:**

**Header compression.** Much of an IPv6 header is **derivable from context** — the version is
always 6, the source and destination prefixes are the link's, and the interface identifier can
be derived from the MAC address. A 40-byte header compresses to as few as 2–3 bytes.

**Fragmentation.** Packets larger than a frame are fragmented and reassembled at the link
layer.

**Mesh forwarding.** Below IP, so the mesh's hops are invisible to IP.

> 6LoWPAN is a compression and adaptation layer that lets a protocol designed for 1,500-byte
> frames run on 127-byte ones, and it is the reason a battery-powered sensor can be a
> first-class Internet host.

## Matter

Not a network. An application layer — and this is the distinction that matters.

Matter (2022, from the Connectivity Standards Alliance — formerly the Zigbee Alliance) runs
over:

- **Thread** — for low-power devices
- **Wi-Fi** — for mains-powered devices needing bandwidth
- **Ethernet**
- **BLE** — for commissioning only

```
   ┌─────────────────────────────────────────┐
   │              Matter                     │  ← the application layer
   ├──────────────┬──────────────┬───────────┤
   │    Thread    │    Wi-Fi     │  Ethernet │  ← the transport
   ├──────────────┼──────────────┴───────────┤
   │  802.15.4    │        IPv6              │
   └──────────────┴──────────────────────────┘
```

**What it standardises:**

**The data model** — what a light, a lock, a thermostat, a sensor *is*, in terms every
implementation agrees on.

**Commissioning** — how a device joins, using a QR code and BLE, the same way regardless of
vendor.

**Security** — certificate-based device attestation, so a device proves it is what it claims.

**Local control** — a Matter device is controllable on the local network without a cloud,
which is the architectural change that matters most.

> Matter's ambition is that a device works with Apple Home, Google Home, Amazon Alexa and
> Samsung SmartThings simultaneously, without a vendor hub, and without the Internet.

**Its progress has been mixed.** The specification is sound and implementations have been
uneven — devices certified and buggy, ecosystems interpreting the model differently, and
commissioning that fails in ways users cannot diagnose. It is better than what preceded it
and it is not yet the seamless outcome promised.

The reason it may succeed where previous attempts failed is that Apple, Google, Amazon
and Samsung are all committed to it simultaneously, which has not happened before. Chapter 28
§28.1's network-effect argument, with the largest players moving together.

## Choosing

| Need | Use |
|---|---|
| **Battery sensors, many, low data** | **Thread** (or Zigbee, on existing estates) |
| Mains-powered, higher bandwidth | **Wi-Fi** |
| Cameras, video | **Wi-Fi** — 802.15.4 cannot carry it |
| **Cross-ecosystem compatibility** | **Matter**, over whichever transport |
| Industrial, deterministic | **not these** — see §47.3 or wired fieldbus |
| Existing Zigbee estate | Zigbee; **Matter bridges exist** |

**And the practical guidance for a building:**

Choose an 802.15.4 channel between the Wi-Fi channels — 15, 20, 25 or 26.

Place Border Routers or coordinators centrally, not in a cupboard.

Add mains-powered devices to extend the mesh, and understand that battery devices do not
route.

And assess the cloud dependency of anything you buy — a product whose basic function
requires a vendor's service is a product that stops working when the vendor does, and this has
happened repeatedly.

## What breaks here

A Zigbee network that degrades as devices are added. Unlikely — mesh usually improves.
Check for channel overlap with Wi-Fi instead.

Zigbee devices from two vendors not working together. Profile incompatibility, largely
pre-3.0. Check the certification.

A smart-home system that stops when the Internet does. Cloud dependency. Matter's local
control is the fix, and only if the product implements it.

Battery devices with poor battery life on a mesh. They may have been made routers, or the
polling interval is aggressive.

**A Thread network with no Border Router.** The mesh works internally and **nothing reaches
it** — the Border Router is the IP connection, not an optional convenience.

**Matter commissioning failing.** Frequently BLE, frequently the ecosystem's implementation.
It is the least mature part.

> **Network+ note.** Objective 2.4 mentions Zigbee and IoT protocols. Over-learn: **Zigbee and
> Thread both use 802.15.4 and mesh**; Thread devices are IPv6 hosts and Zigbee's are not;
> Matter is an application layer over Thread, Wi-Fi or Ethernet; and 802.15.4 operates in
> 2.4 GHz alongside Wi-Fi and Bluetooth.
