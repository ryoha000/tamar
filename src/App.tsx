import type { Component } from "solid-js";
import { Route, Routes } from "@solidjs/router";
import TopPage from "./page/TopPage";
import { Toaster } from "solid-toast";

const App: Component = () => {
  return (
    <>
      <Routes>
        <Route path="/" component={TopPage} />
      </Routes>
      <Toaster />
    </>
  );
};

export default App;
