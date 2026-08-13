# 59.3 Authorization Models and Least Privilege

**§59.1's Target example: authentication worked and authorization failed.** **This section is
about the half that fails.**

## The models

**Four, in increasing order of expressiveness and administrative cost.**

| Model | Decision based on | Cost |
|---|---|---|
| **DAC** — discretionary | **the resource owner decides** | **low; and it drifts** |
| **MAC** — mandatory | **labels and a system-wide policy; owners cannot override** | **high; used where classification is legally required** |
| **RBAC** — role-based | **the subject's role** | **moderate; the practical default** |
| **ABAC** — attribute-based | **attributes of subject, resource, action and context** | **high; expressive; hard to audit** |

**RBAC is what almost every organisation actually uses, and its failure mode is specific:**

> **Role explosion.** **Each exception becomes a new role.** `Finance`, then `Finance-Manager`,
> then `Finance-Manager-EMEA`, then `Finance-Manager-EMEA-ReadOnly`. **After five years there
> are more roles than people, and nobody can say what any of them grants.**

**Which is Chapter 55 §55.1's accumulation argument, in an access control system**, and it has
the same properties: **monotonic, invisible, and compounding.**

**ABAC is the answer to role explosion and introduces its own problem:**

```
   PERMIT if
     subject.department == resource.owner_department
     AND subject.clearance >= resource.classification
     AND request.time within subject.working_hours
     AND request.device.compliant == true
```

**Expressive, and "who can access this?" becomes a question requiring evaluation rather than a
lookup.** **Auditing an ABAC policy is genuinely hard**, and **an organisation that cannot answer
"who has access to X" has a problem regardless of how elegant the policy language is.**

**The pragmatic position:** **RBAC for the structure, with attributes as conditions on it.**
**"Role grants the permission; attributes constrain when it applies"** — which is what most
modern systems (Azure AD conditional access, AWS IAM with conditions) actually implement.

## Least privilege

**Saltzer and Schroeder, 1975** (Chapter 57's reading) — **and it is easy to state and hard to
do.**

> **Every subject should have the minimum privileges necessary to perform its function, and no
> more.**

**Why it is hard, stated honestly:**

**Nobody knows what the minimum is.** **Determining the actual permissions a role requires means
observing it**, and until you do, the safe-feeling option is to grant more.

**The failure modes are asymmetric.** **Too little privilege produces an immediate, visible,
attributable complaint.** **Too much produces nothing at all, until a breach.** **The incentive
points at over-granting, permanently.**

**Privileges accumulate with tenure.** **A person who has moved between three roles in eight
years holds the union of all three**, because **joining a role adds permissions and leaving one
rarely removes them.**

**And revocation requires knowing what to revoke.** Chapter 53's inventory problem, in identity.

### What actually works

**Four mechanisms, in increasing order of effect and effort.**

**Just-in-time elevation.** **Nobody holds administrative rights standing.** **They request them,
for a stated reason, for a bounded period**, and the grant is logged and expires automatically.
**This is the single highest-value change available**, because it converts a permanent standing
target into a time-boxed one — **and because a compromised account holds no privileges by
default.**

**Access review, with a default of removal.** **Periodic recertification in which a manager
confirms each grant** — **and the crucial design detail is that unreviewed grants are removed
rather than retained.** **A review where "no response" means "keep" achieves nothing**, and most
reviews are configured that way.

**Attribute-derived access.** **Access follows from HR data — department, role, location — so a
transfer changes access automatically.** **Removes the accumulation problem at its source**, and
requires the HR data to be accurate, which is a real dependency.

**Break-glass accounts.** **A small number of highly privileged accounts, credentials in a
safe, use alarmed, every action recorded.** **Necessary** — **because the alternative is standing
privilege "in case of emergency"** — **and they must be tested, or they will not work when
needed** (Chapter 56 §56.2).

## Privilege in network devices

**The specific application, and it is where a network engineer's own authorisation lives.**

**Privilege levels** are the traditional mechanism: **level 1 for `show` commands, level 15 for
everything**, and **almost every organisation uses only those two** — which is a binary choice
between reading and total control.

**Role-based CLI access** and **TACACS+ per-command authorisation** (§59.2) are the answers, and
a workable set of roles:

| Role | May |
|---|---|
| **Monitoring** | **`show` commands only; no configuration** |
| **Operator** | **interface enable/disable, clear counters, port descriptions** |
| **Engineer** | **routing, VLANs, ACLs — the standard change set** |
| **Security** | **firewall policy, AAA configuration** |
| **Administrator** | **everything, including AAA — and this is a break-glass role** |

> **The role that should be smallest is the one permitted to change AAA configuration**,
> because **an engineer who can reconfigure authentication can grant themselves anything and
> remove the evidence.** **Separate it, and alarm on changes to it.**

**And the accounting half matters here more than anywhere:** **TACACS+ logs every command
attempted, permitted or denied, with the username.** **Which is the record that answers "who
changed this?" in Chapter 55 §55.2's terms**, and **it is why device administration should not
use shared local accounts.**

**The shared `enable` password is the specific failure to eliminate.** **A password known to
eight people produces logs that say a change was made by "admin", and nothing more.**

## Segregation of duties

**A control from accounting that transfers directly and is under-used in networking.**

> **No single person should be able to complete a sensitive action alone.**

| Action | Split |
|---|---|
| **Firewall rule change** | **requested by one, approved by another, implemented by a third** |
| **Granting administrative access** | **requested and approved separately** |
| **Reviewing logs** | **not by the person whose actions they record** |
| **Deploying a change** | **not the person who wrote it** — Chapter 55 §55.2's peer review |

**Its cost is real** — **it slows work and requires enough people** — **and in a team of three it
is frequently impossible.** **Say so rather than pretending.** **The compensating control in a
small team is logging and after-the-fact review**, which is weaker and is not nothing.

**The specific case worth insisting on even in a small team:** **the person who can grant access
should not be the person who reviews whether access was appropriately granted.**

## Accounting

**The third A, and it is the one that makes the other two verifiable.**

| Records | For |
|---|---|
| **Who authenticated, when, from where, successfully or not** | **detection, and Chapter 62's investigation** |
| **What authorisation decisions were made** | **auditing the policy, not just the outcome** |
| **What commands were executed** | **Chapter 55's change attribution** |
| **Session duration and volume** | billing, capacity, anomaly detection |

**Two design points:**

**Log denials as well as permissions.** **A denial is more interesting than a permission** — it
is either a misconfiguration or an attempt, **and both are worth knowing.** **Most systems log
successes by default and denials only if asked.**

**Send it somewhere the subject cannot alter.** Chapter 54 §54.3's centralisation, with a
security rationale: **an administrator who can delete the log of their own actions has no
accountability at all**, and **the log's value depends on its integrity.**

## What breaks here

**Correct authentication and excessive access.** **The Target pattern**, and the commonest real
finding.

**More roles than people.** **Role explosion.** Attributes as conditions on fewer roles.

**A user who moved departments and holds both sets of permissions.** **Joining adds, leaving does
not remove.** Attribute-derived access, or a review with removal as the default.

**An access review where everything was approved.** **The default was "keep" and nobody read
it.** Default to removal.

**Standing administrative rights on everyone's normal account.** **A compromised account is
immediately an administrative compromise.** Just-in-time elevation.

**Everyone at privilege level 15.** **A binary choice between reading and total control.**
Per-command authorisation.

**A change log that says "admin".** **A shared account.** Individual accounts, always.

**An engineer who could change AAA configuration and did.** **The role was not separated**, and
the evidence may not survive.

**Denials not logged.** **You cannot see attempts**, which is the interesting half.

**Segregation of duties impossible in a team of three.** **Say so, document the compensating
control, and do not claim the control you do not have.**

> **Network+ note.** Objective 4.1 and 4.3. Over-learn: **least privilege grants the minimum
> access necessary**; **RBAC assigns permissions by role**; **separation of duties requires
> more than one person for sensitive actions**; **accounting logs what was done**; and
> **authorization is distinct from authentication.** The authentication/authorization
> distinction is examined in almost every form and is worth being pedantic about.
