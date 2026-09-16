# Contributing to the Transtractor

Thank you for your interest in contributing to the Transtractor. Contributions that add support for additional statement formats are especially welcome, since genuine bank statements are difficult to obtain for testing. Small bug fixes are also welcome; however, changes to the core parsing logic or architecture are generally reserved for maintainers.

This guide covers the usual contributor workflow. For deeper implementation and maintenance details, see the [development guide](md/develop.md). For an overview of the codebase, see the [architecture guide](md/architecture.md).

## Set up the project

You will need:

* [Rust](https://rust-lang.org/tools/install/)
* [Node.js](https://nodejs.org/) for the WebAssembly package
* [uv](https://docs.astral.sh/uv/getting-started/)
* [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) for WebAssembly builds

The repository also includes a VS Code Dev Container with the project tooling preconfigured. With Docker and the Dev Containers extension installed, use **Dev Containers: Reopen in Container** from the Command Palette.

For a local installation, clone the repository and sync all dependencies:

```shell
git clone https://github.com/weberdak/transtractor-lib.git
cd transtractor-lib
uv sync --locked --all-groups
cd wasm
npm ci
```

## Choose the right guide

* To add support for a bank statement format, follow the [statement support workflow](#adding-statement-support).
* To change the Rust parser, Python API, or shared data structures, read the [architecture guide](md/architecture.md) first and add focused tests alongside the change.
* To change the browser or TypeScript interface, see the [WASM guide](md/wasm.md).
* To change user-facing documentation, update the relevant files under `docs/` and build the documentation locally.

## Adding statement support

New statement support generally consists of a configuration, a redacted spec fixture, and any required date or amount formats. The `Parser.test()` method searches the supplied directory and its subdirectories for PDFs and tests whether they can be successfully parsed.

1. Create and test a JSON configuration using the documented [configuration reference](https://transtractor-lib.readthedocs.io/en/latest/configuration.html):

	```python
	from transtractor import Parser

	parser = Parser()
	parser.load("my_config.json")
	parser.test("path/to/statement/pdfs")
	```

2. Convert the configuration into a Rust module under `src/configs/registry/<region>/`. The module filename must match the configuration `key`; register it in the region's `mod.rs`. Register new regions in the `regions` vector in `src/configs/registry/mod.rs`. If the format requires new date or amount formats, add and register them in `src/formats`.

	Rebuild the extension after adding the configuration:

	```shell
	uv run maturin develop --release
	uv sync --locked --group dev
	```

3. Create at least one representative spec fixture. A spec stores the ordered text and coordinates extracted from a PDF together with the expected parsed statement data, allowing realistic layouts to be tested without committing private PDFs. Use `Parser.layout()` to extract layout text, replace sensitive quoted text with dummy values while preserving coordinates and formatting, then validate it with `Parser.debug_layout()` and generate the fixture with `Parser.spec_layout()`:

	In the layout file, search for the regex `"([^"]*)"` to highlight quoted text within the coordinate data. This makes sensitive values easier to find and replace.

	```python
	parser.layout("path/to/representative/statement.pdf", "statement_layout.txt")
	parser.debug_layout("statement_layout.txt", "statement_debug.txt")
	parser.spec_layout(
		 "statement_layout.txt",
		 "{bank-code}__{account-type}__{config-version}__{bank-product}__{spec-version}.json",
	)
	```

	Store the resulting JSON under `tests/fixtures/spec/<region>/`. The directory must match the first component of the configuration `key`; the first three filename components must match the next three components of that key. The fourth component describes the bank product, the fifth must be an integer spec version, and the complete filename must be lowercase. Do not commit real bank statement PDFs or other personal information.

4. Run the spec tests with `cargo test`. They check fixture naming, placement, configuration coverage, and exact parsing behaviour.

## Validate your changes

Run the checks relevant to the code you changed. Before opening a pull request, the core checks are:

```shell
cargo test
uv run pytest
uv run ruff check python scripts tests
uv run ruff format --check python scripts tests
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```

For WASM changes, also run:

```shell
cd wasm
npm run lint
npm run test
npm run build
```

For documentation changes, build the Sphinx site:

```shell
cd docs
uv run make html
```

See [md/develop.md](md/develop.md) for coverage, type checking, dependency audits, CI details, and release procedures.

## Commit messages

Use a short, lowercase commit type followed by a colon and an imperative description:

```text
<type>: <short description>
```

Use the type that best describes the change:

* `feat` — add user-facing functionality or support for a new statement format.
* `fix` — correct a bug or regression.
* `docs` — update documentation or other explanatory content.
* `chore` — perform maintenance, such as updating dependencies or the project version.
* `breaking` — make an incompatible API or behaviour change.

Keep each commit focused. Recent examples include `feat: add support for US Capital One 360 statements`, `docs: document new transaction_balance_ignore param`, and `chore: bump to v0.14.0`.

## Pull requests

Before submitting a pull request:

* Keep the change focused and explain the user-visible or maintenance benefit.
* Add or update tests and fixtures for behavioural changes.
* Check that generated or sensitive files are not included.
* Confirm that formatting, linting, tests, and relevant builds pass locally.
* Update documentation when public behaviour or contributor workflow changes.

Open the pull request from your fork against `main`. GitHub Actions will run the project test, build, lint, type-checking, and audit workflows. Please address failures before requesting review.

Use the following template in the pull-request description:

```markdown
## Summary
One or a few high-level sentences summarising the release. This should be a concise overview of the changes, improvements, or fixes included in this release.

## Changes
### Breaking changes
* One-sentence descriptions without full stops

### Bug fixes
* One-sentence descriptions without full stops

### New features
* One-sentence descriptions without full stops

### Performance enhancements
* One-sentence descriptions without full stops

### Security updates
* One-sentence descriptions without full stops

### Refactoring and code quality improvements
* One-sentence descriptions without full stops

### Documentation updates
* One-sentence descriptions without full stops

### CI/CD improvements
* One-sentence descriptions without full stops

### Issue resolutions
* One-sentence descriptions without full stops

### Miscellaneous
* One-sentence descriptions without full stops
```
