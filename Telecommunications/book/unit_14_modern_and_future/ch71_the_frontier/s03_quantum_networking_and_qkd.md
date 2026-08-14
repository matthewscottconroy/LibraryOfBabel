# 71.3 Quantum Networking and QKD

The area where the gap between the claims and the engineering is widest, and where the
physics is genuinely remarkable and the practical case is genuinely weak.

Both halves need stating, because the field's discourse tends to supply one or the other.

## What QKD actually does

Quantum Key Distribution establishes a shared secret between two parties, with security
guaranteed by physics rather than by computational difficulty.

The mechanism, in outline — **BB84, 1984:**

```
   Alice sends single photons, each encoded in one of two randomly chosen bases
   Bob measures each in a randomly chosen basis
   They publicly compare which bases they used — not the results
   Where the bases matched, the results agree: those bits become the key
   
   An eavesdropper must measure, and measuring in the wrong basis
   disturbs the state — which raises the error rate detectably
```

> **The security property is that eavesdropping is detectable**, because **measurement disturbs
> the quantum state.** **Which is not a computational assumption** — it is the no-cloning
> theorem, **and it does not weaken as computers improve.**

And that is a genuinely different kind of guarantee from everything in Chapter 58, where
security rests on a problem being hard to solve.

## What it does not do

**Four things, and each is routinely elided.**

**It distributes a key, and nothing else.** The data is still encrypted with AES (Chapter 58
§58.1). QKD replaces the key exchange (Chapter 58 §58.2) **and nothing more** — so a
system using it has classical cryptography for everything except the key agreement.

**It does not authenticate.** This is the significant one.

> QKD gives you a shared secret with somebody, and does not tell you with whom — **which is
> exactly Diffie–Hellman's limitation** (Chapter 58 §58.2). A man in the middle can run QKD
> with each party separately, and the physics does not prevent it.

So QKD requires an authenticated classical channel, which requires a pre-shared secret or a
public key infrastructure — and if you have a pre-shared secret you could have used it, and
if you have a PKI its security is classical. The quantum guarantee sits on a classical
foundation.

It does not protect stored data, and it does not survive a compromised endpoint — an
attacker on either machine reads the key regardless of how it arrived.

And it does not remove the need for post-quantum cryptography (Chapter 58 §58.4), because
QKD cannot be deployed everywhere and signatures are not a key exchange problem.

## The engineering constraints

**Three, and they are severe.**

### Distance

**Single photons cannot be amplified.** An EDFA (Chapter 50 §50.3) amplifies by stimulated
emission, which is a measurement, which destroys the quantum state.

**So the loss is the limit:**

| Distance | **Loss at 0.2 dB/km** | **Transmittance** |
|---|---|---|
| 50 km | 10 dB | **10%** |
| **100 km** | **20 dB** | **1%** |
| **200 km** | **40 dB** | **0.01%** |
| 400 km | 80 dB | **$10^{-8}$** |

And the PLOB bound — the proven upper limit on repeaterless quantum key distribution —
gives about 0.014 bits per channel use at 100 km and 0.00014 at 200 km.

> Practical QKD systems achieve useful key rates to perhaps 100 km and marginal ones beyond.
> Records exceeding 400 km exist under laboratory conditions with ultra-low-loss fibre and
> superconducting detectors, and they are not products.

### Trusted nodes

The answer to distance, and it undermines the security argument.

```
   Alice ──QKD── Node 1 ──QKD── Node 2 ──QKD── Bob
                    │              │
              decrypts and    decrypts and
              re-encrypts     re-encrypts
```

> **Each intermediate node holds the key in the clear.** Which means the end-to-end security is
> the security of the intermediate nodes — a classical assumption, and exactly the one QKD
> was meant to remove.

China's Beijing–Shanghai backbone (2,000 km) works this way, with **32 trusted nodes**, and
its security is that of 32 physically-secured facilities rather than of quantum mechanics.

A true quantum repeater — which would extend entanglement without measuring it, using
entanglement swapping and quantum memory — is a research problem. Quantum memories with
sufficient coherence time and efficiency do not exist at the required performance, and the
timescale is decades rather than years.

### Dedicated fibre, and cost

QKD generally requires a dark fibre — the single photons cannot share a fibre with
classical channels at normal power, because Raman scattering from the classical channels
swamps them. Co-existence is demonstrated and constrains the classical power substantially.

And the equipment cost is high, the key rate is low, and the deployment produces a
point-to-point link between two specific locations — which is a very expensive way to solve
a problem AES-256 and a pre-shared key also solve.

## The honest comparison

| | **QKD** | **Post-quantum cryptography** (Chapter 58 §58.4) |
|---|---|---|
| **Security basis** | **physics** | **a mathematical problem believed hard** |
| **Distance** | **~100 km, or trusted nodes** | **unlimited** |
| **Infrastructure** | **dedicated fibre and hardware** | **software** |
| **Cost** | **very high** | **negligible** |
| **Authentication** | **requires a classical mechanism** | **provides it** |
| **Deployable now** | **point to point, at cost** | **yes, and it is being deployed** |
| **Standardised** | partially | **NIST, 2024** |

> **The security agencies' positions are worth knowing because they are unusually direct.**
> The NSA, the UK's NCSC and several European agencies have all published guidance
> recommending post-quantum cryptography over QKD for national security systems, citing the
> authentication requirement, the trusted-node problem, the cost and the absence of a
> compelling advantage over well-implemented classical cryptography.

Which is a strong signal, because these are organisations with the budget and the motivation
to deploy QKD if it were the better answer.

## Where it is nonetheless deployed

Being fair, because the case is not zero.

| | |
|---|---|
| **China's backbone and satellite work** | **a national programme with strategic rather than commercial objectives** |
| **Some financial and government point-to-point links** | **short distances, high-value, and partly demonstrative** |
| **The EU's EuroQCI programme** | **infrastructure investment with a sovereignty rationale** |
| **Research networks** | genuinely |

And the strongest technical argument for it is one this book has made elsewhere (Chapter 58
§58.2): harvest now, decrypt later. A key established by QKD cannot be recovered
retrospectively even by a future quantum computer, and for data that must remain secret for
fifty years the argument has force.

Against which: post-quantum cryptography addresses the same threat, is deployable everywhere,
and costs nothing.

## The quantum internet, honestly

A genuinely different proposition from QKD, and it is further away.

The vision: distributing entanglement between distant quantum processors, which would
enable distributed quantum computation, blind computation, and precision sensing beyond
classical limits.

> This is not a faster Internet and it is not a network for ordinary data. Entanglement
> distribution is a resource for quantum applications, and there is nothing a classical user
> would notice.

And the prerequisites are the same as the repeater's: quantum memory, error correction and
entanglement swapping at useful rates and fidelities — all of which are active research with
demonstrations over tens of kilometres and no path to deployment that anyone can currently
date.

Note also that entanglement does not transmit information. No-signalling is a theorem,
and the recurring popular claim that entanglement permits faster-than-light communication is
simply wrong — the classical channel is still required, and it is still limited by
Chapter 3 §3.1's speed of light.

## How to assess a claim here

**Four questions.**

| | |
|---|---|
| **1** | **Is it QKD or a quantum computer?** — **entirely different technologies, frequently conflated** |
| **2** | **What is the distance, and are there trusted nodes?** |
| **3** | **How is the classical channel authenticated?** |
| **4** | **What does this do that AES-256 with good key management does not?** |

The fourth is the one that settles most conversations.

## What breaks here

A QKD deployment presented as removing the need for classical cryptography. It distributes
a key; AES does the rest.

A QKD link with no authenticated classical channel. A man in the middle runs it with each
party, and the physics does not help.

A "quantum-secure network" that is a chain of trusted nodes. The security is the nodes'
physical security.

A key rate that falls to nothing beyond 100 km. The loss, and single photons cannot be
amplified. Expected.

QKD sharing a fibre with classical channels and failing. **Raman scattering.** Dark fibre, or
substantially reduced classical power.

"Quantum entanglement enables instant communication." **No-signalling.** It does not.

A quantum programme funded instead of a post-quantum migration. The agencies recommend the
opposite, and the second addresses the same threat everywhere for nothing.

> **Network+ note.** Beyond the syllabus entirely. The transferable content is Chapter 58's:
> a key exchange establishes a shared secret and does not establish identity, and security
> is a property of a system rather than of a mechanism. QKD is the clearest available
> illustration of a mechanism with an extraordinary property that does not, by itself, produce a
> secure system.
