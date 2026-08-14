# Chapter 53 — Further Reading

## The four things to read first

Cook, R. (1998). "How Complex Systems Fail."
Four pages, eighteen numbered points, freely available. **Read it today.** It is the most
useful document about operations anyone has written, and it takes ten minutes.

**Gawande, A. — *The Checklist Manifesto* (2009).**
Short, and it will change how you write procedures. The distinction between ignorance and
ineptitude is the part to carry away.

Beyer, B., Jones, C., Petoff, J. & Murphy, N. (eds.) — *Site Reliability Engineering* (2016),
and *The Site Reliability Workbook* (2018).
**Free at sre.google.** The chapters on toil, on documentation, and on postmortem culture are
the ones for this chapter; the availability material belongs with Chapter 56. The Workbook
is the more practical of the two.

Dekker, S. — *The Field Guide to Understanding "Human Error"* (3rd ed.).
The reframing that makes incident review productive. If the whole book is too much, the
first two chapters carry the argument.

## Documentation practice

ANSI/TIA-606-D — "Administration Standard for Telecommunications Infrastructure."
Purchasable, and summaries are freely available. Read a summary rather than the standard
unless you are specifying a large installation. F4 uses it.

**ANSI/TIA-568 and ANSI/TIA-569** — cabling and pathways. Relevant context for §53.2's
physical records.

Uptime Institute's tier documentation, and the various data centre design guides.
Useful for what a rack elevation and a power record should contain, even at small scale.

Google's "Documentation as Code" material, and the Diátaxis framework (diataxis.fr).
Diátaxis is the most useful thing written about technical documentation structure — it
separates tutorials, how-to guides, reference and explanation, and it explains precisely why
most runbooks fail: they are how-to guides written as explanations.

## Address management

**Chapter 27's further reading applies here.** In addition:

RFC 8981 — "Temporary Address Extensions for SLAAC."
Why an IPv6 host has addresses you never assigned, and why they should not be treated as
allocations.

NetBox and Nautobot documentation (netbox.dev, nautobot.com).
Free, open source, and the current standard for network source-of-truth modelling. **F2
uses one.** The data model itself is instructive — it forces you to name things you had not
written down.

**phpIPAM, NIPAP** — lighter-weight alternatives, appropriate where NetBox is more than the
organisation needs.

Infoblox and BlueCat product documentation, read as a specification of what integrated DDI
does rather than as marketing. The DNS/DHCP/IPAM consistency argument of §53.3 is made most
clearly in their own material.

## Incident practice and human factors

Allspaw, J. — "Blameless PostMortems and a Just Culture" (Etsy engineering blog, 2012), and
his subsequent writing at Adaptive Capacity Labs.
Short, practical, and the epistemic argument for blamelessness rather than the moral one.

Perrow, C. — *Normal Accidents: Living with High-Risk Technologies* (1984).
Where "tightly coupled" and "complex interactions" come from. Long, and the introduction
and the Three Mile Island chapter carry the argument.

Woods, D., Dekker, S., Cook, R., Johannesen, L. & Sarter, N. — *Behind Human Error*.
The scholarly treatment. Read it after Dekker's field guide, if the field guide interested
you.

The PagerDuty Incident Response documentation (response.pagerduty.com).
Free, specific, and unusually well organised — roles, severity definitions, communication
templates. A good model for the runbook index of D4.

**Learning From Incidents** (learningfromincidents.io) and its associated writing.
The current community around Cook and Woods's ideas applied to software operations.

## Process frameworks, used sparingly

**ITIL 4 Foundation material.**
Read a summary for the vocabulary — incident, problem, change, configuration item, known
error. The vocabulary is the contribution. Do not implement the documents.

NIST SP 800-53 and ISO/IEC 27001 documentation requirements.
Relevant if your organisation is subject to them, and worth skimming otherwise for the
list of records that auditors expect to exist, which overlaps substantially with what an
incident actually needs.

## Tools

**NetBox / Nautobot** — source of truth. The single highest-value tool in this chapter.

LibreNMS, Observium, or a scripted `lldpctl`/SNMP collector — automated topology and
inventory discovery. **F3 uses one.** The value is the diff against your records, not the
inventory itself.

**draw.io / diagrams.net, Excalidraw, Mermaid, Graphviz.**
Mermaid and Graphviz are text-based, which means diagrams live in version control and diff
sensibly — a substantial advantage over binary drawing files for anything that changes.

**A label printer.** **Genuinely.** Brother P-touch or Dymo Rhino class, with self-laminating
wire wrap cartridges. It costs less than an hour of engineer time and it is the highest
return-on-investment purchase in this chapter.

**Git, for configurations and documentation alike.** Chapter 55 §55.4 develops this;
the habit of putting documentation under version control starts here.

## Where to look next

**Chapter 54** covers measuring what these records describe; **Chapter 55** supplies the change
process that keeps them current and treats configuration as accumulated liability; **Chapter
56** turns the inventory's EOL dates and the single-point-of-knowledge risk into availability
arithmetic; and **Chapter 63** is where the runbooks of §53.4 are actually used.
