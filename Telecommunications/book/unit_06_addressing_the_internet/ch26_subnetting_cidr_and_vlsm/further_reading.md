# Chapter 26 — Further Reading

## Primary sources

**RFC 950 — Mogul, J. & Postel, J. (1985). *Internet Standard Subnetting Procedure.***
The mask as an explicit object. Also the source of the subnet-zero prohibition that
RFC 1878 later removed.

**RFC 1878 — Pummill, T. & Manning, B. (1995). *Variable Length Subnet Table for
IPv4.***
Makes subnet zero and the all-ones subnet explicitly legal, and provides the complete
prefix/mask/host table. **Print the table.**

**RFC 1519 / RFC 4632 — CIDR.**
The aggregation mechanism. RFC 4632 is the current consolidated version and the better
read for the mechanism; RFC 1519 is the better read for the urgency.

**RFC 1518 — Rekhter, Y. & Li, T. (1993). *An Architecture for IP Address Allocation
with CIDR.***
Provider-based allocation and hierarchical addressing — §26.4's argument, at Internet
scale, including an honest account of the renumbering cost.

**RFC 3021 — Retana, A. et al. (2000). *Using 31-Bit Prefixes on IPv4 Point-to-Point
Links.***
Three pages. Read it, then look at how many /30s your network has.

**RFC 1009 — Braden, R. & Postel, J. (1987). *Requirements for Internet Gateways.***
Where routers were first required to handle different masks — the formal enabling of
VLSM, six years before CIDR.

## Books

**Doyle, J. & Carroll, J. (2005). *Routing TCP/IP, Volume 1*, 2nd ed. Cisco Press.**
**The best treatment of VLSM and hierarchical addressing anywhere.** Chapters 2 and 6.
The worked examples are harder than most and closer to real design problems.

**Odom, W. Any current CCNA Official Cert Guide.**
The subnetting chapters are the most refined pedagogy that exists for this material, and
the practice problems are numerous and well graded. Use them regardless of whether you
intend to certify.

**Lammle, T. Any current CCNA study guide.**
The magic-number method, drilled hard. Different presentation from Odom's; some people
find one clicks where the other does not.

**Peterson, L. & Davie, B. *Computer Networks: A Systems Approach*, chapter 3.**
Good on *why* aggregation is necessary rather than merely how, with the routing table
growth data.

**Bush, R. & Meyer, D. RFC 3439, *Some Internet Architectural Guidelines and
Philosophy*.**
Not a book, and it belongs here: the argument that complexity is the enemy and that
hierarchical structure is the main defence. Relevant to every design decision in §26.4.

## Applied — the tools in this book

**[tools/subnet_practice.py](../../../tools/subnet_practice.py)** — the drill generator.

```bash
# A mixed sheet of 30, with a seed so you can regenerate the same one
python3 tools/subnet_practice.py --topic mixed --sheet 30 --seed 7
python3 tools/subnet_practice.py --topic mixed --sheet 30 --seed 7 --answers

# Focus on one weakness
python3 tools/subnet_practice.py --topic binary  --sheet 20
python3 tools/subnet_practice.py --topic mask    --sheet 20
python3 tools/subnet_practice.py --topic subnet  --sheet 20
python3 tools/subnet_practice.py --topic design  --sheet 10
python3 tools/subnet_practice.py --topic summary --sheet 15
```

**Do a sheet a week for six weeks.** Record your time. This is the single highest-return
activity in the course, and it is boring, and it works.

**[tools/netcalc.py](../../../tools/netcalc.py)** — the calculator.

```bash
python3 tools/netcalc.py subnet 192.168.10.70/27 --binary
python3 tools/netcalc.py split 192.168.10.0/24 --into 28
python3 tools/netcalc.py vlsm 192.168.1.0/24 100 50 25 10 2 2 2
python3 tools/netcalc.py summarise 192.168.4.0/24 192.168.5.0/24 \
                                   192.168.6.0/24 192.168.7.0/24
python3 tools/netcalc.py local          # your own machine's networks
```

**Use it to check, never to solve.** The `--binary` output shows exactly where the
boundary falls, and `summarise` will tell you when a set does not aggregate exactly and
give you the minimal covering set instead.

## Other tools

**`ipcalc`, `sipcalc`, `subnetcalc`** — standard Unix packages, all good.

**Python's `ipaddress` module** — on every modern system, correct, and worth knowing:

```python
import ipaddress
n = ipaddress.ip_network('192.168.10.0/24')
list(n.subnets(new_prefix=28))          # split
ipaddress.collapse_addresses([...])     # summarise
```

**`ip route` on your own router.** Look at the actual table. Count the entries. Ask
whether the number matches what the design intended, and if not, why.

**bgp.potaroo.net** — Huston's continuously updated global routing table data. Exercise
F3, and worth a look regardless: the growth curve with CIDR's 1994 inflection visible in
it is the most persuasive single image in this chapter.

**Lab 14** in this book's [labs/](../../../labs/) directory is a full VLSM design
exercise: a requirements sheet, a block, and a working topology to build and verify.
**Lab 15** adds summarisation and demonstrates a black hole caused by summarising too
aggressively.

## For the certification-minded

**Objective 1.7. This is the most heavily examined material in the entire
certification.** Expect many questions, several requiring the full computation under time
pressure.

The preparation, in order:

1. **Memorise the mask octet table** (Chapter 25 §25.1) and the powers of two.
2. **Memorise the prefix/block-size/host table** of §26.1.
3. **Work twenty problems in longhand binary** so the mechanism is not mysterious.
4. **Learn the magic-number method** and drill it.
5. **Drill to under fifteen seconds** for /25 through /30, and under thirty for the
   third-octet cases (/17 through /23).

Step 3 is the one people skip. Skipping it produces someone who can pass and cannot
debug.

Six things worth over-learning:

1. **subnets = 2^b, hosts = 2^h − 2.**
2. **Block size = 256 − mask octet**, and subnets start at multiples of it.
3. **For a /20, the broadcast's fourth octet is 255**, not 0. Third-octet problems are
   where errors cluster.
4. **Four /24s → /22, eight → /21, sixteen → /20**, and the summary must start on a
   boundary.
5. **VLSM requires a classless protocol** — RIPv1 and IGRP cannot.
6. **Allocate largest first.**

And the design instinct worth more than any of them: **plan for aggregation before you
assign anything.** A plan that is efficient and unsummarisable will churn the routing
table for as long as the network exists, and the only fix is renumbering.
