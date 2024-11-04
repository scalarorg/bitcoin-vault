const fs = require("fs");

const wasmFile = "./dist/bitcoin-vault-web_bg.wasm";
const jsFile = "./dist/bitcoin-vault-web.js";

console.log("===== POST BUILD FOR WEB =====");
const wasmData = fs.readFileSync(wasmFile);

// Strings that are inserted automatically by wasm-pack, but
// break library in it's current implementation
const brokenStrings = [
  // This substring is unique, had to
  // write only part of line to make the RegExp works.
  // Probably will rewrite in the future
  `input = import.meta.url.replace`,
];

let jsCode = fs.readFileSync(jsFile).toString();

// Commenting out broken strings
brokenStrings.forEach((str) => {
  jsCode = jsCode.replace(new RegExp(str, "g"), "// " + str);
});

jsCode += `
const wasmCode = Uint8Array.from(atob('${wasmData.toString(
  "base64"
)}'), c => c.charCodeAt(0));


async function initializeWasm() {
  if (typeof WebAssembly === "undefined") {
    throw new Error("WebAssembly is not supported in this browser");
  }

  const wasmResponse = new Response(wasmCode, {
    headers: { "Content-Type": "application/wasm" },
  });

  try {
    await __wbg_init(wasmResponse);
  } catch (e) {
    console.error("Failed to initialize WebAssembly:", e);
    throw e;
  }
}

// Initialize when the script loads
initializeWasm().catch((e) => console.error("Initialization failed:", e));
`;

fs.writeFileSync(jsFile, jsCode);

console.log("\n===== POST BUILD FOR NODE =====");

const nodeJsFile = "./dist/bitcoin-vault-node.js";

const fileContent = fs.readFileSync(nodeJsFile, "utf8");

const searchPattern = `const path = require('path').join(__dirname, 'bitcoin-vault-node_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;`;

if (!fileContent.includes(searchPattern)) {
  throw new Error("Target code block not found in the file");
}

const replacedContent = fileContent.replace(
  searchPattern,
  `
const {join, dirname} = require("path");
const {readFileSync} = require("fs");
const {fileURLToPath} = require("url");

const fn = fileURLToPath(import.meta.url);
const dn = dirname(fn);

const wasmPath = join(dn, 'bitcoin-vault-node_bg.wasm');
const bytes = readFileSync(wasmPath);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;`
);

fs.writeFileSync(nodeJsFile, replacedContent);
