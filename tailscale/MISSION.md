# Mission: Tailscale

## Why

Ben is a self-taught TypeScript / Node.js developer working toward senior-level
mastery, and he runs a mix of machines: a laptop, a home server or Raspberry Pi, a
cloud VM, and containerised services. He is tired of the old ways of reaching them
— fiddling with port forwarding, dynamic IPs, and brittle SSH key sprawl — and keeps
hearing that **Tailscale** makes a private network "just work" between his devices.

He wants to genuinely understand Tailscale, not just install it: what a mesh VPN on
**WireGuard** actually is and why it beats a hub-and-spoke VPN; how **identity**
replaces network trust; the **tailnet** and its `100.x` addresses; **MagicDNS**
names; how **access-control policies** decide who can reach what; how **subnet
routers** and **exit nodes** extend the network to real LANs and the internet; how
**Tailscale SSH** and **Serve/Funnel** expose services; and how to operate it all
(status, relays, key expiry, automation).

This is an independent workspace: it does not reference or link to any other course
in the repository.

## Success looks like

- Explain what Tailscale is: a mesh overlay network built on WireGuard, with
  identity-based access, zero-config traversal of NAT/firewalls, and no central
  gateway — and contrast it with a traditional hub-and-spoke VPN.
- Install the client and bring up a first node, then read `tailscale status` /
  `tailscale ip` and understand the `100.64.0.0/10` tailnet address range.
- Describe the **tailnet**, users, devices, **tags**, device authorization, and
  key expiry — and why identity, not IP, is the security boundary.
- Reach devices by **MagicDNS** name (machine name + `.ts.net` tailnet domain),
  and explain search domains and the FQDN rule for shared devices.
- Read and write **access-control policies**: the huJSON policy file, `src`/`dst`/
  `action`, `tags`, and the facts that ACLs are **deny-by-default, directional, and
  locally enforced** (and that an absent `acls` section means allow-all).
- Set up a **subnet router** (advertise a CIDR, enable the route), understand SNAT
  and the `100.64.0.0/10` return route, and set up an **exit node** to route all
  internet traffic.
- Use **Tailscale SSH** (keyless, identity/ACL-driven, check mode) and understand
  when it is and is not a good fit.
- Use **Serve** to expose a local service to the tailnet and **Funnel** to expose
  it publicly, and know how HTTPS certificates are obtained.
- Operate a tailnet: diagnose direct vs DERP-relayed connections with `netcheck`,
  handle key expiry / fail-close, and automate with auth keys, tags, ephemeral
  nodes, and cloud-init.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped (what a VPN is, what
  NAT does, what an overlay network is) before the Tailscale specifics.
- Node / TypeScript and Linux/CLI mindset; examples read naturally to a developer
  who lives in a terminal.
- Learns by doing — lessons lean on concrete `tailscale` CLI commands and small
  policy snippets.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in cited sources (the official Tailscale documentation);
  claims beyond them are explicit about uncertainty.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- A full WireGuard or cryptography internals tour; WireGuard is covered only as
  far as it explains Tailscale's security and performance.
- A complete enterprise/IAM product tour (PAM, Aperture, ACL GitOps); these are
  mentioned only where they clarify the core model.
- Deep Kubernetes operator configuration beyond a brief mention in the ops lesson.