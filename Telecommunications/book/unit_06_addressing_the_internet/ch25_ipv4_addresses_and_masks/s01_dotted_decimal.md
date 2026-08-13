# 25.1 Dotted Decimal

An IPv4 address is a **32-bit unsigned integer**. Everything else is presentation.

This section establishes that fact firmly, because the single largest obstacle to
learning subnetting is that people learn the dots first and the integer never.

## The integer

```
   192.168.10.70
```

is a human-readable rendering of:

```
   11000000 10101000 00001010 01000110
```

which is the number:

$$3{,}232{,}238{,}150$$

**All three are the same thing.** The dotted form exists because thirty-two binary
digits are hard to read aloud and 3,232,238,150 is hard to remember.

The convention is simple: **split the 32 bits into four 8-bit groups, write each as a
decimal number 0–255, separate with dots.** An 8-bit group is an **octet** — the term is
preferred over "byte" in networking, because in 1981 a byte was not universally 8 bits.

## Converting

You will do this constantly. It is worth being fast.

### Decimal to binary

The eight bit values in an octet:

| 128 | 64 | 32 | 16 | 8 | 4 | 2 | 1 |
|---|---|---|---|---|---|---|---|

**Method:** work left to right. Does the value fit? If yes, write 1 and subtract; if no,
write 0.

**170:**

| Value | 128 | 64 | 32 | 16 | 8 | 4 | 2 | 1 |
|---|---|---|---|---|---|---|---|---|
| Fits? | 170≥128 ✓ | 42≥64 ✗ | 42≥32 ✓ | 10≥16 ✗ | 10≥8 ✓ | 2≥4 ✗ | 2≥2 ✓ | 0≥1 ✗ |
| Bit | **1** | **0** | **1** | **0** | **1** | **0** | **1** | **0** |

$$170 = 10101010$$

### Binary to decimal

Add the values where the bit is 1.

$$11000000 = 128 + 64 = 192$$

### The full address

```
   192  =  11000000
   168  =  10101000
    10  =  00001010
    70  =  01000110

   192.168.10.70 = 11000000 10101000 00001010 01000110
```

## Values worth knowing by sight

Memorising these turns most subnetting arithmetic into recall. **This is the highest-
value memorisation in the subject.**

**Powers of two:**

| 2⁰ | 2¹ | 2² | 2³ | 2⁴ | 2⁵ | 2⁶ | 2⁷ | 2⁸ | 2⁹ | 2¹⁰ |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 2 | 4 | 8 | 16 | 32 | 64 | 128 | 256 | 512 | 1024 |

**The mask octets** — the only eight values a mask octet can take, because a mask's ones
are contiguous (§25.3):

| Binary | Decimal | Ones |
|---|---|---|
| `00000000` | **0** | 0 |
| `10000000` | **128** | 1 |
| `11000000` | **192** | 2 |
| `11100000` | **224** | 3 |
| `11110000` | **240** | 4 |
| `11111000` | **248** | 5 |
| `11111100` | **252** | 6 |
| `11111110` | **254** | 7 |
| `11111111` | **255** | 8 |

**Learn this table.** Every subnetting question in this book, in every certification,
and in every real network uses it. Note the pattern: each value is the previous plus the
next power of two downward — 128, +64, +32, +16, +8, +4, +2, +1.

**Any octet not in that list is not a valid mask octet.** A mask containing 255.255.255.7
is malformed, and recognising that instantly is worth having.

## The range

| | Value |
|---|---|
| Lowest | `0.0.0.0` = 0 |
| Highest | `255.255.255.255` = 4,294,967,295 |
| **Total** | **2³² = 4,294,967,296** |

**Roughly 4.3 billion addresses.**

In 1981 this was extravagant. There were a few hundred computers on the ARPANET, the
world's population was 4.5 billion, and the notion that every person might have several
network-connected devices was science fiction.

Today it is inadequate, and Chapter 27 §27.1 and Chapter 28 §28.1 cover the arithmetic
of exactly how inadequate and when it came due. **The usable count is far below 4.3
billion** — large blocks are reserved for multicast, for private use, for loopback, and
for experiments — and allocation inefficiency wastes much of the rest.

## Why dotted decimal at all?

An accident that turned out well.

The alternative would have been to write addresses as single integers, and the
convention would then be `3232238150` rather than `192.168.10.70`. Note what would be
lost:

- **The hierarchy would be invisible.** `192.168.10.70` and `192.168.10.71` are
  obviously related; `3232238150` and `3232238151` are not.
- **Masks would be incomprehensible.** `255.255.255.0` is recognisable as "the first
  three octets"; `4294967040` is not.
- **Mental arithmetic would be impossible.** Subnetting is done octet by octet, and
  that only works because the notation preserves the octet boundaries.

**The notation makes the structure visible**, and the structure is the whole point of
Chapter 25 and 26. IPv6's notation (Chapter 28 §28.2) does the same job in hexadecimal
with 16-bit groups, for the same reason.

## The trap the notation sets

The octet boundaries in the notation are a **convenience**, not a property of the
address. **Nothing in IP cares about them.**

A `/26` splits an address in the middle of the fourth octet. A `/12` splits it in the
middle of the second. Neither is unusual, and both confuse people who have absorbed the
dots as boundaries rather than as punctuation.

> **The address is 32 bits. The dots are for you. IP does not see them.**

Every subnetting difficulty in Chapter 26 traces back to this, and students who
internalise it early find the rest straightforward.

## Special values, in brief

Chapter 27 covers these properly. Recognise them now:

| Address | Meaning |
|---|---|
| `0.0.0.0` | "this host", or "any address" in configuration |
| `127.0.0.1` | loopback — **the whole `127.0.0.0/8` is loopback** |
| `255.255.255.255` | limited broadcast, never forwarded |
| `169.254.x.x` | link-local (APIPA) — **means DHCP failed** |
| `10.x.x.x`, `172.16–31.x.x`, `192.168.x.x` | private (RFC 1918) |
| `224.0.0.0` – `239.255.255.255` | multicast |

**`169.254.x.x` is worth recognising instantly.** A host with that address did not get a
DHCP reply, which localises a fault in one glance.

## Converting in practice

```bash
# Python, the readable way
python3 -c "import ipaddress; print(int(ipaddress.IPv4Address('192.168.10.70')))"
# 3232238150

python3 -c "import ipaddress; print(ipaddress.IPv4Address(3232238150))"
# 192.168.10.70

# Binary of each octet
python3 -c "print(' '.join(f'{int(o):08b}' for o in '192.168.10.70'.split('.')))"
# 11000000 10101000 00001010 01000110
```

This book's [tools/netcalc.py](../../../tools/netcalc.py) prints the binary form of any
address or network with `--binary`, which is the fastest way to check your hand
arithmetic while learning.

## What breaks here

**Treating the dots as boundaries.** They are punctuation. Masks cross them freely.

**Not knowing the mask octet table.** Everything in Chapter 26 becomes slow.

**Confusing an octet's maximum (255) with a count (256).** An octet holds 256 values,
0 through 255. This off-by-one causes more subnetting errors than any other single
thing.

**Assuming a leading zero is harmless.** `192.168.010.70` is interpreted as **octal** by
some parsers, making that octet 8 rather than 10. This has caused real security
incidents, and it is why you should never pad an address with zeros.

> **Network+ note.** Objective 1.7 expects IPv4 addressing fundamentals. Binary
> conversion is not usually asked directly and is required for everything that is.
> **Memorise the mask octet table** — 0, 128, 192, 224, 240, 248, 252, 254, 255 — and
> the powers of two to 2¹⁰. Everything in Chapter 26 depends on both.
