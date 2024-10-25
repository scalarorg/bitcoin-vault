const fs = require('fs-extra');
const path = require('path');

const srcDir = path.join(__dirname, '..', 'src_ts');
const pkgDir = path.join(__dirname, '..', 'pkg');
const distDir = path.join(__dirname, '..', 'dist');

// Ensure the dist directory exists
fs.ensureDirSync(distDir);

// Copy TypeScript build output
fs.copySync(srcDir, distDir, {
  filter: (src) => !src.includes('node_modules') && path.extname(src) !== '.ts'
});

// Copy WebAssembly build output
fs.copySync(pkgDir, distDir, {
  //filter: (src) => !src.endsWith('.ts') // Exclude TypeScript definition files if any
});

console.log('Build files combined in dist folder');
