# Tailscale — Resources

Curated, high-trust sources. Knowledge in lessons is drawn from these; verify any
assertion against them before teaching. All are the official Tailscale docs.

## Core concepts

- [What is Tailscale?](https://tailscale.com/kb/1151/what-is-tailscale) — mesh VPN
  on WireGuard, identity-based, zero-config across NAT/firewalls, end-to-end
  encryption, zero-trust, the tailnet, distributed (no-bottleneck) architecture.
- [How Tailscale works](https://tailscale.com/blog/how-tailscale-works) — the
  coordination server, DERP relays, and direct vs relayed connections.

## Getting connected

- [Install Tailscale](https://tailscale.com/kb/1017/install) — clients, `tailscale
  up`, and bringing devices into a tailnet.
- [Set up MagicDNS](https://tailscale.com/kb/1081/magicdns) — machine names,
  `.ts.net` tailnet domain, FQDNs, and search domains.

## Access control

- [Manage permissions using ACLs](https://tailscale.com/kb/1018/acls) — the tailnet
  policy file (huJSON), `src`/`dst`/`action`, tags, autogroups; deny-by-default,
  directional, locally enforced; absent `acls` = allow all.
- [Tailnet policy file syntax](https://tailscale.com/kb/1337/policy-syntax) — the
  full declarative reference.

## Extending the network

- [Subnet routers](https://tailscale.com/kb/1019/subnets) — advertise a CIDR, enable
  routes, SNAT, and the `100.64.0.0/10` return route.
- [Exit nodes](https://tailscale.com/kb/1103/exit-nodes) — route all internet
  traffic through a node.

## Servers, SSH, services

- [Tailscale SSH](https://tailscale.com/kb/1193/tailscale-ssh) — keyless SSH using
  identity and access policies; check mode.
- [Tailscale Serve](https://tailscale.com/kb/1242/tailscale-serve) — serve a local
  service to your tailnet.
- [Tailscale Funnel](https://tailscale.com/kb/1223/funnel) — expose a service to the
  public internet.
- [HTTPS certificates](https://tailscale.com/kb/1153/enabling-https) — how Tailscale
  issues TLS certs for `*.ts.net` names.

## Operating

- [tailscale netcheck / status](https://tailscale.com/kb/1082/troubleshooting) —
  diagnosing connectivity.
- [Auth keys](https://tailscale.com/kb/1085/auth-keys) and
  [ephemeral nodes](https://tailscale.com/kb/1111/ephemeral-nodes) — automation.

## Notes on reliability

- Tailscale's product surface changes often (new ACL "grants", Services, PAM,
  Aperture). Teach the durable model and cite the current guide for specifics.
- The Tailscale CGNAT address range is `100.64.0.0/10`.
- If a deep docs URL 404s in a fetch tool, open it in a browser — the canonical
  paths above resolve.