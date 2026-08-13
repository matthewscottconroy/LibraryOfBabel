# Chapter 21 — Why Layering Exists

In April 1956, a converted tanker called the *Ideal X* sailed from Newark to
Houston carrying fifty-eight aluminium boxes. Malcom McLean, a trucking magnate
who had grown tired of watching his lorries sit for days at docks while
longshoremen moved cargo piece by piece, had bought a shipping line in order to
prove a point.

The point was not about boxes. Boxes are not an invention. The point was about the
*interface*: if every crane, lorry chassis, railway wagon and ship hold in the
world agrees on one set of dimensions and one corner-casting fitting, then the
contents cease to matter to any of them. A crane that can lift a container of
televisions can lift a container of coffee, because it has never needed to know the
difference.

Loading cost fell from around $5.86 per ton to about $0.16. That is not an
improvement; it is a different industry. And the mechanism was purely the
standardisation of an interface, with a deliberate refusal to standardise anything
on either side of it.

That is layering, and it is why this chapter starts on a dock rather than in a
protocol stack.

## The multiplication argument

Formally. Let there be *m* applications and *n* network technologies.

**Without a common abstraction**, each application must be implemented against each
network's peculiarities. Email for Ethernet, email for Wi-Fi, email for satellite,
email for the token ring in the accounting department. File transfer likewise.
Total: *m* × *n* implementations, and a new network technology is unusable until
all *m* applications have been ported to it — which means, in practice, that new
network technologies do not get adopted.

**With a common abstraction**, each application is written once against the
abstraction and each network is implemented once to provide it. Total: *m* + *n*.
A new network technology costs one unit of work and instantly runs every existing
application. A new application costs one unit of work and instantly runs over
every existing network.

For *m* = 50 and *n* = 10, that is 500 versus 60. The ratio grows without bound,
and the *strategic* difference is larger than the arithmetic suggests: in the first
world, innovation at either end requires permission and coordination from the
other; in the second, it does not. This is why you can invent a new application
today and deploy it over networks whose owners have never heard of you, and it is
the single most important structural property of the Internet.

## What a layer actually is

Three components, and the distinctions between them matter when you read a
standard.

A **service** is what a layer offers to the layer above, stated in terms of *what*
without *how*: "I will deliver a packet to that address, probably." A service is a
contract.

An **interface** is the mechanical means by which the layer above requests the
service — a socket API call, a function invocation, a hardware register.

A **protocol** is the agreement between *peers* at the same layer on different
machines, about the messages they exchange in order to provide the service. Note
the direction: services and interfaces run vertically within one machine; protocols
run horizontally between machines.

The classical vocabulary for this is the **service primitive** — request,
indication, response, confirm — which comes from the OSI documents. You will
encounter it in standards and rarely elsewhere, but the vertical/horizontal
distinction it encodes is worth internalising, because confusing a protocol with an
interface is a reliable source of muddle.

## What it costs

This is the section that most treatments omit, and it is the one that makes the
difference between reciting the model and understanding it.

**Header overhead.** Each layer adds its own header. Chapter 3 §3.1 computed it: a
1,460-byte payload arrives inside 1,538 bytes on the wire, so 5% is layering tax.
For a small VoIP packet it is 33%. This is real bandwidth, permanently spent.

**Duplicated function.** Ethernet has a CRC. IP has a header checksum. TCP has a
checksum. TLS has a MAC. Four integrity mechanisms on the same bytes, because each
layer was designed not to trust the others. Sometimes this is prudent defence in
depth; sometimes it is waste. The IPv6 designers concluded it was waste and removed
the IP header checksum entirely, on the grounds that Layer 2 and Layer 4 both
check anyway.

**Information hiding that hurts.** The abstraction that makes TCP portable also
means TCP cannot tell why a packet was lost. Congestion? Corruption? A radio
fade? TCP assumes congestion and slows down, which is correct on a wire and
disastrously wrong on a lossy wireless link — a mismatch that consumed a great deal
of research effort in the 1990s and that Chapter 38 revisits. The layer below knows
the answer and has no way to say it.

**Latency.** Each boundary crossing costs a copy, a context switch, or both. This
is why high-performance networking spends so much energy on *bypassing* layers —
DPDK, RDMA, kernel bypass, TCP offload — all of which are admissions that the
abstraction's cost is sometimes intolerable.

## And the violations

§21.4 catalogues the places where layering is deliberately broken in production
systems, because pretending it is not broken leaves you unable to explain the
network you actually have.

NAT rewrites Layer 3 and Layer 4 headers and must understand Layer 7 for some
protocols. Deep packet inspection makes Layer 3 decisions from Layer 7 content.
QUIC moves transport into userspace and encrypts its own headers specifically to
prevent middleboxes from doing this. Load balancers terminate connections at Layer
4 or Layer 7 and originate new ones. Every one of these is a layering violation,
every one is deployed at enormous scale, and every one exists because a layer
boundary prevented something that someone needed.

The mature position: layering is a discipline that pays for itself many times over,
and violations should be understood as *purchases* — you are buying a capability
with future flexibility, and it is worth knowing the price you paid.

## By the end you will be able to

- State the *m* × *n* argument and apply it to a non-networking example.
- Distinguish service, interface, and protocol, and identify each in a
  specification.
- Enumerate four distinct costs of layering and give a concrete instance of each.
- Explain why TCP's inability to distinguish loss causes is a layering consequence
  rather than a bug.
- Identify layer violations in a described system and state what each buys.
