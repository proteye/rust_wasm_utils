const js = import("rust_wasm_utils/rust_wasm_utils.js");

js.then(async (js) => {
  const encrypted = js.aesEncrypt("password", "Hello, world!");
  const decrypted = js.aesDecrypt("password", encrypted);
  let srcImage = await fetch("./nasa-1024x768.png", {
    referrer: "",
  }).then((r) => r.arrayBuffer()); // empty referrer header because imgur blocks requests from 127.0.0.1
  document.getElementById("src-img").src = URL.createObjectURL(
    new Blob([srcImage], { type: "image/png" })
  );
  const dstImage = js.imageResize(new Uint8Array(srcImage), 640, 480);
  document.getElementById("resized-img").src = URL.createObjectURL(
    new Blob([dstImage.buffer], { type: "image/png" })
  );
});
