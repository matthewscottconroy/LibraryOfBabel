# 26.1 Borrowing Bits

Subnetting is one idea. Once you have it, everything else in this chapter is arithmetic.

> **Take bits from the host portion and give them to the network portion.**

That is all. The rest of this section is what follows.

## The trade

An address has 32 bits, split into network and host. **The total is fixed.** Every bit
you give to the network is a bit taken from the host.

```
   /24:   ┌──────── network 24 ────────┬─── host 8 ───┐
          │                            │              │
          └────────────────────────────┴──────────────┘
                                        1 network, 254 hosts

   /26:   ┌──────── network 26 ──────────┬─ host 6 ─┐
          │                              │          │
          └──────────────────────────────┴──────────┘
                                        4 networks, 62 hosts each
```

**Two bits borrowed. Four subnets. Host count drops from 254 to 62.**

| Borrowed | Subnets (2^b) | Host bits | Hosts each (2^h − 2) | Total usable |
|---|---|---|---|---|
| 0 | 1 | 8 | 254 | 254 |
| 1 | 2 | 7 | 126 | 252 |
| 2 | **4** | 6 | **62** | 248 |
| 3 | 8 | 5 | 30 | 240 |
| 4 | 16 | 4 | 14 | 224 |
| 5 | 32 | 3 | 6 | 192 |
| 6 | 64 | 2 | **2** | 128 |

**Two formulas, and they are the whole chapter:**

$$\text{subnets} = 2^{b} \qquad \text{hosts per subnet} = 2^{h} - 2$$

where *b* = bits borrowed and *h* = host bits remaining.

## The cost

Look at the last column above. **Subnetting wastes addresses**, and the waste grows with
the number of subnets, because **each subnet loses two addresses** to its own network
and broadcast.

$$\text{total usable} = 2^{b} \times (2^{h} - 2) = 256 - 2 \times 2^{b}$$

| Subnets | Addresses lost to reservations |
|---|---|
| 1 | 2 |
| 4 | 8 |
| 16 | 32 |
| 64 | **128 — half the block** |

**This is a real cost and it is worth paying**, because what subnetting buys is worth
more than addresses:

- **Separate broadcast domains** — the argument of Chapter 17 §17.3 and Chapter 20 §20.1
- **Security boundaries** — a place to put policy
- **Fault isolation** — a smaller blast radius
- **Routing hierarchy** — aggregable structure (§26.3)

But it means **subnetting into tiny pieces is expensive.** Sixty-four /30 links out of a
/24 gives 128 usable addresses out of 256. Which is exactly why RFC 3021's /31 exists
(Chapter 25 §25.2).

## Choosing the size

The question is always: **how many hosts must this subnet hold, plus growth?**

Then round **up** to the next power of two, and add two for the reservations.

**Worked:**

| Requirement | Need (incl. 2) | Round to | Prefix | Capacity |
|---|---|---|---|---|
| 10 hosts | 12 | 16 | **/28** | 14 |
| 25 hosts | 27 | 32 | **/27** | 30 |
| 50 hosts | 52 | 64 | **/26** | 62 |
| 100 hosts | 102 | 128 | **/25** | 126 |
| 200 hosts | 202 | 256 | **/24** | 254 |
| 500 hosts | 502 | 512 | **/23** | 510 |
| 1000 hosts | 1002 | 1024 | **/22** | 1022 |
| 2 hosts (P2P) | 4 | 4 | **/30** | 2 |

**The trap at the boundary.** A requirement for exactly 30 hosts:

$$30 + 2 = 32 \longrightarrow \text{/27, capacity } 30$$

It fits **exactly**, with zero room. One additional device requires renumbering the
subnet — and renumbering is expensive because every static address, every DHCP
reservation, every firewall rule and every piece of documentation references it.

**Round up when you are near a boundary.** A /26 instead of a /27 costs 32 addresses and
saves a renumbering project. Given how cheap private address space is (Chapter 27 §27.1),
this is almost always the right call.

## The prefix table

**Memorise this.** It is the working reference for everything that follows.

| Prefix | Mask | Block size | Hosts | Blocks in a /24 |
|---|---|---|---|---|
| /24 | 255.255.255.0 | 256 | 254 | 1 |
| /25 | 255.255.255.128 | **128** | 126 | 2 |
| /26 | 255.255.255.192 | **64** | 62 | 4 |
| /27 | 255.255.255.224 | **32** | 30 | 8 |
| /28 | 255.255.255.240 | **16** | 14 | 16 |
| /29 | 255.255.255.248 | **8** | 6 | 32 |
| /30 | 255.255.255.252 | **4** | 2 | 64 |
| /31 | 255.255.255.254 | 2 | 2* | 128 |
| /32 | 255.255.255.255 | 1 | 1* | 256 |

And upward, where the borrowing happens in the third octet:

| Prefix | Mask | Block size | Hosts |
|---|---|---|---|
| /23 | 255.255.254.0 | 512 | 510 |
| /22 | 255.255.252.0 | 1,024 | 1,022 |
| /21 | 255.255.248.0 | 2,048 | 2,046 |
| /20 | 255.255.240.0 | 4,096 | 4,094 |
| /19 | 255.255.224.0 | 8,192 | 8,190 |
| /18 | 255.255.192.0 | 16,384 | 16,382 |
| /17 | 255.255.128.0 | 32,768 | 32,766 |
| /16 | 255.255.0.0 | 65,536 | 65,534 |

**Notice the pattern.** The mask value and the block size always sum to 256:

$$\text{block size} = 256 - \text{mask octet}$$

This is the **magic number**, and §26.2 makes it the basis of the fast method.

## Where the subnets start

Subnets begin at **multiples of the block size**, always, without exception.

**A /24 split into /26 (block size 64):**

| Subnet | Network | Range | Broadcast |
|---|---|---|---|
| 1 | `.0` | `.1` – `.62` | `.63` |
| 2 | `.64` | `.65` – `.126` | `.127` |
| 3 | `.128` | `.129` – `.190` | `.191` |
| 4 | `.192` | `.193` – `.254` | `.255` |

**0, 64, 128, 192.** Multiples of 64.

**Split into /27 (block size 32):**

`.0, .32, .64, .96, .128, .160, .192, .224` — multiples of 32.

**Split into /28 (block size 16):**

`.0, .16, .32, .48, .64, …, .240` — multiples of 16.

> **This is the entire fast method:** find the block size, count in multiples of it, and
> the address you are given falls in one of them.

## Subnet zero and the all-ones subnet

Two subnets that were historically special and no longer are — worth knowing because
older material and older exam questions treat them differently.

**Subnet zero** — the first subnet, whose subnet bits are all zero (`192.168.10.0/26`).
Originally forbidden by RFC 950, because it was ambiguous with the parent network
address.

**The all-ones subnet** — the last (`192.168.10.192/26`). Originally forbidden because
its broadcast address is the same as the parent network's broadcast.

**Both are legal and used now.** RFC 1878 (1995) made it explicit, and
`ip subnet-zero` has been the default on Cisco equipment since IOS 12.0.

**Why it matters:** old questions and old textbooks compute subnet counts as **2^b − 2**
instead of 2^b. If you see that formula, it is the historical convention. **Modern
practice is 2^b**, and Network+ expects the modern answer.

The host formula's minus-two is different and has **not** changed: the network and
broadcast addresses within a subnet remain unusable.

## Two ways to say the same thing

Subnetting problems come in two forms, and recognising which you have been given saves
time.

**"Given a prefix, how many subnets and hosts?"** Direct application of the two
formulas.

**"Given a host requirement, what prefix?"** Round up to a power of two and read the
table backwards.

**"Given an address and prefix, what subnet is it in?"** The §26.2 method.

The third is the one that appears most often under time pressure, and it is the one to
drill.

## What breaks here

**Forgetting that borrowing bits reduces host capacity.** The total is fixed.

**Sizing a subnet with no headroom.** 30 hosts in a /27 fits exactly and cannot grow.

**Using 2^b − 2 for the subnet count.** That is the pre-1995 convention. Use 2^b.

**Using 2^h for the host count.** The minus two is still correct.

**Assuming subnets can start anywhere.** They start at multiples of the block size.

> **Network+ note.** Objective 1.7 examines subnetting heavily. Over-learn: **subnets =
> 2^b, hosts = 2^h − 2**; **the prefix table with block sizes**; **block size = 256 −
> mask octet**; **subnets begin at multiples of the block size**; and **subnet zero is
> legal**. Expect several questions requiring the full computation under time pressure.
