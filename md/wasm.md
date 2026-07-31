# WASM Guide

This document describes how to build and use the Transtractor WASM bindings.

The WASM package is implemented in TypeScript under `wasm/` and wraps Rust bindings exposed with `wasm-bindgen`.

## Current Scope

* The API is exposed as a `Parser` class, aligned with the Python parser shape.
* File-based parsing via `Parser.parse(filePath)` is available.
* Byte-based parsing is already present as `Parser.parseBytes(bytes)` but currently returns a placeholder error because `pdfsink-rs` does not yet expose a byte-input API.
* `Parser.parseLayoutText(layoutText)` is available now and works without filesystem access.

## Build Prerequisites

Install the required toolchain:

```shell
cargo install wasm-pack
```

Install TypeScript package dependencies:

```shell
cd wasm
npm ci
```

Build the Node-oriented package output:

```shell
npm run build
```

Build browser-focused WASM output:

```shell
npm run build:web
```

## Static Website Example (No Transpiler)

The following example uses the direct browser (`--target web`) output as ES modules.

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Transtractor WASM Demo</title>
  </head>
  <body>
    <input id="layout-file" type="file" accept=".txt" />
    <pre id="output"></pre>

    <script type="module">
      import init, { Parser } from "./pkg-web/transtractor.js";

      await init();
      const parser = new Parser();
      const input = document.getElementById("layout-file");
      const output = document.getElementById("output");

      input.addEventListener("change", async (event) => {
        const file = event.target.files?.[0];
        if (!file) {
          return;
        }

        // Current placeholder route: parse layout text directly.
        // PDF byte parsing will be enabled once pdfsink-rs supports byte input.
        const layoutText = await file.text();
        const statementData = parser.parseLayoutText(layoutText);
        output.textContent = JSON.stringify(statementData, null, 2);
      });
    </script>
  </body>
</html>
```

Notes:

* `Parser.parse(...)` is the intended route for PDF parsing, but in browser-only contexts this depends on future byte-based PDF input support in `pdfsink-rs`.
* Until then, use `parseLayoutText(...)` in static web contexts.

## TypeScript Example (Runtime With Filesystem)

```ts
import { Parser } from "@transtractor/wasm";

const parser = await Parser.create();
parser.load("tests/fixtures/test1_config.json");

const statementData = parser.parse("/absolute/path/to/statement.pdf");
console.log(statementData.key);
console.log(statementData.transactions.length);
```

## Test, Lint, and Coverage

From `wasm/`:

```shell
npm run lint
npm run test
npm run coverage
```

Coverage is emitted to `wasm/coverage/cobertura-coverage.xml` and is consumed by CI as `cov-js.xml`.
