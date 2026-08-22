# Chapter 48 — Exercises

## A. Recall

**A1.** Define a tier 1 network without using the word "tier", and say how many there are
approximately.

**A2.** State what changed to make the tier pyramid an inaccurate description of traffic.

**A3.** Give the defining difference between transit and peering in one sentence, expressed as
a routing filter.

**A4.** Describe 95th-percentile billing: the sample interval, the number of samples per
month, and what is discarded.

**A5.** What is an IXP, and what does it not do?

**A6.** Distinguish PA from PI address space, and state the cost of each to the global routing
table.

**A7.** Name the five RIRs and their regions.

**A8.** State the difference between an allocation and an assignment.

**A9.** What does "rough consensus and running code" mean, and who said it?

**A10.** Name four RFC categories and say which one most deployed protocols occupy.

**A11.** State which body standardises each of: Ethernet, TCP, LTE, HTML, the DNS root zone.

**A12.** Which section of an RFC states what the protocol does not protect against?

## B. Apply

**B1.** A network's transit port is sampled every five minutes for a 30-day month.

(a) How many samples are taken?
(b) How many are discarded before billing?
(c) How many hours per month does that represent?
(d) The network runs a two-hour backup at full line rate every night. Is it billed for it?
Show the arithmetic.

**B2.** An ISP's 95th percentile is 8 Gb/s and transit costs \$0.30 per Mb/s per month.

(a) What is the monthly transit bill?
(b) Joining the local IXP would move 60% of traffic off transit. What is the new bill?
(c) The IXP port costs \$800/month and colocation \$300/month. Is peering worth it on cost
alone?
(d) Traffic is growing 40% a year. Recompute (a)–(c) for two years' time and state when the
decision changes.

**B3.** A network sends 9 Gb/s and receives 2 Gb/s at its transit port.

(a) Which figure is billed under the usual convention?
(b) The network wants to reduce its bill. Which direction must it change, and give two
mechanisms.

**B4.** An IXP has 400 participants.

(a) How many bilateral cables would full-mesh private interconnection require?
(b) How many IXP ports achieve the same reachability?
(c) Express the ratio, and state the assumption that makes the comparison fair.

**B5.** Look up `8.8.8.8`, `1.1.1.1` and one address from your own network using WHOIS or
RDAP. For each, record: the RIR, the registered organisation, the origin AS and the abuse
contact. Note any case where the registered country misleads about the server's location.

**B6.** An LIR receives a /32 of IPv6.

(a) How many /48 end sites can it assign?
(b) How many /56 home customers?
(c) The LIR has 2 million customers and wants to give each a /56. Is a /32 sufficient? If
not, what should it request?

**B7.** For each RFC, state its current status and whether it has been obsoleted: 791, 793,
1918, 2460, 5246, 8446, 9293. Use the RFC Editor's index.

## C. Analyse

**C1.** The chapter argues that transit remains essential while carrying a minority of bytes.
Explain the apparent contradiction, and describe what would happen to an access network that
dropped transit and kept only its peering.

**C2.** Analyse the traffic-ratio criterion used in peering negotiations. Whose interest does
it serve, what does it claim to measure, and is the claim technically sound? Argue both sides
before concluding.

**C3.** The IPv4 transfer market accelerates routing table growth. Explain the mechanism
precisely, and say why a technically identical amount of address space allocated by an RIR
would not have the same effect.

**C4.** The IETF has no membership, no fees and no voting. Identify three consequences of this
that are advantages and three that are disadvantages, and state which you consider decisive.

**C5.** Compare the accessibility of IETF and IEEE specifications, and argue whether the
difference has affected which protocols got implemented. Use an example from Chapters 44 or 47.

**C6.** In 2021 a single CDN's configuration error made thousands of unrelated websites
unreachable within seconds. Reconcile this with the claim that the Internet has no single
point of failure. Is the claim wrong, or is it about something else?

**C7.** "Proposed Standard" is the status of TLS 1.3, HTTP/2 and QUIC. Explain why the formal
progression to Internet Standard is rarely completed, and what an engineer should use instead
of status to judge a protocol's maturity.

## D. Design

**D1.** You are the network architect for a regional ISP with 60,000 subscribers, currently
buying all transit from two providers. Design the interconnection strategy: which IXPs, which
PNIs, what transit to retain and why, and how you would sequence the work over three years.
Justify each element on cost, latency or resilience explicitly.

**D2.** A growing SaaS company currently uses its cloud provider's addresses. Argue for or
against obtaining PI space and an AS number, addressing: cost, RIR policy, operational burden,
what it enables, and what it obliges the company to do that it currently does not.

**D3.** Design the IPv6 addressing policy for an ISP that has received a /29. Specify what
each class of customer receives, how the space is structured for aggregation, and how much
headroom you leave. State your assumptions about growth.

**D4.** A content company must decide between (i) building its own backbone and peering
directly, (ii) using a commercial CDN, and (iii) buying transit in each region. Construct the
comparison, including the non-cost factors, and recommend one for a company serving 50 million
users across three continents.

## E. Troubleshoot

**E1.** Traffic to a major streaming service suddenly begins routing through a transit
provider on another continent, and users report buffering. Give the likely cause and the three
commands you would run first.

**E2.** A transit invoice doubles with no change in user numbers. Describe your investigation
in order, and give three plausible causes ranked by likelihood.

**E3.** A peer's BGP session shows as established but no prefixes are being received. Give
four possible causes and the command that distinguishes them.

**E4.** Your abuse desk receives complaints about traffic from a prefix that WHOIS says
belongs to your organisation, but you do not recognise the address. Describe how you would
determine whether this is a sub-delegation, a lease, or a hijack.

**E5.** Users complain that a website is slow. It resolves quickly, pings in 4 ms, and loads
in 12 seconds. What have you ruled out, and where do you look next?

**E6.** After changing transit providers, a customer's PI prefix is unreachable from part of
the Internet but reachable from the rest. Give the two most likely causes and how to confirm
each.

## F. Extend

**F1.** Use a looking glass or a public route collector (RIPE RIS, RouteViews) to examine the
AS paths to your own network's prefix from four different vantage points. Identify your
transit providers, any peers visible in the paths, and write a short description of your
network's position in the topology.

**F2.** Register for a free account on a BGP monitoring service, or use `bgp.tools` /
`stat.ripe.net`, and produce a diagram of the interconnection of one large content network in
your country. Note which IXPs it is present at.

**F3.** Read the peering policies of three large networks (they are public documents). Compare
their stated requirements, and write a paragraph on what the differences reveal about each
network's commercial position.

**F4.** Attend or watch a recording of an IETF working group session, or read one month of a
working group's mailing list archive. Write a page describing how disagreement was handled and
whether the process resembled the description in §48.4.

**F5.** Track the current market price of IPv4 address space from a broker's published data.
Plot the price history if available, and write a short analysis of what the trend implies for
IPv6 adoption economics.

**F6.** Find an RFC that documents a single vendor's protocol as Informational. Explain how a
reader could mistake it for a community standard, and what in the document indicates
otherwise.
