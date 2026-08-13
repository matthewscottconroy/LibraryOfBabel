#!/usr/bin/env python3
"""
simnet.py — small simulations for the arguments this book makes with numbers.

Companion to Chapters 9, 13, 16 and 44. Each subcommand reproduces a result that
the text asserts, so that you can verify it rather than believe it.

Usage
-----
    # ALOHA and slotted ALOHA: reproduce the 18.4% / 36.8% ceilings
    python3 simnet.py aloha

    # CSMA/CD against ALOHA: what carrier sense buys
    python3 simnet.py csma --stations 20 --load 0.8

    # Statistical multiplexing gain (Ch 9 §9.3, Ch 13 §13.4)
    python3 simnet.py statmux --users 100 --rate 1 --activity 0.05 --link 20

    # Queueing delay against utilisation (Ch 3 §3.2)
    python3 simnet.py queue

    # Ethernet minimum frame size from segment length (Ch 16 §16.2)
    python3 simnet.py minframe --length 2500 --rate 10M
"""

from __future__ import annotations

import argparse
import math
import random
import sys


# --------------------------------------------------------------------------- #

def cmd_aloha(args) -> int:
    """Analytic throughput curves; the classic result."""
    print("ALOHA channel throughput S against offered load G")
    print()
    print("  pure ALOHA     S = G e^(-2G)   — a frame is destroyed by any other")
    print("                                   transmission starting within one")
    print("                                   frame time either side")
    print("  slotted ALOHA  S = G e^(-G)    — transmissions align to slots, so")
    print("                                   only the same slot collides")
    print()
    print(f"{'G':>6}  {'pure S':>9}  {'slotted S':>10}")
    print("-" * 30)
    best_p = best_s = (0.0, 0.0)
    g = 0.05
    while g <= 3.0001:
        sp = g * math.exp(-2 * g)
        ss = g * math.exp(-g)
        if sp > best_p[1]:
            best_p = (g, sp)
        if ss > best_s[1]:
            best_s = (g, ss)
        if abs(g * 20 - round(g * 20)) < 1e-9 and round(g * 20) % 2 == 0:
            print(f"{g:>6.2f}  {sp:>9.4f}  {ss:>10.4f}")
        g += 0.05
    print("-" * 30)
    print(f"pure ALOHA peaks at G = {best_p[0]:.2f}, S = {best_p[1]:.4f}"
          f"   (1/2e = {1 / (2 * math.e):.4f})")
    print(f"slotted peaks at    G = {best_s[0]:.2f}, S = {best_s[1]:.4f}"
          f"   (1/e  = {1 / math.e:.4f})")
    print()
    print("18.4% and 36.8%. Abramson knew this and built it anyway, because the")
    print("alternative cost money and this cost nothing (Ch 16 §16.1).")
    return 0


def cmd_csma(args) -> int:
    """Discrete-event-ish simulation of CSMA/CD versus pure ALOHA."""
    rng = random.Random(args.seed)
    slots = args.slots
    n = args.stations
    p = args.load / n            # per-station per-slot transmit probability

    def run(carrier_sense: bool) -> float:
        busy_until = 0
        success = 0
        frame_slots = args.frame_slots
        t = 0
        while t < slots:
            senders = [i for i in range(n) if rng.random() < p]
            if carrier_sense and t < busy_until:
                t += 1
                continue
            if len(senders) == 1:
                success += frame_slots
                busy_until = t + frame_slots
                t += frame_slots
            elif len(senders) > 1:
                # collision: CSMA/CD aborts quickly, ALOHA wastes the whole frame
                wasted = 1 if carrier_sense else frame_slots
                busy_until = t + wasted
                t += wasted
            else:
                t += 1
        return success / slots

    aloha = run(carrier_sense=False)
    csma = run(carrier_sense=True)

    print(f"Stations          : {n}")
    print(f"Offered load      : {args.load}")
    print(f"Frame length      : {args.frame_slots} slots")
    print(f"Simulated slots   : {slots:,}")
    print()
    print(f"Pure ALOHA utilisation   : {aloha:.3f}")
    print(f"CSMA/CD utilisation      : {csma:.3f}")
    print(f"Improvement              : {csma / aloha:.1f}x" if aloha else "")
    print()
    print("Carrier sense avoids starting into a busy medium; collision detection")
    print("aborts a doomed frame after a slot instead of wasting its whole")
    print("duration. Metcalfe's two additions to ALOHA (Ch 16 §16.2).")
    return 0


def cmd_statmux(args) -> int:
    """The argument that decided packet versus circuit switching."""
    n, r, a, link = args.users, args.rate, args.activity, args.link
    fixed = n * r
    expected = n * a * r
    capacity_users = int(link // r)

    # P(more than capacity_users active) under a binomial model
    def binom(k):
        return math.comb(n, k) * (a ** k) * ((1 - a) ** (n - k))

    p_over = sum(binom(k) for k in range(capacity_users + 1, n + 1))

    print(f"Users                  : {n}")
    print(f"Rate when active       : {r} Mb/s")
    print(f"Activity factor        : {a} ({a * 100:.0f}% of the time)")
    print()
    print(f"Circuit switching (reserve for every user):")
    print(f"  capacity required    : {fixed:,.0f} Mb/s")
    print(f"  average utilisation  : {a * 100:.0f}%")
    print()
    print(f"Packet switching (provision for the aggregate):")
    print(f"  expected load        : {expected:,.1f} Mb/s")
    print(f"  provisioned link     : {link:,.0f} Mb/s")
    print(f"  serves               : {capacity_users} simultaneous active users")
    print(f"  P(demand exceeds it) : {p_over:.3e}")
    print()
    gain = fixed / link
    print(f"Statistical multiplexing gain : {gain:.1f}x")
    print()
    print(f"One {1/gain:.2f} of the capacity, essentially the same service. That")
    print("factor, compounded at every level of aggregation, is why packet")
    print("switching won (Ch 13 §13.4).")
    print()
    print("The price: no guarantee. When demand does exceed the link, packets")
    print("queue or are discarded, and nothing can be promised in advance.")
    return 0


def cmd_queue(args) -> int:
    """The rho/(1-rho) curve that governs capacity planning."""
    print("M/M/1 queueing: mean number waiting grows as rho/(1-rho)")
    print()
    print(f"{'Utilisation':>12}  {'Relative delay':>15}  {'Multiple of 50%':>16}")
    print("-" * 48)
    base = 0.5 / (1 - 0.5)
    for rho in (0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.75, 0.8, 0.85,
                0.9, 0.95, 0.98, 0.99):
        d = rho / (1 - rho)
        print(f"{rho:>12.2f}  {d:>15.2f}  {d / base:>15.1f}x")
    print("-" * 48)
    print()
    print("Going from 50% to 90% utilised — which looks like sensible use of a")
    print("resource you paid for — multiplies queueing delay by nine.")
    print()
    print("This is why capacity planning targets 60-70% peak, and why 'the link")
    print("is only 60% utilised so it isn't the problem' must be checked against")
    print("the PEAK rather than the five-minute average (Ch 3 §3.2).")
    print()
    print("Real traffic is bursty and self-similar rather than Poisson, which")
    print("makes the situation worse at a given mean, not better.")
    return 0


def cmd_minframe(args) -> int:
    """Where Ethernet's 64-byte minimum comes from."""
    length = args.length                      # metres
    rate = args.rate
    if isinstance(rate, str):
        mult = {"k": 1e3, "K": 1e3, "m": 1e6, "M": 1e6, "g": 1e9, "G": 1e9}
        rate = float(rate[:-1]) * mult[rate[-1]] if rate[-1] in mult else float(rate)
    velocity = 2.0e8                          # m/s in coax, ~0.67c

    one_way = length / velocity
    round_trip = 2 * one_way
    bits = round_trip * rate

    print(f"Segment length     : {length:,} m")
    print(f"Propagation speed  : {velocity:.2e} m/s (~0.67c in coax)")
    print(f"Data rate          : {rate:,.0f} b/s")
    print()
    print(f"One-way propagation  : {one_way * 1e6:.2f} us")
    print(f"Round-trip time      : {round_trip * 1e6:.2f} us")
    print()
    print("A station must still be transmitting when a collision from the far")
    print("end reaches it, or it will not detect the collision at all. So the")
    print("frame must occupy the wire for at least one round trip:")
    print()
    print(f"  minimum frame = round trip x rate")
    print(f"                = {round_trip:.3e} s x {rate:,.0f} b/s")
    print(f"                = {bits:,.0f} bits = {bits / 8:,.0f} bytes")
    print()
    print("Ethernet standardised 512 bits (64 bytes) with margin for repeaters.")
    print("That number is still enforced by the switch on your desk, for a cable")
    print("that has not been manufactured in decades (Ch 16 §16.2).")
    return 0


# --------------------------------------------------------------------------- #

def main() -> int:
    ap = argparse.ArgumentParser(
        description="Simulations reproducing the book's numerical arguments.")
    sub = ap.add_subparsers(dest="cmd", required=True)

    p = sub.add_parser("aloha", help="ALOHA throughput ceilings")
    p.set_defaults(func=cmd_aloha)

    p = sub.add_parser("csma", help="CSMA/CD against pure ALOHA")
    p.add_argument("--stations", type=int, default=20)
    p.add_argument("--load", type=float, default=0.8)
    p.add_argument("--frame-slots", type=int, default=10)
    p.add_argument("--slots", type=int, default=200_000)
    p.add_argument("--seed", type=int, default=1)
    p.set_defaults(func=cmd_csma)

    p = sub.add_parser("statmux", help="statistical multiplexing gain")
    p.add_argument("--users", type=int, default=100)
    p.add_argument("--rate", type=float, default=1.0, help="Mb/s when active")
    p.add_argument("--activity", type=float, default=0.05)
    p.add_argument("--link", type=float, default=20.0, help="Mb/s provisioned")
    p.set_defaults(func=cmd_statmux)

    p = sub.add_parser("queue", help="queueing delay against utilisation")
    p.set_defaults(func=cmd_queue)

    p = sub.add_parser("minframe", help="Ethernet minimum frame size derivation")
    p.add_argument("--length", type=float, default=2500, help="segment metres")
    p.add_argument("--rate", default="10M")
    p.set_defaults(func=cmd_minframe)

    args = ap.parse_args()
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
