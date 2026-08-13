# Lab 10 — WAN Behaviour, Latency, and Tunnels

**Corresponds to:** Chapters 3, 49, 51, 61
**Week:** 10
**Time:** 120 minutes

---

## Objectives

- Simulate WAN conditions and measure their effect on real protocols.
- Demonstrate the bandwidth–delay product empirically, and fix a
  window-limited transfer.
- Show that a chatty protocol is a latency problem wearing a bandwidth costume.
- Build an encrypted tunnel and measure its overhead and MTU consequence.
- Read a traceroute across a real long-haul path and identify the geography.

---

## You will need

- Two Linux hosts (virtual machines are ideal — you will be impairing the
  network deliberately).
- `tc` with `netem`, `iperf3`, `curl`, `ss`, `tcpdump`.
- WireGuard, or OpenVPN, or `ip link add type gre` for the tunnel section.
- Internet access for Part 5.

**Fallback:** everything except Part 5 runs on two VMs on one laptop.

---

## Procedure

### Part 1 — Building a WAN in software

**1.** Baseline first. Measure between the two hosts with no impairment:

```bash
iperf3 -c <server> -t 20
ping -c 100 <server>
```

Record throughput, and all five ping statistics.

**2.** Now add delay:

```bash
sudo tc qdisc add dev <iface> root netem delay 40ms
```

This adds 40 ms each way, giving roughly 80 ms RTT. Confirm with `ping`.

**3.** Measure throughput again. Record.

**4.** Add jitter:

```bash
sudo tc qdisc change dev <iface> root netem delay 40ms 15ms distribution normal
```

Measure `ping` again and record `mdev`. Then measure UDP jitter directly:

```bash
iperf3 -c <server> -u -b 50M -t 20
```

**5.** Add loss:

```bash
sudo tc qdisc change dev <iface> root netem delay 40ms loss 1%
```

Measure TCP throughput and compare with the Mathis prediction:

```bash
python3 ../tools/perfcalc.py loss --mss 1460 --rtt 80 --loss 0.01
```

**6.** Simulate a geostationary satellite:

```bash
sudo tc qdisc change dev <iface> root netem delay 250ms
```

Measure. Then try an interactive SSH session over it and record the experience.

---

### Part 2 — The bandwidth–delay product, demonstrated

**7.** With 80 ms RTT (`netem delay 40ms`), compute what the path needs:

```bash
python3 ../tools/perfcalc.py bdp --rate 1G --rtt 80
```

**8.** Now cripple the window deliberately. On the receiving host:

```bash
sudo sysctl -w net.ipv4.tcp_rmem="4096 65536 65536"
sudo sysctl -w net.ipv4.tcp_wmem="4096 65536 65536"
```

**9.** Measure single-stream throughput. Compare with:

```bash
python3 ../tools/perfcalc.py window --window 64K --rtt 80
```

**10.** Now measure with sixteen parallel streams:

```bash
iperf3 -c <server> -P 16 -t 20
```

Record the total and explain the difference.

**11.** Restore the window and repeat the single-stream test:

```bash
sudo sysctl -w net.ipv4.tcp_rmem="4096 131072 33554432"
sudo sysctl -w net.ipv4.tcp_wmem="4096 65536 33554432"
```

**12.** Inspect a live connection's negotiated scale factor and current window:

```bash
ss -tin | head -20
```

Record `wscale`, `cwnd` and `rtt`.

---

### Part 3 — Round trips versus bandwidth

**13.** With 80 ms RTT applied, fetch a single large file and time it:

```bash
time curl -s -o /dev/null http://<server>/largefile
```

**14.** Now fetch **sixty small files** sequentially:

```bash
time for i in $(seq 1 60); do curl -s -o /dev/null http://<server>/small$i; done
```

**15.** Repeat step 14 with connection reuse:

```bash
time curl -s -o /dev/null $(for i in $(seq 1 60); do echo http://<server>/small$i; done)
```

**16.** Compute, for step 14, how much of the elapsed time was round trips and how
much was data transfer. Record the ratio.

**17.** Change the impairment to 250 ms and repeat step 14. Record the new time
and confirm it scales with RTT rather than with file size.

---

### Part 4 — Tunnels

**18.** Build a WireGuard tunnel between the two hosts (or GRE if WireGuard is
unavailable — the MTU lesson works either way).

**19.** With the tunnel up and **no** impairment, measure throughput through the
tunnel and directly. Record the difference and account for it: encapsulation
overhead plus encryption cost.

**20.** Check the tunnel interface's MTU:

```bash
ip link show <tunnel-iface>
```

Record it and compute the overhead against the underlying interface's MTU.

**21.** Now create the classic failure. Set the tunnel MTU to 1500 — larger than
it should be — and block ICMP fragmentation-needed on the path:

```bash
sudo ip link set <tunnel-iface> mtu 1500
sudo iptables -A OUTPUT -p icmp --icmp-type fragmentation-needed -j DROP
```

**22.** Through the tunnel: `ping -c 2` (small), then `ping -c 2 -s 1400 -M do`,
then a large `curl` or `scp`. Record all three.

**23.** Fix it two ways and confirm each works:
(a) set the tunnel MTU correctly;
(b) restore the correct MTU, re-block ICMP, and clamp MSS instead:

```bash
sudo iptables -t mangle -A FORWARD -p tcp --tcp-flags SYN,RST SYN \
    -j TCPMSS --clamp-mss-to-pmtu
```

---

### Part 5 — A real path

**24.** Traceroute to a host on another continent. Use `mtr` if available:

```bash
mtr -n --report -c 50 <distant-host>
traceroute <distant-host>          # without -n, to see the names
```

**25.** From the **names** in the traceroute, identify: your ISP's access network,
the point where it hands off to a backbone, any airport codes embedded in router
names, and the transoceanic hop.

**26.** Identify the transoceanic hop by the RTT jump. Estimate the distance from
the jump using 204 km/ms and compare with the actual great-circle distance.

**27.** Look up the autonomous system numbers along the path if a looking glass or
`whois` is available, and identify where the traffic left your provider's AS.

---

## Expected observations

- **Step 3: throughput falls sharply with 80 ms of added delay**, even though no
  bandwidth was removed and no packets were lost.
- **Step 5:** the Mathis figure at 1% loss and 80 ms is around 15 Mb/s; your
  measurement should be in that region or somewhat above.
- **Step 9: about 6.5 Mb/s**, matching `window / RTT` exactly, and completely
  independent of the link's actual capacity.
- **Step 10: sixteen streams give roughly sixteen times the throughput.** Nothing
  about the network changed; sixteen windows were in flight instead of one.
- **Step 14 versus 13:** the sixty small files take far longer than one large file
  of the same total size, and the elapsed time is dominated by 60 × RTT.
- **Step 17:** tripling the RTT roughly triples the sequential-fetch time,
  confirming that the bottleneck is round trips.
- **Step 22: small pings work, `-s 1400 -M do` fails, the large transfer hangs
  after establishing.** The PMTUD black hole, built deliberately.
- **Step 26:** the RTT jump across an ocean is 60–80 ms and matches the distance
  arithmetic to within a factor of about 1.5, the excess being cable routing.

---

## Break it

Integrated — steps 8 and 21 are the deliberate faults. One further:

**A. Asymmetric impairment.** Apply 200 ms of delay in **one direction only**
(`netem` on one host's egress). Measure throughput both ways and record the
asymmetry. Then run `ping` and note that it reports the *sum* — and therefore
cannot tell you which direction is impaired. State what would.

---

## Debrief

**1.** Present your measurements from Part 1 as a table: impairment, TCP
throughput, UDP jitter, loss. For each impairment, state which network quantity
it models and which real-world path it resembles.

**2.** In step 9 you measured throughput almost exactly equal to window ÷ RTT.
State both numbers. Then explain why the calculation makes no reference to the
link's capacity, and what that implies for a customer who has just bought a
faster circuit to fix this symptom.

**3.** Sixteen parallel streams gave roughly sixteen times the throughput. Explain
the mechanism. Then explain why this is a *workaround* rather than a fix, and what
the correct fix is.

**4.** Compare your step 13 and step 14 timings. Compute what fraction of step 14
was round-trip latency. Then explain, in one sentence a manager would understand,
why an application that "works fine in the office" can be unusable from a branch
with plenty of bandwidth.

**5.** Your tunnel's MTU was X. Show the arithmetic: underlying MTU minus
encapsulation overhead. Then describe the failure you produced in step 22 exactly
as a user would report it, and explain why "ping works" misleads.

**6.** You fixed the black hole two ways. State the advantage of each, and say
which you would deploy on a carrier network serving customers whose firewall
policies you do not control — and why.

**7.** From your real traceroute: name the hop where your traffic left your
provider, name the transoceanic hop, and state how you identified each. Then
compute the theoretical minimum RTT for that ocean crossing and compare with
measurement.
