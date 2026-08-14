# 54.3 Logging, Syslog and Correlation

A log entry on the device that generated it is useless in exactly two situations: when the
device has failed, and when you need to compare its behaviour with another device's.

Those are the two situations you have logging for.

## Syslog's structure

A protocol from the 1980s, standardised retroactively in RFC 5424, and its structure is
worth knowing because every field is used operationally.

```
   <134>1 2026-04-12T02:41:07.331Z sw-hq-02-1 %LINK-3-UPDOWN - -
    │  │  │                        │          │
    │  │  │                        │          └── the message
    │  │  │                        └───────────── hostname
    │  │  └────────────────────────────────────── timestamp, with timezone
    │  └───────────────────────────────────────── version
    └───────────────────────────────────────────── priority = facility × 8 + severity
                                                    134 = 16×8 + 6 = local0, informational
```

### The eight severities

| | Level | Meaning | Act? |
|---|---|---|---|
| **0** | **Emergency** | **system unusable** | **immediately** |
| **1** | **Alert** | **action required immediately** | **immediately** |
| **2** | **Critical** | critical condition | **soon** |
| **3** | **Error** | **error condition** | **investigate** |
| **4** | **Warning** | warning condition | **review** |
| 5 | Notice | normal but significant | log |
| **6** | **Informational** | **informational** | **log** |
| **7** | **Debug** | debug | **do not, in production** |

The numbering is inverted relative to intuition — **lower is worse** — and "log level 6"
means "informational and above", i.e. everything except debug. **This catches people.**

**The facilities** — kernel, mail, daemon, auth, and **local0 to local7** — matter mostly
because `local0`–`local7` are the ones you assign yourself, and using them consistently lets
the collector route messages sensibly.

## Centralise

The single most important thing in this section.

```
   Without central logging:              With:

   ┌────┐ logs                          ┌────┐──┐
   │ sw │ locally, in a                 │ sw │  │
   └────┘ small circular                └────┘  │   ┌──────────┐
   ┌────┐ buffer that                   ┌────┐  ├──▶│ Collector│
   │ rtr│ wraps in hours                │ rtr│  │   │ + search │
   └────┘ and is lost on                └────┘  │   └──────────┘
   ┌────┐ reboot                        ┌────┐  │
   │ fw │                               │ fw │──┘
   └────┘                               └────┘
```

**Three arguments, and each alone is sufficient:**

**The device may be gone.** A switch that crashed, lost power, or was replaced took its logs
with it — and its last few messages are exactly what you needed.

**Correlation requires one place.** "The firewall logged a session teardown 300 ms before the
router logged the BGP session dropping" is a diagnosis. It is not available if the two logs
are on two devices.

**Buffers are small.** A device's local log buffer is typically a few thousand lines and
wraps in hours on a busy device — or in seconds during the event you care about, because
that is when logging volume spikes.

**And centralising is cheap:** `logging host 10.0.0.50` on every device, plus a collector.
The cost is almost entirely in the collector's storage and in nobody looking at it.

## Time, and why NTP is a networking problem

> Log entries from two devices whose clocks differ by four minutes cannot be sequenced, and
> sequencing is the entire point.

Chapter 41 §41.3 insisted that clock skew is a networking problem. This is why.

```
   Router A  02:41:07  BGP neighbour down
   Firewall  02:37:22  interface Gi0/1 down     ← 4 minutes earlier?
                                                  or is the firewall's clock wrong?
```

With skewed clocks you cannot tell whether the firewall event caused the router event or
followed it, and causality is what an investigation is trying to establish.

**The requirements, in order:**

1. NTP on every device, from at least two sources
2. **The same sources**, so that even a common error is a common error
3. **UTC everywhere**, with the timezone applied at display time — never local time in the
   log
4. Timestamps including sub-second precision, which many platforms disable by default
5. Monitoring of clock offset itself, because NTP failing silently is the normal failure
   mode

**Point 3 deserves emphasis.** A device logging in local time across a daylight-saving
transition produces an hour that occurs twice or not at all, and an estate spanning
timezones produces logs that cannot be merged. Log in UTC; display in whatever you like.

## Log levels in production

The discipline that prevents a specific, documented class of outage.

Debug-level logging generates volume that overwhelms storage and, on some platforms, the
device's own CPU.

> There are documented outages caused by logging configuration alone — a device configured
> to log every packet matching an ACL, or debug output left enabled after a troubleshooting
> session, consuming the control-plane CPU until the device stopped forwarding.

**The rules:**

| | |
|---|---|
| **Production default** | **informational (6) to the collector, warning (4) to the console** |
| **Console logging** | **disable or restrict** — console output is synchronous on many platforms and **blocks** |
| **Debug** | **only during active troubleshooting, with a timer or a reminder to remove it** |
| **ACL logging** | **rate-limit it**, always — a permit/deny log on a busy rule is a self-inflicted flood |
| **After troubleshooting** | **turn it off.** Write it on the change record. |

The console point is the one that surprises people. On many platforms, writing to the
console is synchronous — the device waits for the characters to be emitted — and at 9,600
baud a burst of log messages can stall packet forwarding. `logging console` should be off
on production equipment, and `logging buffered` plus a collector used instead.

## Correlation in practice

**What the centralised log is for.**

**Start from a timestamp and widen.** Given "users reported problems at about 14:30", search
all sources for 14:25 to 14:35, and read the sequence rather than filtering for what you
expect.

> **The commonest investigation error is filtering too early.** Searching for `error` finds
> errors; the event that caused the incident may have been logged as informational — an
> interface coming up, a configuration change, a DHCP scope exhausting.

**Look for what stopped.** A message that appears every five minutes and then does not is
frequently more informative than a new message.

**Correlate across layers.** Chapter 65's layered diagnosis, applied to logs: a physical
event (link down), a Layer 2 event (topology change), a Layer 3 event (adjacency lost), an
application event (health check failure) — in that order, within a second — is a chain, and
the first one is the cause.

**Watch for the flood.** A device generating thousands of identical messages is telling you
something is oscillating, and the message content matters less than the rate.

## What to alert on, from logs

Most log-based alerting is badly done, and §54.4 develops the principles. **The
log-specific ones:**

| Alert on | Not on |
|---|---|
| **Severity 0–2, always** | every severity 3 |
| **Specific known-bad messages** | **any message containing "error"** |
| **Rate of a message** — 50 in a minute | a single occurrence of most things |
| **Absence** of an expected message | |
| **Configuration change messages** | |
| **Authentication failures, aggregated** | **each failed login** |

"Absence of an expected message" is the underused one. A device that logs a successful
backup nightly and stops is telling you something, and no threshold will catch it.

## Retention, and the awkward questions

**Two constraints pull in opposite directions.**

**Investigation wants long retention.** A breach is typically discovered months after it
occurred (Chapter 62), and logs from before the discovery are what establish scope.

**Storage, cost and privacy want short retention.** Logs contain personal data — usernames,
addresses, which sites were visited — and in many jurisdictions that carries legal obligations
about retention period, access control and deletion.

**A defensible arrangement:**

| Tier | Retention | Content |
|---|---|---|
| **Hot, searchable** | **30–90 days** | everything |
| **Warm, archived** | **1 year** | everything, compressed |
| **Cold** | **as long as policy requires** | **security-relevant only** |
| **Aggregated metrics** | **indefinitely** | **counts, not content** |

And access must be controlled and audited, because the log system holds a record of
everyone's activity — which makes it both a security asset and a security target.

## What breaks here

Logs from two devices that cannot be sequenced. **NTP.** Fix it before investigating
anything else; the investigation is not possible otherwise.

A device's logs missing for exactly the period of interest. **Local buffer wrapped**,
because the event generated a flood. Central logging would have kept it.

**Timestamps an hour out, twice a year.** Local time and daylight saving. Log in UTC.

A device becoming slow or unresponsive after enabling logging. Console logging, or
unrate-limited ACL logging. This is the outage caused by monitoring.

No logs from a device that is clearly having problems. It cannot reach the collector —
which is itself information, and an argument for alerting on the absence of expected
messages.

A search returning nothing because it filtered for "error". Widen to a time range and read
the sequence. The cause is often informational.

Log storage filling and old data being deleted silently. Discovered during an
investigation. Monitor the log system as carefully as the network.

> **Network+ note.** Objective 3.1 covers logging. Over-learn: **syslog uses UDP 514** (and TCP
> or TLS 6514 where secured); severity levels run 0 (Emergency) to 7 (Debug), lower being
> more severe; logs should be centralised on a syslog server or SIEM; and **NTP is
> required for meaningful correlation.** The severity ordering is examined and the inversion
> catches people.
