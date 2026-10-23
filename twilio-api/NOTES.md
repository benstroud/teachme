# Notes — Twilio API workspace

## Working notes (author/maintainer)

- Dark mode everywhere: deep slate paper, light ink, Twilio-red accent. Print query
  flips back to light so pages print cleanly.
- Learner: self-taught TS/Node dev, senior-track, building a multi-tenant voice app.
  Fill foundational gaps (webhook, JWT, tenancy, webhook signature) before Twilio
  specifics.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to a Twilio doc. Never assert
  behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.

## Facts verified during authoring (2026-09)

- Access Tokens are short-lived JWTs used to authenticate client-side Twilio SDKs
  (Voice, Conversations, Sync, Video). They are created on the server, carry an
  `identity`, and can include grants (e.g. `VoiceGrant`). Lifetimes are limited and
  configurable up to 24 hours; the docs recommend the shortest feasible lifetime.
  Deleting the API key used to sign tokens revokes them. (Access Tokens docs)
- A `VoiceGrant` carries an `outgoingApplicationSid` that points at a TwiML
  application; the Voice SDK uses it to ask the app what to do with the call.
- Webhooks are authenticated with an `X-Twilio-Signature` header. Validation must
  use the exact URL Twilio requested (including URL-encoded characters — decoding or
  re-encoding breaks validation) and the POST form parameters
  (application/x-www-form-urlencoded) sorted alphabetically. For JSON bodies, Twilio
  appends `bodySHA256`; use `validateRequestWithBody` with the raw body. (Secure
  webhooks docs)
- Subaccounts are accounts owned by the main account, used to segment customers. A
  main account can have up to 1,000 subaccounts by default. Twilio deletes closed
  subaccounts 30 days after closure by default. Billing rolls up to the main
  account; suspending the main account suspends subaccounts. Closing a subaccount
  releases its phone numbers (transfer them first). (Subaccounts docs)
- Node helper library v6 (5.x→6.x, 2026-04-20) has no breaking API changes but raises
  the minimum Node.js version to 20; upgrade Node before updating the library. (Node
  migration guide)
- Twilio's default call rate is about 1 CPS per account/subaccount, with requests
  above the limit queued; a call can last at most 4 hours. (Support: CPS and
  concurrent call limits)
- Trusted calling (SHAKEN/STIR) and US outbound rules apply; consult the US voice
  guidelines and high-volume voice considerations. (Twilio guidelines/blog)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.

## Facts to verify before teaching

- Exact SDK major version for the browser Voice SDK changes; the supplied note flags
  an old 1.14 quickstart, so always cite the current v2 docs.
- CPS/concurrency and the 4-hour cap are account-tier dependent; present them as
  documented defaults and flag that limits can be raised.