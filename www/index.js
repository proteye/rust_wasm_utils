const js = import("rust_wasm_utils/rust_wasm_utils.js");

// Convert file to bytes.
const fileToByteArray = (file) => {
  return new Promise((resolve, reject) => {
    try {
      let reader = new FileReader();
      let fileByteArray = [];
      reader.readAsArrayBuffer(file);
      reader.onloadend = (evt) => {
        if (evt.target.readyState == FileReader.DONE) {
          let arrayBuffer = evt.target.result,
            array = new Uint8Array(arrayBuffer);
          for (byte of array) {
            fileByteArray.push(byte);
          }
        }
        resolve(fileByteArray);
      };
    } catch (e) {
      reject(e);
    }
  });
};

// Convert file to string.
const fileToText = (file) => {
  return new Promise((resolve, reject) => {
    try {
      let reader = new FileReader();
      let text = "";
      reader.readAsText(file);
      reader.onloadend = (evt) => {
        if (evt.target.readyState == FileReader.DONE) {
          text = evt.target.result;
        }
        resolve(text);
      };
    } catch (e) {
      reject(e);
    }
  });
};

// Init script.
js.then(async (js) => {
  var disabled = false;

  // Resize an image by WASM.
  // ~ 1350 ms
  const resize = async (e) => {
    e.preventDefault();

    if (disabled) {
      return;
    }

    const formData = new FormData(document.forms[0]);
    const width = formData.get("width");
    const height = formData.get("height");
    const img = formData.get("image");

    if (!img.size) {
      return;
    }

    disabled = true;
    document.getElementById("resize-submit").disabled = true;

    console.time("Image load");
    const byteArray = await fileToByteArray(img);
    console.timeEnd("Image load");

    console.time("Image resize");
    const dstImage = js.imageResize(byteArray, width, height);
    console.timeEnd("Image resize");

    const url = URL.createObjectURL(
      new Blob([dstImage.buffer], { type: "image/png" })
    );
    document.getElementById("dst-img").src = url;
    const imgLink = document.getElementById("dst-img-link");
    imgLink.href = url;
    imgLink.download = `resized_img_${width}x${height}.png`;

    disabled = false;
    document.getElementById("resize-submit").disabled = false;
  };

  // Encrypt a file by WASM.
  // ~ 280 ms
  const encrypt = async (e) => {
    e.preventDefault();

    if (disabled) {
      return;
    }

    const formData = new FormData(document.forms[1]);
    const textFile = formData.get("textFile");
    const password = formData.get("password");

    if (!textFile.size) {
      return;
    }

    disabled = true;
    document.getElementById("btn-encrypt").disabled = true;

    console.time("Text load");
    const text = await fileToText(textFile);
    console.timeEnd("Text load");

    console.time("Encrypt");
    const encryptedBytes = js.aesEncrypt(password, text);
    console.timeEnd("Encrypt");

    const url = URL.createObjectURL(
      new Blob([encryptedBytes], { type: "application/octet-stream" })
    );
    const link = document.getElementById("encrypted");
    link.innerText = link.download = "encrypted_text.bin";
    link.href = url;

    disabled = false;
    document.getElementById("btn-encrypt").disabled = false;
  };

  // Decrypt a file by WASM.
  // ~ 320 ms
  const decrypt = async (e) => {
    e.preventDefault();

    if (disabled) {
      return;
    }

    const formData = new FormData(document.forms[1]);
    const textFile = formData.get("textFile");
    const password = formData.get("password");

    if (!textFile.size) {
      return;
    }

    disabled = true;
    document.getElementById("btn-decrypt").disabled = true;

    console.time("Text load");
    const cipher = await fileToByteArray(textFile);
    console.timeEnd("Text load");

    console.time("Decrypt");
    const decrypted = js.aesDecrypt(password, cipher);
    console.timeEnd("Decrypt");

    const url = URL.createObjectURL(
      new Blob([decrypted], { type: "text/plain" })
    );
    const link = document.getElementById("decrypted");
    link.innerText = link.download = "decrypted_text.txt";
    link.href = url;

    disabled = false;
    document.getElementById("btn-decrypt").disabled = false;
  };

  // Encrypt a file by JS (sjcl.js).
  // ~ 615 ms
  const encryptByJs = async (e) => {
    e.preventDefault();

    if (!sjcl.cipher.aes) {
      throw new Error("sjcl.js lib is not found!");
    }

    if (disabled) {
      return;
    }

    const formData = new FormData(document.forms[1]);
    const textFile = formData.get("textFile");
    const password = formData.get("password");

    if (!textFile.size) {
      return;
    }

    disabled = true;
    document.getElementById("btn-encrypt").disabled = true;

    console.time("Text load");
    const text = await fileToText(textFile);
    console.timeEnd("Text load");

    console.time("Encrypt JS");
    // const aes = sjcl.cipher.aes("qwerasdf");
    // const encryptedBytes = aes.encrypt(text);
    const encryptedBytes = sjcl.encrypt(password, text, {
      cipher: "aes",
      ks: 256,
    });
    console.timeEnd("Encrypt JS");
    console.info("ENCRYPTED size:", Buffer.byteLength(encryptedBytes, "utf8"));

    const url = URL.createObjectURL(
      new Blob([encryptedBytes], { type: "application/octet-stream" })
    );
    const link = document.getElementById("encrypted");
    link.innerText = link.download = "encrypted_text.bin";
    link.href = url;

    disabled = false;
    document.getElementById("btn-encrypt").disabled = false;
  };

  // Decrypt a file by JS (sjcl.js).
  // ~ 580 ms
  const decryptByJs = async (e) => {
    e.preventDefault();

    if (disabled) {
      return;
    }

    const formData = new FormData(document.forms[1]);
    const textFile = formData.get("textFile");
    const password = formData.get("password");

    if (!textFile.size) {
      return;
    }

    disabled = true;
    document.getElementById("btn-decrypt").disabled = true;

    console.time("Text load");
    const cipher = await fileToText(textFile);
    console.timeEnd("Text load");

    console.time("Decrypt JS");
    // const aes = sjcl.cipher.aes("qwerasdf");
    // const encryptedBytes = aes.decrypt(text);
    const decrypted = sjcl.decrypt(password, cipher, {
      cipher: "aes",
      ks: 256,
    });
    console.timeEnd("Decrypt JS");
    console.info("DECRYPTED size:", Buffer.byteLength(decrypted, "utf8"));

    const url = URL.createObjectURL(
      new Blob([decrypted], { type: "text/plain" })
    );
    const link = document.getElementById("decrypted");
    link.innerText = link.download = "decrypted_text.txt";
    link.href = url;

    disabled = false;
    document.getElementById("btn-decrypt").disabled = false;
  };

  // Add event listeners.
  document.getElementById("resize").addEventListener("submit", resize, false);
  document.getElementById("encrypt").addEventListener("submit", encrypt, false);
  document
    .getElementById("btn-encrypt")
    .addEventListener("click", encrypt, false);
  document
    .getElementById("btn-encrypt-js")
    .addEventListener("click", encryptByJs, false);
  document
    .getElementById("btn-decrypt")
    .addEventListener("click", decrypt, false);
  document
    .getElementById("btn-decrypt-js")
    .addEventListener("click", decryptByJs, false);
});
