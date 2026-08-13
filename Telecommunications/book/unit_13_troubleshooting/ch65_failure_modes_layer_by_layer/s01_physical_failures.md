# 65.1 Physical Failures

**Layer 1 is where the boring faults live, and they are the commonest ones.** **Chapter 63
§63.2's "question the obvious" is mostly this section.**

## The state table

**Every diagnosis starts by reading the interface, and the four combinations mean different
things.**

| Admin | Line protocol | Means |
|---|---|---|
| **down** | down | **administratively shut down** — someone did this deliberately |
| **up** | **down** | **no carrier** — **cable, transceiver, or the far end** |
| **up** | **up** | **Layer 1 and 2 are fine.** Look higher |
| **err-disabled** | — | **the switch disabled it** — and it will say why |

**The second row is this section.** **The fourth is worth its own note:**

> **`err-disabled` is a switch protecting itself, and the reason is always logged.** **BPDU
> guard, port security violation, link flapping, duplex mismatch detection, storm control.**
> **`show interface status err-disabled` names the cause**, and re-enabling without reading it
> is how a fault recurs in ninety seconds.

## Copper faults

### No link at all

**In order of probability, which is also in order of cost to check:**

| | Cause | Check |
|---|---|---|
| **1** | **Nothing plugged in, or plugged into the wrong port** | **look** |
| **2** | **The far end is down or shut** | the other device |
| **3** | **A patch lead is faulty** | **swap it — the fastest conclusive test** |
| **4** | **The wall port is not patched through** | Chapter 53 §53.2's records |
| **5** | **Wrong cable type** | see below |
| **6** | **The port itself is faulty** | move to another port |
| **7** | **Speed/duplex forced and mismatched** | **a forced 1000/full against auto will not link** |

**Auto-MDI-X has made crossover cables irrelevant on any equipment made this century**, and
**it is worth knowing they existed** because **older equipment, some media converters and some
console arrangements still require the right one.**

### Bad pinout

**A cable that links and does not work properly.**

**T568A and T568B differ in the pairs' order.** **A cable with A at one end and B at the other is
a crossover** — **which auto-MDI-X will accommodate at Gigabit and which will fail at 10 Gb/s and
on some PoE arrangements.**

**And the specific fault worth recognising: a split pair.**

> **A split pair is wired correctly by the continuity test — every pin connects to the right
> pin — and the twisting is wrong**, because two conductors from different pairs have been used
> for one signal.

**Its symptoms:** **10/100 works and Gigabit does not**, or **Gigabit links and shows CRC errors
under load**, or **the link works over three metres and fails over sixty.**

**A continuity tester cannot find it. A certifier can**, because **the fault is in NEXT — crosstalk
— rather than in connectivity** (Chapter 10 §10.1's twisting argument, violated).

### Distance and attenuation

| Standard | Maximum | Note |
|---|---|---|
| **10/100/1000BASE-T** | **100 m** | **including patch leads at both ends** |
| **10GBASE-T** | **100 m on Cat6a**, 55 m on Cat6 | **and Cat6 is marginal at 55 m** |
| **PoE** | 100 m | **and voltage drop matters at the far end** |

**The 100 m figure includes everything** — **90 m of permanent link and 5 m of patch at each
end is the design assumption.** **A 15 m patch lead at each end of a 90 m run is out of
specification**, and it usually works, and it is the explanation when it does not.

### Errors that name their cause

**Chapter 66 §66.2 treats the counters properly. The Layer 1 subset:**

| Counter | Means |
|---|---|
| **CRC errors** | **corruption** — cable, connector, interference, or a duplex mismatch |
| **Runts** | **frames under 64 bytes** — collisions, or a duplex mismatch |
| **Giants / jumbo** | **frames over the MTU** — a mismatched jumbo configuration |
| **Input errors without CRC** | frame errors, overruns |
| **Late collisions** | **a cable exceeding the maximum length, or a duplex mismatch** |
| **Carrier transitions / flaps** | **the link is going up and down** |

> **CRC errors with no late collisions and no runts point at the physical layer. CRC errors
> accompanied by late collisions point at a duplex mismatch** (Chapter 66 §66.2), **and the two
> are diagnosed and fixed entirely differently.**

## Fibre faults

**Different failure modes, and a much better diagnostic signal.**

### Read the optical power first

```
   $ show interface transceiver
   Port      Temp   Voltage  Tx Power   Rx Power   Sensitivity
   Te1/0/1   38.4C  3.29V    -2.1 dBm   -18.4 dBm  -21 dBm      OK
   Te1/0/2   39.1C  3.30V    -2.3 dBm   -31.2 dBm  -21 dBm      ← 10 dB below
```

> **This is free, requires no tools, and answers the question directly.** **Rx power near or
> below the optic's documented sensitivity is the fault**, and everything else is a search for
> why.

**And Tx power tells you whether the transceiver at the other end is transmitting at all** —
**if you can read the far end, compare its Tx with your Rx and the difference is the loss in
between.**

### The causes, in order of probability

| | Cause | Note |
|---|---|---|
| **1** | **A dirty connector** | **the single most common fibre fault** — and it takes ten seconds to check with a scope (Chapter 64 §64.4) |
| **2** | **A bend exceeding the minimum radius** | **a cable tie pulled tight, or a fibre routed round a sharp corner** |
| **3** | **The wrong fibre type** | **single-mode transceiver into multimode fibre, or the reverse** |
| **4** | **Wavelength mismatch** | **1310 nm optic against a 1550 nm one** — no light at all |
| **5** | **Tx and Rx swapped** | **LC duplex connectors can be reversed**; the symptom is no link and normal Tx power |
| **6** | **A break or a bad splice** | **an OTDR gives the distance** |
| **7** | **Too much attenuation for the budget** | Chapter 50 §50.3 |
| **8** | **Too little attenuation** | **see below** |

**Item 8 catches people:**

> **A long-reach transceiver on a short fibre can overload the receiver.** **The symptom is
> errors on a link that is "too good"**, and **the fix is an optical attenuator** — a passive
> component that costs very little and looks absurd until you understand it.

### Fibre types, which must match

| Type | Colour (typically) | Core | Use |
|---|---|---|---|
| **OM3** | **aqua** | 50 µm multimode | **10 Gb/s to 300 m** |
| **OM4** | **aqua or violet** | 50 µm | **10 Gb/s to 400 m** |
| **OM5** | lime green | 50 µm | wideband multimode |
| **OS1/OS2** | **yellow** | **9 µm single-mode** | **long distance** |

**The colours are conventional and not guaranteed**, and **a single-mode transceiver into
multimode fibre may link and produce errors, which is worse than not linking at all.**

## Power

**PoE has its own set, and they present as intermittent device faults rather than as network
ones.**

| Standard | Watts at the source | Note |
|---|---|---|
| **802.3af (PoE)** | **15.4 W** | **12.95 W at the device** |
| **802.3at (PoE+)** | **30 W** | 25.5 W at the device |
| **802.3bt Type 3** | **60 W** | |
| **802.3bt Type 4** | **90 W** | |

**Three failure modes:**

**Budget exhaustion.** **A switch with a 370 W PoE budget cannot power 48 devices at 25 W**
(Chapter 56 §56.3). **The symptom is that some devices power up and others do not, apparently
at random** — **and it is whichever ones negotiated last.**

**Voltage drop over distance.** **Power is delivered at the source and consumed at the far end**,
and **a 90 m run to a device at the top of its class may deliver less than it needs.** **The
device reboots under load** — when the camera's infrared illuminator turns on, when the access
point's radios both transmit.

**And a device that requires PoE+ on a PoE port.** **It will power up and misbehave**, because
**the negotiation gave it 12.95 W and it wants 25.5.**

> **A PoE device that reboots intermittently, especially at a predictable time of day or under
> a specific activity, is a power problem until proved otherwise** — and **`show power inline`
> is the command.**

## Environmental and mechanical

**The ones that are not on any diagram.**

| Cause | Symptom |
|---|---|
| **Heat** | **errors rising with temperature**; Chapter 56 §56.3 |
| **Electrical interference** | **CRC errors near motors, lifts, fluorescent ballasts, welding** |
| **Water ingress** | **a run that degrades after rain** — Chapter 49 §49.1's copper fault |
| **Rodents** | **genuinely** — a clean break in an older building |
| **Vibration** | **a connector working loose over months** |
| **A cable disturbed by other work** | **the commonest cause of a fault that appears "for no reason"** |

**The last one deserves a note.** **"Nothing changed" is frequently false at the physical layer**
— **a contractor working in a ceiling, a cleaner moving a desk, someone pulling a cable to
reach a socket.** **Chapter 63 §63.2's "what changed?" should include physical work**, and the
building's facilities team frequently knows.

## The diagnostic sequence

**Six steps, in cost order, and most faults are found in the first three.**

```
   1.  Look at the interface state and the counters.       (free, 5 seconds)
   2.  Look at the transceiver power, if fibre.            (free, 5 seconds)
   3.  Swap the patch lead.                                (1 minute, conclusive)
   4.  Move to a different port.                           (2 minutes)
   5.  Loopback the port; loopback the far end.            (5 minutes)
   6.  Test the permanent link with a certifier or OTDR.   (an hour, and definitive)
```

> **Step 3 is the highest-value action in this section.** **A patch lead costs less than the
> time spent reasoning about whether it is the problem**, and swapping it either fixes the fault
> or eliminates a whole category.

## What breaks here

**`up/down` on an interface.** **No carrier.** Cable, transceiver, or the far end. Steps 1–4.

**A port that re-disables immediately after being enabled.** **`err-disabled`, and the cause was
not read.**

**10/100 works and Gigabit does not.** **A split pair, or a damaged pair.** A certifier finds
it; a continuity tester does not.

**A link that works short and fails long.** **Split pair, or attenuation, or the 100 m limit
including patch leads.**

**CRC errors with late collisions.** **Duplex mismatch** (Chapter 66 §66.2), not a cable.

**A fibre link with errors and Rx power at −31 dBm.** **Clean the connector first**, then check
for bends, then the fibre type.

**A fibre link with errors and Rx power at −2 dBm on a 40 km optic over 200 m.** **Receiver
overload.** An attenuator.

**A fibre that links and produces errors.** **Possibly a single-mode optic into multimode
fibre** — which is worse than not linking, because it appears to work.

**An access point that reboots when both radios are busy.** **PoE budget, class negotiation, or
voltage drop.**

**A fault that appeared "for no reason".** **Ask facilities what work has been done in the
ceiling.**

> **Network+ note.** Objective 5.2 covers physical issues directly. Over-learn: **attenuation,
> interference, decibel loss, incorrect pinout, bad ports, opens and shorts, and
> transceiver mismatch**; **CRC errors, runts, giants and late collisions and what each
> indicates**; **the 100 m copper limit**; **single-mode versus multimode and the consequence of
> mismatching**; and **PoE budget and class.** The counter-to-cause mapping is examined heavily.
