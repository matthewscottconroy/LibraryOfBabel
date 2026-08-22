# Chapter 43 — Important Concepts

Spectrum is governed by law, not protocol *(§43.1)* — It cannot be manufactured, everyone
in range shares it, and there is no technical mechanism preventing anyone from
transmitting. The failure mode is complete and mutual: neither party can fix it
unilaterally, and raising power escalates rather than resolves.

**Regulation followed disaster** *(§43.1)* — Ship-to-shore chaos and the Titanic in 1912
produced the Radio Act within months, and the pattern has repeated at every expansion of
radio use.

**Licensed versus unlicensed** *(§43.1)* — Licensed is exclusive, expensive and legally
protected — the 2021 US C-band auction raised \$81 billion. Unlicensed is free, open
to anyone, and has no interference protection whatsoever. A quality guarantee is
possible in one and impossible in the other.

**The ISM accident** *(§43.1)* — The bands were allocated to **noise sources** — microwave
ovens, industrial heaters, medical diathermy — and were considered worthless. In 1985 the
FCC permitted unlicensed spread-spectrum communication there, on condition that devices
tolerate interference and claim no protection.

> The 2.4 GHz band was given away because nobody wanted it, and became the most
> economically significant spectrum allocation in history.

And it succeeded because it required nothing of anyone — no licence, no coordination, no
permission. Chapter 28 §28.1's precondition for rapid adoption.

Unlicensed is not unregulated *(§43.1)* — **EIRP limits** (20 dBm in Europe at 2.4 GHz),
out-of-band emission masks, duty cycles, **DFS**, transmit power control, and listen-before-
talk. The EIRP limit is why "turn the power up" is often unavailable.

**DFS** *(§43.1)* — Much of 5 GHz is shared with radar, which has priority. Listen for
60 seconds before use, monitor continuously, vacate within 10 seconds on detection and
stay off for 30 minutes. The symptom is an unexplained mass disconnection, sometimes
from a false detection. And because DFS is inconvenient, deployments crowd onto the
non-DFS channels — an individually rational choice with a bad collective outcome.

**6 GHz** *(§43.1)* — 1,200 MHz in the US, 480 in Europe, against 2.4 GHz's entire
83.5 MHz. **Low Power Indoor** needs no DFS; Standard Power requires AFC, a database
service that tells a device which channels it may use where. AFC is a genuinely new
regulatory model — coordinated sharing mediated by a service, rather than exclusivity or
free-for-all.

A channel is a centre frequency plus a bandwidth *(§43.2)* — And the sides do not fall
vertically; a spectral mask governs the spill, which is why adjacent-channel interference
exists.

**The 1/6/11 derivation** *(§43.2)* — Channel numbers are 5 MHz apart and channels are
20 MHz wide, so 5 channel numbers gives 25 MHz of separation for a 20 MHz signal.
Channel 1 spans 2402–2422 and channel 6 spans 2427–2447 — a 5 MHz gap. Channels 1 and 5
**touch exactly**, with no margin for the mask's skirts.

Overlap is worse than sharing *(§43.2)* — Two APs on the same channel hear each other
and take turns. Two on overlapping channels cannot decode each other, so each sees the
other as raised noise, neither defers, and both transmit over each other continuously.
So 1, 3, 5, 7, 9, 11 performs substantially worse than 1, 6, 11 — the commonest wireless
configuration error, arising from the reasonable belief that using more of the band must be
better.

Channel width trades capacity for count *(§43.2)* — Every doubling of width **halves the
channels**, adds 3 dB of noise (so costs range), and quadruples the chance of
overlapping something at 4×.

Narrow channels are better in dense deployments *(§43.2)* — Forty APs with 20 MHz
channels get roughly one each; with 80 MHz they share six channels about seven ways.
Four times the theoretical rate and seven times the contention, and the wide-channel
deployment performs worse. **Counter-intuitive and correct.**

5 GHz channel numbers do not overlap *(§43.2)* — The crucial difference from 2.4 GHz:
20 MHz channels are spaced 20 MHz apart, so 36 and 40 are adjacent in numbering and separate
in frequency. Wider channels are formed by fixed bonding — 80 MHz starting at 36 occupies
36, 40, 44 and 48.

Avoiding DFS costs you most of the band *(§43.2)* — UNII-1 and UNII-3 alone give about 9
channels at 20 MHz, 4 at 40, and 2 at 80.

2.4 GHz's virtue is propagation and its problem is everything else *(§43.3)* — Three
channels, **heavy non-Wi-Fi interference**, and a **legacy burden** — one associated 802.11b
device forces protection mechanisms that slow every client on the radio.

**The microwave oven** *(§43.3)* — 2.45 GHz, high power, ~50% duty cycle with ~10 ms
on and off. Unmistakable on a waterfall display, and it explains "the wireless breaks every
day at lunchtime".

Disabling 2.4 GHz on most access points *(§43.3)* — Counter-intuitive and often correct:
fewer 2.4 GHz radios means the three channels go further. Leave it on a minority for
coverage and IoT.

5 GHz is the workhorse *(§43.3)* — ~25 channels, almost no non-Wi-Fi interference,
universal client support. Its costs are range and DFS complexity. And because so many
avoid DFS, a deployment willing to use it often finds a clean channel where its neighbours
have none.

6 GHz gives capacity and costs range *(§43.3)* — Seven 160 MHz channels in the US, no
legacy, no DFS indoors, essentially no interference. But ~8 dB more free-space loss than
2.4 GHz, worse penetration, lower LPI power, and recent clients only. A one-for-one
replacement of 5 GHz APs will leave 6 GHz coverage holes.

Band steering is delicate *(§43.3)* — Clients often prefer 2.4 GHz because it is
stronger, and stronger is not better when the band is congested. But steering a client
at the edge of 5 GHz coverage makes it worse — so steer only where 5 GHz coverage is
genuinely good, with an RSSI threshold.

Size the cells by the highest band you rely on *(§43.3)* — A tri-band deployment
designed for 5 GHz coverage will have 6 GHz holes.

The tragedy of the commons *(§43.3)* — Each network's rational choice is wide channels
and high power; if everyone does it, everyone is worse off, and there is no mechanism to
prevent it. Mitigated partially by CSMA/CA's politeness when devices can hear each other,
by power limits, by professional restraint with no enforcement behind it — and genuinely
only by more spectrum. The same structure as Chapter 32 §32.4 and Chapter 27 §27.2.

**The noise floor** *(§43.4)* — N = −174 + 10log(B) + NF. For 20 MHz with a 5 dB noise
figure, **−96 dBm.** Clean environments measure −95 to −100; −85 is worth investigating and
−80 is serious. Every 3 dB of elevation costs 3 dB of link budget for every client
simultaneously.

**Co-channel versus adjacent-channel** *(§43.4)* — **CCI is contention** — devices decode
each other and take turns, so the symptom is uniform slowness scaling with device count.
ACI and non-Wi-Fi interference is corruption — frames are damaged, so the symptom is
**retries and errors.** Different problems, different remedies.

The measurement that separates them *(§43.4)* — High utilisation with low retries is
contention. High retries with moderate utilisation is corruption. An elevated noise floor
with no Wi-Fi visible is a non-Wi-Fi emitter.

A Wi-Fi adapter cannot see what it cannot demodulate *(§43.4)* — "My survey shows the
channel is clear and nothing works" is the signature of a non-Wi-Fi interferer, and it
requires a spectrum analyser. Many enterprise access points have one built in and unused.

**The interferer signatures** *(§43.4)* — Microwave: wide, 50% duty. Video sender:
continuous, wideband, and the worst case — it cannot be negotiated with and must be found
and removed. Bluetooth: narrow, hopping, mild. Failing electrical equipment: broadband
noise with no structure, intermittent in a way that correlates with nothing — the one
people never suspect.

Utilisation degrades well before 100% *(§43.4)* — 70% is the practical ceiling,
because contention overhead rises sharply — the same shape as Chapter 16's ALOHA analysis.

Retries distinguish the two problems *(§43.4)* — High retries with good RSSI means
interference; high retries with poor RSSI means coverage. Identical from the user's side
and requiring opposite remedies.

What works, in order *(§43.4)* — Find and remove the source; change channel; **narrow the
channel**; more access points at lower power (which raises signal rather than lowering
noise, and is usually right); move band. What does not work: raising transmit power —
Chapter 42 §42.2's reciprocity, plus it escalates the commons problem.

**The diagnostic pivot** *(§43.4)* — Good RSSI with poor SNR can only mean an elevated
noise floor, which immediately excludes coverage, placement and power as causes.
