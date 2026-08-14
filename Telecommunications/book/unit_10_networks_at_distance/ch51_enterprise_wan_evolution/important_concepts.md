# Chapter 51 — Important Concepts

Hub-and-spoke was a response to a pricing model, not a design preference *(§51.1)* — A full
mesh of *n* sites needs $n(n-1)/2$ circuits; twenty sites is 190 circuits and thirty is 435.
Every operational property of hub-and-spoke — the HQ bottleneck, the doubled latency, the
single point of failure — followed from an invoice.

CIR is oversubscription sold honestly *(§51.1)* — Traffic up to the committed rate is
guaranteed; above it is marked discard-eligible and carried if there is room. The carrier
oversubscribes deliberately and the statistics work (Chapter 9) — the customer gets far more
than the CIR most of the time and exactly the CIR when it matters.

FECN and BECN were explicit congestion notification in 1990, and were ignored *(§51.1)* —
The bits were defined, carriers set them, and almost no equipment acted on them. ECN's
slow adoption in IP is the same story told again (Chapter 38 §38.3).

An enterprise buying MPLS was buying a promise *(§51.1)* — Any-to-any without a mesh,
a contractual SLA with penalties, separation of overlapping RFC 1918 space, end-to-end class
of service, and someone contractually obliged to answer the phone. Chapter 13's account of
what packet switching gave up explains why that commanded a premium.

A 250× difference in cost per megabit is a different market, not a premium *(§51.1)* —
£40/Mb/s for MPLS against £0.16 for business broadband. "But the SLA" is a legitimate argument
that does not survive that ratio applied to traffic that mostly goes to the Internet.

Tromboning is bad in three independent ways *(§51.1)* — **Latency**, because the detour is
added to the most sensitive traffic; **cost**, because the expensive circuit carries traffic
never destined for the data centre; and **capacity**, because the central gateway must be sized
for every branch at once.

Only one of the three forces that broke MPLS was technical *(§51.1)* — The traffic pattern
changed; the other two were price and provisioning time. The technology that displaced
MPLS did not need to be better at anything MPLS was good at. It needed to be cheap and fast
to install.

SD-WAN's mechanisms all predate it *(§51.2)* — DMVPN built dynamic IPsec overlays a decade
earlier; policy-based routing existed; split tunnelling existed. What was missing was central
policy, per-application steering and continuous measurement — and combining them was the
whole contribution.

SD-WAN in one sentence: an overlay with a separated control plane and continuous telemetry
*(§51.2)* — Chapter 67's overlay, Chapter 68's control-plane separation, Chapter 61's IPsec,
plus probing every path several times a second.

Transport independence means every link carries traffic *(§51.2)* — Not one active and one
idle standby. Two 99.5% links give 99.9975% if genuinely independent — and a broadband
circuit and an MPLS tail from the same carrier down the same duct are not independent
(Chapter 56 §56.2).

Duplicating a voice call across two paths is absurd and correct *(§51.2)* — It doubles the
bandwidth for that flow and eliminates the loss. Chapter 5's rule: redundancy is cheap when
the payload is small and the consequence of loss is large.

Local breakout pays for the project and creates the security problem *(§51.2)* — Cloud
traffic leaves at the branch, the detour disappears, and forty branches become forty Internet
edges. SASE's answer is to move inspection to a provider's cloud edge — the right
architecture for a distributed organisation, and a single point of failure and of trust.

Zero-touch provisioning is usually what wins the business case *(§51.2)* — Not bandwidth
and not cost. A new site becomes a shipping problem rather than a project.

SD-WAN does not create an SLA on the public Internet *(§51.2)* — When every path is
congested it picks the least bad one. Sufficient for most traffic; for trading, industrial
control and some clinical systems it is not, and the honest design keeps a dedicated circuit
and says so.

The complexity moved; it did not vanish *(§51.2)* — Central policy replaces per-device
configuration, and adds a controller, an overlay, a measurement system and a vendor
relationship. There is no interoperability between SD-WAN vendors, so choosing one is
choosing a vendor for five to seven years.

Ask what happens when the controller is unreachable *(§51.2)* — Existing tunnels should keep
forwarding; no policy change, no new site, no visibility. Be sceptical of a vague answer.

Egress cost is usually the real reason for direct cloud interconnect *(§51.3)* — Roughly
$0.09/GB over the Internet against $0.02/GB direct, with ingress free both ways. The
crossover sits near 10–20 TB per month, and above it the circuit pays for itself
repeatedly. Model the egress before designing the connectivity.

The cloud is cheap to enter and expensive to leave *(§51.3)* — Ingress free, egress charged.
A commercial fact with architectural consequences: data gravity is real, and multi-cloud
traffic is charged as egress by both providers.

One direct connection carries no SLA worth the name *(§51.3)* — Providers commit to
availability only for multiple connections at multiple locations. And the fallback must
be configured, tested, and its capacity understood — a VPN will not carry what a 10 Gb/s
circuit did.

The direct connection and the SD-WAN are usually procured by different teams *(§51.3)* —
And the policy must know that cloud traffic takes the private path. The correct pattern is
to terminate the interconnect at a colocation facility that is also an SD-WAN hub.

An under-utilised interconnect port is the most expensive bandwidth you own *(§51.3)* — Port
charges are fixed. A 1 Gb/s port costs the same at 5% utilisation as at 95%.

Size the upstream first *(§51.4)* — Video calls are symmetric, so a branch on 500 Mb/s
down and 30 Mb/s up is constrained by the 30. It is the number that will fail and the number
nobody looks at, and it is Chapter 49 §49.2's asymmetry becoming an enterprise problem.

Video changed branch sizing within about two years *(§51.4)* — A branch sized in 2018 is
undersized now, and not because there are more users.

Redundancy design starts from what an hour costs *(§51.4)* — And two circuits from two
carriers may share a duct, a pole, a cabinet and a building entry. Ask where the fibre
enters the building — the commonest single point of failure is one duct through one wall.
LTE backup is genuinely independent and must be **tested monthly**, or it will not work.

Local breakout breaks source-IP restrictions *(§51.4)* — Many SaaS platforms, partners and
banking systems allow-list by address, and every branch now has a different, often dynamic
one. The defensible default: break out well-known SaaS locally via a cloud security service
with stable egress addresses, backhaul everything else.

The remote worker is a branch office with no budget and no site survey *(§51.4)* — You do
not control the broadband, the Wi-Fi, the household or the environment. You control the
device, the identity, the client and the policy — and that list is the argument for zero
trust.

A VPN that connects a user to a data centre so they can reach a cloud service is doing work
for no reason *(§51.4)* — The VPN exists to place a user "on the network", which was
meaningful when the applications were on it. In March 2020 concentrators sized for 10% of
staff met 100% of them, and the model's premise failed publicly.

If the user is not on your network and the application is not in your data centre, a
perimeter is not inadequate — it is irrelevant *(§51.4)* — Access is granted per
application, per session, on authenticated identity and verified device posture, with no
network access implied. Endpoint management thereby becomes a network dependency.

The only place left that you control is the endpoint *(§51.4)* — So the answer to "was it
slow?" must be measurement at the endpoint rather than recollection. "Please plug in a
cable" resolves a large fraction of remote complaints and is diagnostic either way.
