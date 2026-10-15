# Notes — Tailscale workspace

## Working notes (author/maintainer)

- Dark mode everywhere: deep slate paper, light ink, emerald accent (a
  "connected" green). Print query flips back to light so pages print cleanly.
- Learner: self-taught TS/Node dev, senior-track, comfortable in a terminal. Fill
  foundational gaps (what a VPN is, what NAT/firewalls do, what an overlay network
  is) before Tailscale specifics.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to a Tailscale doc. Never
  assert behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.

## Facts verified during authoring (2026-09)

- Tailscale is a mesh VPN built on the WireGuard protocol. It is identity-based and
  zero-config: connections work across NAT and firewalls without port forwarding or
  manual firewall rules. It uses end-to-end encryption and a zero-trust design, and
  its distributed architecture avoids the bottlenecks of traditional VPNs.
  (docs: What is Tailscale)
- A network of devices is a "tailnet". ACLs (and the newer "grants") are defined in
  a declarative tailnet policy file using huJSON. ACLs are deny-by-default,
  directional, and locally enforced on each device; an ACL rule has an `action`
  (accept), `src`, and `dst`. If no `acls` section is present, Tailscale applies a
  default allow-all policy; an empty `acls` object denies all. Rules can target
  users, groups, autogroups, tags, and CIDR ranges. (docs: Manage ACLs)
- Subnet routers advertise a route (e.g. `tailscale up --advertise-routes=10.0.0.0/24`);
  the route must be enabled/approved in the admin console. Tailscale performs SNAT
  (masquerading) by default; it can be disabled on Linux
  (`--snat-subnet-routes=false`), in which case devices need a return route for the
  `100.64.0.0/10` range pointing at the router's LAN IP. (docs: Subnet routers)
- MagicDNS gives each machine a name; the fully qualified domain name is the machine
  name plus the tailnet DNS name (which ends in `.ts.net`). MagicDNS adds search
  domains so you can reach a device by machine name alone; devices shared with you
  require the full domain name. It can be disabled per device with
  `tailscale set --accept-dns=false`. (docs: MagicDNS)
- Tailscale SSH removes key management: authentication and authorization use your
  identity and the tailnet access policies, enforced server-side. It assumes port
  22. Check mode can require SSO approval (e.g. `checkPeriod` of always). Any OS
  user on the client can connect, scoped by the server-side policy. (docs:
  Tailscale SSH)
- Tailscale Serve exposes a local service to your tailnet (with HTTPS); Funnel
  exposes it to the public internet. (docs: tailscale serve / Funnel)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.

## Facts to verify before teaching

- ACL vs "grants" syntax and the visual policy editor evolve; teach the model and
  cite the current guide.
- Plan-gated features (autogroups, CIDR ranges in rules, Services) differ by plan;
  flag availability rather than asserting.