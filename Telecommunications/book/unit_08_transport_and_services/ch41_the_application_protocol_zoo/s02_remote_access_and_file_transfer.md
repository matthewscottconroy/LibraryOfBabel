# 41.2 Remote Access and File Transfer

The protocols for reaching a machine and moving files to it. Nearly all of them exist in
a secure version and an insecure predecessor, and the insecure ones are still deployed —
which makes this section as much about migration as about protocols.

## The plaintext era

Telnet (23), FTP (20/21), rsh, rlogin, TFTP (69).

All of them send credentials in clear text. In 1980 this was reasonable: the network was
a few hundred machines administered by colleagues, and physical access to the cable was the
trust boundary (Chapter 18's notes on ARP make the same point).

By 1995 it was not, and the demonstration was public: packet sniffers on shared
Ethernet segments (Chapter 17 §17.1) collected passwords by the thousand, and the tooling
was freely available.

> **The protocols did not become insecure. The context did.**

**And they persist**, in three places: network equipment with no alternative, industrial
systems that cannot be updated, and internal networks where "it is only internal" is
offered as a reason. Chapter 62 addresses why the third is wrong.

## SSH — port 22

**Tatu Ylönen, 1995**, written in response to a password-sniffing attack on his own
university network. SSH replaced Telnet, rsh, rlogin and FTP essentially completely
within a decade, which is unusually fast.

**What it provides:**

| | |
|---|---|
| **Encryption** | of everything, including the password |
| **Server authentication** | **host keys** — you verify the server, not only the reverse |
| **Client authentication** | password, **public key**, keyboard-interactive, certificates |
| **Integrity** | tampering is detected |
| **Multiplexing** | shells, file transfer and forwarded ports over one connection |

### Host keys and TOFU

**The first connection shows this:**

```
The authenticity of host 'server (203.0.113.10)' can't be established.
ED25519 key fingerprint is SHA256:abc123...
Are you sure you want to continue connecting (yes/no)?
```

This is the trust decision, and almost everyone types `yes` without checking.

The model is TOFU — Trust On First Use. You accept the key once, it is stored in
`~/.ssh/known_hosts`, and any later change produces a loud warning.

So SSH protects you against a man-in-the-middle appearing *later*, and not against one
present on your first connection. Verifying the fingerprint out of band is what closes
that gap, and SSHFP records in DNS (with DNSSEC) or **SSH certificates** are the
scalable versions.

**And the warning matters:**

```
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@    WARNING: REMOTE HOST IDENTIFICATION HAS CHANGED!     @
```

It usually means a server was rebuilt. It might mean an attack. Deleting the line
without finding out which is the reflex to resist.

### Public key authentication

Better than passwords in every respect, and it should be the default:

```bash
ssh-keygen -t ed25519 -C "you@example.com"
ssh-copy-id user@server
```

The private key never leaves your machine; the server holds only the public half.
Nothing guessable is transmitted, so brute-forcing is impossible and a compromised
server does not yield a reusable credential.

Ed25519 is the current recommendation — small, fast, and with no parameter choices to
get wrong. RSA remains fine at 3072 bits or above; DSA is obsolete and ECDSA is
acceptable but has awkward curve questions.

### What SSH carries besides shells

The multiplexing is underused and is genuinely powerful:

```bash
# File transfer, over the same protocol
scp file user@server:/path/
sftp user@server
rsync -avz -e ssh dir/ user@server:/path/

# Port forwarding — local port to a remote destination
ssh -L 8080:internal.example.com:80 user@bastion
# now http://localhost:8080 reaches internal.example.com:80

# Reverse forwarding — expose a local service to the remote side
ssh -R 9000:localhost:3000 user@server

# SOCKS proxy — a general-purpose tunnel
ssh -D 1080 user@bastion

# Run a command without a shell
ssh user@server 'systemctl status nginx'

# Reuse one connection for many sessions
ssh -o ControlMaster=auto -o ControlPath=~/.ssh/cm-%r@%h:%p user@server
```

Port forwarding turns SSH into a VPN for one service, and it is how a great deal of
administrative access to internal systems actually happens.

**And it is a security consideration:** a user with shell access can forward **any** port,
inbound or outbound. `AllowTcpForwarding no` and `PermitOpen` restrict it, and a bastion
host that permits arbitrary forwarding is a bastion in name only.

### Hardening

```
# /etc/ssh/sshd_config
PermitRootLogin no
PasswordAuthentication no          # keys only
PubkeyAuthentication yes
AllowUsers alice bob
MaxAuthTries 3
X11Forwarding no
AllowTcpForwarding no              # unless needed
ClientAliveInterval 300
Protocol 2                         # SSHv1 is broken; modern builds omit it entirely
```

**`PasswordAuthentication no` is the single highest-value line.** It eliminates brute
forcing entirely, and the automated attempts against port 22 on any public address are
continuous — a new server sees them within minutes.

Changing the port to 2222 reduces log noise substantially and **provides no security**
(Chapter 35 §35.3). Both facts are worth stating together.

## FTP — 20 and 21

Chapter 33 §33.3 covered why it breaks across NAT. Here is the protocol.

**Two connections:** a **control** connection on 21 carrying commands, and a **separate data
connection** for each transfer or listing.

**Active mode** — the server connects **back** to the client:

```
   Client → Server (21):  PORT 192,168,1,10,20,100
   Server → Client (from 20):  connects to 192.168.1.10:5220
```

Fails behind NAT and behind any client-side firewall, because it is an inbound
connection to the client.

**Passive mode** — the client connects to the server for data too:

```
   Client → Server (21):  PASV
   Server → Client:       227 Entering Passive Mode (203,0,113,10,195,80)
   Client → Server:       connects to 203.0.113.10:50000
```

Works for clients behind NAT, and requires the **server** to permit a range of inbound
ports — which is why FTP servers are awkward to firewall.

Passive is the default in every modern client.

### The secure variants — and they are not the same thing

**A genuine and common confusion:**

| | Transport | Ports | Relationship |
|---|---|---|---|
| **FTPS** | **FTP over TLS** | 21 (+ data range), or 990 | **FTP with encryption added** |
| **SFTP** | **a subsystem of SSH** | **22** | **not FTP at all** |

> SFTP is not "secure FTP". It is a file-transfer protocol that runs inside SSH and
> shares no code, no design and no ports with FTP.

**Prefer SFTP**, and the reasons are practical:

- **One port (22)**, so it is trivially firewalled
- No separate data connection, so no NAT traversal problem
- The application-layer gateway problem disappears (Chapter 33 §33.3) — FTPS breaks
  ALGs because the control channel is encrypted, so a NAT cannot rewrite the embedded
  addresses
- Authentication and host verification are SSH's

FTPS is appropriate when a counterparty requires it, and not otherwise.

## TFTP — 69

Chapter 36 §36.3 explained why it uses UDP. Its properties:

**No authentication of any kind.** No username, no password, nothing.

**Lock-step acknowledgement**, so throughput is bounded by round-trip time regardless of
link speed — about 25 KB/s on a 20 ms path.

And it is still essential, for one reason: it fits in a boot ROM. PXE network boot,
switch and router firmware loading, and IP phone provisioning all use it because the client
is a few kilobytes of code with no operating system.

Never expose it beyond the segment that needs it. A TFTP server with configuration
backups on it is a configuration disclosure waiting to happen, and there is no
authentication to prevent it.

## Remote desktop

| Protocol | Port | Notes |
|---|---|---|
| **RDP** | **3389/TCP** | Microsoft; encrypted, and **its exposure history is severe** |
| VNC / RFB | 5900+ | simple, **often unencrypted — tunnel it** |
| X11 forwarding | via SSH 22 | `ssh -X`; **`-Y` disables protections, so prefer `-X`** |

**RDP deserves a specific warning.** Exposed RDP is among the most common initial access
vectors for ransomware, and internet-wide scanning for port 3389 is continuous.

**BlueKeep (CVE-2019-0708)** was a wormable pre-authentication vulnerability in RDP, and the
response — Microsoft issuing patches for **out-of-support** Windows versions — indicates how
serious it was judged to be.

> **RDP should never be directly exposed to the Internet.** Put it behind a VPN, a bastion,
> or an RD Gateway, and require multi-factor authentication.

## The migration table

What to use instead of what, which is the practical content of this section:

| Insecure | Port | Use instead | Port |
|---|---|---|---|
| **Telnet** | 23 | **SSH** | 22 |
| **FTP** | 21 | **SFTP** | 22 |
| rsh / rlogin / rcp | 513/514 | SSH / scp / rsync-over-ssh | 22 |
| **HTTP** (for anything with credentials) | 80 | **HTTPS** | 443 |
| **TFTP** | 69 | SFTP where possible; **restrict where not** | 22 |
| VNC unencrypted | 5900 | **VNC over SSH tunnel**, or RDP with TLS | 22 / 3389 |
| SNMPv1/v2c | 161 | **SNMPv3** | 161 |
| LDAP | 389 | **LDAPS or StartTLS** | 636 / 389 |

**The pattern is clear and worth stating:** almost every plaintext protocol has a
drop-in-ish replacement, and the barrier is operational inertia rather than technical
difficulty.

## What breaks here

FTP working in one mode and not the other. Active versus passive, and which side is
behind NAT.

**FTPS failing where FTP worked.** The ALG cannot parse an encrypted control channel.

"Remote host identification has changed." Usually a rebuilt server. Find out which
before deleting the line.

SSH accepting a password when you configured keys. `PasswordAuthentication` is still
`yes`.

**A file transfer over TFTP taking forever.** Lock-step, bounded by RTT.

**RDP exposed and the machine compromised.** This is the expected outcome, not bad luck.

**X11 forwarding not working.** `X11Forwarding no` on the server, or no `$DISPLAY`.

> **Network+ note.** Objective 1.4 expects these ports, and **they are examined directly**:
> SSH 22, Telnet 23, FTP 20/21, TFTP 69, RDP 3389, VNC 5900. Objective 4.4 expects
> secure alternatives to insecure protocols. Over-learn the migration table, and
> **especially that SFTP is not FTPS** — that distinction is examined and is commonly
> confused.
