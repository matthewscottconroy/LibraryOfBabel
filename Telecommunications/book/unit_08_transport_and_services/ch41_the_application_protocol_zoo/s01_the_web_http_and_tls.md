# 41.1 The Web: HTTP and TLS

HTTP is the protocol most people mean when they say "the Internet", it is remarkably
simple, and its evolution from 1991 to now is a compressed history of everything in
Units V–VIII.

## HTTP's shape

**Request, response. That is the protocol.**

```
GET /index.html HTTP/1.1
Host: www.example.com
User-Agent: Mozilla/5.0
Accept: text/html
Connection: keep-alive

```

```
HTTP/1.1 200 OK
Date: Mon, 15 Mar 2024 14:23:01 GMT
Server: nginx
Content-Type: text/html; charset=utf-8
Content-Length: 1270
Cache-Control: max-age=3600

<!doctype html>...
```

Text, line-oriented, with a blank line separating headers from body. You can type it by
hand:

```bash
printf 'GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n' | nc example.com 80
```

Doing that once is worth more than reading about it — it makes concrete that a web
request is a few lines of text over a TCP connection.

### The methods

| Method | Purpose | Safe? | Idempotent? |
|---|---|---|---|
| **GET** | retrieve | **yes** | yes |
| **HEAD** | headers only | yes | yes |
| **POST** | submit; **not idempotent** | no | **no** |
| **PUT** | replace | no | **yes** |
| **DELETE** | remove | no | yes |
| PATCH | partial update | no | no |
| OPTIONS | what is allowed | yes | yes |

"Safe" means no side effects; "idempotent" means repeating it changes nothing further.

**These matter operationally**, not merely theoretically: a proxy may retry an idempotent
request and must not retry a POST, and QUIC's 0-RTT is safe only for idempotent
requests (Chapter 38 §38.4). A POST replayed can charge a card twice.

### The status codes

| Class | Meaning | The ones to know |
|---|---|---|
| **1xx** | informational | 101 Switching Protocols (WebSocket) |
| **2xx** | success | **200 OK**, 201 Created, 204 No Content |
| **3xx** | redirection | **301** permanent, **302** temporary, **304 Not Modified** |
| **4xx** | **client error** | **400**, **401** Unauthorized, **403** Forbidden, **404**, 429 Too Many Requests |
| **5xx** | **server error** | **500**, **502 Bad Gateway**, **503 Unavailable**, **504 Gateway Timeout** |

The 4xx/5xx distinction is the one that matters during an incident:

> **4xx means the client sent something wrong. 5xx means the server failed.** If you are
> seeing 5xx, the fault is yours; if 4xx, it is the request's.

**And three deserve specific note:**

**304 Not Modified** is the caching mechanism working — the client asked "has this changed
since?" and the server said no, sending no body. A high 304 rate is good.

**502 and 504** are **proxy** errors: 502 means the upstream returned something invalid, 504
means it did not answer in time. Both point past the thing you are talking to, which is
why they are the characteristic errors of a load-balanced or reverse-proxied service.

**429** is rate limiting, and it is a deliberate response rather than a fault.

## The evolution

Each version solved the previous one's binding constraint, and the sequence is a good
summary of this unit.

### HTTP/0.9 (1991) — one line

```
GET /index.html
```

No headers, no status codes, no methods but GET. The connection closed after the
response, and the response was the document.

### HTTP/1.0 (1996) — headers

**Headers, status codes, methods, content types.** And one request per TCP connection:

```
   Connect ─ request ─ response ─ close
   Connect ─ request ─ response ─ close      ← for every image, every stylesheet
```

A page with forty resources meant forty TCP connections, each with a handshake
(Chapter 37 §37.1) and each starting in slow start (Chapter 38 §38.2). The transport cost
dominated the content cost entirely.

### HTTP/1.1 (1997) — persistent connections

**`Connection: keep-alive` became the default.** One connection, many requests.

And the `Host` header became mandatory — which is why virtual hosting works. Before
it, one IP address served one website, because the request named only a path. `Host` is
what let the web scale to more sites than there were addresses.

Its remaining problem: head-of-line blocking at the application layer. Requests on a
connection are answered **in order**, so a slow response blocks the ones behind it.

**Pipelining** — sending several requests without waiting — was specified and **failed in
practice**, because a slow first response still blocked the rest and because middleboxes
mishandled it.

So browsers opened six connections per host — not greed, but the only available
parallelism (Chapter 38 §38.1's fairness note).

### HTTP/2 (2015) — multiplexing

| Feature | Effect |
|---|---|
| **Binary framing** | parsing is unambiguous and cheap |
| **Multiplexed streams** | many requests **concurrently on one connection** |
| **HPACK header compression** | headers repeat enormously; this removes it |
| Server push | send resources before they are asked for |
| Stream priorities | tell the server what matters |

Multiplexing removed application-layer head-of-line blocking — and **exposed TCP's**
(Chapter 38 §38.4). One lost packet blocked every stream, so on a lossy path HTTP/2 was
measurably worse than HTTP/1.1's six connections.

Server push was removed from Chrome in 2022. It sounded good and in practice pushed
resources the client already had, wasting bandwidth. A feature that measurement
retired.

### HTTP/3 (2022) — QUIC

**Chapter 38 §38.4's subject.** HTTP semantics unchanged; **the transport replaced.**
Independent streams, 1-RTT or 0-RTT handshake, connection migration, mandatory encryption.

> HTTP's methods and status codes have not changed since 1997. Everything else has been
> replaced twice. The semantics were right; the transport was the problem.

## TLS

**Transport Layer Security** — encryption, integrity and authentication between TCP and the
application.

### What it provides

| Property | Mechanism |
|---|---|
| **Confidentiality** | symmetric encryption (AES-GCM, ChaCha20-Poly1305) |
| **Integrity** | authenticated encryption — tampering is detected |
| **Authentication** | **the server proves its identity with a certificate** |
| Forward secrecy | ephemeral keys, so a stolen long-term key does not decrypt past traffic |

**The third is the one people underestimate.** Encryption without authentication protects
you from a passive observer and not from an active one — you would have a private
conversation with an attacker. The certificate is what makes it a conversation with the
right party.

### The handshake

**TLS 1.2** — two round trips:

```
   Client ── ClientHello ────────────────────▶      (versions, ciphers, random)
          ◀── ServerHello, Certificate, ─────       (chosen cipher, cert, key share)
              ServerKeyExchange, Done
          ── ClientKeyExchange, ─────────────▶
             ChangeCipherSpec, Finished
          ◀── ChangeCipherSpec, Finished ────
          ═══ application data ═══
```

**TLS 1.3 (2018)** — **one round trip**, and it is a substantial redesign:

```
   Client ── ClientHello + key share ────────▶
          ◀── ServerHello + key share, ──────
              {Certificate, Finished}          ← already encrypted
          ── {Finished} ────────────────────▶
          ═══ application data ═══
```

The client guesses the key exchange and sends its share immediately, so the server can
complete the exchange in one message. And everything after ServerHello is encrypted,
including the certificate — which was visible in 1.2.

TLS 1.3 also removed things, and the removals are the security improvement:

| Removed | Why |
|---|---|
| **RSA key exchange** | no forward secrecy |
| **CBC mode ciphers** | a decade of padding-oracle attacks |
| **RC4, 3DES, MD5, SHA-1** | broken |
| **Compression** | CRIME attack |
| Renegotiation | a source of vulnerabilities |

> **TLS 1.3's design principle was to remove options.** Every negotiable parameter is an
> opportunity for downgrade, and the protocol's history — BEAST, CRIME, POODLE, FREAK,
> Logjam — is largely a history of attacks that forced a connection back to a weak option.

**Combined with QUIC** (Chapter 38 §38.4), the handshake merges with the transport's and
costs one round trip total, or zero on resumption.

### Certificates and the trust model

A certificate binds a name to a public key, signed by a Certificate Authority.

Your browser trusts perhaps 150 root CAs, and any of them can issue a certificate for
any name.

> The trust model's weakness is that it is a logical OR: the security of every site is
> the security of the *weakest* CA your client trusts.

**And this has failed in practice:** DigiNotar (2011) was compromised and issued fraudulent
certificates for Google, used against Iranian users; the CA was destroyed by the incident.

**The mitigations:**

**Certificate Transparency** (RFC 6962) — every certificate must be logged publicly, in
append-only logs, and browsers reject unlogged ones. A CA can still issue a fraudulent
certificate; it cannot do so secretly, and domain owners monitor the logs for their own
names.

**CAA records** (DNS, Chapter 39 §39.3) — a domain declares which CAs may issue for it,
and compliant CAs check.

**Short lifetimes** — certificates were once valid for years; the maximum is now around
one year and falling, and 90 days is normal because Let's Encrypt made automation the
default.

Let's Encrypt (2016) changed the web, and the mechanism is worth noting: free
certificates, issued automatically via ACME, using a DNS or HTTP challenge to prove
domain control (Chapter 39 §39.3's TXT records). HTTPS went from roughly 30% of page
loads to over 95% in under a decade — because the barrier had been cost and effort, not
belief.

## Where this touches the network

Three practical consequences for anyone operating a network:

**SNI — Server Name Indication.** TLS's extension letting a client say which host it
wants before the certificate is chosen, so one address can serve many HTTPS sites.

And in TLS 1.2 the SNI is plaintext — visible to anyone on the path. Which is what
network filtering and monitoring use to identify destinations, and what Encrypted Client
Hello (ECH) now conceals. Chapter 61 returns to the tension.

Certificate expiry is a scheduled outage nobody scheduled. Chapter 22 §22.4's
"everything worked yesterday and nothing changed" fault. **Monitor expiry dates**; it is the
cheapest possible check.

**TLS inspection.** An enterprise middlebox decrypting traffic must install its own root CA
on every client and issue certificates on the fly. It works, it breaks certificate
pinning, and it makes the inspection device the single most security-critical thing on the
network — because it holds a key that can impersonate any site.

## What breaks here

**A certificate error after everything worked.** Expiry. Check the date first.

HTTPS working in a browser and failing from a script. The script does not have the
enterprise root CA, or does not send SNI.

**502 or 504 from a proxy.** The problem is upstream of what you are talking to.

**HTTP/2 slower than HTTP/1.1.** A lossy path, and TCP head-of-line blocking. HTTP/3 fixes
it.

A site unreachable only from the corporate network. TLS inspection failing, or SNI-based
filtering.

**Mixed content warnings.** An HTTPS page loading HTTP resources.

> **Network+ note.** Objective 1.4 expects HTTP (80) and HTTPS (443); objective 4.4 expects
> TLS. Over-learn: **HTTP is 80 and HTTPS is 443**; the status code classes — 2xx
> success, 3xx redirect, 4xx client error, 5xx server error; **TLS provides
> confidentiality, integrity and authentication**; and a certificate binds a name to a
> key and is signed by a CA. The 4xx/5xx distinction appears in troubleshooting
> scenarios.
