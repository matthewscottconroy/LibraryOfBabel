# Lab 01 — Inspecting a Working Host

**Corresponds to:** Chapters 1, 2, 3, 23
**Week:** 1
**Time:** 90 minutes

---

## Objectives

By the end of this lab you will be able to:

- Read a host's complete network configuration and explain what each value does.
- Measure latency and interpret all five statistics `ping` reports, not just the
  average.
- Distinguish propagation delay from queueing delay from a measurement.
- Capture and identify the layers of encapsulation in a single packet, matching
  them to Chapter 23's model.
- Convert an address between dotted decimal, binary and hexadecimal, and find it
  in a hex dump.

---

## You will need

- One Linux machine (a virtual machine is fine), or macOS with the noted command
  substitutions. Windows works with `ipconfig`/`tracert` and Wireshark.
- Wireshark, or `tcpdump` with the ability to run it as root.
- Internet access.

Everything in this lab runs on a single machine. No special equipment.

---

## Procedure

### Part 1 — What am I?

**1.** Display your interfaces and addresses.

```bash
ip addr                 # Linux
ifconfig -a             # macOS / older Linux
ipconfig /all           # Windows
```

Record, for your primary interface: the IPv4 address, the prefix length, the MAC
address, and any IPv6 addresses. **Note how many IPv6 addresses there are** —
Chapter 28 §28.2 explains why there is more than one.

**2.** Display your routing table.

```bash
ip route
route -n                # alternative
netstat -rn             # macOS / Windows
```

Identify the default route, the directly-connected route, and the interface each
uses. For every entry, state in one sentence what it means.

**3.** Display your DNS configuration.

```bash
resolvectl status       # systemd-resolved
cat /etc/resolv.conf    # traditional
ipconfig /all           # Windows — look for "DNS Servers"
```

**4.** In your notebook, complete this sentence with your own values and no
gaps: *"My machine is at ___/___, on the network ___, and to reach anything
outside that network it sends frames to ___, whose MAC address it learned by
___."*

You cannot complete the last clause yet with certainty. Part 3 will let you.

---

### Part 2 — How far away is everything?

**5.** Ping four destinations of increasing distance. Use 100 packets so the
statistics mean something.

```bash
ping -c 100 <your default gateway>
ping -c 100 1.1.1.1
ping -c 100 <a host in another country — ask your instructor>
ping -c 100 <a host on another continent>
```

Record **all five** numbers each time: min, avg, max, mdev, and loss.

**6.** For each destination, compute `avg − min`.

**7.** Estimate the great-circle distance to each destination (any online tool
will do) and compute the theoretical minimum round-trip time using 204 km/ms for
fibre. Compare with your measured minimum.

**8.** Traceroute to the most distant destination.

```bash
traceroute -n <destination>       # Linux/macOS
tracert -d <destination>          # Windows
mtr -n --report -c 50 <dest>      # better, if available
```

Identify the hop at which the round-trip time jumps substantially. That is very
likely a long-haul or submarine link, and Chapter 48 will let you read the
router names.

---

### Part 3 — What does a packet look like?

**9.** Start a capture, filtered to keep it manageable.

```bash
sudo tcpdump -i any -n -x -c 5 'icmp'
```

In another terminal, `ping -c 3 1.1.1.1`.

**10.** From the hex output, identify by byte offset:

- destination MAC (bytes 0–5)
- source MAC (bytes 6–11)
- EtherType (bytes 12–13) — confirm it is `0800`
- IP version and header length (byte 14) — confirm it is `45`
- TTL (byte 22)
- protocol (byte 23) — confirm it is `01` for ICMP
- source IP (bytes 26–29)
- destination IP (bytes 30–33)

**11.** Convert the source IP bytes from hex to dotted decimal by hand. Confirm
it matches your `ip addr` output from step 1.

**12.** Open the same capture in Wireshark and expand the protocol tree. Match
each expandable layer to a layer of Chapter 23's model, and to the byte offsets
you identified by hand.

---

## Expected observations

- Your gateway ping should be **under 5 ms** with near-zero `mdev`. If it is
  not, something on the local segment is wrong and that is worth investigating.
- Measured minimum RTT to a distant host should be **1.3–2× your theoretical
  minimum**. Fibre does not run great-circle; it routes around continents and
  follows existing infrastructure.
- `avg − min` should be **small (under 5 ms) for nearby hosts** and larger for
  distant ones.
- The TTL in the capture should be a round number minus the hop count — Linux
  starts at 64, Windows at 128, many network devices at 255. Knowing the
  starting value lets you infer the hop count from the observed TTL.

---

## Break it

Introduce each of the following, observe the symptom, then restore. Record what
you saw *before* reading the explanation.

**A. Remove your default route.**

```bash
sudo ip route del default
ping 1.1.1.1            # observe
ping <a host on your own subnet>   # observe
sudo ip route add default via <your gateway>
```

**B. Set a wrong subnet mask.** Change your prefix from `/24` to `/25` (or
whatever narrows it), then try to reach a host that was previously local.

```bash
sudo ip addr change <your addr>/25 dev <iface>
```

**C. Point at a nonexistent DNS server**, then try `ping google.com` and
`ping 1.1.1.1`.

For each, answer before restoring: **which of Chapter 22's layers is still
working, and how do you know?**

---

## Debrief

Answer in writing. These are the assessment.

**1.** Your ping to a host 8,000 km away reported a minimum of X ms. Compute the
theoretical minimum from the distance. Account for the difference — name at
least three contributions and estimate the magnitude of each.

**2.** You measured `avg − min` for four destinations. What physical quantity
does that difference estimate, and why is the *minimum* a good proxy for the
irreducible delay?

**3.** In Break-It B you changed the mask and lost connectivity to *some* hosts
but not others. Explain exactly which hosts became unreachable and why, using
the AND operation from Chapter 2 §2.2. Predict, before testing, which hosts
would become unreachable with a `/26` instead.

**4.** In Break-It C, `ping google.com` failed and `ping 1.1.1.1` succeeded.
Using Chapter 22 §22.4's layered method, state which layers this single pair of
observations exonerates, and how many of the seven you eliminated with one test.

**5.** Your capture showed a TTL of N. Given the starting values above, how many
routers did the packet cross? Compare with your traceroute hop count. If they
disagree, propose an explanation.

**6.** You have now watched a single ICMP packet carry four layers of headers to
deliver 32 bytes of payload. Compute the efficiency. Then compute it for a
1,460-byte TCP payload using `perfcalc.py goodput`. Explain the difference in
one sentence.
