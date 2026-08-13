# 54.1 Baselines and What "Normal" Means

**"Is the network slow?" is not a question you can answer.** Not for lack of tools — **because
the question is comparative and you have nothing to compare against.**

## The corollary that costs people

> **A baseline must be collected before you need it**, during the period when everything is
> fine and nobody is asking. **The moment you need one is the moment it is too late to start.**

**Which means baselining is an investment made against a future incident**, by people who
cannot demonstrate its value until the incident happens. **That is a hard thing to fund and it
is why so few organisations have one.**

**The observable consequence of not having one:**

| Question | Without a baseline | With one |
|---|---|---|
| "Is the link busy?" | **"It's at 60%."** | **"It's at 60%; it's normally 25% at this hour."** |
| "Has this got worse?" | **impressions** | **a graph** |
| "Do we need more bandwidth?" | **a guess** | **a trend and a date** |
| "Did the change break it?" | **argument** | **before and after** |

## What a baseline is

**Not a single number. A distribution, over time, with enough granularity to show the shape of
a day and a week.**

| Measure | Per | Why |
|---|---|---|
| **Interface utilisation, in and out** | link | **the primary capacity signal** |
| **Error and discard counters** | interface | **discards are congestion; errors are physical** |
| **Latency and jitter** | reference destinations | **what users experience** |
| **Packet loss** | reference destinations | |
| CPU and memory | device | **control-plane health** |
| **Wireless channel utilisation and client counts** | AP/radio | Chapter 45 §45.4 |
| **Application response time** | service | **the closest thing to the user's experience** |
| DHCP pool utilisation | scope | **runs out silently** |
| Certificate expiry dates | service | **the most predictable outage there is** |

**And the value is in the shape, not the figures.**

```
   Utilisation, one week
   
   100% ┤
    75% ┤    ╭╮      ╭╮      ╭╮      ╭╮      ╭╮
    50% ┤   ╭╯╰╮    ╭╯╰╮    ╭╯╰╮    ╭╯╰╮    ╭╯╰╮
    25% ┤ ╭─╯  ╰──╮╭╯  ╰──╮╭╯  ╰──╮╭╯  ╰──╮╭╯  ╰──╮      ╭─╮
     0% ┤─╯       ╰╯      ╰╯      ╰╯      ╰╯      ╰──────╯ ╰──
        └──────────────────────────────────────────────────────
          Mon    Tue     Wed     Thu     Fri      Sat    Sun
                                                  ▲
                                          the weekend backup —
                                          and you need to know
                                          it is supposed to be there
```

> **A link at 60% at 14:00 on a Tuesday means nothing until you know it is normally at 25% at
> 14:00 on a Tuesday.** **Deviation from the expected shape is the signal; absolute thresholds
> are a crude proxy for it.**

## Averaging interval

**The detail that separates a useful baseline from a decorative one**, and it is routinely got
wrong.

**Consider a link averaging 40% over five minutes.** **That figure is consistent with:**

- **a steady 40%** — entirely healthy
- **100% for 20 seconds of every minute, idle the rest** — **queues filling and draining,
  users complaining, and nothing on the graph**

```
   What the 5-min average shows:      What is actually happening:

   100 ┤                              100 ┤██   ██   ██   ██
    50 ┤────────────────               50 ┤██   ██   ██   ██
     0 ┤                                0 ┤██▁▁▁██▁▁▁██▁▁▁██
       └────────────────                  └────────────────
       "40%, fine"                        33% duty cycle at line rate
```

**Those twenty seconds are where the user's video call lives** (Chapter 3 §3.2's queueing
curve, Chapter 52 §52.1's queue arithmetic).

**Five-minute averages hide microbursts completely**, and **microbursts are the commonest cause
of "the graph looks fine and it isn't."**

| Interval | Sees | Cost |
|---|---|---|
| **5 minutes** | trends, capacity planning | **cheap; the default; blind to bursts** |
| **1 minute** | most congestion events | moderate |
| **10 seconds** | **most microbursts** | **significant polling load** |
| **1 second and below** | **everything** | **streaming telemetry only** (§54.4) |

**The practical answer is not to sample everything fast.** **It is to sample fast where it
matters** — WAN edges, uplinks, anything carrying real-time traffic — **and every five minutes
elsewhere.**

**And where you cannot sample fast, use what the device already counts.** **Output discards and
output queue drops are burst detectors** — a link averaging 40% with rising discards **is
bursting**, and the counter says so without any change in polling rate.

> **Discards on an interface that is not congested "on average" is the signature of a
> microburst**, and it is one of the most useful single observations in operational networking.

## Percentiles beat averages

**The second detail, and it applies to latency far more than to utilisation.**

| | |
|---|---|
| **Mean latency: 22 ms** | tells you almost nothing |
| **p50: 18 ms** | typical |
| **p95: 61 ms** | **what your unluckier users get** |
| **p99: 340 ms** | **where the complaints come from** |
| **max: 2,100 ms** | **one event, and worth investigating** |

> **Almost every complaint is generated by the tail.** A mean that has not moved is entirely
> consistent with a p99 that has tripled, **and the p99 is the number a user notices.**

**Record percentiles, not averages, for anything a person waits for.** **And be aware that
percentiles do not average** — **you cannot compute the p95 of a day from the p95s of its
hours**, which is a real and common error in reporting.

## Seasonality

**A baseline has more than one period, and confusing them produces false alarms.**

| Cycle | Example |
|---|---|
| **Daily** | working hours, the overnight backup window |
| **Weekly** | weekday against weekend |
| **Monthly** | **month-end reporting, payroll, invoicing** |
| **Annual** | **the retail peak, the academic year, the tax deadline** |
| **Event-driven** | **a product launch, a results announcement, a school term start** |

**A monitoring system that alerts on "utilisation is unusually high" without knowing about
month-end will alert every month**, and **within three months nobody will read it** (§54.4's
alert fatigue).

**And the annual cycle is the one that catches people**, because **you need a year of data to
see it** — which is another argument for starting the baseline long before you need it.

## Using a baseline

**Three distinct uses, and they want different things from it.**

**Detecting anomalies.** **Compare now against the same time last week.** Robust to daily and
weekly seasonality, and it needs only a week of history.

**Capacity planning.** **Fit a trend to the busy-hour figure over months, and project.**
Chapter 56 uses this. **The useful output is a date** — "the Manchester circuit reaches 80% in
about November" — **not a percentage.**

**Validating a change.** **Compare the hour after against the same hour on previous days.**
This is the most under-used application, and it is nearly free: **a change window that includes
"check these five graphs against last week" catches a large fraction of changes that were
technically successful and operationally wrong.**

## What breaks here

**"The network is slow" with no baseline.** **The conversation cannot progress.** Start
collecting; it will be useful in a month and essential in a year.

**A graph showing 40% and users complaining.** **Microbursts.** Check discards; sample faster if
you can.

**An alert on absolute utilisation firing every month-end.** **Seasonality not modelled.**
Compare against the same period, or suppress the known window.

**The mean is unchanged and complaints have risen.** **Look at p95 and p99.** The mean is not
where user experience lives.

**A p95 computed by averaging hourly p95s.** **Arithmetically wrong.** Percentiles do not
compose that way; compute from the underlying samples.

**A baseline that shows nothing unusual during a known incident.** **The polling interval is
longer than the event**, or the measurement is in the wrong place — **frequently it measures the
device rather than the path the user takes.**

**Historical data lost in a retention policy.** **Capacity trends need months and seasonality
needs a year.** **Downsample rather than delete** — five-minute data for a month, hourly for a
year, daily forever, is a defensible policy and costs very little.

> **Network+ note.** Objective 3.1 and 5.1 cover baselines. Over-learn: **a baseline documents
> normal performance for later comparison**; **it must be captured before problems occur**;
> **network performance metrics include bandwidth, latency, jitter and loss**; and **anomalies
> are deviations from the baseline.** The "collect it before you need it" point is the one that
> matters in practice.
