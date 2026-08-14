# 28.2 Notation and Address Types

IPv6 addresses look intimidating and are not. There are three notation rules and about
five address types that matter, and both lists are short enough to learn in an afternoon.

## The notation

**128 bits, written as eight groups of 16 bits, in hexadecimal, separated by colons.**

```
   2001:0db8:0000:0000:0000:ff00:0042:8329
   └──┘ └──┘ └──┘ └──┘ └──┘ └──┘ └──┘ └──┘
     1    2    3    4    5    6    7    8

   8 groups × 16 bits = 128 bits
```

Each group is four hexadecimal digits — a **hextet**, or "quibble", or just "group";
the terminology never settled.

**Why hexadecimal?** Because 128 bits in dotted decimal would be sixteen numbers
(`32.1.13.184.0.0...`) and unreadable, and because **each hex digit is exactly four
bits**, so the prefix boundaries stay visible — which is the same argument Chapter 25
§25.1 made for dotted decimal.

## Rule 1 — Drop leading zeros in each group

```
   2001:0db8:0000:0000:0000:ff00:0042:8329
   2001:db8:0:0:0:ff00:42:8329
```

`0db8` → `db8`, `0000` → `0`, `0042` → `42`.

**Leading only.** `8329` stays `8329`; `ff00` stays `ff00`.

## Rule 2 — Replace one run of zero groups with `::`

```
   2001:db8:0:0:0:ff00:42:8329
   2001:db8::ff00:42:8329
```

The `::` stands for *"as many all-zero groups as are needed to make eight"*.

**The critical restriction: `::` may appear only once.**

```
   2001:0:0:1:0:0:0:1

   2001::1:0:0:0:1     ✓  (the first run)
   2001:0:0:1::1       ✓  (the second run — the longer one, and preferred)
   2001::1::1          ×  ambiguous — how many groups in each gap?
```

Two `::` would be unparseable, because there would be no way to know how to distribute
the missing groups.

**When two runs are equal length, compress the first.** When they differ, **compress the
longer** (RFC 5952).

## Rule 3 — The canonical form (RFC 5952)

For consistency in configuration and logs:

- **Lowercase** hexadecimal
- **Compress the longest** run of zeros
- **Do not** use `::` for a single zero group — write `:0:`
- Compress **as much as possible**

```
   2001:DB8:0:0:1:0:0:1     →   2001:db8::1:0:0:1
   2001:db8::0:1            →   2001:db8::1
   2001:db8:0:1:1:1:1:1     →   2001:db8:0:1:1:1:1:1   (single zero, no ::)
```

**Why it matters:** two spellings of the same address will not match a string comparison,
which breaks log searches, ACL matching, and any tool doing textual rather than numeric
comparison. **Follow the canonical form and the problem disappears.**

## Worked compressions

| Full | Compressed |
|---|---|
| `2001:0db8:0000:0000:0000:0000:0000:0001` | `2001:db8::1` |
| `fe80:0000:0000:0000:0204:61ff:fe9d:f156` | `fe80::204:61ff:fe9d:f156` |
| `0000:0000:0000:0000:0000:0000:0000:0001` | **`::1`** (loopback) |
| `0000:0000:0000:0000:0000:0000:0000:0000` | **`::`** (unspecified) |
| `2001:0db8:0000:0001:0000:0000:0000:0001` | `2001:db8:0:1::1` |
| `ff02:0000:0000:0000:0000:0000:0000:0001` | `ff02::1` |

## Prefixes

**Identical to IPv4's CIDR notation**, and this is the one place IPv6 is genuinely
simpler:

```
   2001:db8:1234:5678::1/64
                        └─┘
                      prefix length in bits
```

**There is no dotted-decimal mask form.** Prefix length only. One notation, no
conversion table, no mask octets to memorise.

**The prefix lengths that matter:**

| Prefix | Meaning |
|---|---|
| **/64** | **one subnet** — essentially always |
| /56 | a home or small site |
| /48 | an organisation |
| /32 | an ISP allocation |
| /128 | a single host |
| /127 | a point-to-point link (RFC 6164) |

**Chapter 26 does not apply.** There is no VLSM arithmetic, no host-count calculation, no
sizing decision. **Every subnet is a /64.** The design question becomes *how many subnets
do I need*, and the answer is always *fewer than 65,536, so a /48 is enough*.

## The address types

### Global Unicast — `2000::/3`

**The public, routable addresses.** Equivalent to IPv4 public space.

`2000::/3` means the first three bits are `001`, so **anything beginning `2` or `3`**.

```
   2001:db8:1234:5678:0000:0000:0000:0001
   └────── /48 ──────┘└─┘└───── interface ID ─────┘
     from your ISP    subnet     64 bits
                       ID
```

**The structure is deliberate:**

| Bits | Field | Assigned by |
|---|---|---|
| 0–47 | Global routing prefix | ISP / registry |
| 48–63 | **Subnet ID** | **you** — 65,536 subnets |
| 64–127 | Interface ID | the host, usually |

**The /64 boundary is architectural**, not conventional. SLAAC (Chapter 29 §29.2)
requires 64 host bits, and a great deal of IPv6 assumes it. Using anything else breaks
autoconfiguration and gains nothing.

**`2001:db8::/32` is the documentation prefix** (RFC 3849) — IPv6's equivalent of
`203.0.113.0/24`, and it appears throughout this book.

### Link-Local — `fe80::/10`

**Every IPv6 interface has one, always**, whether or not anything is configured.

```
   fe80::204:61ff:fe9d:f156
```

**Properties:**

- **Automatically generated**, with no DHCP, no router and no configuration
- **Never forwarded by a router** — valid on one link only
- **Required** — NDP, router discovery and OSPFv3 all use it (Chapter 18 §18.4)

**The comparison that matters:** IPv4's `169.254.x.x` is a **failure signal**
(Chapter 27 §27.2). **IPv6's `fe80::` is normal.** An interface with only a link-local
address has not failed — it simply has no global address yet, which may be correct.

**The scope identifier.** Because every interface has a link-local address and they may
be identical, you must say which interface:

```
   ping6 fe80::1%eth0
                └───┘
              the zone / scope ID — required for link-local
```

**Forgetting the `%interface` is the most common IPv6 command-line error.**

### Unique Local — `fc00::/7`, in practice `fd00::/8`

**IPv6's RFC 1918.** Private, not routed on the Internet, for internal use.

```
   fd  +  40 random bits  +  16-bit subnet  +  64-bit interface
   fd12:3456:789a:0001::1
   └┬┘ └──────┬──────┘
   fd    RANDOM — generate it, do not choose it
```

**The random 40 bits are the design's cleverest part.** Chapter 27 §27.1's collision
problem — two organisations both using `10.1.0.0/16` and unable to merge — is solved
probabilistically: **generate a random /48 and the chance of collision is negligible.**

**Generate it properly:**

```bash
# Do not use fd00::/8 or fd12:3456::. Generate one:
printf 'fd%02x:%04x:%04x::/48\n' $((RANDOM%256)) $((RANDOM%65536)) $((RANDOM%65536))
```

**Everyone using `fd00::1` recreates the `192.168.1.0/24` problem exactly.**

`fc00::/8` — the other half of `fc00::/7` — was intended for centrally-assigned ULAs and
was never defined. Only `fd00::/8` is used.

### Multicast — `ff00::/8`

**Anything beginning `ff`.** IPv6 has **no broadcast at all** — every former use of
broadcast is a multicast group.

| Address | Meaning |
|---|---|
| **`ff02::1`** | **all nodes on this link** (replaces broadcast) |
| **`ff02::2`** | **all routers on this link** |
| `ff02::5`, `::6` | OSPFv3 routers, DRs |
| `ff02::9` | RIPng |
| `ff02::fb` | mDNS |
| `ff02::1:2` | all DHCPv6 servers and relays |
| **`ff02::1:ffXX:XXXX`** | **solicited-node** (Chapter 18 §18.4) |

**`ping6 ff02::2%eth0` asks every router on the link to identify itself.** There is no
useful IPv4 equivalent, and it is the fastest way to answer *"what is routing this
segment?"*

The second nibble encodes **scope**: `ff02` is link-local, `ff05` site-local, `ff0e`
global. So `ff02::1` cannot leave the link by construction.

### The special ones

| Address | Meaning |
|---|---|
| **`::1`** | **loopback** — the whole of IPv4's `127.0.0.0/8` reduced to one address |
| **`::`** | unspecified — "no address yet", used as a source during DAD |
| `::ffff:192.0.2.1` | **IPv4-mapped** — how an IPv4 address appears to a dual-stack socket |

**IPv4-mapped addresses matter in practice.** A dual-stack server listening on `::` sees
IPv4 clients as `::ffff:a.b.c.d`, which appears in logs and confuses access-control lists
that expect a dotted quad.

### Anycast

**No special range.** Exactly as in IPv4 (Chapter 27 §27.3) — assign the same address to
several interfaces and let routing choose.

One reserved form: the **subnet-router anycast address**, which is the /64 with all
interface bits zero (`2001:db8:1:1::`), reaching any router on that subnet.

## No broadcast

Worth stating separately because it is examined and because it is a genuine improvement.

**IPv6 has no broadcast address.** Every use was replaced:

| IPv4 broadcast use | IPv6 replacement |
|---|---|
| ARP | **NDP** via solicited-node multicast |
| DHCP discovery | DHCPv6 to `ff02::1:2` |
| "everyone on this link" | `ff02::1` |
| Routing protocol hellos | protocol-specific multicast groups |

**The benefit** (Chapter 18 §18.4): a broadcast interrupts every host's CPU; a
solicited-node multicast is filtered **in the NIC hardware** and interrupts roughly one.
On a large segment this is the difference between overhead that scales with host count
and overhead that does not.

## Reading an address

```
   2001:db8:acad:1::1/64

   2001    →  begins with 2  →  GLOBAL UNICAST, public
   2001:db8 →  documentation prefix
   :acad:1 →  the site's subnet structure
   ::1     →  interface ID 1 — manually configured, since SLAAC
              would produce a long random-looking value
   /64     →  one subnet, as always
```

**Recognition, at a glance:**

| Starts with | Type |
|---|---|
| `2` or `3` | global unicast — public |
| **`fe80`** | **link-local** |
| **`fd`** | **unique local — private** |
| **`ff`** | **multicast** |
| `::1` | loopback |
| `::` | unspecified |

**That table is most of what you need.** Four prefixes, and any IPv6 address you meet can
be classified instantly.

## What breaks here

**Two `::` in one address.** Invalid and unparseable.

**Forgetting `%interface` with a link-local address.** The most common command-line
error.

**Using anything other than /64 for a subnet.** Breaks SLAAC, gains nothing.

**Everyone choosing `fd00::/8`.** Recreates the `192.168.1.0/24` collision problem.
**Generate the 40 bits randomly.**

**Non-canonical forms in configuration.** `2001:DB8::1` and `2001:db8:0:0:0:0:0:1` are
the same address and will not match as strings.

**Treating `fe80::` as a failure.** It is normal. Only IPv4's `169.254` means failure.

> **Network+ note.** Objective 1.8 expects IPv6 notation, compression and address types.
> **This is examined directly**, usually as "compress this address" or "what type is
> this?" Over-learn: **the compression rules**; **`::` once only**; **`fe80::/10`
> link-local, `fd00::/8` unique local, `ff00::/8` multicast, `2000::/3` global**;
> **`::1` loopback**; and **IPv6 has no broadcast**.
