# Contribution Guide
This guide is for adding support for additional PDF bank statement formats in the Transtractor.

## Step 1: Fork the Repo and Set Up Local Development Environment
Follow the instructions in the [Development Guide](develop.md#local-development-environment) to set up Rust and Python. WASM and TypeScript dependencies are not required here.

## Step 2: Develop a Working Configuration
Follow the [user documentation](https://transtractor-lib.readthedocs.io/en/latest/configuration.html) to create a JSON parsing configuration for your statements. Then try parsing the folder that contains your statement PDFs with `Parser.test`:

```python
from transtractor import Parser

parser = Parser()
parser.load("my_config.json")
parser.test("path/to/my/bank/pdf/statements")
```

The `test` method also searches subdirectories for PDFs.

You may also need to create and register new `AmountFormats` and `DateFormats` in the Rust `crate::formats` module.

## Step 3: Incorporate the Configuration into the Source Code
Convert the JSON file into a Rust submodule under `crate::configs::registry::<region code>` and register it in the matching `src/configs/registry/<region code>/mod.rs` file. The file name must match the `key` of the `StatementConfig` being registered.

If you add a new region, register it in `src/configs/registry/mod.rs` in the `regions` vector inside `get_config_map()`.

Recompile and install:

```shell
uv run maturin develop --release
uv sync --locked --group dev
```

## Step 4: Develop at Least One Spec File
Spec files are JSON test fixtures that keep bank statement parsing behaviour stable during development. Each file stores the ordered `TextItem` input extracted from a PDF by `crate::parsers::flows::pdf_to_text_items`, along with the structured `StatementData` produced after parsing. Real bank statement PDFs are not used as fixtures for privacy reasons, but spec files let you redact personal information while preserving the layout details that matter for testing.

To create a spec file, first extract a representative PDF statement into *layout* format in Python:

```python
from transtractor import Parser

parser = Parser()
parser.layout("path/to/representative/bank/statement.pdf", "statement_layout.txt")
```

Then open the layout file in *VS Code* and replace any sensitive personal information with dummy values. Keep the formatting intact and make sure opening, closing, and running balances are consistent with the transaction table. Use the regex `"([^"]*)"` to distinguish text from coordinate data. Common parts you may want to substitute include:

* Names
* Account numbers
* Transaction descriptions
* Amounts
* Balances
* Addresses or other personal contact information

Now test that the modified layout file parses using the `debug_layout` method to provide full diagnostics:

```python
parser.debug_layout("statement_layout.txt", "statement_debug.txt")
```

If parsed successfully, convert the layout to a spec file:

```python
parser.spec_layout("statement_layout.txt", "{bank-code}__{account-type}__{config-version}__{bank-product}__{spec-version}.json")
```

Copy the file into the appropriate regional subdirectory under `tests/fixtures/spec` and follow this naming convention:

* The directory name must match the first component of the applicable configuration `key`
* The first three filename components must match the second, third, and fourth components of `key`
* The fourth filename component is free text describing the bank product
* The fifth filename component must be an integer
* The full filename must be lowercase

## Step 5: Run Tests
Run `cargo test`. A selection of tests will automatically collect all spec files (no registration required) and check them for:

* Correct filename formatting
* Placement in the correct subdirectory
* Coverage for every registered `StatementConfig`
* Exact parsing behaviour as declared

## Step 6: Open a Pull Request
Open a pull request. GitHub Actions workflows will confirm that everything is passing.