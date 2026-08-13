# Lab 11 — Documentation, Monitoring, and Baselines

**Corresponds to:** Chapters 53, 54
**Week:** 11
**Time:** 120 minutes

---

## Objectives

- Produce the three standard diagrams for a network you did not design.
- Collect a baseline and demonstrate why the averaging interval matters.
- Configure SNMP and syslog collection, and read what each provides.
- Capture flow data and answer a question that counters cannot.
- Design an alert that would actually be acted on, and identify one that would not.
- Write a runbook that passes the 03:00 test.

---

## You will need

- The lab network from weeks 3–7, still assembled, or any multi-device topology.
- An SNMP-capable device and `snmpwalk`/`snmpget` (`net-snmp` package).
- A syslog collector — `rsyslog` on a Linux host is sufficient.
- `nfcapd`/`nfdump` or `softflowd`, or a router that exports NetFlow, for Part 4.
  If none is available, `iftop` and `ss` substitute partially and the debrief
  asks you to say what you lost.
- Drawing tools: any, including paper.

---

## Procedure

### Part 1 — Document a network you did not design

**1.** Swap topologies with another team. Do **not** ask them how it is built.

**2.** Discover it. Use only what is on the network:

```bash
ip neigh                            # who is on my segment
arp -a
lldpctl                             # if LLDP is running
sudo nmap -sn 10.0.0.0/24           # your own lab range only
traceroute <far-host>
```

On managed switches:

```
show cdp neighbors detail
show lldp neighbors detail
show mac address-table
show vlan brief
show interfaces status
```

**3.** Produce the **three diagrams** from Chapter 53 §53.1:

- **L1 physical:** devices, ports, cable types, patch positions. No IP addresses.
- **L2 logical:** VLANs, trunks, aggregation, spanning tree root and blocked
  ports.
- **L3 routed:** subnets, gateways, routing, the path off-network.

Each on one page. Each with a version number and today's date.

**4.** Produce an inventory table: device, model, role, management address,
firmware version, and — the field that matters — **end-of-support date** if you
can determine it.

**5.** Hand your diagrams to the team whose network it is. Record every error they
find, and how long the discovery took you. Both numbers go in the debrief.

---

### Part 2 — Baselines and the averaging trap

**6.** Set up continuous measurement between two hosts for at least thirty
minutes:

```bash
ping -i 1 <host> | ts >> /tmp/latency.log     # 'ts' from moreutils
```

and sample interface counters every 5 seconds:

```bash
while true; do
  date +%s >> /tmp/ifcounters.log
  cat /sys/class/net/<iface>/statistics/rx_bytes >> /tmp/ifcounters.log
  sleep 5
done
```

**7.** During the collection, generate **bursty** traffic: 5 seconds of `iperf3`
at line rate, then 55 seconds idle, repeatedly.

**8.** Compute from your samples: the 5-second peak utilisation, the 1-minute
average, and the 5-minute average.

**9.** Tabulate all three. Record the ratio between the 5-second peak and the
5-minute average.

**10.** Plot latency over the same period and mark where the bursts were. Record
what happened to latency during them.

**11.** Compute the 50th, 95th and 99th percentiles of your latency samples, and
compare with the mean.

---

### Part 3 — SNMP and syslog

**12.** Configure SNMPv2c on a device (lab only — Part 5 asks you why this is
indefensible in production), then walk it:

```bash
snmpwalk -v2c -c <community> <device> system
snmpwalk -v2c -c <community> <device> ifDescr
snmpget -v2c -c <community> <device> ifHCInOctets.<ifIndex>
```

**13.** Poll `ifHCInOctets` twice, sixty seconds apart. Compute the rate.

**14.** Now poll the **32-bit** counter `ifInOctets` instead, during heavy
traffic, twice thirty seconds apart. Compare with the 64-bit result.

**15.** Configure the device to send syslog to your collector, and the collector
to receive it:

```
# on the device
logging host <collector>
logging trap informational
```

```
# rsyslog on the collector
module(load="imudp")
input(type="imudp" port="514")
```

**16.** Generate events — shut and no-shut an interface, fail a login, change a
configuration — and record what arrives, at what severity, with what timestamp.

**17.** Compare the collector's timestamp with the device's. If they differ,
record by how much and explain the consequence for correlation.

---

### Part 4 — Flow data

**18.** Enable flow export, or run `softflowd` on a Linux router:

```bash
sudo softflowd -i <iface> -n <collector>:9995
```

and collect:

```bash
nfcapd -w -D -l /tmp/flows -p 9995
```

**19.** Generate a mix of traffic: a large file transfer, several small web
requests, some DNS, and a continuous ping.

**20.** Analyse:

```bash
nfdump -R /tmp/flows -s srcip/bytes -n 10
nfdump -R /tmp/flows -s port/bytes -n 10
nfdump -R /tmp/flows 'proto tcp and dst port 443'
```

**21.** Answer, using flow data alone: which host consumed the most bytes, to
which destination, on which port, and during which minute.

**22.** Now try to answer the same question using **only** interface counters.
Record what you can and cannot determine.

---

### Part 5 — Alerts and runbooks

**23.** Write **five alerts** for your lab network. For each, specify:

| Field | Content |
|---|---|
| Name | |
| Trigger condition | including threshold and duration |
| Severity | |
| **Action the technician takes** | specific commands |
| Why this will not fire spuriously | |

**24.** Now write **two alerts you have decided NOT to create**, with the reason.
Chapter 54's test: if the answer to "what do I do about this" is "nothing, it
clears itself", it is a graph and not an alert.

**25.** Write **one runbook** for a failure you have actually caused in an earlier
lab — the duplex mismatch, the PMTUD black hole, the rogue DHCP server, or the
VLAN mismatch. It must contain: the symptom as reported, the command that
confirms, expected healthy output, the three most likely causes in order, the fix,
the expected disruption, and who to escalate to.

**26.** Give the runbook to someone from another team who did not cause that
fault. Have them execute it against a network you have broken accordingly. Record
where they got stuck.

---

## Expected observations

- **Step 5:** discovery takes far longer than you expect, and the other team finds
  errors — typically in the L1 diagram, because cable routes are the hardest thing
  to discover from the network side.
- **Step 9: the 5-second peak is several times the 5-minute average.** A link
  averaging 40% may be at 100% for a fifth of the time, and the 5-minute average
  hides it completely.
- **Step 10: latency rises during the bursts**, which is the queueing delay the
  average conceals.
- **Step 11: the 95th percentile is substantially above the mean.** Almost every
  user complaint comes from the tail.
- **Step 14: the 32-bit counter may have wrapped**, producing a nonsensical or
  negative rate. On a fast interface it wraps in seconds.
- **Step 17: the clocks probably differ**, and if they differ by more than a few
  seconds, correlating events between devices becomes guesswork.
- **Step 22: interface counters cannot answer the question.** They tell you a link
  is at 90%; they cannot tell you what is on it.

---

## Break it

**A.** Change the SNMP community string on a device and leave the monitoring
system pointing at the old one. Record how long before anyone notices, and what
the monitoring system displays in the meantime — silence, or stale data
presented as current.

**B.** Stop NTP on one device and skew its clock by ten minutes. Generate
correlated events on two devices and try to reconstruct the sequence from the
logs.

**C.** Fill a device's log at debug level during traffic and record the effect on
its CPU and on the collector's storage rate.

---

## Debrief

**1.** How long did discovery take, and how many errors did the owning team find
in your diagrams? Which diagram had the most errors, and why is that the hardest
one to produce from the network side? State what documentation would have made
your task ten minutes instead.

**2.** Present your three utilisation figures — 5-second peak, 1-minute, 5-minute.
State the ratio. Then explain, referring to Chapter 3 §3.2's queueing curve, why a
link at 40% by the five-minute average can produce user complaints.

**3.** Give your mean latency and your 95th percentile. Explain which one you would
put on a dashboard for management and which you would alert on, and why they are
different answers.

**4.** In step 14 the 32-bit counter misbehaved. Explain the mechanism, compute how
long a 32-bit octet counter takes to wrap on a 10 Gb/s interface at full rate, and
state what a monitoring system that used it would display.

**5.** State the question you answered with flow data in step 21, and state
precisely what interface counters could and could not tell you about it. Then name
one privacy consideration that flow collection raises and how you would govern it.

**6.** Present two of your five alerts and both of your rejected alerts. For each
rejected one, explain what it *looked* like it was telling you and why acting on it
would have been wasted effort. Then explain what alert fatigue does to an
organisation's ability to detect a real incident.

**7.** Your runbook was executed by someone unfamiliar with the fault. Where did
they get stuck? Rewrite that step. Then state the general principle about who
writes runbooks and why they are usually written badly.
