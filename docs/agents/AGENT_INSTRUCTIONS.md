# Agent Instructions for Language Specification

This document provides guidelines for AI agents and contributors editing the fib language specification.

## Document Purpose

The language specification is the authoritative reference for the fib programming language. It serves as:

- A precise definition of language semantics
- A reference for compiler implementers
- Documentation for language users
- A PDF-exportable technical document

---

## Writing Style and Tone

### Voice and Register

- **Use formal, technical prose**: Write in an impersonal, third-person style.
- **Avoid first and second person**: Do not use "we," "you," "our," or "your."
- **Use passive voice judiciously**: Prefer active constructions when clear, but passive is acceptable for technical descriptions.
- **Be declarative**: State facts and rules directly.

**Correct**:

> A variable declaration introduces a binding that associates an identifier with a variable.

**Incorrect**:

> When you declare a variable, we create a binding that associates your identifier with a variable.

### Precision and Clarity

- **Define terms before using them**: Introduce terminology with italics on first use and provide a definition.
- **Be unambiguous**: Each statement should have exactly one interpretation.
- **Avoid hedging language**: Do not use "basically," "sort of," "kind of," or "generally speaking."
- **Use consistent terminology**: Once a term is defined, use it consistently throughout.

### Conciseness

- **One concept per sentence**: Keep sentences focused on a single idea.
- **One sentence per line**: Write each sentence on its own line in the source file for easier diffs and reviews.
- **Avoid redundancy**: Do not repeat information unnecessarily.
- **Remove filler words**: Eliminate "very," "really," "quite," "actually," and similar weak modifiers.

---

## Document Structure

### Section Hierarchy

The document uses a strict heading hierarchy:

```
# Language specification (H1 - document title only)
## Major Section (H2)
### Subsection (H3)
#### Sub-subsection (H4)
```

- **H1**: Reserved for the document title only
- **H2**: Major conceptual divisions (e.g., "Type system," "Functions," "Control flow")
- **H3**: Primary topics within a section
- **H4**: Detailed subtopics when necessary

Never skip heading levels (e.g., do not go from H2 directly to H4).

### Section Organization Pattern

Each section or subsection should follow this general pattern when applicable:

1. **Definition**: A one-sentence italicized definition of the concept
2. **Description**: Explanatory prose elaborating on the definition
3. **Syntax**: Formal syntax in a labeled code block
4. **Semantics**: Behavioral rules and constraints (if not covered above)
5. **Examples**: Illustrative code samples
6. **Notes**: Additional considerations, edge cases, or caveats

Not every section requires all elements—use what is appropriate.

---

## Formatting Conventions

### Definitions

Introduce new terms with italics on first use:

```markdown
A _variable_ is a storage location that holds a _value_.
```

### Syntax Blocks

Always label syntax blocks with a bold **Syntax**: prefix:

```markdown
**Syntax**:

\`\`\`
let <identifier> <type> = <expression>[;]
\`\`\`
```

Use angle brackets for placeholders: `<identifier>`, `<expression>`, `<type>`
Use square brackets for optional elements: `[;]`, `[<type>]`
Use `...` for repetition patterns

### Example Blocks

Always label example blocks with a bold **Example**: or **Examples**: prefix:

```markdown
**Example**:

\`\`\`
let x int = 42;
\`\`\`
```

### Code Samples

- Use fenced code blocks with no language specifier for fib code (plain ``` blocks)
- Keep examples minimal—show only what is needed to illustrate the concept
- Add inline comments in examples only when necessary for clarity
- Ensure all examples are syntactically correct fib code

### Tables

Use tables for:

- Operator listings with descriptions and examples
- Escape sequence references
- Type-to-description mappings
- Any structured reference information

Format tables with consistent alignment:

```markdown
| Operator | Description | Example |
| -------- | ----------- | ------- |
| `+`      | Addition    | `a + b` |
| `-`      | Subtraction | `a - b` |
```

### Inline Code

Use backticks for:

- Keywords: `let`, `function`, `type`, `if`, `match`
- Operators: `+`, `*`, `==`, `->`, `&&`
- Type names: `int`, `string`, `bool`
- Literal values: `42`, `true`, `"hello"`
- Variable and function names when referenced: `x`, `add()`
- File names and paths

### Cross-References

Use Markdown link syntax for internal references:

```markdown
[Name resolution](#name-resolution) determines which binding an identifier refers to.
See [Array index access](#array-index-access) for details.
```

Ensure anchor names match the auto-generated slugs (lowercase, hyphens for spaces).

### Notes and Caveats

Use blockquotes with a bold **Note**: prefix for important asides:

```markdown
> **Note**: When declaring a variable with a product type, the type expression must be enclosed in parentheses.
```

### Lists

Use bulleted lists for:

- Unordered collections of items
- Access rules
- Feature enumerations

Use numbered lists only when order matters (steps in a process).

Keep list items parallel in grammatical structure.

---

## PDF Export Formatting

The specification is designed for PDF export using markdown-pdf tooling with custom CSS.

### Page Breaks

Use the `<div class="page"/>` element to force a page break before a new major section:

```markdown
<div class="page"/>

## New Major Section
```

Place page breaks:

- Before each H2 section (major divisions)
- After the table of contents
- Before particularly long sections that would otherwise break awkwardly

Do **not** place page breaks:

- Before H3 or H4 sections (these should flow with their parent)
- In the middle of conceptual units
- Between a heading and its first paragraph

### Table of Contents

The TOC is auto-generated by `doctoc`. The markers must be preserved:

```markdown
<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

...TOC content...

<!-- END doctoc generated TOC please keep comment here to allow auto update -->
```

After editing, regenerate the TOC by running:

```bash
doctoc docs/language_specification.md
```

### Content That Should Not Break Across Pages

The CSS handles most cases automatically, but be mindful:

- Keep code blocks short enough to fit on one page
- Keep tables compact; split very long tables into multiple tables
- Ensure headings are not orphaned at the bottom of pages

If a code block is unavoidably long, consider whether it can be split into multiple smaller examples with explanatory text between them.

### Avoiding Formatting Issues

- Do not use HTML except for `<div class="page"/>` page breaks
- Do not use inline styles
- Do not use raw HTML tables—use Markdown tables
- Ensure there is a blank line before and after code blocks
- Ensure there is a blank line before and after tables

---

## Content Guidelines

### What Belongs in the Specification

- Syntax definitions (how to write constructs)
- Semantic rules (what constructs mean)
- Type system rules (how types work)
- Evaluation rules (how expressions are computed)
- Compile-time vs. runtime behavior distinctions
- Error conditions and their handling

### What Does NOT Belong

- Tutorials or learning-oriented content
- Rationale or design justification (use `language_philosophy.md` for this)
- Implementation details specific to a compiler
- Standard library documentation
- Style guides or best practices for users

### Handling Incomplete Sections

If a section is planned but not yet written:

```markdown
### Future Feature

_This section is planned for a future revision._
```

Do not leave sections empty or with placeholder text like "TODO" or "TBD."

### Maintaining Consistency

When adding new content:

1. Review similar existing sections for structure and style
2. Use the same terminology as established elsewhere in the document
3. Cross-reference related sections when appropriate
4. Ensure new syntax integrates with the existing grammar

---

## Specific Patterns

### Defining a New Construct

Follow this template:

```markdown
### Construct Name

A _construct name_ is [one-sentence definition].
[Additional explanatory sentences as needed.]

**Syntax**:

\`\`\`
<syntax_definition>
\`\`\`

[Explanation of syntax elements if not self-evident.]

**Example**:

\`\`\`
// Minimal illustrative code
\`\`\`

[Optional additional explanation of the example.]
```

### Describing Operators

Use a table format:

```markdown
#### Operator Category

[Brief description of this category.]

| Operator | Description  | Example  |
| -------- | ------------ | -------- |
| `op`     | What it does | `a op b` |

[Additional semantic details not captured in the table.]
```

### Describing Type Constructors

```markdown
#### Type Name

A _type name_ [definition].
Type names are constructed using [syntax element].

**Syntax**:

\`\`\`
<type_syntax>
\`\`\`

**Examples**:

\`\`\`
// Several examples showing variety
\`\`\`

[Semantic rules, constraints, and behavior.]
```

### Error Conditions

When describing error conditions, be specific about whether they are:

- Compile-time errors (static)
- Runtime errors (dynamic)
- Panics (unrecoverable)

```markdown
Accessing an index outside the array bounds causes a panic.
Attempting to access a private entity from outside its module causes a compile-time error.
```

---

## Checklist for Contributions

Before submitting changes to the specification:

- [ ] All new terms are italicized on first use and defined
- [ ] Syntax blocks are labeled with **Syntax**:
- [ ] Example blocks are labeled with **Example**: or **Examples**:
- [ ] Code examples are syntactically correct
- [ ] Cross-references use correct anchor links
- [ ] No first or second person pronouns
- [ ] One sentence per line in source
- [ ] Page breaks (`<div class="page"/>`) are placed appropriately
- [ ] Tables are properly formatted with alignment
- [ ] No HTML except for page breaks
- [ ] TOC regenerated if headings changed

---

## Common Mistakes to Avoid

1. **Mixing syntax and examples**: Keep formal syntax separate from illustrative examples
2. **Undefined terminology**: Always define a term before or when first using it
3. **Inconsistent naming**: Use the same name for the same concept throughout
4. **Over-explaining**: The spec is a reference, not a tutorial
5. **Under-specifying**: Every construct needs clear syntax and semantics
6. **Breaking heading hierarchy**: Never skip levels (H2 → H4)
7. **Orphaned headings**: Never end a page with just a heading
8. **Missing blank lines**: Always have blank lines around code blocks and tables
