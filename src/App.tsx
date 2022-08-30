import type { Component } from "solid-js";
import { Route, Routes } from "@solidjs/router";
import TopPage from "./page/TopPage";
import { Toaster } from "solid-toast";
import WorkPage from "./page/WorkPage";
import ArtistPage from "./page/ArtistPage";

const App: Component = () => {
  return (
    <>
      <Routes>
        <Route path="/" component={TopPage} />
        <Route path="/work/:id/:page" component={WorkPage} />
        <Route path="/artist/:id" component={ArtistPage} />
      </Routes>
      <Toaster />
    </>
  );
};

export default App;
