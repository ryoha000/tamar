import type { Component } from "solid-js";
import { Route, Routes } from "@solidjs/router";
import TopPage from "./page/TopPage";
import { Toaster } from "solid-toast";
import ViewWorkPage from "./page/ViewWorkPage";

const App: Component = () => {
  return (
    <>
      <Routes>
        <Route path="/" component={TopPage} />
        <Route path="/work/:id/view" component={ViewWorkPage} />
      </Routes>
      <Toaster />
    </>
  );
};

export default App;
