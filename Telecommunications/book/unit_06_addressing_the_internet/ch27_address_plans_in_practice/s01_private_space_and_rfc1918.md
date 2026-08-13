# 27.1 Private Space and RFC 1918

Chapter 26 assumed you have addresses to allocate. This section is about where they come
from, and about the three ranges that essentially every network you will ever build uses.

## The three ranges

**RFC 1918**, March 1996:

| Range | Prefix | Addresses | Was |
|---|---|---|---|
| `10.0.0.0` – `10.255.255.255` | **`10.0.0.0/8`** | 16,777,216 | one class A |
| `172.16.0.0` – `172.31.255.255` | **`172.16.0.0/12`** | 1,048,576 | 16 class Bs |
| `192.168.0.0` – `192.168.255.255` | **`192.168.0.0/16`** | 65,536 | 256 class Cs |

**Memorise all three, including the awkward middle one.** The `172.16–172.31` range is
the most-forgotten and the most-examined, for the reason Chapter 25 §25.4 gave: it is a
classful fossil, sixteen class Bs, which is why the second octet range is 16–31 and the
prefix is /12.

**`172.32.5.1` is not private.** It is public and belongs to somebody. This is the
standard exam distractor and it is also a real configuration error.

## What "private" means

**Precisely:** these addresses are **not routed on the public Internet**. Every provider
filters them at their borders, by policy and by universal agreement.

**They are not:**

- **Secure.** A private address is not a security control (the same argument as VLANs,
  Chapter 20 §20.1). Anything on the same private network reaches them.
- **Unique.** Millions of organisations use `192.168.1.0/24` simultaneously. That is
  the point.
- **Hidden.** They appear in logs, in headers, in error messages, and in any traffic that
  leaks.
- **Unreachable.** They are reachable by anything with a route to them, which includes
  anything on your network and anything connected via a VPN.

**What they are** is *free, plentiful and locally meaningful* — and that combination is
why the modern Internet exists in the form it does.

## Why they exist

By 1996 the exhaustion projections were alarming. Chapter 25 §25.4's class B crisis had
been deferred by CIDR, and the underlying problem — four billion addresses for a
planet — had not gone away.

**The observation that produced RFC 1918:** an enormous fraction of connected hosts
**never need to be reached from the Internet at all.** A workstation, a printer, a
building sensor, a point-of-sale terminal — each initiates outbound connections and
accepts none.

**Giving each a globally unique address wastes one**, and the waste is at the scale of
billions of devices.

So: give them addresses that are unique **locally**, and translate at the boundary
(Chapter 33). One public address serves hundreds or thousands of private ones.

**The effect was profound.** IPv4 exhaustion, projected for the late 1990s, arrived at
IANA in **2011** and at the regional registries between 2011 and 2020. **RFC 1918 plus
NAT bought roughly fifteen years**, and it is the single largest reason IPv6's transition
has been so slow: the crisis that was supposed to force it was deferred past the point
where anyone felt urgency.

## Which range to use

A real decision, with real consequences.

### `10.0.0.0/8` — the default choice

**16.7 million addresses.** Enough for essentially any organisation, and structured
addressing (Chapter 26 §26.4) becomes easy because there is space to be generous.

```
   10 . site . subnet . host
   10 .  1   .   5    .  70
```

**Use this unless you have a specific reason not to.** The three-octets-of-structure
arrangement is readable, aggregable, and has room for everything.

**The one caution:** it is popular, so **VPN and merger collisions are common**. Two
organisations that both use `10.1.0.0/16` cannot connect their networks without
translating. §27.4 covers what to do about it.

### `172.16.0.0/12` — the underused one

**1 million addresses.** Perfectly good, less commonly used, and therefore **less likely
to collide** in a VPN or an acquisition.

**Its practical advantage is exactly its obscurity.** If you expect to connect to many
partner networks, choosing from `172.16–31` reduces the chance of a clash considerably —
and picking a *random* /16 within it reduces it further.

Its disadvantage is that people forget the range's boundaries and misconfigure
`172.32.x.x`.

### `192.168.0.0/16` — the small one

**65,536 addresses.** Every home router uses `192.168.0.0/24` or `192.168.1.0/24`.

**Avoid it in enterprise networks**, for two reasons:

- **Guaranteed collisions with home networks.** A remote worker on `192.168.1.0/24` at
  home, connecting by VPN to a corporate `192.168.1.0/24`, has a network that cannot
  work. Their local subnet always wins, so they lose access to the corporate range —
  and this is a very common support call.
- It is small enough to constrain a hierarchical plan.

**Use `10/8` or `172.16/12` for anything an employee might VPN into.**

## Other reserved ranges

Beyond RFC 1918, several blocks are reserved and worth recognising.

### `100.64.0.0/10` — Carrier-Grade NAT (RFC 6598)

**4 million addresses**, for a specific problem: a service provider that has run out of
public addresses must give customers private ones — but the customer's own network is
already using RFC 1918 space, and the two must not collide.

So CGNAT space is **neither public nor RFC 1918**. It exists purely to be the provider's
private range, distinct from the customer's.

**Recognising it matters operationally:** if your WAN interface holds a `100.64.x.x`
address, **you are behind carrier-grade NAT**. You have no public address at all, inbound
connections are impossible, port forwarding cannot work, and some applications will
behave strangely. This is increasingly common on mobile and residential broadband.

**Do not use `100.64.0.0/10` internally.** It will collide with your provider.

### Documentation ranges (RFC 5737)

| Range | Name |
|---|---|
| `192.0.2.0/24` | TEST-NET-1 |
| `198.51.100.0/24` | TEST-NET-2 |
| `203.0.113.0/24` | TEST-NET-3 |

**For examples, documentation and teaching.** They are guaranteed never to be assigned,
so an example using them cannot accidentally reference a real network.

**This book uses them throughout** for anything representing a public address — which is
why `203.0.113.x` keeps appearing.

**Use them in your own documentation.** Writing a diagram with a real public address
someone else owns is a small mistake that occasionally becomes a large one.

### Benchmarking (RFC 2544)

`198.18.0.0/15` — for network device performance testing. Reserved so that benchmark
traffic cannot escape into the Internet.

### The rest

| Range | Purpose |
|---|---|
| `0.0.0.0/8` | "this network" |
| `127.0.0.0/8` | **loopback — the whole /8** |
| `169.254.0.0/16` | link-local / APIPA (§27.2) |
| `224.0.0.0/4` | multicast (§27.3) |
| `240.0.0.0/4` | reserved, formerly "class E" |
| `255.255.255.255/32` | limited broadcast |

**`240.0.0.0/4` is 268 million addresses sitting unused.** Proposals to release it
recur every few years and have never succeeded, because essentially every IP stack ever
written rejects addresses in that range as invalid — a hard-coded check that would have
to be removed from billions of devices. **It is a perfect illustration of Chapter 23
§23.4's point that IP cannot be changed**: a change requiring no protocol modification at
all, only the removal of a check, is still infeasible.

## Choosing for a real network

**Do not use `192.168.1.0/24`.** Everyone does, and the collision cost is real.

**Pick a range with room and structure it.** For a mid-sized organisation:

```
   10.0.0.0/8
     ├── 10.0.0.0/16      reserved / infrastructure
     ├── 10.1.0.0/16      Site 1
     ├── 10.2.0.0/16      Site 2
     …
     ├── 10.100.0.0/16    lab / test
     ├── 10.200.0.0/16    DMZ
     └── 10.255.0.0/16    WAN links
```

**Pick something unlikely if you expect to connect to partners.** `10.147.0.0/16` is as
valid as `10.1.0.0/16` and far less likely to collide. **Randomising the second octet
costs nothing and prevents a real class of problem** — the same reasoning that makes a
random VXLAN or AS number choice worthwhile.

**Document what you reserved and why.** Chapter 26 §26.4's point: undocumented free space
is unusable.

## What breaks here

**Using `192.168.1.0/24` for a corporate network.** Every VPN user whose home router uses
the same range loses access.

**Overlapping ranges after a merger.** Both organisations use `10.1.0.0/16`. The fix is
NAT between them (Chapter 33 §33.3) or renumbering one side, and both are expensive.

**Assuming `172.x.x.x` is private.** Only `172.16` through `172.31`.

**Using `100.64.0.0/10` internally.** It collides with carrier-grade NAT and produces
faults that appear to be the provider's.

**Expecting inbound connections behind CGNAT.** There is no public address to forward to.

**Treating private addressing as security.** It is not, and Chapter 33 §33.3 makes the
argument properly.

> **Network+ note.** Objective 1.7 expects the private ranges and the special-purpose
> ranges. Over-learn: **`10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`**; that
> **`172.32.x.x` is public**; **`100.64.0.0/10` is CGNAT**; **`127.0.0.0/8` is
> loopback**; and **`169.254.0.0/16` means DHCP failed**. These appear as direct recall
> and as distractors in scenario questions.
