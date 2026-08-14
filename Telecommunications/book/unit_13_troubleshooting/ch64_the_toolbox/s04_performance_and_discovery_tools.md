# 64.4 Performance and Discovery Tools

Measuring how much, finding what is there, and testing the physical layer — the three
remaining categories.

## iperf3

The standard throughput measurement tool, and the standard way to measure it wrongly.

```
   Server:  $ iperf3 -s
   Client:  $ iperf3 -c 10.9.0.5 -t 30
   
   [ ID] Interval        Transfer   Bitrate      Retr
   [  5] 0.00-30.00 sec  3.28 GBytes  940 Mbits/sec  0   sender
   [  5] 0.00-30.00 sec  3.27 GBytes  938 Mbits/sec      receiver
```

The options that change what you are measuring:

| Option | Does | When |
|---|---|---|
| **`-t N`** | duration | **at least 30 s** — TCP needs time to reach steady state |
| **`-P N`** | **parallel streams** | **essential on high-BDP paths** — see below |
| **`-R`** | **reverse** — server sends | **always test both directions** |
| **`-u -b <rate>`** | **UDP at a rate** | **measures loss and jitter, not throughput** |
| **`-w <size>`** | **window size** | **when the default is the constraint** |
| **`-i N`** | interval reporting | **shows ramp-up and instability** |
| **`-M <mss>`** | segment size | MTU investigation |
| **`--get-server-output`** | the far end's view | |

### The single-stream trap

The commonest misuse, and the arithmetic explains it.

> A single TCP stream's throughput is bounded by window ÷ RTT (Chapter 3 §3.4).

| RTT | Link | **BDP** | **A 64 KB window gives** |
|---|---|---|---|
| 1 ms | 1 Gb/s | 0.12 MB | **524 Mb/s** |
| **30 ms** | 1 Gb/s | 3.75 MB | **17.5 Mb/s** |
| **100 ms** | **1 Gb/s** | **12.5 MB** | **5.2 Mb/s** |
| 150 ms | 10 Gb/s | 187 MB | **3.5 Mb/s** |

So a single-stream test across a long path measures the window, not the link.

**Two responses, and they answer different questions:**

**`-P 8` or more.** **Parallel streams aggregate**, and the total approaches the link rate.
This measures what the link can carry.

**`-w` to raise the window.** This measures what one properly-tuned flow can achieve, which
is what an application will actually get.

> **Report which you did.** "We measured 940 Mb/s with 8 parallel streams" and "we measured
> 940 Mb/s single-stream" are different claims about the network, and a carrier will ask.

### UDP mode, and what it is actually for

`iperf3 -u -b 100M` does not measure throughput. It sends at a fixed rate and reports
loss and jitter, which is what you want for a voice or video assessment (Chapter 52 §52.1).

**And it must be rate-limited deliberately.** `-b 0` sends as fast as possible and will
saturate the path, which measures nothing useful and disrupts everyone.

**Read the loss figure, not the bitrate:**

```
   [ ID] Interval    Transfer  Bitrate     Jitter  Lost/Total
   [  5] 0.00-30.00  358 MBytes 100 Mbits/sec 0.412 ms  1204/258630 (0.47%)
```

0.47% loss with 0.4 ms jitter is acceptable for video and marginal for voice (Chapter 52
§52.1's table).

### Testing honestly

**Four requirements, and each is routinely violated:**

**Test to the right place.** A test to a server in your own data centre measures your LAN.
A speed-test server inside your ISP measures the access link and nothing else (Chapter 49
§49.4).

**Test in both directions.** **`-R`**, and asymmetric links (Chapter 49 §49.2) make this
essential.

Test while the link is otherwise idle, and again while it is not. The second is the
useful one — Chapter 66 §66.4's bufferbloat is invisible on an idle link.

**And check the endpoints.** A test that measures 300 Mb/s on a 1 Gb/s link may be measuring a
laptop's CPU, its disk, or a virtual machine's vNIC, and the way to find out is to test
between two other machines on the same path.

## nmap

Discovery and port scanning, and the first rule is authorisation.

> Scanning a network you do not own or are not explicitly authorised to test is a criminal
> offence in many jurisdictions, and an internal scan without notice will trigger security
> alerting and waste other people's time. **Tell someone first.**

| Command | Does |
|---|---|
| **`nmap -sn 10.20.5.0/24`** | **host discovery only — no port scan.** The inventory question |
| **`nmap -sT -p 443 host`** | **TCP connect** — needs no privilege, and it completes handshakes |
| **`nmap -sS -p- host`** | **SYN scan, all ports** — faster, needs privilege |
| **`nmap -sU -p 53,123,161 host`** | **UDP** — slow, and frequently ambiguous |
| **`nmap -sV host`** | **service version detection** |
| **`nmap -O host`** | OS fingerprinting |
| **`nmap --script vuln host`** | **the scripting engine** — and read what a script does before running it |
| **`nmap -Pn host`** | **skip host discovery** — for hosts that do not respond to ping |

Reading the states, which is where the diagnostic value is:

| State | Means |
|---|---|
| **`open`** | **something is listening and accepted** |
| **`closed`** | **the host is up and nothing is listening** — **an RST was returned** |
| **`filtered`** | **no response at all** — a firewall dropping silently |
| `open|filtered` | **UDP, usually** — no response could mean either |

> **The distinction between `closed` and `filtered` is the useful one.** `closed` means you
> reached the host and the service is not running; `filtered` means you did not reach it.
> **Two entirely different faults, distinguished by one word.**

And for a single quick check, `nc` is faster than `nmap`:

```
   $ nc -zv 10.9.0.5 443
   Connection to 10.9.0.5 443 port [tcp/*] succeeded!
   
   PS> Test-NetConnection 10.9.0.5 -Port 443
```

## Physical layer tools

The category network engineers most often lack and most often need.

| Tool | Tests | Note |
|---|---|---|
| **Cable tester (continuity)** | **wiremap — which pin goes where** | **cheap; finds miswires and opens** |
| **Cable certifier** | **attenuation, NEXT, return loss, against a category standard** | **expensive; required to certify an installation** |
| **Tone generator and probe** | **which cable is which** | **the tool for an unlabelled patch panel** (Chapter 53 §53.2) |
| **TDR** (time-domain reflectometer) | **distance to a fault in copper** | built into many certifiers |
| **OTDR** (optical) | **distance to a break, splice loss, bend loss** | **essential for fibre faults** |
| **Optical power meter and light source** | **actual dBm at each end** | **the fibre equivalent of a cable tester** |
| **Fibre inspection scope** | **whether the connector is dirty** | **the commonest fibre fault, and the cheapest to fix** |
| **Loopback plug / adapter** | **is the port itself working?** | trivial and conclusive |
| **Wi-Fi analyser / spectrum analyser** | Chapter 45 §45.1 | |

**Two of those deserve emphasis.**

> **The fibre inspection scope is the highest-value physical tool per pound.** A dirty
> connector is the single most common fibre fault, and it presents as attenuation,
> intermittent errors, or a link that will not come up — all of which are diagnosed
> expensively by every other means and in ten seconds by looking.

And the tone generator solves a problem that has no other solution. An unlabelled cable in
a bundle of forty cannot be traced by any amount of software (Chapter 53 §53.2's argument for
labelling, from the other side).

What the device itself tells you about the optics:

```
   $ show interface transceiver
   Port      Temp   Voltage  Tx Power  Rx Power
   Te1/0/1   38.4C  3.29V    -2.1 dBm  -18.4 dBm
   Te1/0/2   39.1C  3.30V    -2.3 dBm  -31.2 dBm   ← below sensitivity
```

Which is free, requires no tools, and answers the fibre question immediately — compare Rx
power against the optic's documented sensitivity, and a value near or below it is the fault.

## The `show` commands that matter

**Vendor syntax differs; the questions do not.**

| Question | Cisco-ish | Linux |
|---|---|---|
| **Is the link up and clean?** | `show interface` | `ip -s link`, `ethtool` |
| **What errors?** | `show interface \| include error` | `ip -s -s link` |
| **What is on this port?** | `show mac address-table interface` | `bridge fdb` |
| **What VLAN?** | `show interface switchport` | `bridge vlan` |
| **Is STP forwarding?** | `show spanning-tree interface` | `mstpctl` |
| **What routes?** | `show ip route <dest>` | `ip route get <dest>` |
| **What neighbours?** | `show cdp/lldp neighbors` | `lldpctl` |
| **What is the CPU doing?** | `show processes cpu sorted` | `top` |
| **What did it log?** | `show logging` | `journalctl` |
| **Everything, for a vendor case** | **`show tech-support`** | `sosreport` |

**`show tech-support` deserves its own line.** One command, everything captured, before you
change anything (Chapter 63 §63.1's evidence-before-action rule). It takes a minute and it
is the difference between a vendor case that progresses and one that asks you to reproduce the
fault.

## Choosing a tool from a symptom

| Symptom | Reach for |
|---|---|
| **Cannot reach anything** | `ip addr`, `ip route`, `ping <gateway>` (§64.1, §64.2) |
| **Cannot reach one thing** | `ping` by IP then by name, then `nc -zv` |
| **Slow** | **`iperf3` both directions, `mtr`, and interface counters** (Chapter 66) |
| **Intermittent** | **`mtr -c 1000`, ring-buffer `tcpdump`, counter polling** |
| **Names not resolving** | **`dig @<each resolver>`, `dig +trace`** |
| **Link will not come up** | **`show interface`, transceiver power, a loopback, a scope** |
| **Works for some, not others** | **compare configurations; `show interface switchport`** |
| **"The network is broken"** | **Chapter 63 §63.1's questions, before any tool** |

## What breaks here

An `iperf3` result far below the link rate on a long path. Single stream and the window.
`-P 8`, or `-w`.

A speed test showing full rate and applications slow. The test server is inside the ISP.

**A throughput test limited by the laptop.** Test between two other machines to eliminate the
endpoints.

**`iperf3 -u -b 0` disrupting the network.** It sends as fast as it can. Always specify a
rate.

**A scan that triggered a security incident.** **Tell someone first.**

**`filtered` interpreted as `closed`.** **Two different faults.**

A fibre link with errors and nothing found. Look at the connector with a scope, and read
the transceiver's Rx power.

**A vendor case that goes nowhere.** `show tech-support` was not captured before the reboot.

**An unlabelled cable and no tone generator.** There is no software solution.

> **Network+ note.** Objective 5.5 covers tools directly. Over-learn: **`iperf` measures
> throughput**; **`nmap` performs discovery and port scanning**; a cable tester checks
> continuity and wiremap while a certifier tests to a standard; **a toner and probe locate a
> cable**; **an OTDR locates a fibre fault by distance**; **a light meter measures optical
> power**; and **`show` commands report interface status and errors.** The
> tester-versus-certifier and TDR-versus-OTDR distinctions are both examined.
