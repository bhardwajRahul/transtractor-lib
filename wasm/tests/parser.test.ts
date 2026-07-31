import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import { beforeAll, describe, expect, it } from "vitest";

import { Parser } from "../src/index.js";

const FIXTURE_DIR = resolve(import.meta.dirname, "../../tests/fixtures");

describe("Parser", () => {
  let parser: Parser;

  beforeAll(async () => {
    parser = await Parser.create();
    const configPath = resolve(FIXTURE_DIR, "test1_config.json");
    const configJson = readFileSync(configPath, "utf8");
    parser.loadConfigFromJson(configJson);
  });

  it("parses layout text into statement data", () => {
    const layoutPath = resolve(FIXTURE_DIR, "test1_layout.txt");
    const specPath = resolve(FIXTURE_DIR, "test1_spec.json");

    const layoutText = readFileSync(layoutPath, "utf8");
    const expected = JSON.parse(readFileSync(specPath, "utf8")) as {
      statement_data: {
        key: string;
        account_number: string;
        start_date: number;
        start_date_year: number;
        opening_balance: number;
        closing_balance: number;
        proto_transactions: unknown[];
      };
    };

    const actual = parser.parseLayoutText(layoutText);

    expect(actual.key).toBe(expected.statement_data.key);
    expect(actual.account_number).toBe(expected.statement_data.account_number);
    expect(actual.start_date).toBe(expected.statement_data.start_date);
    expect(actual.start_date_year).toBe(expected.statement_data.start_date_year);
    expect(actual.opening_balance).toBe(expected.statement_data.opening_balance);
    expect(actual.closing_balance).toBe(expected.statement_data.closing_balance);
    expect(actual.transactions).toHaveLength(
      expected.statement_data.proto_transactions.length,
    );
    expect(actual.transactions[0]).toMatchObject({
      date: expect.any(Number),
      index: expect.any(Number),
      description: expect.any(String),
      amount: expect.any(Number),
      balance: expect.any(Number),
    });
  });

  it("keeps bytes-based parsing as a placeholder", () => {
    expect(() => parser.parseBytes(new Uint8Array([0x25, 0x50, 0x44, 0x46]))).toThrow(
      /Byte-based PDF parsing is not available yet/,
    );
  });
});
