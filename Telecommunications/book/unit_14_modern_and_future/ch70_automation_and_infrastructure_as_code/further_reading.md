# Chapter 70 — Further Reading

## Read these first

**Edelman, J., Lowe, S. & Oswalt, M. — *Network Programmability and Automation* (2nd ed.,
2021).**
**The book for this chapter.** **It establishes the curriculum — Python, data formats,
templating, APIs, source of truth, testing — in that order**, and it assumes a network engineer
rather than a programmer.

**Humble, J. & Farley, D. — *Continuous Delivery*.**
**Written about software and applicable throughout.** **The chapters on the deployment pipeline
and on configuration management are §70.4's structure.**

**Forsgren, N., Humble, J. & Kim, G. — *Accelerate*** (Chapter 55's reading).
**The evidence that smaller and more frequent is safer**, which is the argument against the large
infrequent change window.

**Burgess, M. — the CFEngine documentation and *In Search of Certainty*.**
**Where convergent configuration and promise theory come from**, and the promise-theory argument
about verification being a separate act is worth the effort.

## Protocols and models

**RFC 6241 (NETCONF), RFC 8040 (RESTCONF), RFC 6020 and RFC 7950 (YANG 1.0 and 1.1).**
**RFC 6241's operations section is short.** **RFC 7950's type system and constraint sections are
the ones to read** — they are what §70.2 says makes the interface a contract.

**The gNMI specification and reference implementation** (github.com/openconfig/gnmi).
**Short, and the `Subscribe` semantics are the part that matters.**

**The OpenConfig models** (github.com/openconfig/public).
**Read the interfaces model**, and **compare it with RFC 2863's MIB** (Chapter 54's reading) to
see what thirty years of hindsight bought.

**RFC 8342 — the Network Management Datastore Architecture.**
**Clarifies the datastore model that NETCONF left ambiguous** — intended, applied, operational —
and it is the vocabulary current tooling uses.

## Tools

**Ansible's network modules documentation**, and the vendor collections.
**Read the `state` parameter documentation carefully** — §70.3's argument about `merged` versus
`replaced` lives there, and the semantics differ subtly between modules.

**Terraform's documentation, particularly the state and the plan material.**
**And the "Terraform state" page specifically**, because §70.3's liabilities are all in it.

**Netmiko, NAPALM, Nornir, `scrapli`, `ncclient`.**
**The libraries most real automation runs on.** **Byers's Netmiko documentation is the practical
entry point**, and **NAPALM's `compare_config` and `commit_config` are the safest available
starting point for making a change from code.**

**NetBox and Nautobot** (Chapter 53's reading) — **F2 uses one.** **The data model is the
teaching material**; modelling your own network in it reveals more than the tool provides.

**Jinja2's documentation**, and specifically its guidance on when logic belongs elsewhere.

**Batfish** (Chapter 55's and Chapter 68's reading) — **F6.** **Free, and it makes reachability a
test.**

**containerlab** (Chapter 67's reading) — **F5.** **Virtual topologies from a YAML file**, which
is what makes network integration testing achievable.

**`pytest` with `pytest-ansible`, `pyATS`/Genie, and `robot framework`** — the testing
frameworks in use. **pyATS's parsers are the largest available library of structured `show`
command output** and are useful independently of the framework.

## Community and practice

**The Network to Code and NetDevOps communities** — **Slack, blogs, and the annual events.**
**This is where the practice is discussed**, and it is more current than anything published.

**Ivan Pepelnjak's automation material** (`ipSpace.net`) — **recommended throughout this book and
particularly here.** **His courses and blog are the most rigorous available treatment**, and his
scepticism about intent-based products (Chapter 68) extends to automation vendors generally.

**The Packet Pushers' automation and `Day Two Cloud` podcast archives** — **practitioners
describing what they built and what failed.**

**Oswalt's writing on testing network automation** — **§70.4's pyramid is substantially his**,
and the material on what a virtual lab does and does not verify is honest in a way most is not.

## Observability and AIOps, read sceptically

**Majors, C., Fong-Jones, L. & Miranda, G. — *Observability Engineering*.**
**The monitoring-versus-observability distinction that §70.4 uses**, argued properly.

**Google's SRE material on monitoring** (Chapter 54's reading) — **and the four golden signals**,
which is a better starting point than any product.

**The academic literature on anomaly detection in networks**, sampled rather than read.
**Denning's 1987 false-positive constraint** (Chapter 57's reading) **has not been repealed**,
and papers that do not address it are describing a demonstration rather than a system.

**Vendor AIOps documentation**, read with §70.4's five questions in hand. **F8 uses them.**
**The question that separates products is whether it can show you why it concluded what it
concluded.**

## Security of the automation platform

**NIST SP 800-53's configuration management controls**, and **the CIS Controls' material on
secure configuration.**
**Relevant because §70.2's argument is that the automation credential is the most valuable one in
the estate.**

**HashiCorp Vault's, and the equivalents', documentation on secrets management.**
**Credential storage, rotation and dynamic credentials** — **and the last is the interesting one:
an automation run receiving a credential valid for its duration only.**

**Your platform's per-command authorisation documentation** (Chapter 59 §59.2's TACACS+) —
**because "the automation account should be authorised for what the automation does" is achievable
and almost never done.**

## Where to look next

**Chapter 55 §55.4** is where this chapter's argument began — the repository as source of truth
rather than record; **Chapter 68 §68.4** is the verification half, sold as a product; **Chapter
69** is the environment where this is the only way to work; and **Chapter 72** takes up what an
automated network means for how a network is designed and by whom.
