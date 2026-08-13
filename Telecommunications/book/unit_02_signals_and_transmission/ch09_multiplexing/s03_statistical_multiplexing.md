# 9.3 Statistical Multiplexing

This section contains the argument that decided the shape of the modern world's
communications infrastructure. It is not a physical technique — it does not divide
frequency, time, code or wavelength — and it is the reason packet switching
displaced circuit switching, which Chapter 13 tells as history and which is
computed here.

## The observation

Data traffic is **bursty**. A terminal session sends a burst of keystrokes and then
nothing for thirty seconds while the user reads. A web page loads in two seconds and
the browser sits idle for two minutes. A file transfer runs flat out for eleven
seconds and stops.

Voice traffic is not bursty in this way — a call occupies its channel continuously
— which is why the reserved-slot architecture of §9.2 suited it and why the
telephone engineers who built that architecture were not making a mistake for their
own workload.

But if a source is active only 5% of the time, then reserving capacity for it
wastes 95% of that capacity, permanently, whether the reservation is a frequency
band or a time slot.

## The arithmetic

Take a concrete population: **100 users**, each needing **1 Mb/s while active**,
each active **5% of the time**.

### Reserved allocation

Circuit switching, whether by FDM or synchronous TDM, must reserve capacity for
each active session. To serve all 100 simultaneously:

$$100 \times 1 \ \text{Mb/s} = 100 \ \text{Mb/s}$$

and the average utilisation of that 100 Mb/s is

$$100 \times 0.05 \times 1 = 5 \ \text{Mb/s}, \quad \text{i.e. } 5\%$$

Ninety-five per cent of the capacity is idle at any moment, and it cannot be lent
to anyone because it is reserved.

### Statistical allocation

Provision for the **aggregate** rather than for the sum of the peaks. Provision
20 Mb/s and ask: what is the probability that more than 20 users are simultaneously
active?

The number active follows a binomial distribution with *n* = 100 and *p* = 0.05,
so the mean is 5 and the standard deviation is

$$\sigma = \sqrt{np(1-p)} = \sqrt{100 \times 0.05 \times 0.95} = 2.18$$

Twenty active users is (20 − 5)/2.18 ≈ **6.9 standard deviations** above the mean.
Computing the binomial tail exactly:

$$P(X > 20) = \sum_{k=21}^{100} \binom{100}{k} 0.05^k 0.95^{100-k} \approx 2 \times 10^{-8}$$

**About one chance in fifty million** that demand exceeds the provisioned capacity
at any given moment.

### The gain

$$\text{multiplexing gain} = \frac{100 \ \text{Mb/s}}{20 \ \text{Mb/s}} = 5\times$$

**One fifth of the capacity, essentially the same service.** Verify it yourself:

```bash
python3 tools/simnet.py statmux --users 100 --rate 1 --activity 0.05 --link 20
```

## Why the gain grows with scale

The crucial property, and the one that makes this an architectural argument rather
than a local optimisation.

The standard deviation of the number of active users grows as √*n*, while the mean
grows as *n*. So the **relative** variability — the coefficient of variation —
falls as 1/√*n*:

| Users | Mean active | σ | σ/mean | Capacity for 6σ headroom | Gain |
|---|---|---|---|---|---|
| 10 | 0.5 | 0.69 | 138% | ~5 Mb/s | 2× |
| 100 | 5 | 2.18 | 44% | ~18 Mb/s | 5.5× |
| 1,000 | 50 | 6.9 | 14% | ~91 Mb/s | 11× |
| 10,000 | 500 | 21.8 | 4.4% | ~631 Mb/s | 16× |
| 100,000 | 5,000 | 68.9 | 1.4% | ~5,413 Mb/s | 18× |

**The larger the population, the more predictable the aggregate, and the less
headroom you need.** This is the law of large numbers doing engineering work, and
it is why aggregation is valuable at every level of a network: an ISP aggregating
ten thousand subscribers gets a better multiplexing gain than a building
aggregating fifty, and the gain compounds as traffic moves up the hierarchy.

It is also why the economics favour large operators, which is a structural fact
about the industry with consequences well beyond engineering.

## What is given up

This is the honest half, and treatments of statistical multiplexing frequently omit
it.

**No guarantee.** With reserved capacity, your allocation is yours. With
statistical multiplexing, if enough others transmit simultaneously, your packets
queue — and if the queue is full, they are discarded. The service you receive
depends on what everybody else is doing.

**Variable delay.** Reserved TDM delivers constant delay; a slot arrives every
125 µs, always. Statistical multiplexing delivers whatever the queue depth happens
to be, which varies with load and produces the jitter of Chapter 3 §3.3. Real-time
applications must then buffer, converting jitter into latency.

**No admission control.** The telephone network's busy signal is an honest refusal
delivered before you invest effort. A packet network accepts your traffic and
degrades everyone, including you. This is a genuine loss and Chapter 13 §13.4
argues it is the one the industry most regrets.

**Per-packet overhead.** A reserved slot needs no identifier — slot *i* is
conversation *i* by construction. A statistically multiplexed unit must carry a
label saying whose it is, and that label is the packet header. Chapter 3 §3.1
computed its cost: 5% for a large frame, 33% for a small one.

**The possibility of congestion collapse.** A reserved-capacity network cannot
collapse under load; it simply blocks new calls. A statistically multiplexed one
can, and did, in October 1986 (Chapter 38 §38.1). Preventing it required inventing
congestion control, which took two more years and is still an active research area.

## The tradeoff, stated fairly

| | Reserved (FDM, sync TDM) | Statistical |
|---|---|---|
| Efficiency with bursty traffic | Poor | **Excellent** |
| Efficiency with continuous traffic | **Good** | Comparable |
| Delay | **Constant** | Variable |
| Guarantee | **Yes** | No |
| Admission control | **Yes** | No |
| Per-unit overhead | **None** | Header |
| Failure mode under overload | Blocking | **Collapse** (without control) |
| Gain from aggregation | None | **Grows with scale** |

Neither column is simply better. The right choice depends on the traffic, and the
reason packet switching won is that the traffic changed — computer traffic is
overwhelmingly bursty, and the factor of five to eighteen was decisive.

## Where it appears

Once you can see it, statistical multiplexing is everywhere in this book:

- **Every packet network**, by definition.
- **Cable and PON access** (Chapter 49), where 100–500 homes share a segment. The
  "slow at 8 p.m." complaint is the gain being consumed by a correlated peak.
- **ISP peering and transit capacity**, provisioned against aggregate rather than
  the sum of subscriber line rates. An ISP with 10,000 subscribers on 100 Mb/s lines
  does not buy 1 Tb/s of transit.
- **Cloud compute**, which is statistical multiplexing of servers rather than
  bandwidth, with exactly the same mathematics.
- **Erlang's telephone trunk dimensioning** (Chapter 12 §12.4), which is the same
  calculation done seventy years earlier for a different resource. Erlang got there
  first, and the packet people rediscovered it.

## What breaks here

**Provisioning from the sum of line rates.** A link sized as the sum of what every
subscriber could theoretically pull is oversized by the multiplexing gain and
costs accordingly.

**Provisioning from the average.** The other error, and worse. The gain depends on
the peaks being *uncorrelated*, and traffic has strong daily and weekly correlation.
Size against the peak, with the ρ/(1−ρ) headroom of Chapter 3 §3.2.

**Assuming independence when it does not hold.** The binomial calculation assumes
users are independent. A software update pushed to everyone at 09:00, a live event,
or a school's timetable synchronising four hundred devices destroys the assumption
and with it the gain. This is the single most common way a capacity plan built on
this arithmetic fails.

**Forgetting that the gain has a price.** An organisation that saved 80% on
capacity and then complains about jitter has been paid for the tradeoff and is now
objecting to the other half of it.

> **Network+ note.** Not examined by name. Its consequences are, throughout the
> WAN and troubleshooting objectives: why a shared access medium performs
> differently at peak, why capacity planning targets peak rather than average, and
> why oversubscription ratios are a design parameter rather than a defect. All
> three are this section.
