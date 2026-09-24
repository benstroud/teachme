# AWS CDK Resources

Curated, high-trust sources. Knowledge for lessons is drawn from these, not from
memory. Wisdom lives in communities — note: the learner is a solo learner and has
opted out of communities, so that section is kept minimal and is not pushed.

## Knowledge

### Primary sources (cited throughout)

- [AWS CDK Developer Guide — What is the AWS CDK](https://docs.aws.amazon.com/cdk/v2/guide/home.html)
  Definition of CDK v2, the Construct Library and Toolkit/CLI, supported languages,
  and CloudFormation deployment.
  Use for: lessons 0001 and 0002.

- [AWS CDK — Core concepts](https://docs.aws.amazon.com/cdk/v2/guide/basic_concepts.html)
  App, Stack, Construct, environment, synth, deploy, bootstrap, and assets — the
  vocabulary of the whole course.
  Use for: lessons 0001, 0002, 0003, 0008.

- [AWS CDK — Constructs](https://docs.aws.amazon.com/cdk/v2/guide/constructs.html)
  Constructs as building blocks; the L1 (CFN resources), L2 (curated), and L3
  (patterns) levels; authoring constructs.
  Use for: lessons 0004 and 0005.

- [AWS CDK — Working with the AWS CDK in TypeScript](https://docs.aws.amazon.com/cdk/v2/guide/work-with-cdk-typescript.html)
  Project layout, `cdk init app --language typescript`, `aws-cdk-lib`,
  build/run commands, and TypeScript idioms.
  Use for: lessons 0002, 0003, and 0005.

- [AWS CDK — Getting started](https://docs.aws.amazon.com/cdk/v2/guide/getting_started.html)
  Install the CLI, init, bootstrap, and deploy.
  Use for: lessons 0001 and 0002.

### Supporting references

- [AWS CDK API Reference (aws-cdk-lib)](https://docs.aws.amazon.com/cdk/api/v2/docs/aws-cdk-lib.html)
  The construct library API. Confirm exact types and options for any construct.
- [AWS CloudFormation concepts](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cfn-whatis-concepts.html)
  The deployment target CDK synthesizes to; understand templates, stacks, and
  change-driven updates.
- [AWS CDK Best practices](https://docs.aws.amazon.com/cdk/v2/guide/best_practices.html)
  Design and maintainability guidance for production CDK apps.

## Wisdom (Communities)

- Solo learner; opted out of communities. Future sessions should not keep
  proposing them. If real-world feedback is ever needed (e.g. signing off a
  production bootstrap/deploy), revisit — but do not push.
- The official `cdk.dev` community hub and AWS CDK on Stack Overflow are listed in
  the guide as natural places for real-world validation, offered only as pointers.

## Gaps

- A couple of Amazon doc pages are JS-rendered and hard to scrape; lessons keep
  core-concept claims conservative and cite the canonical URLs. Confirm any
  version-specific CLI flag or module name against the installed `aws-cdk-lib`.
- Live verification of bootstrap, asset upload, and deploy requires an AWS account
  with credentials; lessons flag any step that needs the console/CLI and note the
  learner should confirm against their own account.