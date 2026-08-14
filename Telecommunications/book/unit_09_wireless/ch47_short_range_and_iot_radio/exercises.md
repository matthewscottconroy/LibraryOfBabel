# Chapter 47 — Exercises

## A. Recall

**A1.** State the access method classic Bluetooth uses and the hop rate, and give two
properties it buys.

**A2.** Give three differences between classic Bluetooth and BLE, and state the design
principle behind BLE's battery life.

**A3.** What are BLE's three advertising channels, and why were those three chosen?

**A4.** Describe the GATT hierarchy, and say what property it gives a BLE device.

**A5.** State the frame size, data rate and bands of IEEE 802.15.4.

**A6.** Give the most significant architectural difference between Zigbee and Thread.

**A7.** What is Matter, in one sentence, and name the transports it runs over.

**A8.** State the range/rate/power trilemma and say which corner LPWAN occupies.

**A9.** What is the European duty cycle limit on the 868 MHz sub-bands, in per cent and in
seconds per hour?

**A10.** Give the three LoRaWAN device classes and state which is the default and why.

**A11.** Distinguish a passive from an active RFID tag, and state how a passive tag replies in
the near field and in the far field.

**A12.** State NFC's frequency and typical range, and say what its security model actually
rests on.

## B. Apply

**B1.** Compute the near-field boundary $\lambda/2\pi$ for 125 kHz, 13.56 MHz, 868 MHz and
2.4 GHz. Which of these technologies operate in the near field at their working distance, and
which do not?

**B2.** NFC coupling falls as $1/d^6$.

(a) A card reads reliably at 4 cm. By how many decibels is the coupling weaker at 12 cm?
(b) At what distance is the coupling 60 dB below its value at 4 cm?
(c) Compare with the far-field $1/d^2$ law over the same distance ratio in (a). What does the
difference tell you about NFC's range limit?

**B3.** A LoRaWAN device in Europe sends a 12-byte payload.

(a) At SF7 (50 ms time on air) and at SF12 (1.5 s), how many messages per hour does the 1%
duty cycle permit?
(b) The application requires a reading every minute. Which of SF7, SF9 and SF12 are
permissible? Show the arithmetic.
(c) The device is moved and only SF12 now reaches a gateway. What must change in the
application?

**B4.** A LoRaWAN gateway serves 500 Class A devices and is subject to the same 1% duty cycle
on its downlink. Each downlink acknowledgement takes 1.5 s at SF12.

(a) How many downlinks per hour can the gateway send in total?
(b) If every device required an acknowledgement, how often could each be acknowledged?
(c) Explain in one sentence why LoRaWAN's confirmed-uplink mode is discouraged in dense
deployments.

**B5.** An 802.15.4 frame carries about 102 bytes of payload after the MAC header and
security.

(a) How many application bytes remain after an uncompressed IPv6 header and a UDP header?
(b) How many remain if 6LoWPAN compresses the IPv6 header to 3 bytes and the UDP header to 4?
(c) IPv6 requires a minimum MTU of 1,280 bytes. State how 6LoWPAN reconciles this with (a).

**B6.** Using the free-space path loss expression from Chapter 42 §42.3, compute the
difference in path loss between 868 MHz and 2.4 GHz at the same distance. State the result in
decibels and explain what it means for an LPWAN designer's choice of band.

**B7.** A Sigfox device is limited to 140 messages per day.

(a) What is the minimum interval between messages?
(b) A customer wants a five-minute reporting interval. Is Sigfox suitable? Show the
arithmetic.

## C. Analyse

**C1.** BLE's coded PHY trades 1 Mb/s for 125 kb/s and gains roughly four times the range.
Express the range gain in decibels of link budget, and relate the trade to Chapter 4's channel
capacity argument. Is the exchange favourable, and under what circumstances?

**C2.** Zigbee's mesh gets better as you add mains-powered devices; Thread's does too. Explain
why battery-powered devices cannot route in either, and what that implies for the placement of
a battery sensor at the edge of coverage.

**C3.** Bluetooth mesh uses managed flooding rather than routing. Compare it with Thread's
routed mesh on: message overhead, convergence after a node fails, scalability, and
implementation complexity. Argue which is the better choice for a 400-luminaire lighting
installation.

**C4.** LoRaWAN devices do not associate with a gateway; every gateway that hears a
transmission forwards it and the network server deduplicates. Analyse the consequences of this
design for (a) mobility, (b) gateway deployment economics, (c) downlink, and (d) security.

**C5.** Matter is an application layer over several transports rather than a new network.
Argue why that architectural choice was made, referring to Chapter 21's layering argument and
to what Zigbee's profile fragmentation cost it.

**C6.** NFC's short range is described in §47.4 as a security property. Assess this claim
honestly: state what it does protect against, what it does not, and why relay attacks defeat
it while eavesdropping is genuinely hard.

**C7.** Sigfox entered receivership in 2022. Explain what class of risk this represents that a
purely technical evaluation would not have surfaced, and identify which of the other
technologies in this chapter share it.

## D. Design

**D1.** A farm of 800 hectares needs soil moisture readings from 400 sensors, every 30
minutes, with a ten-year battery life and no recurring cost. There is mains power and
Internet at the farmhouse only. Design the system: technology, spreading factor strategy,
number and siting of gateways, and the duty-cycle budget. Show that your design fits within
the 1% limit.

**D2.** A hospital wants asset tracking for 3,000 items — beds, pumps, wheelchairs — with
room-level accuracy and five-year battery life on the tags. Evaluate BLE, UWB, and UHF RFID
against the requirement, and recommend one with justification. State what would change your
recommendation.

**D3.** A new-build house is to be fitted with 60 smart devices: lights, switches, sensors,
locks, thermostats, three cameras and a doorbell. Specify which transport each class of device
should use and why, whether Matter should be required, and what the failure modes are when the
Internet connection is down.

**D4.** Design the connectivity for a fleet of 10,000 water meters in buried pits across a
city, read monthly, with a fifteen-year battery life. Compare a private LoRaWAN deployment
against NB-IoT on total cost of ownership over fifteen years, and state the non-technical
factors that should decide it.

**D5.** A car manufacturer wants keyless entry that resists relay attacks. Specify the
technology, explain the mechanism that provides the resistance, and state what the system must
do if the ranging measurement is unavailable.

## E. Troubleshoot

**E1.** A Zigbee network in a house is unreliable, and the problem is worse in the evening.
Signal strength at the hub is reported as good. Give the most likely cause, the measurement
that would confirm it, and the fix.

**E2.** A BLE sensor's battery, specified for two years, is exhausted in three months. List
four possible causes and the measurement that distinguishes them.

**E3.** A LoRaWAN device transmits successfully during testing and then stops sending for
long periods once deployed, with no error reported. What is happening, and what has changed
between the test and the deployment?

**E4.** UHF RFID readers at a warehouse dock read cardboard-boxed goods reliably and miss
roughly a third of tags on palletised bottled water. Explain the physics, and give two
practical mitigations.

**E5.** A Thread network's devices are individually reachable but a newly added battery sensor
at the far end of the house joins and then drops repeatedly. Give the likely cause and the
diagnostic step.

**E6.** A Matter device is certified, and it commissions successfully into Apple Home but
fails into Google Home. State where in the stack the problem is most likely to be, and what
information you would gather.

**E7.** A contactless card is read at a distance the user did not expect. Explain whether this
indicates a security failure, and what actually protects the transaction.

**E8.** A UWB-based item finder reports a distance that jumps by two metres as the user walks
around a corner. Explain the mechanism, referring to Chapter 42.

## F. Extend

**F1.** Read the LoRaWAN regional parameters for your own region and compare the duty cycle,
maximum payload and dwell-time rules with the European ones described in §47.3. Explain how a
device designed for Europe must change to operate in the United States.

**F2.** Install a generic BLE scanner application on a phone and enumerate the GATT services
of three devices around you. Identify which use standard UUIDs and which are proprietary, and
write a short note on what a device reveals about itself before any pairing occurs.

**F3.** Investigate the 802.15.4 channel occupancy in your building with a spectrum analyser
or an inexpensive Zigbee sniffer. Determine which of channels 15, 20, 25 and 26 is quietest,
and relate the result to the Wi-Fi channels in use (Chapter 43 §43.2).

**F4.** Research distance bounding protocols. Explain the cryptographic and physical
requirements that make them work, why they were not deployed for twenty years after being
proposed, and what changed.

**F5.** Estimate the number of passive RFID tags manufactured worldwide in a recent year, and
compare it with the number of Wi-Fi and cellular devices. Write a paragraph on what the
comparison suggests about which radio technology is most widely deployed, and why it is the
one least discussed.

**F6.** Take one commercial smart-home product and determine, from its documentation and from
observing its traffic, whether it can function with no Internet connection. Report what breaks
and what continues to work, and relate the result to §47.2's argument about local control.
