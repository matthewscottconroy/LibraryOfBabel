# 40.2 DORA

Four messages. The acronym is the standard mnemonic and the mechanism repays a closer look
than the acronym gives it — particularly the two places where a **broadcast** is used and
the reason each is unavoidable.

## The exchange

```
   Client                                              Server
   (no address)                                        10.0.0.53
     │                                                     │
     │── DISCOVER ────────────────────────────────────────▶│
     │   src 0.0.0.0:68  dst 255.255.255.255:67            │
     │   "is there a DHCP server?"                         │
     │                                                     │
     │◀──────────────────────────────────────── OFFER ─────│
     │   src 10.0.0.53:67  dst 255.255.255.255:68          │
     │   "you may have 10.0.0.100, here are the options"   │
     │                                                     │
     │── REQUEST ─────────────────────────────────────────▶│
     │   src 0.0.0.0:68  dst 255.255.255.255:67            │
     │   "I accept 10.0.0.100 from server 10.0.0.53"       │
     │                                                     │
     │◀───────────────────────────────────────── ACK ──────│
     │   "confirmed, lease 86400 seconds"                  │
     │                                                     │
```

**D-O-R-A: Discover, Offer, Request, Acknowledge.**

## Why the client must broadcast

**The client has no address and does not know the server's.**

**Both halves matter.** It cannot put a source address in the packet because it has none —
so it uses **`0.0.0.0`**, the "this host" address (Chapter 27 §27.2). And it cannot address
the server because it does not know one exists — so it uses **`255.255.255.255`**, the
limited broadcast, which needs no resolution.

> **This is the bootstrap problem** (Chapter 18 §18.2), and broadcast is the only mechanism
> that solves it. ARP has it, DHCP has it, and in both cases the answer is the same.

**And it means DHCP cannot cross a router unaided**, because a limited broadcast is never
forwarded (Chapter 27 §27.3). §40.4's relay agent is the answer.

## Why the *server* broadcasts too

**Less obvious, and worth understanding.**

**The OFFER is sent to `255.255.255.255`, not to the address being offered.**

**Because the client does not have that address yet.** If the server unicast the offer to
`10.0.0.100`, it would have to ARP for it — and nothing answers, because no host holds it.

**Some clients set the `BROADCAST` flag** in their DISCOVER to request this explicitly;
others can receive a unicast to an address they have not yet configured, and servers may
unicast to those. **The flag exists because implementations differ**, and a client whose
stack cannot accept such a packet sets it.

## Message by message

### DISCOVER

**The client's opening broadcast.** It carries:

- **Its MAC address** in the `chaddr` field — **the identity the server keys on**
- **A transaction ID (`xid`)** — a random number matching this exchange's messages
- Optionally, a **requested address** (option 50) — *"I had 10.0.0.100 before, may I have
  it again?"*
- **A parameter request list** (option 55) — *"tell me the mask, gateway, DNS, domain,
  NTP…"*

**Option 55 is worth noticing.** The client says what it wants, and a server only sends
options that were asked for. **Which is why a device sometimes does not receive an option
that is configured** — it never requested it.

### OFFER

**A server's proposal.** Address, mask, gateway, DNS, lease time, and the **server
identifier** (option 54).

**Several servers may offer.** They all hear the broadcast; each may reply. **The client
receives several offers and picks one** — conventionally the first to arrive.

**This is how DHCP redundancy works, and how a rogue server does damage** (§40.4): **there
is no authentication and no arbitration.** Whichever server answers fastest wins, and the
client has no way to tell a legitimate server from a laptop running one.

**The offered address is reserved but not committed.** A server that offers and never hears
a REQUEST releases it after a short timeout.

### REQUEST

**The client's acceptance — and it is broadcast, deliberately.**

**Two jobs in one message:**

**1. It tells the chosen server yes.**

**2. It tells the *other* servers no.** The REQUEST carries the **server identifier**
(option 54) of the chosen server, so every other server that made an offer hears it, sees a
different identifier, and **releases the address it had reserved.**

> **The REQUEST is broadcast so that the servers whose offers were declined can free their
> reservations.** A unicast REQUEST would leave every non-chosen server holding an address
> until its timeout.

**This is the detail the acronym hides**, and it is the reason the third message is not
simply a unicast confirmation.

### ACK

**The server commits.** The lease is now bound: the server records the MAC, the address and
the expiry, and the client configures its interface.

**And the client should verify.** RFC 2131 recommends **ARP for the address before using
it** (Chapter 18 §18.3's duplicate detection) — if something answers, the address is already
in use, and the client sends a **DECLINE**, which tells the server to mark it bad and
re-offer.

**Not every client does this**, which is why a static address inside a DHCP pool produces a
duplicate rather than a clean rejection.

### NAK — the fifth message

**When the server refuses.**

```
   Client:  REQUEST 10.0.0.100  ("I had this before")
   Server:  NAK                 ("not on this network you don't")
```

**The classic case is a laptop that moved.** It suspends at the office holding
`10.0.0.100`, opens at home, and requests the same address — and the home server, whose pool
is `192.168.1.0/24`, **NAKs it.** The client discards its configuration and starts DORA
afresh.

**A NAK is the mechanism that makes mobility work**, and seeing one in a capture is normal
rather than alarming.

## Renewal — the part that is not DORA

**A lease is renewed long before it expires, and the timers are worth knowing because they
explain DHCP's resilience.**

| Timer | At | Action |
|---|---|---|
| **T1** | **50% of the lease** | **unicast REQUEST to the original server** |
| **T2** | **87.5% (7/8)** | **broadcast REQUEST to any server** |
| Expiry | 100% | **release the address; start DORA again** |

**With a 24-hour lease:**

```
   0h                12h                 21h            24h
   │─────────────────│───────────────────│──────────────│
   ACK              T1                  T2           expiry
                 renew with           try ANY       give up
                 my server            server
```

**The design is deliberately forgiving:**

**At T1 the client asks its own server**, by unicast — a two-message exchange (REQUEST,
ACK), not the full four. **No broadcast, no discovery, minimal traffic.**

**If that fails, nothing happens yet.** The client keeps using the address and retries.

**At T2 it broadcasts**, asking any server — because its own may be gone for good.

**Only at expiry does it give up.**

> **A client with a 24-hour lease survives a twelve-hour DHCP outage without noticing.**
> This is the most under-appreciated property of the protocol, and it is why a DHCP server
> failure is often discovered hours later, by the first host to boot rather than by anyone
> already running.

**And it is why lease length is a resilience decision** (§40.1), not merely an
address-management one.

## RELEASE and DECLINE

**RELEASE** — *"I am finished with this address"*, sent when an interface is shut down
cleanly. **Unicast to the server.**

**Frequently not sent.** A laptop closed and carried away, a machine that crashes, a VM
destroyed — **none send a RELEASE**, which is exactly why the lease exists.

**DECLINE** — *"this address is already in use"*, after the ARP check found a conflict. The
server marks it unavailable and offers another.

**A pool with several declined addresses is evidence of static assignments inside the
dynamic range**, and it is worth looking at.

## INFORM

**The message people forget exists.**

**A host with a statically configured address that wants the *other* options** — DNS
servers, domain name, NTP, proxy configuration — sends **INFORM**, and the server replies
with an ACK containing options but **no address and no lease.**

**Used by statically-addressed servers** that still want centrally-managed DNS and domain
settings, and by Windows for proxy discovery.

## Reading it

```bash
tcpdump -i eth0 -nn port 67 or port 68
```

```
IP 0.0.0.0.68 > 255.255.255.255.67: BOOTP/DHCP, Request from aa:bb:cc:dd:ee:ff, length 300
IP 10.0.0.53.67 > 255.255.255.255.68: BOOTP/DHCP, Reply, length 300
IP 0.0.0.0.68 > 255.255.255.255.67: BOOTP/DHCP, Request from aa:bb:cc:dd:ee:ff, length 300
IP 10.0.0.53.67 > 255.255.255.255.68: BOOTP/DHCP, Reply, length 300
```

**`tcpdump` labels everything "BOOTP/DHCP"** — the inheritance of §40.1, visible.

**In Wireshark, filter `dhcp` (or `bootp` on older versions)** and expand the options; the
message type is **option 53**, and it is what distinguishes DISCOVER from REQUEST.

**Client-side:**

```bash
# Linux
journalctl -u NetworkManager | grep -i dhcp
dhclient -v eth0                     # run in the foreground and watch
cat /var/lib/dhcp/dhclient.leases

# Windows
ipconfig /all                        # lease obtained and expiry
ipconfig /release && ipconfig /renew

# macOS
ipconfig getpacket en0               # the full DHCP packet, decoded
```

**`ipconfig getpacket` on macOS is unusually good** — it prints every option the client
received, which answers "did the server actually send option 43?" directly.

## What breaks here

**`169.254.x.x`.** No reply to DISCOVER. §40.4's causes.

**An address obtained, and something missing.** The client did not request that option
(55), or the server does not have it configured.

**A NAK on connecting to a new network.** Normal — the client asked for an address from
elsewhere.

**Duplicate addresses despite DHCP.** A static address inside the pool, and a client that
does not ARP-check.

**A DHCP outage nobody noticed for hours.** Leases renewing at T1 without incident. Working
as designed, and it is why DHCP needs its own monitoring.

**Several offers and a client choosing the wrong one.** No authentication and no
arbitration — §40.4's rogue server.

> **Network+ note.** Objective 2.3 expects DORA, and **it is examined directly.**
> Over-learn: **Discover, Offer, Request, Acknowledge**; **the client broadcasts from
> `0.0.0.0` to `255.255.255.255`**; **UDP 67 server, 68 client**; **renewal at 50% (T1) and
> 87.5% (T2)**; and **NAK means the requested address is not valid on this network.** The
> T1/T2 percentages appear as recall items.
