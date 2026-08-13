# Chapter 47 — Further Reading

## Specifications

**Bluetooth Core Specification** (bluetooth.com/specifications).
**Freely available and enormous** — well over 3,000 pages. **Do not read it linearly.** The
architectural overview in Volume 1 and the GAP/GATT material in Volume 3 are the parts worth
reading; the rest is a reference.

**Bluetooth Assigned Numbers.**
**The list of standard service and characteristic UUIDs.** Short, and it is what you need when
inspecting a real device with a scanner.

**IEEE 802.15.4.**
The physical and MAC layer under both Zigbee and Thread. **The frame format section is the one
that explains §47.2's whole 6LoWPAN discussion.**

**RFC 4944, RFC 6282, RFC 6775 — 6LoWPAN.**
Transmission of IPv6 over 802.15.4, header compression, and neighbour discovery optimisation.
**RFC 6282 on header compression is the elegant one** — read it as a case study in how much of
a protocol header is actually derivable from context.

**RFC 6550 — RPL.**
The routing protocol for low-power lossy networks. **Worth reading against Chapter 31's
protocols** to see what changes when links are unreliable and nodes are asleep.

**Thread Specification** (threadgroup.org).
Requires free registration. **The commissioning and border router sections are the
distinctive parts.**

**Matter Specification** (csa-iot.org, and the SDK on GitHub as `project-chip/connectedhomeip`).
**Open, and the data model is the part to read** — it is a careful attempt to define what a
device *is*, which is the problem Zigbee never solved.

**LoRaWAN Specification and Regional Parameters** (lora-alliance.org).
**Read the Regional Parameters document for your own region** — the duty cycle, dwell time and
payload limits differ substantially, and F1 depends on it.

**3GPP TS 36.331 and the NB-IoT / LTE-M feature descriptions.**
Dense. **The GSMA's NB-IoT and LTE-M deployment guides are the readable summary.**

**ISO/IEC 14443** (proximity cards), **ISO/IEC 15693** (vicinity cards), **ISO/IEC 18000-6C /
EPC Gen2** (UHF RFID), **NFC Forum specifications.**
**EPC Gen2's anti-collision section is the practical one**, and it is the slotted-ALOHA
argument of Chapter 16 in production.

## Books

**Finkenzeller, K. — *RFID Handbook*.**
**The standard reference on RFID and NFC physics**, and the best treatment anywhere of
near-field coupling, load modulation and backscatter. **Mathematical, and worth the effort if
§47.4 interested you.**

**Townsend, K., Cufí, C., Akiba & Davidson, R. — *Getting Started with Bluetooth Low
Energy*.**
**The clearest short introduction to BLE and GATT.** Practical, and it will get you building
in an afternoon.

**Heydon, R. — *Bluetooth Low Energy: The Developer's Handbook*.**
Deeper, by one of the specification's authors.

**Sinha, R. S. et al. — "A survey on LPWA technology: LoRa and NB-IoT."** *ICT Express*, 2017.
**The clearest side-by-side comparison**, and it holds up.

**Shelby, Z. & Bormann, C. — *6LoWPAN: The Wireless Embedded Internet*.**
By two of the people who wrote the RFCs. **The design rationale that the RFCs omit.**

## Papers and history

**Stockman, H. (1948). "Communication by Means of Reflected Power." *Proceedings of the
IRE*.**
**The founding paper of backscatter**, and a short one. **Read the closing paragraph** and
note how accurately it estimates the work remaining.

**Haartsen, J. (2000). "The Bluetooth radio system." *IEEE Personal Communications*.**
The architect's own account, written while the system was new. **Unusually candid about the
cost constraints that drove the design.**

**Semtech AN1200.22 — "LoRa Modulation Basics."**
**The accessible explanation of chirp spread spectrum**, with the spreading-factor arithmetic
of §47.3 worked through.

**Francillon, A., Danev, B. & Capkun, S. (2011). "Relay Attacks on Passive Keyless Entry and
Start Systems in Modern Cars." NDSS.**
**The paper that demonstrated the attack in §47.4 against production vehicles.** Clear,
alarming, and the reason UWB is now in cars.

**Brands, S. & Chaum, D. (1994). "Distance-Bounding Protocols." EUROCRYPT.**
**The defence, proposed seventeen years before the attack above was demonstrated and roughly
thirty before it was deployed.** F4 is built on this.

**Hancke, G. & Kuhn, M. (2005). "An RFID Distance Bounding Protocol." SecureComm.**
The practical version, and the one closest to what modern systems implement.

## Tools and practical work

**nRF Connect** (Nordic Semiconductor), **LightBlue**, or **Bluetility** — **BLE scanners** for
phone or desktop. **F2 needs one of these**, and ten minutes with one teaches more about GATT
than any amount of reading.

**Wireshark with a BLE or 802.15.4 sniffer** — nRF52840 dongles and CC2531 sticks are
inexpensive and both are supported. **Watching a Thread device join is instructive.**

**The Things Network** (thethingsnetwork.org).
**A free, community-operated global LoRaWAN network.** You can put a device on it today
without deploying a gateway, and its documentation is the best practical LoRaWAN teaching
material available.

**`hcitool` / `bluetoothctl` / `btmon`** on Linux — the low-level Bluetooth tooling. **`btmon`
shows the HCI traffic**, which is where pairing problems become visible.

**Proxmark3** — the RFID/NFC research tool. **Powerful, entirely legal to own, and trivially
capable of things that are not legal to do**; use it only on cards you own.

**A software-defined radio** (Chapter 43's further reading) will receive LoRa and 433 MHz
traffic with `rtl_433`, and **decoding your own utility meters or weather station is a
satisfying afternoon.**

## Where to look next

**Chapter 57** gives the threat-modelling framework these devices need and rarely receive, and
**Chapter 62** attacks the stack they sit on; **Chapter 56** is the right place to think about
what happens to a deployed estate when the network it depends on is switched off — **the 2G
shutdown of Chapter 46 §46.2, generalised into an availability problem**; and **Chapter 71**
takes up where the technologies of this chapter are going next.
