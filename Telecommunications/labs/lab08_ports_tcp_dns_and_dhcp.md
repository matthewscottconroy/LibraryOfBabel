# Lab 08 — Ports, TCP, DNS, and DHCP

**Corresponds to:** Chapters 35, 36, 37, 39, 40, 41
**Week:** 8
**Time:** 120 minutes

---

## Objectives

- Read a host's listening sockets and established connections, and distinguish
  exposure from safety at a glance.
- Capture and annotate a TCP three-way handshake and teardown.
- Recognise the five standard TCP capture signatures.
- Trace a DNS resolution from the root, and distinguish caching from record
  faults.
- Capture DORA and diagnose the three classic DHCP failures.
- Distinguish refused, timed-out and successful connection attempts.

---

## You will need

- Two hosts, Internet access, Wireshark and `tcpdump`.
- `dig`, `ss`, `nc` (netcat), `curl`.
- A DHCP server you may stop and start — a router's, or `dnsmasq` on a lab host.
- A second subnet with a router, for the relay section.

---

## Procedure

### Part 1 — Who is listening?

**1.** Enumerate every listening socket and its owning process:

```bash
sudo ss -tulnp
```

**2.** For each, record: protocol, local address, port, process. Then classify
each as **exposed** (bound to `0.0.0.0` or `::`) or **loopback-only**
(`127.0.0.1` or `::1`).

**3.** For every exposed service, answer: does this machine need to offer that
service to the network? Anything you cannot justify is an unnecessary attack
surface, and this is the whole of the first step of hardening.

**4.** Open a connection and observe the five-tuple:

```bash
curl -s https://example.com > /dev/null &
ss -tnp | grep example
```

**5.** Open four simultaneous connections to the same server and record all four
five-tuples. Identify which of the five components differs.

---

### Part 2 — The handshake

**6.** Start a capture, then make one HTTP connection:

```bash
sudo tcpdump -i any -n -w /tmp/tcp.pcap 'tcp port 80' &
curl -s http://example.com > /dev/null
```

**7.** Open in Wireshark. Find the first three packets and annotate:

- Packet 1: flags, sequence number, and the options — is window scaling
  negotiated? What scale factor?
- Packet 2: flags, sequence number, acknowledgement number
- Packet 3: flags, acknowledgement number

**8.** Record the **initial sequence numbers** from both directions. Are they
small or effectively random? Explain why.

**9.** Find the teardown. Record the flag sequence and identify which side closed
first. Then find the socket in `TIME_WAIT`:

```bash
ss -tan | grep TIME-WAIT
```

**10.** Use **Statistics → Conversations** and **Follow → TCP Stream** on the
capture. Record the total bytes and the number of round trips before the first
byte of HTML.

---

### Part 3 — TCP under stress

**11.** Introduce loss and delay on one host with `tc`:

```bash
sudo tc qdisc add dev <iface> root netem delay 80ms loss 1%
```

**12.** Run a transfer and capture:

```bash
iperf3 -c <server> -t 30
```

**13.** In the capture, find and screenshot or record:

- duplicate ACKs
- a fast retransmit
- a retransmission-timeout retransmit (note the gap before it)
- the congestion window's sawtooth, via **Statistics → TCP Stream Graphs**

**14.** Compute the expected ceiling and compare with measurement:

```bash
python3 ../tools/perfcalc.py loss --mss 1460 --rtt 80 --loss 0.01
```

**15.** Remove the impairment:

```bash
sudo tc qdisc del dev <iface> root
```

---

### Part 4 — DNS

**16.** Trace a resolution from the root:

```bash
dig +trace www.example.com
```

Record each referral: which server was asked, and what it returned.

**17.** Query the authoritative server directly and compare with your resolver:

```bash
dig www.example.com                          # your resolver
dig @<authoritative-ns> www.example.com      # authoritative
```

**18.** Record the TTL from your resolver's answer. Query again immediately and
record the TTL. Explain the difference.

**19.** Examine the record types for a domain:

```bash
dig example.com SOA
dig example.com NS
dig example.com MX
dig example.com TXT
```

Identify any SPF record in the TXT output and read what it permits.

**20.** Capture a DNS query and confirm it used UDP port 53. Then force TCP and
confirm:

```bash
dig +tcp www.example.com
```

---

### Part 5 — DHCP

**21.** Capture DORA. Release and renew:

```bash
sudo tcpdump -i <iface> -n port 67 or port 68 &
sudo dhclient -r <iface> && sudo dhclient <iface>
```

**22.** For each of the four messages, record: source IP, destination IP, source
port, destination port, and whether it was broadcast or unicast.

**23.** Explain why DISCOVER is sourced from `0.0.0.0`, and why REQUEST is
broadcast rather than unicast to the chosen server.

**24.** Examine the options in the ACK. List every option present and what it
configures.

---

## Expected observations

- **Step 2:** several services are listening. A service on `127.0.0.1` is
  unreachable from the network entirely; the same service on `0.0.0.0` is exposed.
  That one difference is frequently the difference between a safe and a breached
  database.
- **Step 5:** only the **source port** differs across four connections to the same
  server.
- **Step 8:** ISNs are effectively random, per RFC 6528, because predictable ones
  permitted connection spoofing.
- **Step 10:** roughly three round trips before content — DNS, TCP handshake,
  then the request — and more with TLS.
- **Step 13:** three duplicate ACKs trigger a fast retransmit; an RTO retransmit
  shows a visible gap and backs off.
- **Step 14:** the Mathis figure at 1% loss and 80 ms RTT is around 15 Mb/s, and
  your measurement should be in that region — likely somewhat above, since CUBIC
  is more loss-tolerant than the classic model.
- **Step 18:** the TTL **counts down** between queries, because the resolver is
  serving a cached answer and reporting the remaining lifetime.
- **Step 22:** DISCOVER is `0.0.0.0:68 → 255.255.255.255:67`; the client has no
  address yet, so the protocol must work entirely in broadcast.

---

## Break it

**A. Connection refused versus timeout.** Test three cases and record the exact
behaviour and timing of each:

```bash
nc -zv <host-with-nothing-listening> 9999      # refused: RST
nc -zv <host-behind-a-drop-rule> 9999           # timeout: silence
nc -zv <host-with-service> 22                   # connected
```

State what each proves about the path and about the service.

**B. Break DNS only.** Point the resolver at an address that does not answer.
Confirm `ping 1.1.1.1` works and `ping example.com` does not. Note how many OSI
layers this single pair of tests exonerates.

**C. Stop the DHCP server** and release a client's lease. Record the address the
client self-assigns and the time it takes.

**D. Rogue DHCP.** On an isolated segment, run a second DHCP server offering a
different subnet and gateway. Have several clients request addresses and record
**which server each got** — and that it varies. This is the deterministic cause
producing a non-reproducible symptom.

**E. Remove the DHCP relay** from a second subnet's router while leaving the
first subnet's clients working. Record the asymmetry.

---

## Debrief

**1.** From your `ss` output, identify one service bound to `0.0.0.0` and state
what would be required to reach it from another subnet. Then state the one-line
configuration change that would make it loopback-only, and what would break.

**2.** Four connections to one server differed only in source port. Explain, using
the five-tuple, how the kernel demultiplexes their return traffic. Then explain
why NAT must rewrite ports and not merely addresses.

**3.** Annotate your handshake capture: for each of the three packets, state what
each party knows after receiving it that it did not know before. Use that to
explain why two packets are insufficient.

**4.** Your measured throughput under 1% loss and 80 ms RTT was X. Compare with
the Mathis prediction and account for the difference. Then state what would happen
to X if the RTT doubled and the loss stayed the same, and verify with the tool.

**5.** In step 17 you compared your resolver's answer with the authoritative
server's. Explain what a *difference* would have meant and what an *agreement*
with a wrong value would have meant. State why this single comparison eliminates
half the DNS possibilities.

**6.** In Break-It D, different clients got addresses from different servers,
unpredictably. Explain the mechanism, state why this is one of the most confusing
faults in networking, and name the switch feature that prevents it entirely.

**7.** Break-It A produced three distinct outcomes. Tabulate them — outcome, what
it proves about the path, what it proves about the service, and the next
diagnostic step. Keep this table; the week 13 gauntlet uses it.
