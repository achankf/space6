import React from "react";
import ReactDOM from "react-dom";

import App from "./App";
import { Loading } from "./components/Loading";
import { ModelProvider } from "./context/ModelProxy";
import { setWasmOnce } from "./Wasm";

const container = document.body.appendChild(document.createElement("div"));

ReactDOM.render(
  <React.StrictMode>
    <Loading />
  </React.StrictMode>,
  container
);

import("../pkg")
  .then(async (wasm) => {
    const { Model } = wasm;
    setWasmOnce(wasm);

    const model = Model.create();

    ReactDOM.render(
      <React.StrictMode>
        <ModelProvider model={model}>
          <App />
        </ModelProvider>
      </React.StrictMode>,
      container
    );
  })
  .catch(console.error);
