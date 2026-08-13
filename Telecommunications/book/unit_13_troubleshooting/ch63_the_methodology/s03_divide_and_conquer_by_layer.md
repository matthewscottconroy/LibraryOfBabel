# 63.3 Divide and Conquer by Layer

**The seven steps say what to do. This section says where to look**, and the answer is a
structured search rather than an inspired one.

## Four approaches

| Approach | Start at | Best when |
|---|---|---|
| **Bottom-up** | **the physical layer** | **a hard failure — nothing works** |
| **Top-down** | **the application** | **one application affected, others fine** |
| **Divide and conquer** | **the middle** | **you have no strong prior** |
| **Follow the path** | **the source, moving towards the destination** | **you know the path** |

**And the choice is not arbitrary. It follows from what §63.1's question 2 established.**

### Bottom-up

**Start at Layer 1 and work upward.** **Link, then frames, then addressing, then routing, then
transport, then application.**

**Correct when the failure is total.** **A device with no connectivity at all is far more likely
to have a physical or Layer 2 problem than an application one**, and **checking the link light
takes four seconds while checking the application takes ten minutes.**

**Its cost is that it is slow when the fault is high.** **Working up six layers to find a
certificate problem wastes the first five checks.**

### Top-down

**Start at the application and work down.**

**Correct when one application fails and others work**, because **that fact has already
eliminated the lower layers** — if the network were broken, everything would be broken.

> **"Other applications work" is a Layer 1 to 4 test that has already been performed for you.**
> **Do not repeat it.**

### Divide and conquer

**Start in the middle — Layer 3 — and eliminate half the stack with each test.**

```
   Test: can this host ping its default gateway by IP?

   YES → Layers 1, 2 and 3 (locally) are working. Look upward and outward.
   NO  → Layers 1, 2 or 3 are broken. Look downward.

   Each subsequent test halves what remains.
```

**The arithmetic is the argument:**

| Elements to search | **Linear, worst case** | **Bisection** |
|---|---|---|
| 7 | 7 tests | **3** |
| 15 | 15 | **4** |
| **31** | **31** | **5** |
| 63 | 63 | **6** |

> **Bisection is logarithmic and linear search is not**, and **on a path of any length the
> difference is the whole afternoon.**

**And it applies to more than layers:** **bisect the path (which hop?), bisect the population
(which users?), bisect time (when did it start?), and bisect the change set (which of the six
changes?).** **The technique is the same and the medium differs.**

### Follow the path

**Trace the packet from source to destination, checking each element in turn.**

**Correct when you know the path and it is short**, and **it is the approach that best matches
how a network engineer actually thinks** — which makes it the default and sometimes the wrong
default, because **a long path checked linearly is §63.3's linear column.**

**Its real value is different:** **it forces you to enumerate the path**, and **the enumeration
frequently reveals the fault before any test is run** — an element nobody remembered, a device
that should not be there, an asymmetry between the two directions.

## The bisection questions that eliminate most

**Six tests, each of which halves the search space, in the order that costs least.**

| | Test | Eliminates if it passes |
|---|---|---|
| **1** | **Does it work from another device on the same segment?** | **the network; it is the client** |
| **2** | **Does it work from another segment?** | **the segment; it is that VLAN or its policy** |
| **3** | **Does it work to another destination?** | **the path; it is the destination** |
| **4** | **Does it work by IP address rather than by name?** | **everything below; it is DNS** |
| **5** | **Does it work over a different protocol or port?** | **reachability; it is policy or the service** |
| **6** | **Did it work before a known change?** | **everything not in the change** |

**Test 4 deserves its own note, because it is the highest-yield single test in networking:**

> **`ping 10.9.0.5` succeeds and `ping app.example.com` fails.** **You have just eliminated
> Layers 1 through 4 and identified the layer in one command**, and it takes two seconds.

**Test 1 is the second highest-yield and is frequently skipped** because it requires finding
another device. **It is worth the walk.**

## The layered checklist

**What "check this layer" actually means at each level, and it is Chapter 65's material in
outline.**

| Layer | The question | The command |
|---|---|---|
| **1 Physical** | **Is there a link, and is it clean?** | `show interface` — status, errors, CRC |
| **2 Data link** | **Right VLAN? Right MAC learned? STP forwarding?** | `show mac address-table`, `show spanning-tree`, `show vlan` |
| **3 Network** | **Right address, mask, gateway? Route present? ARP resolved?** | `ip addr`, `ip route`, `arp -a`, `ping <gateway>` |
| **4 Transport** | **Is the port reachable? Is anything listening?** | `ss -tlnp`, `nc -zv`, and the firewall's counters |
| **5–7 Application** | **DNS? Certificate? Authentication? The service itself?** | `dig`, `openssl s_client`, the application's own logs |

**And two cross-layer checks that belong in every investigation:**

**Is time correct?** (Chapter 54 §54.3.) **Clock skew breaks Kerberos, certificate validation
and log correlation**, and it presents as an authentication problem rather than as a time
problem.

**Is it symmetric?** **Test both directions.** **A great many faults — asymmetric routing, a
one-way ACL, a duplex mismatch, a one-way fibre — look like a general failure and are
directional**, and **testing only outbound misses half of them.**

## Where the layer model helps and where it misleads

**Honesty, because §63.3's whole method rests on it.**

**Where it helps:** **it provides a search order that is exhaustive and non-overlapping**, which
is exactly what an unstructured search lacks. **And it maps onto the tooling** — each layer has
its commands.

**Where it misleads** (Chapter 21 §21.4, Chapter 22 §22.3):

**Middleboxes span layers.** **A firewall making decisions on Layer 7 content while forwarding
at Layer 3 does not sit at a layer**, and **a fault in it presents at whichever layer it chose to
act on.**

**Encapsulation makes "which layer" ambiguous.** **A tunnel's outer Layer 3 and inner Layer 3
are both Layer 3** (Chapter 61 §61.1), **and an MTU fault in the outer presents as an
application fault in the inner.**

**And some faults are not at a layer at all.** **A capacity problem, a licence expiry, an
authentication backend, a DNS record pointing at a decommissioned host** — **the model has no
place for them**, and forcing them into it delays the diagnosis.

> **The layer model is a search strategy, not a theory of the network.** **Use it to structure
> the search and abandon it the moment the evidence points somewhere it does not describe.**

## Working an intermittent fault

**The case where none of the above works directly**, and it deserves its own method.

**The problem: you cannot test a fault that is not currently happening.**

**So the method inverts — instrument first, then wait:**

| | |
|---|---|
| **1** | **Establish what "it happened" means observably** — an error, a counter, a log line, a probe failure |
| **2** | **Instrument every candidate point** — counters polled at short intervals, continuous captures with a ring buffer, synthetic probes |
| **3** | **Wait for an occurrence** |
| **4** | **Correlate** across the instrumented points, by time (Chapter 54 §54.3's NTP requirement) |
| **5** | **Only then** form a theory |

**Two techniques that make step 2 practical:**

**Ring-buffer capture.** **`tcpdump -W 20 -C 100 -w cap.pcap`** — twenty files of 100 MB,
rotating. **Run it for days; when the fault occurs, the preceding minutes are on disk.**

**And a trigger.** **A script that watches for the symptom and stops the capture, or fires a
`show tech-support`, or timestamps a marker** — **so you know which of the twenty files to
read.**

> **The commonest failure with intermittent faults is that when it finally happens, nobody was
> capturing.** **The instrumentation must be running before, unattended, for as long as it
> takes.**

## What breaks here

**Six layers checked to find a certificate error.** **Bottom-up on a high-layer fault.** Use the
scope to choose the approach.

**An hour spent on the network when other applications worked throughout.** **Top-down was
indicated** and the lower layers had already been tested for you.

**A long path checked hop by hop.** **Linear where bisection was available.**

**A fault that only occurs outbound, tested only outbound.** **Test both directions.**

**Authentication failures diagnosed as an authentication problem.** **Check the clock.**

**An intermittent fault, and no capture running when it occurred.** **Instrument first.** The
instrumentation must outlast your attention.

**The layer model applied to a licence expiry.** **It has no place for it.** Abandon the model
when the evidence leaves it.

> **Network+ note.** Objective 5.1 names the approaches. Over-learn: **top-down, bottom-up and
> divide-and-conquer are the recognised methods**; **bottom-up starts at the physical layer**;
> **divide-and-conquer starts in the middle**; and **the choice depends on the symptom.** The
> approach-to-symptom mapping is examined, and **"does it work by IP but not by name" is the
> single test worth carrying into practice.**
