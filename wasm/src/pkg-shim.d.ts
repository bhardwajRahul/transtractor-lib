declare module "../pkg/transtractor.js" {
  interface WasmParser {
    parse(pdfFilePath: string): unknown;
    parseBytes(pdfBytes: Uint8Array): unknown;
    parseLayout(layoutFilePath: string): unknown;
    parseLayoutText(layoutText: string): unknown;
    load(configJsonPath: string): void;
    loadConfigFromJson(configJson: string): void;
    getDeprecationWarnings(): unknown[];
    layout(pdfFilePath: string, outputFile: string): void;
    layoutBytes(pdfBytes: Uint8Array): string;
    debug(pdfFilePath: string, outputFile: string): void;
    debugLayout(layoutFilePath: string, outputFile: string): void;
    debugBytes(pdfBytes: Uint8Array): string;
    debugLayoutText(layoutText: string): string;
    spec(pdfFilePath: string, outputFile: string): void;
    specLayout(layoutFilePath: string, outputFile: string): void;
    specBytes(pdfBytes: Uint8Array): string;
    specLayoutText(layoutText: string): string;
    validateSpec(specFilePath: string): void;
    validateSpecText(specJson: string): void;
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
    layout(pdfFilePath: string, outputFile: string): void;
    layoutBytes(pdfBytes: Uint8Array): string;
    debug(pdfFilePath: string, outputFile: string): void;
    debugLayout(layoutFilePath: string, outputFile: string): void;
    debugBytes(pdfBytes: Uint8Array): string;
    debugLayoutText(layoutText: string): string;
    spec(pdfFilePath: string, outputFile: string): void;
    specLayout(layoutFilePath: string, outputFile: string): void;
    specBytes(pdfBytes: Uint8Array): string;
    specLayoutText(layoutText: string): string;
    validateSpec(specFilePath: string): void;
    validateSpecText(specJson: string): void;
  }

  export default function init(): Promise<void>;
}
