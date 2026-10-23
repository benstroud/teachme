# Twilio API — Resources

Curated, high-trust sources. Knowledge in lessons is drawn from these; verify any
assertion against them before teaching. All are official Twilio docs/support (plus
the official Node helper library repo).

## Identity, accounts & tokens

- [Access Tokens](https://www.twilio.com/docs/iam/access-tokens) — short-lived JWTs
  for client SDKs; `identity`, grants (`VoiceGrant`), `outgoingApplicationSid`, and
  lifetime (up to 24h; use the shortest feasible).
- [Subaccounts API](https://www.twilio.com/docs/iam/api/subaccounts) — subaccounts
  owned by the main account; 1,000 default limit; closed subaccounts deleted after
  30 days; billing rolls up to the main account.

## Voice client & TwiML

- [Voice SDK overview](https://twilio.com/docs/voice/client) — how the browser
  connects and why Twilio then requests the TwiML URL.
- [TwiML `<Dial>`](https://www.twilio.com/docs/voice/twiml/dial) — dialing `To` with
  `callerId`; which caller IDs Twilio allows.

## Webhook security

- [Secure webhooks](https://www.twilio.com/docs/usage/webhooks/webhooks-security) —
  HTTPS, `X-Twilio-Signature`, exact-URL and form-parameter rules, JSON
  `bodySHA256` + `validateRequestWithBody`, and per-framework tutorials.

## Phone numbers

- [AvailablePhoneNumber](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource)
  and [AvailablePhoneNumberLocal](https://www.twilio.com/docs/phone-numbers/api/availablephonenumberlocal-resource)
  — searching numbers to buy.
- [IncomingPhoneNumber](https://www.twilio.com/docs/phone-numbers/api/incomingphonenumber-resource)
  — purchasing/updating a number (e.g. setting `voiceApplicationSid`).

## Limits & compliance

- [CPS and concurrent call limits](https://support.twilio.com/hc/en-us/articles/40868960307995-Understanding-CPS-and-Concurrent-Call-Limits-in-Twilio)
  — ~1 CPS default with queuing, concurrent limits, 4-hour max call.
- [US voice guidelines](https://www.twilio.com/en-us/guidelines/us/voice) and
  [High-volume voice considerations](https://www.twilio.com/en-us/blog/high-volume-voice-considerations)
  — trusted calling (SHAKEN/STIR) and outbound rules.

## Library

- [twilio-node on GitHub](https://github.com/twilio/twilio-node) — the Node helper
  library (the app pins 5.5.1).
- [Node migration guide](https://www.twilio.com/docs/libraries/node/migration-guide) —
  v5→v6 raises the minimum Node.js to 20 (no breaking API changes otherwise).

## Notes on reliability

- Twilio docs are versioned by product; API/SDK behavior can change across majors.
  Cite the current page rather than assuming an old version's behavior.
- v6 of the Node helper requires Node.js 20; the app's Dockerfile already uses
  Node 20, but confirm before bumping the dependency.
- Signature validation is easy to get subtly wrong (URL encoding, form params, JSON
  bodies). Point people at the SDK helpers rather than hand-rolling.