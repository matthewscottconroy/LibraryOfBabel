# Chapter 44 — Important Concepts

**The lettering is not chronological** *(§44.1)* — It follows the order projects were
*started*. **`a` and `b` were ratified the same year**, which is why nobody can remember the
sequence and why the Wi-Fi Alliance's numbering exists.

**802.11b beat 802.11a** *(§44.1)* — **11 Mb/s beat 54 Mb/s**, because 2.4 GHz radios were
cheap, the band propagates better, and Apple shipped it at a consumer price in 1999. **The
technically superior standard lost to the cheaper one that shipped in volume** — the same
story as Chapter 22 §22.1 and Chapter 23 §23.1.

**802.11n's real gain was aggregation as much as MIMO** *(§44.1)* — MIMO gave 4× and **frame
aggregation amortised the per-frame overhead**, without which the higher rates would have
delivered almost nothing.

**802.11ax changed the goal** *(§44.1)* — Every previous amendment raised the peak rate; **ax
was designed for dense environments where the problem is contention.** **OFDMA**, uplink
MU-MIMO, **BSS colouring** (so you can tell your network's transmissions from a neighbour's)
and **TWT** (scheduled wake-ups, an order of magnitude of battery life for sensors).

**Wi-Fi 7's MLO is the first structural change to association** *(§44.1)* — A client attaches
over **several bands at once**, aggregating for throughput, or using one as backup, or
duplicating for latency.

**The rate on the box is fiction** *(§44.1)* — 9.6 Gb/s assumes **8 spatial streams that no
client has**, a 160 MHz channel that is often unavailable, and 1024-QAM that requires being
metres from the access point. **Design against measured throughput, not datasheets.**

**MCS is what a client actually negotiates** *(§44.1)* — And it is a direct function of SNR.
**MCS 11 needs ~35 dB**; most clients most of the time run at MCS 4–7. **Rate adaptation is
doing something real**, and forcing rates up produces errors that make throughput worse.

**Wireless cannot detect collisions** *(§44.2)* — **A radio cannot listen while
transmitting** (a 90 dB difference deafens the receiver); **a collision happens at the
receiver, not the transmitter**; and hidden nodes make it worse. **So it must avoid them**,
which is necessarily more conservative and more expensive.

**Every unicast frame is acknowledged** *(§44.2)* — There is no other way to know it arrived.

**Priority by patience** *(§44.2)* — **SIFS 16 µs for ACKs, DIFS 34 µs for data**, so the ACK
always wins. **No negotiation required**, and 802.11e's QoS categories extend it with
different AIFS values.

**The overhead dominates at high rates** *(§44.2)* — **58% efficiency at 54 Mb/s, ~11% at
600 Mb/s, ~5% at 1.3 Gb/s**, because the data time shrinks and the fixed costs do not.
**Over 90% of the airtime is protocol overhead at gigabit rates**, which is why frame
aggregation was essential. **And it is why real throughput is about half the nominal rate.**

**The hidden node problem** *(§44.2)* — Two stations in range of the access point and out of
range of each other. **Carrier sense says the medium is free and they collide at the
receiver**, and **neither can detect it** — they learn only from the missing acknowledgement
and will do it again. **Not an edge case**: it arises whenever coverage exceeds the stations'
mutual range, which is normal.

**RTS/CTS and the NAV** *(§44.2)* — **The access point's CTS is heard by everyone**,
including stations that cannot hear each other. **The NAV is virtual carrier sense: a station
defers because it was told to, not because it heard anything.** **The cost is two extra frames
per transmission**, so it is threshold-controlled and **off by default** — a targeted remedy,
not a general improvement.

**The exposed node problem** *(§44.2)* — A station defers to a transmission that would not
have interfered, because **carrier sense is about the transmitter's neighbourhood and
interference is about the receiver's.** **BSS colouring** addresses part of it.

**Protection mechanisms** *(§44.2)* — An 802.11b device cannot decode OFDM, so every OFDM
transmission must be announced at a legacy rate. **One 802.11b device can halve the
throughput for every other client on the radio.** **The remedy is disabling low data rates**,
which also usefully shrinks the cell.

**Airtime fairness** *(§44.2)* — **CSMA/CA gives equal transmission opportunities, not equal
time**, and a slow client occupies the medium far longer for the same data. **A client at
12 Mb/s consumes 97% of the airtime against one at 400 Mb/s moving the same 5 MB.** **One
slow client really does ruin it for everyone**, and airtime fairness scheduling, disabling low
rates, and better coverage are the remedies.

**Three frame types** *(§44.3)* — **Management** (join, leave, discover), **control** (medium
access), **data**. **Management frames carry the diagnosis**, and historically they were
unprotected.

**Beacons cost airtime** *(§44.3)* — Ten per second **per SSID per radio**, at the lowest
basic rate. **Four SSIDs on two radios is 78 beacons per second and nearly 8% of the
airtime.** **Three SSIDs maximum; two is better.**

**A client associates with a BSSID, not an SSID** *(§44.3)* — **One access point has several
BSSIDs** — one per radio per SSID — which is why a scan shows far more entries than devices.
**Roaming is moving between BSSIDs within an ESS, and the client decides.**

**Hidden SSIDs provide no security** *(§44.3)* — The name appears in every association and
probe. **And it makes things worse**: clients must probe actively, **broadcasting the network
name everywhere they go**, so the network becomes discoverable by following the client. It
also breaks some devices and slows roaming.

**The association sequence** *(§44.3)* — Discovery, **802.11 authentication (vestigial — Open
System accepts unconditionally)**, association, **then WPA2/WPA3 — which is the real
authentication.**

**The four-way handshake** *(§44.3)* — **The passphrase is never transmitted**; both sides
prove knowledge by producing a MIC. **A wrong passphrase fails at message 2.** **And capturing
the handshake permits an offline dictionary attack**, which is why WPA3's SAE matters.

**Deauthentication attacks** *(§44.3)* — Management frames were unauthenticated, so **anyone
can forge a disconnect.** Used for denial of service, for **forcing a handshake capture**, and
for evil twins. **802.11w PMF is the defence** — mandatory in WPA3, optional in WPA2, and
**worth enabling.**

**A sleeping client looks unresponsive** *(§44.3)* — First packet slow, subsequent packets
fast. **Normal, and frequently mistaken for a fault.**

**Where association fails** *(§44.3)* — Discovery (range, band, radio), association
(capabilities), **the handshake ("incorrect password" — which on enterprise means RADIUS,
certificate or credentials)**, and — **most commonly and most misattributed** — **after
association, which is DHCP and not wireless.**

**The four mechanisms, distinguished** *(§44.4)* — **SU-MIMO divides space for one client;
MU-MIMO divides space for several; OFDMA divides frequency for several; beamforming is not
multiplexing at all** — it improves one link.

**Streams are limited by the weaker end** *(§44.4)* — Phones have 1–2, laptops 2, sensors 1.
**An 8×8 access point gives a 2-stream laptop 2 streams**, so stream counts beyond about four
give diminishing throughput returns.

**MU-MIMO's requirements are demanding** *(§44.4)* — **Clients must be spatially separated**;
**channel state information goes stale and sounding costs airtime**; client support was slow.
**It helps in specific conditions and does nothing in many real situations** — less than the
marketing suggested.

**OFDMA is the Wi-Fi 6 change** *(§44.4)* — **Most Wi-Fi frames are small, and giving an
80 MHz channel to a 64-byte frame wastes almost all of it.** OFDMA divides the channel into
resource units so **several clients share one transmission and the per-frame overhead is paid
once for all of them.** **It does not make any client faster; it makes many clients cheaper to
serve.**

**And it is scheduled rather than contended** *(§44.4)* — **802.11 moves toward central
scheduling for the first time**, in the direction cellular has always worked.

**Beamforming** *(§44.4)* — Phase-controlled copies adding constructively at the receiver,
**3–5 dB of gain.** **Explicit** (the client reports back) is more accurate than **implicit**.
**It is why an access point can have eight antennas and three streams** — the extra chains
shape the beam rather than carrying data.

**Each modulation step costs ~6 dB** *(§44.4)* — Because the constellation points pack twice
as densely and the noise margin halves. **4096-QAM needs ~40 dB SNR** — a signal of −55 dBm
against a −95 dBm floor, **metres from the access point.**

**The full reduction** *(§44.4)* — 80 MHz, 256-QAM, 5/6, 2 streams gives **≈960 Mb/s
nominal**; **≈480 Mb/s** after protocol efficiency; **≈16 Mb/s** each with thirty active
clients. **A 9.6 Gb/s datasheet, 480 Mb/s alone, and 16 Mb/s in a busy room are the same
access point** — and every step is legitimate and predictable.
