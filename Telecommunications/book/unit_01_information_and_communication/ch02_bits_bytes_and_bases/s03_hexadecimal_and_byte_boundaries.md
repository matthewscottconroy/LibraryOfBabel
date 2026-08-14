# 2.3 Hexadecimal and Byte Boundaries

Nobody has sixteen fingers. Hexadecimal exists anyway, and it exists for exactly
one reason, which is worth stating before the notation:

> **Sixteen is two to the fourth.** Therefore one hex digit is exactly four bits,
> always, with no carrying and no arithmetic. Conversion is a lookup, not a
> calculation.

Decimal does not have this property. 10 is not a power of 2, so decimal digits do
not align with bit boundaries, and converting a decimal number to binary requires
actual work (as §2.2 demonstrated). Hex conversion requires only the following
table, which you should know by recognition within a week of meeting it:

| Hex | Binary | Dec | | Hex | Binary | Dec |
|---|---|---|---|---|---|---|
| 0 | 0000 | 0 | | 8 | 1000 | 8 |
| 1 | 0001 | 1 | | 9 | 1001 | 9 |
| 2 | 0010 | 2 | | A | 1010 | 10 |
| 3 | 0011 | 3 | | B | 1011 | 11 |
| 4 | 0100 | 4 | | C | 1100 | 12 |
| 5 | 0101 | 5 | | D | 1101 | 13 |
| 6 | 0110 | 6 | | E | 1110 | 14 |
| 7 | 0111 | 7 | | F | 1111 | 15 |

Conversion in both directions is now mechanical. `0x3F` is `0011 1111`. `1010
1100` is `0xAC`. No division, no subtraction, no thought.

## Nibbles and bytes

Four bits is a **nibble** (occasionally spelled *nybble*), and the pun is
deliberate — it is half a byte. One hex digit is one nibble.

Eight bits is a **byte**, and two hex digits are one byte. This is the reason hex
is the universal notation for machine data: byte-aligned data is hex-aligned data,
and you can read a dump without counting.

The word **octet** means the same as byte and is used throughout networking
standards. The distinction is historical: "byte" originally meant "the number of
bits needed to encode one character," which was 6 on some machines and 9 on
others, and the standards bodies wanted a word that meant *exactly eight* with no
ambiguity. When RFC 791 defines the IPv4 header, it counts octets. In modern
practice the words are interchangeable and you should be comfortable with both.

## Reading a packet dump

Here is the beginning of a real Ethernet frame as `tcpdump -x` would print it:

```
0x0000:  0050 5601 2345 001b 4411 3ab7 0800 4500
0x0010:  003c 1c46 4000 4006 b1e6 c0a8 0a46 c0a8
0x0020:  0a01 dcbc 0050 4a1b 8f2c 0000 0000 a002
```

Sixteen bytes per line, grouped in pairs, with the byte offset in hex on the left.
Now watch how much structure falls out of Chapter 15's frame format:

- **Bytes 0–5**: `00:50:56:01:23:45` — destination MAC address.
- **Bytes 6–11**: `00:1b:44:11:3a:b7` — source MAC address.
- **Bytes 12–13**: `0800` — EtherType. 0x0800 means "the payload is IPv4."
- **Byte 14**: `45` — the IPv4 version (`4`) and header length in 32-bit words
  (`5`, so 20 bytes). One byte, two fields, four bits each — and note that you can
  only see that split because hex digits *are* nibbles.
- **Bytes 26–29**: `c0a8 0a46` — the source IP address. In decimal:
  192.168.10.70. You can read that directly: 0xc0 = 192, 0xa8 = 168, 0x0a = 10,
  0x46 = 70.
- **Bytes 30–33**: `c0a8 0a01` — destination, 192.168.10.1.
- **Bytes 34–35**: `dcbc` — source port, 56508.
- **Bytes 36–37**: `0050` — destination port, 80. HTTP.

Nine fields identified in a wall of hex, using nothing but the nibble table and
a frame layout you have not formally met yet. This is the skill Chapter 64 builds
on, and it is why every protocol analyser in existence displays hex.

## Why IPv4 uses decimal and MAC addresses use hex

A reasonable question, since both are just integers.

IPv4's dotted-decimal notation dates from a time when addresses were assigned to
organisations on octet boundaries and were meant to be read, spoken, and typed by
humans who did not think in binary. `192.168.1.1` is easier to say over a
telephone than `0xC0A80101`. The cost of that decision is that the notation
obscures the bit structure, which is precisely why subnetting is the topic
students find hardest.

MAC addresses were never meant to be arithmetic. Nobody masks a MAC address;
nobody computes a MAC subnet, because MAC addresses are flat (§1.3, and
Chapter 17). What you *do* with a MAC address is compare it, and look up its
manufacturer prefix, and both operations are easier in a byte-aligned notation.
Hence hex.

IPv6, designed in the 1990s by people who had watched the dotted-decimal decision
play out, uses hex. `2001:db8::1` groups sixteen bits per field, which is four hex
digits, which is byte-aligned and prefix-legible. Chapter 28 shows how much easier
IPv6 subnetting is as a direct consequence of that notation choice.

## Endianness

Two systems can agree perfectly that a value is 32 bits and still disagree about
which byte goes first.

**Big-endian** puts the most significant byte at the lowest address. The value
0x0A0B0C0D is stored as `0A 0B 0C 0D`. This is what you would write on paper, and
it is what every Internet protocol uses — RFC 1700 calls it **network byte
order**, and it is mandatory in every header field in this book.

**Little-endian** puts the least significant byte first: `0D 0C 0B 0A`. This is
what x86 and ARM processors use internally, for reasons connected to how
arithmetic carries propagate.

The consequence: every network stack on a little-endian machine byte-swaps every
multi-byte header field on the way in and on the way out. The C functions are
named `htons`, `htonl`, `ntohs`, `ntohl` — host-to-network-short and friends — and
forgetting one is a rite of passage for every programmer who has ever written a
socket.

The names come from *Gulliver's Travels*, where Lilliput and Blefuscu go to war
over which end of a boiled egg should be broken. Danny Cohen's 1980 memo *On Holy
Wars and a Plea for Peace* made the analogy, argued that the choice was arbitrary
but that *making* a choice was essential, and got network byte order standardised.
It is one of the funniest documents in the RFC series and it settled the question.

When endianness goes wrong, you do not get an error. You get a port number of
20480 where you expected 80 (0x0050 read backwards is 0x5000 = 20480), or an IP
address of 70.10.168.192. The failure mode is *plausible nonsense*, which is the
worst kind, and it is the first instance of the principle §2.4 generalises.

> **Network+ note.** The exam expects you to recognise hex, read MAC addresses,
> and identify an OUI. It does not test endianness. But every hex dump you meet in
> the troubleshooting domain assumes the nibble table, so learn it now.
