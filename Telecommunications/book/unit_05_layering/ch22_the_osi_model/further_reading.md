# Chapter 22 — Further Reading

## Primary sources

**ISO/IEC 7498-1, *Open Systems Interconnection — Basic Reference Model*.**
The model itself. Worth reading §5, which states the committee's principles for
creating a layer boundary — and then noticing that those principles do not determine
seven.

**Zimmermann, H. (1980). "OSI Reference Model." *IEEE Transactions on Communications*,
28(4).**
The readable canonical account, and much clearer than the standard. Where the
service/protocol distinction is properly stated.

**Pouzin, L. (1973). "Presentation and Major Design Aspects of the CYCLADES Computer
Network." *ACM/IEEE Symposium on Data Communications*.**
The datagram, and the argument that the network should be simple and unreliable with
the endpoints handling correctness. Heretical at the time, correct, and the direct
ancestor of everything in Chapter 23.

**Clark, D. (1988). "The Design Philosophy of the DARPA Internet Protocols."
*ACM SIGCOMM*.**
The other side's reasoning, in priority order, by one of the architects.

**RFC 1958 — Carpenter, B. (1996). *Architectural Principles of the Internet.***
Three pages stating what the Internet's architecture actually assumes. A useful
counterweight to the seven-layer chart.

## Books

**Russell, A. L. (2014). *Open Standards and the Digital Age: History, Ideology, and
Networks.* Cambridge University Press.**
**The best history of the OSI/TCP-IP contest.** Takes both sides seriously, is
scrupulous about the politics, and demolishes the retrospective inevitability that
makes TCP/IP's victory look obvious. Read this if you read only one thing from this
list.

**Rose, M. (1990). *The Open Book: A Practical Perspective on OSI.* Prentice Hall.**
Written by someone who implemented OSI properly and then argued against it. The
technical criticism is worth far more than the tribal kind.

**Day, J. (2008). *Patterns in Network Architecture.* Prentice Hall.**
The dissenting view: that both models are wrong and the field settled prematurely. Day
worked on OSI and is not defending it — his argument is that the correct decomposition
is neither seven layers nor four. Contentious and serious.

**Tanenbaum, A. & Wetherall, D. *Computer Networks*, 5th ed.**
Chapter 1's comparison of the two models is the standard treatment and is very fair to
both.

**Abbate, J. (1999). *Inventing the Internet.* MIT Press.**
The broader history, with good coverage of CYCLADES, the ARPANET, and the standards
politics of the 1980s.

## On the diagnostic method

**CompTIA Network+ troubleshooting methodology**, in any current study guide.
The seven-step process (identify, theorise, test, plan, implement, verify, document) is
examined directly, and §22.4's layer method is how you execute steps 2 and 3.

**Limoncelli, T., Hogan, C. & Chalup, S. (2016). *The Practice of System and Network
Administration*, 3rd ed.**
Chapter 15 on debugging. The general discipline around the technique: how to know when
you have found the cause rather than a symptom, and why changing one thing at a time is
non-negotiable.

**Google, *Site Reliability Engineering*, chapter 12 ("Effective Troubleshooting").**
Freely available online. The divide-and-conquer method generalised beyond networks,
with the point that **hypothesis-driven bisection beats intuition** stated with
evidence.

## Applied

**Run the §22.4 method on a working system** before you need it on a broken one. Learn
what healthy output looks like at every layer — `ip link`, `ip addr`, `ip route`,
`ip neigh`, `ss -tlnp`, `dig`. **You cannot recognise abnormal without knowing
normal**, and the time to learn it is not during an outage.

**`arping`.** Chapter 18's tool, and the single most useful command in §22.4's Layer 2
step. Use it once deliberately so you remember it exists.

**`mtr`** rather than `traceroute`, for continuous per-hop loss and latency. It answers
"where does it break" and "is it consistent" at the same time.

**`ss -tan`** and the connection states. Learn to read SYN-SENT (nothing answered),
ESTABLISHED (Layer 4 fine, look higher) and TIME-WAIT at a glance.

**`openssl s_client -connect host:443`**, for certificate expiry and chain problems.
The "worked yesterday, nothing changed" fault.

**Lab 10** in this book's [labs/](../../../labs/) directory injects a fault at a
randomly chosen layer and requires diagnosis by the method, with the steps documented.
Run it several times; the value is in the repetition.

**The week-13 fault gauntlet** in [instructor/exam-blueprints.md](../../../instructor/exam-blueprints.md)
is twelve scripted faults across all layers, timed. It is the practical assessment this
section exists for.

## For the certification-minded

**Objective 1.1 is the OSI model, and it is among the most heavily examined objectives
on the entire test.** Objective 5.1 is the troubleshooting methodology and 5.5 the
tools; §22.4 is directly examined.

Six things worth over-learning:

1. **The seven layers in both directions**, with a mnemonic each way.
2. **The PDU names**: bit, frame, packet, segment/datagram.
3. **The device table**: hub 1, switch 2, **access point 2**, router 3, **Layer 3
   switch 3**.
4. **The protocol table**: HTTP/DNS/DHCP/SMTP 7, TLS 6, NetBIOS/SMB 5, TCP/UDP 4,
   IP/ICMP 3, Ethernet/802.1Q 2.
5. **The three methods** — bottom-up, top-down, divide-and-conquer — by name, and when
   each applies.
6. **The seven-step CompTIA methodology**, in order, including **document the findings**
   as the last step. It is examined and it is the step people skip.

The three most-missed items: **the wireless access point is Layer 2, not Layer 1**;
**the Layer 3 switch is Layer 3, despite being a switch**; and **ARP does not fit — say
Layer 2 if the exam forces a choice.**

And the thing worth more than all of it: **a successful test at layer *n* proves layers
1 through *n* are working.** That single sentence turns an unbounded problem into three
commands.
