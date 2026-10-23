# Mission: Twilio API Development (Programmable Voice in Node/TypeScript)

## Why

Ben is a self-taught TypeScript / Node.js developer aiming for senior mastery,
driven by career growth and interviews. He is building a multi-tenant product
where each customer organization gets its **Twilio subaccount**, and users make
**browser-to-phone calls** through the Twilio Voice SDK: the backend issues a
short-lived **Access Token** with a **VoiceGrant**, the browser connects, and
Twilio calls back to the app's **`/twiml`** route, which returns a `<Dial>` to the
destination. The service pins **twilio 5.5.1** and runs on **Node 20**.

He wants to go from "it works in the happy path" to genuinely understanding the
Twilio voice stack: how access tokens and grants authorize a client, how the call
flow actually routes through TwiML, how to secure the webhook with
`X-Twilio-Signature`, how subaccounts and phone-number purchase work, what the
account's call-rate and duration limits are, and what the v5→v6 library upgrade
requires.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain Twilio's account model: a main account with **subaccounts** (one per
  organization), the 1,000-subaccount default limit, the 30-day delete-after-close
  behavior, and usage rolling up to the main account.
- Use the **Node helper library** in TypeScript: initialize the client, prefer API
  Keys, and understand version pinning (5.5.1) and that **v6 requires Node 20**.
- Issue **Access Tokens** correctly: a short-lived JWT with an `identity` and a
  `VoiceGrant` whose `outgoingApplicationSid` points at a TwiML application.
- Explain the **browser Voice SDK** flow: token → connect → Twilio fetches `/twiml`
  for instructions, and note the current SDK major version.
- Write TwiML with **`<Dial>`**: dial `To` with `callerId` set to an owned number,
  and know which caller IDs Twilio permits.
- Secure webhooks: verify the **`X-Twilio-Signature`** with the SDK's validation
  function, using the exact URL and form parameters — and recognize when a handler
  (like the current one) is missing that check.
- Provision numbers: **search** `AvailablePhoneNumber` (local/toll-free), then
  **purchase** an `IncomingPhoneNumber` and set `voiceApplicationSid`.
- Reason about **limits and compliance**: ~1 CPS default per account/subaccount with
  queuing, the 4-hour maximum call, SHAKEN/STIR trusted calling, and US voice rules.
- Harden and scale the integration: validation, idempotency, retries, subaccount
  lifecycle, observability, and a clean upgrade path.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped (what a webhook is,
  what a JWT is, what tenancy means) before Twilio specifics.
- Node / TypeScript mindset; examples read naturally to a backend developer.
- Learns by doing — lessons use concrete Node/TS snippets, TwiML, and request shapes.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in cited sources (the official Twilio docs and support
  articles); claims beyond them are explicit about uncertainty.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- Messaging (SMS/WhatsApp) and other products beyond what the voice flow needs.
- A full TwiML language tour; only the elements used by an outbound `<Dial>` flow.
- Carrier/telecom regulatory filings beyond a practical compliance overview.