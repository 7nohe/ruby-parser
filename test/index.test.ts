/// <reference types="vite/client" />

import { describe, expect, it } from "vitest";

import { parse, RubyNodeType } from "..";
import { readFileSync } from "fs";
import path from "path";

describe("fixtures", () => {
  it("should parse a simple ruby code", () => {
    const code = readFileSync(path.resolve(__dirname, "all_nodes.rb"), "utf-8");
    const result = parse(code)

    expect(JSON.stringify(result, null, 2)).toMatchSnapshot();
  });

  it("should have all nodes", () => {
    const code = readFileSync(path.resolve(__dirname, "all_nodes.rb"), "utf-8");
    const result = parse(code)
    // @ts-ignore
    const allNodes = Object.keys(RubyNodeType);


    const findNode = (node: any) => {
      if (typeof node === "object" && 'typeName' in node) {
          allNodes.splice(allNodes.indexOf(node.typeName), 1);
          Object.values(node).forEach(findNode);
      }

      if (Array.isArray(node)) {
        node.forEach(findNode);
      }
    }

    findNode(result);

    expect(allNodes).toEqual([]);
  })
});