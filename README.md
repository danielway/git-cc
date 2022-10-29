# git-cc

A simple interface for authoring Git commit messages according to the 
[Conventional Commits specification](https://www.conventionalcommits.org).

## Installation

Simply build the application with `cargo build --release` or download a prebuilt binary from a 
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
with `ENTER`, return to a previous step with `ESC`, or quit with `CTRL+C`.

### Step 1: Description

The first step prompts for three summary-level values.

```
Specify type (UP/DOWN), scope and summary (TAB):
<type>[optional scope]: <description>
```

 - The commit type is scrolled with the `UP`/`DOWN` keys
 - The commit scope is the default text entry and is optional
 - The commit description can be switched-to with `TAB` and is required
 - The text-length limit for this step is 80 characters
 - All text from this step is lower-cased

### Step 2: Body

The second step accepts multi-line prose as input.

```
Specify commit detail ('\' for newline):
[optional body]
```

 - The commit body is the only input and is optional
 - Newlines may be entered into the commit body with '\\'
 - Every line has a length-limit of 100 characters
 - If a line is too long, it will auto-wrap broken by spaces

### Step 3: Footer

The footer is broken into two sub-steps for breaking changes and footer/trailer 
values.

#### Breaking changes

For breaking changes, the user can toggle between "Yes" and "No" with `UP`/
`DOWN`. If yes, a text entry is provided for the breaking change description.

```
Is this a breaking change? Yes: <description>
```
_An example of the "Yes" scenario prompting for a description._

```
Breaking change? No
```
_An example of the "No" scenario._

 - If the commit is a breaking change, the line's length-limit is 
 `(100 - len("Is this a breaking change? Yes: "))` or 78

#### Footer or trailer values

For "footer" or "trailer" values, users may enter key-values separated by a 
space ` `, a colon-space `: `, or a space-hash ` #`. These key-value-pairs may 
indicate a variety of data such as the commit reviewer or task number.

```
Enter footer values: [footer kvps]
```

 - The input split by ' ', ': ', and ' #' must be an even number
 - Multiple key-values may be specified like `Reviewed-by XYZ Issue #123`

## Implementation

This tool makes the following assumptions:
 - The user is competent with Git and command-line utilities
 - The user is familiar with the [Conventional Commits specification](https://www.conventionalcommits.org)
 - The user may be using this tool in a variety of CLI environments

The interface is designed with the following goals:
 - Provide the minimal interface to effectively guide a user
 - Avoid hiding subsequent prompts and stepping through input unnecessarily
 - Maximize the amount of input from a single step in the interface
