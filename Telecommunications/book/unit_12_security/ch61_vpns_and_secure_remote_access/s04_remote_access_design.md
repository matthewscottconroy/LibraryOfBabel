# 61.4 Remote Access Design

**The chapter's practical section**, and the argument it must confront is that **the thing this
chapter has been describing is being replaced.**

## Sizing a concentrator

**The arithmetic that was got wrong at scale in March 2020.**

| | |
|---|---|
| **Concurrent users** | **not total users** — and the ratio changed permanently |
| **Bandwidth per user** | **depends entirely on split versus full tunnel** |
| **Throughput with crypto** | **the data sheet's encrypted figure, not its headline** |
| **Licences** | **frequently the binding constraint, and the one nobody checks** |

**A worked example, for 1,200 staff:**

| | Full tunnel | Split tunnel |
|---|---|---|
| Concurrent, normal | **35%** = 420 | 420 |
| **Concurrent, everyone remote** | **90%** = **1,080** | 1,080 |
| **Per user** | **~2.5 Mb/s** (all traffic, including video) | **~0.4 Mb/s** (corporate only) |
| **Aggregate, worst case** | **2.7 Gb/s** | **430 Mb/s** |

> **The full-tunnel figure is six times the split-tunnel one**, and **it is why organisations
> that full-tunnelled discovered their concentrators, their Internet circuits and their
> firewalls were all undersized simultaneously.**

**Three sizing errors that recur:**

**Sizing for normal concurrency.** **The event that makes everyone work remotely is exactly the
event during which the VPN matters**, and it is not a rare category — weather, transport
disruption, a building problem, a pandemic.

**Ignoring licensing.** **Many products licence by concurrent session**, and **the licence limit
produces a hard, immediate, confusing failure** — users simply cannot connect, with an error
that does not say why.

**Sizing the concentrator and not the path.** **The concentrator, its Internet circuit, the
firewall in front of it and the internal path to the applications must all carry the load**,
and Chapter 56 §56.1's series argument applies.

## Redundancy

**A VPN concentrator is a single point of failure for the entire remote workforce**, and it
should be designed as Chapter 56 §56.2 requires.

| Approach | Failover | Notes |
|---|---|---|
| **Active/standby pair** | **sessions drop and reconnect** | simplest; users notice |
| **Active/active with a load balancer** | **sessions drop** unless state is shared | more capacity |
| **Multiple gateways, DNS or client-side selection** | **client reconnects to another** | **geographic diversity, and the client does the work** |
| **Cloud-delivered service** | **the provider's problem** | Chapter 51 §51.2's SASE |

**And the properties to check are Chapter 56's:**

**Test the failover.** **A standby concentrator whose configuration has drifted, or whose
licence has not been applied, fails over into a broken state.**

**Check the shared fate.** **Two concentrators in one data centre, behind one firewall pair, on
one Internet circuit, are one concentrator.**

**And know what happens to sessions.** **Reconnection is acceptable for most work and is not
acceptable for a voice call or a long-running clinical session**, and **stating which applies is
a design decision.**

## The design questions, answered

**Six, and each has a defensible answer.**

**Split or full tunnel?** **Split, for almost everyone** (§61.1), **with inspection provided by
an endpoint agent or a cloud service rather than by the tunnel.** **Full tunnel only where
regulation requires all traffic to be inspected on premises**, and then size for it.

**What authentication?** **Certificate plus a phishing-resistant factor** (Chapter 59 §59.1).
**A VPN protected by a password alone is the single most productive target in most
organisations**, and it is how a large share of ransomware incidents begin (Chapter 57 §57.1).

**What does a connected client reach?** **This is the question that matters most and is answered
least.** **The default in most deployments is "everything", and it should be "what this user's
role requires"** — **enforced by an ACL applied per user group at the concentrator**, which every
product supports and few deployments configure.

**Is device posture checked?** **If not, a personal machine with an unknown patch level is on
the network** (Chapter 59 §59.4). **Deciding this explicitly is the difference between a policy
and an accident.**

**Always-on or user-initiated?** **Always-on gives consistent policy and no user decision**; **it
also means the device is on the corporate network permanently, and captive portals require
special handling.** **Always-on with split tunnelling is the common modern answer.**

**Where does it terminate?** **Not in the data centre, if the applications are not there**
(Chapter 51 §51.4's tromboning). **Terminate near the users or near the applications**, and for
a distributed organisation that argues for a cloud-delivered service.

## The argument that this is ending

**Stated honestly, because the chapter's subject is being superseded and pretending otherwise
would be dishonest.**

> **A VPN exists to place a user "on the network", which was meaningful when the applications
> were on that network.** **They are not.** **A VPN that connects a user to a data centre so
> they can reach a cloud service is doing work for no reason** (Chapter 51 §51.4).

**And the security argument is Chapter 59 §59.4's:**

| | **VPN** | **Zero trust access** |
|---|---|---|
| Grants | **network access** | **per-application access** |
| After connecting, reachable | **everything routable** | **what was granted** |
| Lateral movement | **possible** | **not implied** |
| The gateway is | **an Internet-facing device with a network behind it** | **a broker; the resource has no listener** |
| Device posture | **optional** | **integral** |

**The honest qualifications:**

**Legacy applications still need it.** **A twenty-year-old system with no modern authentication,
or one that requires a Layer 3 path for a thick client, cannot be brokered easily.** **Every real
deployment retains a VPN for these**, and **the correct architecture is a small VPN reaching a
tightly segmented zone** (Chapter 60 §60.4), **not the general-purpose VPN that exists now.**

**Site-to-site is not going away.** **Two networks that must be joined still need a tunnel**, and
**IPsec is still the answer.** **The zero trust argument is about user access, not about
connecting networks** — and conflating them is a common error in vendor material.

**And the transition is long.** **Chapter 59's BeyondCorp reference: it took years, the device
inventory did not exist, and the legacy tier required a proxy.** **An organisation planning to
"remove the VPN" in a quarter has not read anyone's account of doing it.**

> **The realistic destination: a much smaller VPN, reaching a much smaller network, used by
> fewer people for fewer things** — **with per-application brokered access carrying everything
> that can be moved to it.** **Which is a reduction in blast radius rather than an elimination
> of a technology**, and it is achievable in a way that "remove the VPN" is not.

## What breaks here

**Everyone connects and nothing works.** **Concurrency, licence limit, or the path behind the
concentrator.** **Check the licence count first** — it is the fastest to eliminate and the most
confusing when it is the cause.

**The VPN is slow for everyone.** **Full tunnelling and tromboning**, or MTU (§61.1), or the
concentrator's encrypted throughput.

**A VPN compromise leading to a ransomware incident.** **Password-only authentication.** **This
is the single commonest initial access route into organisations with a VPN**, and MFA on it is
the highest-value control available.

**A connected contractor reaching the finance servers.** **The concentrator applies no per-group
policy.** Every product supports it.

**Failover to a standby that does not work.** **Configuration drift or an unapplied licence**,
and it was never tested (Chapter 56 §56.2).

**Always-on VPN preventing a captive portal login.** **Expected**, and every product has a
mechanism for it that must be configured.

**"We are removing the VPN" and the legacy applications.** **They are the residual perimeter**
(Chapter 59 §59.4), and they should be planned for rather than discovered.

**A zero trust product proposed to replace a site-to-site tunnel.** **A category error.** They
solve different problems.

> **Network+ note.** Objective 4.4 and 1.8. Over-learn: **client-to-site VPNs connect remote
> users and site-to-site connects networks**; **a VPN concentrator terminates tunnels**;
> **split tunnelling reduces load and reduces inspection**; **always-on VPN connects
> automatically**; and **remote access should use MFA.** The split-tunnel trade and the MFA
> requirement are both examined, and the second is the one that matters most in practice.
