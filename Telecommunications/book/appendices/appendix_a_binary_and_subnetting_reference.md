# Appendix A — Binary, Hex, and Subnetting Reference

Everything here is derived in Chapter 2 and Chapter 26. This appendix exists so that
you do not have to re-derive it under time pressure, and so that you can check your
working. It is not a substitute for the derivations; a chart you cannot reconstruct is
a chart you will misremember.

---

## A.1 Powers of Two

| *n* | 2ⁿ | | *n* | 2ⁿ |
|---|---|---|---|---|
| 0 | 1 | | 16 | 65,536 |
| 1 | 2 | | 17 | 131,072 |
| 2 | 4 | | 18 | 262,144 |
| 3 | 8 | | 19 | 524,288 |
| 4 | 16 | | 20 | 1,048,576 |
| 5 | 32 | | 21 | 2,097,152 |
| 6 | 64 | | 22 | 4,194,304 |
| 7 | 128 | | 23 | 8,388,608 |
| 8 | 256 | | 24 | 16,777,216 |
| 9 | 512 | | 25 | 33,554,432 |
| 10 | 1,024 | | 26 | 67,108,864 |
| 11 | 2,048 | | 27 | 134,217,728 |
| 12 | 4,096 | | 28 | 268,435,456 |
| 13 | 8,192 | | 29 | 536,870,912 |
| 14 | 16,384 | | 30 | 1,073,741,824 |
| 15 | 32,768 | | 32 | 4,294,967,296 |

**Octet place values** — the row to know by recognition:

```
 128   64   32   16    8    4    2    1
```

---

## A.2 Hexadecimal

| Hex | Bin | Dec | | Hex | Bin | Dec |
|---|---|---|---|---|---|---|
| 0 | 0000 | 0 | | 8 | 1000 | 8 |
| 1 | 0001 | 1 | | 9 | 1001 | 9 |
| 2 | 0010 | 2 | | A | 1010 | 10 |
| 3 | 0011 | 3 | | B | 1011 | 11 |
| 4 | 0100 | 4 | | C | 1100 | 12 |
| 5 | 0101 | 5 | | D | 1101 | 13 |
| 6 | 0110 | 6 | | E | 1110 | 14 |
| 7 | 0111 | 7 | | F | 1111 | 15 |

One hex digit = one nibble = 4 bits. Two hex digits = one byte. Conversion is a
lookup, never a calculation.

---

## A.3 The Nine Legal Mask Octets

A contiguous mask octet is always a run of 1s followed by 0s. Only these nine values
can legally appear:

| Binary | Decimal | Bits set | Block size |
|---|---|---|---|
| `00000000` | 0 | 0 | 256 |
| `10000000` | 128 | 1 | 128 |
| `11000000` | 192 | 2 | 64 |
| `11100000` | 224 | 3 | 32 |
| `11110000` | 240 | 4 | 16 |
| `11111000` | 248 | 5 | 8 |
| `11111100` | 252 | 6 | 4 |
| `11111110` | 254 | 7 | 2 |
| `11111111` | 255 | 8 | 1 |

**Block size = 256 − mask octet value.** Subnet boundaries fall at multiples of the
block size. Any other value is a typo.

---

## A.4 Prefix Length Reference

| CIDR | Dotted decimal | Wildcard | Total addrs | Usable hosts | Notes |
|---|---|---|---|---|---|
| /8 | 255.0.0.0 | 0.255.255.255 | 16,777,216 | 16,777,214 | old Class A |
| /12 | 255.240.0.0 | 0.15.255.255 | 1,048,576 | 1,048,574 | RFC 1918 172.16/12 |
| /16 | 255.255.0.0 | 0.0.255.255 | 65,536 | 65,534 | old Class B |
| /17 | 255.255.128.0 | 0.0.127.255 | 32,768 | 32,766 | |
| /18 | 255.255.192.0 | 0.0.63.255 | 16,384 | 16,382 | |
| /19 | 255.255.224.0 | 0.0.31.255 | 8,192 | 8,190 | |
| /20 | 255.255.240.0 | 0.0.15.255 | 4,096 | 4,094 | |
| /21 | 255.255.248.0 | 0.0.7.255 | 2,048 | 2,046 | |
| /22 | 255.255.252.0 | 0.0.3.255 | 1,024 | 1,022 | |
| /23 | 255.255.254.0 | 0.0.1.255 | 512 | 510 | |
| /24 | 255.255.255.0 | 0.0.0.255 | 256 | 254 | old Class C |
| /25 | 255.255.255.128 | 0.0.0.127 | 128 | 126 | |
| /26 | 255.255.255.192 | 0.0.0.63 | 64 | 62 | |
| /27 | 255.255.255.224 | 0.0.0.31 | 32 | 30 | |
| /28 | 255.255.255.240 | 0.0.0.15 | 16 | 14 | |
| /29 | 255.255.255.248 | 0.0.0.7 | 8 | 6 | |
| /30 | 255.255.255.252 | 0.0.0.3 | 4 | 2 | classic P2P link |
| /31 | 255.255.255.254 | 0.0.0.1 | 2 | 2 | P2P only, RFC 3021 |
| /32 | 255.255.255.255 | 0.0.0.0 | 1 | 1 | single host route |

Usable hosts = 2^(32−prefix) − 2, except /31 (RFC 3021: both addresses usable on a
point-to-point link) and /32 (a host route).

---

## A.5 The Subnetting Procedure

Given an address and a prefix, in six steps. Worked for `192.168.10.70/27`.

**1. Find the interesting octet** — the one containing the prefix boundary.
/27 → 27 ÷ 8 = 3 remainder 3, so the boundary is 3 bits into the **fourth** octet.

**2. Compute the block size.** Mask octet for 3 bits set = 224. Block = 256 − 224 =
**32**.

**3. List the boundaries** in that octet: 0, 32, 64, 96, 128, 160, 192, 224.

**4. Find which block contains the address.** 70 falls between 64 and 96.
→ **Network address: 192.168.10.64**

**5. Broadcast = one below the next boundary.** Next is 96, so
→ **Broadcast: 192.168.10.95**

**6. Usable range** is everything between.
→ **First host 192.168.10.65, last host 192.168.10.94, 30 usable.**

Verify in binary if unsure:

```
Address    11000000.10101000.00001010.010 00110
Mask       11111111.11111111.11111111.111 00000
Network    11000000.10101000.00001010.010 00000  = .64
Broadcast  11000000.10101000.00001010.010 11111  = .95
```

---

## A.6 VLSM Procedure

1. **List every subnet with its required host count**, including growth headroom.
2. **Sort descending by size.**
3. **Allocate largest first**, from the start of the block. Allocating small subnets
   first fragments the space so that a later large subnet cannot be placed.
4. **Align each subnet on its own block-size boundary.**
5. **Keep each site's subnets contiguous** so the site can be summarised as one prefix
   (Chapter 31 §31.4 depends on this).
6. **Leave deliberate gaps** for growth. Space in 10/8 is free; renumbering is not.

Worked example, allocating from `172.16.0.0/16`:

| Requirement | Hosts needed | Prefix | Allocation | Usable |
|---|---|---|---|---|
| Manufacturing | 2,000 | /21 | 172.16.0.0/21 | 2,046 |
| Engineering | 500 | /23 | 172.16.8.0/23 | 510 |
| Sales | 100 | /25 | 172.16.10.0/25 | 126 |
| Warehouse | 25 | /27 | 172.16.10.128/27 | 30 |
| WAN link 1 | 2 | /31 | 172.16.10.160/31 | 2 |
| WAN link 2 | 2 | /31 | 172.16.10.162/31 | 2 |

Remaining from 172.16.10.164 onward and all of 172.16.11.0/16 upward is reserved for
growth.

---

## A.7 Summarisation

To find the shortest prefix covering a set of addresses:

1. Convert all addresses to binary.
2. Find the longest run of leading bits common to all.
3. That count is the prefix length; the common bits followed by zeros is the network.

Example: summarise `198.51.100.0/24`, `198.51.101.0/24`, `198.51.102.0/24`,
`198.51.103.0/24`.

```
198.51.100.0   11000110.00110011.01100100.00000000
198.51.101.0   11000110.00110011.01100101.00000000
198.51.102.0   11000110.00110011.01100110.00000000
198.51.103.0   11000110.00110011.01100111.00000000
common:        11000110.00110011.011001__ = 22 bits
```

→ **198.51.100.0/22**

A set can be summarised into a single prefix only if it is **contiguous** and
**aligned** — the first network must be a multiple of the combined block size. This is
why address plans must be designed for summarisation from the start.

---

## A.8 Special-Purpose IPv4 Ranges

| Range | Purpose | RFC |
|---|---|---|
| 0.0.0.0/8 | "This network" | 1122 |
| 10.0.0.0/8 | Private | 1918 |
| 100.64.0.0/10 | Carrier-grade NAT | 6598 |
| 127.0.0.0/8 | Loopback | 1122 |
| 169.254.0.0/16 | Link-local (APIPA) | 3927 |
| 172.16.0.0/12 | Private | 1918 |
| 192.0.2.0/24 | Documentation (TEST-NET-1) | 5737 |
| 192.88.99.0/24 | 6to4 relay anycast (deprecated) | 7526 |
| 192.168.0.0/16 | Private | 1918 |
| 198.18.0.0/15 | Benchmark testing | 2544 |
| 198.51.100.0/24 | Documentation (TEST-NET-2) | 5737 |
| 203.0.113.0/24 | Documentation (TEST-NET-3) | 5737 |
| 224.0.0.0/4 | Multicast | 5771 |
| 240.0.0.0/4 | Reserved | 1112 |
| 255.255.255.255/32 | Limited broadcast | 919 |

---

## A.9 IPv6 Quick Reference

**Notation (RFC 5952):** lowercase hex; drop leading zeros in each group; replace the
**longest** run of all-zero groups with `::`, once only.

| Prefix | Type |
|---|---|
| `::/128` | Unspecified |
| `::1/128` | Loopback |
| `2000::/3` | Global unicast |
| `fc00::/7` | Unique local (in practice `fd00::/8`) |
| `fe80::/10` | Link-local |
| `ff00::/8` | Multicast |
| `ff02::1` | All nodes on link |
| `ff02::2` | All routers on link |
| `2001:db8::/32` | Documentation (RFC 3849) |

**Standard allocations:** `/48` to an end site, `/56` sometimes to residential,
`/64` per subnet always (required for SLAAC). Because the subnet size is fixed,
site-internal subnetting is allocation rather than arithmetic.

---

## A.10 Decibels

| Ratio | dB | | Ratio | dB |
|---|---|---|---|---|
| 1 | 0 | | 1/2 | −3 |
| 2 | +3 | | 1/4 | −6 |
| 4 | +6 | | 1/10 | −10 |
| 8 | +9 | | 1/100 | −20 |
| 10 | +10 | | 1/1000 | −30 |
| 100 | +20 | | | |
| 1000 | +30 | | | |

Power ratios: dB = 10 log₁₀(P₁/P₂). Amplitude ratios: dB = 20 log₁₀(A₁/A₂).

**dBm** is absolute, referenced to 1 mW. **dBm + dB = dBm**, which makes a link budget
one line of addition.

| Power | dBm | | Power | dBm |
|---|---|---|---|---|
| 1 W | +30 | | 1 µW | −30 |
| 100 mW | +20 | | 1 nW | −60 |
| 20 mW | +13 | | 1 pW | −90 |
| 1 mW | 0 | | | |

**Thermal noise floor:** −174 dBm/Hz at 290 K. Add 10 log₁₀(*B*) for bandwidth *B*.

| Bandwidth | Noise floor |
|---|---|
| 1 kHz | −144 dBm |
| 1 MHz | −114 dBm |
| 20 MHz | −101 dBm |
| 80 MHz | −95 dBm |
| 160 MHz | −92 dBm |

Add the receiver's noise figure (typically 4–10 dB) for a practical floor.

**Free-space path loss:** FSPL(dB) = 32.45 + 20 log₁₀ *f*(MHz) + 20 log₁₀ *d*(km).
Doubling distance costs 6 dB; doubling frequency costs 6 dB.

**Shannon capacity:** *C* = *B* log₂(1 + SNR), with SNR as a **linear ratio**, not dB.
