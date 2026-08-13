# Chapter 39 — Exercises

## A. Recall

**A1.** Give the four ways HOSTS.TXT failed and the DNS design decision that answers each.

**A2.** Distinguish a zone from a domain in one sentence.

**A3.** Name the five roles in a resolution and what each does.

**A4.** Distinguish recursive from iterative querying, and say which the stub does.

**A5.** What is a glue record and when is one required?

**A6.** Why are there thirteen root server addresses, and how many machines serve them?

**A7.** Give the record type for each: IPv4 address, IPv6 address, alias, mail routing,
delegation, reverse lookup, zone authority, service-and-port, arbitrary text.

**A8.** In an MX record, is a lower or higher preference number tried first?

**A9.** What does the SOA serial do, and what happens if it is not incremented?

**A10.** State the correct order of operations for a DNS migration.

## B. Apply

**B1.** Trace the complete resolution of `mail.eng.example.co.uk` from a cold cache. List
every query, who is asked, and what each returns.

**B2.** For each cache state, state how many queries the resolver must make to answer
`www.example.com`:

(a) empty  (b) has the root's `.com` referral  (c) has `.com`'s `example.com` referral
(d) has the answer

**B3.** Write the zone file for `example.org` with: two nameservers (`ns1`, `ns2`, at
`192.0.2.10` and `198.51.100.10`), a web server at `203.0.113.5` reachable as both the apex
and `www`, mail to `mail.example.org` at `203.0.113.25` with a backup at
`mx.backup.net` preference 20, an SPF record, and a delegation of `lab.example.org` to
`ns1.lab.example.org` at `192.0.2.60`.

**B4.** Identify every error in this zone fragment and give the correction:

```
$ORIGIN example.com.
@       IN  SOA ns1.example.com admin@example.com ( 1 7200 3600 1209600 3600 )
@       IN  CNAME  webhost.provider.net.
@       IN  MX     10 mail.example.com.
www     IN  A      93.184.216.34
www     IN  CNAME  www.example.com
mail    IN  CNAME  mailhost.example.com.
ftp     IN  A      93.184.216.40
```

**B5.** A record has TTL 86400 and must be changed at 14:00 on Friday with propagation
complete within five minutes. Give the complete schedule, with times.

**B6.** A domain uses round-robin DNS with three A records and TTL 300. One server fails at
09:00.

(a) What fraction of new lookups get the dead address?
(b) When does that stop, if you remove the record at 09:05?
(c) What would have prevented the outage entirely?

**B7.** Compute the entropy an off-path attacker must guess for a cache-poisoning attack
(a) with a fixed source port, (b) with port randomisation, (c) with port randomisation and
0x20 encoding over a 15-character name.

## C. Analyse

**C1.** Explain why a centralised HOSTS.TXT could not scale, in terms of how load grew with
network size.

**C2.** "Hierarchy solves two different problems at once." Identify both and explain how
delegation addresses each.

**C3.** Explain the glue-record chicken-and-egg problem and exactly when glue is
unnecessary.

**C4.** Explain why anycast is what allows the DNS root to survive volumetric attack.

**C5.** Explain why a CNAME cannot exist at a zone apex, and evaluate the three workarounds.

**C6.** Explain why DNS became an authentication mechanism (ACME, domain verification), and
what property of DNS makes it work.

**C7.** Explain the Kaminsky attack completely: what was different from prior
understanding, why it poisons a whole zone rather than one name, and why port randomisation
is a mitigation rather than a fix.

**C8.** Explain why DNSSEC deployment stalled, giving four reasons, and identify which is
the same obstacle as IPv6's.

**C9.** "DNSSEC does not protect the last hop." Explain what this means and why it drove
encrypted transport instead.

**C10.** Set out the DoH controversy fairly: three arguments for and three against, and
state what the canary-domain mechanism attempts.

**C11.** Analyse the Facebook 2021 outage as a dependency failure. Identify each layer of
the compounding and state the general lesson.

**C12.** Explain why the Dyn 2016 outage took down companies whose own infrastructure was
healthy.

## D. Design

**D1.** Design the DNS architecture for an organisation with public services, internal
services, and a requirement to survive the loss of any single provider. Specify zones,
providers, TTLs and the split-horizon approach.

**D2.** For the semester project's network, produce the complete zone file for the internal
domain plus the external records, with justified TTLs.

**D3.** Write the migration runbook for moving a website to a new provider with a hard
requirement of under two minutes of stale answers. Include the timeline and rollback.

**D4.** Design the monitoring for DNS: what you check, from where, how often, and what
alerts. Include DNSSEC expiry and delegation consistency.

**D5.** An organisation wants DNSSEC. Write the assessment: what it gains, what it risks,
what operational processes must exist first, and your recommendation.

## E. Troubleshoot

**E1.** Every DNS lookup takes exactly five seconds and then succeeds. Diagnose.

**E2.** `ping 8.8.8.8` works; `ping google.com` does not. Give the layer and the next two
commands.

**E3.** A record was changed four hours ago. Some users see the new value, some the old.
Explain and state what determines when it resolves.

**E4.** A newly-created record does not resolve, and `dig @ns1.example.com` returns it
correctly. Explain and give the wait time.

**E5.** Mail for a domain is being delivered to its web server. Diagnose.

**E6.** Secondary nameservers serve different data from the primary. The zone file is
correct. Diagnose.

**E7.** A domain becomes unreachable for some users and works for others, and the pattern
correlates with which resolver they use. The zone is DNSSEC-signed. Give the most likely
cause.

**E8.** A name resolves as `www.example.com.example.com`. Explain.

**E9.** DNSSEC validation fails for one zone; ordinary resolution works. The firewall
permits UDP/53. Give two candidate causes.

**E10.** The security team reports they can no longer see DNS queries from managed
laptops. Explain and give two remedies.

**E11.** `dig @ns1.example.com example.com AXFR` from an arbitrary host returns the whole
zone. Assess the severity and give the fix.

## F. Extend

**F1.** Run `dig +trace` for three names of increasing depth and annotate every referral.
Identify where glue appears and why.

**F2.** Set up a small authoritative server (BIND, Knot or NSD) for a test zone, add each
record type from §39.3, and verify each with `dig`. Then deliberately make each of B4's
errors and observe the failure.

**F3.** Measure your own resolver's cache behaviour: query a name, note the TTL, query
again after 10 seconds, and confirm the TTL is counting down. Explain what that proves.

**F4.** Use `dig CHAOS TXT id.server` and `dig +nsid` against a public resolver from two
different networks. Explain the results in terms of anycast.

**F5.** Sign a test zone with DNSSEC and validate it with `delv`. Then let a signature
expire deliberately and observe the failure mode.

**F6.** Capture DNS traffic for ten minutes on a busy machine. Count queries, identify the
most-queried names, and calculate what fraction were cache hits.

**F7.** Compare a DoH query and a plain DNS query in a capture. Explain what an on-path
observer learns from each.
