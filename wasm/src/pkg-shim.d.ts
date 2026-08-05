declare module "../pkg/transtractor.js" {
  interface WasmParser {
    parse(pdfFilePath: string): unknown;
    parseBytes(pdfBytes: Uint8Array): unknown;
    parseLayout(layoutFilePath: string): unknown;
    parseLayoutText(layoutText: string): unknown;
    load(configJsonPath: string): void;
    loadConfigFromJson(configJson: string): void;
    getDeprecationWarnings(): unknown[];
  }

  export class Parser implements WasmParser {
    constructor();
    parse(pdfFilePath: string): unknown;
    parseBytes(pdfBytes: Uint8Array): unknown;
    parseLayout(layoutFilePath: string): unknown;
    parseLayoutText(layoutText: string): unknown;
    load(configJsonPath: string): void;
    loadConfigFromJson(configJson: string): void;
    getDeprecationWarnings(): unknown[];
  }

  export default function init(): Promise<void>;
}
