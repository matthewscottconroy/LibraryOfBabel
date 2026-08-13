#!/usr/bin/env python3
"""
netcalc.py — subnet, VLSM and summarisation calculator.

Companion to Chapters 25–27 of the Telecommunications textbook. Everything here
is derivable by hand using Appendix A's procedure; this tool exists to check your
working, not to replace it.

Usage
-----
    # Analyse a prefix
    python3 netcalc.py subnet 192.168.10.70/27

    # Show the binary working as well
    python3 netcalc.py subnet 192.168.10.70/27 --binary

    # Split a block into N equal subnets
    python3 netcalc.py split 172.16.0.0/16 --into 8

    # Split a block by required host counts (VLSM, largest-first)
    python3 netcalc.py vlsm 172.16.0.0/16 2000 500 100 25 2 2

    # Summarise a set of prefixes into the shortest covering prefix
    python3 netcalc.py summarise 198.51.100.0/24 198.51.101.0/24 \\
                                 198.51.102.0/24 198.51.103.0/24

    # Determine whether a destination is local or remote for a host
    python3 netcalc.py local 192.168.10.70/27 192.168.10.100
"""

from __future__ import annotations

import argparse
import ipaddress
import sys


# --------------------------------------------------------------------------- #
# Formatting helpers
# --------------------------------------------------------------------------- #

def dotted_binary(addr: ipaddress.IPv4Address) -> str:
    """192.168.10.70 -> '11000000.10101000.00001010.01000110'"""
    return ".".join(f"{o:08b}" for o in addr.packed)


def split_binary_at_prefix(addr: ipaddress.IPv4Address, prefix: int) -> str:
    """Binary with a space marking the network/host boundary."""
    bits = "".join(f"{o:08b}" for o in addr.packed)
    return f"{bits[:prefix]} {bits[prefix:]}"


def hosts_in(net: ipaddress.IPv4Network) -> int:
    """Usable host addresses, honouring RFC 3021 for /31 and host routes."""
    if net.prefixlen == 32:
        return 1
    if net.prefixlen == 31:
        return 2          # RFC 3021: both addresses usable point-to-point
    return net.num_addresses - 2


def prefix_for_hosts(n: int) -> int:
    """Smallest prefix length providing at least n usable hosts."""
    if n <= 0:
        raise ValueError("host count must be positive")
    if n <= 2:
        return 31         # RFC 3021 point-to-point
    bits = 2
    while (2 ** bits) - 2 < n:
        bits += 1
        if bits > 30:
            raise ValueError(f"cannot satisfy {n} hosts in IPv4")
    return 32 - bits


# --------------------------------------------------------------------------- #
# subnet
# --------------------------------------------------------------------------- #

def cmd_subnet(args) -> int:
    iface = ipaddress.ip_interface(args.cidr)
    net = iface.network
    if not isinstance(net, ipaddress.IPv4Network):
        return cmd_subnet_v6(iface)

    interesting = net.prefixlen // 8
    bits_in_octet = net.prefixlen % 8
    mask_octet = net.netmask.packed[interesting] if interesting < 4 else 255
    block = 256 - mask_octet if bits_in_octet else 256

    print(f"Address      : {iface.ip}")
    print(f"Prefix       : /{net.prefixlen}")
    print(f"Netmask      : {net.netmask}")
    print(f"Wildcard     : {net.hostmask}")
    print()
    print(f"Network      : {net.network_address}")
    if net.prefixlen <= 30:
        hosts = list(net.hosts())
        print(f"First host   : {hosts[0]}")
        print(f"Last host    : {hosts[-1]}")
        print(f"Broadcast    : {net.broadcast_address}")
    elif net.prefixlen == 31:
        print(f"Host A       : {net.network_address}    (RFC 3021, no broadcast)")
        print(f"Host B       : {net.broadcast_address}")
    print(f"Usable hosts : {hosts_in(net):,}")
    print(f"Total addrs  : {net.num_addresses:,}")
    print()
    print(f"Interesting octet : {interesting + 1} "
          f"({bits_in_octet} bit(s) into it)" if bits_in_octet
          else f"Interesting octet : boundary falls on an octet edge")
    if bits_in_octet:
        print(f"Block size        : 256 - {mask_octet} = {block}")
        starts = list(range(0, 256, block))
        print(f"Boundaries        : {', '.join(str(s) for s in starts[:12])}"
              f"{' ...' if len(starts) > 12 else ''}")

    if args.binary:
        print()
        print("Binary working (space marks the network/host boundary):")
        print(f"  address    {split_binary_at_prefix(iface.ip, net.prefixlen)}")
        print(f"  mask       {split_binary_at_prefix(ipaddress.IPv4Address(int(net.netmask)), net.prefixlen)}")
        print(f"  network    {split_binary_at_prefix(net.network_address, net.prefixlen)}")
        if net.prefixlen <= 30:
            print(f"  broadcast  {split_binary_at_prefix(net.broadcast_address, net.prefixlen)}")
    return 0


def cmd_subnet_v6(iface) -> int:
    net = iface.network
    print(f"Address      : {iface.ip}")
    print(f"Prefix       : /{net.prefixlen}")
    print(f"Network      : {net.network_address}")
    print(f"Total addrs  : {net.num_addresses:,}")
    if net.prefixlen < 64:
        print(f"/64 subnets  : {2 ** (64 - net.prefixlen):,}")
    print()
    print("Note: IPv6 subnets are /64 by convention (SLAAC requires it), so")
    print("site-internal work is allocation rather than host arithmetic.")
    return 0


# --------------------------------------------------------------------------- #
# split
# --------------------------------------------------------------------------- #

def cmd_split(args) -> int:
    net = ipaddress.ip_network(args.cidr, strict=False)
    n = args.into
    if n < 1 or (n & (n - 1)) != 0:
        print(f"error: --into must be a power of two (got {n})", file=sys.stderr)
        return 1
    extra_bits = n.bit_length() - 1
    new_prefix = net.prefixlen + extra_bits
    if new_prefix > 32:
        print("error: cannot split that far", file=sys.stderr)
        return 1

    print(f"Splitting {net} into {n} subnets "
          f"(borrowing {extra_bits} bit(s) -> /{new_prefix})")
    print()
    print(f"{'#':>3}  {'Network':<20} {'Range':<34} {'Broadcast':<16} Hosts")
    print("-" * 96)
    for i, sub in enumerate(net.subnets(new_prefix=new_prefix)):
        if sub.prefixlen <= 30:
            hs = list(sub.hosts())
            rng = f"{hs[0]} - {hs[-1]}"
            bc = str(sub.broadcast_address)
        else:
            rng = f"{sub.network_address} - {sub.broadcast_address}"
            bc = "-"
        print(f"{i:>3}  {str(sub):<20} {rng:<34} {bc:<16} {hosts_in(sub):,}")
    return 0


# --------------------------------------------------------------------------- #
# vlsm
# --------------------------------------------------------------------------- #

def cmd_vlsm(args) -> int:
    block = ipaddress.ip_network(args.cidr, strict=False)
    requirements = sorted(args.hosts, reverse=True)   # largest first — §26.4

    print(f"VLSM allocation from {block}")
    print("Allocating largest-first; allocating in any other order fragments")
    print("the space so that a later large subnet cannot be placed.")
    print()
    print(f"{'Need':>7}  {'Prefix':<20} {'Range':<34} Usable  Waste")
    print("-" * 88)

    cursor = int(block.network_address)
    end = int(block.broadcast_address)
    total_used = 0

    for need in requirements:
        try:
            plen = prefix_for_hosts(need)
        except ValueError as exc:
            print(f"error: {exc}", file=sys.stderr)
            return 1
        size = 2 ** (32 - plen)
        # align the cursor to this subnet's own boundary
        if cursor % size:
            cursor += size - (cursor % size)
        if cursor + size - 1 > end:
            print(f"{need:>7}  -- does not fit in {block} --")
            return 1
        sub = ipaddress.ip_network((cursor, plen))
        usable = hosts_in(sub)
        if sub.prefixlen <= 30:
            hs = list(sub.hosts())
            rng = f"{hs[0]} - {hs[-1]}"
        else:
            rng = f"{sub.network_address} - {sub.broadcast_address}"
        print(f"{need:>7}  {str(sub):<20} {rng:<34} {usable:>6}  {usable - need:>5}")
        cursor += size
        total_used += size

    remaining = end - cursor + 1
    print("-" * 88)
    print(f"Used      : {total_used:,} addresses")
    print(f"Remaining : {remaining:,} addresses, from {ipaddress.IPv4Address(cursor)}")
    print()
    print("Leave deliberate gaps for growth: renumbering a live subnet is")
    print("disruptive, and address space in RFC 1918 ranges is free.")
    return 0


# --------------------------------------------------------------------------- #
# summarise
# --------------------------------------------------------------------------- #

def cmd_summarise(args) -> int:
    nets = [ipaddress.ip_network(c, strict=False) for c in args.cidrs]
    lo = min(int(n.network_address) for n in nets)
    hi = max(int(n.broadcast_address) for n in nets)

    # shortest prefix covering [lo, hi]
    plen = 32
    while plen >= 0:
        cand = ipaddress.ip_network((lo & (0xFFFFFFFF << (32 - plen)) & 0xFFFFFFFF, plen))
        if int(cand.network_address) <= lo and int(cand.broadcast_address) >= hi:
            break
        plen -= 1
    summary = cand

    print("Inputs:")
    for n in nets:
        bits = "".join(f"{o:08b}" for o in n.network_address.packed)
        print(f"  {str(n):<20} {bits[:summary.prefixlen]}"
              f"|{bits[summary.prefixlen:]}")
    print()
    print(f"Shortest covering prefix : {summary}")
    print(f"Covers                   : {summary.network_address} - "
          f"{summary.broadcast_address} ({summary.num_addresses:,} addresses)")

    covered = sum(n.num_addresses for n in nets)
    exact = covered == summary.num_addresses
    print(f"Exact aggregate          : {'yes' if exact else 'NO'}")
    if not exact:
        print()
        print(f"  The summary includes {summary.num_addresses - covered:,} addresses")
        print("  not in the input set. A set aggregates exactly only when it is")
        print("  contiguous AND aligned on the combined block boundary (§26.3).")
        collapsed = list(ipaddress.collapse_addresses(nets))
        print(f"  Minimal exact covering set: {', '.join(str(c) for c in collapsed)}")
    return 0


# --------------------------------------------------------------------------- #
# local
# --------------------------------------------------------------------------- #

def cmd_local(args) -> int:
    iface = ipaddress.ip_interface(args.host_cidr)
    dest = ipaddress.ip_address(args.destination)
    net = iface.network

    src_and = ipaddress.ip_address(int(iface.ip) & int(net.netmask))
    dst_and = ipaddress.ip_address(int(dest) & int(net.netmask))
    local = src_and == dst_and

    print("The host's decision for every packet (§25.3):")
    print()
    print(f"  source      {dotted_binary(iface.ip)}   {iface.ip}")
    print(f"  mask    AND {dotted_binary(ipaddress.IPv4Address(int(net.netmask)))}   {net.netmask}")
    print(f"          =   {dotted_binary(src_and)}   {src_and}")
    print()
    print(f"  dest        {dotted_binary(dest)}   {dest}")
    print(f"  mask    AND {dotted_binary(ipaddress.IPv4Address(int(net.netmask)))}   {net.netmask}")
    print(f"          =   {dotted_binary(dst_and)}   {dst_and}")
    print()
    if local:
        print("  Results MATCH -> destination is LOCAL.")
        print(f"  The host will ARP for {dest} itself and send the frame directly.")
    else:
        print("  Results DIFFER -> destination is REMOTE.")
        print("  The host will ARP for its DEFAULT GATEWAY and send the frame there,")
        print(f"  with the IP destination still set to {dest}.")
    return 0


# --------------------------------------------------------------------------- #

def main() -> int:
    ap = argparse.ArgumentParser(
        description="Subnet, VLSM and summarisation calculator.",
        epilog="Companion to Chapters 25-27. Check your hand working with it; "
               "do not substitute it for the working.")
    sub = ap.add_subparsers(dest="cmd", required=True)

    p = sub.add_parser("subnet", help="analyse a prefix")
    p.add_argument("cidr", help="e.g. 192.168.10.70/27")
    p.add_argument("--binary", action="store_true", help="show binary working")
    p.set_defaults(func=cmd_subnet)

    p = sub.add_parser("split", help="split a block into N equal subnets")
    p.add_argument("cidr")
    p.add_argument("--into", type=int, required=True,
                   help="number of subnets (must be a power of two)")
    p.set_defaults(func=cmd_split)

    p = sub.add_parser("vlsm", help="allocate by required host counts")
    p.add_argument("cidr")
    p.add_argument("hosts", type=int, nargs="+", help="required host counts")
    p.set_defaults(func=cmd_vlsm)

    p = sub.add_parser("summarise", aliases=["summarize"],
                       help="find the shortest covering prefix")
    p.add_argument("cidrs", nargs="+")
    p.set_defaults(func=cmd_summarise)

    p = sub.add_parser("local", help="local-or-remote decision for a destination")
    p.add_argument("host_cidr", help="the host's own address/prefix")
    p.add_argument("destination")
    p.set_defaults(func=cmd_local)

    args = ap.parse_args()
    try:
        return args.func(args)
    except ValueError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
