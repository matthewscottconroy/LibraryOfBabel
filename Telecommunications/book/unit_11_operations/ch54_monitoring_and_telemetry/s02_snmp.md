# 54.2 SNMP

**Thirty-five years old, insecure by default, architecturally awkward, and still doing most of
the world's device monitoring.** It is worth understanding properly, including the parts that
are wrong.

## The model

**A managed device runs an agent. A manager asks it for values. The values live in a
hierarchical namespace.**

```
   ┌─────────┐  GET / GETNEXT / GETBULK (UDP 161)   ┌────────┐
   │ Manager │ ────────────────────────────────────▶│ Agent  │
   │  (NMS)  │ ◀──────────────── RESPONSE ──────────│(device)│
   │         │                                       │        │
   │         │ ◀── TRAP / INFORM (UDP 162) ──────────│        │
   └─────────┘        the device volunteers          └────────┘
```

| Operation | Direction | Use |
|---|---|---|
| **GET** | manager → agent | fetch one object |
| **GETNEXT** | manager → agent | **walk the tree** |
| **GETBULK** (v2c+) | manager → agent | **fetch many at once — far more efficient** |
| **SET** | manager → agent | **change a value** — and the reason read-write is dangerous |
| **TRAP** | agent → manager | **unsolicited; fire and forget** |
| **INFORM** (v2c+) | agent → manager | **a trap that is acknowledged** |

**The TRAP/INFORM distinction matters.** **A trap is UDP with no acknowledgement**, so **a trap
lost in transit is simply lost** — and the loss is most likely precisely when the network is in
trouble. **INFORM retransmits until acknowledged.** **Use INFORM for anything you care about.**

## MIBs and OIDs

**A MIB is a schema. An OID is an address in it.**

```
   1.3.6.1.2.1.2.2.1.10.5
   │ │ │ │ │ │ │ │ │ │  └── interface index 5
   │ │ │ │ │ │ │ │ │  └───── ifInOctets
   │ │ │ │ │ │ │ │  └──────── ifEntry
   │ │ │ │ │ │ │  └─────────── ifTable
   │ │ │ │ │ │  └────────────── interfaces
   │ │ │ │ │  └───────────────── mib-2
   │ │ │ │  └──────────────────── mgmt
   │ │ │  └───────────────────────internet
   │ │  └────────────────────────── dod
   │  └───────────────────────────── org
   └──────────────────────────────── iso
```

**The ones you will actually meet:**

| OID | Name | |
|---|---|---|
| **1.3.6.1.2.1.1** | **system** | sysDescr, sysUpTime, sysName, sysLocation |
| **1.3.6.1.2.1.2** | **interfaces** | **the table everything monitors** |
| **1.3.6.1.2.1.31** | **ifXTable** | **the 64-bit counters — use these** |
| **1.3.6.1.4.1.\<n>** | **enterprises** | **vendor-specific; `n` is the vendor's number** |

**Enterprise numbers are IANA registrations** (Chapter 48 §48.3) — **9 is Cisco, 2636 Juniper,
2011 Huawei, 8072 Net-SNMP** — and **a MIB file is what translates a vendor OID into a name.**

**Without the MIB you get numbers; with it you get names.** **Loading vendor MIBs into the
monitoring system is a tedious task that is worth doing once**, because otherwise every
vendor-specific alert reads as a numeric string that nobody can interpret at 03:00.

## The security problem, stated plainly

| Version | Authentication | Encryption |
|---|---|---|
| **v1** | **community string, cleartext** | **none** |
| **v2c** | **community string, cleartext** | **none** |
| **v3** | **user, with HMAC-SHA** | **AES** |

> **A community string is a password sent in cleartext in every request.** Anyone who can
> capture a packet can read your entire device inventory — **interface names, addresses,
> topology, serial numbers, software versions** — **and if the string is read-write, reconfigure
> your equipment.**

**The default strings are `public` (read) and `private` (read-write)**, and **they remain in use
on production equipment with dispiriting frequency.** **Internet-facing devices with `public`
are found by scanning constantly**, and the information they disclose is exactly what an
attacker wants first (Chapter 62).

**SNMPv3's security model:**

| Level | Meaning |
|---|---|
| `noAuthNoPriv` | **username only — no better than v2c** |
| `authNoPriv` | **authenticated, not encrypted** |
| **`authPriv`** | **authenticated and encrypted — use this** |

**The reason v3 is often not deployed is that it is more work to configure** — per-user
credentials, engine IDs, and a configuration that differs more between vendors than v2c's does.
**That is not a good reason**, and the mitigation if v3 is genuinely impractical is
**restricting SNMP to a management VRF or ACL from the monitoring system's address only**, plus
**read-only communities, always**.

> **There is essentially no legitimate use for SNMP read-write in 2026.** Configuration is done
> by SSH, by NETCONF, or by an automation tool (Chapter 70). **Disable SET.**

## Counters wrap

**The artefact that produces impossible readings, and it is worth understanding because the
symptom is confusing.**

**SNMP counters are unsigned integers that increment and roll over.** **The monitoring system
computes a rate from the difference between two polls** — **and if the counter wrapped more than
once between polls, the difference is meaningless.**

| Counter | Interface rate | **Wraps in** |
|---|---|---|
| **32-bit** (`ifInOctets`) | 100 Mb/s | 344 s |
| **32-bit** | **1 Gb/s** | **34 s** |
| **32-bit** | **10 Gb/s** | **3.4 s** |
| **64-bit** (`ifHCInOctets`) | 10 Gb/s | **468 years** |
| 64-bit | 100 Gb/s | **47 years** |

> **A 32-bit octet counter on a 10 Gb/s interface wraps in about three seconds at line rate.**
> Polling every five minutes therefore produces **a number with no relationship to reality.**

**The symptom is a graph with impossible spikes** — negative rates, or rates far above the
interface's capacity — **appearing intermittently and only on fast links.**

**The fix is to use `ifXTable`'s 64-bit high-capacity counters**, and **a monitoring system that
silently falls back to the 32-bit ones on a device that supports both is a monitoring system
producing nonsense.** **Check which your system is using.**

**And note the second-order problem:** **64-bit counters are only available over SNMPv2c and
above.** **A device polled with v1 cannot provide them**, which is a practical reason to
abandon v1 entirely.

## Polling has a resolution floor

**The architectural limitation, and the reason streaming telemetry exists.**

$$\text{polling load} = \frac{\text{objects}}{\text{interval}}$$

| Scale | Interval | Requests/s |
|---|---|---|
| 10,000 interfaces | **5 min** | **33** |
| 10,000 interfaces | **1 min** | 167 |
| **10,000 interfaces** | **10 s** | **1,000** |
| 10,000 interfaces | 1 s | **10,000 — not feasible** |

**And the load falls on the device's control-plane CPU**, which is frequently modest.
**Aggressive SNMP polling has caused outages** — a device spending its CPU answering the
monitoring system rather than processing routing updates. **This is a real failure mode and it
is embarrassing when it happens.**

**Mitigations:** **GETBULK rather than repeated GETNEXT**; **poll only the objects you use**
(the default templates fetch a great deal nobody looks at); **stagger polls** rather than
querying every device on the minute; **and accept a longer interval for things that change
slowly.**

> **Polling every five minutes cannot see a thirty-second event, and polling every ten seconds
> across a large estate is a substantial load.** **That tension is what §54.4's streaming
> telemetry was invented to resolve**, and it is the honest reason SNMP is being replaced rather
> than any elegance argument.

## What SNMP is still good at

**A fair assessment, because the criticism above is one-sided.**

**Universality.** **Everything supports it** — switches, routers, firewalls, UPSs, printers,
environmental sensors, PDUs. **No other management protocol comes close**, and a monitoring
system that speaks SNMP can monitor equipment its authors never saw.

**Simplicity.** **A GET is one packet.** No session, no state, no schema negotiation.

**Maturity.** **Every tool supports it, every MIB is documented, and the failure modes are
known.**

**Which is why it will still be in service in twenty years**, alongside whatever replaces it —
**exactly as Chapter 50 §50.1's T1 circuits are.**

## What breaks here

**Impossible traffic spikes on a fast link.** **32-bit counter wrap.** Use `ifXTable`.

**A device stops responding to polls under load.** **Control-plane CPU exhausted**, possibly by
the polling itself. Reduce the object set and the frequency.

**Interface indexes changing after a reboot.** **`ifIndex` is not guaranteed persistent.** Graphs
attach to the wrong interface and history is lost. **Enable index persistence** where the
platform supports it, or key on `ifName` instead.

**A trap that was never received.** **Traps are UDP and unacknowledged.** Use INFORM for
anything important, **and never rely on traps alone for detecting a device being down** —
a device that has lost power cannot send a trap saying so. **Polling detects absence; traps
cannot.**

**Alerts arriving as numeric OIDs.** **The vendor MIB is not loaded.** Tedious, one-off,
worth doing.

**SNMP working from one management station and not another.** **A community or v3 credential
restricted by ACL**, which is correct behaviour and looks like a fault.

**Community strings found in a configuration backup in version control.** **They are
cleartext.** Chapter 55 §55.4 — and this is a real and common disclosure.

> **Network+ note.** Objective 3.1 covers SNMP directly. Over-learn: **SNMP uses UDP 161 for
> polling and 162 for traps**; **a MIB defines the objects and an OID identifies one**; **v1 and
> v2c use community strings in cleartext, v3 adds authentication and encryption**; **a trap is
> device-initiated and a poll is manager-initiated**; and **an SNMP walk retrieves a subtree.**
> The port numbers and the v3 security point are examined regularly.
