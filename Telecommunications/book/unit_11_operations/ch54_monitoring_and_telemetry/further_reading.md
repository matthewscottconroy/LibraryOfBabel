# Chapter 54 — Further Reading

## Specifications

RFC 1157 (SNMPv1), RFC 3416 (SNMPv2c operations), RFC 3410–3418 (SNMPv3).
RFC 3410 is the readable overview — start there rather than with the protocol documents.

RFC 2863 — "The Interfaces Group MIB."
The MIB everything monitors. Read the section on `ifIndex` persistence and on the
64-bit `ifXTable` counters; both are §54.2's practical problems.

RFC 5424 — "The Syslog Protocol", and RFC 3164 — "The BSD syslog Protocol."
Read 3164 first for the history — it is explicitly descriptive rather than normative, which
is unusual and explains syslog's oddities. **RFC 5425** adds TLS transport; **RFC 6587** covers
TCP framing.

RFC 7011 — "Specification of the IPFIX Protocol."
The standardised flow export protocol. **RFC 7012** defines the information elements —
consult it rather than reading it.

**RFC 3176 — sFlow.**
Short, and unusually good about the statistics. The section on sampling accuracy states
the error bounds explicitly, which protocol documents rarely do.

RFC 6020 — YANG, and RFC 6241 — NETCONF.
YANG is the modelling language; read the introduction and the type system. gNMI is
specified at github.com/openconfig/gnmi.

**RFC 5905 — NTPv4.**
Because §54.3 depends on it entirely, and Chapter 41 §41.3 covers it.

## Books

Mauro, D. & Schmidt, K. — *Essential SNMP*.
The standard practical book. Slightly dated and still the clearest explanation of MIBs,
OIDs and the operations.

Rose, M. — *The Simple Book: An Introduction to Internet Management*.
By one of SNMP's designers. Read it for the design reasoning, and read Rose's later
critical writing alongside it.

Beyer, B. et al. — *Site Reliability Engineering*, chapters on monitoring and on being on
call (free at sre.google).
The best available writing on alert design. "Monitoring Distributed Systems" is the
chapter that matters for §54.4, and the four rules there are more rigorously argued than most
treatments.

**Ligus, S. — *Effective Monitoring and Alerting*.**
Specifically about the alerting problem, which most monitoring books treat as an
afterthought.

Turnbull, J. — *The Art of Monitoring*, and *Monitoring with Prometheus*.
Practical, current, and oriented towards the metrics-and-time-series model rather than the
SNMP one — which is where new deployments actually are.

## Papers and analysis

Appenzeller's and Cook's work is in Chapters 52 and 53's reading, and both bear on this
chapter.

Kerr, D. & Bruins, B. — the NetFlow patent (US 6,243,667, filed 1996).
Worth skimming to see that it is a forwarding-cache patent, with monitoring described as a
secondary benefit.

Phaal, P. & Lavine, M. — "sFlow Version 5" and the InMon technical notes on sampling
accuracy.
The mathematics of what sampling costs you, stated properly. §54.4's elephants-and-mice
argument comes from here.

Google's "Monarch" paper (VLDB 2020) and Facebook's "Gorilla" (VLDB 2015).
How time-series monitoring is actually built at scale. Gorilla's compression section is
the reason modern time-series databases can retain what they do, and it makes §54.1's
"downsample rather than delete" advice look conservative.

## Tools

**Prometheus + Grafana** — the current default for metrics. Pull-based, dimensional labels
rather than a hierarchical namespace, and `snmp_exporter` bridges to SNMP devices. Learn
this stack if you learn one.

LibreNMS, Observium, Zabbix, Icinga — traditional SNMP-centric network monitoring.
LibreNMS is the easiest to stand up for a real network and auto-discovers a great deal.

**Net-SNMP** — `snmpwalk`, `snmpget`, `snmpbulkwalk`, `snmptranslate`. **F3 uses these.**
`snmptranslate -Tp` prints a MIB as a tree, which is the fastest way to understand one.

**rsyslog / syslog-ng**, and Loki, Graylog, OpenSearch, or an ELK stack for search.
**F2 uses one.** Loki is the lightest for a small environment; a full ELK stack is
substantial to run.

nfdump / nfsen, pmacct, softflowd, `goflow2`, Akvorado, ElastiFlow.
**F4 uses one.** `softflowd` will export flows from a Linux host with no network equipment
required, which makes the exercise possible on a laptop.

**`ntopng`** — flow analysis with a usable interface, and it will run against a span port or a
flow export.

**gNMIc** (gnmic.openconfig.net) — a CLI for gNMI subscriptions. F5 uses it if you have
capable equipment.

`chrony` and `ntpq -p` — check your clock offset before investigating anything
time-correlated.

**SmokePing, and `mtr`** — latency and loss over time to reference destinations, which is
§54.1's most under-collected baseline measure and takes ten minutes to set up.

## Following the field

**The OpenConfig models** (github.com/openconfig/public) — **read one.** The interfaces model is
a good starting point, and comparing it with RFC 2863's MIB shows exactly what thirty years
of hindsight bought.

The Prometheus and OpenTelemetry communities — the observability world's centre of gravity
is there rather than in network-specific tooling, and network monitoring is slowly converging
on it.

NANOG and RIPE presentations on telemetry at scale — operators describing what they
actually run, which is consistently more instructive than vendor material.

## Where to look next

**Chapter 55** covers the change process that this chapter's data should gate; **Chapter 56**
turns baselines into availability arithmetic and capacity dates; **Chapter 63** and Chapter
64 are where this chapter's data is used under pressure; and Chapter 70 §70.2 develops
streaming telemetry as part of network automation.
