# 59.2 802.1X and RADIUS

The mechanism that makes port-based network access control possible, and the reason a
device cannot simply be plugged into a wall socket to gain access.

## The three roles

```
   ┌────────────┐  EAP over LAN  ┌───────────────┐   RADIUS    ┌──────────────┐
   │ Supplicant │◀──────────────▶│ Authenticator │◀───────────▶│ Auth. server │
   │  (device)  │   (EAPOL)      │ (switch / AP) │  UDP 1812   │   (RADIUS)   │
   └────────────┘                └───────────────┘             └──────┬───────┘
                                                                      │
                                                               ┌──────┴───────┐
                                                               │  Directory   │
                                                               │  (LDAP/AD)   │
                                                               └──────────────┘
```

| Role | Is | Does |
|---|---|---|
| **Supplicant** | **the device** | presents credentials |
| **Authenticator** | **the switch or access point** | **relays; decides nothing** |
| **Authentication server** | **RADIUS** | **decides, and returns attributes** |

**The authenticator's ignorance is the design's strength.** The switch does not know what a
certificate is or how to check a password. It relays EAP frames and applies whatever the
server tells it, which means the policy lives in one place and the switches need no
configuration when it changes.

## What the port does before authentication

> **Until authentication succeeds, the port forwards nothing but EAP frames.** **The device has
> link, and no network.**

**Which is the property that matters:** an unauthorised device in a meeting room's wall socket
gets a link light and nothing else. No DHCP, no ARP, no traffic of any kind.

And the frames are EAPOL — EAP over LAN — addressed to a reserved multicast address that
the switch intercepts. They are not IP; they do not need an address; they work before
anything else does.

## EAP methods, and why the choice matters

EAP is a framework, not an authentication method. The method carried inside it is the
thing being chosen, and the choices differ enormously in security.

| Method | Client credential | Server credential | Assessment |
|---|---|---|---|
| **EAP-TLS** | **certificate** | **certificate** | **the strongest; mutual; no password to phish** |
| **EAP-TTLS** | password (inside TLS) | **certificate** | good, if the server certificate is validated |
| **PEAP (MSCHAPv2)** | **password (inside TLS)** | **certificate** | **good, if validated — and it usually is not** |
| **EAP-FAST** | password / PAC | PAC | Cisco; adequate |
| **EAP-MD5** | password hash | **none** | **broken. No mutual authentication, offline-crackable.** |
| **EAP-PWD, EAP-pwd** | password | — | resistant to dictionary attack; little deployed |

The critical caveat applies to every password-based method:

> PEAP and EAP-TTLS protect the password inside a TLS tunnel — and only if the client
> validates the server's certificate. A client configured to accept any certificate will
> happily tunnel its credentials to an attacker's access point, and "accept any certificate"
> is the default on a distressing number of clients and the path of least resistance for
> whoever set it up.

Which is the single most common 802.1X misconfiguration, and its consequence is that an
attacker with a laptop and a Wi-Fi card in the car park collects credentials.

**The correct client configuration specifies:** the CA that must have issued the server
certificate, the expected server name, and "do not prompt the user" — because a prompt
that says "the certificate is not trusted, continue?" will be accepted.

> **EAP-TLS removes the problem entirely.** There is no password to steal, mutual
> authentication is inherent, and a stolen certificate can be revoked. **Its cost is a PKI**
> (Chapter 58 §58.4) **and device enrolment**, which is real work and is the reason it is not
> universal.

## RADIUS

Remote Authentication Dial-In User Service — and the name tells you how old it is.

| | |
|---|---|
| Transport | **UDP 1812 (auth), 1813 (accounting)** — historically 1645/1646 |
| Authentication of the exchange | **a shared secret between authenticator and server** |
| **Encryption** | **only the password field, and with MD5** |
| Attributes | **AVPs — extensible, and vendor-specific ones are heavily used** |
| Modern transport | **RADIUS over TLS (RadSec, RFC 6614)** |

**Three honest observations about the protocol:**

It is old and its cryptography is poor. Only the User-Password attribute is obscured, using
an MD5-based construction, and everything else — usernames, attributes, accounting records —
crosses in the clear. RADIUS traffic on an untrusted path is a disclosure.

The shared secret is per-client and rarely rotated. Every switch has one, they are usually
identical across the estate, they are in every configuration backup (Chapter 55 §55.4), and
rotating them requires touching every device.

**And UDP means silent failure.** A RADIUS server that is unreachable produces a timeout, and
what the authenticator does then is a policy decision you must make — see the failure modes
below.

RadSec (RADIUS over TLS on TCP 2083) fixes the transport, and **deployment is patchy.**
Where the RADIUS path crosses anything untrusted, it should be used or the traffic should be
in a management VRF (Chapter 60 §60.4).

## What RADIUS returns, and why it is the interesting part

Authentication is a yes or no. The attributes are what makes 802.1X useful.

```
   Access-Accept
     Tunnel-Type            = VLAN
     Tunnel-Medium-Type     = 802
     Tunnel-Private-Group-ID = 240        ← put this port in VLAN 240
     Filter-Id              = CONTRACTOR   ← apply this ACL
     Session-Timeout        = 3600
     Vendor-specific: dACL, SGT, role, bandwidth limit …
```

> The port's VLAN, its ACL and its policy are decided per authentication, by the server, from
> the directory. A contractor's laptop and a finance workstation plug into identical ports
> and land in different networks with different rules.

Which is dynamic segmentation, and it is Chapter 60 §60.4's argument implemented at the
access edge — and it is the strongest reason to deploy 802.1X, well beyond keeping strangers
out.

## The devices that cannot authenticate

**The practical problem that dominates real deployments.**

Printers, cameras, badge readers, building management controllers, medical devices, industrial
equipment — many have no 802.1X supplicant at all, and many that do have one implemented
badly.

**The fallbacks, in decreasing order of goodness:**

| Mechanism | How | Weakness |
|---|---|---|
| **MAB** — MAC Authentication Bypass | **the switch sends the MAC as the username** | **MAC addresses are trivially spoofed** |
| **Profiling** | **the server fingerprints the device** — DHCP options, HTTP user agent, CDP/LLDP | **also spoofable, and better than MAB alone** |
| **Static port configuration** | a designated VLAN, no authentication | **an unauthenticated port** |

> MAB is not authentication. It is identification of a claim anyone can make. Its value is
> that it puts the device in a restricted VLAN with an ACL that permits only what that device
> class needs — so spoofing a printer's MAC gets you a printer's access, which is very
> little.

Which is the correct way to think about it: MAB plus tight authorisation is defensible;
MAB into the general network is theatre.

## Failure modes, which must be designed

The question that decides whether 802.1X is deployable: what happens when the RADIUS server is
unreachable?

| Mode | Behaviour | Use |
|---|---|---|
| **Fail-closed** | **no access** | **high-security sites, and it means a RADIUS outage is a total outage** |
| **Critical VLAN / auth-fail VLAN** | **place the port in a restricted VLAN** | **the usual compromise** |
| **Fail-open** | **full access** | **defeats the control, and is sometimes right** |
| **Inactivity / re-auth timers** | **already-authenticated sessions persist** | **essential — otherwise a RADIUS blip disconnects the estate** |

Chapter 57 §57.2's fail-open/fail-closed argument, in its most concrete form. And the
usual correct answer is a critical VLAN with enough access to work degraded — users reach
the file server and not the Internet, or reach nothing but the helpdesk.

Two further points that are learned during deployment:

**Already-authenticated sessions must survive a server outage.** A switch that re-authenticates
every 3,600 seconds and cannot reach the server will drop every device, in a wave. Set
re-authentication generously and configure the switch to retain sessions when the server is
unreachable.

**Deploy in monitor mode first.** Every 802.1X implementation supports a mode in which
authentication is performed and the result is logged rather than enforced. Run it for weeks.
It finds the devices you did not know about, which is always more than expected, and it
converts a disruptive project into a boring one.

## TACACS+, and when to use which

A different protocol for a different job, and the distinction is regularly muddled.

| | **RADIUS** | **TACACS+** |
|---|---|---|
| Standard | **RFC, open** | **Cisco-originated; documented as RFC 8907 (2020)** |
| Transport | **UDP** | **TCP 49** |
| **Encryption** | **the password field only** | **the entire payload** |
| **AAA separation** | **authentication and authorization combined** | **separate — and this is the point** |
| Per-command control | **no** | **yes** |
| **Best for** | **network access** — 802.1X, VPN, Wi-Fi | **device administration** |

> **TACACS+'s separation of authorization from authentication is what makes per-command
> authorisation possible.** A junior engineer may run `show` commands and not `configure`,
> and every command they attempt is authorised individually and logged — which RADIUS cannot
> do.

So the standard arrangement in a mature network is both: RADIUS for who may join the
network, TACACS+ for who may configure the devices. They are not competitors.

## What breaks here

**A device with link and no network.** 802.1X working as designed. Check the authentication
log before anything else.

**Credentials harvested from a car park.** PEAP with server certificate validation
disabled. The single most common misconfiguration.

**A RADIUS outage disconnecting the whole estate.** Re-authentication timers too aggressive,
and no session retention. Design the failure mode.

**MAB granting full network access.** **Not authentication.** Restrict what the resulting
authorisation permits.

**A printer that authenticated as a workstation.** MAC spoofing, and MAB. Profiling plus
tight authorisation.

RADIUS shared secrets identical across 200 switches and in every backup. Common, and it is
a real exposure. Per-device secrets, or RadSec.

Users prompted to trust an unknown certificate and doing so. A prompt is not a control.
Configure the client to specify the CA and the server name, and to refuse.

An 802.1X rollout that caused a week of outages. **No monitor mode.** It would have found the
unknown devices before enforcement did.

**Per-command authorisation attempted with RADIUS.** **It cannot.** TACACS+.

> **Network+ note.** Objective 4.1 and 4.3 cover 802.1X, RADIUS and TACACS+. Over-learn:
> 802.1X provides port-based network access control with supplicant, authenticator and
> authentication server; **EAP is the framework and EAP-TLS uses certificates**; RADIUS uses
> UDP and encrypts only the password, TACACS+ uses TCP and encrypts the whole payload;
> **TACACS+ separates AAA and supports per-command authorisation**; and **MAB authenticates by
> MAC address.** The RADIUS/TACACS+ comparison is examined in almost every form.
