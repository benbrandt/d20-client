import binaryen from "binaryen";
import fs from "fs";
import findFiles from "find";

/**
 * Load, optimize and save all .wasm files in folder `dist`
 */
findFiles.eachfile(/\.wasm$/, "./dist", file => {
  const wasmModule = binaryen.readBinary(fs.readFileSync(file));
  wasmModule.optimize();
  fs.writeFileSync(file, wasmModule.emitBinary());
  console.log(`File: '${file}' optimized.`);
});
