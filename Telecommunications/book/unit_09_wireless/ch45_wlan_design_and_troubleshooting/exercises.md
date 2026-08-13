# Chapter 45 — Exercises

## A. Recall

**A1.** Name the four survey types and state what each measures.

**A2.** What is the design-target RSSI for voice, and what SNR accompanies it?

**A3.** Give four rules for conducting a survey properly.

**A4.** What cell overlap is required for clean roaming, and what happens with too little and
too much?

**A5.** Who decides when a client roams?

**A6.** Distinguish 802.11k, 802.11v and 802.11r by what each addresses.

**A7.** Give the four controller architectures and the data path of each.

**A8.** State the opposite parameter choices of coverage design and capacity design, for at
least four parameters.

**A9.** What is the working target for clients per radio?

**A10.** Give the four measurements for diagnosing a slow-wireless complaint, in order.

## B. Apply

**B1.** A lecture theatre seats 350. Estimate concurrent devices, active devices, and the
number of access points required — by throughput and by client count. State which binds and
why.

**B2.** A warehouse is 8,000 m² with 40 handheld scanners. Estimate the access-point count
and specify power, channel width, antennas, minimum rate and 2.4 GHz policy. Contrast every
choice with your answer to B1.

**B3.** For each measurement set, give the diagnosis:

(a) RSSI −78, SNR 14, retries 22%, utilisation 20%
(b) RSSI −55, SNR 12, retries 30%, utilisation 25%
(c) RSSI −58, SNR 32, retries 4%, utilisation 82%
(d) RSSI −60, SNR 30, retries 3%, utilisation 15%, user says it is slow
(e) RSSI −57, SNR 31, retries 24%, utilisation 30%

**B4.** Compute the total roaming time for an enterprise client without 802.11k or 802.11r,
using §45.2's figures. Then with both. State whether each supports voice.

**B5.** A client roams between access points on different subnets. List everything that
breaks and give three remedies in order of preference.

**B6.** For each complaint, give the first question you would ask and what the answer would
rule out:

(a) one user cannot connect
(b) everyone in the east wing says it is slow
(c) everyone everywhere says it is slow
(d) a user's calls drop when they walk to the kitchen

**B7.** An access point serves 65 clients on its 5 GHz radio. Utilisation is 78%. Is adding a
second access point at the same power the right answer? Justify with §45.3's reasoning.

## C. Analyse

**C1.** Explain why a predictive survey's accuracy depends on wall data, quantifying the
error from one misidentified wall type.

**C2.** Explain why you should survey with the worst common client device rather than a
survey adapter.

**C3.** Explain why the coverage target is −67 dBm rather than the signal a client will
tolerate, referring to airtime.

**C4.** Explain why raising the minimum basic rate is a better cell-shrinking lever than
reducing transmit power.

**C5.** Explain why the network cannot make a client roam, and enumerate the three indirect
mechanisms available.

**C6.** Explain the three compounding reasons that more access points at lower power
increases capacity.

**C7.** Explain why client count rather than throughput determines access-point count in
dense environments, referring to CSMA/CA.

**C8.** Explain why raising transmit power in a dense deployment reliably makes it worse.

**C9.** Compare centralised-controller, local-switching and cloud-managed architectures by
failure mode. Which fails worst when a WAN link drops, and why?

**C10.** Explain why RRM can produce a plan that is optimal by its own metric and poor in
practice.

**C11.** "Wireless is where complaints arrive, not where they originate." Defend this, and
give the single test that settles it.

**C12.** Explain why "incorrect password" on an enterprise network has five possible causes,
and where you would look to distinguish them.

## D. Design

**D1.** Design the WLAN for the semester project's site. Specify: survey approach, AP count
and placement, power, channel widths, minimum rates, SSIDs, roaming amendments, and the
controller architecture. Justify each against this unit.

**D2.** Design for a 400-seat auditorium used for conferences. Include the mounting strategy
and explain why it differs from an office.

**D3.** An organisation's wireless "worked fine until we moved to hot-desking and occupancy
doubled". Write the remediation plan.

**D4.** Write the survey specification you would give a contractor: what they must measure,
with what, under what conditions, and what the deliverable must contain.

**D5.** Design the voice-over-WLAN configuration for a hospital: coverage target, roaming
amendments, VLAN strategy, QoS, and what you would test before go-live.

## E. Troubleshoot

**E1.** A newly-surveyed office has complaints in a meeting room that surveyed well.
Diagnose.

**E2.** Voice calls drop consistently when users walk from one wing to another. Give three
candidate causes and how to distinguish them.

**E3.** A Windows laptop holds an access point at −84 dBm while three better ones are
audible. Explain and give three remedies.

**E4.** Every client on one access point disconnected at 14:07. The AP log shows a channel
change. Diagnose.

**E5.** After adding six access points to a busy floor, performance got worse. Explain what
was probably not done.

**E6.** A tester measures 400 Mb/s in an empty room and users report 10 Mb/s when it is full.
Is anything broken?

**E7.** Handheld scanners roam badly; laptops are fine. Where do you look?

**E8.** An IoT sensor cannot join the corporate SSID. Give four candidate causes.

**E9.** Users report slow wireless. RSSI −59, SNR 33, retries 2%, utilisation 12%. What do
you do next?

**E10.** After enabling 802.11r, a group of older handsets can no longer connect. Give the
cause and two options.

## F. Extend

**F1.** Conduct a passive survey of a space you have access to. Produce a heat map and
identify any area below −70 dBm.

**F2.** Repeat as an active survey while associated, measuring throughput. Compare the two
maps and explain any difference.

**F3.** Measure the same space with a laptop and with a phone. Quantify the difference and
state which you would design against.

**F4.** Capture the roaming process while walking between two access points. Measure the gap
in traffic, and identify each stage.

**F5.** Use `netsh wlan show wlanreport` (Windows) or the controller's client history to
review a week of one client's connections. Identify every disconnection and its reason.

**F6.** For an access point you control, count the associated clients and measure aggregate
throughput. Compare with §45.3's per-radio targets.

**F7.** Take a real complaint of "wireless is slow", work §45.4's four measurements, and
document the conclusion — including if the conclusion is that it was not wireless.
