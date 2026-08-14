# Unit X — Networks at Distance

Ask most people to draw the Internet and they draw a cloud.

The cloud is a genuinely useful abstraction and it is also, for anyone who works with
networks professionally, a place where understanding stops. This unit replaces it
with a picture: physical cables in specific places, owned by specific companies,
interconnecting at specific buildings, under commercial arrangements that determine
which path your traffic takes far more than any shortest-path algorithm does.

The Internet has a geography and an economics, and both are visible in your
traceroutes once you know how to read them.

## What the unit answers

**What shape is it actually?** (Chapter 48.) Not a hierarchy, not a cloud, and not
the tidy tier-1/2/3 pyramid still drawn in older textbooks. A flattened mesh of
roughly seventy-five thousand autonomous systems, in which a handful of content
networks and CDNs peer so widely that most traffic now reaches its destination
without crossing a traditional transit backbone at all. Chapter 32 gave the protocol;
this chapter gives the map, plus who allocates addresses and how a standard becomes a
standard.

**How does the connection reach a building?** (Chapter 49.) The last mile is where
most of the world's telecommunications capital is buried, and where the interesting
engineering compromises live — because it is the one segment that cannot be shared
among many customers and therefore cannot amortise its cost. DSL extracting megabits
from telephone copper laid for voice; DOCSIS extracting gigabits from television
coax; PON splitting one fibre among thirty-two homes; and the satellite systems whose
performance is dictated entirely by orbital altitude and the speed of light.

**How does it cross continents?** (Chapter 50.) The carrier and optical layer: the
digital hierarchy inherited from Chapter 12, SONET's rings and their 50 ms
protection, DWDM multiplying a fibre's capacity by ninety-six, MPLS's label switching
— which is Chapter 13's virtual circuit idea, rebuilt on packet infrastructure — and
the submarine cables that are the physical Internet.

**How does an organisation connect its own sites?** (Chapter 51.) The enterprise WAN,
from leased lines through Frame Relay and MPLS to SD-WAN and direct cloud
interconnect, tracking a thirty-year shift in what a branch office actually needs to
reach.

**And how do we make a shared network behave when it matters?** (Chapter 52.) Quality
of service, and the honest account of what it can and cannot do — plus the other half
of the answer, which is not to send the traffic across the distance at all. Caching,
CDNs and anycast move the content to the user, and they have done more for perceived
Internet performance than every QoS mechanism ever deployed.

## The recurring economic fact

One idea runs through the whole unit and is worth having in advance, because it
explains decisions that look irrational from a purely technical view.

The cost of a communications link is overwhelmingly in the path, not the capacity.
Chapter 9 stated this; here is where it determines everything.

Digging a trench costs the same for one fibre or 144. Laying a submarine cable costs
several hundred million dollars in ships and permits, and the terminal electronics
that decide whether it carries 10 Tb/s or 250 Tb/s are a small fraction of that. A
cell tower costs the same whether it serves ten subscribers or a thousand.

Therefore: capacity is cheap and paths are expensive, and the entire industry's
structure follows. It is why upgrades are electronics-only wherever possible. It is
why incumbents with existing copper and duct have an advantage that no amount of
technical superiority overcomes. It is why "just run fibre to it" is a sentence that
sounds simple and costs a fortune. And it is why peering — which lets two networks
exchange traffic without either paying a third party for transit — is worth the
substantial effort of arranging.

## Reading a traceroute properly

By the end of this unit you should be able to look at a traceroute and see the
industry in it. The hop where your ISP's access network hands off to its core. The
airport code embedded in a router's reverse DNS name (`ae-1.lon-cr-1.example.net` is
in London). The point where the path leaves your provider and enters another
network's — visible as a change in naming convention and often in latency. The
transatlantic hop, identifiable because the RTT jumps by about 70 ms in one step and
no amount of engineering will reduce it. The IXP, sometimes named explicitly in the
hostname.

That reading is not a party trick. It is how you determine, during an outage, whether
a problem is yours, your provider's, or somebody else's — which is the first question
worth answering and the one that determines who you telephone.
