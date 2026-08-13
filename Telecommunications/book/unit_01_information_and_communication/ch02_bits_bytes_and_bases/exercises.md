# Chapter 2 — Exercises

## A. Recall

**A1.** Convert to decimal: `10101010` · `11111100` · `00011111` · `11000000` ·
`10000001`

**A2.** Convert to binary (8 bits each): 224 · 172 · 31 · 248 · 100 · 7

**A3.** Convert to hexadecimal: `11011110` · `10101101` · `10111110` ·
`11101111`. (The four together spell something a programmer would recognise.)

**A4.** Convert from hex to binary and then to decimal: `0x2F` · `0xC0` · `0xFF` ·
`0x0A` · `0x80`

**A5.** How many bits are needed to uniquely identify one of: 12 items? 100 items?
1,000 items? 1,000,000 items? In each case, how many identifiers are wasted?

## B. Apply

**B1.** Complete the practice set from §2.2. Convert each address to binary, AND it
with `255.255.255.240`, and give the resulting network address in dotted decimal:
`10.1.1.200` · `172.16.30.99` · `192.168.4.17` · `203.0.113.254`

**B2.** For each prefix length, state the number of host bits, the total addresses
in the block, and the usable host addresses (total minus network and broadcast):
`/26` · `/30` · `/22` · `/19` · `/31`. The last one is a trick; RFC 3021 explains
why, and Chapter 27 discusses it.

**B3.** A network engineer writes a subnet mask as `255.255.208.0`. Convert the
third octet to binary and explain why this mask is invalid. Which two valid masks
is it most likely a typo for?

**B4.** Compute the following bitwise operations on 8-bit values, showing binary
working:
(a) `10110011 AND 11110000`
(b) `10110011 OR 00001111`
(c) `10110011 XOR 10100000`
(d) `NOT 11111000`
(e) The wildcard mask corresponding to `255.255.255.192`.

**B5.** Two IPv4 addresses are `192.168.10.70` and `192.168.10.100`. XOR them and
use the result to determine the longest prefix on which they agree. Then state the
smallest CIDR block that contains both.

**B6.** From the hex dump in §2.3, identify: the OUI (first three bytes) of the
source MAC address; the IP header's total length field (bytes 16–17 of the frame);
and the TCP flags byte. You will need the frame layout given in §2.3 and a little
counting.

**B7.** A 32-bit value is stored on a little-endian machine as the bytes
`50 00 00 00`. What value does a big-endian reader see? If this field is a TCP
port number in a header where only the low 16 bits are used, what port did the
sender intend, and what port does a naive reader report?

## C. Analyse

**C1.** A sensor reports one of five states with probabilities 0.90, 0.06, 0.02,
0.015, 0.005. (a) Compute the information content in bits of each individual
report. (b) Compute the average (the entropy). (c) A fixed-length code needs
⌈log₂ 5⌉ = 3 bits per report. What fraction of capacity is being wasted?
(d) Design a variable-length code that beats 3 bits on average, and compute its
average length. (Assign short codes to likely states; ensure no code is a prefix
of another.)

**C2.** UTF-8 is self-synchronising: continuation bytes begin `10` and lead bytes
never do. Prove that from any byte position in a valid UTF-8 stream you can find
the start of the next character by examining at most 4 bytes. Then construct a
2-byte-per-character encoding that is *not* self-synchronising and describe
concretely what happens when one byte is lost in transit.

**C3.** Show that for a contiguous subnet mask, `block size = 256 − mask octet`
always holds within the octet where the prefix boundary falls. Then show that it
fails for a hypothetical non-contiguous mask, and explain what a router would have
to do differently to support one.

**C4.** ASCII places `A` at 0x41 and `a` at 0x61, and digits `0`–`9` at
0x30–0x39. (a) Give a single bitwise operation that converts any uppercase letter
to lowercase. (b) Give one that converts a digit character to its numeric value.
(c) Explain why neither trick works in EBCDIC, and what that cost IBM's customers.

## D. Design

**D1.** You are designing a telemetry protocol for 20,000 battery-powered sensors
that each report once per minute over a network where every transmitted bit costs
measurable battery life. Each report contains: a device identifier, a temperature
between −40 °C and +85 °C to 0.5 °C resolution, a battery level, and a status code
from a set of 8.

Design the bit layout. Justify every field width using the log₂ argument from
§2.1. State your total frame size in bits, and compute the annual transmitted
volume for the whole fleet. Then identify which field you would make
variable-length, and why.

## E. Troubleshoot

**E1.** An application team reports that a REST API is returning corrupted data.
Investigation shows: the TCP connection completes normally; no retransmissions
appear in a packet capture; the HTTP status is 200; the `Content-Length` matches
the bytes received; a byte-for-byte comparison of the captured payload against the
server's source data shows they are *identical*. Yet the client displays
`Ã¤Ã¶Ã¼` where the server has `äöü`.

State where the fault is, using the vocabulary of §2.4. Explain why no
transport-layer or physical-layer investigation could ever have found it, and name
the single header field most likely to be wrong.
