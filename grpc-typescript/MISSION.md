# Mission: gRPC in TypeScript for Service-to-Service APIs

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. He builds backends on AWS where
client-facing code (a GraphQL BFF and a REST BFF) must talk to a layer of
smaller domain services (orders, catalog, inventory, billing, identity).

That middle hop — between an edge/BFF layer and domain services — is the fracture
point in a microservice architecture. Hand-written JSON REST between every pair
of services drifts, goes stale, and produces a web of untyped fan-out calls. The
goal is the ability to design and articulate a *gRPC-backed* domain service
layer: services whose interface is a contract-first `.proto` file, consumed by
Node/TypeScript BFFs over HTTP/2, deployed on AWS — and to defend that choice in
a senior-level conversation.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain why gRPC fits *inside* the boundary (service↔service) and why REST/GraphQL stay at the *edge* (client-facing BFFs), naming the concrete failure modes of hand-rolled JSON REST between services.
- Read and write a `proto3` contract by hand: `message` fields with type + number, enums, nested messages, `repeated`, imports — and apply the field-number compatibility rules that keep old and new versions interoperable.
- Define a `service` with `rpc` methods and know the four RPC kinds (unary, server-streaming, client-streaming, bidi) and when each belongs in a domain layer.
- Generate TypeScript client/server stubs from a `.proto` (grpc-tools / buf / @grpc/proto-loader) and wire them into a Node project.
- Stand up a running server with `@grpc/grpc-js`, implement handlers, map domain errors to the correct `status.code`, and set realistic deadlines.
- Use channels, name resolution, and a retry policy correctly, and write idempotent handlers so a retry never causes a duplicate side effect.
- Propagate context across the BFF→domain hop: request-id and trace metadata, tenant context, and interceptors; attach authentication (TLS + token/JWT call credentials).
- Use streaming where it earns its complexity in a domain API (bulk query and bulk ingest), and reject it where it does not.
- Design the BFF boundary: how a GraphQL resolver and a REST handler each call the gRPC domain layer, aggregate, and map gRPC status codes to GraphQL/REST errors.
- Pick the right AWS hosting model: direct gRPC on ECS/EKS behind an ALB with HTTP/2 (TLS or h2c), vs Lambda/API Gateway gRPC transcoding, and lay out a reference architecture with service discovery and OpenTelemetry.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node 22 / TypeScript mindset; examples should feel natural to a TS developer.
- Learns by doing — lessons lean on hands-on skill (writing contracts, tracing calls) over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is architecture-and-typing focused, not an exhaustive gRPC/protobuf API tour.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- Deep HTTP/2 wire-protocol internals beyond what explains compatibility and behavior.
- Kubernetes operator/mesh plumbing (Istio, Linkerd) beyond a conceptual mention.
- Full GraphQL schema design or REST API design — only the boundary call pattern.
- Manual protoc C++/plugin authoring; cross-language (Go/Java) codegen beyond the conceptual win of a single contract.
- gRPC-Web framing details; it is mentioned only as a rejected/default option for the BFF boundary.
- Edge service meshes, egress proxy authN, or fine-grained IAM policy authoring.