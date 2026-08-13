# 20.3 Access Ports, Trunks and the Native VLAN

Two port types, one legacy accommodation, and the security consequence that follows
from it.

## Access ports

A port belonging to **exactly one VLAN**, carrying **untagged** frames.

```
interface GigabitEthernet0/5
 switchport mode access
 switchport access vlan 10
 spanning-tree portfast
 spanning-tree bpduguard enable
```

The behaviour:

| Direction | Action |
|---|---|
| **Ingress** | Frame arrives untagged; the switch associates it internally with VLAN 10 |
| **Egress** | Any tag is **removed** before transmission |

**The device attached knows nothing about VLANs.** A workstation, a printer, a
camera — none of them see a tag, none of them need to be configured, and none of them
can be. This is the whole design intent: **VLAN membership is a property of the
network, invisible to the endpoint.**

Which is why VLANs deployed so easily. Nothing on any of the thousands of existing
devices had to change.

Note the two spanning-tree lines. They are not optional (§19.3): PortFast removes the
30-second delay on an access port, and BPDU Guard is what makes PortFast safe.

## Trunk ports

A port carrying **multiple VLANs**, with frames **tagged** to identify which.

```
interface GigabitEthernet0/24
 switchport mode trunk
 switchport trunk allowed vlan 10,20,30
 switchport trunk native vlan 999
```

Used between switches, to routers, to firewalls, and to virtualisation hosts — any
device that must handle traffic for more than one VLAN over one physical link.

**`switchport trunk allowed vlan` is the most important line**, and the most commonly
neglected. By default a trunk carries **all** VLANs. Restricting it to those actually
needed:

- limits broadcast propagation
- limits the blast radius of a misconfiguration
- limits what an attacker who compromises a switch can reach
- and makes the configuration document the intent

Be careful with the syntax: `switchport trunk allowed vlan 40` **replaces** the list.
`switchport trunk allowed vlan add 40` extends it. Getting this wrong during a change
window removes every other VLAN from the trunk instantly, and it is one of the classic
self-inflicted outages.

## DTP, and why to turn it off

Cisco's **Dynamic Trunking Protocol** negotiates whether a link becomes a trunk.

| Mode | Behaviour |
|---|---|
| `dynamic auto` | becomes a trunk if asked |
| `dynamic desirable` | actively asks |
| `trunk` | is a trunk, and by default still sends DTP |
| `access` | is an access port |
| `nonegotiate` | send no DTP frames |

**Turn it off.** Explicitly, on every port:

```
switchport mode access
switchport nonegotiate
```

The reason is an attack. A port left at `dynamic auto` — a common default — will
**become a trunk if the attached device asks it to**. An attacker plugging into a wall
socket sends a DTP frame, the port becomes a trunk, and the attacker now receives
traffic for **every VLAN** the trunk permits. This is **switch spoofing**, and it turns
a wall socket into a monitoring position for the entire network.

The configuration cost of prevention is one line. **Configure trunks explicitly and
access ports explicitly; negotiate nothing.**

## The native VLAN

Here is the legacy accommodation, and the security problem it creates.

**A trunk's native VLAN is the one whose frames are sent untagged.**

Why does such a thing exist? Because in 1998 a trunk might connect to a device that
did not understand 802.1Q — an older switch, a hub, a management interface. Tagged
frames would be discarded by such a device (§20.2). The native VLAN provides a
compatibility path: one VLAN's traffic passes untagged and so remains comprehensible
to anything.

**Default native VLAN: 1.** Which is also the default access VLAN for every
unconfigured port on the switch.

### The consequence

Combine those two defaults and the result is uncomfortable:

> **On a default configuration, every unconfigured port is in VLAN 1, and VLAN 1
> traffic crosses every trunk untagged.**

An attacker on any unconfigured port is on the VLAN that traverses the entire
infrastructure without a tag.

### VLAN hopping by double tagging

The specific attack, and it is worth working through because it explains the
mitigation exactly.

**Setup:** the attacker is on an access port in VLAN 1. The trunk's native VLAN is also
1. The target is in VLAN 20.

**The attack:** the attacker crafts a frame with **two** tags:

```
   ┌──────┬──────┬══════════╦══════════┬─────────┐
   │ Dest │ Src  ║ VLAN 1   ║ VLAN 20  │ payload │
   └──────┴──────┴══════════╩══════════┴─────────┘
             outer (native)   inner (target)
```

**Step 1.** The frame reaches the first switch on an access port. The switch is about
to send it out the trunk. Because the outer tag is VLAN 1 — **the native VLAN** — the
switch **strips it**, as the native VLAN rule requires.

**Step 2.** The frame now leaves the trunk carrying only the **inner tag: VLAN 20**.

**Step 3.** The second switch receives a frame tagged VLAN 20 on a trunk. It forwards
it into VLAN 20.

The attacker has injected a frame into a VLAN they have no access to. Note two things:

- **It is unidirectional.** Replies come back through normal forwarding and will not
  reach the attacker. That limits it — but it is sufficient for injection attacks,
  for triggering actions, and for anything where the reply is not needed.
- **Every switch behaved exactly as specified.** Nothing is broken. The attack is a
  consequence of the native VLAN's defined behaviour, which is what makes it
  interesting.

### The mitigation

Three lines, and they compose:

**1. Change the native VLAN to an unused one.**

```
switchport trunk native vlan 999
```

VLAN 999 exists, carries no traffic, and has no ports in it. Double tagging with an
outer tag of 999 requires the attacker to be *in* 999, and nobody is.

**2. Do not use VLAN 1 for anything.**

Not for data, not for management, not for the native VLAN. Move every unused port into
a "parking" VLAN that is shut down:

```
interface range GigabitEthernet0/1 - 48
 switchport access vlan 999
 shutdown
```

**An unused port should be administratively down and in a dead VLAN.** Both, not
either.

**3. Tag the native VLAN.**

```
vlan dot1q tag native
```

Every frame on the trunk is tagged, including the native VLAN's. The double-tagging
attack has nothing to exploit because there is no untagged case. Supported on modern
equipment; the cleanest answer where available.

The native VLAN must **match on both ends of a trunk**. A mismatch means traffic from
one VLAN silently arrives in another — connectivity between two VLANs that should be
isolated, with no error and no obvious cause. CDP and LLDP detect and log it, which is
one of the better arguments for leaving them enabled internally.

## VLAN propagation: VTP, and a warning

VTP (VLAN Trunking Protocol) propagates VLAN definitions between Cisco switches, so a
VLAN created on one appears on all.

It is convenient and it has a famous failure mode. Every VTP update carries a
**revision number**, and switches accept any update with a **higher** number.

**A switch returned from a lab, with a high revision number and an empty VLAN
database, connected to a production trunk, will delete every VLAN in the domain.**

This has happened to a great many organisations. The device does not have to be
malicious or even misconfigured — it merely has to have been configured a lot,
somewhere else.

The safe practice:

- **VTP transparent mode** — participate in forwarding VTP messages but do not act on
  them, and configure VLANs locally
- **VTP version 3**, which requires an explicit primary server and cannot be
  overridden by revision number alone
- Or **off entirely**, which is what most modern designs do

Configuring VLANs on each switch is a small cost. Automation (Chapter 70) has removed
even that.

## The commands

```
show vlan brief                     # VLANs and their ports
show interfaces trunk               # trunks, allowed VLANs, native VLAN
show interfaces Gi0/5 switchport    # one port's full VLAN state
show vtp status                     # mode and revision number — check this
show interfaces status              # a quick overview of ports and VLANs
```

`show interfaces trunk` is the one to reach for during an incident. It shows, on one
screen, which VLANs are **allowed**, which are **active**, and which are **forwarding**
— and the difference between those three columns is where most VLAN faults live.

## What breaks here

**A device on a trunk port sees nothing.** It does not understand tags. Make the port
an access port.

**A VLAN works on one switch and not the next.** Not in the trunk's allowed list, or
not defined on the second switch. `show interfaces trunk` shows both.

**Traffic appearing in the wrong VLAN.** Native VLAN mismatch between trunk ends.

**Every VLAN vanished at once.** VTP. Check `show vtp status` for the revision number
and find what was recently connected.

**A port became a trunk unexpectedly.** DTP. Disable it everywhere.

**Half a change window spent restoring VLANs.** `switchport trunk allowed vlan 40`
without `add`.

> **Network+ note.** Objectives 2.3 and 4.2 expect **VLAN hopping** — both the
> switch-spoofing (DTP) and double-tagging variants — and their mitigations. Over-
> learn: **access port = one VLAN, untagged; trunk = many VLANs, tagged**; **the
> native VLAN is untagged on a trunk**; **default native and default access VLAN are
> both 1**; **change the native VLAN, do not use VLAN 1, disable DTP, shut unused
> ports**. This is among the most heavily examined security topics at Layer 2.
