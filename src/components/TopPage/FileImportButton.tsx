import { Component, createSignal } from "solid-js";
import { FaSolidFileImport } from "solid-icons/fa";
import FileImportDialog from "./FileImportDialog";
import { dialog } from "@tauri-apps/api";

const FileImportButton: Component = () => {
  const [isOpen, setIsOpen] = createSignal(false);
  const [directory, setDirectory] = createSignal("");

  const openDialog = async () => {
    const dir = await dialog.open({ directory: true });
    if (dir && !Array.isArray(dir)) {
      setDirectory(dir);
      setIsOpen(true);
    }
  };

  return (
    <>
      <FaSolidFileImport
        class="cursor-pointer hover:bg-secondary rounded transition-all p-1"
        onclick={openDialog}
        size="1.5rem"
      />
      <FileImportDialog
        isOpen={isOpen()}
        dir={directory()}
        close={() => setIsOpen(false)}
      />
    </>
  );
};

export default FileImportButton;
