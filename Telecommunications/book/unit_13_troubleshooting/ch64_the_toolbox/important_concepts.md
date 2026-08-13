# Chapter 64 — Important Concepts

**A successful ping proves Layers 1–3 in both directions and nothing about the service**
*(§64.1)* — **A host can ping perfectly with every application down**, and this is the commonest
misreading of the result.

**"It doesn't ping" is not "it's down"** *(§64.1)* — **ICMP filtering is routine**, a firewall may
permit 443 and drop echo, and **the reply may be blocked rather than the request.** **Test the
actual service before concluding anything.**

**Add 28 to `ping -s` to get the IP MTU** *(§64.1)* — 20 bytes of IP plus 8 of ICMP. **And a
local "message too long" names the MTU; a silent drop is the black hole** — the second means
something upstream is discarding without telling you.

**The reply's TTL is free information** *(§64.1)* — **`ttl=62` from a Linux host (initial 64)
means two hops.** 128 for Windows, 255 for many network devices.

**Traceroute's latency at hop N is the round trip to hop N, not the latency of hop N**
*(§64.1)* — **A high figure that returns to normal at N+1 means that router's control plane was
slow to generate ICMP**, which is deprioritised by design. **Not a fault.**

**Asterisks are not packet loss** *(§64.1)* — **They mean no Time Exceeded was returned**, which
is usually a router configured not to send them. **Traffic passes through that hop perfectly.**
**Worry only when the asterisks run from hop N to the end.**

**If traceroute stops and the destination is reachable, change the probe type** *(§64.1)* —
**Linux uses UDP by default and Windows uses ICMP; `traceroute -T -p 443` gets through where
both are filtered**, and its success proves the path is fine.

**Loss at an intermediate `mtr` hop that does not appear at subsequent hops is not loss**
*(§64.1)* — **It is ICMP rate limiting**, and if the traffic were genuinely lost the later hops
could not show 0%. **Loss that begins at hop N and persists to the destination is real.**

**Traceroute shows the forward path only** *(§64.1)* — **Each Time Exceeded returns by whatever
route its router chooses.** **Asymmetric routing is common and traceroute cannot show it**,
which is why a traceroute from each end is worth having.

**`UP` without `LOWER_UP` means no carrier** *(§64.2)* — And **`169.254.x.x` means DHCP failed**,
and **a `/24` where the subnet is a `/22` makes local destinations remote or remote ones local.**

**`ip route get <destination>` asks the kernel which route it would actually use** *(§64.2)* —
Which removes the guesswork from a route table with overlapping entries. **And two default
routes with equal metrics is a fault that produces intermittent behaviour looking like anything
but routing.**

**`FAILED` in the neighbour table for the default gateway is unambiguous** *(§64.2)* — **The
gateway is down, or the host is in the wrong VLAN, or the address or mask is wrong.**

**The two `dig` queries that solve most problems** *(§64.2)* — **`dig @<authoritative>` versus
`dig @<your resolver>`.** **If they differ the answer is cached and stale, and the TTL says for
how long; if they agree the record is what you think and the problem is elsewhere.** **And
`dig +trace` stops at the point where the delegation is wrong.**

**Read the `SERVER` line** *(§64.2)* — **It is frequently not the resolver you assumed**, and
"resolves on one machine and not another" is usually this.

**Intermittent DNS failure usually means one of several configured resolvers is unhealthy**
*(§64.2)* — **Query each explicitly rather than testing the aggregate.**

**A capture is evidence in a way that a device's summary is not** *(§64.3)* — **"The firewall
says it permitted it" and "the packet arrived on the far side" are different claims**, and when
a vendor disputes a fault the capture ends the conversation.

**Capture at both ends of the suspect element** *(§64.3)* — **The question is almost always "did
it arrive?"**

**A SPAN port mirroring both directions of a 1 Gb/s link to one 1 Gb/s port drops traffic
whenever the total exceeds 1 Gb/s** *(§64.3)* — **Which is routinely.** **If your capture shows
loss, check whether the loss is in the network or in your capture.** **A TAP does not drop and
does show errors and runts, which a switch discards before mirroring.**

**Capture broadly, filter narrowly** *(§64.3)* — **A capture filter that is too tight discards
the packet that would have explained everything, and you cannot get it back.** **And `not port
22` when capturing on the machine you are connected to**, or the capture records itself.

**Find the failure, not the traffic** *(§64.3)* — **Filter for resets, retransmissions, ICMP
errors and DNS failures**, not for the application. **`tcp.analysis.flags` shows Wireshark's
entire opinion of what went wrong in one filter.** **And read the summary — Protocol Hierarchy
and Conversations — before any individual packet.**

**The signatures worth recognising instantly** *(§64.3)* — **SYN with no reply is filtered or
nothing listening; SYN → RST is actively refused; handshake completing then RST is the
application closing; zero window is the receiving application not reading, which is not a network
fault; and large packets absent with small ones present is an MTU black hole.**

**"Everything arrived correctly and the application still fails" is one of the most valuable
capture outcomes** *(§64.3)* — **It ends the network team's involvement with evidence rather
than assertion.**

**A single TCP stream's throughput is window ÷ RTT** *(§64.4)* — **A 64 KB window on a 40 ms
path gives 13 Mb/s regardless of the link.** **`-P 8` measures what the link can carry; `-w`
measures what one tuned flow achieves** — **report which you did**, because a carrier will ask.

**`iperf3 -u` does not measure throughput** *(§64.4)* — **It sends at a fixed rate and reports
loss and jitter**, which is the voice and video assessment. **And `-b 0` saturates the path and
measures nothing useful.**

**A speed test to a server inside your ISP measures the access link and nothing else** *(§64.4)*
— And **a result below the link rate may be measuring the laptop's CPU, disk or vNIC**; test
between two other machines to find out.

**`closed` and `filtered` are two entirely different faults distinguished by one word**
*(§64.4)* — **`closed` means you reached the host and nothing is listening; `filtered` means you
did not reach it.**

**Tell someone before scanning** *(§64.4)* — **Unauthorised scanning is a criminal offence in
many jurisdictions**, and an unannounced internal scan wastes the security team's day.

**The fibre inspection scope is the highest-value physical tool per pound** *(§64.4)* — **A dirty
connector is the single most common fibre fault**, presenting as attenuation, intermittent
errors or a link that will not come up — **all diagnosed expensively by every other means and in
ten seconds by looking.** **And `show interface transceiver` gives the Rx power for free**;
compare it against the optic's sensitivity.

**A tone generator solves a problem with no software solution** *(§64.4)* — **An unlabelled cable
in a bundle of forty cannot be traced by any amount of software** (Chapter 53 §53.2's labelling
argument, from the other side).

**`show tech-support` before you change anything** *(§64.4)* — **One command, everything
captured** (Chapter 63 §63.1's evidence-before-action rule), **and it is the difference between a
vendor case that progresses and one that asks you to reproduce the fault.**

**A packet capture records other people's traffic** *(§64.3)* — **In many jurisdictions capturing
on a network you do not own, or capturing content rather than headers, is a criminal offence**,
and in most organisations it requires authorisation regardless. **Capture what you need, store it
with access control, and delete it when the incident closes.**
