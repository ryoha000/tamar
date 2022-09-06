import type { Component } from "solid-js";
import { Route, Routes } from "@solidjs/router";
import TopPage from "./page/TopPage";
import { Toaster } from "solid-toast";
import WorkPage from "./page/WorkPage";
import ArtistPage from "./page/ArtistPage";
import useDrop from "./lib/drop";
import FileImportDialog from "./components/TopPage/FileImportDialog";

const App: Component = () => {
  const { isOpenFileDialog, closeFileDialog, refetch, filePaths } = useDrop();

  return (
    <>
      <Routes>
        <Route path="/" component={TopPage} />
        <Route path="/work/:id/:page" component={WorkPage} />
        <Route path="/artist/:id" component={ArtistPage} />
      </Routes>
      <Toaster />
      <FileImportDialog
        isOpen={isOpenFileDialog()}
        close={closeFileDialog}
        refetch={refetch}
        filePaths={filePaths()}
      />
    </>
  );
};

export default App;
