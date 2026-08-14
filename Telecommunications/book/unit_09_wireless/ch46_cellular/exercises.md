# Chapter 46 — Exercises

## A. Recall

**A1.** State the cellular idea in one sentence, and say what it replaced.

**A2.** Why did the cellular idea wait thirty years between proposal and deployment?

**A3.** Give the valid reuse factors and the formula that generates them.

**A4.** Distinguish hard from soft handover, and say which technology requires which.

**A5.** What did 2G's SIM separate, and why did that matter commercially?

**A6.** What is the near-far problem, and what does CDMA do about it?

**A7.** What is cell breathing, and in which technology does it occur?

**A8.** What is the largest architectural change in LTE?

**A9.** Distinguish 5G SA from NSA, and state what NSA cannot do.

**A10.** Name the three 5G service classes and state which is actually deployed.

## B. Apply

**B1.** A city has 336 channels. Compute channels per cell and the D/R ratio for N = 3, 4, 7,
12 and 19. For a 2 km cell radius, give the co-channel reuse distance in each case.

**B2.** A pre-cellular system has 56 channels citywide. A cellular system covers the same
city with 40 cells at N = 7.

(a) How many simultaneous calls does each support?
(b) By what factor did capacity increase?
(c) The cells are split into four each. Recompute.

**B3.** For each requirement, choose a 5G band and justify:

(a) rural coverage over 20 km
(b) a stadium concourse needing multi-gigabit
(c) city-wide capacity in a business district
(d) a factory floor needing deterministic latency

**B4.** Compare LTE and Wi-Fi as multiple-access schemes: how each allocates the medium, what
happens under load, and which achieves higher efficiency when busy. Refer to Chapter 44 §44.2.

**B5.** A VoLTE call and a CSFB call are placed on the same handset. Compare setup time, data
throughput during the call, and audio quality.

**B6.** For each scenario, state whether private 5G or Wi-Fi is the better answer, with the
deciding factor:

(a) a 4-storey office of 300 desks
(b) a 2 km² container port with cranes and vehicles
(c) a warehouse with handheld scanners
(d) an open-cast mine with autonomous haul trucks
(e) a hospital ward

## C. Analyse

**C1.** Explain why one high-power transmitter gives a capacity that cannot be increased, and
why many low-power transmitters gives one that can.

**C2.** Explain why the cellular idea was a computing problem rather than a radio problem.

**C3.** Explain the trade encoded in the reuse factor, and why modern systems use N = 1.

**C4.** Explain how CDMA distinguishes users on one frequency, and derive the near-far problem
from the mechanism.

**C5.** Explain cell breathing, and why coverage holes appearing only at peak times cannot be
fixed by adjusting the cell.

**C6.** "The jump from circuits to packets mattered more than the jump from analogue to
digital." Defend or refute.

**C7.** Explain why 3G disappointed commercially despite being technically correct and on
time. Identify the general lesson and one other instance in this book.

**C8.** Explain why LTE removed the Radio Network Controller and what it bought.

**C9.** Explain frequency-selective scheduling and multi-user diversity, and why they make
fading a source of gain rather than a problem.

**C10.** Explain why LTE's uplink uses SC-FDMA rather than OFDMA, and whose interest that
serves.

**C11.** Explain why network slicing's guarantees are in tension with statistical multiplexing,
and connect it to Chapter 13.

**C12.** "Air latency is not end-to-end latency." Explain, and state what 5G's latency claim
actually requires.

## D. Design

**D1.** A container port of 2 km² needs connectivity for cranes, vehicles and handheld
devices, with deterministic control traffic. Evaluate Wi-Fi, private 5G and a hybrid. Make a
recommendation with reasoning.

**D2.** An organisation's 3,000 alarm panels use 2G, which is being switched off in eighteen
months. Write the migration assessment: options, costs, risks and a recommendation.

**D3.** Write the evaluation criteria you would apply to any vendor's 5G proposal, based on
§46.4's distinctions.

**D4.** For the semester project's site, assess whether cellular has any role — as backup
WAN, for IoT, or otherwise — and justify.

## E. Troubleshoot

**E1.** Users in one area lose 3G service only at peak times. Diagnose.

**E2.** A phone shows 5G and achieves 80 Mb/s. Is anything wrong?

**E3.** A user's data connection drops to 3G speeds during every voice call. Diagnose and
give the fix.

**E4.** A lift telephone stopped working after a network upgrade by the carrier. Diagnose.

**E5.** A 5G fixed-wireless installation worked at commissioning and degraded when trees came
into leaf. Explain, referring to Chapter 42.

**E6.** An LTE connection shows excellent signal bars and poor throughput. What do you measure
and what does it distinguish?

**E7.** A private 5G proposal claims 1 ms latency for a factory application. What three
questions do you ask?

## F. Extend

**F1.** On an Android phone, use a field-test app to record RSRP, RSRQ and SINR while moving.
Relate them to Chapter 42 §42.1's RSSI and SNR.

**F2.** Determine which bands your carrier uses in your area and whether your connection is
SA or NSA. Explain how you found out.

**F3.** Measure LTE and 5G throughput and latency at the same location, several times a day.
Explain the variation.

**F4.** Research the 2G/3G shutdown timetable in your country and identify one category of
device that will be affected.

**F5.** Read the CBRS spectrum-sharing model and compare it with Chapter 43 §43.1's AFC.
Identify what both have in common and what is genuinely new.

**F6.** Compare a private 5G quotation with a Wi-Fi 6E quotation for the same industrial site,
if you can obtain them. Identify where the cost difference lies.
