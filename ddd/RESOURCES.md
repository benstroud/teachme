# Domain-Driven Design Resources

Curated, high-trust sources. Lessons draw on these. Verify any assertion against them before teaching it.

## Knowledge

### Primary source: the book

- Eric Evans, *Domain-Driven Design: Tackling Complexity in the Heart of Software*. Addison-Wesley, 2003. ISBN 0-321-12521-5.
  Table of contents confirmed from the publisher's catalogue page:
  <https://www.informit.com/store/domain-driven-design-tackling-complexity-in-the-heart-9780321125217>

  Structure as printed:

  - **Part I: Putting the Domain Model to Work.** Ch. 1 Crunching Knowledge; Ch. 2 Communication and the Use of Language; Ch. 3 Binding Model and Implementation.
  - **Part II: The Building Blocks of a Model-Driven Design.** Ch. 4 Isolating the Domain; Ch. 5 A Model Expressed in Software; Ch. 6 The Life Cycle of a Domain Object; Ch. 7 Using the Language: An Extended Example.
  - **Part III: Refactoring Toward Deeper Insight.** Ch. 8 Breakthrough; Ch. 9 Making Implicit Concepts Explicit; Ch. 10 Supple Design; Ch. 11 Applying Analysis Patterns; Ch. 12 Relating Design Patterns to the Model; Ch. 13 Refactoring Toward Deeper Insight.
  - **Part IV: Strategic Design.** Ch. 14 Maintaining Model Integrity; Ch. 15 Distillation; Ch. 16 Large-Scale Structure; Ch. 17 Bringing the Strategy Together.

  Use for: every lesson. The chapter pointers in the lessons refer to these titles.
  The InformIT page is a single source for the table of contents. Check it against a physical copy before relying on the exact numbering.

- Eric Evans, *Domain-Driven Design Reference* (summary of the patterns, 2015).
  <https://www.domainlanguage.com/ddd/reference/>
  The summary page is live. Its PDF was not readable as text when fetched, so lesson definitions are not drawn from it. Read the PDF directly when you can.
  Use for: a pattern index once you have the PDF open.

### Commentary

- Martin Fowler, "Ubiquitous Language" (bliki, dated 31 October 2006).
  <https://martinfowler.com/bliki/UbiquitousLanguage.html>
  The common language between developers and users, built on the domain model, and kept rigorous.
  Use for: lesson 0002.

- Martin Fowler, "Anemic Domain Model" (bliki, dated 25 November 2003).
  <https://martinfowler.com/bliki/AnemicDomainModel.html>
  Domain objects with data and no behaviour, with the logic in services. The O/R mapping cost, and the Transaction Script drift.
  Use for: lessons 0004 and 0005.

- Martin Fowler, "DDD Aggregate" (bliki, dated 23 April 2013).
  <https://martinfowler.com/bliki/DDD_Aggregate.html>
  An aggregate is a cluster of objects treated as one unit. References from outside go to the root. Transactions should not cross aggregate boundaries.
  Use for: lesson 0006.

- Martin Fowler, "Bounded Context" (bliki, dated 15 January 2014).
  <https://martinfowler.com/bliki/BoundedContext.html>
  Large systems are divided into contexts, each with a unified model, and the mappings between them are made explicit.
  Use for: lesson 0009.

## Wisdom (Communities)

- None recorded. This set does not propose communities or workshops.

## Gaps

- The Anticorruption Layer article on Fowler's bliki returned HTTP 404 when fetched. It is not cited here.
- The Evans reference PDF could not be read as text. Pattern definitions beyond the chapter titles are the author's summaries of the book and are not quoted from either source.
- The InformIT page is one catalogue. The chapter numbering has not been checked against a second source or a physical copy.
