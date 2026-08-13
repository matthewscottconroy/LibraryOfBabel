# Chapter 39 — DNS

Until the early 1980s, the mapping from names to addresses on the ARPANET lived in a
single file called `HOSTS.TXT`. It was maintained at the Stanford Research Institute
by, for much of its life, one person — Elizabeth Feinler, whose group ran the Network
Information Center. If you connected a new host, you telephoned or emailed the NIC,
and Feinler's team edited the file. Everyone else downloaded a fresh copy by FTP,
periodically, and hoped it was current.

This worked. It worked for a decade, for a network of a few hundred hosts, and it is
worth pausing on the fact that the Internet's name service was, within living memory,
a text file maintained by hand.

By 1983 it was failing in every possible way at once. The file had grown large
enough that every host downloading it was a measurable fraction of the ARPANET's
bandwidth. Updates took days to propagate. Name collisions had to be resolved by
human negotiation. And the growth rate was such that all of these were getting worse
monthly.

Paul Mockapetris at USC's Information Sciences Institute was asked to design a
replacement, and in November 1983 published RFC 882 and 883, revised in 1987 into
RFC 1034 and 1035, which remain the specification.

## The three ideas

Mockapetris's design has three properties, and they are the reason it has survived
forty-three years of growth by roughly nine orders of magnitude without structural
change.

**Hierarchy.** Names are read right to left, from most general to most specific:
`www.example.com.` — the trailing dot is the root, usually omitted and always
implied. Each level is a separate zone of authority. This is the same hierarchical
argument as Chapter 26's subnetting and Chapter 31's OSPF areas, applied to names.

**Delegation.** The root does not know about `www.example.com`. It knows who is
responsible for `com`, and nothing more. The `com` servers do not know about `www`;
they know who is responsible for `example.com`. Authority is delegated downward at
each boundary, and **no server anywhere holds the complete database.** There is no
central point of control, which is why nobody can turn DNS off and why registries
can be operated by different organisations in different jurisdictions.

**Caching.** Every answer carries a **time to live**, and every resolver keeps it for
that long. This is what makes the system survivable: the root servers are consulted
astonishingly rarely, because the answer to "who handles `com`" is in every
resolver's cache almost permanently. Caching converts a system that would require
global-scale query handling into one where the vast majority of queries are answered
locally.

## The walk

§39.2 traces a full recursive resolution. A resolver with an empty cache being asked
for `www.example.com`:

```
  1. → root server:              "Where is www.example.com?"
     ← "I don't know, but the com. servers are at ..."       (referral)
  2. → com. server:              "Where is www.example.com?"
     ← "I don't know, but example.com's servers are at ..."  (referral)
  3. → example.com server:       "Where is www.example.com?"
     ← "93.184.216.34"                                       (authoritative answer)
```

Three queries, each answered by a server that knew only the next step. The pattern is
exactly Chapter 29's hop-by-hop forwarding, applied to a namespace instead of an
address space, and noticing that correspondence is worth more than memorising the
diagram.

The distinction between a **stub resolver** (on your machine, asks one question and
expects an answer), a **recursive resolver** (your ISP's, or `8.8.8.8`, which does
the walking and holds the cache), and an **authoritative server** (which holds a
zone and answers for it) is the vocabulary that makes DNS operational discussions
comprehensible, and §39.2 fixes it precisely.

## The record types worth knowing

Not all of them; the ones that appear in real work.

| Type | Purpose |
|---|---|
| `A` | Name → IPv4 address |
| `AAAA` | Name → IPv6 address |
| `CNAME` | Name → another name (an alias) |
| `MX` | Where to deliver mail for this domain, with priority |
| `NS` | Which servers are authoritative for this zone — the delegation itself |
| `SOA` | Zone metadata: serial, refresh, retry, expire, negative TTL |
| `TXT` | Arbitrary text; now carries SPF, DKIM, DMARC and domain-ownership proofs |
| `PTR` | Address → name, for reverse lookups |
| `SRV` | Service location: protocol, port, host — used heavily by Active Directory and SIP |
| `CAA` | Which certificate authorities may issue for this domain |

Two traps that produce real outages and appear on exams: a `CNAME` may not coexist
with other records at the same name, which is why you cannot `CNAME` a zone apex —
hence the various provider-specific `ALIAS`/`ANAME` workarounds. And a `PTR` record
lives in the `in-addr.arpa` tree and is delegated by whoever owns the *address*
space, not the name — so you usually cannot create your own reverse records, which
surprises people configuring mail servers.

## Why it is always DNS

The operations joke — *"It's not DNS. There's no way it's DNS. It was DNS."* — is a
joke because it is reliably true, and §39.4 explains why in structural terms rather
than as folklore.

**DNS is on the critical path for everything and is architecturally invisible.** A
DNS failure does not look like a DNS failure. It looks like "the website is down,"
or "email is broken," or "the application can't reach the database," and the first
three things anyone checks are the website, the mail server and the database.

**Caching makes failures non-deterministic.** Some users have the old answer and are
fine; others have expired and are not. The symptom is intermittent and appears to
correlate with nothing.

**TTLs mean mistakes have a long tail.** Publishing a wrong record with a 24-hour TTL
means the fix takes up to 24 hours to reach everyone, and lowering the TTL *after*
the incident does not help. The professional habit — lower TTLs to 300 seconds
several days before a planned change, restore them afterwards — is one of the more
valuable small practices in this book.

**Negative caching exists too**, per RFC 2308, so an NXDOMAIN is cached as well and a
record you just created may be invisible for the SOA's minimum TTL.

§39.4 also covers the security work: DNSSEC's chain of signatures (deployed, but far
from universally), and the encrypted transports DoT and DoH, which protect query
privacy from the network and simultaneously move visibility from network operators
to browser vendors — a shift with genuine arguments on both sides that the chapter
presents rather than adjudicates.

## By the end you will be able to

- Explain hierarchy, delegation and caching, and why each is necessary.
- Trace a recursive resolution and identify referrals versus authoritative answers.
- Distinguish stub, recursive, forwarding and authoritative servers.
- Read a zone file and identify each record type's function.
- Use `dig` to query a specific server, trace a delegation, and read the flags.
- Explain why a DNS change did not take effect, from the TTL and the SOA.
- Diagnose the four classic failures: wrong record, stale cache, broken delegation,
  and resolver unreachable.
