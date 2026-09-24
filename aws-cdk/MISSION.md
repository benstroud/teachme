# Mission: AWS CDK in TypeScript

## Why

Ben is a self-taught TypeScript developer aiming for senior-level JS/TS + Node.js
mastery, driven by career growth and interviews. He builds backends on AWS and is
increasingly expected to define and operate infrastructure, not just application
code. Hand-clicking the console, or maintaining long hand-written CloudFormation
templates, does not scale teams or keep environments repeatable.

He wants to master the **AWS Cloud Development Kit (CDK)**: defining cloud
infrastructure in a real programming language (TypeScript), synthesizing it to
CloudFormation, and deploying via CloudFormation — with the same rigor as
application code (reviews, testing, source control). The goal is the ability to
model an app as an <em>App → Stack → Construct</em> tree, choose the right level of
abstraction (L1/L2/L3), compose constructs, wire resources together, manage assets
and context, and maintain it all safely across environments — and to defend the
approach in a senior-level conversation.

This is an independent workspace: it does not reference or link to any other
course in the repository.

## Success looks like

- Explain what AWS CDK is, what the CDK Construct Library and CDK Toolkit are, and
  how CDK deploys through CloudFormation.
- Walk the deploy lifecycle: init, synth, bootstrap, diff, deploy, destroy — and
  say what each does and when each is needed.
- Model an application as App, Stack, and Construct; define multiple stacks and
  choose target environments (account + region).
- Explain the three construct levels (L1 CFN resources, L2 curated constructs,
  L3 patterns) and pick the right one for a resource.
- Author a custom construct in TypeScript with scope, id, and props, and compose it
  from L2 constructs; expose values for consumers.
- Use the aws-cdk-lib service modules (aws-s3, aws-lambda, aws-iam) and reach down
  to L1 or the escape hatch for CFN-level control.
- Understand props bundles and Tokens/lazy values, and predict the CloudFormation
  references that get resolved (Ref, GetAtt, etc.).
- Bundle and deploy assets (Lambda code, Docker images) knowing how the bootstrap
  bucket is involved.
- Wire resources together: pass values, let CDK infer dependencies, use IAM
  grants, and reason about cross-stack references.
- Use context (cdk.json, cdk.context.json) and Aspects for cross-cutting policy,
  and apply CDK best practices for maintainability and safe changes.

## Constraints

- Self-taught; foundational gaps must be filled, not skipped.
- Node / TypeScript mindset; examples should feel natural to a TS developer.
- Learns by doing — lessons lean on hands-on skill (cdk init, synth, diff, small
  constructs) over lecture.
- Prefers dark-mode HTML for generated materials.
- Solo learner; opted out of joining communities.
- Content is grounded in the official AWS CDK Developer Guide; claims beyond it are
  explicit about uncertainty. It is architecture-and-authoring focused, not a tour
  of every CDK module.
- Independent workspace: no cross-references to sibling courses, even by name.

## Out of scope

- A deep dive into CloudFormation template syntax; CloudFormation is treated as the
  deployment target CDK synthesizes to.
- A full guided project (a separate hands-on workflow); lessons build skill in
  small, verifiable increments.
- Other IaC tools (Terraform, Pulumi, SAM) beyond naming them as comparisons.
- The internals of CDK code generation; the focus is authoring and operating.