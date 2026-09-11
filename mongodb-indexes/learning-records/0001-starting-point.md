# Starting Point: MongoDB Indexes & Query Performance

The learner is a self-taught TypeScript backend developer on AWS, working toward
senior-level JS/TS + Node.js mastery. He uses MongoDB Atlas as a core data store
and can model documents and write aggregations, but he treats indexes as a
guess-and-check afterthought. His gap is a precise command of how indexes make
queries fast: the B-tree structure, IXSCAN vs COLLSCAN, compound indexes and
their prefix rule, the ESR (Equality–Sort–Range) ordering, index-served sorts,
multikey (array) indexes, unique/TTL/partial/sparse properties, and reading
`explain` output to prove an index is used. He wants to design the right, minimal
set of indexes and verify them with query plans, and defend the reasoning in a
senior interview. This lesson set is self-contained and independent of all other
courses in the repository.