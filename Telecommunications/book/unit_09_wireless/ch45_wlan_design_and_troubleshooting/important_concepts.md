# Chapter 45 — Important Concepts

**The four surveys** *(§45.1)* — **Predictive** models before building; **passive** walks and
listens, measuring coverage; **active** walks associated, measuring **whether it works**;
**spectrum** finds non-Wi-Fi interference. A competent project uses all of them, and
active is the one most often skipped.

Predictive surveys are as good as their inputs *(§45.1)* — Wall materials mislabelled by
one category is 6–10 dB of error, compounding through several walls. Excellent for
budgeting; not a substitute for measuring.

Survey with the worst common client *(§45.1)* — A survey adapter hears access points a
phone cannot. The measurement that matters is what your worst common device experiences,
which in most organisations is a phone or a handheld scanner.

Survey occupied, and while moving *(§45.1)* — Bodies absorb (Chapter 42 §42.1), and
fast fading means a stationary reading samples one point in a pattern varying over
centimetres (Chapter 42 §42.4). A survey of an empty warehouse before the racking arrives
is worthless.

−67 dBm is the target *(§45.1)* — Not arbitrary: it is roughly where a client sustains a
rate high enough that airtime consumption stays reasonable (Chapter 44 §44.2). Design
for the rate you want, not for the signal a client will tolerate — a client will associate
at −85 dBm and consume disproportionate airtime doing it.

Design at the shortest-range band *(§45.1)* — A design meeting −67 dBm at 2.4 GHz will
have 5 GHz holes, and a 5 GHz design will have 6 GHz holes (Chapter 43 §43.3).

**Placement principles** *(§45.1)* — Ceiling-mounted in open space; **not in corners** (half
the pattern wasted, and it leaks outside); not directly above the densest area (the
antenna null, Chapter 42 §42.2); away from metal and interferers; and within 100 m of a
comms room, which constrains placement more often than expected.

**Cell size levers** *(§45.1)* — **Reduce power** or raise the minimum basic rate. The
rate is the better lever, because it shrinks the cell without creating uplink asymmetry
(Chapter 42 §42.2).

**15–20% overlap** *(§45.1)* — Measured at the design threshold, not at the edge of
audibility. Too little means holes and failed roaming; too much means co-channel
contention and sticky clients.

The client decides when to roam *(§45.2)* — There is no mechanism by which an access
point can move a client. Every vendor's algorithm differs, tuned for battery as much as
performance, and Windows laptops are notoriously sticky.

**The sticky client** *(§45.2)* — Holding an access point at −85 dBm past three better ones,
running at a low MCS and **consuming airtime**. The network's options are indirect: reduce
power, raise the minimum rate, 802.11v suggestions, or deauthentication as a blunt last
resort. The real fix is cell design.

Where roaming time goes *(§45.2)* — **Scanning (10–500 ms)** and 802.1X
(100–1000 ms) dominate; the association exchange itself is ~4 ms. An unoptimised
enterprise roam takes long enough to drop a call, and voice needs under 150 ms.

The three amendments, distinguished *(§45.2)* — 802.11k gives neighbour reports, so a
client scans three channels instead of twenty-five. **802.11v suggests transitions** — and a
client may decline. 802.11r pre-distributes the key hierarchy, so a roam needs only the
four-way handshake. k addresses scanning, r addresses authentication, v addresses when.

A roam that changes subnet is a reconnection *(§45.2)* — Every connection breaks. Keep
the wireless VLAN continuous across the roaming domain — the simplest and best answer — or
tunnel to a controller. A design where a client changes subnet while walking is a design
error.

**The controller trajectory** *(§45.2)* — Autonomous → centralised → **local switching** →
**cloud-managed**. From configuring each device, to centralising everything, to centralising
only the control plane — which is Chapter 68's SDN argument, and wireless arrived there
first because RF is shared and channel decisions must be global.

The data path is the design question *(§45.2)* — Tunnelling everything to a controller
means the controller's capacity is the network's capacity, and traffic may cross the
network twice. Cloud management's advantage is the failure mode: access points keep
serving clients when the WAN fails, because only management is remote.

RRM optimises what it can measure *(§45.2)* — It sees other access points and the noise
floor; it does not see your users' experience. Verify what it chose — the commonest
finding is power set higher than a human would choose, because more coverage looks better
to an algorithm that does not measure contention.

Coverage and capacity pull opposite ways on every parameter *(§45.3)* — Cell size, power,
channel width, antenna type, minimum rate. A design that covers a lecture theatre with one
access point will fail the moment it is used.

Three compounding reasons more APs at lower power works *(§45.3)* — Fewer clients share
each access point; smaller cells mean better signal so higher MCS, so each transfer
occupies the medium for less time; and airtime fairness improves because the slow distant
clients are gone. **They compound** — doubling the access points more than doubles the
capacity.

Raising power in a dense deployment reliably makes it worse *(§45.3)* — Larger cells mean
more clients sharing, and more co-channel overlap so neighbouring cells contend more.
Power should be reduced, often to the minimum the coverage target permits.

Client count binds, not throughput *(§45.3)* — The single most important idea in
capacity design. A 350-seat theatre needs about 2 access points by throughput and 18 by
client count, and the client count wins — because CSMA/CA's contention overhead rises
with the number of contending stations and every client adds management traffic.

25–40 clients per radio *(§45.3)* — Comfortable below 25, significant contention at 50,
degraded above 70 regardless of bandwidth.

Voice is airtime, not bandwidth *(§45.3)* — 0.1 Mb/s and stringent latency. Capacity
planning for voice is about contention and roaming, not megabits.

The first question is scope *(§45.4)* — How many people, and where? One user one
device means the client; one area means coverage or interference there; everyone everywhere
means infrastructure. It eliminates most of the search space before any tool is used.

**The association ladder** *(§45.4)* — SSID visible → associates → authenticates → **gets an
address** → passes traffic. Step four is the most common failure and is not a wireless
problem — `169.254.x.x` says DHCP did not answer.

"Incorrect password" has five causes on enterprise *(§45.4)* — Wrong passphrase, RADIUS
timeout, expired certificate, wrong username, untrusted certificate. The client cannot
distinguish them and the RADIUS log can.

**The four measurements** *(§45.4)* — RSSI, SNR, retry rate, channel utilisation, and the
combination is the diagnosis:

| RSSI | SNR | Retries | Util | Diagnosis |
|---|---|---|---|---|
| poor | poor | high | low | **coverage** |
| **good** | **poor** | high | low | **interference** |
| good | good | **high** | low | **hidden nodes** |
| good | good | low | **high** | **capacity** |
| **good** | **good** | **low** | **low** | **not the wireless** |

The last row matters most *(§45.4)* — Everything measures well and the user complains.
The problem is upstream — WAN, DNS, server, application. Wireless is where complaints
arrive, not where they originate, and "does it happen on wired too?" settles it in one
test.

Dropping has four distinguishable causes *(§45.4)* — **Roaming** (correlates with
movement); **deauthentication frames** (capture and filter subtype 12); **power saving**
(appears dropped, works when used — normal); and **DFS** (everyone on one channel at once,
with a channel change in the log).

`netsh wlan show wlanreport` is underused *(§45.4)* — A graphical timeline of every
connection and disconnection with reasons over three days, answering "it keeps dropping"
with no capture at all.

A room worse than its neighbours is nearly always construction *(§45.4)* — Metal walls,
lift lobby, foil-backed insulation, or modern glazing (Chapter 42 §42.1).
