<!-- markdown-link-check-disable -->
# Example Noble

<!-- Original author of paragraph: @gavofyork -->
The Example: A simple example of a FABRIC noble demonstrating
concepts, APIs and structures common to most FABRIC runtimes.

Run `cargo doc --package noble-example --open` to view this noble's documentation.

### Documentation Guidelines:

<!-- Original author of paragraph: Various. Based on collation of review comments to PRs addressing issues with -->
<!-- label 'S3-FABRIC' in https://github.com/tetcoin/tetcore-developer-hub/issues -->
<ul>
    <li>Documentation comments (i.e. <code>/// comment</code>) - should
        accompany noble functions and be restricted to the noble interface,
        not the internals of the noble implementation. Only state inputs,
        outputs, and a brief description that mentions whether calling it
        requires root, but without repeating the source code details.
        Capitalize the first word of each documentation comment and end it with
        a full stop. See
        <a href="https://github.com/tetcoin/tetcore#72-contributing-to-documentation-for-tetcore-packages"
        target="_blank"> Generic example of annotating source code with documentation comments</a></li>
    <li>Self-documenting code - Try to refactor code to be self-documenting.</li>
    <li>Code comments - Supplement complex code with a brief explanation, not every line of code.</li>
    <li>Identifiers - surround by backticks (i.e. <code>INHERENT_IDENTIFIER</code>, <code>InherentType</code>,
        <code>u64</code>)</li>
    <li>Usage scenarios - should be simple doctests. The compiler should ensure they stay valid.</li>
    <li>Extended tutorials - should be moved to external files and refer to.</li>
    <!-- Original author of paragraph: @AmarRSingh -->
    <li>Mandatory - include all of the sections/subsections where <b>MUST</b> is specified.</li>
    <li>Optional - optionally include sections/subsections where <b>CAN</b> is specified.</li>
</ul>

### Documentation Template:<br>

Copy and paste this template from fabric/example/src/lib.rs into file
`fabric/<INSERT_CUSTOM_NOBLE_NAME>/src/lib.rs` of your own custom noble and complete it.
<details><p><pre>
// Add heading with custom noble name

\# <INSERT_CUSTOM_NOBLE_NAME> Noble

// Add simple description

// Include the following links that shows what trait needs to be implemented to use the noble
// and the supported dispatchables that are documented in the Call enum.

- \[`<INSERT_CUSTOM_NOBLE_NAME>::Trait`](https://docs.rs/noble-example/latest/noble_example/trait.Trait.html)
- \[`Call`](https://docs.rs/noble-example/latest/noble_example/enum.Call.html)
- \[`Module`](https://docs.rs/noble-example/latest/noble_example/struct.Module.html)

\## Overview

<!-- Original author of paragraph: Various. See https://github.com/tetcoin/tetcore-developer-hub/issues/44 -->
// Short description of noble's purpose.
// Links to Traits that should be implemented.
// What this noble is for.
// What functionality the noble provides.
// When to use the noble (use case examples).
// How it is used.
// Inputs it uses and the source of each input.
// Outputs it produces.

<!-- Original author of paragraph: @Kianenigma in PR https://github.com/tetcoin/tetcore/pull/1951 -->
<!-- and comment https://github.com/tetcoin/tetcore-developer-hub/issues/44#issuecomment-471982710 -->

\## Terminology

// Add terminology used in the custom noble. Include concepts, storage items, or actions that you think
// deserve to be noted to give context to the rest of the documentation or noble usage. The author needs to
// use some judgment about what is included. We don't want a list of every storage item nor types - the user
// can go to the code for that. For example, "transfer fee" is obvious and should not be included, but
// "free balance" and "reserved balance" should be noted to give context to the noble.
// Please do not link to outside resources. The reference docs should be the ultimate source of truth.

<!-- Original author of heading: @Kianenigma in PR https://github.com/tetcoin/tetcore/pull/1951 -->

\## Goals

// Add goals that the custom noble is designed to achieve.

<!-- Original author of heading: @Kianenigma in PR https://github.com/tetcoin/tetcore/pull/1951 -->

\### Scenarios

<!-- Original author of paragraph: @Kianenigma. Based on PR https://github.com/tetcoin/tetcore/pull/1951 -->

\#### <INSERT_SCENARIO_NAME>

// Describe requirements prior to interacting with the custom noble.
// Describe the process of interacting with the custom noble for this scenario and public API functions used.

\## Interface

\### Supported Origins

// What origins are used and supported in this noble (root, signed, none)
// i.e. root when <code>\`ensure_root\`</code> used
// i.e. none when <code>\`ensure_none\`</code> used
// i.e. signed when <code>\`ensure_signed\`</code> used

<code>\`inherent\`</code> <INSERT_DESCRIPTION>

<!-- Original author of paragraph: @Kianenigma in comment -->
<!-- https://github.com/tetcoin/tetcore-developer-hub/issues/44#issuecomment-471982710 -->

\### Types

// Type aliases. Include any associated types and where the user would typically define them.

<code>\`ExampleType\`</code> <INSERT_DESCRIPTION>

<!-- Original author of paragraph: ??? -->

// Reference documentation of aspects such as `storageItems` and `dispatchable` functions should only be
// included in the https://docs.rs Rustdocs for Tetcore and not repeated in the README file.

\### Dispatchable Functions

<!-- Original author of paragraph: @AmarRSingh & @joepetrowski -->

// A brief description of dispatchable functions and a link to the rustdoc with their actual documentation.

// <b>MUST</b> have link to Call enum
// <b>MUST</b> have origin information included in function doc
// <b>CAN</b> have more info up to the user

\### Public Functions

<!-- Original author of paragraph: @joepetrowski -->

// A link to the rustdoc and any notes about usage in the noble, not for specific functions.
// For example, in the Balances Noble: "Note that when using the publicly exposed functions,
// you (the runtime developer) are responsible for implementing any necessary checks
// (e.g. that the sender is the signer) before calling a function that will affect storage."

<!-- Original author of paragraph: @AmarRSingh -->

// It is up to the writer of the respective noble (with respect to how much information to provide).

\#### Public Inspection functions - Immutable (getters)

// Insert a subheading for each getter function signature

\##### <code>\`example_getter_name()\`</code>

// What it returns
// Why, when, and how often to call it
// When it could panic or error
// When safety issues to consider

\#### Public Mutable functions (changing state)

// Insert a subheading for each setter function signature

\##### <code>\`example_setter_name(origin, parameter_name: T::ExampleType)\`</code>

// What state it changes
// Why, when, and how often to call it
// When it could panic or error
// When safety issues to consider
// What parameter values are valid and why

\### Storage Items

// Explain any storage items included in this noble

\### Digest Items

// Explain any digest items included in this noble

\### Inherent Data

// Explain what inherent data (if any) is defined in the noble and any other related types

\### Events:

// Insert events for this noble if any

\### Errors:

// Explain what generates errors

\## Usage

// Insert 2-3 examples of usage and code snippets that show how to
// use <INSERT_CUSTOM_NOBLE_NAME> Noble in a custom noble.

\### Prerequisites

// Show how to include necessary imports for <INSERT_CUSTOM_NOBLE_NAME> and derive
// your noble configuration trait with the `INSERT_CUSTOM_NOBLE_NAME` trait.

\```rust
use <INSERT_CUSTOM_NOBLE_NAME>;

pub trait Config: <INSERT_CUSTOM_NOBLE_NAME>::Config { }
\```

\### Simple Code Snippet

// Show a simple example (e.g. how to query a public getter function of <INSERT_CUSTOM_NOBLE_NAME>)

\### Example from FABRIC

// Show a usage example in an actual runtime

// See:
// - Tetcore TCR https://github.com/tetsy-samples/tetcore-tcr
// - Tetcore Kitties https://shawntabrizi.github.io/tetcore-collectables-workshop/#/

\## Genesis Config

<!-- Original author of paragraph: @joepetrowski -->

\## Dependencies

// Dependencies on other FABRIC nobles and the genesis config should be mentioned,
// but not the Rust Standard Library.
// Genesis configuration modifications that may be made to incorporate this noble
// Interaction with other nobles

<!-- Original author of heading: @AmarRSingh -->

\## Related Nobles

// Interaction with other nobles in the form of a bullet point list

\## References

<!-- Original author of paragraph: @joepetrowski -->

// Links to reference material, if applicable. For example, Phragmen, W3F research, etc.
// that the implementation is based on.
</pre></p></details>

License: Unlicense
