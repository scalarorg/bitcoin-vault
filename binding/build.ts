/// <reference types="bun-types" />
import { generateDtsBundle } from "dts-bundle-generator";

import { dependencies } from "./package.json";

import fs from "node:fs";

console.log("Building...");

// Build for browser
await Bun.build({
  entrypoints: ["./src/index.ts"],
  external: Object.keys(dependencies),
  format: "esm",
  minify: true,
  outdir: "./dist",
  sourcemap: "external",
  target: "browser",
  naming: {
    entry: "[dir]/[name].[ext]",
    chunk: "[dir]/[name].[ext]",
  },
});

// Build for node
await Bun.build({
  entrypoints: ["./src/index.ts"],
  external: Object.keys(dependencies),
  format: "esm",
  minify: true,
  outdir: "./dist/node",
  sourcemap: "external",
  target: "node",
});

console.log("JSCompiling", "Done!");
console.log("TypeCompiling", "Building...");
const typedContentForBrowser = generateDtsBundle([
  {
    filePath: "./src/index.ts",
  },
]);

fs.mkdirSync("./dist", { recursive: true });

// Write typed content to index.d.ts for both browser and node
fs.writeFileSync("./dist/index.d.ts", typedContentForBrowser.join("\n"));

const typedContent = generateDtsBundle([
  {
    filePath: "./src/index.ts",
  },
]);

fs.mkdirSync("./dist/node", { recursive: true });
fs.writeFileSync("./dist/node/index.d.ts", typedContent.join("\n"));
console.log("TypeCompiling", "Done!");
