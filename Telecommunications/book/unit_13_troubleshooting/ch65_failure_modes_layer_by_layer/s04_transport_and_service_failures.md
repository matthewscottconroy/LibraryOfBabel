# 65.4 Transport and Service Failures

**The network delivers packets correctly and the application does not work.** **This is where
most incidents actually end up**, and it is where a network engineer's most useful contribution
is frequently proving that the network is not at fault.

## Blocked ports

**The commonest fault in this section**, and the distinction that identifies it is Chapter 64
§64.4's.

| Response to a connection attempt | Means |
|---|---|
| **Connection established** | it works |
| **RST returned immediately** | **you reached the host and nothing is listening** — or a reject rule |
| **No response at all — timeout** | **something dropped it silently** — a firewall, or the host is unreachable |
| **`Connection refused`** | the RST case, as the application reports it |
| **`Connection timed out`** | the drop case |

> **"Refused" and "timed out" are two entirely different faults.** **Refused means the packet
> arrived and something said no; timed out means the packet vanished.** **Every operating
> system's error message distinguishes them**, and a great deal of time is wasted by not
> reading which one appeared.

**And the checks, in order:**

```
   1.  Is anything listening?          ss -tlnp | grep :443     (on the server)
   2.  Is it listening on the right address?  0.0.0.0 vs 127.0.0.1
   3.  Is the host firewall permitting it?    iptables/nftables/Windows Firewall
   4.  Does the connection arrive?     tcpdump on the server (Ch 64 §64.3)
   5.  Which device dropped it?        the firewalls along the path, in order
```

**Step 2 catches more faults than it should:**

> **A service listening on `127.0.0.1:443` is reachable only from the machine itself.**
> **`ss -tlnp` shows the address, and "it works locally and not remotely" is this, very often** —
> and it is a configuration line in the application, not a network problem.

## Firewall and ACL faults

**Chapter 60 §60.1's material, as symptoms.**

| Symptom | Cause |
|---|---|
| **A rule is present, correct, and has no effect** | **shadowed by a rule above** (Chapter 60 §60.1) |
| **Outbound works, the reply is dropped** | **a stateless filter, or asymmetric routing** (§65.3) |
| **Small requests work, large ones fail** | **MTU** (Chapter 66 §66.3), not the firewall |
| **Works, then fails after a period of inactivity** | **the connection table entry timed out** (Chapter 60 §60.2) |
| **Works from one source and not another** | **the rule is source-specific**, or the sources take different paths |
| **Intermittent under load** | **connection table or session limit** |

**The counter is the evidence** (Chapter 63 §63.1): **increment a test connection and see which
rule's counter moves.** **If none does, the traffic is not reaching that device at all**, which
is itself a finding.

## DHCP

**Chapter 40's failure modes, as symptoms.**

| Symptom | Cause |
|---|---|
| **`169.254.x.x` on one device** | **that device's port, VLAN, or NIC** |
| **`169.254.x.x` on everything in one VLAN** | **the relay, or the server, or the scope** |
| **Addresses from the wrong scope** | **the relay's giaddr, or a rogue server** (Chapter 62 §62.1) |
| **Some devices get addresses and others do not** | **pool exhaustion** |
| **Intermittent duplicate-address conflicts** | **a static address inside the pool** (Chapter 53 §53.3) |
| **Works after a reboot and fails on renewal** | **the relay forwards DISCOVER and not RENEW** — unicast renewal is a different path |
| **A wrong gateway or DNS server for everyone** | **the scope options** |

**Pool exhaustion deserves the arithmetic:**

> **A `/24` with a pool from .100 to .200 has 101 addresses.** **A site that grew to 140 devices
> is short by 39, permanently.** **And shortening the lease does not create addresses** — **it
> only reclaims idle ones faster**, which helps a site with high turnover and does nothing for a
> site with 140 permanently-present devices.

**The check is the server's own statistics**, and **monitoring pool utilisation** (Chapter 54
§54.1) **is the preventive measure** — **because pool exhaustion arrives silently and presents
as an apparently random subset of users failing.**

## DNS

**Chapter 64 §64.2 covers the tooling; this is the symptom catalogue.**

| Symptom | Cause |
|---|---|
| **Nothing resolves** | **the resolver is unreachable, or wrong** |
| **One name does not resolve** | **the record, or a stale cache** |
| **Resolves to the wrong address** | **cache, split-horizon, or a modified record** |
| **Slow and then successful** | **the first configured resolver is timing out** |
| **Intermittent** | **one of several resolvers or authoritative servers is unhealthy** |
| **Works internally, fails externally (or the reverse)** | **split-horizon** (Chapter 39 §39.3) |
| **Fails after a record change** | **the TTL** — and it was not lowered in advance |
| **`SERVFAIL` on a valid name** | **DNSSEC validation failure**, or an upstream problem |

> **The single most useful DNS diagnostic remains `dig @<resolver>` against each configured
> resolver individually**, because **an aggregate test hides which one is broken.**

## Certificates and time

**Two service faults that present as network faults, and both are Chapter 58's.**

**Certificate faults:**

| Error | Cause |
|---|---|
| **"unable to get local issuer certificate"** | **the intermediate is not being sent** (Chapter 58 §58.4) — **and it works in browsers** |
| **"certificate has expired"** | **the most predictable outage in this book** |
| **"hostname mismatch"** | **no matching SAN** — the Common Name is ignored |
| **"self-signed certificate in chain"** | **a TLS-inspecting middlebox** (Chapter 60 §60.3) **whose CA the client does not trust** |
| **Works in a browser, fails everywhere else** | **the intermediate, almost always** |

**And time:**

> **Clock skew presents as an authentication failure, not as a time problem.** **Kerberos
> tolerates about five minutes; certificate validation fails when the current time is outside
> the validity window; and log correlation becomes impossible** (Chapter 54 §54.3).

**Which makes "check the clock" a standing item in any authentication investigation**, and it
takes one command.

## Application-layer symptoms with network causes

**The set worth recognising, because each has a specific network diagnosis.**

| Symptom | Network cause |
|---|---|
| **The page loads and images do not** | **MTU** — the small request works and the large response does not (Chapter 66 §66.3) |
| **Login works and file transfer hangs** | **the same** |
| **Fast for small files, slow for large** | **window size and RTT** (Chapter 64 §64.4), or loss |
| **Works from the office, not from home** | **VPN MTU, split tunnelling, or a different path** (Chapter 61) |
| **Fails only for the first request after idle** | **a NAT or firewall timeout, and the retry re-establishes** |
| **Every request takes exactly 5 seconds** | **a DNS timeout on one resolver, then falling back** |
| **Every request takes exactly 30 seconds then works** | **a connection attempt to an unreachable address first** — frequently IPv6 |
| **Slow for everyone at one site** | the site's link, or its DNS, or its gateway |

**The "exactly N seconds" pattern is the strongest signal in this table:**

> **A consistent, round delay is a timeout, and the value identifies which one.** **5 seconds is
> a DNS resolver timeout; 21 seconds is TCP's default connection timeout on Linux; 30 seconds is
> a common application timeout.** **Nothing in a working network produces a consistent delay of
> exactly five seconds.**

## Proving it is not the network

**The most valuable thing a network engineer produces in this section**, and it requires
evidence rather than assertion.

**The three-part demonstration:**

**1. The connection is established.** **A capture at both ends showing the handshake completing**
(Chapter 64 §64.3).

**2. The request arrives and the response is sent.** **Both visible in the server-side
capture.**

**3. The timing is in the application.** **The gap between the request arriving and the response
being sent is the server's processing time**, and it is measured directly in the capture.

```
   14:22:01.104  client → server  GET /report HTTP/1.1
   14:22:09.882  server → client  HTTP/1.1 200 OK
                 ▲
            8.8 seconds, entirely on the server
```

> **That single observation ends the argument.** **The network delivered the request in 1 ms and
> the response in 1 ms, and the server took 8.8 seconds** — **and it is not an assertion, it is
> a timestamp.**

**And the corollary is worth stating for professional reasons:** **produce this without
triumph.** **The point is to direct effort at the actual problem**, and an engineer who uses it
to win an argument will not be invited to the next investigation.

## The diagnostic sequence

```
   1.  Does the port respond?           nc -zv host port
   2.  Refused or timed out?            — two different faults
   3.  Is anything listening, and where? ss -tlnp on the server
   4.  Does the packet arrive?          tcpdump on the server
   5.  Is it DNS?                       dig @each resolver
   6.  Is it a certificate?             openssl s_client -showcerts
   7.  Is it the clock?                 date, on both
   8.  Where is the time going?         capture, and read the timestamps
```

## What breaks here

**"Connection refused" treated as a network problem.** **The packet arrived.** It is the service
or a reject rule.

**A service that works locally and not remotely.** **Listening on 127.0.0.1.**

**A rule with no effect.** **Shadowed** (Chapter 60 §60.1). Look upward.

**An apparently random subset of users without addresses.** **DHCP pool exhaustion**, and
shortening the lease will not fix it.

**A page that loads with missing images.** **MTU**, not the web server.

**Requests that take exactly five seconds.** **A DNS resolver timing out.**

**Authentication failing across the estate.** **Check the clock before anything else.**

**A certificate error in one client and not in a browser.** **The intermediate is not being
sent.**

**Everything correct and the application still failing.** **Capture at both ends and read the
timestamps** — and the answer is frequently that the network is fine.

> **Network+ note.** Objective 5.3 and 5.4. Over-learn: **blocked ports and services**;
> **DHCP scope exhaustion and its symptoms**; **incorrect DNS settings and their effects**;
> **expired certificates**; **untrusted certificate authorities**; **time synchronisation
> issues**; and **the distinction between a refused connection and a timeout.** The
> refused-versus-timeout distinction and the APIPA/DHCP relationship are both examined
> regularly.
