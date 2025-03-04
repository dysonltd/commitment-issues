# Commitment issues

[![MegaLinter](https://github.com/dysonltd/commitment-issues/actions/workflows/mega-linter.yaml/badge.svg)](https://github.com/dysonltd/commitment-issues/actions/workflows/mega-linter.yaml) [![Continuous Build](https://github.com/dysonltd/commitment-issues/actions/workflows/continuous-build.yaml/badge.svg)](https://github.com/dysonltd/commitment-issues/actions/workflows/continuous-build.yaml)

Embed git repository metadata into compiled binaries.

## Features
- `no_std` compatible
- Works on both natively-compiled and cross-compiled binaries
- Works for baremetal, embedded OS and full OS environments
- Binary metadata is easily accessible from within source code

## Usage

### Creating a new project from template

A very simple `cargo generate` example template is provided which demonstrates usage of the crate in a natively-compiled binary.
To use the template, you will need cargo generate installed (for instructions see [here](https://github.com/cargo-generate/cargo-generate/tree/main)).

#### Generate the project

Run:

```sh
cargo generate -g https://github.com/dysonltd/commitment-issues
```

#### Create the initial commit

At least one commit needs to have been made in order for the metadata to be populated.
If there are no commits in the project, compilation will fail.
Create your initial commit with:

```sh
git add -A
git commit -m "Initial commit"
```

#### Compile and run

The project can now be compiled and run with:

```sh
cargo run
```

### Use in a pre-existing project

The easiest way to use this crate in a pre-existing project is also with `cargo generate`.

#### Add commitment-issues as a dependency

Navigate into your project directory and add the crate to your dependencies with:

```sh
cargo add --git https://github.com/dysonltd/commitment-issues
```

#### Configure the project to include the metadata in the binary

Run `cargo generate` with the following command and follow the prompts:

```sh
cargo generate -g https://github.com/dysonltd/commitment-issues --init
```

**Please note** the `--init` argument being passed to `cargo generate`. This specifies that the target project already exists.

If certain files already exist in your project, you will need to add commands to them to complete the configuration.
Make sure to follow all the prompts given during the `cargo generate` process.

#### Invoke metadata embedding

Add the following to one of your project's source files (e.g. `src/main.rs`):

```rust
use commitment_issues::include_metadata;

include_metadata!();
```

This invokes the `include_metadata!()` macro, which generates a module called `metadata` within your source file at compile time.

#### Accessing metadata within project source code

As well as containing the metadata section, the `metadata` module exposes a set of macros to access the various fields of the metadata section from within your code.
The template's [main.rs](https://github.com/dysonltd/commitment-issues/browse/template/src/main.rs) file gives an example of how these macros can be used.

#### Compile and run

Compile and run your project using your standard process.

## Inspecting binary metadata

The metadata can be found in a section called ".metadata" within the executable binary generated by cargo.
This can easily be read using `objdump` from the [`binutils`](https://www.gnu.org/software/binutils/) package:

```sh
objdump -s <path/to/binary> | grep "section \.metadata" -A 20
```

The metadata is an 80-byte block beginning with the sequence 0xFFFEFDFC and ending with the sequence 0x01020304 (inclusive).
The data contained within is as follows:

| Field | Size(bytes) | Description |
| --- | ---: | --- |
| Schema version | 1 | Placeholder for version numbering for metadata schema. Currently fixed |
| Build timestamp | 20 | Timestamp when the last build occurred in RFC3339 format |
| Commit hash | 10 | Short hash of the active commit |
| Dirty build | 1 | Boolean value representing whether the build contains uncommitted changes |
| Last tag | 20 | Most recent relevant tag in the active commit's history |
| Author | 20 | Author of the active commit |

Future work will include adding a command line tool for reading metadata from a "raw" binary and presenting it in a human-readable format.

The current structure of the metadata is fixed.
Future work will aim to make it possible to configure the structure of the metadata through a configuration file.

## Windows & Mac targets

**TODO!** This crate has not yet been tested for a Windows or Mac target.

## Running your Linter Locally

This project uses [MegaLinter](https://github.com/oxsecurity/megalinter) which provides linters for various different file formats and languages. When a Pull request to main is done, the linters will run and ensure the codebase is in good standing. It is recommended that you run the linter locally beforehand as it can sometimes autofix common mistakes.

```bash
npx mega-linter-runner
```

You will need to have docker and Node installed to use this, more information can be found on their [repo](https://github.com/oxsecurity/megalinter)

## Future work

- Allow configuration of the metadata structure through a `.toml` file
- Add a tool for reading metadata from a binary
