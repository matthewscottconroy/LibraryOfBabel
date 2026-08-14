# 39.2 The Resolution Walk

**What happens between typing a name and getting an address.** It is more steps than most
people expect, and knowing them is what makes DNS faults diagnosable.

## The cast

| Role | What it does |
|---|---|
| **Stub resolver** | the library in your application — asks one server, believes the answer |
| **Recursive resolver** | does the actual work; **caches** |
| **Root servers** | know where the TLD servers are |
| **TLD servers** | know where each domain's servers are |
| **Authoritative servers** | hold the real answers for a zone |

**The division of labour is the point.** Your machine does almost nothing; **the recursive
resolver does the walking**, and because it caches, it does it far less often than the
query rate suggests.

## The walk

**Resolving `www.example.com`, from a completely cold cache:**

```
   Stub                Recursive              Root      .com     example.com
   resolver            resolver              server    server      server
     │                     │                    │         │           │
     │── www.example.com ─▶│                    │         │           │
     │                     │                    │         │           │
     │                     │── www.example.com ▶│         │           │
     │                     │◀── "ask .com" ─────│         │           │
     │                     │      (NS + glue)   │         │           │
     │                     │                              │           │
     │                     │── www.example.com ──────────▶│           │
     │                     │◀── "ask ns1.example.com" ────│           │
     │                     │      (NS + glue)             │           │
     │                     │                                          │
     │                     │── www.example.com ──────────────────────▶│
     │                     │◀── A 93.184.216.34 ──────────────────────│
     │                     │                                          │
     │◀── 93.184.216.34 ───│                                          │
```

**Four queries** — or three, plus the stub's one.

**Note the direction of the arrows.** The recursive resolver asks each server in turn and
**each answers with a referral rather than forwarding the query itself.** This is
**iterative** querying, and it is what the resolver does.

**The stub's single query is *recursive*** — "go and find out, and come back with an
answer."

> **Recursive: "get me the answer." Iterative: "tell me who to ask next."** The stub asks
> recursively; the resolver queries iteratively.

**And each referral moves one level down the tree** — which is the same shape as Chapter 29
§29.1's hop-by-hop forwarding, applied to a namespace instead of a topology.

## Glue records

**The chicken-and-egg problem, and its solution.**

`.com` says *"ask `ns1.example.com`"*. **But to reach `ns1.example.com` you need its
address — and that address is inside `example.com`, which is what you were trying to
resolve.**

**Glue records break the loop.** When `.com` returns the NS record it **also returns the A
record for the nameserver**, even though `.com` is not authoritative for it:

```
;; AUTHORITY SECTION:
example.com.    172800  IN  NS  ns1.example.com.

;; ADDITIONAL SECTION:
ns1.example.com. 172800 IN  A   192.0.2.53          ← the glue
```

**Glue is required only when the nameserver is *inside* the zone it serves.** If
`example.com`'s nameservers were `ns1.example.net`, no glue would be needed — that name
resolves independently.

**And stale glue is a real fault.** The parent zone holds a copy of the address, and if the
nameserver moves and the registrar's glue is not updated, **resolution fails for everyone
whose cache does not already hold the right answer** — intermittently and confusingly.

## The root

**Thirteen addresses, `a.root-servers.net` through `m.root-servers.net`.**

**Why thirteen?** Because a DNS response over UDP had to fit in **512 bytes** (§39.2's size
discussion), and thirteen NS records with their glue was the most that fitted.

**They are not thirteen machines.** Each address is served by **anycast** (Chapter 27
§27.3) from many physical locations — **over 1,900 instances worldwide.**

> **Anycast is why the root survives.** Volumetric attacks against it have been attempted
> repeatedly and absorbed, because the traffic is spread across the instance nearest each
> attacker rather than concentrated on thirteen machines.

**And the root is queried far less than the walk suggests**, because the TLD referrals are
cached with long TTLs — typically 172,800 seconds (48 hours). **A busy resolver may query
the root a handful of times per day.**

**The root hints file** — the list of the thirteen addresses — is the one piece of
bootstrap configuration every resolver ships with. It changes very rarely, and a resolver
with a stale copy still works because the servers that have not moved will tell it about
the ones that have.

## Caching — where the cost actually goes

**The walk above is the *cold* case.** In practice:

```
   www.example.com queried again, 30 seconds later:

   Stub ──▶ Recursive resolver ──▶ (cache hit) ──▶ answer
```

**One query, no network traffic beyond the first hop.**

**And partial hits are the common case.** A resolver that has never seen `example.com` but
has `.com`'s referral cached skips two of the four steps:

| Cached | Queries needed |
|---|---|
| Nothing | 4 |
| The root's `.com` referral | 3 |
| `.com`'s `example.com` referral | **2** |
| The answer itself | **1** (the stub's) |

**Typical cache hit rates on a busy resolver exceed 80%**, which is why the hierarchy's
apparent inefficiency is not one.

§39.4 covers TTLs and what caching costs.

## The stub's side

**Your machine's part is small and is where several faults live.**

```bash
# Linux
cat /etc/resolv.conf
resolvectl status              # systemd-resolved

# macOS
scutil --dns

# Windows
ipconfig /all
Get-DnsClientServerAddress
```

**`/etc/resolv.conf`:**

```
nameserver 10.0.0.53
nameserver 10.0.0.54
search example.com eng.example.com
options timeout:2 attempts:2
```

**Three things worth understanding:**

**Multiple nameservers are tried in order, not load-balanced.** The second is used only
when the first fails or times out — **and the timeout is 5 seconds by default**, so a dead
primary makes every lookup take 5 seconds until it is removed. **This is a very common
cause of "everything is slow".**

**The `search` list appends domains to unqualified names.** Typing `www` tries
`www.example.com`, then `www.eng.example.com`, then `www.` — **which means an unqualified
name can resolve to something unexpected**, and it is why FQDNs (with the trailing dot)
matter in configuration files.

**`options timeout` and `attempts` are worth tuning down** on systems where a fast failure
is better than a slow success.

## The response

```bash
$ dig www.example.com

;; QUESTION SECTION:
;www.example.com.               IN      A

;; ANSWER SECTION:
www.example.com.        86400   IN      A       93.184.216.34
                          ↑     ↑       ↑            ↑
                         TTL   class   type        answer

;; Query time: 23 msec
;; SERVER: 10.0.0.53#53(10.0.0.53)
;; flags: qr rd ra
```

**The flags are the part people skip and should not:**

| Flag | Meaning |
|---|---|
| `qr` | this is a response |
| **`rd`** | **recursion desired** — the stub asked for a full answer |
| **`ra`** | **recursion available** — the server is willing |
| **`aa`** | **authoritative answer** — this came from the zone's own server, not a cache |
| `tc` | **truncated** — the response did not fit; retry over TCP |

**`aa` absent means the answer came from a cache**, which is normal from a recursive
resolver and is the first thing to check when an answer looks stale.

**`tc` set is the transition to TCP** — §39.2's size problem below.

**And to see the walk yourself:**

```bash
dig +trace www.example.com
```

**This performs the iterative walk step by step and prints every referral.** It is the
most instructive DNS command, and running it once makes this whole section
concrete.

## The size problem

**DNS was designed for UDP with a 512-byte limit** (Chapter 36 §36.3), because that was
what any path could carry without fragmentation.

**Modern responses exceed it routinely** — DNSSEC signatures, many A records, IPv6
addresses alongside IPv4.

**Two mechanisms handle it:**

**Truncation and TCP.** The server sets `tc`, the resolver retries over **TCP port 53**.
Correct, and it costs a round trip plus a handshake.

**EDNS0** (RFC 6891) — the client advertises a larger buffer, commonly **1232 or 4096
bytes**, and the server uses UDP up to that size.

**And EDNS0's larger sizes caused their own problem.** A 4096-byte UDP response
**fragments** (Chapter 36 §36.4), and fragments are frequently dropped — so
**RFC 9715 (2025) now recommends limiting DNS responses to about 1,232 bytes**, a
deliberate retreat toward the original reasoning after two decades.

> **A firewall permitting only UDP/53 breaks large responses, zone transfers and DNSSEC.**
> Permit TCP/53 as well; this is one of the most common DNS-related firewall errors.

## Reverse DNS

**Address to name**, using a special hierarchy.

**The address is reversed and placed under `in-addr.arpa`:**

```
   93.184.216.34   →   34.216.184.93.in-addr.arpa
```

**Reversed because DNS is hierarchical from the right**, and IP addresses are hierarchical
from the left. **Flipping them aligns the two hierarchies**, so delegation of an address
block corresponds to delegation of a DNS subtree.

**IPv6 uses `ip6.arpa` with nibble-reversed hex**, which is verbose:

```
   2001:db8::1 → 1.0.0.0.....8.b.d.0.1.0.0.2.ip6.arpa
```

**Reverse DNS is not automatic and is not verified.** Anyone can claim any name in a
forward zone; **only the address block's holder controls the reverse.** Which is why
**forward-confirmed reverse DNS** — checking that the reverse name's forward lookup returns
the original address — is used by mail servers as a weak sender check.

## What breaks here

**Every lookup taking exactly 5 seconds.** The first nameserver in `resolv.conf` is dead
and the resolver is timing out before trying the second.

**A name resolving to something unexpected.** The `search` list appended a domain. Use a
trailing dot.

**Large responses failing while small ones work.** UDP/53 permitted and TCP/53 blocked, or
fragmentation. Chapter 34 §34.4.

**Resolution failing for one domain only.** Its nameservers, or stale glue at the parent.

**An answer that is correct at the authoritative server and wrong everywhere else.**
Caching (§39.4).

> **Network+ note.** Objective 1.6 expects the resolution process. Over-learn: **the stub
> asks recursively and the resolver queries iteratively**; **root → TLD → authoritative**;
> **DNS uses UDP/53 with TCP/53 for large responses and zone transfers**; and **the 13 root
> server addresses are anycast to many instances.** The recursive/iterative distinction is
> examined and is frequently reversed.
