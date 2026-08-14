# 64.2 Name and Address Tools

Chapter 63 §63.3 identified "does it work by IP but not by name?" as the highest-yield single
test. This section is what you do after the answer is yes.

## Local address configuration

Establish what the host believes about itself, before anything else.

| | Linux (`iproute2`) | Windows | macOS |
|---|---|---|---|
| **Addresses** | **`ip addr`** | `ipconfig /all` | `ifconfig` |
| **Routes** | **`ip route`** | `route print` | `netstat -rn` |
| **Neighbours** | **`ip neigh`** | `arp -a` | `arp -a` |
| **Interface state** | **`ip -s link`** | `netsh interface show` | `ifconfig` |
| **Listening sockets** | **`ss -tlnp`** | `netstat -ano` | `lsof -i` |

The deprecated tools — `ifconfig`, `route`, `netstat`, `arp` on Linux — still work and are
frozen. `ip` and `ss` show things they cannot, notably multiple addresses per interface,
policy routing, and socket process ownership.

**What to look for, in order:**

```
   $ ip addr show dev eth0
   2: eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 …
       inet 10.20.5.14/24 brd 10.20.5.255 scope global dynamic eth0
          valid_lft 42311sec preferred_lft 42311sec
       inet6 fe80::a00:27ff:fe4e:66a1/64 scope link
```

| Check | Wrong looks like |
|---|---|
| **`UP` and `LOWER_UP`** | **`UP` without `LOWER_UP` means no carrier — a cable problem** |
| **The address** | **`169.254.x.x` means DHCP failed** (Chapter 40 §40.4) |
| **The prefix length** | **`/24` where the subnet is a `/22`** — Chapter 65 §65.3's classic |
| **`valid_lft`** | **a lease about to expire, or a static address where DHCP was expected** |
| **The MTU** | 1500, or the tunnel's value (Chapter 61 §61.1) |
| **IPv6** | **link-local only means no RA received** (Chapter 28 §28.3) |

**And the route table:**

```
   $ ip route
   default via 10.20.5.1 dev eth0 proto dhcp metric 100
   10.20.5.0/24 dev eth0 proto kernel scope link src 10.20.5.14
   10.9.0.0/16 via 10.20.5.9 dev eth0 proto static
```

> Two default routes with the same metric is a fault, and it produces intermittent
> behaviour that looks like anything but a routing problem. `ip route get <destination>` asks
> the kernel which route it would actually use, which removes the guesswork entirely.

## ARP and neighbour tables

**Chapter 18's mechanism, inspected.**

```
   $ ip neigh
   10.20.5.1 dev eth0 lladdr 00:1a:2b:3c:4d:5e REACHABLE
   10.20.5.99 dev eth0 lladdr 00:aa:bb:cc:dd:ee STALE
   10.20.5.42 dev eth0  FAILED
```

| State | Means |
|---|---|
| **`REACHABLE`** | confirmed recently |
| **`STALE`** | **known, unverified — normal, and will be re-verified on use** |
| **`INCOMPLETE`** | **a request was sent and nothing answered** |
| **`FAILED`** | **resolution failed — the host is not on this segment, or is down** |
| `PERMANENT` | statically configured |

`FAILED` for the default gateway is a specific and unambiguous finding: the host cannot
resolve its gateway's address, which means the gateway is down, or the host is in the wrong
VLAN, or the address or mask is wrong (Chapter 65 §65.3).

And two entries with the same MAC and different IPs — or the same IP appearing with two MACs
over time — is either ARP spoofing or a duplicate address (Chapter 62 §62.1, Chapter 53
§53.3). The switch's MAC table distinguishes them: a duplicate address appears on two ports.

## dig

The DNS tool worth learning properly (Chapter 39).

```
   $ dig app.example.com

   ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 41291
   ;; flags: qr rd ra; QUERY: 1, ANSWER: 2, AUTHORITY: 0, ADDITIONAL: 1

   ;; ANSWER SECTION:
   app.example.com.     300  IN  CNAME  app-lb.example.net.
   app-lb.example.net.   60  IN  A      203.0.113.10

   ;; Query time: 14 msec
   ;; SERVER: 10.9.0.9#53(10.9.0.9)
```

Five things in that output are worth reading every time:

| | |
|---|---|
| **`status`** | **`NOERROR`, `NXDOMAIN` (no such name), `SERVFAIL` (the resolver failed), `REFUSED`** |
| **flags** | **`aa`** authoritative, **`ra`** recursion available, **`ad`** DNSSEC-validated |
| **TTL** | **`60` means the answer changes often — and it bounds how long a change takes to propagate** |
| **`SERVER`** | **which resolver answered** — and it is frequently not the one you assumed |
| **Query time** | **a slow resolver is a slow application** |

### The queries that answer questions

| Command | Answers |
|---|---|
| **`dig +short name`** | **just the answer** — for scripts |
| **`dig @8.8.8.8 name`** | **ask a specific resolver** — is it my resolver or the record? |
| **`dig +trace name`** | **walk from the root down** — where in the delegation is it wrong? |
| **`dig name +norecurse`** | **is it in this resolver's cache?** |
| **`dig -x 203.0.113.10`** | **reverse lookup** |
| **`dig name ANY`** / **`NS`** / **`MX`** / **`TXT`** / **`SOA`** | specific record types |
| **`dig name @ns1.example.com`** | **ask the authoritative server directly** — bypassing every cache |

**The two that solve most problems:**

> **`dig @<the authoritative server> name` versus `dig @<your resolver> name`.** If they
> differ, the answer is cached and stale — and the TTL tells you for how long. If they
> agree, the record itself is what you think it is and the problem is elsewhere.

**And `dig +trace`** shows the delegation chain, which is how you find a broken NS record, a
lame delegation, or a DNSSEC failure — the point at which the trace stops is the point at
which the delegation is wrong.

`nslookup` still exists and is on every Windows machine, and its output is less
informative and its behaviour differs between platforms. Use `dig` where it is available;
`Resolve-DnsName` is the better Windows option.

## Reading DNS failures

| Symptom | Cause |
|---|---|
| **`NXDOMAIN`** | **the name does not exist** — a typo, or a record that was removed |
| **`SERVFAIL`** | **the resolver could not get an answer** — upstream failure, or **DNSSEC validation failure** |
| **`REFUSED`** | **the server will not answer you** — an ACL, or you queried a server that is not authoritative and does not recurse |
| **Correct answer, wrong address** | **stale cache**, or **split-horizon DNS giving you the external view** |
| **Resolves on one machine, not another** | **different resolvers** — check the `SERVER` line |
| **Slow resolution, then success** | **the first resolver is timing out** and the second is answering |
| **Works, then fails, then works** | **one of several resolvers or authoritative servers is broken** — query each individually |

**The last one is worth a note.** A host with three configured resolvers will use them in
some order and fall back, so an intermittent DNS failure frequently means one of the three
is unhealthy. Query each explicitly — `dig @<each one>` — rather than testing the
aggregate.

**And split-horizon** (Chapter 39 §39.3) is the cause of a specific confusing symptom:
the name resolves to a public address from inside the network, so traffic hairpins out to
the firewall and back, or fails entirely. Diagnosed by comparing the answer from the internal
and external resolvers.

## Windows-specific

| Command | Does |
|---|---|
| **`ipconfig /all`** | **addresses, DNS servers, DHCP server, lease times** |
| **`ipconfig /release` `/renew`** | **DHCP** |
| **`ipconfig /flushdns`** | **clear the client cache** — the first thing to try after a DNS change |
| **`ipconfig /displaydns`** | **what is cached, with remaining TTLs** |
| **`nbtstat -n`** | NetBIOS, on older estates |
| **`Test-NetConnection -Port`** | **the closest thing to `nc -zv`**, and it is built in |
| **`Get-NetTCPConnection`** | **`ss`'s equivalent** |
| **`Resolve-DnsName -Server`** | **`dig @server`'s equivalent** |

**`ipconfig /displaydns` is the under-used one:** it shows what the client has cached and for
how long, which answers "why is this machine still using the old address?" immediately.

## What breaks here

**`UP` with no `LOWER_UP`.** **No carrier.** A cable, an SFP, or the far end is down.

**`169.254.x.x`.** **DHCP failed** (Chapter 40 §40.4), and the host is on the network.

A `/24` where the subnet is a `/22`. Local destinations become remote and go via the
gateway, or remote ones are treated as local and ARP fails. Chapter 65 §65.3.

`FAILED` in the neighbour table for the gateway. Wrong VLAN, wrong subnet, or the gateway
is down.

**Two default routes with equal metrics.** Intermittent, and it will not look like a routing
problem.

Resolution works from one machine and not another. **Different resolvers.** Check `SERVER`.

**`SERVFAIL` on a name that exists.** Upstream failure or DNSSEC validation. `dig +trace`
locates it.

A record changed and clients still using the old value. The TTL, plus the client's own
cache. `ipconfig /flushdns`, and next time lower the TTL in advance.

An internal name resolving to a public address. Split-horizon, and you received the
external view.

**Intermittent DNS failures.** One of several configured resolvers is unhealthy. Query each.

> **Network+ note.** Objective 5.5 covers these. Over-learn: **`ipconfig`/`ifconfig`/`ip` show
> interface configuration**; **`nslookup`/`dig` query DNS**; **`arp -a` shows the ARP cache**;
> **`netstat`/`ss` show connections and listening ports**; **`ipconfig /flushdns` clears the
> resolver cache**; and **an APIPA address means DHCP failed.** The APIPA point is examined in
> almost every form.
