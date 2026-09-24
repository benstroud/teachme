# Notes — AWS CDK workspace

## Working notes (author/maintainer)

- Dark mode everywhere: dark warm paper, light ink, amber accent (AWS/serverless
  feel). Print query flips back to light so pages print cleanly.
- Learner: self-taught TS dev, senior-track. Fill foundational gaps (what
  synthesis is, what a CloudFormation template is, what an L1 vs L2 construct is)
  before advanced material.
- Lessons are skill-first and short. Every lesson ends with an interactive quiz
  (self-contained inline `<script>`) plus an "ask the agent" box.
- Every non-trivial claim carries a footnote citation to the AWS CDK Developer
  Guide or the CDK API reference. Never assert CDK behavior from memory alone.
- Learner is solo and opted out of communities — do not keep proposing them.
- Cross-references stay WITHIN this workspace. These are independent lessons; do
  not link to, or name, any other course in the repository.
- AWS docs are sometimes JS-rendered; cite canonical URLs, keep claims
  conservative, and flag anything to confirm against the installed `aws-cdk-lib`.

## Facts verified during authoring (2026-09)

- AWS CDK v2 is an open-source framework for defining cloud infrastructure in code
  and provisioning it through AWS CloudFormation. It has two parts: the CDK
  Construct Library (pre-written modular constructs) and the CDK Toolkit (the CDK
  CLI). Supports TypeScript, JavaScript, Python, Java, C#/.NET, and Go. (CDK home)
- Constructs are the basic building blocks of CDK apps, each representing one or
  more CloudFormation resources and their configuration. Prepared and used via
  import; you can also author your own. (CDK constructs)
- Construct levels: L1 (CFN resources, named Cfn*, one per CloudFormation
  resource, no abstraction, generated from the CFN resource specification), L2
  (curated/widely used, intent-based API, sensible defaults, best-practice
  security, glue logic, helper methods), and L3 (patterns, higher abstraction).
  (CDK constructs)
- All Construct Library classes are instantiated with three arguments: the scope
  (parent in the construct tree), an id, and props (a key/value bundle conforming
  to an interface like BucketProps). TypeScript uses interfaces for props shapes.
  (CDK work-in-TypeScript)
- To work in TypeScript you create a project with `cdk init app --language
  typescript`, which installs aws-cdk-lib. It uses npm and the TypeScript compiler
  (tsc); build with `npm run build`, watch with `npm run watch`. You can use global
  tools (`npm install -g aws-cdk`) or local ones (`npx aws-cdk ...`). (CDK TS docs)
- The CDK integrates with CloudFormation to deploy and provision resources, with
  predictable repeated deployments and rollback on error. (CDK home)
- Standard CLI lifecycle: init a project, `cdk synth` to emit CloudFormation
  templates, `cdk bootstrap` for the bootstrap/CDKToolkit stack with an assets
  bucket, `cdk deploy` to provision, plus `cdk diff`, `cdk list`, `cdk destroy`.
  (CDK getting-started / core concepts; verify flags against installed version)

Sources stored in RESOURCES.md; tap them before asserting any fact in a lesson.