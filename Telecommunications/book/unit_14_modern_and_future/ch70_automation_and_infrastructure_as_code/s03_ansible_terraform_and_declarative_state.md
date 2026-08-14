# 70.3 Ansible, Terraform and Declarative State

Two tools that are frequently compared and are answering different questions, and the
distinction between imperative and declarative is the one that matters more than either.

## Imperative and declarative

| | **Imperative** | **Declarative** |
|---|---|---|
| Says | **do these steps** | **this is the desired state** |
| Assumes | **the starting state** | **nothing** |
| Running it twice | **may do it twice** | **changes nothing the second time** |
| Failure partway | **an unknown state** | **retry converges** |
| Expressed as | a script | **a description** |

> **Declarative is what makes automation safe to re-run**, and **idempotence is the property's
> name**: applying the same desired state repeatedly produces the same result.

Which is Chapter 55's Mark Burgess argument — convergent configuration, from 1993 —
and it is why a declarative tool can be run on a schedule to correct drift, and an imperative
script cannot.

And the distinction is not clean in practice: Ansible is declarative in intent and
imperative in structure — a sequence of tasks, each of which is meant to be idempotent —
and whether it actually is depends on the module.

## Ansible

Agentless, push-based, and it dominates network automation for a specific reason.

```
   ---
   - name: Configure access switches
     hosts: access_switches
     gather_facts: false
     tasks:
       - name: Ensure NTP servers
         cisco.ios.ios_ntp_global:
           config:
             servers:
               - server: 10.9.0.10
               - server: 10.9.0.11
           state: merged

       - name: Ensure the access port template
         cisco.ios.ios_l2_interfaces:
           config: "{{ access_ports }}"
           state: replaced
```

| Property | Why it matters here |
|---|---|
| **Agentless** | **network devices cannot run an agent** — this is the decisive property |
| **SSH or API transport** | works with what devices have |
| **YAML** | readable by people who are not programmers |
| **Inventory with groups and variables** | **the source of truth, if you let it be** |
| **Templates (Jinja2)** | **generate configuration from data** |
| **Modules per platform** | **and the coverage and quality vary enormously** |

The `state:` parameter is the part worth understanding, because it is where declarative
intent is expressed:

| | |
|---|---|
| **`merged`** | **add what is specified; leave the rest** |
| **`replaced`** | **make this resource match exactly; remove what is not listed** |
| **`overridden`** | **make the whole section match; remove other resources entirely** |
| **`deleted`** | remove |
| **`gathered`** | **read only** — and this is where to start (§70.1) |

> **`replaced` and `overridden` are what actually eliminate drift**, and **they are the ones
> people avoid**, because **removing configuration nobody understands is frightening**
> (Chapter 55 §55.1). Which means most Ansible network automation uses `merged` and therefore
> adds without ever removing — reproducing exactly the accumulation problem it was meant to
> solve.

Ansible's weakness is that it has no state. It knows what it did; it does not know what
exists. Which is fine for "ensure this is configured" and inadequate for "this resource
should no longer exist" unless you tell it explicitly.

## Terraform

Declarative, with state, and it is the right tool for the cloud (Chapter 69).

```
   resource "aws_vpc" "main" {
     cidr_block = "10.20.0.0/16"
     tags = { Name = "production" }
   }

   resource "aws_subnet" "app" {
     for_each          = var.availability_zones
     vpc_id            = aws_vpc.main.id
     cidr_block        = cidrsubnet(aws_vpc.main.cidr_block, 4, each.value.index)
     availability_zone = each.key
   }
```

**Three properties distinguish it:**

**It maintains a state file.** A record of what it created and its current attributes —
which is what lets it know that a resource it created no longer appears in the configuration
and must therefore be destroyed.

**It builds a dependency graph.** `aws_subnet.app` references `aws_vpc.main.id`, so the VPC is
created first — and the ordering is derived rather than written.

And `plan` shows what will change before it changes.

```
   Terraform will perform the following actions:
     + aws_subnet.app["eu-west-1c"] will be created
     ~ aws_security_group.web will be updated in-place
     - aws_instance.legacy will be destroyed
   Plan: 1 to add, 1 to change, 1 to destroy.
```

> **`terraform plan` is Chapter 55 §55.2's change record, generated automatically and
> accurately.** **It is the most valuable feature**, because it converts "what will this
> change do?" from a judgement into a computation — and a plan showing an unexpected
> destruction has caught more errors than any review process.

And the state file is also the liability:

| | |
|---|---|
| **It contains secrets** | **and must be stored securely** — Chapter 55 §55.4's argument |
| **It must be shared** | **a team needs a remote backend with locking** |
| **It can drift from reality** | **someone changed something by hand** |
| **Losing it is serious** | **Terraform no longer knows what it owns** |

Terraform for network devices exists — providers for the major vendors, and for NSX, ACI
and the controllers — and it is less mature than the cloud providers'. The honest
guidance is Terraform for anything with an API and a lifecycle (cloud, controllers), Ansible for
devices.

## Which for what

| | **Ansible** | **Terraform** |
|---|---|---|
| **Configuring an existing device** | **yes** | awkward |
| **Creating and destroying resources** | awkward | **yes** |
| **State tracking** | **none** | **yes** |
| **Ordering** | **you write it** | **derived** |
| **Preview of changes** | **`--check`, imperfectly** | **`plan`, accurately** |
| **Cloud networks** | possible | **the right answer** |
| **Network devices** | **the right answer** | improving |
| **Agentless** | **yes** | yes |

> The distinction is that Terraform manages resources with a lifecycle and Ansible configures
> things that already exist, and **a network estate has both.** **Most mature practice uses
> both**, which is untidy and correct.

## The source of truth

The piece that makes any of it work, and it is not a tool.

> Automation without a source of truth is a faster way to apply whatever someone typed into a
> variable file.

**What a source of truth holds:**

| | |
|---|---|
| **Devices, roles, sites, racks** | Chapter 53 §53.1 |
| **Interfaces and connections** | |
| **Addresses, prefixes, VLANs** | Chapter 53 §53.3 |
| **Circuits, providers, references** | Chapter 53 §53.2 |
| **The intended configuration parameters** | **not the configuration** |

**And the crucial property:**

```
   Source of truth ──▶ [ template ] ──▶ configuration ──▶ device
        ▲                                                     │
        └──────────── verified against ───────────────────────┘
```

> **The data lives once.** A VLAN's number, name, subnet, gateway and which switches carry it
> are recorded in one place, and the switch configuration, the DHCP scope, the DNS records,
> the monitoring configuration and the firewall object are all generated from it.

Which is what eliminates the class of error where three systems disagree (Chapter 53
§53.3's DDI argument, generalised).

NetBox and Nautobot are the standard implementations (Chapter 53's reading), and the
practical advice is to model the network in one before writing any templates — because the
modelling exercise reveals what you do not know, and that is more valuable than the automation.

## Templates, and the discipline they need

Jinja2 is the standard, and the failure mode is a template that becomes a program.

```
   {% for vlan in device.vlans %}
   vlan {{ vlan.id }}
    name {{ vlan.name }}
   {% endfor %}
   !
   {% for iface in device.interfaces if iface.role == 'access' %}
   interface {{ iface.name }}
    description {{ iface.description }}
    switchport mode access
    switchport access vlan {{ iface.vlan }}
    spanning-tree portfast
    spanning-tree bpduguard enable
   {% endfor %}
```

Which is readable, and it stops being readable at about a hundred lines of nested conditions.

**The discipline:**

- Logic belongs in the data or in the code, not in the template
- One template per device role, not one template with conditionals for every role
- Render and diff before applying — `--check` mode, or generating to a file
- And test the rendering — a template that produces syntactically invalid configuration
  should fail in CI (§70.4), not on a device

> The commonest template failure is not a bug; it is a template that nobody understands
> because the logic that should have been in the data model migrated into it — which is
> Chapter 55 §55.1's accumulation, in a new place.

## What breaks here

**Ansible using `merged` everywhere and configuration accumulating.** The tool reproducing the
problem it was meant to solve.

**A Terraform plan showing an unexpected destruction.** Read it. It is doing you a favour.

**Terraform state lost or corrupted.** It no longer knows what it owns. Remote backend,
locking, versioning and backups.

**Terraform state containing secrets, in a repository.** Chapter 55 §55.4's warning, exactly.

Someone changed a resource by hand and the next `apply` reverted it. **Working as
designed**, and it is the argument for stage 3 (§70.1) being an organisational change.

A template of four hundred lines with nested conditionals. The logic migrated from the data
model. Split by role.

**Automation applying data nobody validated.** A faster way to apply whatever someone typed.
A source of truth, with constraints.

A source of truth that describes rather than generates. **It will drift** (Chapter 55
§55.4). Generate from it, or accept that it is documentation.

> **Network+ note.** Objective 1.8 and 3.2. Over-learn: **infrastructure as code manages
> configuration in version-controlled files**; **Ansible, Terraform and similar tools automate
> deployment**; **playbooks and templates define desired state**; and **idempotence means
> reapplying produces no change.** The idempotence definition is examined and the declarative
> concept is what makes the rest coherent.
