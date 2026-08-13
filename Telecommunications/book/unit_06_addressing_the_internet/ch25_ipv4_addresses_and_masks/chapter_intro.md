# Chapter 25 — IPv4 Addresses and Masks

An IPv4 address is a 32-bit unsigned integer. That is the complete truth about it,
and everything else in this chapter is either a consequence of that fact or a
notation that conceals it.

`192.168.10.70` is the integer 3,232,238,150. The dots are punctuation invented for
humans. The four numbers are the four bytes of the integer written in decimal, and
they exist because in 1981 it seemed friendlier than sixteen hexadecimal digits, and
because addresses were then handed out in units of whole octets so the grouping
matched the administration.

The grouping no longer matches anything. Addresses have been allocated on arbitrary
bit boundaries since 1993, and the dots now fall in places that have no
significance whatsoever — a `/27` boundary sits five bits into the last octet, and
the notation gives you no help at all in finding it.

This chapter's purpose is to get you to see the integer through the costume.

## The two questions every address must answer

An address must identify a host. It must *also* identify the network the host is
on, because otherwise the hierarchy of Unit VI's introduction is unavailable and
routers must know every host individually.

So a single 32-bit value is read in two parts:

```
  11000000 10101000 00001010 010 00110
  └──────────── network ────────┘ └host┘
         (27 bits)                (5 bits)
```

The **network portion** is shared by every host on the same network. The **host
portion** distinguishes them. Routers care almost exclusively about the first;
switches and hosts on the final segment care about the second.

And here is the fact that makes this non-trivial: **the address does not say where
the boundary is.** Given only `192.168.10.70`, there is no way to determine which
bits are network and which are host. The boundary is external information, and it
is carried by the subnet mask.

## The mask

A **subnet mask** is another 32-bit value in which every network bit is 1 and every
host bit is 0. It is not an address. It is not a gateway. It is an operator — the
thing you AND with an address to extract its network.

```
    11000000 10101000 00001010 01000110    192.168.10.70
AND 11111111 11111111 11111111 11100000    255.255.255.224
  = 11000000 10101000 00001010 01000000    192.168.10.64  ← the network
```

That single AND operation, which Chapter 2 §2.2 introduced, is performed in
hardware by every router for every packet it forwards. It is the fundamental
operation of IP networking, and understanding it completely makes the whole of
Chapter 26 mechanical.

Because a mask is always a contiguous run of 1s followed by 0s (Chapter 2 §2.2 —
RFC 4632 requires it), it can be written far more compactly as simply the count of
1 bits. `255.255.255.224` has 27 ones, so it is `/27`. This is **CIDR notation**,
it is universal in modern practice, and the dotted-decimal form persists mainly in
device configuration dialogues and in the muscle memory of people who learned in
the 1990s.

The two forms are identical information. You should be able to convert between them
instantly in both directions, and Chapter 2's nine legal octet values are the whole
of what you need.

## Why a host needs the mask, concretely

Students sometimes treat the mask as bureaucratic. It is not; a host cannot
function without it, and the reason is a decision it makes for every single packet.

When a host has a packet to send, it asks exactly one question: **is the destination
on my own network?**

It answers by ANDing its own address with its mask, ANDing the destination with the
*same* mask, and comparing.

- **Same result** → the destination is local. ARP for the destination itself
  (Chapter 18) and send the frame directly.
- **Different result** → the destination is remote. ARP for the **default gateway**
  and send the frame to the router, with the IP destination still set to the final
  target.

Notice how much rides on this. A wrong mask does not produce an error. It produces
a host that makes the wrong decision about *some* destinations and the right
decision about others, so connectivity becomes selective in a way that looks
inexplicable: you can reach half your own subnet and not the other half, or you can
reach the Internet and not the printer down the hall. Chapter 65 catalogues the
symptom; it is one of the most characteristic in the whole field, and recognising
it immediately is a mark of experience.

## The classful ghost

From 1981 to 1993, the network/host boundary was determined by the address's own
leading bits. Class A began with `0` and had an 8-bit network portion; Class B began
with `10` and had 16; Class C began with `110` and had 24. No mask was needed
because the address encoded its own boundary.

This was abolished by RFC 1519 in 1993, thirty-three years ago, and replaced by
CIDR — in which the boundary is arbitrary and always explicit.

It nonetheless refuses to die, and §25.4 explains why you still need to know it:

- **Default masks in equipment.** Type an address into many devices and they will
  suggest a mask based on its class. Accept the suggestion without thinking and you
  have a /8 where you wanted a /24.
- **Routing protocol behaviour.** RIPv1 and early IGRP were classful; some modern
  implementations retain classful auto-summarisation as a default that must be
  disabled.
- **Reserved ranges.** The private ranges of RFC 1918 are stated in class terms —
  one Class A, sixteen Class Bs, 256 Class Cs — and the multicast range is
  "Class D."
- **The language persists.** Engineers say "a Class C" when they mean a /24, and
  will do so for another generation.

The correct stance: classful addressing is history that shapes defaults. Know it
well enough to recognise it when a device does something surprising, and never
reason from it.

## What this chapter does

§25.1 covers dotted decimal and the integer beneath it, with conversion practice in
both directions.

§25.2 develops the network/host split and its consequences for routing.

§25.3 covers the mask as an operator: the AND, the CIDR notation, the local/remote
decision, and the symptoms of getting it wrong.

§25.4 covers the classful era, its abolition, and the specific places its ghost
still appears.

## By the end you will be able to

- Convert an IPv4 address between dotted decimal, binary, and its integer value.
- Convert between dotted-decimal masks and CIDR prefix lengths instantly.
- Perform the local/remote determination for any address, mask and destination.
- Predict the exact connectivity symptoms produced by a given wrong mask.
- Identify the classful class of an address and explain why that is now only
  historically relevant.
