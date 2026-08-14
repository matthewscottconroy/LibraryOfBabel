# 69.4 Cloud-Native Load Balancing and Service Mesh

Load balancing is Chapter 52 §52.4's material with a control plane, and the service mesh is
what happens when the same functions move into the application's own network path.

## The load balancer taxonomy

Three kinds, and the distinction is which layer they terminate at.

| | **Layer 4** | **Layer 7** |
|---|---|---|
| Decides on | **five-tuple** (Chapter 35 §35.3) | **URL, headers, cookies, method** |
| Terminates | **the TCP connection, or forwards it** | **the connection, always** |
| Sees | **addresses and ports** | **the request** |
| **TLS** | **passes through, or terminates** | **terminates, usually** |
| Cost per connection | **low** | **higher** |
| **Can do** | **DSR, extreme throughput, any protocol** | **path routing, header rewriting, WAF, retries** |

**And the cloud providers' names for them:**

| | AWS | Azure | GCP |
|---|---|---|---|
| **L4** | Network Load Balancer | Load Balancer | TCP/UDP Load Balancing |
| **L7** | Application Load Balancer | Application Gateway | HTTP(S) Load Balancing |
| **Global anycast** | Global Accelerator | Front Door | Global Load Balancing |

The third row is the one worth understanding, because it is Chapter 52 §52.4's anycast sold
as a product:

> **A global load balancer advertises one anycast address from every edge location.** The user
> reaches the nearest edge by BGP, the edge terminates TLS, and the request travels to the
> origin over the provider's own backbone — which is faster and more reliable than the
> public Internet path (Chapter 48 §48.1).

Which is a CDN's architecture applied to dynamic traffic, and it is why the distinction
between a CDN and a global load balancer has largely dissolved.

## Health checks, which determine everything

A load balancer's behaviour is entirely determined by what it considers healthy, and this is
where the failures are.

| Check type | Tests | Misses |
|---|---|---|
| **TCP connect** | **the port is open** | **the application is broken** |
| **HTTP status** | **the application responds** | **it responds wrongly** |
| **A dedicated health endpoint** | **whatever the endpoint checks** | **whatever it does not** |

> A health endpoint that returns 200 unconditionally is a health check that never fails, and
> one that checks every downstream dependency is a health check that fails when any of them
> does — taking a healthy instance out of service because a non-critical dependency is
> degraded.

**The defensible design is two endpoints:**

| | Checks | Used by |
|---|---|---|
| **Liveness** | **is this process alive and should it be restarted?** | the orchestrator |
| **Readiness** | **can this instance serve requests right now?** | **the load balancer** |

And readiness should check what the instance needs and not what it merely uses — its own
database connection pool, yes; a recommendation service it can degrade without, no.

**Two tuning traps:**

**Too aggressive.** A two-second interval with a one-failure threshold removes an instance
during a garbage collection pause, and the resulting flapping is worse than the problem.

**Too slow.** A thirty-second interval with three failures means ninety seconds of requests
sent to a dead instance.

> **And the interaction with deployment matters:** a readiness check that passes before the
> application is warm sends traffic to an instance that will time out, which is the commonest
> cause of errors during a rolling deployment.

## Connection draining and deployment

The mechanism that makes a deployment invisible, and its absence is why deployments cause
errors.

```
   Without draining:              With draining:
   
   Instance marked unhealthy      Instance marked draining
   Connections terminated         New connections stop
   In-flight requests fail        In-flight requests complete
                                  Instance removed after a timeout
```

And the timeout must exceed the longest legitimate request, which for an application with a
five-minute report generation means a five-minute drain — and nobody configures that until it
breaks.

## Ingress, and the Kubernetes case

Chapter 67 §67.1's Service gives a cluster-internal virtual address. Reaching it from
outside needs something more.

| Mechanism | Is |
|---|---|
| **NodePort** | **a port on every node** — crude, and it works |
| **LoadBalancer** | **the cloud provider creates an L4 load balancer** — one per service, and it is expensive |
| **Ingress** | **an L7 router inside the cluster**, behind one load balancer |
| **Gateway API** | **Ingress's successor** — richer, and role-separated |

The Ingress model is the one to understand:

> One cloud load balancer, one ingress controller inside the cluster, and HTTP routing rules
> that direct paths and hostnames to services. **Which collapses fifty per-service load
> balancers into one**, and moves the routing into a configuration object the platform team
> owns.

And the Gateway API's contribution is role separation — the infrastructure team defines
the Gateway, the application team defines the Routes — which is the same argument as
Chapter 59 §59.3's separation of duties, expressed in an API.

## The service mesh

What happens when load balancing, retries, TLS, telemetry and policy move into the request
path of every service.

```
   Without a mesh:                    With a sidecar mesh:

   Service A ────────▶ Service B      Service A ─▶ [proxy] ══mTLS══▶ [proxy] ─▶ Service B
                                                      │                │
   Retries, TLS, timeouts,                            └──── control ───┘
   metrics: in each application's                          plane
   code, in each language
                                      All of it in the proxy, uniformly
```

**What it provides:**

| | |
|---|---|
| **Mutual TLS between every service** | **automatically, with rotating certificates** (Chapter 58 §58.4) |
| **Retries, timeouts and circuit breaking** | **uniformly, not per language** |
| **Load balancing** | **client-side, with real health information** |
| **Traffic splitting** | **canary and blue-green deployments, by percentage** |
| **Telemetry** | **every request, every service, uniformly** |
| **Policy** | **which service may call which** — Chapter 59 §59.4's per-application access |

> **The strongest argument is the uniformity.** An organisation with services in six languages
> has six retry implementations, six TLS configurations and six metrics conventions. **A mesh
> has one**, and it is operated by the platform team rather than reimplemented by each
> application team.

And the mutual TLS is the security argument (Chapter 59 §59.4): every service call is
authenticated and encrypted, with identity per workload, without any application changing.

### What it costs

**Four things, and they are frequently underestimated.**

**Latency.** Two extra proxy hops per request — typically 0.5–1 ms each. Which is
negligible for one call and is not for twelve:

| Sequential service calls | **Added latency** |
|---|---|
| 1 | ~1.5 ms |
| 5 | **~7.5 ms** |
| **12** | **~18 ms** |

**Resources.** A sidecar per pod — memory and CPU, multiplied by the pod count — and
in a large cluster this is a measurable fraction of the capacity.

**Complexity.** A control plane, a data plane, a certificate authority, and a new class of
failure that presents as an application error.

**And a debugging boundary.** A request that fails may have failed in the application, in its
sidecar, in the network, in the remote sidecar or in the remote application — and the mesh's
own telemetry is the only thing that distinguishes them, which is a dependency on the thing
that may be broken.

### When it is worth it

| Worth it | Not worth it |
|---|---|
| **Many services, several languages** | **a dozen services in one language** |
| **mTLS required between services** | **a trusted network and a compliance regime that accepts it** |
| **Sophisticated deployment patterns needed** | **rolling deployments suffice** |
| **A platform team to operate it** | **no one to own it** |

> A mesh deployed without a team to operate it is a mesh that will be blamed for every
> application fault and understood by nobody — and "we have twelve services and we installed
> Istio" is a recognisable and expensive mistake.

**And the alternative is worth naming:** **a library.** gRPC, or a well-chosen HTTP client with
retries and mTLS, gives most of the benefit with none of the sidecar's cost — at the price
of the uniformity the mesh exists to provide, which is exactly the trade.

**The sidecar-free direction is also real:** ambient mesh, eBPF-based implementations
(Cilium) — moving the mesh's functions into the node rather than into every pod, which
removes the per-pod resource cost and much of the latency. It is the current direction and it
is not yet the default.

## What breaks here

**Instances removed during a garbage collection pause.** Health checks too aggressive.

Errors for ninety seconds after an instance dies. Health checks too slow.

A healthy instance removed because a non-critical dependency was degraded. The readiness
check checks too much.

**Errors during every deployment.** No connection draining, or a drain timeout shorter than the
longest request — or readiness passing before the application is warm.

**Fifty cloud load balancers for fifty services.** **`LoadBalancer` per service.** An ingress
controller.

A mesh adding 18 ms to a request. Twelve sequential calls, two hops each. The
architecture, not the mesh.

A cluster where sidecars consume 20% of the capacity. **One per pod**, and it is arithmetic
rather than a fault.

A request failure that cannot be attributed to a layer. **Five candidates**, and the mesh's
telemetry is the only discriminator.

Istio installed for twelve services in one language. The cost without the benefit.

> **Network+ note.** Objective 1.8 and 3.3 touch load balancing. Over-learn: **a load balancer
> distributes traffic across servers**; Layer 4 balances on addresses and ports and Layer 7 on
> application content; **health checks determine which servers receive traffic**; **session
> persistence keeps a client on one server**; and global load balancing directs users to the
> nearest site. The L4/L7 distinction and the role of health checks are both examined.
