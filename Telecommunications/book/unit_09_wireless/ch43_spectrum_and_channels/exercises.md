# Chapter 43 — Exercises

## A. Recall

**A1.** Why is spectrum governed by law rather than by protocol?

**A2.** Distinguish licensed from unlicensed spectrum in four respects.

**A3.** What were the ISM bands originally for, and what changed in 1985?

**A4.** What does DFS require a device to do, and for how long must it stay off a channel
after a radar detection?

**A5.** How wide is the 2.4 GHz band, how far apart are channel centres, how wide is a
channel, and how many non-overlapping channels result?

**A6.** Why is partial channel overlap worse than sharing a channel?

**A7.** State the trade between channel width and channel count, and give the noise penalty
of doubling the width.

**A8.** Give the thermal noise floor formula and compute it for a 20 MHz channel with a 5 dB
noise figure.

**A9.** Distinguish co-channel from adjacent-channel interference by cause, effect and
remedy.

**A10.** What can a spectrum analyser see that a Wi-Fi adapter cannot?

## B. Apply

**B1.** Compute the frequency span of 2.4 GHz channels 1, 6 and 11, and confirm they do not
overlap. Then do the same for 1, 5, 9 and comment.

**B2.** Compute the noise floor for 20, 40, 80, 160 and 320 MHz channels with a 6 dB noise
figure. State the range consequence of moving from 20 to 160 MHz, in dB.

**B3.** A building needs 40 access points on 5 GHz. Compute the number of access points per
channel at 20, 40, 80 and 160 MHz widths, assuming 25 non-overlapping 20 MHz channels.
Recommend a width and justify.

**B4.** A client measures RSSI −55 dBm and SNR 14 dB.

(a) What is the noise floor?
(b) How far above thermal (20 MHz, NF 5 dB) is it?
(c) Is this a coverage problem or an interference problem?
(d) What would you measure next?

**B5.** For each observation, give the diagnosis:

(a) 75% channel utilisation, 3% retries
(b) 30% utilisation, 28% retries, RSSI −52 dBm
(c) 30% utilisation, 28% retries, RSSI −79 dBm
(d) Noise floor −72 dBm, no networks visible on the channel
(e) All clients disconnect simultaneously; the AP logs a channel change

**B6.** A 6 GHz deployment in the US uses 160 MHz channels. How many are available? Repeat
for the EU allocation. What does this imply for a multinational design?

**B7.** An access point transmits at 20 dBm EIRP. A neighbour raises theirs from 20 to
26 dBm. By how much has your noise floor from them risen, and what is the effect on your
usable range?

## C. Analyse

**C1.** Explain why the ISM bands were given away and why that decision produced Wi-Fi.
Connect it to Chapter 28 §28.1's argument about adoption.

**C2.** Explain why "just turn the power up" is unavailable as a remedy, giving both the
regulatory and the physical reason.

**C3.** Derive the 1/6/11 rule from the band width, channel spacing and channel width.

**C4.** Explain, mechanistically, why two access points on channels 1 and 3 perform worse
than two on channel 1.

**C5.** Explain why narrow channels are better in dense deployments, using the forty-AP
example.

**C6.** Explain why avoiding DFS is individually rational and collectively harmful.

**C7.** Explain the microwave oven's interference signature and why it produces a complaint
that recurs at the same time each day.

**C8.** "A survey showing a clear channel that does not work" — explain what this means and
why a Wi-Fi adapter cannot diagnose it.

**C9.** Explain why 70% channel utilisation is the practical ceiling rather than 100%,
referring to Chapter 16's analysis.

**C10.** Explain why band steering can make performance worse, and state the condition under
which it should be enabled.

**C11.** Set out the tragedy of the commons in unlicensed spectrum: the individually rational
choice, the collective outcome, and the three partial mitigations. Compare with Chapter 32
§32.4.

## D. Design

**D1.** Design the channel plan for a three-storey office building with eight access points
per floor, on both 2.4 and 5 GHz. Show the plan for all three floors and explain the
vertical considerations.

**D2.** For the semester project's site, specify the band strategy: which bands on which
access points, what widths, and the reasoning.

**D3.** An organisation has a 5 GHz deployment using only UNII-1 and UNII-3. Write the case
for enabling DFS, including what could go wrong and how you would mitigate it.

**D4.** Design a tri-band deployment. Specify access-point spacing, and explain why spacing
must be determined by 6 GHz rather than 5 GHz.

**D5.** Write the interference investigation procedure for a helpdesk escalation: what is
measured, in what order, and what each result rules out.

## E. Troubleshoot

**E1.** Users in a lecture theatre report that wireless is unusable when the room is full
and fine when it is empty. Give two mechanisms.

**E2.** Wireless in a break room fails between 12:00 and 14:00 daily. Diagnose.

**E3.** A floor has good RSSI everywhere and universally poor performance. Retries are 25%.
Give your next three measurements.

**E4.** After a neighbouring business moved in, one channel became unusable. What are your
options, and what is not available to you?

**E5.** An access point refuses to use the channel it is configured for and reports nothing
in the log for 60 seconds. Explain.

**E6.** A site has 25 access points on 5 GHz with 80 MHz channels and complaints about
speed. Utilisation is 85% on several. Give the fix and explain the arithmetic.

**E7.** A wireless camera was installed in a storeroom, and three floors began reporting
problems. Explain the mechanism and the remedy.

**E8.** After enabling 6 GHz on existing access points, clients in some areas report worse
performance than before. Explain.

## F. Extend

**F1.** Use a Wi-Fi analyser to survey your own environment. Record every network, its
channel and its width. Identify overlaps, and assess whether the channel plan is sensible.

**F2.** Measure channel utilisation and retry rate on a busy network at a quiet time and a
busy time. Correlate with user experience.

**F3.** Obtain an RTL-SDR or use an access point's spectrum analysis. Observe the 2.4 GHz
band with a microwave oven running. Photograph or describe the signature.

**F4.** Measure the noise floor at several locations in a building. Identify the highest and
find out why.

**F5.** Compute the noise floor for every channel width using
[tools/perfcalc.py](../../../tools/perfcalc.py) `noise`, and verify one by hand.

**F6.** Find your country's regulations for 2.4, 5 and 6 GHz: available channels, EIRP
limits, DFS requirements, and indoor/outdoor restrictions. Compare with one other country.
