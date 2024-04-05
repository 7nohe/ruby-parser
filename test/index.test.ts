/// <reference types="vite/client" />

import { describe, expect, it } from "vitest";

import { parse } from "..";
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

    expect(result.typeName).toBe("Begin");
    if (!('statements' in result))
      throw new Error("No statements in result");

    const statements = result.statements;
    
    // class A::B < ::C { }
    const class_ = statements[0];
    expect(class_.typeName).toBe("Class");

    if (!('name' in class_))
      throw new Error("No name in class");

    const name = class_.name;

    if (name === undefined )
      throw new Error("Name is undefined");

    if (typeof name === 'string' )
      throw new Error("Name is a string");

    if (!('typeName' in name))
      throw new Error("No typeName in name");

    expect(name.typeName).toBe("Const");

    if (!('name' in name))
      throw new Error("No name in name");

    expect(name.name).toBe("B");

    if (!('scope' in name))
      throw new Error("No scope in name");

    if (name.scope === undefined)
      throw new Error("Scope is undefined");

    const scope = name.scope;

    if (!( 'name' in scope))
      throw new Error("Scope is a string");
    
    expect(scope.typeName).toBe("Const");
    expect(scope.name).toBe("A");


    if (!('superclass' in class_))
      throw new Error("No superclass in class");

    const superclass = class_.superclass;

    if (superclass === undefined)
      throw new Error("Superclass is undefined");

    if (!('typeName' in superclass))
      throw new Error("No typeName in superclass");

    expect(superclass.typeName).toBe("Const");

    if (!('name' in superclass))
      throw new Error("No name in superclass");

    expect(superclass.name).toBe("C");

    const body = class_.body;

    if (body === undefined)
      throw new Error("Body is undefined");

    expect(body.typeName).toBe("Begin");

    if (!('statements' in body))
      throw new Error("No statements in body");

    const bodyStatements = body.statements;

    // alias foo bar
    const alias = bodyStatements[0];
    expect(alias.typeName).toBe("Alias");

    if (!('to' in alias))
      throw new Error("No to in alias");

    expect(alias.to.typeName).toBe("Sym");
    expect(alias.from.typeName).toBe("Sym");

    const to = alias.to;

    if (!('name' in to))
      throw new Error("No name in to");

    if (to.name === undefined)
      throw new Error("Name is undefined");

    if (typeof to.name === 'string')
      throw new Error("Name is a string");

    if (!('raw' in to.name))
      throw new Error("No raw in name");

    const toRawName = to.name.raw.map((byte) => String.fromCharCode(byte)).join('');
    expect(toRawName).toBe("foo");

    if (!('from' in alias))
      throw new Error("No from in alias");

    const from = alias.from;

    if (!('name' in from))
      throw new Error("No name in from");

    if (from.name === undefined)
      throw new Error("Name is undefined");

    if (typeof from.name === 'string')
      throw new Error("Name is a string");

    if (!('raw' in from.name))
      throw new Error("No raw in name");

    const fromRawName = from.name.raw.map((byte) => String.fromCharCode(byte)).join('');
    expect(fromRawName).toBe("bar");

    // undef foo
    const undef = bodyStatements[1];
    expect(undef.typeName).toBe("Undef");

    // def m(a, (foo), b = foo, *c, d:, e: foo, **f, &g)
    const def_ = bodyStatements[2];
    expect(def_.typeName).toBe("Def");

    // def self.foo
    const defs = bodyStatements[3] 
    expect(defs.typeName).toBe("Defs");
    
    // def m() = 42
    const def_2 = bodyStatements[4];
    expect(def_2.typeName).toBe("Def");

    const defs_2 = bodyStatements[5];
    expect(defs_2.typeName).toBe("Defs");

    // module M
    const module = bodyStatements[6];
    expect(module.typeName).toBe("Module");

    // ::A = 1
    const casgn = bodyStatements[7];
    expect(casgn.typeName).toBe("Casgn");

    // ::A::B = 1
    const casgn2 = bodyStatements[8];
    expect(casgn2.typeName).toBe("Casgn");

    // A = 1
    const casgn3 = bodyStatements[9];
    expect(casgn3.typeName).toBe("Casgn");

    
    const def_3 = bodyStatements[10];
    expect(def_3.typeName).toBe("Def");

    const def_4 = bodyStatements[11];
    expect(def_4.typeName).toBe("Def");

    // BEGIN { foo }
    const preexe = statements[1];
    expect(preexe.typeName).toBe("Preexe");

    // END { foo }
    const postexe = statements[2];
    expect(postexe.typeName).toBe("Postexe");
  })
});