const fs = require("fs");

const wasmFile = "./dist/bitcoin-vault-web_bg.wasm";
const jsFile = "./dist/bitcoin-vault-web.js";

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

export async function initializeWasm() {
  const wasmResponse = new Response(wasmCode, { headers: { "Content-Type": "application/wasm" } });
  await __wbg_init(wasmResponse);
}
`;

fs.writeFileSync(jsFile, jsCode);
