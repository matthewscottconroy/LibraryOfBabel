# Chapter 46 — Important Concepts

**Ring's 1947 memorandum** *(§46.1)* — The cellular idea was complete thirty years before it
could be built, and what was missing was computation, not radio.

The pre-cellular constraint is arithmetic *(§46.1)* — One transmitter means one set of
frequencies used once across a city, so capacity is the channel count permanently. New
York in 1976: 545 subscribers, 3,700 waiting, and 20–30 minutes for a channel.

Frequency reuse converts fixed capacity into growable capacity *(§46.1)* — Because a
low-power signal falls away, the same frequency can be reused a few cells distant. And
capacity can be increased indefinitely by cell splitting — which is Chapter 45 §45.3's
argument, arrived at three decades earlier.

Hexagons are a modelling convenience *(§46.1)* — They tile without gaps and are the
closest regular tiling to a circle. Real coverage is irregular.

**The reuse trade** *(§46.1)* — N = i² + ij + j² gives 1, 3, 4, 7, 9, 12, 13, 19; and
**D/R = √(3N)**. Smaller N means more channels per cell and more co-channel interference;
N = 7 was the analogue compromise, and modern systems use N = 1 because CDMA and OFDMA
tolerate what analogue FM could not.

Handover is why it waited for computers *(§46.1)* — Measuring, deciding, allocating and
switching for thousands of calls at once, in under a few hundred milliseconds. There is
no number of human operators that can do it. A computing problem wearing a radio problem's
clothes.

Hard versus soft handover *(§46.1)* — Break-before-make versus make-before-break. Soft
requires both cells on the same frequency, so only CDMA permits it; LTE and 5G returned to
hard because it is now fast enough and soft consumes resources in two cells.

1G had no security because analogue offers nowhere to put any *(§46.2)* — Scanners
intercepted calls; **cloning was trivial** because identity was transmitted in the clear.
Encryption requires digital representation, which is an argument for 2G independent of
capacity.

2G's SIM separated subscriber from handset *(§46.2)* — **The underrated contribution.** It
is why Europeans changed phones by moving a card and Americans on CDMA could not — an
identity decision that shaped two continents' handset markets.

SMS was spare capacity in a control channel *(§46.2)* — 160 characters because that is
what fitted. The afterthought outlasted most of the system's designed purposes.

The transition that mattered was circuit to packet *(§46.2)* — Not analogue to digital.
GPRS carried IP for the first time, and packet switching lets a device be always-on while
consuming resources only when transmitting (Chapter 13).

CDMA: everyone on one frequency, separated by codes *(§46.2)* — Which gives reuse of 1
and no channel plan at all, and permits soft handover.

**The near-far problem** *(§46.2)* — A close handset swamps a distant one, so CDMA commands
every handset's power 1,500 times per second. Poorer battery life, and power-control
failure is collapse rather than degradation.

**Cell breathing** *(§46.2)* — In CDMA, capacity and coverage are the same resource — each
user raises everyone's noise floor, so a busy cell physically shrinks. Coverage holes
that appear only at peak times are the signature, and only added capacity fixes them.

3G's technology was right and its business case was wrong *(§46.2)* — European licences
raised **over €100 billion** on a prediction of video calling; the demand that justified it
arrived with the iPhone six years later. The same lesson as Chapter 43's ISM bands, inverted:
the allocation priced on a prediction failed, and the one that predicted nothing succeeded.

Voice stayed circuit-switched through 3G *(§46.2)* — Unifying it onto IP is LTE's
contribution.

LTE's significance is architectural *(§46.3)* — No circuit-switched domain at all.
Voice became an application over IP — Chapter 23 §23.4's end-to-end argument arriving in
telephony seventy years late — and operator resistance was substantial partly because
billing systems were built around minutes.

Control and user planes separated *(§46.3)* — MME signals, SGW/PGW carry. Chapter 29
§29.1 and Chapter 68's SDN argument, applied to a mobile core.

The eNodeB is intelligent *(§46.3)* — LTE removed the Radio Network Controller and put
scheduling and handover in the base station. Flatter architecture, and 30–50 ms latency
against 3G's 100–200.

**Frequency-selective scheduling** *(§46.3)* — The scheduler allocates resource blocks every
millisecond using channel reports, giving each user the subcarriers where their channel is
currently good. With enough users some user is always experiencing good conditions, so
multi-user diversity turns fading into a source of gain.

Central scheduling versus contention *(§46.3)* — LTE has no collisions and no backoff,
so its efficiency under load far exceeds Wi-Fi's. The cost is requiring owned spectrum and
authoritative scheduling — exactly what unlicensed operation cannot have.

SC-FDMA on the uplink *(§46.3)* — Lower peak-to-average power ratio means a more efficient
amplifier and better handset battery life. A design choice made for the device rather than
the network.

**CSFB versus VoLTE** *(§46.3)* — Fallback drops data to 3G during calls and requires the
legacy network to remain; VoLTE keeps full LTE, sets up in under two seconds, and delivers
wideband audio — the first improvement in telephone audio quality since the 1930s. VoLTE
is what permits 2G/3G shutdown.

**Carrier aggregation** *(§46.3)* — Combine carriers across bands: low band for coverage,
high bands for capacity, simultaneously. Wi-Fi 7's MLO arriving in cellular a decade
earlier.

5G is three systems sharing a name *(§46.4)* — eMBB is deployed; URLLC is barely; mMTC
is served by LTE-M and NB-IoT. Consumer 5G is eMBB and nothing else, and the other two
justified much of the investment.

The bands differ enormously *(§46.4)* — Low band is barely faster than LTE; mid band is
the useful 5G; mmWave is the marketing figure and covers a few hundred metres with almost
no penetration. The gigabit demonstrations were shot in line of sight.

**NR's real advances** *(§46.4)* — **Flexible numerology** (shorter symbols, lower latency,
mmWave tolerance); massive MIMO with per-user beamforming, which is the largest mid-band
capacity contributor; **beam management**, since a mmWave link is a steered pencil rather than
a coverage area; and better codes.

Most 5G is NSA *(§46.4)* — 5G radio on an LTE core: eMBB's speed, LTE's control-plane
latency, and no slicing or URLLC. Most capability claims describe SA and most deployments
are NSA, so asking which is the first question.

**Network slicing** *(§46.4)* — Logical networks with their own scheduling and guarantees on
one infrastructure. Requires SA and a virtualised core. And a slice with a hard latency
guarantee is a slice with capacity held idle — statistical multiplexing with per-tenant
guarantees, which is the oldest tension in this book.

**Private 5G** *(§46.4)* — Licensed or coordinated spectrum, **network-controlled handover**,
hundreds of metres per cell, and schedulable determinism — against Wi-Fi's low cost,
universal client support and available expertise. Right for ports, mines, refineries and
AGVs; wrong for an office, where it is nonetheless being sold.

CBRS extends the AFC idea *(§46.4)* — Three tiers with a **Spectrum Access System**
arbitrating in real time. Chapter 43 §43.1's database-mediated sharing, taken further.

Edge computing is required for the latency claim *(§46.4)* — 1 ms air latency is
meaningless if the server is 100 ms away. MEC places compute at base stations, and it makes
an operator a distributed cloud provider.

**The honest assessment** *(§46.4)* — 5G is a solid generational improvement sold as a
revolution. Assessing a proposal means asking which band, SA or NSA, and which service
class the claim depends on.
