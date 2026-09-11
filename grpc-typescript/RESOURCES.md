# gRPC TypeScript Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is solo and has opted out
of joining communities, so the Wisdom section is minimal.

## Knowledge

### gRPC core & guides (grpc.io)

- [Introduction to gRPC](https://grpc.io/docs/what-is-grpc/introduction/)
  Official orientation: gRPC = RPC framework + protobuf as IDL; the stub/client
  model; `protoc` + gRPC plugin codegen; proto3 recommended.
  Use for: every foundational lesson.

- [gRPC Core Concepts](https://grpc.io/docs/what-is-grpc/core-concepts/)
  Channels, RPC lifecycle, synchronous vs asynchronous, and the four call types.
  Use for: lesson 0005 (channels).

- [gRPC Deadlines](https://grpc.io/docs/guides/deadlines/)
  No default deadline (# client waits forever), server auto-cancels,
  deadline→remaining-timeout propagation to dodge clock skew.
  Use for: lessons 0004 and 0005.

- [gRPC Metadata](https://grpc.io/docs/guides/metadata/)
  Key/value side channel via HTTP/2 headers + trailers; `grpc-` prefix reserved.
  Use for: lesson 0006.

- [gRPC Status Codes](https://grpc.io/docs/guides/status-codes/)
  The full 0–16 table with exact meanings; which codes are generated only by user
  code; FAILED_PRECONDITION / ABORTED / UNAVAILABLE guidance.
  Use for: lessons 0004 and 0008.

- [gRPC Retry](https://grpc.io/docs/guides/retry/)
  Transparent retry vs configured retry; service-config `retryPolicy` knobs;
  jitter, throttling, commit-on-response-header.
  Use for: lesson 0005.

- [gRPC Interceptors](https://grpc.io/docs/guides/interceptors/)
  Middleware for every RPC; client vs server interceptors; ordering.
  Use for: lesson 0006.

- [gRPC Authentication](https://grpc.io/docs/guides/auth/)
  SSL/TLS channel creds, token-based call creds via metadata generator,
  combining channel + call credentials.
  Use for: lessons 0006 and 0009.

- [gRPC Error Handling](https://grpc.io/docs/guides/error-handling/)
  How status + rich details (e.g. `google.rpc.ErrorInfo`) are surfaced.
  Use for: lessons 0004 and 0008.

### gRPC Node / TypeScript

- [gRPC Node implementation packages guide](https://github.com/grpc/grpc-node/blob/master/PACKAGE-COMPARISON.md)
  Pros/cons of `@grpc/grpc-js`, `grpc`, `@grpc/proto-loader`, protoc static codegen.
  Use for: lesson 0003.

- [gRPC Node Basics tutorial](https://grpc.io/docs/languages/node/basics/)
  Dynamic vs static codegen; the four call types; streaming client events.
  Use for: lessons 0003, 0004, 0007.

- [gRPC Node Quick start](https://grpc.io/docs/languages/node/quickstart/)
  Minimal client/server with `@grpc/grpc-js`.
  Use for: lesson 0004.

- [@grpc/grpc-js API reference](https://grpc.github.io/grpc/node/)
  Channel options, deadline options, metadata, server API.
  Use for: lessons 0003–0007.

### Protocol Buffers

- [protobuf.dev — Language Guide (proto3)](https://protobuf.dev/programming-guides/proto3/)
  Messages, scalars, enums, repeated, nested, imports, oneof; updating messages;
  the rule that you never reuse or rename a field number.
  Use for: lesson 0002.

- [protobuf.dev — Encoding (wire format)](https://protobuf.dev/programming-guides/encoding/)
  Field tags (number<<3 | wire type), varints, LEN records, unknown-field
  preservation — why compatibility is possible.
  Use for: lesson 0002.

- [protobuf.dev — Style Guide](https://protobuf.dev/programming-guides/style/)
  Naming and file layout conventions.
  Use for: lessons 0002 and 0003.

### gRPC-Web / the BFF boundary

- [gRPC-Web Basics](https://grpc.io/docs/languages/web/basics/)
  Browser → proxy (Envoy grpc_web filter) → gRPC backend; commonjs/grpcwebtext modes.
  Use for: lesson 0008 (deciding the BFF boundary).

### AWS deployment

- [AWS ELB — Register gRPC targets with an Application Load Balancer](https://docs.aws.amazon.com/elasticloadbalancing/latest/application/load-balancer-target-grpcs.html)
  ALB gRPC target type, HTTP/2 requirement, TLS/h2c, health checks.
  Use for: lesson 0009.

- [AWS Lambda — Using gRPC with API Gateway (transcoding)](https://docs.aws.amazon.com/lambda/latest/dg/api-gateway-grpc.html)
  How gRPC→REST transcoding maps gRPC calls onto Lambda/API Gateway.
  Use for: lesson 0009.

- [AWS Cloud Map — service discovery](https://docs.aws.amazon.com/cloud-map/latest/dg/cloud-map-overview.html)
  Service discovery attributes and health checks for dynamic instance resolution.
  Use for: lesson 0009.

- [Amazon ECS — gRPC](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/)
  Hosting a long-running Node gRPC service in ECS behind an ALB.
  Use for: lesson 0009.

### Observability

- [OpenTelemetry — gRPC instrumentation](https://opentelemetry.io/docs/specs/otlp/)
  Trace context propagation across RPC boundaries; `grpc-trace-bin` / traceparent.
  Use for: lessons 0006 and 0009.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. proofing a real
  gRPC→ALB setup), revisit — but do not push.

## Gaps

- The AWS doc pages are JS-rendered and not always scrapable, so AWS-specific
  claims (ALB gRPC exact fields, API Gateway transcoding limits) are cited to the
  canonical URLs and kept conservative. Live verification of an actual
  ECS+ALB gRPC deployment would need an AWS account — currently unknown whether
  the learner has one; lessons flag any step that requires the console/CLI.
- Whether to prefer static TS codegen (buf / grpc-tools) vs dynamic
  `@grpc/proto-loader` depends on the learner's repo setup; lessons present both
  and let the learner pick a default.