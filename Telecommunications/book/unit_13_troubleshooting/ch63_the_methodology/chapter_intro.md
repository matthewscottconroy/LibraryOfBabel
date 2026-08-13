# Chapter 63 — The Methodology

In 1982, a British Airways Boeing 747 flew into a volcanic ash cloud over Java and all
four engines failed. The crew had no procedure for four-engine failure — the scenario
was considered so improbable that no checklist existed — and they had eight thousand
metres of altitude and about fifteen minutes.

What they did was systematic rather than inspired. They worked the problem in a
defined order: fly the aircraft first, then diagnose, then act. They did not guess.
They restarted the engines by the procedure for a single engine restart, applied four
times, and after several attempts three relit.

Aviation's discipline of *aviate, navigate, communicate* — and its insistence that
under pressure you follow a method rather than your instincts — exists because
instincts under pressure are unreliable in specific, predictable ways. Network
troubleshooting is not aviation and nobody dies, but the cognitive failure modes are
identical, and the response is the same: **have a method, and use it precisely when
you feel least inclined to.**

## The failure modes a method prevents

§63.1 names them, because recognising your own behaviour is most of the fix.

**Confirmation bias.** You suspect the firewall. You look at the firewall. You find
something in the firewall logs that could be related — there is always something in
the firewall logs — and you stop looking. The evidence did not point to the firewall;
you pointed at the firewall and then found evidence.

**Recency bias.** The last three problems were DNS, so this is DNS.

**Changing multiple things at once.** Under pressure, you restart the service, clear
the ARP cache and reseat the cable. It works. **You now do not know what was wrong**,
cannot prevent recurrence, and cannot write a useful record.

**Acting before observing.** Rebooting destroys the state that would have identified
the cause. Counters reset, tables clear, logs roll. The reboot may fix the symptom and
it guarantees you will meet the problem again with no more information than you have
now.

**Anchoring on the reporter's diagnosis.** "The VPN is broken" is a hypothesis
presented as an observation. The observation is "I cannot open the file share." Those
are different statements, and starting from the first can waste an hour.

## The seven steps

CompTIA's methodology, which N10-009 examines, and which §63.2 derives from the
failure modes above rather than presenting as a list to memorise.

**1. Identify the problem.** Gather information, question users, identify symptoms,
determine what changed, and — critically — **duplicate the problem if possible.** A
fault you cannot reproduce is a fault you cannot confirm you have fixed.

The most valuable question, and it is asked far too rarely: *when did it last work?*
Followed by: *what changed between then and now?* Because Chapter 55 established that
most unplanned outages are caused by planned changes, and the change log is therefore
the highest-yield evidence available.

**2. Establish a theory of probable cause.** Note the order: this comes *after*
gathering evidence. Question the obvious. Consider multiple approaches — top-down,
bottom-up, divide-and-conquer.

**3. Test the theory.** If confirmed, proceed. If not, establish a new theory or
escalate. **Testing means an observation that would distinguish this theory from
others**, not an action that would fix it if the theory is right.

**4. Establish a plan of action**, including the side effects and the rollback.

**5. Implement the solution or escalate.**

**6. Verify full system functionality**, and implement preventive measures. "Full"
matters: fixing one user's access while breaking another's is a common outcome of a
hurried fix.

**7. Document findings, actions and outcomes.** The step that is skipped, and the step
that determines whether the organisation gets better or merely gets through.

## Choosing the direction

§63.3 covers the three approaches and — more usefully — when each is right, since the
usual treatment presents them as equivalent options.

**Bottom-up** (Layer 1 upward). Right when a physical problem is plausible: a new
installation, after building work, or when a link light is off. Wrong when you already
know the physical layer is fine, since you will spend twenty minutes confirming it.

**Top-down** (Layer 7 downward). Right when one application fails and others work,
which localises the fault above the layers they share.

**Divide and conquer.** Start in the middle — usually Layer 3, with a `ping` — and
bisect. Right in most situations, and the default, because a single test at Layer 3
eliminates either the three layers below it or the four above.

**Follow the path** is the fourth, and it is the one experienced engineers use most.
Trace the traffic's actual route hop by hop and test at each point. It requires knowing
the topology, which is what Chapter 53's documentation is for, and it is the approach
that finds faults the layered methods miss because the fault is *between* two things
that are each fine.

## Documenting

§63.4 makes the case, and the case is economic rather than moral.

A record that says "restarted the service, resolved" is worthless. A record that says:

> *Symptom: users in Building C could reach internal servers but not the Internet,
> intermittently, starting 14:20. Evidence: default route present; `ping` to gateway
> succeeded; `ping` to 8.8.8.8 failed roughly 40% of the time; traceroute showed loss
> beginning at the second ISP hop; the ISP's other circuit was unaffected. Cause: the
> primary circuit's BGP session was flapping due to a fibre fault 6 km away. Fix:
> ISP repaired at 19:40; we shifted traffic to the secondary in the interim.
> **What made this hard:** our monitoring alerted on circuit *down* but not on
> circuit *degraded*, so we learned about it from users.*

— is worth an hour of anyone's time, because the last sentence generates an
improvement. **The most valuable field in an incident record is what made it hard to
find**, and it is the field almost no template includes.

## By the end you will be able to

- Name five cognitive failure modes in troubleshooting and recognise them in your own
  work.
- Execute the seven-step methodology on a described fault.
- Distinguish a test that discriminates between theories from an action that merely
  fixes.
- Choose between bottom-up, top-down, divide-and-conquer and follow-the-path, with a
  reason.
- Write an incident record that improves the next incident.
