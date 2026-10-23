# Starting Point: Twilio API Development (Programmable Voice)

The learner is a self-taught TypeScript / Node.js developer working toward senior
mastery who is building a multi-tenant voice product. Each customer organization
gets a Twilio subaccount; the backend issues a short-lived Access Token with a
VoiceGrant so users can call from the browser via the Twilio Voice SDK; Twilio then
requests the app's /twiml route, which returns a <Dial> to the destination. The
service pins twilio 5.5.1 on Node 20. He wants to understand the whole stack well
enough to secure it and operate it: access tokens and grants, the SDK call flow,
TwiML Dial, X-Twilio-Signature validation, subaccounts, searching/purchasing phone
numbers and binding them to a TwiML app, call-rate and duration limits, trusted
calling/compliance, and the v5→v6 Node library upgrade. This lesson set is designed
to be self-contained and independent of all other courses in the repository.