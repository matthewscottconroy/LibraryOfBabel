# Chapter 68 — The People

Martin Casado (b. 1976), Nick McKeown (b. 1963) and Scott Shenker (b. 1955). Stanford,
2005–2008 — Ethane, then OpenFlow.

Casado's route into the problem is worth knowing because it shaped the argument. He worked
on network security for a US national laboratory and found that expressing a security policy
across a network required configuring dozens of devices, each partially, with no way to verify
the result.

> **Ethane (2007) was a security architecture, not a networking one.** Its proposal was that a
> central controller should decide, per flow, whether communication is permitted — and that the
> switches should do nothing but ask and enforce. **OpenFlow is Ethane's switch interface,
> generalised.**

Which explains OpenFlow's shape — the fine-grained per-flow matching, the reactive
controller consultation, the emphasis on policy — and it explains why the model fitted
security better than it fitted forwarding at scale.

McKeown's contribution was the standardisation and the campus argument:

> **"OpenFlow: Enabling Innovation in Campus Networks" (2008)** — **the title is the pitch.**
> Researchers could not experiment with new protocols because the switches were closed, and
> OpenFlow was proposed as a modest interface that vendors could add without exposing anything
> proprietary.

It was a deliberately small ask, and it became a movement, which was not the intention.

Shenker supplied the architectural framing — that networking had never had an abstraction
for the control plane the way operating systems had for memory or processes — and his talks
on "the future of networking and the past of protocols" are the clearest statement of the case.

> Shenker's argument is that networking's problem is not a shortage of mechanisms but an
> absence of abstractions, and that the field solved every problem with a new protocol
> because it had no way to build one on top of another. Which is a fair criticism, and this
> book's Chapter 21 makes the same point about layering being a solution people memorise rather
> than apply.

All three founded Nicira (Chapter 67's entry), and Casado has since been explicit that
network virtualisation succeeded where OpenFlow did not because it changed the abstraction
without requiring new hardware.

Nick Feamster, Jennifer Rexford and the earlier work.

The idea predates OpenFlow by several years, and the acknowledgement is usually cursory.

| | |
|---|---|
| **RCP — Routing Control Platform** (Feamster, Rexford et al., 2004) | **computing BGP routes centrally for an entire AS** |
| **4D** (Greenberg, Hjalmtysson, Maltz et al., 2005) | **decision, dissemination, discovery, data — a four-plane architecture** |
| **SANE and Ethane** (2006–2007) | the security lineage |
| **ForCES** (IETF, from 2000) | **a control/forwarding separation protocol that predates OpenFlow entirely** |

ForCES deserves a sentence because its fate is instructive:

> The IETF standardised control-plane/forwarding-plane separation years before OpenFlow, and
> essentially nobody implemented it. **The difference was not technical** — it was that
> OpenFlow arrived with a research community, a demonstration, a conference circuit and a
> narrative, and ForCES arrived as a specification.

Rexford's later work is the more useful for a practitioner: on verification, on the
correctness of routing configurations, and on why BGP policy interactions produce outcomes
nobody intended. Chapter 32's route leaks and Chapter 68 §68.4's verification argument both
draw on it.

**Guru Parulkar and the Open Networking Foundation.**

The ONF was founded in 2011 with Google, Facebook, Microsoft, Verizon, Deutsche Telekom and
Yahoo as board members — buyers rather than vendors, deliberately.

> The intent was to use purchasing power to force the interface open, which is the same
> strategy as Chapter 54's OpenConfig and Chapter 67's Open Compute Project. **It worked
> partially**: the specification was produced and adopted, and the implementations were
> partial, because a buyer consortium can compel a data sheet entry and cannot compel a
> good implementation.

Nate Foster, Jennifer Rexford, Nick McKeown and the P4 authors.

P4 (2014) is explicitly a response to OpenFlow's failure, and the paper says so:

> "OpenFlow's rigid specification of a fixed set of header fields and a fixed pipeline was a
> mistake." The proposal is to describe the pipeline rather than to assume it.

Foster's background is in programming languages rather than in networking, which is visible
in P4's design — a typed language with a compiler, a target abstraction and a formal
semantics — and in the verification work that followed.

> The involvement of programming-language researchers in networking is one of the more
> productive interdisciplinary movements of the last fifteen years, and it produced P4,
> NetKAT, the verification tools of §68.4, and a great deal of the formal work on routing
> correctness.

Amin Vahdat, Sushant Jain and the B4 team at Google.

**B4 (2013)** is **the honest success**, and the paper's value is its candour.

**What it reports:** a centralised traffic engineering system driving OpenFlow switches across
Google's inter-data-centre WAN, running links at near 100% utilisation where a conventionally
engineered WAN runs at 30–40%.

**And what it also reports:**

> **The failures.** A control plane outage that took hours to diagnose. The difficulty of
> making a centralised system fail safely. The need to build their own switches because nothing
> available was adequate. And the explicit statement that the approach depended on
> controlling the applications, the hardware and the failure model.

**Which is the paper's real contribution:** it demonstrated the architecture and delimited its
applicability, and the delimitation has been ignored far more often than the
demonstration.

## What this chapter's history shows

**Three observations.**

The idea had been proposed several times before it caught. ForCES, RCP, 4D — all correct,
all earlier, all ignored. OpenFlow succeeded in becoming a movement because it had a
community and a narrative, which is a statement about how technical ideas propagate rather
than about their merit.

The originators were clearer about the limits than the market was. Casado on why
virtualisation succeeded where OpenFlow did not; the B4 paper on its own preconditions; the P4
paper on OpenFlow's mistake. The overclaiming happened downstream of the research, and the
research is worth reading for that reason.

**And the buyers' consortium strategy partly worked.** ONF, OpenConfig, OCP and SONiC are all
the same move — large purchasers forcing openness — and the pattern is that it succeeds at
producing specifications and struggles at producing good implementations, because the
supplier's incentive is unchanged.

> The recurring lesson of this chapter is that architecture and economics are separate
> questions, and that a sound architecture with adverse economics loses — which Chapter 57
> §57.4 established for security and which is true more generally than anyone in this field
> finds comfortable.
