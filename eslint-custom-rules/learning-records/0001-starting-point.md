# Starting Point: ESLint & Authoring Custom Rules

The learner is a self-taught TypeScript / Node.js developer working toward senior
mastery. He uses ESLint on every project but treats it as a black box: he copies
a shared config, toggles rules until CI passes, and has never written a rule of
his own. He cannot yet explain what the linter does with his source text, how the
config array composes, or why an autofix sometimes refuses to apply.

His goal is to close that gap in the direction senior work actually happens:
encoding team conventions as executable policy, catching a bug class the core
rules miss, and shipping that rule so every repository gets identical feedback.

The zone of proximal development starts below rule authoring. He needs the
mechanism first — static analysis over an ESTree AST, and the flat config array
as a cascade — before `meta`/`create` will mean anything. From there the ladder
is: read an AST and target nodes with selectors, write a reporting rule, make it
fix safely, parameterise it with schema-validated options, test it with
`RuleTester`, package it as a plugin, and finally make it scope-aware and roll it
out without drowning the team in warnings.

Likely misconceptions to watch for: thinking ESLint executes or type-checks the
code; assuming `plugins` is still an array of strings; assuming severity changes
what is detected; expecting `fix()` to edit text immediately; and treating a
plugin's `configs` as auto-applied. Each of these is addressed directly in the
lesson that introduces the mechanism.

This lesson set is designed to be self-contained and independent of all other
courses in the repository.
