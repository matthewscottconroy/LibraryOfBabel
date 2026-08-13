# Chapter 65 — Important Concepts

**`up/down` is no carrier; `err-disabled` is the switch protecting itself** *(§65.1)* — **And the
reason is always logged.** BPDU guard, port security, link flapping, storm control. **Re-enabling
without reading it is how a fault recurs in ninety seconds.**

**Swapping the patch lead is the highest-value action in Layer 1 diagnosis** *(§65.1)* — **A
patch lead costs less than the time spent reasoning about whether it is the problem**, and the
swap either fixes the fault or eliminates a category.

**A split pair passes a continuity test** *(§65.1)* — **Every pin connects to the right pin and
the twisting is wrong.** **10/100 works and Gigabit does not**, or Gigabit links with CRC errors
under load, or it works short and fails long. **Only a certifier finds it**, because the fault is
in NEXT rather than in connectivity.

**The 100 m limit includes the patch leads at both ends** *(§65.1)* — 90 m permanent plus 5 m
each end is the design assumption, **and 15 m patches at each end of a 90 m run is out of
specification** — which usually works and is the explanation when it does not.

**CRC with late collisions is a duplex mismatch; CRC without is physical** *(§65.1)* — **Two
faults diagnosed and fixed entirely differently.**

**Read the transceiver's optical power first** *(§65.1)* — **Free, five seconds, and it answers
the question.** **Rx power near or below the optic's sensitivity is the fault**, and everything
else is a search for why. **Compare the far end's Tx with your Rx and the difference is the
loss in between.**

**A dirty connector is the most common fibre fault, and it takes ten seconds to check**
*(§65.1)* — Then bends, then fibre type, then wavelength, then swapped Tx/Rx. **And receiver
overload is real**: a 40 km optic on 200 m of fibre produces errors on a link that is "too
good", and the fix is a passive attenuator.

**A single-mode optic into multimode fibre may link and produce errors** *(§65.1)* — **Which is
worse than not linking, because it appears to work.**

**A PoE device that reboots at a predictable time or under a specific activity is a power
problem** *(§65.1)* — **Budget exhaustion means whichever devices negotiated last do not power
up**; **voltage drop over 90 m means the device reboots when its load increases**; and **a PoE+
device on a PoE port gets 12.95 W and misbehaves.**

**"Nothing changed" is frequently false at the physical layer** *(§65.1)* — **A contractor in a
ceiling, a cleaner moving a desk, someone pulling a cable to reach a socket.** **Chapter 63's
"what changed?" should include physical work**, and facilities frequently knows.

**"No IP address" is very often a VLAN problem, not a DHCP problem** *(§65.2)* — **A device on
the wrong VLAN sees the wrong DHCP server, or none.** **The most useful misattribution to
correct in this book.**

**Check the operational mode, not the documentation** *(§65.2)* — `show interface switchport`,
**and the documentation is frequently the thing that is wrong.**

**A VLAN missing from the trunk's allowed list works within each switch and not between them**
*(§65.2)* — **Which looks like routing and is not.** **It is the step most often forgotten when a
VLAN is added.**

**`dynamic auto` at both ends means neither initiates** *(§65.2)* — **The link stays access,
works for one VLAN, and silently fails for the rest**, with correct-looking configuration at both
ends.

**The topology change counter is the single most useful spanning tree diagnostic** *(§65.2)* —
**"1,847 changes, last 42 seconds ago, from Gi1/0/13" names the port**, and a port generating
topology changes is flapping or lacks PortFast.

**A MAC flapping between two ports identifies a loop faster than any topology analysis**
*(§65.2)* — **The log message names both ports**, and it is frequently ignored because it is
informational.

**Both sides auto, or both sides forced identically — never one of each** *(§65.2)* — **A forced
side does not participate in negotiation and the auto side falls back to half duplex.** The
symptom is throughput collapse under load, not failure.

**Proxy ARP makes a wrongly-masked host work anyway** *(§65.2)* — **Which hides the fault for
years**, until it is disabled or a device that does not do it is installed, **and then everything
breaks at once.**

**Mask too narrow works via the gateway; mask too wide fails absolutely** *(§65.3)* — **Too
narrow treats local destinations as remote — functional, with a redirect and an extra hop.** **Too
wide treats remote destinations as local — the host ARPs and nothing answers.** **"Works for some
destinations and not others on the same subnet" is a mask question**, and `ip route get` answers
it.

**Asymmetric routing is not a fault until something stateful is in the path** *(§65.3)* — **IP
permits it and it is common.** **A stateful firewall, NAT, a load balancer or strict uRPF makes it
one**, and the signature is "some flows work and some do not", appearing after a redundancy
change. **Diagnosed only by traceroute from both ends.**

**An OSPF adjacency stuck at ExStart is an MTU mismatch** *(§65.3)* — **The database description
packets are large and one side cannot receive them.** **The state name is the diagnosis.**

**A route that flaps is worse than one that is missing** *(§65.3)* — **Every change is recomputed
everywhere**, and the fix is at the flapping element rather than at the routing protocol.

**NAT hairpinning: the service works from outside and not from inside** *(§65.3)* — **Which is
the opposite of what anyone expects.** **Split-horizon DNS is the better fix** — give internal
clients the internal address and the problem does not arise.

**"Turn off IPv6" is the reflexive fix and it is usually wrong** *(§65.3)* — **It hides a
misconfiguration that will be waiting when IPv6 is required.** Fix the RA, the routing, or the
ICMPv6 filter.

**"Refused" and "timed out" are two entirely different faults** *(§65.4)* — **Refused means the
packet arrived and something said no; timed out means it vanished.** **Every operating system
distinguishes them**, and a great deal of time is wasted by not reading which appeared.

**A service listening on `127.0.0.1` is reachable only from the machine itself** *(§65.4)* —
**"It works locally and not remotely" is very often this**, and it is a line in the application's
configuration, not a network problem.

**Increment a test connection and see which rule's counter moves** *(§65.4)* — **And if none
does, the traffic is not reaching that device at all**, which is itself a finding.

**Shortening a DHCP lease does not create addresses** *(§65.4)* — **A /24 pool of 101 addresses
serving 145 devices is short by 44, permanently.** **A shorter lease reclaims idle addresses
faster**, which helps high turnover and does nothing for permanent occupancy. **Monitor pool
utilisation**, because exhaustion arrives silently and presents as an apparently random subset of
users failing.

**Query each configured DNS resolver individually** *(§65.4)* — **An aggregate test hides which
one is broken**, and intermittent DNS failure is usually one unhealthy resolver out of several.

**Clock skew presents as an authentication failure, not as a time problem** *(§65.4)* — **Kerberos
tolerates about five minutes; certificate validation fails outside the validity window; and log
correlation becomes impossible.** **"Check the clock" is a standing item in any authentication
investigation.**

**A consistently round delay is a timeout, and the value names it** *(§65.4)* — **5 seconds is a
DNS resolver timeout; 21 is Linux's TCP connection timeout; 30 is a common application one.**
**Nothing in a working network produces a consistent delay of exactly five seconds.**

**"The page loads and the images do not" is MTU** *(§65.4)* — Small request, large response.
Chapter 66 §66.3.

**Proving the network is not at fault requires evidence and should be produced without triumph**
*(§65.4)* — **Capture at both ends, show the request arriving and the response being sent, and
read the gap between them.** **"The network delivered it in 1 ms and the server took 8.8 seconds"
is a timestamp, not an assertion** — **and an engineer who uses it to win an argument will not be
invited to the next investigation.**
