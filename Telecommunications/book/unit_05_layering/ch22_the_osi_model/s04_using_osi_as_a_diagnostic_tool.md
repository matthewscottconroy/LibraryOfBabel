# 22.4 Using OSI as a Diagnostic Tool

This is the section that justifies the chapter. Everything before it was vocabulary;
this is a method, and it is the most effective general troubleshooting technique in
this book.

## The problem with "the network is down"

An unbounded problem. The cause could be a cable, a duplex setting, a spanning-tree
recalculation, a subnet mask, a routing change, a firewall rule, a DNS record, an
expired certificate, an application defect, or a user typing the wrong password.

Without a method, people check what they last saw break, or what they most enjoy
checking, or what they most recently read about. This produces long outages with an
audience.

**The model converts one unbounded problem into seven bounded ones, in a fixed
order.** That is its entire diagnostic value, and it is enormous.

## The bottom-up method

**Start at Layer 1 and work up. Do not skip.**

### Layer 1 — Is it physically connected?

| Check | How |
|---|---|
| Cable seated at both ends | look |
| Link light | look |
| Interface up | `ip link` / `show interface` |
| Correct cable type and category | look |
| Within distance limits | measure |
| Transceiver seated and correct type | `show interface transceiver` |
| Error counters | `ip -s link` / `show interface` |

**Layer 1 faults are the most common and the cheapest to check.** Check them first even
when you are certain the problem is elsewhere, because the check costs thirty seconds
and being wrong about it costs an hour.

*If Layer 1 fails:* replace the cable, reseat the transceiver, check the distance.
Stop; do not proceed until it passes.

### Layer 2 — Can it reach the local segment?

| Check | How |
|---|---|
| MAC address present | `ip link` |
| Switch has learned it | `show mac address-table` |
| Correct VLAN | `show interfaces switchport` |
| Port forwarding, not blocking | `show spanning-tree` |
| Duplex matches | `show interface` both ends |
| ARP resolving | `ip neigh` / `arp -a` |
| Reachable at Layer 2 | **`arping`** |

**`arping` is the key tool here** (Chapter 18 §18.3). It tests reachability with no IP,
no ICMP and nothing a firewall usually filters. If `arping` succeeds and `ping` fails,
**you have proved the problem is above Layer 2** — which eliminates cabling, switching,
VLANs and spanning tree in one command.

*If Layer 2 fails:* VLAN, spanning tree, duplex, or a switch problem.

### Layer 3 — Can it reach other networks?

| Check | How |
|---|---|
| IP address correct | `ip addr` |
| **Subnet mask correct** | `ip addr` |
| Default gateway correct | `ip route` |
| Gateway reachable | `ping <gateway>` |
| Beyond the gateway | `ping 8.8.8.8` |
| Path | `traceroute` / `mtr` |
| MTU | `ping -M do -s 1472` |

**Ping the gateway first.** It separates *"my segment is broken"* from *"the world
beyond is broken"*, and those have almost disjoint cause sets.

**Check the mask carefully.** Chapter 25 §25.3: a wrong mask produces *selective*
failure — some destinations work, others do not — which is the most confusing symptom
in this layer and the one people misattribute most often.

*If Layer 3 fails:* addressing, mask, gateway, routing, or MTU.

### Layer 4 — Can it reach the service?

| Check | How |
|---|---|
| Port open | `telnet host 443`, `nc -zv host 443` |
| What is listening | `ss -tlnp` / `netstat -an` |
| Connection state | `ss -tan` |
| Handshake completing | capture, look for SYN/SYN-ACK/ACK |

**The diagnostic that matters here:**

| Observation | Meaning |
|---|---|
| SYN sent, **RST received** | The host is up and **nothing is listening** on that port |
| SYN sent, **nothing at all** | A firewall is **silently dropping**, or the route is asymmetric |
| SYN, SYN-ACK, ACK, then nothing | Connected — the problem is at Layer 7 |

**A RST and a silent drop mean very different things**, and distinguishing them narrows
the search enormously. A RST is an answer; silence is a firewall.

*If Layer 4 fails:* the service is not running, or a firewall is blocking.

### Layers 5–7 — Is the application working?

| Check | How |
|---|---|
| Name resolution | `dig` / `nslookup` |
| The protocol itself | `curl -v` |
| Certificate validity | `openssl s_client -connect host:443` |
| Authentication | application logs |
| Server-side errors | application logs |

**Check DNS explicitly.** An enormous share of "the network is broken" is DNS, and
`ping 8.8.8.8` succeeding while `ping google.com` fails identifies it in two commands.
Chapter 39 §39.4 covers it properly.

**Check certificate expiry.** It is a scheduled outage that nobody scheduled, and the
symptom — everything worked yesterday, nothing changed — matches almost nothing else.

## Divide and conquer — the faster method

Bottom-up is thorough. When time matters, **start in the middle**.

**Test Layer 3 first.** `ping` the default gateway:

- **Succeeds** → Layers 1, 2 and 3 are working locally. Go **up**.
- **Fails** → the problem is at Layer 3 or below. Go **down**.

One test eliminates half the stack. Repeat within the remaining half.

$$\log_2 7 \approx 3 \text{ tests}$$

Three tests rather than seven, and the gain grows with the size of the space.

**Use bottom-up when** the problem is new, physical work happened recently, or you are
inexperienced with the system.
**Use divide-and-conquer when** you know the environment, time is short, or the
symptom already points somewhere.

## Top-down

Occasionally right: when the symptom is clearly application-specific.

*"Only this one application is broken; everything else works."* Starting at Layer 1
wastes time — the working applications already proved Layers 1–4. Start at Layer 7.

**What the working things prove is the most underused information in troubleshooting.**
If the user's email works, their DNS works, their routing works, their switch port
works, and their cable works. That eliminates most of the stack for free.

## Worked example

**Report:** *"I can't get to the intranet site."*

| Step | Test | Result | Inference |
|---|---|---|---|
| 1 | Link light, `ip link` | up | L1 fine |
| 2 | `ip addr` | 10.20.5.44/24 | plausible |
| 3 | `ping 10.20.5.1` (gateway) | success | **L1–L3 local fine** |
| 4 | `ping 8.8.8.8` | success | L3 routing fine |
| 5 | `ping intranet.corp` | *unknown host* | **DNS** |
| 6 | `dig intranet.corp` | SERVFAIL | DNS server or record |
| 7 | `dig @10.20.1.10 intranet.corp` | success | **the configured resolver is wrong** |
| 8 | `ip addr show` / DHCP options | resolver = 8.8.8.8 | **found it** |

The laptop is using a public resolver — set manually, or handed out by a misconfigured
DHCP scope — which cannot resolve internal names. Eight steps, each eliminating a
layer, and the fault is at Layer 7 in a network that is perfect at Layers 1–4.

**Note step 3.** One successful ping eliminated cabling, the switch port, the VLAN,
spanning tree, duplex, the IP address, the mask and the gateway. That is the method
working.

## Why the method is worth the model

The seven layers may be an arbitrary committee number and the upper three may not exist
as separate implementations. **It does not matter for this purpose.** What matters is:

- The layers are **ordered by dependency** — nothing above works if something below is
  broken
- Each layer has a **small, known set of faults**
- Each layer has **specific tools**
- A test at any layer **eliminates everything below it**

That last property is the whole thing. **A successful test at layer *n* proves layers 1
through *n* are functioning**, which is a large amount of information from one command.

Chapter 63 builds a full methodology on this foundation, with the documentation and
escalation practices that turn a technique into a discipline. The core is here.

## What breaks here

**Skipping Layer 1 because "it can't be the cable".** It is the cable more often than
anything else on this list.

**Not using what already works.** If anything on that host reaches the network, most of
the stack is proven.

**Confusing "no response" with "refused".** RST means listening-nothing; silence means
firewall. Different problems.

**Changing several things at once.** You will fix it and never know what was wrong,
which guarantees a recurrence.

**Not checking DNS early.** It is a large fraction of all reported network faults and
takes two commands to exclude.

> **Network+ note.** Objective 5.1 is the troubleshooting methodology and objective
> 5.5 the tools. **This section is directly examined.** Know bottom-up, top-down and
> divide-and-conquer by name, know when each applies, and know which tool belongs at
> which layer. Expect scenario questions of exactly the worked-example form.
