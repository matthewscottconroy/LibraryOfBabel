# Chapter 44 — Exercises

## A. Recall

**A1.** Give the band and maximum rate for 802.11a, b, g, n, ac and ax, and their Wi-Fi
Alliance names where they have one.

**A2.** Why is the amendment lettering not chronological?

**A3.** Give three reasons wireless cannot use collision detection.

**A4.** State the CSMA/CA sequence in seven steps.

**A5.** Why is every unicast 802.11 frame acknowledged?

**A6.** Define the hidden node problem and the exposed node problem.

**A7.** Distinguish SSID, BSSID, BSS and ESS.

**A8.** Give the four stages of association and say which one is the real authentication.

**A9.** Distinguish SU-MIMO, MU-MIMO, OFDMA and beamforming by what each divides and whom
each serves.

**A10.** What SNR does 256-QAM require? 1024-QAM? 4096-QAM?

## B. Apply

**B1.** Compute the CSMA/CA efficiency for a 1500-byte frame at 24 Mb/s, 150 Mb/s and
2.4 Gb/s, using DIFS 34 µs, average backoff 67 µs, preamble 20 µs, SIFS 16 µs, ACK 24 µs.
Comment on the trend and state what mitigates it.

**B2.** An access point has 4 SSIDs on 2 radios, beacon interval 102.4 ms. How many beacons
per second does it transmit? If each takes 1 ms at the lowest basic rate, what fraction of
airtime is consumed?

**B3.** Compute the nominal PHY rate for:

(a) 40 MHz, 64-QAM, 3/4 coding, 2 streams, 13.6 µs symbol, 468 subcarriers
(b) 160 MHz, 1024-QAM, 5/6 coding, 4 streams, 13.6 µs symbol, 1960 subcarriers

Then give realistic achieved throughput for each, and per-client throughput with 25 active
clients.

**B4.** A 4×4:4 access point serves: two 2-stream laptops, four 1-stream phones and twelve
1-stream sensors. How many spatial streams can each achieve? What is the maximum the AP can
use with SU-MIMO to one client?

**B5.** Client A is at 400 Mb/s and client B at 12 Mb/s. Both transfer 5 MB.

(a) How long does each take, alone?
(b) With equal transmission opportunities, what fraction of airtime does B consume?
(c) What mechanism addresses this?

**B6.** For each observation, give the diagnosis:

(a) 30% retries, all clients at −55 dBm, throughput poor
(b) Throughput exactly half the nominal rate
(c) A network with one 802.11b device and 40% of expected throughput
(d) Clients disconnect every few minutes; capture shows subtype 12 frames
(e) A client associates and has no IP address

**B7.** A 20 MHz OFDMA channel serves nine clients with 26-subcarrier resource units. Compare
the airtime for nine 100-byte frames with and without OFDMA, using §44.2's overhead figures.

## C. Analyse

**C1.** Explain why 802.11b succeeded commercially and 802.11a did not, and identify the same
pattern elsewhere in this book.

**C2.** Explain why frame aggregation was essential rather than optional for 802.11n, using
the efficiency arithmetic.

**C3.** Explain, from first principles, why a radio cannot detect collisions.

**C4.** Explain why priority in 802.11 is implemented as different waiting times, and why
this needs no negotiation.

**C5.** Explain why the hidden node problem cannot be solved by carrier sense alone, and what
RTS/CTS substitutes for it.

**C6.** Explain the NAV and why it is called "virtual carrier sense".

**C7.** Explain why the exposed node problem wastes capacity, and what BSS colouring does
about it.

**C8.** Explain the airtime fairness problem completely: the mechanism, the arithmetic, and
three remedies.

**C9.** Give the full argument against hiding an SSID — why it does not provide security and
three ways it makes things worse.

**C10.** Explain what a captured four-way handshake permits, and why WPA3's SAE changes it.

**C11.** Explain why deauthentication attacks work, what they enable, and what 802.11w does.

**C12.** Explain the difference between OFDMA and MU-MIMO in terms of what each divides,
and state which helps more in a dense deployment of small frames.

**C13.** Explain why 4096-QAM is usable only very close to an access point, using the SNR
and noise-floor arithmetic.

**C14.** Walk through the reduction from a 9.6 Gb/s datasheet figure to 16 Mb/s per client
in a busy room, justifying every step.

## D. Design

**D1.** For the semester project's site, specify: the 802.11 generation, the SSIDs and their
purposes, the channel widths, and the rate settings. Justify each.

**D2.** An organisation has 300 IoT sensors that report once a minute and 200 laptops.
Design the wireless configuration, and explain which Wi-Fi 6 features address which
population.

**D3.** Write the SSID policy for an enterprise: how many, what for, and the argument you
would give management for reducing an existing six to three.

**D4.** Design the rate configuration for a high-density lecture theatre: which rates are
enabled, which disabled, and the effect of each choice on cell size and airtime.

**D5.** A network suffers from hidden nodes in a warehouse. Write the remediation plan,
considering RTS/CTS, access-point placement, cell size and antenna choice.

## E. Troubleshoot

**E1.** Users report that wireless "gets slower as the day goes on" in an open-plan office
that fills up. Give two distinct mechanisms.

**E2.** A capture shows 35% retries with every client above −60 dBm. Give three candidate
causes and how to distinguish them.

**E3.** A cell performs badly only when one particular contractor's old laptop is present.
Diagnose and give the fix.

**E4.** After enabling RTS/CTS globally, throughput fell everywhere. Explain.

**E5.** A Wi-Fi 6 upgrade produced no measurable improvement. Give four possible reasons.

**E6.** A client shows "incorrect password" on an 802.1X network although the password is
correct. Give three causes.

**E7.** An IoT device cannot join a network that laptops use successfully. Give three
candidate causes.

**E8.** A survey shows six SSIDs from your own controller and users complain about speed.
Explain the connection.

**E9.** Ping to an idle phone shows 300 ms for the first packet and 3 ms thereafter. Is this
a fault?

**E10.** A stadium deployment with 160 MHz channels performs worse than the 40 MHz
deployment it replaced. Explain, referring to Chapter 43 §43.2.

## F. Extend

**F1.** Put an adapter into monitor mode and capture beacons. Decode one fully: SSID, BSSID,
channel, supported rates, capabilities, RSN.

**F2.** Capture a complete association including the four-way handshake. Identify each of the
four EAPOL messages and what each proves.

**F3.** Count beacons per second in your environment and estimate the airtime they consume.
Compare with your answer to B2.

**F4.** Measure retry rate (`wlan.fc.retry == 1`) during a large transfer at close range and
at the edge of coverage. Explain the difference.

**F5.** Use `iw dev wlan0 link` or equivalent to observe the negotiated MCS while walking away
from an access point. Plot MCS against distance and relate it to §44.4's SNR table.

**F6.** Compare achieved throughput with the nominal PHY rate for your own connection.
Account for the difference using §44.2's overhead analysis.

**F7.** If you have equipment that supports it, measure throughput with and without frame
aggregation. Quantify its contribution.
