#!/usr/bin/env python3
"""
perfcalc.py — network performance arithmetic.

Companion to Chapters 3, 4 and 42. Computes the quantities that decide whether a
proposed link is possible and why a working one is slow.

Usage
-----
    # Bandwidth-delay product and the window needed to fill a path
    python3 perfcalc.py bdp --rate 1G --rtt 100

    # Maximum single-stream TCP throughput for a given window
    python3 perfcalc.py window --window 64K --rtt 100

    # Mathis: throughput ceiling from loss
    python3 perfcalc.py loss --mss 1460 --rtt 80 --loss 0.001

    # Decompose latency into its four components
    python3 perfcalc.py latency --distance 7000 --hops 14 --rate 10G --size 1500

    # Ethernet goodput efficiency for a payload size
    python3 perfcalc.py goodput --payload 1460 --rate 1G

    # Shannon capacity
    python3 perfcalc.py shannon --bandwidth 20M --snr-db 30

    # Thermal noise floor
    python3 perfcalc.py noise --bandwidth 20M --nf 6

    # Free-space path loss and a complete link budget
    python3 perfcalc.py linkbudget --freq 5800 --distance 34 --tx-power 23 \\
        --tx-gain 27 --rx-gain 27 --tx-loss 2 --rx-loss 2 --bandwidth 20M --nf 6

    # dB / dBm conversions
    python3 perfcalc.py db --ratio 400
    python3 perfcalc.py db --db 26
"""

from __future__ import annotations

import argparse
import math
import sys

C = 299_792_458.0                 # m/s, exact
FIBRE_KM_PER_MS = 204.0           # c / 1.468, rounded (Ch 1 §1.1)
K_BOLTZMANN = 1.380649e-23        # J/K, exact
T_REF = 290.0                     # K, standard reference temperature


# --------------------------------------------------------------------------- #
# Parsing helpers
# --------------------------------------------------------------------------- #

_SUFFIX = {"k": 1e3, "K": 1e3, "m": 1e6, "M": 1e6,
           "g": 1e9, "G": 1e9, "t": 1e12, "T": 1e12}


def si(value: str) -> float:
    """'1G' -> 1e9, '64K' -> 64000, '20M' -> 2e7, '1500' -> 1500."""
    value = str(value).strip()
    if value and value[-1] in _SUFFIX:
        return float(value[:-1]) * _SUFFIX[value[-1]]
    return float(value)


def human_rate(bps: float) -> str:
    for unit, div in (("Tb/s", 1e12), ("Gb/s", 1e9), ("Mb/s", 1e6), ("kb/s", 1e3)):
        if bps >= div:
            return f"{bps / div:,.3f} {unit}"
    return f"{bps:,.1f} b/s"


def human_bytes(b: float) -> str:
    for unit, div in (("GB", 1e9), ("MB", 1e6), ("KB", 1e3)):
        if b >= div:
            return f"{b / div:,.2f} {unit}"
    return f"{b:,.0f} B"


# --------------------------------------------------------------------------- #

def cmd_bdp(args) -> int:
    rate = si(args.rate)
    rtt = args.rtt / 1000.0
    bdp_bits = rate * rtt
    bdp_bytes = bdp_bits / 8

    print(f"Rate                 : {human_rate(rate)}")
    print(f"Round-trip time      : {args.rtt} ms")
    print()
    print(f"Bandwidth-delay product = rate x RTT")
    print(f"                        = {rate:,.0f} b/s x {rtt} s")
    print(f"                        = {bdp_bits:,.0f} bits = {human_bytes(bdp_bytes)}")
    print()
    print(f"That is the data in flight on a fully utilised path, and therefore")
    print(f"the receive window a single TCP stream needs to fill it.")
    print()
    print(f"Required window      : {human_bytes(bdp_bytes)}")
    if bdp_bytes > 65535:
        scale = math.ceil(math.log2(bdp_bytes / 65535))
        print(f"Exceeds the unscaled 16-bit TCP window (65,535 B).")
        print(f"Window scaling (RFC 7323) required, shift factor >= {scale}.")
        print(f"Without it, this path caps at "
              f"{human_rate(65535 * 8 / rtt)} per stream (Ch 3 §3.4).")
    else:
        print("Fits within the unscaled 16-bit TCP window.")
    return 0


def cmd_window(args) -> int:
    win = si(args.window)
    rtt = args.rtt / 1000.0
    tput = win * 8 / rtt
    print(f"Window   : {human_bytes(win)}")
    print(f"RTT      : {args.rtt} ms")
    print()
    print(f"Max single-stream throughput = window / RTT")
    print(f"                             = {human_rate(tput)}")
    print()
    print("Note this figure does not mention the link's capacity, because the")
    print("link's capacity is not involved (Ch 3 §3.4).")
    return 0


# Mathis constant for the standard AIMD sawtooth: sqrt(3/2).
# Ch 38 §38.2 uses this value; keep the two in agreement.
MATHIS_C = math.sqrt(1.5)


def cmd_loss(args) -> int:
    mss, rtt, p = args.mss, args.rtt / 1000.0, args.loss
    if p <= 0:
        print("error: loss must be > 0", file=sys.stderr)
        return 1

    def mathis(prob: float) -> float:
        return (mss * 8 / rtt) * (MATHIS_C / math.sqrt(prob))

    print(f"MSS      : {mss} bytes")
    print(f"RTT      : {args.rtt} ms")
    print(f"Loss     : {p * 100:.4f}%")
    print()
    print("Mathis relation:  throughput ~ (MSS / RTT) x (C / sqrt(p))")
    print(f"                  C = sqrt(3/2) = {MATHIS_C:.3f}")
    print(f"                = {human_rate(mathis(p))}")
    print()
    print("Ceiling at other loss rates on this path:")
    for q in (1e-6, 1e-5, 1e-4, 1e-3, 1e-2):
        print(f"  {q * 100:>8.4f}%  ->  {human_rate(mathis(q))}")
    print()
    print("Note the shape: throughput falls with the SQUARE ROOT of loss, and")
    print("is inversely proportional to RTT, so a long path is penalised twice.")
    print("Even one loss per million packets caps a 100 ms flow near 140 Mb/s.")
    print()
    print("Pessimistic for modern CUBIC/BBR stacks; the inverse-square-root")
    print("shape still governs (Ch 3 §3.3, Ch 38 §38.2).")
    return 0


def cmd_latency(args) -> int:
    rate = si(args.rate)
    prop = args.distance / FIBRE_KM_PER_MS
    trans = (args.size * 8 / rate) * 1000 * args.hops
    proc = args.proc_us * args.hops / 1000.0
    total = prop + trans + proc

    print(f"Distance   : {args.distance:,} km of fibre")
    print(f"Hops       : {args.hops}")
    print(f"Link rate  : {human_rate(rate)}")
    print(f"Frame size : {args.size} bytes")
    print()
    print(f"{'Component':<16} {'Calculation':<38} {'Value':>12}")
    print("-" * 68)
    print(f"{'Propagation':<16} {f'{args.distance} km / {FIBRE_KM_PER_MS} km/ms':<38} "
          f"{prop:>9.3f} ms")
    print(f"{'Transmission':<16} {f'{args.size * 8} bits / {human_rate(rate)} x {args.hops}':<38} "
          f"{trans:>9.3f} ms")
    print(f"{'Processing':<16} {f'{args.proc_us} us x {args.hops} hops':<38} "
          f"{proc:>9.3f} ms")
    print(f"{'Queueing':<16} {'load-dependent, not modelled here':<38} {'varies':>12}")
    print("-" * 68)
    print(f"{'One-way total':<16} {'(excluding queueing)':<38} {total:>9.3f} ms")
    print(f"{'Round trip':<16} {'':<38} {total * 2:>9.3f} ms")
    print()
    print(f"Propagation is {prop / total * 100:.1f}% of the fixed delay; "
          f"transmission is {trans / total * 100:.2f}%.")
    if prop / total > 0.9:
        print("A faster link would change almost nothing (Ch 3 §3.2).")
    return 0


def cmd_goodput(args) -> int:
    rate = si(args.rate)
    p = args.payload
    overhead = {"Interframe gap": 12, "Preamble + SFD": 8, "Ethernet header": 14,
                f"{'IPv6' if args.ipv6 else 'IPv4'} header": 40 if args.ipv6 else 20,
                "TCP header": 20, "FCS": 4}
    total = p + sum(overhead.values())

    print(f"Link rate : {human_rate(rate)}")
    print()
    print(f"{'Component':<28} {'Bytes':>8}")
    print("-" * 38)
    for k, v in overhead.items():
        print(f"{k:<28} {v:>8}")
    print(f"{'Application payload':<28} {p:>8}")
    print("-" * 38)
    print(f"{'Total on the wire':<28} {total:>8}")
    print()
    eff = p / total
    print(f"Efficiency   : {p} / {total} = {eff:.4f} ({eff * 100:.2f}%)")
    print(f"Max goodput  : {human_rate(rate * eff)}")
    print()
    if p < 500:
        print("Small payloads pay heavy header overhead. This is why voice codec")
        print("packetisation interval matters and why 'calls per circuit' is never")
        print("codec bitrate divided into link rate (Ch 3 §3.1).")
    elif p > 5000:
        print("Jumbo frames. Requires every device on the path to agree; a single")
        print("dissenting hop produces the black-hole failure of Ch 66 §66.3.")
    return 0


def cmd_shannon(args) -> int:
    b = si(args.bandwidth)
    if args.snr_db is not None:
        snr_lin = 10 ** (args.snr_db / 10)
        snr_db = args.snr_db
    else:
        snr_lin = args.snr
        snr_db = 10 * math.log10(snr_lin)
    cap = b * math.log2(1 + snr_lin)

    print(f"Bandwidth : {b / 1e6:,.3f} MHz")
    print(f"SNR       : {snr_db:.2f} dB  ({snr_lin:,.1f} linear)")
    print()
    print("Shannon-Hartley:  C = B log2(1 + SNR)")
    print(f"                    = {b:,.0f} x log2({1 + snr_lin:,.1f})")
    print(f"                    = {human_rate(cap)}")
    print()
    print(f"Spectral efficiency : {cap / b:.2f} bits/s/Hz")
    print()
    print("Capacity at other SNRs on this bandwidth:")
    for d in (5, 10, 15, 20, 25, 30, 35, 40):
        c = b * math.log2(1 + 10 ** (d / 10))
        print(f"  {d:>3} dB  ->  {human_rate(c):>14}   ({c / b:.2f} b/s/Hz)")
    print()
    print("Linear in bandwidth, logarithmic in SNR: +3 dB of power buys one extra")
    print("bit/symbol/Hz. This is why the industry chases spectrum, not watts.")
    return 0


def noise_floor_dbm(bandwidth_hz: float, noise_figure_db: float = 0.0) -> float:
    thermal_dbm_per_hz = 10 * math.log10(K_BOLTZMANN * T_REF / 1e-3)
    return thermal_dbm_per_hz + 10 * math.log10(bandwidth_hz) + noise_figure_db


def cmd_noise(args) -> int:
    b = si(args.bandwidth)
    per_hz = 10 * math.log10(K_BOLTZMANN * T_REF / 1e-3)
    thermal = noise_floor_dbm(b)
    practical = noise_floor_dbm(b, args.nf)

    print(f"Bandwidth      : {b / 1e6:,.3f} MHz")
    print(f"Temperature    : {T_REF} K (standard reference)")
    print()
    print(f"kT per hertz   : {per_hz:.2f} dBm/Hz")
    print(f"+ 10log10(B)   : +{10 * math.log10(b):.2f} dB")
    print(f"Thermal floor  : {thermal:.2f} dBm")
    if args.nf:
        print(f"+ noise figure : +{args.nf:.1f} dB")
        print(f"Practical floor: {practical:.2f} dBm")
    print()
    print("A received signal must exceed this floor by the SNR the chosen")
    print("modulation requires (Ch 4 §4.3).")
    return 0


def cmd_linkbudget(args) -> int:
    fspl = 32.45 + 20 * math.log10(args.freq) + 20 * math.log10(args.distance)
    rx = (args.tx_power + args.tx_gain - args.tx_loss
          - fspl - args.obstruction + args.rx_gain - args.rx_loss)

    print(f"Frequency  : {args.freq:,.0f} MHz   (wavelength "
          f"{C / (args.freq * 1e6) * 100:.1f} cm, quarter-wave "
          f"{C / (args.freq * 1e6) * 25:.1f} cm)")
    print(f"Distance   : {args.distance} km")
    print()
    print(f"FSPL = 32.45 + 20log10({args.freq:,.0f}) + 20log10({args.distance})")
    print(f"     = {fspl:.2f} dB")
    print()
    print(f"{'Term':<28} {'dB':>9}")
    print("-" * 39)
    print(f"{'Transmit power (dBm)':<28} {args.tx_power:>9.2f}")
    print(f"{'Transmit antenna gain':<28} {args.tx_gain:>+9.2f}")
    print(f"{'Transmit cable loss':<28} {-args.tx_loss:>+9.2f}")
    print(f"{'Free-space path loss':<28} {-fspl:>+9.2f}")
    if args.obstruction:
        print(f"{'Obstruction loss':<28} {-args.obstruction:>+9.2f}")
    print(f"{'Receive antenna gain':<28} {args.rx_gain:>+9.2f}")
    print(f"{'Receive cable loss':<28} {-args.rx_loss:>+9.2f}")
    print("-" * 39)
    print(f"{'Received power (dBm)':<28} {rx:>9.2f}")
    print()

    eirp = args.tx_power + args.tx_gain - args.tx_loss
    print(f"EIRP       : {eirp:.2f} dBm  "
          f"(check against local regulatory limit)")

    if args.bandwidth:
        b = si(args.bandwidth)
        floor = noise_floor_dbm(b, args.nf)
        snr = rx - floor
        cap = b * math.log2(1 + 10 ** (snr / 10)) if snr > -20 else 0.0
        print(f"Noise floor: {floor:.2f} dBm  ({b / 1e6:.1f} MHz, NF {args.nf:.1f} dB)")
        print(f"SNR        : {snr:.2f} dB")
        print(f"Shannon    : {human_rate(cap)}")

    # Margin is reported whenever a sensitivity is given, with or without a
    # bandwidth figure -- it is the question most link budgets are asked to
    # answer (Ch 42 §42.3).
    if args.sensitivity is not None:
        margin = rx - args.sensitivity
        print()
        print(f"Receiver sensitivity : {args.sensitivity:.1f} dBm")
        print(f"Link margin          : {margin:+.2f} dB")
        verdict = ("comfortable" if margin >= 20 else
                   "workable"    if margin >= 10 else
                   "marginal"    if margin >= 3  else
                   "INADEQUATE")
        print(f"Verdict              : {verdict}")
        if margin < 10:
            print()
            print("A margin below ~10 dB leaves nothing for rain, foliage growth,")
            print("or equipment ageing. Design for fade margin (Ch 42 §42.3).")

    print()
    print("Fresnel: keep 60% of the first zone clear -- line of sight is")
    print("necessary and not sufficient (Ch 42 §42.3).")
    r = 17.32 * math.sqrt(args.distance / (4 * args.freq / 1000.0))
    print(f"  first-zone radius at midpoint : {r:.2f} m")
    print(f"  60% clearance required        : {r * 0.6:.2f} m")
    print()
    print("Reminder: doubling distance costs 6 dB; doubling frequency costs 6 dB.")
    return 0


def cmd_db(args) -> int:
    if args.ratio is not None:
        print(f"Ratio {args.ratio:g}  ->  {10 * math.log10(args.ratio):.2f} dB (power)")
        print(f"                 {20 * math.log10(args.ratio):.2f} dB (amplitude)")
    if args.db is not None:
        print(f"{args.db:g} dB  ->  {10 ** (args.db / 10):.6g}x (power)")
        print(f"           {10 ** (args.db / 20):.6g}x (amplitude)")
    if args.dbm is not None:
        w = 10 ** (args.dbm / 10) / 1000
        print(f"{args.dbm:g} dBm  ->  {w:.6g} W  ({w * 1000:.6g} mW)")
    if args.watts is not None:
        print(f"{args.watts:g} W  ->  {10 * math.log10(args.watts * 1000):.2f} dBm")
    if all(v is None for v in (args.ratio, args.db, args.dbm, args.watts)):
        print("Give one of --ratio, --db, --dbm, --watts")
        return 1
    return 0


# --------------------------------------------------------------------------- #

def main() -> int:
    ap = argparse.ArgumentParser(
        description="Network performance and link arithmetic (Ch 3, 4, 42).")
    sub = ap.add_subparsers(dest="cmd", required=True)

    p = sub.add_parser("bdp", help="bandwidth-delay product and required window")
    p.add_argument("--rate", required=True, help="e.g. 1G, 100M")
    p.add_argument("--rtt", type=float, required=True, help="milliseconds")
    p.set_defaults(func=cmd_bdp)

    p = sub.add_parser("window", help="max single-stream throughput from a window")
    p.add_argument("--window", required=True, help="e.g. 64K, 2M")
    p.add_argument("--rtt", type=float, required=True)
    p.set_defaults(func=cmd_window)

    p = sub.add_parser("loss", help="Mathis throughput ceiling from loss")
    p.add_argument("--mss", type=int, default=1460)
    p.add_argument("--rtt", type=float, required=True)
    p.add_argument("--loss", type=float, required=True, help="fraction, e.g. 0.001")
    p.set_defaults(func=cmd_loss)

    p = sub.add_parser("latency", help="decompose one-way latency")
    p.add_argument("--distance", type=float, required=True, help="km of fibre")
    p.add_argument("--hops", type=int, default=1)
    p.add_argument("--rate", default="10G")
    p.add_argument("--size", type=int, default=1500, help="frame bytes")
    p.add_argument("--proc-us", type=float, default=5.0, help="per-hop processing us")
    p.set_defaults(func=cmd_latency)

    p = sub.add_parser("goodput", help="Ethernet efficiency for a payload size")
    p.add_argument("--payload", type=int, default=1460)
    p.add_argument("--rate", default="1G")
    p.add_argument("--ipv6", action="store_true")
    p.set_defaults(func=cmd_goodput)

    p = sub.add_parser("shannon", help="channel capacity")
    p.add_argument("--bandwidth", required=True, help="e.g. 20M, 3100")
    g = p.add_mutually_exclusive_group(required=True)
    g.add_argument("--snr-db", type=float)
    g.add_argument("--snr", type=float, help="linear ratio")
    p.set_defaults(func=cmd_shannon)

    p = sub.add_parser("noise", help="thermal noise floor")
    p.add_argument("--bandwidth", required=True)
    p.add_argument("--nf", type=float, default=0.0, help="receiver noise figure dB")
    p.set_defaults(func=cmd_noise)

    p = sub.add_parser("linkbudget", help="FSPL and a complete link budget")
    p.add_argument("--freq", type=float, required=True, help="MHz")
    p.add_argument("--distance", type=float, required=True, help="km")
    p.add_argument("--tx-power", type=float, required=True, help="dBm")
    p.add_argument("--tx-gain", type=float, default=0.0, help="dBi")
    p.add_argument("--rx-gain", type=float, default=0.0, help="dBi")
    p.add_argument("--tx-loss", type=float, default=0.0, help="dB")
    p.add_argument("--rx-loss", type=float, default=0.0, help="dB")
    p.add_argument("--obstruction", type=float, default=0.0, help="dB")
    p.add_argument("--bandwidth", help="for SNR and capacity, e.g. 20M")
    p.add_argument("--nf", type=float, default=6.0)
    p.add_argument("--sensitivity", type=float, help="receiver sensitivity dBm")
    p.set_defaults(func=cmd_linkbudget)

    p = sub.add_parser("db", help="decibel conversions")
    p.add_argument("--ratio", type=float)
    p.add_argument("--db", type=float)
    p.add_argument("--dbm", type=float)
    p.add_argument("--watts", type=float)
    p.set_defaults(func=cmd_db)

    args = ap.parse_args()
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())
