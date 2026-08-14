# 70.2 APIs, NETCONF, RESTCONF and YANG

§70.1's argument was that the CLI is a human interface. This section is what replaces it,
and the model matters more than the protocol.

## The three pieces

A management interface needs three things, and they are frequently confused.

| | Is | Analogy |
|---|---|---|
| **A data model** | **what can be configured, with types and constraints** | **the schema** |
| **A protocol** | **how to send and retrieve it** | **the transport** |
| **An encoding** | **how it is represented on the wire** | **the format** |

| | Model | Protocol | Encoding |
|---|---|---|---|
| **NETCONF** | **YANG** | **NETCONF over SSH** | **XML** |
| **RESTCONF** | **YANG** | **HTTP** | **JSON or XML** |
| **gNMI** | **YANG** | **gRPC over HTTP/2** | **protobuf** |
| **A vendor REST API** | **whatever the vendor chose** | HTTP | JSON |
| **SNMP** | **MIB** | SNMP | BER |

> **YANG is the important one**, because it is what makes the interface a contract rather than
> a convention. **Three protocols share one model**, which means the same understanding
> transfers between them.

## YANG

A language for describing configuration and state data, and its contribution is that it is
typed, constrained and hierarchical.

```
   container interfaces {
     list interface {
       key "name";
       leaf name        { type string; }
       leaf enabled     { type boolean; default true; }
       leaf mtu         { type uint16 { range "68..9216"; } }
       leaf description { type string { length "0..255"; } }
       container state {
         config false;                    // read-only
         leaf oper-status { type enumeration { enum up; enum down; } }
         leaf in-octets   { type uint64; }
       }
     }
   }
```

Five things in that fragment do work the CLI cannot:

| | |
|---|---|
| **Types** | **`mtu` is a `uint16`, not a string that happens to contain digits** |
| **Constraints** | **`range "68..9216"` — an invalid value is rejected before it is sent** |
| **Defaults** | **stated, so "not configured" has a defined meaning** |
| **`config false`** | **the distinction between what you set and what you observe**, explicitly |
| **Structure** | **a list with a key, which means "the interface named X" is addressable** |

> **The `config false` distinction is the one the CLI never made.** `show interface` mixes
> configuration and state in one text blob, and separating them is what allows a tool to ask
> "what did I configure?" and "what is actually happening?" as different questions.

**And the models come from three places:**

| Source | Notes |
|---|---|
| **IETF** | **standard models** — `ietf-interfaces`, `ietf-routing` — **and coverage is thin** |
| **OpenConfig** | **operator-defined** (Chapter 54's entry) — **broader, and the practical choice** |
| **Vendor** | **complete, and vendor-specific** — which defeats the point |

**The honest position:** the standard models cover a fraction of what a device does, the vendor
models cover everything and do not transfer, and OpenConfig sits between and is the best
available compromise. Most real automation uses a mixture, and the mixture is a maintenance
burden.

## NETCONF

The 2006 protocol, and its two contributions are the ones the CLI lacks.

### Datastores

```
   ┌──────────────┐   ┌──────────────┐   ┌──────────────┐
   │  candidate   │──▶│   running    │──▶│   startup    │
   │ (edit here)  │   │  (in effect) │   │ (after boot) │
   └──────────────┘   └──────────────┘   └──────────────┘
        edit-config       commit          copy-config
```

> A candidate datastore means a change is assembled completely and then committed
> atomically — **which is exactly what the CLI cannot do**, because **each line takes effect
> as it is typed** (Chapter 55 §55.2's point-of-no-return problem).

**And `commit confirmed`** (Chapter 55 §55.2) is a NETCONF operation — the device reverts
unless confirmed within a timeout — which is the single highest-value habit in this book,
available as a protocol primitive.

### Locking and transactions

`lock` prevents two systems configuring simultaneously, and `validate` checks the candidate
before commit.

> Which are ordinary database properties and were absent from network configuration for forty
> years. Two engineers configuring the same device at the same time is a real failure mode
> that NETCONF simply removes.

**Its operations are few:** `get`, `get-config`, `edit-config`, `copy-config`, `delete-config`,
`lock`, `unlock`, `close-session`, `kill-session`, `commit`, `validate` — and the whole
protocol is comprehensible in an afternoon.

Its cost is XML, which is verbose and awkward to generate by hand — and which nobody
generates by hand, because libraries do it.

## RESTCONF

NETCONF's model over HTTP, and its argument is accessibility.

```
   GET    /restconf/data/ietf-interfaces:interfaces/interface=eth0
   PUT    /restconf/data/ietf-interfaces:interfaces/interface=eth0
   PATCH  /restconf/data/ietf-interfaces:interfaces/interface=eth0
   DELETE /restconf/data/ietf-interfaces:interfaces/interface=eth0
```

| | **NETCONF** | **RESTCONF** |
|---|---|---|
| Transport | **SSH** | **HTTPS** |
| Encoding | **XML** | **JSON or XML** |
| **Transactions** | **yes — candidate, lock, commit** | **no** |
| **Tooling** | specialist libraries | **`curl`, any HTTP client** |
| **Learning curve** | steeper | **shallow** |

> **RESTCONF trades transactions for accessibility**, and the loss of the candidate datastore
> is the significant half. **A RESTCONF change takes effect immediately, per resource**, which
> reintroduces exactly the partial-application problem NETCONF solved.

Which makes it excellent for reading and for simple changes, and the wrong choice for a
multi-part configuration change that must be atomic.

## gNMI

The modern one, and it is the direction of travel (Chapter 54 §54.4).

| | |
|---|---|
| **Transport** | **gRPC over HTTP/2**, with TLS mandatory |
| **Encoding** | **protobuf** — compact and typed |
| **Model** | **YANG, usually OpenConfig** |
| **Operations** | **`Get`, `Set`, `Subscribe`, `Capabilities`** |

**`Subscribe` is the one that matters:**

> A single subscription produces a continuous stream of updates — on change, or at an
> interval — for a set of paths. Which is Chapter 54 §54.4's streaming telemetry, and it is
> the same protocol that does configuration.

One interface, one model, one credential, for both configuration and telemetry — which is a
genuine simplification over SNMP for monitoring, NETCONF for configuration and a CLI for
everything else.

**Its limitation is coverage.** Support is good on modern service provider and data centre
platforms and thin elsewhere, and the models vary between vendors despite OpenConfig —
which is Chapter 54's honest observation, unchanged.

## Choosing

| Requirement | Use |
|---|---|
| **Atomic multi-part configuration change** | **NETCONF** |
| **Reading state from a script** | **RESTCONF, or gNMI** |
| **Streaming telemetry** | **gNMI** |
| **Both configuration and telemetry, one interface** | **gNMI** |
| **A device that supports none of them** | **the CLI, via Netmiko** — and validate the parse |
| **A cloud network** | **the provider's API** — Chapter 69 |
| **A controller-managed estate** | **the controller's API** — and it is the only one |

**And the practical reality:** an estate of any age has devices in every row of that table,
which is why automation frameworks abstract over the transport (§70.3) rather than
committing to one.

## The security consequence

Which is worth its own note because it is under-considered.

> An automation interface with configuration privileges on every device is the most valuable
> credential in the organisation (Chapter 57 §57.3).

And it is frequently protected less carefully than the devices:

| | |
|---|---|
| **The credential** | **in a CI system, a vault, or a file** (Chapter 55 §55.4) |
| **The transport** | **NETCONF and gNMI mandate TLS or SSH; a vendor REST API may not** |
| **The network path** | **it should be the management VRF** (Chapter 60 §60.4) |
| **Authorisation** | **can the automation account do only what it needs?** — usually not |
| **Accounting** | **TACACS+ per-command logging** (Chapter 59 §59.2) **applies here too** |

**And the specific control worth insisting on:** the automation account should be
authorised for the changes the automation makes and not for everything — which requires
per-command authorisation and is almost never configured, because it is easier to grant
level 15.

## What breaks here

A standard YANG model that does not cover what you need. **Coverage is thin.** OpenConfig,
or the vendor model, or the CLI.

The same OpenConfig path behaving differently on two vendors. **Model interpretation
varies**, and it is the reason "vendor-neutral automation" is aspirational.

**A RESTCONF change applied partially.** **No transactions.** NETCONF for anything multi-part.

A gNMI subscription returning nothing for a valid path. The platform's model coverage.
Check `Capabilities`.

An automation credential with level 15 on every device, in a CI system. The highest-value
credential in the estate.

A `commit confirmed` not used for a remote change. Available as a protocol primitive, and
Chapter 55 §55.2 says why.

**Two systems configuring one device simultaneously.** NETCONF's `lock` removes this, and
nothing else does.

> **Network+ note.** Objective 1.8 and 3.2. Over-learn: **APIs allow programmatic configuration
> and monitoring**; **REST APIs use HTTP methods and JSON**; **NETCONF and RESTCONF use YANG data
> models**; and **automation reduces human error.** YANG's typing and the transaction model are
> beyond the syllabus and are the reason the interface is better than a scripted CLI.
