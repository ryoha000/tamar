/* @refresh reload */
import "./index.css";
import { render } from "solid-js/web";

import App from "./App";
import { Router } from "@solidjs/router";

import { StoreProvider } from "./lib/store";

render(
  () => (
    <StoreProvider>
      <Router>
        <App />
      </Router>
    </StoreProvider>
  ),
  document.getElementById("root") as HTMLElement
);
