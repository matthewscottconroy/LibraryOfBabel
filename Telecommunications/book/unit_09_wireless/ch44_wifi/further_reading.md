# Chapter 44 — Further Reading

## Primary sources

**IEEE 802.11-2020** (the consolidated standard).
Four thousand pages, and not for reading through. Worth knowing how to look things up in:
**Clause 9** is frame formats, **Clause 10** is the MAC including CSMA/CA and the NAV,
**Clause 12** is security. The IEEE now publishes it free after six months via the Get
IEEE 802 programme.

Karn, P. (1990). "MACA — A New Channel Access Method for Packet Radio." *ARRL Computer
Networking Conference*.
RTS/CTS, in nine pages. The hidden-node argument of §44.2, made by the person who solved
it. Short and clear.

Bharghavan, V. et al. (1994). "MACAW: A Media Access Protocol for Wireless LANs."
*ACM SIGCOMM*.
The refinement — acknowledgements and better backoff — that 802.11's DCF is based on.

Foschini, G. J. & Gans, M. J. (1998) and **Telatar, E. (1999).**
The MIMO capacity results — Chapter 42's reading list.

Vanhoef, M. & Piessens, F. (2017). "Key Reinstallation Attacks: Forcing Nonce Reuse in
WPA2." *ACM CCS*.
**KRACK.** Read it for the method as much as the result: attacking a state machine that had
been proved secure, by exploiting what the proof did not cover.

## Books

Gast, M. (2005). *802.11 Wireless Networks: The Definitive Guide*, 2nd ed. O'Reilly.
The book on 802.11's mechanism. Dated on standards — it predates n — and unmatched on how
the MAC actually works. The frame-format and MAC chapters are still the clearest
explanation available.

Gast, M. (2013). *802.11ac: A Survival Guide*. O'Reilly.
Short, free from O'Reilly, and excellent on MIMO, MU-MIMO, channel bonding and why the
headline rates are unattainable. If you read one thing from this list, this.

Coleman, D. & Westcott, D. — *CWNA Study Guide*.
Again the best practical single volume for the unit.

Perahia, E. & Stacey, R. (2013). *Next Generation Wireless LANs: 802.11n and 802.11ac*,
2nd ed. Cambridge.
The technical treatment of the modern PHY — MIMO, beamforming, aggregation — by Intel
engineers who worked on it. Mathematical, and the right depth if §44.4 left you wanting more.

Bejarano, O., Knightly, E. & Park, M. — the 802.11ac and ax survey papers.
Freely available, and good on what the standards actually deliver versus what they
promise.

## Applied

**Get into monitor mode and capture.** This chapter is not really learnable otherwise.

```bash
# Linux
sudo airmon-ng check kill
sudo airmon-ng start wlan0
sudo tcpdump -i wlan0mon -nn -e

# macOS — no special tools needed
sudo /System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport en0 sniff 36
```

**Exercise F1 — decode a beacon fully.** SSID, BSSID, channel, rates, capabilities, RSN.
Twenty minutes, and every field in §44.3 becomes concrete.

**Exercise F2 — capture a four-way handshake.** Filter `eapol`, watch the four messages, and
identify what each proves. Then note that you have just captured what an offline dictionary
attack needs, which makes §44.3's security argument visceral.

**Wireshark filters worth having to hand:**

```
wlan.fc.type_subtype == 8      # beacons
wlan.fc.type_subtype == 4      # probe requests — watch what devices leak
wlan.fc.type_subtype == 12     # deauthentication
wlan.fc.retry == 1             # retransmissions
eapol                          # the handshake
wlan.ta == aa:bb:cc:dd:ee:ff   # one transmitter
```

`wlan.fc.type_subtype == 4` is worth ten minutes of your attention. Probe requests from
phones leak the names of networks those phones have joined before — including hidden ones
(§44.3), home networks, and previous employers'. It is a privacy problem, and observing it
is the fastest way to understand why MAC randomisation was introduced.

Watch MCS change as you walk (exercise F5):

```bash
watch -n1 'iw dev wlan0 link'         # Linux
# macOS: Option-click the Wi-Fi menu shows the rate live
```

Plot MCS against distance and compare with §44.4's SNR table. This single measurement
connects Chapter 42's physics, Chapter 43's noise floor and this chapter's modulation.

Count beacons and estimate their airtime (exercise F3). If your environment has six
SSIDs, the answer is uncomfortable.

Measure retry rate at close range and at the edge (exercise F4), and relate it to
Chapter 43 §43.4's diagnosis table.

**`iw` for the details:**

```bash
iw dev wlan0 link            # rate, MCS, signal
iw dev wlan0 station dump    # on an AP: per-client rates and retries
iw dev wlan0 scan | grep -E 'SSID|signal|DS Parameter|HT|VHT|HE'
```

**Lab 33** in this book's [labs/](../../../labs/) directory captures a full association
including the handshake, demonstrates a deauthentication on an isolated network and then
defeats it with 802.11w, measures the airtime-fairness effect by introducing a deliberately
slow client, and quantifies the protection-mechanism cost of one legacy device.

## For the certification-minded

Objective 2.4 expects the 802.11 standards and their characteristics, and the table is
examined directly.

**The standards table is the memorisation item:**

| | Band | Max rate | Wi-Fi name |
|---|---|---|---|
| **802.11a** | **5 GHz** | 54 Mb/s | — |
| **802.11b** | **2.4 GHz** | **11 Mb/s** | — |
| **802.11g** | 2.4 GHz | 54 Mb/s | — |
| **802.11n** | **both** | 600 Mb/s | **Wi-Fi 4** |
| **802.11ac** | **5 GHz only** | ~7 Gb/s | **Wi-Fi 5** |
| **802.11ax** | both (+6 GHz as 6E) | ~9.6 Gb/s | **Wi-Fi 6 / 6E** |
| 802.11be | all three | ~46 Gb/s | Wi-Fi 7 |

The a-versus-b confusion is the most-missed item: `a` is the fast one on 5 GHz and `b`
is the slow one on 2.4 GHz, despite the lettering.

Seven more things worth over-learning:

1. **CSMA/CA, not CSMA/CD** — wireless cannot detect collisions.
2. Every unicast frame is acknowledged.
3. SSID is the name; BSSID is the access point radio's MAC.
4. The hidden node problem, and RTS/CTS as its mitigation.
5. MIMO uses multiple antennas for multiple spatial streams.
6. MU-MIMO serves several clients at once; OFDMA divides the channel in frequency.
7. Deauthentication attacks, evil twins, and 802.11w PMF.

And the four operational facts worth more than the objective:

Real throughput is about half the nominal rate, and you can now explain why.

**One 802.11b device halves a cell's throughput.** Disable low rates.

One slow client can consume 97% of the airtime. Check airtime fairness is enabled.

A client that associates and gets no address has a DHCP problem, not a wireless one —
and this is the most commonly misattributed wireless complaint there is.
