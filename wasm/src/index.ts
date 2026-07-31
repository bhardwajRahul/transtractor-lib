import * as wasmModule from "../pkg/transtractor.js";
import type { StatementData } from "./types.js";

let initPromise: Promise<void> | null = null;

type WasmParserCtor = {
  new (): {
    parse(pdfFilePath: string): unknown;
    parseBytes(pdfBytes: Uint8Array): unknown;
    parseLayout(layoutFilePath: string): unknown;
    parseLayoutText(layoutText: string): unknown;
    load(configJsonPath: string): void;
    loadConfigFromJson(configJson: string): void;
    getDeprecationWarnings(): unknown[];
  };
};

const WasmParser = wasmModule.Parser as unknown as WasmParserCtor;

async function ensureInitialized(): Promise<void> {
  if (!initPromise) {
    const maybeInit = (wasmModule as unknown as { default?: () => Promise<void> })
      .default;
    initPromise = typeof maybeInit === "function" ? maybeInit() : Promise.resolve();
  }
  await initPromise;
}

export class Parser {
  private readonly inner: InstanceType<WasmParserCtor>;

  private constructor(inner: InstanceType<WasmParserCtor>) {
    this.inner = inner;
  }

  static async create(): Promise<Parser> {
    await ensureInitialized();
    return new Parser(new WasmParser());
  }

  parse(pdfFilePath: string): StatementData {
    return this.inner.parse(pdfFilePath) as StatementData;
  }

  parseBytes(pdfBytes: Uint8Array): StatementData {
    return this.inner.parseBytes(pdfBytes) as StatementData;
  }

  parseLayout(layoutFilePath: string): StatementData {
    return this.inner.parseLayout(layoutFilePath) as StatementData;
  }

  parseLayoutText(layoutText: string): StatementData {
    return this.inner.parseLayoutText(layoutText) as StatementData;
  }

  load(configJsonPath: string): void {
    this.inner.load(configJsonPath);
  }

  loadConfigFromJson(configJson: string): void {
    this.inner.loadConfigFromJson(configJson);
  }

  getDeprecationWarnings(): string[] {
    return this.inner.getDeprecationWarnings().map(String);
  }
}

export async function init(): Promise<void> {
  await ensureInitialized();
}

export type { StatementData } from "./types.js";
