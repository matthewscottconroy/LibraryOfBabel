# Chapter 26 — Subnetting, CIDR, and VLSM

This is the chapter the rest of the book has been arranging.

Not because subnetting is intellectually deep — it is not; it is bitwise AND and
counting — but because it is the skill that most reliably separates people who
understand networks from people who have memorised facts about them, and because
the memorisation route genuinely does not work. Subnet charts are forgotten. The
binary is not, once it is properly acquired, and acquiring it properly takes one
determined afternoon.

## The problem, stated

You are given `172.16.0.0/16` — one network, 65,534 usable host addresses — and an
organisation with:

- Sales, 100 hosts
- Engineering, 500 hosts
- Manufacturing, 2,000 hosts
- Warehouse, 25 hosts
- Six point-to-point WAN links between sites, 2 hosts each
- Room to grow

Putting all of them in one /16 would technically work and would be a serious error:
one broadcast domain of 2,625 hosts, no ability to filter traffic between
departments, and a single spanning-tree failure taking down the entire company.

So the address space must be divided, and the division must give each group enough
addresses without wasting so many that you run out. That division is **subnetting**,
and the tool for doing it efficiently — different-sized subnets from one block — is
**variable-length subnet masking**.

## Borrowing bits, and the two counts that trade against each other

Subnetting takes bits from the host portion and gives them to the network portion.
Every bit moved doubles the number of subnets and halves the hosts in each.

Starting from `/16` with 16 host bits:

| Prefix | Subnet bits borrowed | Subnets | Host bits | Usable hosts each |
|---|---|---|---|---|
| /16 | 0 | 1 | 16 | 65,534 |
| /17 | 1 | 2 | 15 | 32,766 |
| /20 | 4 | 16 | 12 | 4,094 |
| /24 | 8 | 256 | 8 | 254 |
| /26 | 10 | 1,024 | 6 | 62 |
| /30 | 14 | 16,384 | 2 | 2 |

Two formulas, and they are Chapter 2 §2.1's log₂ argument in disguise:

$$\text{subnets} = 2^{s} \qquad \text{usable hosts} = 2^{h} - 2$$

where *s* is bits borrowed and *h* is host bits remaining. The −2 removes the
all-zeros host address (the **network address**, naming the subnet itself) and the
all-ones host address (the **broadcast address** for that subnet). Neither can be
assigned to a host.

The exception worth knowing: a `/31` has two addresses and, by the −2 rule, zero
usable hosts — which would make it useless. RFC 3021 permits `/31` on point-to-point
links specifically, where there is no need for a broadcast address because there is
exactly one possible recipient. Modern practice uses /31 for router-to-router links
and saves a great deal of address space; /30 remains common in older designs and in
equipment that does not support /31.

## The one worked example that matters

§26.2 does `192.168.10.70/27` completely, in binary, slowly, and then extracts every
shortcut from it. The result, which you should be able to produce in under thirty
seconds by the end of the chapter:

```
Address:    192.168.10.70    11000000.10101000.00001010.010 00110
Mask:       /27              11111111.11111111.11111111.111 00000
                                                          └┬┘└─┬─┘
                                            network bits ──┘    └── host bits

Network:    192.168.10.64    ...010 00000    (AND)
First host: 192.168.10.65
Last host:  192.168.10.94
Broadcast:  192.168.10.95    ...010 11111    (OR with inverted mask)
Next net:   192.168.10.96
Hosts:      2⁵ − 2 = 30
```

And then the shortcut, which is not a trick to memorise but a consequence to
recognise:

> **Block size = 256 − 224 = 32.** Subnets in the fourth octet therefore begin at
> 0, 32, 64, 96, 128, 160, 192, 224. The address .70 falls in the block starting
> at 64. The broadcast is one below the next block, 95. Done.

Chapter 2 §2.2 gave you that rule already, as an observation about mask octets.
Here it is doing work.

## CIDR: the same idea, upward

Subnetting divides a block into smaller ones. **Classless Inter-Domain Routing**,
RFC 1519 (1993), does the reverse: it combines adjacent blocks into a larger one, so
that a router can advertise one route instead of many.

If a provider holds `203.0.113.0/24` through `203.0.113.255/24`... more usefully,
if an organisation holds `198.51.100.0/24`, `198.51.101.0/24`, `198.51.102.0/24` and
`198.51.103.0/24`, those four /24s are contiguous and aligned, and can be advertised
as a single `198.51.100.0/22`. Four routing table entries become one.

CIDR was introduced in response to a genuine emergency. By 1992 the global routing
table was growing faster than router memory, Class B space was nearly exhausted, and
the projections had the Internet running out of both within a few years. CIDR
addressed all three problems at once — it allowed allocation on any bit boundary
(no more giving a Class B to an organisation needing 300 addresses), and it allowed
aggregation to hold the routing table down.

It bought roughly a decade. The rest was bought by NAT (Chapter 33). Then IPv4 ran
out anyway: IANA allocated its final blocks on 3 February 2011, and the regional
registries exhausted between 2011 and 2019.

## VLSM, and the discipline it requires

**Variable-length subnet masking** means using different prefix lengths within one
address block, sized to each subnet's actual need. It is how the problem at the top
of this chapter is solved: /21 for manufacturing, /23 for engineering, /25 for
sales, /27 for the warehouse, /31 for each WAN link.

The discipline VLSM requires is **allocate largest first**, and §26.4 shows why
allocating in the wrong order fragments the space so that a later large subnet
cannot be placed even though enough total addresses remain. It is exactly the memory
allocation problem, and it has exactly the same solution.

The second discipline is **allocate so that summarisation remains possible** — keep
each site's subnets contiguous and aligned so that the site can be advertised as one
prefix. An address plan that ignores this produces a routing table that cannot be
aggregated, which is a problem you will not notice until the network is large and
which is then very expensive to correct.

## What this chapter does

§26.1 covers borrowing bits and the two formulas, with the /31 exception.

§26.2 works the full binary example and derives the shortcuts from it, then drills.

§26.3 covers CIDR, supernetting, aggregation, and the exhaustion history.

§26.4 covers VLSM: the largest-first discipline, the alignment discipline, and a
complete worked address plan for the organisation above.

## By the end you will be able to

- Given any address and prefix, state the network address, broadcast address, first
  and last usable host, and host count — in under thirty seconds, without a chart.
- Given a required host count, choose the smallest sufficient prefix.
- Aggregate a set of contiguous prefixes into the shortest covering prefix, and
  determine whether a given set *can* be aggregated.
- Produce a complete VLSM plan for a stated organisation, allocated largest-first
  and aligned for summarisation.
- Recognise an address plan that will not summarise, and say what should have been
  done instead.
