# Starting Point: gRPC TypeScript Domain Services

The learner is a self-taught TypeScript backend developer on AWS, working toward
senior-level JS/TS + Node.js mastery. He knows modern Node, TypeScript typing, and
serverless coordination patterns, and has built GraphQL and REST APIs. His gap is
the *service-to-service* hop: how a GraphQL/REST BFF layer should talk to a layer
of domain services. He wants to master contract-first gRPC over HTTP/2 in
TypeScript — proto syntax, codegen, a running `@grpc/grpc-js` server, resilience
(deadlines/retries/idempotency), context propagation and auth, streaming, the BFF
boundary, and AWS hosting — and be able to defend the architecture in a senior
interview. This lesson set is designed to be self-contained and independent of all
other courses in the repository.