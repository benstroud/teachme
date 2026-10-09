# Starting Point: Microservices Service-to-Service CDC

The learner is a self-taught TypeScript backend developer working toward senior
JS/TS + Node.js mastery. He builds microservices on AWS with MongoDB Atlas and
needs to propagate data changes between services that each own their own database.
He wants to master service-to-service CDC in his exact stack: MongoDB Change
Streams as the change source, Amazon EventBridge for routing, TypeScript Lambda
relay and consumer functions with idempotency and retries, and the whole pipeline
implemented as infrastructure-as-code with the AWS CDK (event bus, rules, Lambda
targets, IAM). This lesson set is designed to be self-contained and independent of
all other courses in the repository.