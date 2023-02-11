# git-cc

A simple interface for authoring Git commit messages according to the 
[Conventional Commits specification](https://www.conventionalcommits.org).

https://user-images.githubusercontent.com/1724257/200127461-176898e8-1216-4c94-bc72-630a2fdb995e.mov

## Installation

Build the application with `cargo build --release` or download a prebuilt binary from a 
release in GitHub, then place that binary in your `PATH`. As long as you have Git installed and 
have left the binary named `git-cc`, it should then be invokable as `git cc`.

## Operation

When invoked through `git cc`, an interface is presented which guides the user 
through creating a commit message which follows the 
[Conventional Commits specification](https://www.conventionalcommits.org).

Commits in this specification follow this format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

This tool will present the user with three steps corresponding to the 
description, body, and footers and attempt to constrain input according to the 
specification. Throughout the interface, the user may continue to a next step 
with `ENTER`/`Tab`, return to a previous step with `ESC`/`BackTab`, or quit with `CTRL+C`.

### Quick Mode

If invoked as `git cc -q`, a shorter prompt sequence is presented including only the first
description line and omitting the body, breaking changes, and trailers.

### Step 1: Description

The first step prompts for three summary-level values.

```
<type>[optional scope]!: <description>
```

 - The commit type is scrolled with the `UP`/`DOWN` keys
 - The optional commit scope describes the unit being changed
 - The commit description is a short summary of the change
 - The text-length limit for this step is 80 characters
 - All text from this step is lower-cased
 - A breaking-change indicator "!" may be inserted by a later step

### Step 2: Body

The second step accepts multi-line prose as input.

```
[optional body]
```

 - The commit body is the only input for this step and is optional
 - Three consecutive newlines will continue to the next step
 - Every line has a length-limit of 100 characters

### Step 3: Footer

The footer is broken into two sub-steps for breaking changes and footer/trailer 
values.

#### Breaking changes

For breaking changes, the user can toggle between "Yes" and "No" with `UP`/
`DOWN`. If yes, a text entry is provided for the breaking change description.

```
BREAKING CHANGE: A description of the change.
```
_An example of the "Yes" scenario prompting for a description._

```
BREAKING CHANGE: No
```
_An example of the "No" scenario._

#### Footer or trailer values

For "footer" or "trailer" values, users may enter key-values separated by a colon-space `: `. The 
value is optional These key-value-pairs may indicate a variety of data such as the commit reviewer 
or task number.

```
[key]: [value]
[key]
[key]: [value]
```

## Implementation

This tool makes the following assumptions:
 - The user is competent with Git and command-line utilities
 - The user is familiar with the [Conventional Commits specification](https://www.conventionalcommits.org)
 - The user may be using this tool in a variety of CLI environments

The interface is designed with the following goals:
 - Provide the minimal interface to effectively guide a user
 - Avoid hiding subsequent prompts and stepping through input unnecessarily
 - Maximize the amount of input from a single step in the interface
