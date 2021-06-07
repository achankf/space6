export type Wasm = typeof import("../pkg");

let wasm: Wasm;

export function setWasmOnce(input: Wasm): void {
  if (wasm) {
    throw new Error("Unreachable - wasm reference already set");
  }
  wasm = input;
}

export function getWasm(): Wasm {
  if (!wasm) {
    throw new Error("Unreachable - wasm isn't set");
  }
  return wasm;
}
