// import * as wasm from "rust_wasm_utils";

// wasm.greet();

const js = import("rust_wasm_utils/rust_wasm_utils.js");

js.then((js) => {
  // js.greet("WASM");
  const result = js.sum(1, 2);
  // js.aesEncrypt("password", "Hello, world!");
  // js.imageResize([], 1024, 768);
  console.log(result);
});

// var Module = {};
// fetch("./lib/rust_wasm_utils_bg.wasm")
//   .then((response) => response.arrayBuffer())
//   .then((buffer) => {
//     Module.wasmBinary = buffer;
//     const script = document.createElement("script");
//     script.src = "./lib/rust_wasm_utils.js";
//     document.body.appendChild(script);
//   });

// WebAssembly.instantiateStreaming(fetch("./lib/rust_wasm_utils_bg.wasm")).then(
//   (wasmModule) => {
//     // this saves the exported function from WASM module for use in JS
//     wasm_greet = wasmModule.instance.exports.greet;
//   }
// );

// import { greet, default as init } from "./lib/rust_wasm_utils.js";

// async function run() {
//   await init("./lib/rust_wasm_utils_bg.wasm");

//   // make the function available to the browser
//   window.greet = greet;
// }

// run();
