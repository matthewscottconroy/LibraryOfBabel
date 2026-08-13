# 3.1 Bandwidth, Throughput, and Goodput

Three words, routinely used interchangeably, denoting three different quantities.
Sorting them out takes ten minutes and pays for itself immediately.

## Bandwidth: two meanings, and the confusion between them

**Bandwidth** in its original and physical sense is a measurement of *frequency*:
the width of the band of frequencies a channel can carry, measured in hertz. A
voice-grade telephone circuit passes roughly 300 Hz to 3,400 Hz, so its bandwidth
is about 3,100 Hz. A Wi-Fi channel in the 5 GHz band might be 20, 40, 80, or
160 MHz wide. This is the sense Nyquist and Shannon use, it is the sense we need
in Chapter 4, and it is a property of the physical channel.

**Bandwidth** in networking's colloquial sense means the maximum data rate of a
link, in bits per second. "A gigabit link has a gigabit of bandwidth."

These are related — Chapter 4 gives the exact relationship — but they are not the
same quantity and they do not even share units. The colloquial usage is now
universal and this book uses it too, but you should know the difference, because
when a wireless standard says "160 MHz channel width" and "2.4 Gb/s data rate," it
is quoting both, and the relationship between them is the entire content of
Chapter 8.

For clarity, this book uses **capacity** when the distinction matters:

> **Capacity** is the maximum rate at which a link can deliver bits. It is a
> property of the link, fixed by its physics and its standard, and it does not
> vary with what you are doing.

## Throughput: what you actually get

**Throughput** is the rate actually achieved, measured over some interval. It is
always less than or equal to capacity, usually by more than people expect, and it
depends on everything — the protocol, the peer, the path, the load, the
application, the disk at the far end.

Capacity is what you buy. Throughput is what you get. The gap between them is
where this book lives.

A concrete example, on a 1 Gb/s Ethernet link:

- **Capacity:** 1,000 Mb/s.
- **Measured throughput of a single large file transfer:** typically 940 Mb/s.
- **Measured throughput of a transfer of 10,000 small files:** perhaps 120 Mb/s.
- **Measured throughput to a server 150 ms away without window scaling:**
  about 3.5 Mb/s.

Same link, same cable, same switch. Four different answers, and none of them is
the link's fault.

## Goodput: what the application receives

**Goodput** is the rate of *useful application payload* delivered, excluding all
protocol overhead and all retransmitted data. This is the number a user
experiences and the only one they care about.

Let us compute where the missing bits go on that 1 Gb/s Ethernet link, because the
arithmetic is instructive and because the same accounting recurs at every layer.

A standard Ethernet frame carrying a maximum-size TCP segment over IPv4:

| Component | Bytes |
|---|---|
| Interframe gap (mandatory idle) | 12 |
| Preamble + start frame delimiter | 8 |
| Ethernet header (dst, src, EtherType) | 14 |
| IPv4 header (no options) | 20 |
| TCP header (no options) | 20 |
| **Application payload (MSS)** | **1,460** |
| Ethernet frame check sequence | 4 |
| **Total on the wire** | **1,538** |

So for every 1,538 bytes of wire time, 1,460 bytes are payload. The efficiency is

$$\frac{1{,}460}{1{,}538} = 0.9493$$

and 1,000 Mb/s × 0.9493 = **949 Mb/s** of theoretical maximum goodput. Real
measurements land around 940 Mb/s once TCP timestamps (12 more bytes of options)
are counted, which is exactly what `iperf3` reports on a healthy gigabit link.
This is not a defect. It is arithmetic, and if someone shows you a gigabit link
delivering 940 Mb/s, they are showing you a perfect link.

Now do the same for a small packet — a 64-byte VoIP payload:

| Component | Bytes |
|---|---|
| Interframe gap + preamble + SFD | 20 |
| Ethernet header + FCS | 18 |
| IPv4 + UDP + RTP headers | 20 + 8 + 12 = 40 |
| **Payload** | **160** |
| **Total** | **238** |

Efficiency: 160/238 = 67%. A third of the wire is header. This is why voice
codecs care about packetisation interval, why header compression exists on
low-bandwidth links, and why "how many calls fit on this circuit" is never
answered by dividing by the codec bitrate. Chapter 52 does that calculation
properly.

**Jumbo frames** attack this from the other end: raise the payload to 9,000 bytes
and efficiency goes to 9,000/9,078 = 99.1%. The catch is that every device on the
path must agree, and a single device that does not produces the black-hole
failure of Chapter 66. This is why jumbo frames are common inside data centres,
where one team controls every hop, and rare across the Internet, where nobody does.

## Bits, bytes, and the unit trap

Two conventions, and one commercial ambiguity.

**Rates are in bits; storage is in bytes.** A 100 Mb/s link moves 12.5 MB/s. The
factor of 8 is responsible for an enormous number of support calls from customers
who bought "100 megabit" service and observe their browser reporting 11 MB/s. The
customer is not being cheated; they are reading a different unit.

This book writes rates with a lowercase `b` (`Mb/s`, `Gb/s`) and storage with an
uppercase `B` (`MB`, `GB`). Many vendors use `Mbps` and `MBps`, which differ by
one capital letter in a position nobody notices. Be careful reading datasheets.

**Decimal versus binary prefixes.** In telecommunications, prefixes are strictly
decimal: 1 kb/s = 1,000 b/s, 1 Mb/s = 10⁶ b/s, 1 Gb/s = 10⁹ b/s. This is not
negotiable and derives from the ITU. In memory, prefixes are historically binary:
1 KiB = 1,024 bytes. Storage vendors use decimal, operating systems have
historically used binary, and this is why a "500 GB" drive shows as 465 GB in
Windows. The IEC binary prefixes (KiB, MiB, GiB) exist to disambiguate and this
book uses them where memory is meant.

## Measuring throughput honestly

Four cautions, each of which has produced a wrong conclusion in someone's report:

**Measure for long enough.** TCP starts slowly (Chapter 38's slow start). A
one-second test on a high-latency path measures the ramp, not the steady state.
Ten seconds minimum; thirty on a long path.

**Know what is limiting you.** A file copy that reports 40 MB/s may be limited by
the disk at either end, the CPU doing encryption, the filesystem's small-file
overhead, or the network. `iperf3` exists precisely to remove everything but the
network from the measurement, which is why "test with iperf before blaming the
network" is standard practice.

**One stream is not the link's capacity.** A single TCP stream is limited by the
bandwidth–delay product (§3.4) and by its own loss response. `iperf3 -P 16`
measures something closer to the path's capacity.

**Direction matters.** Many access technologies are deliberately asymmetric —
ADSL, DOCSIS, most cellular. Testing only downstream on an asymmetric link tells
you half the story, and the upstream is usually the half that breaks video calls.

> **Network+ note.** N10-009 uses "bandwidth," "throughput," and "goodput" and
> expects the distinction, particularly in performance-troubleshooting scenarios
> where a question describes a link that "should" be fast. It also expects you to
> know `iperf` as a throughput-testing tool (objective 5.5). The trap answer in
> those questions is always the one that increases capacity.
