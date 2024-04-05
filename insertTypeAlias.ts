import fs from "node:fs/promises";
import { RubyNodeType } from "./index";

let content = await fs.readFile("index.d.ts", "utf-8");

// @ts-ignore
const keys = Object.keys(RubyNodeType);

content = content + "\n" +  `type RubyNode = ${keys.map((key) => `${key}`).join(" | ")}`;

await fs.writeFile("index.d.ts", content);