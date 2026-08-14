# 59.4 Zero Trust

A term that has been applied to so many products that it has nearly lost meaning. The
underlying argument is sound, older than the name, and worth separating from the marketing.

## The claim

> **Being on the network confers no privilege.**

**That is the whole of it.** Every other element — identity, device posture, per-application
access, continuous verification — follows from taking that sentence seriously.

**The model it replaces:**

```
   Perimeter model:                    Zero trust:

   ┌───────────────────────────┐       Every request:
   │  ▓▓▓▓ FIREWALL ▓▓▓▓▓▓▓▓▓▓ │         who is this? (identity)
   ├───────────────────────────┤         on what device? (posture)
   │                           │         to reach what? (resource)
   │   TRUSTED INSIDE          │         under what conditions? (context)
   │   flat, mutually          │         → decide, per request
   │   reachable, largely      │
   │   unmonitored             │       No inside. No outside.
   └───────────────────────────┘
```

And the perimeter model's assumption — that internal traffic is trustworthy — had been false
for years before anyone said so (Chapter 51's Kindervag). Once an attacker is inside, a hard
shell around a soft interior offers nothing — which is Chapter 57 §57.1's lateral movement
step, restated.

## The three inputs to a decision

| Input | Asks | Evidence |
|---|---|---|
| **Identity** | **who** | authenticated, ideally with a phishing-resistant factor (§59.1) |
| **Device** | **on what** | **managed? patched? encrypted? EDR running? compliant?** |
| **Context** | **under what circumstances** | location, time, behaviour, sensitivity of the resource |

Device posture is the input that distinguishes zero trust from good authentication.

> A correct credential on an unmanaged, unpatched personal machine is not the same as the same
> credential on a managed, current, encrypted corporate one — and a system that cannot tell
> the difference is doing authentication, not zero trust.

Which makes endpoint management a network dependency (Chapter 51 §51.4), and it is the
part organisations underestimate: the posture signal must be trustworthy, which means the
agent must be tamper-resistant, and an unmanaged device cannot be assessed at all — so BYOD
becomes a policy decision with teeth.

## Per-application, not per-network

The mechanism that makes it more than a slogan.

| | **VPN** | **Zero trust access** |
|---|---|---|
| Grants | **network access** | **application access** |
| After connecting, the user can reach | **everything routable** | **exactly what was granted** |
| Lateral movement | **possible** | **not implied** |
| **The resource's exposure** | **listening on a reachable network** | **frequently not listening at all** |

The last row is the strongest property and the least discussed.

> In a broker-mediated design, the application makes an outbound connection to the broker and
> has no inbound listener at all. There is nothing to scan, nothing to exploit
> pre-authentication, and no network path to the resource that does not pass through the
> policy decision point.

Which converts §57.1's opportunistic scanning from a threat into a non-event, and it is why
"software-defined perimeter" designs are genuinely different from a VPN with better
authentication.

## Continuous verification

The property that distinguishes it from a one-time gate.

Traditional authentication is a gate: pass it, and the session is trusted until it ends.
**Zero trust re-evaluates.**

| Trigger | Response |
|---|---|
| **Device falls out of compliance mid-session** | **access revoked** |
| **Impossible travel** — two logins, distant, minutes apart | **step-up or block** |
| **Anomalous behaviour** | **step-up authentication** |
| **Session age** | **re-evaluate** |
| **Resource sensitivity changes** | re-evaluate |

This is Saltzer and Schroeder's complete mediation (Chapter 57's reading): check every
access, every time, rather than checking once and issuing a capability.

**And its cost is real:** the policy decision point is in the path of everything, so its
availability and latency are now everyone's availability and latency — Chapter 56 §56.1's
series arithmetic, applied to a security control.

## The architecture, per NIST SP 800-207

```
                    ┌──────────────────────┐
                    │  Policy Decision     │  ← identity, posture, context,
                    │  Point (PDP)         │    threat intelligence
                    └──────────┬───────────┘
                               │ decision
   ┌────────┐          ┌───────┴────────┐          ┌──────────┐
   │ Subject│─────────▶│ Policy         │─────────▶│ Resource │
   │        │          │ Enforcement    │          │          │
   └────────┘          │ Point (PEP)    │          └──────────┘
                       └────────────────┘
```

**The separation is the point.** The PDP holds the policy and decides; the PEP sits in the
traffic path and enforces. Which is Chapter 68's control-plane/data-plane separation
argument, applied to access control — and it is why the same idea keeps appearing.

## What the network engineer actually contributes

Zero trust is frequently presented as an identity project, and the network's part is
substantial.

**Segmentation remains necessary.** Zero trust does not remove the need for it; it changes its
purpose. A flat network with per-application access control still permits an attacker who
compromises a device to scan and attack everything at the network layer, regardless of what
the application-layer policy says. The two are complementary (Chapter 60 §60.4).

802.1X provides the device identity (§59.2) that posture assessment depends on, and
dynamic VLAN assignment is zero trust's enforcement point at the wired edge.

**Encryption everywhere**, because "the internal network is trusted" was the assumption that
justified plaintext internal traffic (Chapter 62). Removing the assumption means removing
the plaintext.

**Monitoring changes shape.** Perimeter-focused monitoring assumed the interesting traffic
crossed a boundary. When there is no boundary, monitoring must be east–west — internal
traffic, lateral connections, and the things that used to be invisible.

And the network provides the thing identity cannot: containment when identity fails.
Every identity system will eventually authenticate an attacker, and what happens next is a
network property.

## Honest assessment

Three things that are true and are not in the marketing.

It is a direction, not a project with an end. No organisation is "zero trust". The
useful question is "which of our access decisions still rest on network location?", and
answering it produces a work list rather than a certification.

**Legacy applications do not participate.** A twenty-year-old system that authenticates by
source IP address, or has no authentication at all because it was only ever on the internal
network, cannot be brought into the model — it can only be wrapped in a broker or
segmented. Every real deployment has a residual perimeter around these, and pretending
otherwise produces plans that fail.

**It concentrates risk in the identity provider.** §59.1's point, sharpened: the PDP decides
everything, so compromising it compromises everything, and its availability is now
everyone's. The break-glass path, the PDP's own administrative access, and its availability
target all need designing before the architecture is adopted, not after.

> The strongest honest summary: zero trust replaces one trusted thing (the network) with
> another (the identity and posture system), and the second is a much better thing to trust —
> because it can be verified per request, revoked instantly, and logged completely. It is not
> the absence of trust; it is trust placed somewhere defensible.

## What breaks here

"Zero trust" deployed and the internal network still flat. Application-layer policy does
not prevent network-layer attack. Segmentation is still required.

**Posture assessment on an unmanaged device.** It cannot be assessed. This is a policy
decision, and it has to be made explicitly.

The PDP unavailable and nobody can reach anything. **Series availability.** Its target is
higher than anything it protects, and it needs a break-glass path.

Legacy applications excluded from the model and forgotten. They are the residual
perimeter, and they should be the most tightly segmented things you own.

**Continuous verification producing constant re-authentication prompts.** Users will find a
way around it — Chapter 57's psychological acceptability. Step up on signal, not on a
timer.

**Impossible-travel alerts firing on VPN users.** **Expected**, and it is why context signals
need tuning against your own population before they are enforced.

A zero trust product that is a VPN with MFA. **Ask three questions:** does it grant
application access or network access; does it assess device posture; and does it re-evaluate
during a session? A "no" to all three is a VPN.

> **Network+ note.** Objective 4.1 and 1.8 touch zero trust. Over-learn: zero trust assumes no
> implicit trust based on network location; it requires verification of identity and device
> for every request; **least privilege and microsegmentation are components**; and **a policy
> engine makes access decisions.** The "network location confers no trust" statement is the
> examinable core.
