#!/usr/bin/env python3
"""
subnet_practice.py — unlimited subnetting and binary drills with worked solutions.

Companion to Chapters 2 and 26. The point of this tool is repetition until the
operations are automatic; Chapter 26 argues that the investment is worth making
exactly once, and this is how you make it.

Usage
-----
    python3 subnet_practice.py                       # 10 mixed questions, interactive
    python3 subnet_practice.py --count 25 --topic subnet
    python3 subnet_practice.py --topic binary --seed 42
    python3 subnet_practice.py --sheet 20            # printable worksheet, no prompts
    python3 subnet_practice.py --sheet 20 --answers  # worksheet with answer key

Topics
------
    binary   binary/decimal/hex conversion
    mask     prefix <-> dotted mask, host counts
    subnet   network, broadcast, first/last host from address + prefix
    design   choose a prefix for a required host count
    summary  summarise a set of contiguous prefixes
    mixed    all of the above (default)
"""

from __future__ import annotations

import argparse
import ipaddress
import random
import sys

TOPICS = ("binary", "mask", "subnet", "design", "summary")

MASK_OCTETS = {0: 0, 1: 128, 2: 192, 3: 224, 4: 240, 5: 248, 6: 252, 7: 254, 8: 255}


# --------------------------------------------------------------------------- #
# Question generators. Each returns (prompt, answer, worked_solution)
# --------------------------------------------------------------------------- #

def q_binary(rng: random.Random):
    kind = rng.choice(("d2b", "b2d", "d2h", "h2d"))
    n = rng.randint(0, 255)
    if kind == "d2b":
        prompt = f"Convert {n} to 8-bit binary."
        answer = f"{n:08b}"
        work = _binary_working(n)
    elif kind == "b2d":
        prompt = f"Convert {n:08b} to decimal."
        answer = str(n)
        bits = f"{n:08b}"
        parts = [str(128 >> i) for i, b in enumerate(bits) if b == "1"]
        work = (f"  place values  128  64  32  16   8   4   2   1\n"
                f"  bits           {'   '.join(bits)}\n"
                f"  sum of set places: {' + '.join(parts) if parts else '0'} = {n}")
    elif kind == "d2h":
        prompt = f"Convert {n} to hexadecimal."
        answer = f"0x{n:02X}"
        work = (f"  {n} = {n:08b} in binary\n"
                f"  split into nibbles: {n >> 4:04b} {n & 0xF:04b}\n"
                f"  each nibble is one hex digit: {n >> 4:X} {n & 0xF:X}\n"
                f"  = 0x{n:02X}")
    else:
        prompt = f"Convert 0x{n:02X} to decimal."
        answer = str(n)
        work = (f"  0x{n:02X} -> nibbles {n >> 4:04b} {n & 0xF:04b} = {n:08b}\n"
                f"  = ({n >> 4} x 16) + {n & 0xF} = {n}")
    return prompt, answer, work


def _binary_working(n: int) -> str:
    lines, rem = [], n
    for place in (128, 64, 32, 16, 8, 4, 2, 1):
        if place <= rem:
            lines.append(f"  {place:>3} fits in {rem:>3}  -> 1, remainder {rem - place}")
            rem -= place
        else:
            lines.append(f"  {place:>3} > {rem:>3}          -> 0")
    lines.append(f"  = {n:08b}")
    return "\n".join(lines)


def q_mask(rng: random.Random):
    plen = rng.randint(8, 30)
    net = ipaddress.ip_network(f"0.0.0.0/{plen}")
    if rng.random() < 0.5:
        prompt = f"Give the dotted-decimal mask for /{plen}, and the usable host count."
        answer = f"{net.netmask}, {net.num_addresses - 2:,} hosts"
    else:
        prompt = f"Give the CIDR prefix length for {net.netmask}, and the usable host count."
        answer = f"/{plen}, {net.num_addresses - 2:,} hosts"
    host_bits = 32 - plen
    work = (f"  /{plen} means {plen} network bits, {host_bits} host bits.\n"
            f"  mask = {plen} ones then {host_bits} zeros\n"
            f"       = {'.'.join(f'{o:08b}' for o in net.netmask.packed)}\n"
            f"       = {net.netmask}\n"
            f"  usable hosts = 2^{host_bits} - 2 = {net.num_addresses:,} - 2 "
            f"= {net.num_addresses - 2:,}")
    return prompt, answer, work


def q_subnet(rng: random.Random):
    plen = rng.choice([17, 18, 19, 20, 21, 22, 23, 25, 26, 27, 28, 29, 30])
    base = rng.choice(("10", "172.16", "192.168", "203.0.113"))
    if base == "10":
        addr = f"10.{rng.randint(0,255)}.{rng.randint(0,255)}.{rng.randint(1,254)}"
    elif base == "172.16":
        addr = f"172.{rng.randint(16,31)}.{rng.randint(0,255)}.{rng.randint(1,254)}"
    elif base == "192.168":
        addr = f"192.168.{rng.randint(0,255)}.{rng.randint(1,254)}"
    else:
        addr = f"203.0.113.{rng.randint(1,254)}"

    iface = ipaddress.ip_interface(f"{addr}/{plen}")
    net = iface.network
    hosts = list(net.hosts())
    prompt = (f"For {addr}/{plen}, give the network address, first and last usable "
              f"host, broadcast address, and usable host count.")
    answer = (f"net {net.network_address}, first {hosts[0]}, last {hosts[-1]}, "
              f"bcast {net.broadcast_address}, {len(hosts):,} hosts")

    oct_idx = plen // 8
    bits_in = plen % 8
    if bits_in:
        mask_octet = MASK_OCTETS[bits_in]
        block = 256 - mask_octet
        a_oct = int(addr.split(".")[oct_idx])
        start = (a_oct // block) * block
        work = (f"  /{plen}: boundary is {bits_in} bit(s) into octet {oct_idx + 1}.\n"
                f"  mask octet = {mask_octet}; block size = 256 - {mask_octet} = {block}\n"
                f"  boundaries in that octet: "
                f"{', '.join(str(x) for x in range(0, 256, block))}\n"
                f"  {a_oct} falls in the block starting at {start}\n"
                f"  network   = {net.network_address}\n"
                f"  broadcast = one below the next boundary = {net.broadcast_address}\n"
                f"  usable    = 2^{32 - plen} - 2 = {len(hosts):,}")
    else:
        work = (f"  /{plen} falls on an octet boundary — the mask is "
                f"{net.netmask}.\n"
                f"  network   = {net.network_address}\n"
                f"  broadcast = {net.broadcast_address}\n"
                f"  usable    = 2^{32 - plen} - 2 = {len(hosts):,}")
    return prompt, answer, work


def q_design(rng: random.Random):
    need = rng.choice([2, 6, 14, 25, 30, 60, 100, 200, 400, 500, 1000, 2000])
    bits = 2
    while (2 ** bits) - 2 < need:
        bits += 1
    plen = 32 - bits
    net = ipaddress.ip_network(f"0.0.0.0/{plen}")
    prompt = (f"A subnet must hold {need} hosts. Give the smallest sufficient "
              f"prefix length and the resulting usable host count.")
    answer = f"/{plen} ({net.num_addresses - 2:,} usable)"
    work = (f"  need {need} usable, so need 2^h - 2 >= {need}\n"
            f"  2^{bits - 1} - 2 = {2 ** (bits - 1) - 2:,}  -> too small\n"
            f"  2^{bits} - 2 = {2 ** bits - 2:,}  -> sufficient\n"
            f"  host bits = {bits}, so prefix = 32 - {bits} = /{plen}\n"
            f"  waste = {net.num_addresses - 2 - need:,} addresses")
    return prompt, answer, work


def q_summary(rng: random.Random):
    count = rng.choice((2, 4, 8))
    shift = count.bit_length() - 1

    if rng.random() < 0.5:
        # Contiguous /24s inside the third octet.
        base_plen = 24
        third = rng.randrange(0, 256 - count + 1, count)
        nets = [ipaddress.ip_network(f"198.51.{third + i}.0/24")
                for i in range(count)]
    else:
        # Contiguous small subnets inside the fourth octet. Choose a base prefix
        # long enough that `count` of them still fit within one octet.
        base_plen = rng.choice([p for p in (26, 27, 28, 29)
                                if 2 ** (32 - p) * count <= 256])
        step = 2 ** (32 - base_plen)
        third = rng.randint(0, 255)
        start = rng.randrange(0, 256 - step * count + 1, step * count)
        nets = [ipaddress.ip_network(
            f"192.168.{third}.{start + i * step}/{base_plen}")
            for i in range(count)]

    combined_plen = base_plen - shift
    summary = ipaddress.ip_network(
        (int(nets[0].network_address), combined_plen))
    prompt = ("Summarise these into the shortest covering prefix:\n    "
              + "\n    ".join(str(n) for n in nets))
    answer = str(summary)
    bits = ["".join(f"{o:08b}" for o in n.network_address.packed) for n in nets]
    common = 0
    while common < 32 and len(set(b[common] for b in bits)) == 1:
        common += 1
    work = "  binary, with the common prefix marked:\n"
    for n, b in zip(nets, bits):
        work += f"    {str(n):<20} {b[:common]}|{b[common:]}\n"
    work += (f"  {common} leading bits are common to all -> /{common}\n"
             f"  summary = {summary}")
    return prompt, answer, work


GENERATORS = {"binary": q_binary, "mask": q_mask, "subnet": q_subnet,
              "design": q_design, "summary": q_summary}


# --------------------------------------------------------------------------- #

def make(topic: str, rng: random.Random):
    if topic == "mixed":
        topic = rng.choice(TOPICS)
    return GENERATORS[topic](rng)


def run_interactive(topic: str, count: int, rng: random.Random) -> int:
    print(f"{count} questions on: {topic}")
    print("Press Enter to reveal each answer; Ctrl-C to stop.\n")
    right = 0
    for i in range(1, count + 1):
        prompt, answer, work = make(topic, rng)
        print(f"--- Question {i}/{count} " + "-" * 46)
        print(prompt)
        try:
            given = input("\nYour answer (Enter to reveal): ").strip()
        except (EOFError, KeyboardInterrupt):
            print("\nStopped.")
            break
        print(f"\nAnswer: {answer}")
        print("Working:")
        print(work)
        if given:
            ok = _loose_match(given, answer)
            print(f"\n{'CORRECT' if ok else 'check your working'}")
            right += ok
        print()
    print(f"Done. Self-marked correct: {right}/{count}")
    return 0


def _loose_match(given: str, answer: str) -> bool:
    """Very forgiving comparison — this is a drill, not an exam."""
    norm = lambda s: "".join(ch for ch in s.lower() if ch.isalnum() or ch == ".")
    g, a = norm(given), norm(answer)
    return g == a or (len(g) > 3 and g in a)


def run_sheet(topic: str, count: int, rng: random.Random, answers: bool) -> int:
    items = [make(topic, rng) for _ in range(count)]
    print("=" * 72)
    print(f"SUBNETTING WORKSHEET — {count} questions ({topic})")
    print("=" * 72)
    print()
    for i, (prompt, _, _) in enumerate(items, 1):
        print(f"{i:>3}. {prompt}")
        print()
        print()
    if answers:
        print("\n" + "=" * 72)
        print("ANSWER KEY")
        print("=" * 72 + "\n")
        for i, (_, answer, work) in enumerate(items, 1):
            print(f"{i:>3}. {answer}")
            print(work)
            print()
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(
        description="Unlimited subnetting and binary drills with worked solutions.")
    ap.add_argument("--topic", choices=list(TOPICS) + ["mixed"], default="mixed")
    ap.add_argument("--count", type=int, default=10)
    ap.add_argument("--sheet", type=int, metavar="N",
                    help="print a worksheet of N questions instead of prompting")
    ap.add_argument("--answers", action="store_true",
                    help="include the answer key with --sheet")
    ap.add_argument("--seed", type=int, help="reproducible question set")
    args = ap.parse_args()

    rng = random.Random(args.seed)
    if args.sheet:
        return run_sheet(args.topic, args.sheet, rng, args.answers)
    return run_interactive(args.topic, args.count, rng)


if __name__ == "__main__":
    sys.exit(main())
