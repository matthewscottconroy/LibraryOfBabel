# Chapter 25 — Further Reading

## Primary sources

**RFC 791 — Postel, J. (1981). *Internet Protocol*, §3.2.**
Where the classes are defined. Read it to see how little space the design had to work
with, and judge it accordingly.

**RFC 950 — Mogul, J. & Postel, J. (1985). *Internet Standard Subnetting Procedure.***
The subnet mask as an explicit object. Eighteen pages, and the moment the class became
redundant even though nobody said so for another eight years.

**RFC 1519 — Fuller, V., Li, T., Yu, J. & Varadhan, K. (1993). *Classless Inter-Domain
Routing (CIDR).***
**The fix.** Read the introduction for the 1992 projections that motivated it — routing
table exhaustion before 1996 — and note how urgent the tone is.

**RFC 4632 — Fuller, V. & Li, T. (2006). *CIDR: The Internet Address Assignment and
Aggregation Plan.***
The current consolidated version, and the better one to read for the mechanism rather
than the history.

**RFC 1518 — Rekhter, Y. & Li, T. (1993). *An Architecture for IP Address Allocation
with CIDR.***
The provider-based allocation argument, including an honest treatment of the renumbering
cost it imposes.

**RFC 3021 — Retana, A. et al. (2000). *Using 31-Bit Prefixes on IPv4 Point-to-Point
Links.***
Three pages explaining why a /30 on a point-to-point link wastes half its addresses, and
what to do instead. Widely supported and widely unused.

**RFC 1918 — Rekhter, Y. et al. (1996). *Address Allocation for Private Internets.***
The private ranges. Chapter 27 covers them; read it here to see the classful fossil in
`172.16.0.0/12` for yourself.

## Books

**Stevens, W. R. (1994). *TCP/IP Illustrated, Volume 1*, chapter 3.**
Addressing and masks, with the arithmetic worked properly.

**Doyle, J. & Carroll, J. (2005). *Routing TCP/IP, Volume 1*, 2nd ed. Cisco Press.**
**The best treatment of addressing and subnetting for people who will configure real
networks.** Chapter 2's binary and mask material is thorough, and the exercises are
harder and better than most.

**Lammle, T. Any current CCNA study guide.**
The subnetting shortcuts, drilled. Whatever your view of certification pedagogy, the
technique is genuinely useful and the drilling works.

**Comer, D. *Internetworking with TCP/IP, Volume 1*, chapters 4–5.**
More formal, and good on why the classful design was reasonable when it was made.

## Applied

**[tools/netcalc.py](../../../tools/netcalc.py)** — this book's calculator.
`netcalc.py subnet 192.168.10.70/26 --binary` prints the full binary AND alongside the
network, broadcast, range and count. **Use it to check hand arithmetic, not to replace
it** — the point is to become fast without it.

**[tools/subnet_practice.py](../../../tools/subnet_practice.py)** — the drill generator.
`subnet_practice.py --topic mixed --sheet 30 --seed 7` produces a worksheet;
`--answers` prints the key. **Do a sheet a week.** Subnetting is a motor skill and
there is no substitute for repetition.

**`ipcalc`** and **`sipcalc`**, if available on your system. Older, still good, and
present on many systems where nothing else is.

**Python's `ipaddress` module.** `python3 -c "import ipaddress;
n=ipaddress.ip_network('192.168.10.64/26'); print(n.network_address, n.broadcast_address,
n.num_addresses)"`. Worth knowing because it is on every modern system and it is correct.

**`ip addr` and `ip route`** on your own machine. Read your own mask. Compute your own
network by hand and check it against `ip route` — your directly-connected route is
exactly that computation.

**Lab 13** in this book's [labs/](../../../labs/) directory builds a deliberate mask
mismatch between two hosts on one segment, captures the ARP traffic from both sides, and
documents the asymmetric symptom set. **It is the fastest way to make §25.3 permanent.**

## Practice, specifically

Subnetting is examined under time pressure and used under incident pressure, and in both
cases **speed matters more than method**. The path:

1. **Memorise the mask octet table.** 0, 128, 192, 224, 240, 248, 252, 254, 255.
2. **Memorise powers of two to 1024.**
3. Work twenty problems **in binary, longhand**, until the mechanism is not mysterious.
4. Learn the shortcuts of Chapter 26 §26.2.
5. Drill until you can do a /26 or /27 problem in **under fifteen seconds** without
   writing binary.

Step 3 is the one people skip, and skipping it produces someone who can pass an exam and
cannot debug an unusual mask. **Do the binary first, then earn the shortcut.**

## For the certification-minded

Objective 1.7 is IPv4 addressing and it is examined heavily — subnetting questions are
among the most numerous on the test. Objective 5.3 expects **incorrect subnet mask** as a
named cause.

Seven things worth over-learning:

1. **The mask octet table**, cold.
2. **Usable hosts = 2^h − 2**, and why.
3. **Network address = all host bits zero; broadcast = all host bits one.**
4. **The classful ranges**: A 1–126, B 128–191, C 192–223, D 224–239, E 240–255,
   **127 loopback**.
5. **The three RFC 1918 ranges**, including `172.16.0.0/12` — and that `172.32.x.x` is
   **not** private, which is the standard distractor.
6. **`169.254.x.x` means DHCP failed.**
7. **A /30 gives two usable addresses** and is the point-to-point convention.

And the operational one that is worth more than several of those: **when connectivity is
selective or one-way between hosts on the same segment, compare the masks first.** It
takes ten seconds, it is the answer more often than anything else, and almost nobody
checks it first.
