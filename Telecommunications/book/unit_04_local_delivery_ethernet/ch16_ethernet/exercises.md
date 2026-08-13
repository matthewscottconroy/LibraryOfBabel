# Chapter 16 — Exercises

## A. Recall

**A1.** State the maximum channel utilisation of pure and slotted ALOHA, and the
value of *G* at which each peaks.

**A2.** What two things did Metcalfe add to ALOHA, and what does each buy?

**A3.** Why is Ethernet's minimum frame 64 bytes?

**A4.** Read these standard names: `100BASE-FX`, `1000BASE-LX`, `10GBASE-SR`,
`2.5GBASE-T`. State rate, signalling, medium and typical reach for each.

**A5.** State the power available at the device for each of the four PoE standards.

## B. Apply

**B1.** Derive *S* = *Ge*⁻²ᴳ for pure ALOHA from the vulnerable-period argument.
Differentiate to find the maximum and confirm it is 1/2*e*. Then repeat for slotted
ALOHA.

**B2.** Verify both figures:
```bash
python3 tools/simnet.py aloha
```
Then explain what happens to throughput as *G* rises past its optimum, and why that
behaviour is called collapse rather than saturation.

**B3.** Derive the minimum frame size for a 500 m segment at 100 Mb/s, using
2 × 10⁸ m/s. Compare with the actual 64-byte minimum and explain why 100BASE-TX's
collision domain is far smaller than 10BASE5's.

**B4.** Trace a collision between two stations 1,800 m apart on a 10 Mb/s segment.
Give the timeline: when each begins, when each detects the collision, and how much
of each frame was transmitted before abort. Then compute both backoff windows after
one collision and after four.

**B5.** A station has collided six times in succession. State the backoff window
size, the maximum wait in slot times, and the maximum wait in milliseconds at
10 Mb/s. How many more collisions before it gives up?

**B6.** Work the 1000BASE-T arithmetic: 1 Gb/s over four pairs with PAM-5 carrying
2 useful bits per symbol. Give bits per pair, symbols per second, and fundamental
frequency. Show it fits within Cat5e's 100 MHz.

**B7.** A 48-port switch has a 500 W PoE budget. The deployment needs 18 access
points at 25.5 W, 24 cameras at 12.9 W and 6 telephones at 6.5 W. Does it fit? If
not, give three solutions and their costs.

**B8.** Explain, step by step, how a duplex mismatch arises when one end is
hard-coded to 100/full and the other autonegotiates. State what the autonegotiating
end detects, what it cannot detect, and what the standard requires it to assume.

## C. Analyse

**C1.** ALOHA's throughput falls as offered load rises past *G* = 0.5. Explain the
positive feedback loop this creates and identify the mechanism that damps it. Then
identify the same phenomenon at internetwork scale in Chapter 38 and state what
damps it there.

**C2.** Ethernet was non-deterministic and Token Ring was not; Ethernet won.
Construct the three-part argument from §16.3, then explain precisely why switching
made Token Ring's central advantage moot rather than merely less important.

**C3.** "Standardise the interface, not the mechanism." Tabulate what changed and
what did not across Ethernet's four decades, then find two other instances of the
same principle elsewhere in this book and one instance where it was *not* followed,
with the consequence.

**C4.** Explain how Mode A PoE delivers power on the data pairs without corrupting
the data. Your answer must reference differential signalling and common-mode
rejection explicitly, and must explain why this would not work on a single-ended
medium such as coax.

**C5.** Hard-coding speed and duplex is widely believed to be safer than
autonegotiation. Trace the origin of that belief, explain why it is now wrong, and
state the one circumstance in which hard-coding is still justified — and what must be
done if you do it.

## D. Design

**D1.** You are specifying access-layer switching for a floor with:

- 96 workstations at 1 Gb/s
- 12 access points requiring 802.3at
- 18 cameras requiring 802.3af
- 24 IP telephones requiring 802.3af, each with a PC daisy-chained through it
- Two 10 Gb/s uplinks to the distribution layer

Determine: the number and type of switches; the total PoE budget required and the
supply needed; the port count including uplinks; and the speed/duplex configuration
policy you will apply and why. State what you would specify about UPS provision and
justify it by reference to which services lose power with the switch.

## E. Troubleshoot

**E1.** A file server was moved to a new switch port during a maintenance window
three weeks ago. Since then, users report that transfers to it are "slow", and
increasingly so during busy periods. Nobody connects it to the move.

Evidence:

- The link shows 100/full on the switch; the server's OS reports 100 Mb/s.
- `ping` from a client: min 0.4 ms, avg 0.6 ms, max 3 ms, 0% loss.
- `iperf3` single stream at 09:00: 34 Mb/s. At 14:00: 6 Mb/s.
- Switch port counters: **late collisions 88,412 and rising**; alignment errors
  12,004; CRC errors 11,882; runts 402.
- Output drops: 0.
- The server's NIC counters show carrier sense errors.
- The switch port was configured during the move; the previous port was not.

Diagnose it precisely. Explain each counter and why it appears. Explain why
throughput is worse at 14:00 than 09:00, and why that pattern is diagnostic rather
than incidental. State the configuration change required, on which device, and what
you would check on the other end before making it.
