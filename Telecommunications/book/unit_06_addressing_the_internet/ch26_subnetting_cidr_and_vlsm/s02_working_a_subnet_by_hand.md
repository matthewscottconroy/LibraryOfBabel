# 26.2 Working a Subnet by Hand

This section is the practical heart of the course. It works one problem in complete
binary detail, then derives the shortcut, then drills it.

**Do the binary first.** The shortcut is faster and the binary is what lets you handle
the unusual case, verify a doubtful answer, and understand what you are doing. Students
who learn only the shortcut can pass an exam and cannot debug a /19.

## The problem

> **Given `192.168.10.70/27`, find the network address, broadcast address, first and
> last usable addresses, and the number of usable hosts.**

## Method 1 — Binary, longhand

### Step 1: Write the address in binary

```
   192  =  11000000
   168  =  10101000
    10  =  00001010
    70  =  01000110

   192.168.10.70  =  11000000 10101000 00001010 01000110
```

### Step 2: Write the mask in binary

/27 means 27 ones followed by 5 zeros:

```
   /27  =  11111111 11111111 11111111 11100000
           └──8──┘ └──8──┘ └──8──┘ └3┘└──5──┘
                                    27 ones, 5 zeros
```

In dotted decimal: `11100000` = 224, so **255.255.255.224**.

### Step 3: Draw the boundary

```
   address   11000000 10101000 00001010 010 00110
   mask      11111111 11111111 11111111 111 00000
                                        └┬┘ └─┬─┘
                              network ───┘    └─── host
                                27 bits       5 bits
```

### Step 4: Network address — AND, or set host bits to zero

```
   address   11000000 10101000 00001010 010 00110
   mask      11111111 11111111 11111111 111 00000
   AND       ─────────────────────────────────────
   network   11000000 10101000 00001010 010 00000
                                        └───┬───┘
                                       01000000 = 64
```

**Network: `192.168.10.64`**

### Step 5: Broadcast address — set host bits to one

```
   network    11000000 10101000 00001010 010 00000
   broadcast  11000000 10101000 00001010 010 11111
                                         └───┬───┘
                                        01011111 = 95
```

**Broadcast: `192.168.10.95`**

### Step 6: The usable range

**First usable** = network + 1 = **`192.168.10.65`**
**Last usable** = broadcast − 1 = **`192.168.10.94`**

### Step 7: Count

$$2^5 - 2 = 32 - 2 = \mathbf{30}$$

### The answer

| | |
|---|---|
| Network | `192.168.10.64` |
| First usable | `192.168.10.65` |
| Last usable | `192.168.10.94` |
| Broadcast | `192.168.10.95` |
| Mask | `255.255.255.224` |
| Usable hosts | **30** |

**Note again:** `192.168.10.70` is the **7th** address in this subnet, not the 70th of
anything (Chapter 25 §25.2).

## Method 2 — The magic number

The same answer in about ten seconds.

### The insight

$$\text{block size} = 256 - \text{interesting mask octet}$$

The **interesting octet** is the one where the mask is neither 255 nor 0 — where the
boundary falls.

### The five steps

**1. Find the interesting octet.**

/27 → mask `255.255.255.224` → the **fourth** octet.

**2. Compute the block size.**

$$256 - 224 = \mathbf{32}$$

**3. Count in multiples of 32 until you pass the address's value in that octet.**

```
   0, 32, 64, 96, …

   Address octet is 70.
   64 ≤ 70 < 96   →   the subnet starts at 64
```

**4. Network = that multiple. Broadcast = next multiple − 1.**

```
   Network    = 192.168.10.64
   Broadcast  = 192.168.10.96 − 1 = 192.168.10.95
```

**5. Usable = network + 1 to broadcast − 1.**

```
   192.168.10.65  –  192.168.10.94        (32 − 2 = 30 hosts)
```

**Done.** Same answer, ten seconds, no binary.

### Why it works

The block size *is* 2^h — the number of addresses in the subnet. Subnets partition the
octet into equal blocks starting at zero, so **every subnet boundary is a multiple of
the block size.** Counting multiples is exactly finding which block the address falls
into.

The binary method and the magic number are the same computation; the shortcut just skips
writing it out.

## When the interesting octet is not the fourth

The method is identical — only the octet changes.

### `172.16.155.100/20`

**1.** /20 → mask `255.255.240.0` → the **third** octet.

**2.** Block size = 256 − 240 = **16**.

**3.** Multiples of 16 in the third octet: 0, 16, 32, …, 144, **160**.
The address's third octet is 155, and 144 ≤ 155 < 160, so the subnet starts at **144**.

**4.**

| | |
|---|---|
| Network | `172.16.144.0` |
| Broadcast | `172.16.159.255` |

The broadcast is `144 + 16 − 1 = 159` in the third octet, and **all-ones (255) in every
octet after it**. This is the step people get wrong — the host portion extends past the
interesting octet.

**5.**

| | |
|---|---|
| First usable | `172.16.144.1` |
| Last usable | `172.16.159.254` |
| Hosts | 2¹² − 2 = **4,094** |

### `10.200.50.75/13`

**1.** /13 → mask `255.248.0.0` → the **second** octet.

**2.** Block size = 256 − 248 = **8**.

**3.** Multiples of 8: …, 192, **200**, 208. The second octet is 200, and 200 ≤ 200 <
208, so the subnet starts at **200**.

**4.**

| | |
|---|---|
| Network | `10.200.0.0` |
| Broadcast | `10.207.255.255` |
| First / last usable | `10.200.0.1` / `10.207.255.254` |
| Hosts | 2¹⁹ − 2 = **524,286** |

**The pattern generalises:** find the interesting octet, count blocks in it, zero
everything after it for the network, and set everything after it to 255 for the
broadcast.

## Drill

Cover the answers. Work each one, aim for under fifteen seconds.

| # | Given | Network | Broadcast | First | Last | Hosts |
|---|---|---|---|---|---|---|
| 1 | `192.168.1.100/26` | `.64` | `.127` | `.65` | `.126` | 62 |
| 2 | `10.0.0.200/25` | `.128` | `.255` | `.129` | `.254` | 126 |
| 3 | `172.16.4.50/28` | `.48` | `.63` | `.49` | `.62` | 14 |
| 4 | `192.168.100.10/29` | `.8` | `.15` | `.9` | `.14` | 6 |
| 5 | `10.1.1.1/30` | `.0` | `.3` | `.1` | `.2` | 2 |
| 6 | `203.0.113.200/27` | `.192` | `.223` | `.193` | `.222` | 30 |
| 7 | `192.168.20.130/25` | `.128` | `.255` | `.129` | `.254` | 126 |
| 8 | `172.20.75.90/22` | `172.20.72.0` | `172.20.75.255` | `172.20.72.1` | `172.20.75.254` | 1022 |
| 9 | `10.50.100.7/21` | `10.50.96.0` | `10.50.103.255` | `10.50.96.1` | `10.50.103.254` | 2046 |
| 10 | `192.168.15.55/19` | `192.168.0.0` | `192.168.31.255` | `192.168.0.1` | `192.168.31.254` | 8190 |

**Numbers 8, 9 and 10 are the ones worth repeating**, because the interesting octet is
the third and the broadcast extends into the fourth. That is where errors cluster.

## Listing every subnet

A different question: *"divide `192.168.10.0/24` into /28 subnets and list them."*

Block size = 256 − 240 = 16. Count in 16s:

| # | Network | Range | Broadcast |
|---|---|---|---|
| 1 | `.0` | `.1` – `.14` | `.15` |
| 2 | `.16` | `.17` – `.30` | `.31` |
| 3 | `.32` | `.33` – `.46` | `.47` |
| 4 | `.48` | `.49` – `.62` | `.63` |
| … | … | … | … |
| 16 | `.240` | `.241` – `.254` | `.255` |

**Sixteen subnets** (2⁴, four bits borrowed), fourteen hosts each.

**Check:** 16 × 16 = 256 ✓. Always verify that subnets × block size equals the parent
block size; it catches most arithmetic slips.

## Verifying

Three checks, and they are worth doing every time until the method is automatic:

**1.** Is the network address a multiple of the block size, in the interesting octet?
**2.** Is (broadcast − network + 1) equal to the block size?
**3.** Does the original address fall between first and last usable?

If any fails, the arithmetic is wrong.

And to check against a machine:

```bash
python3 tools/netcalc.py subnet 192.168.10.70/27 --binary
python3 tools/netcalc.py split 192.168.10.0/24 --into 28
```

**Use this to check, not to solve.** The goal is to be fast without it.

## What breaks here

**Using the wrong interesting octet.** For /20 it is the third, not the fourth.

**Forgetting the octets after the interesting one.** In a /20, the broadcast's fourth
octet is 255, not 0.

**Counting from 1 rather than 0.** Blocks start at 0.

**Confusing block size with host count.** Block size is 2^h; host count is 2^h − 2.

**Reading /27 as "27 hosts".** It is 27 network bits.

> **Network+ note.** Objective 1.7. **This section is the most heavily examined material
> in the entire certification.** Expect several questions requiring exactly this
> computation, under time pressure. Drill until a /26 or /27 problem takes under fifteen
> seconds — and know the binary method too, because it is what saves you when a question
> is unusual.
