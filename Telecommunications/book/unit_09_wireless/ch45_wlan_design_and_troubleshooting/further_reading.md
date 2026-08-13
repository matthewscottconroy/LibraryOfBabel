# Chapter 45 — Further Reading

## Books

**Coleman, D. & Westcott, D. — *CWNA Certified Wireless Network Administrator Study
Guide*.**
**The single best book for this unit.** The site survey, design and troubleshooting chapters
are exactly this chapter's material, at working depth.

**Coleman, D., Westcott, D. & Harkins, B. — *CWDP Certified Wireless Design
Professional*.**
**The design-specific volume**, and the one to read if you will actually specify
deployments. Capacity planning, high-density design and the survey methodology in full.

**Coleman & Harkins — *CWAP Certified Wireless Analysis Professional*.**
The troubleshooting side: reading captures, interpreting the four measurements of §45.4, and
the frame-level analysis that Chapter 44 §44.3 introduces.

**Minella, J. (2022). *Wireless Security Architecture.* Wiley.**
The enterprise authentication and policy material behind §45.4's "incorrect password has five
causes", and the practical treatment of 802.1X deployment.

**Bardwell, J. — the WLAN analysis papers.**
On what the measurements actually mean, and why vendors' RSSI figures differ. Freely
available and worth the hour.

## Vendor design guides

**They are marketing documents containing genuine engineering**, and the good ones are very
good:

- **Cisco Enterprise Mobility Design Guide** — long, thorough, and the high-density chapter
  is excellent regardless of whose equipment you run
- **Aruba Validated Reference Design** guides — particularly the high-density and
  voice-over-WLAN documents
- **Mist / Juniper** and **Ruckus** high-density guides
- **Ekahau's** survey methodology material

**Read at least two vendors' guides on the same topic.** Where they agree, it is engineering;
where they differ, it is usually product positioning, **and telling the two apart is a useful
skill.**

## Standards

**IEEE 802.11k, 802.11v, 802.11r** — now folded into 802.11-2020, clauses 11 and 13.
Read the **802.11r key hierarchy** description if you want to understand why fast transition
is possible at all.

**Wi-Fi Alliance Voice-Enterprise** certification requirements.
**What a device must actually do to roam fast enough for voice** — more useful than
datasheets when selecting handsets.

## Applied

**Conduct a survey.** Exercise F1, and there is no substitute.

**Free and adequate for learning:**

- **NetSpot** (macOS/Windows) — free tier does small spaces
- **WiFiman** (mobile) — quick and surprisingly useful
- **Wireless Diagnostics** (macOS, built in) — Option-click the Wi-Fi menu → Window → Scan,
  and the Performance window plots RSSI, noise and rate live
- **`iw dev wlan0 survey dump`** on Linux for utilisation and noise

**Professional:** **Ekahau Sidekick**, **Hamina**, **AirMagnet**. Expensive, and if you will
do this for a living the Ekahau ecosystem is the de facto standard.

**Exercise F3 is the one that changes behaviour:** **survey the same space with a laptop and
with a phone.** The difference is usually 6–10 dB, and **it is the difference between a design
that works and one that does not** — because your users carry the phone.

**Capture a roam** (exercise F4):

```bash
# monitor mode, then walk between two APs
sudo tcpdump -i wlan0mon -nn 'wlan type mgt' -w roam.pcap
# in Wireshark: filter on your client's MAC and find the reassociation
```

**Measure the gap in data frames.** That is the roaming time of §45.2, and comparing it before
and after enabling 802.11r is the most convincing demonstration in the chapter.

**`netsh wlan show wlanreport`** on Windows (exercise F5) — **run it now**, on any Windows
machine. The HTML report shows three days of connections, disconnections and reasons, and
most people are surprised by what is in it.

**Controller data.** Whatever platform you run, it holds per-client RSSI and SNR history,
per-AP client counts, retry rates and roaming events. **Look there before capturing** — the
answer is frequently already recorded.

**Lab 34** in this book's [labs/](../../../labs/) directory conducts a passive and an active
survey of the same space, compares laptop and phone measurements, captures a roam with and
without 802.11r, and works a capacity estimate for a room whose actual occupancy is then
measured against it.

## For the certification-minded

Objective 3.1 expects site surveys and heat maps; objective 2.4 expects coverage, capacity
and roaming; **objective 5.4 is wireless troubleshooting and this chapter is directly
examined.**

Eight things worth over-learning:

1. **Survey types**: predictive, passive, active, spectrum — and what each measures.
2. **−67 dBm** is the voice coverage target.
3. **15–20% cell overlap** for roaming.
4. **802.11k neighbour reports, 802.11v transition management, 802.11r fast transition.**
5. **The client decides when to roam.**
6. **Coverage design wants large cells; capacity design wants small ones.**
7. **In dense environments, more access points at lower power.**
8. **The four measurements**: RSSI, SNR, retries, utilisation.

**The most likely examined scenario** is a user complaining of slow wireless with a good
signal, and **the expected answer is interference or capacity rather than coverage.**

And the three things worth more than the objective:

**Ask "how many people and where" before touching a tool.** It eliminates most causes for
free.

**`169.254.x.x` on a wireless client is a DHCP problem.** It is the most commonly
misattributed wireless complaint there is.

**When all four measurements are good and the user still complains, it is not the wireless.**
Test on wired, and the search moves to where the problem actually is.
