fetch('../target/wasm32-unknown-unknown/release/wasm_rust.wasm').then(response =>
  response.arrayBuffer()
).then(bytes => WebAssembly.instantiate(bytes)).then(results => {
  instance = results.instance;
  document.getElementById("container").textContent = instance.exports.add_one(41);
}).catch(console.error);
