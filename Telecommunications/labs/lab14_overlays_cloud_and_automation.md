# Lab 14 — Overlays, Cloud, and Automation

**Corresponds to:** Chapters 67, 69, 70
**Week:** 14
**Time:** 120 minutes

---

## Objectives

- Build a VXLAN overlay and observe the encapsulation on the wire.
- Compute and demonstrate the overlay's MTU consequence.
- Map cloud networking constructs onto the traditional concepts they implement.
- Reproduce the stateful/stateless security-group failure.
- Write declarative configuration, demonstrate idempotence, and detect drift.
- Test a network change against a model before deploying it.

---

## You will need

- Two or three Linux hosts (VMs are ideal) with `ip link` supporting VXLAN —
  any modern kernel.
- Docker or Podman for the container networking section.
- A cloud account with free-tier VPC capability, **or** the offline substitute in
  Part 3 if none is available.
- Ansible, or Python with `netmiko`/`napalm`, for Part 4.
- Containerlab, GNS3 or Packet Tracer for Part 5.

---

## Procedure

### Part 1 — Build an overlay

**1.** Two hosts on the same subnet — call them VTEP-A (`10.1.1.10`) and VTEP-B
(`10.1.1.11`). Verify they can ping each other. This is the **underlay**.

**2.** On each, create a VXLAN interface with VNI 100:

```bash
# on VTEP-A
sudo ip link add vxlan100 type vxlan id 100 remote 10.1.1.11 \
     dstport 4789 dev <underlay-iface>
sudo ip addr add 192.168.100.1/24 dev vxlan100
sudo ip link set vxlan100 up
```

```bash
# on VTEP-B — note the remote is reversed
sudo ip link add vxlan100 type vxlan id 100 remote 10.1.1.10 \
     dstport 4789 dev <underlay-iface>
sudo ip addr add 192.168.100.2/24 dev vxlan100
sudo ip link set vxlan100 up
```

**3.** Ping across the overlay: `ping 192.168.100.2`.

**4.** Capture on the **underlay** interface while pinging across the overlay:

```bash
sudo tcpdump -i <underlay-iface> -n -v udp port 4789
```

**5.** In the capture, identify every layer, outermost first:

| Layer | What to record |
|---|---|
| Outer Ethernet | the underlay MACs |
| Outer IP | the VTEP addresses |
| UDP | destination port 4789 |
| VXLAN header | the VNI — confirm it is 100 |
| Inner Ethernet | the overlay MACs |
| Inner IP | the overlay addresses |
| ICMP | the payload |

**6.** Compute the total encapsulation overhead in bytes and check it against the
frame sizes in your capture.

**7.** Check the VXLAN interface's MTU and compare with the underlay's:

```bash
ip link show vxlan100
ip link show <underlay-iface>
```

---

### Part 2 — Container networking

**8.** Create two containers on a bridge network and inspect what was built:

```bash
docker network create labnet
docker run -d --name c1 --network labnet alpine sleep 3600
docker run -d --name c2 --network labnet alpine sleep 3600
```

**9.** On the host, find the artefacts:

```bash
ip link | grep -E 'veth|br-'
bridge link
ip netns list
docker network inspect labnet
```

**10.** From inside a container, examine its network view:

```bash
docker exec c1 ip addr
docker exec c1 ip route
docker exec c1 ping -c 2 c2
```

**11.** Identify: the bridge, the veth pair, the container's namespace, the
address allocation, and how outbound traffic is translated. Draw it.

**12.** Confirm the NAT:

```bash
sudo iptables -t nat -L -n | grep -A5 DOCKER
```

---

### Part 3 — Cloud constructs

**With a cloud account:**

**13.** Create a VPC with a `10.50.0.0/16` CIDR, two subnets in different
availability zones, an internet gateway, and a route table.

**14.** Launch one instance in each subnet.

**15.** Make one subnet **public** — add a default route to the internet gateway —
and leave the other private. Confirm from each instance which can reach the
Internet.

**16.** Create a security group permitting inbound TCP 22 from your address, and
**nothing outbound explicitly**. Connect by SSH. Record whether it works and
explain why.

**17.** Now create a network ACL on the same subnet permitting inbound TCP 22 and
**nothing else**. Attempt SSH. Record.

**18.** Add an outbound rule for the ephemeral port range (1024–65535) to the
network ACL. Retest.

**19.** Peer two VPCs, and confirm that a third VPC peered to one of them **cannot**
reach the other.

**Without a cloud account — the offline substitute:**

**13a.** For each construct below, write down the traditional networking concept it
implements and the chapter that develops it. Then, for each, state one thing the
cloud version does that the traditional one does not, or vice versa:

VPC · subnet · route table · internet gateway · NAT gateway · security group ·
network ACL · availability zone · VPC peering · transit gateway

**14a.** Design an address plan for a three-VPC deployment (production, staging,
shared services) that must peer with each other and with an on-premises
`10.0.0.0/8` estate. State every CIDR and justify the non-overlap.

**15a.** Explain, with a packet walk, why a security group needs no outbound rule
for return traffic and a network ACL does.

---

### Part 4 — Declarative configuration

**20.** Write an Ansible playbook (or a `netmiko` script) that configures three
VLANs and their names on a switch:

```yaml
- name: Configure VLANs
  hosts: switches
  gather_facts: no
  tasks:
    - name: Ensure VLANs exist
      ios_vlans:
        config:
          - vlan_id: 10
            name: STAFF
          - vlan_id: 20
            name: GUEST
          - vlan_id: 30
            name: VOICE
        state: merged
```

**21.** Run it. Record the changed/ok count.

**22.** Run it **again without changing anything**. Record the changed/ok count.
This is idempotence, and the second run should report zero changes.

**23.** Now change VLAN 20's name **manually on the device** — this is drift.

**24.** Run the playbook with `--check --diff`. Record what it reports without
changing anything.

**25.** Run it for real and confirm the drift is corrected.

**26.** Put the playbook in git. Make a change, commit it, and demonstrate that
`git diff` answers "what changed" — the first question of every incident.

**27.** Write an **imperative** script that does the same thing, run it twice, and
record how its second run differs from the playbook's.

---

### Part 5 — Test before deploying

**28.** Build a model of your lab topology in Containerlab, GNS3 or Packet Tracer.

**29.** Write a change — add a VLAN, or a route, or a firewall rule — and apply it
**to the model first**.

**30.** Verify reachability in the model before and after, and record what the
change did that you did not intend. (Design the change so that it does have an
unintended effect; a change that works is a less useful exercise.)

**31.** Now write a test that would have caught it automatically. Something as
simple as:

```bash
# reachability assertions
ping -c1 -W1 10.1.1.10 || echo "FAIL: server unreachable"
nc -z -w1 10.1.1.10 443 || echo "FAIL: https blocked"
nc -z -w1 10.1.1.10 22 && echo "FAIL: ssh should be blocked"
```

**32.** Run the test suite against the model before and after the change.

---

## Expected observations

- **Step 5:** the capture shows the full stack. The underlay sees ordinary routed
  UDP; the overlay sees a flat Ethernet segment. Neither knows about the other.
- **Step 6: 50 bytes of overhead** for VXLAN over IPv4 — 14 outer Ethernet, 20
  outer IP, 8 UDP, 8 VXLAN.
- **Step 7: the VXLAN interface's MTU is 50 bytes lower** than the underlay's, and
  if it were not, you would have built Lab 10's black hole.
- **Step 11:** a bridge on the host, one veth pair per container with one end in
  the container's namespace, and MASQUERADE for outbound. Every mechanism is from
  Units IV and VII, implemented in software.
- **Step 16: SSH works with no outbound rule**, because security groups are
  stateful.
- **Step 17: SSH fails** with the network ACL in place, because it is stateless and
  the return traffic to your ephemeral port is not permitted. **This is the most
  common cloud networking fault.**
- **Step 19: peering is non-transitive.** A peered to B and B peered to C does not
  give A reachability to C.
- **Step 22: the second playbook run reports zero changes.** Step 27's imperative
  script does not — it either errors or duplicates work.
- **Step 24: `--check --diff` reports the drift without correcting it**, which is
  drift detection for free.

---

## Break it

**A.** Set the VXLAN interface's MTU to match the underlay's (removing the 50-byte
allowance) and transfer a large file across the overlay. Record the failure.

**B.** Change the VNI on one VTEP only. Record what happens and how it appears in
the capture.

**C.** In cloud: remove the route to the internet gateway from a public subnet's
route table without changing anything else. Record how quickly instances lose
Internet access and what the instances themselves report.

**D.** Deploy the playbook to a device whose firmware uses a different command
syntax. Record how the failure manifests, and use it to argue about screen
scraping versus NETCONF.

---

## Debrief

**1.** From your step 5 capture, list every header in order with its size, and give
the total overhead. Then explain why the VTEP must reduce the overlay MTU, and
what happens if it does not — naming the chapter that predicted it.

**2.** Explain why the underlay must be routed and what that buys, referring to the
east–west traffic shift. Then explain why an overlay is nonetheless necessary on a
routed fabric.

**3.** For each of the ten cloud constructs, name the traditional concept it
implements. Then state which two surprised you most in the mapping, and why.

**4.** Explain the security group / network ACL result from steps 16–18 with a
packet walk in both directions. State why this is the most common cloud fault and
what makes it hard to diagnose from inside the instance.

**5.** Your playbook's second run reported zero changes; the imperative script's did
not. Explain idempotence in one sentence, and state what it enables operationally
that a script cannot — specifically, what running the playbook every hour would
give you.

**6.** In step 30 your change had an unintended effect. Describe it, state whether
you would have noticed it in production before a user did, and present the test
from step 31 that catches it. Then estimate what fraction of change-related
outages a test suite like that would prevent, and defend the estimate.

**7.** Draw the container networking diagram from step 11. For each element,
name the chapter of this book that introduced the underlying mechanism. The point
of this question is that Unit XIV introduces almost no new networking — it
introduces new places to put the networking you already know.
