import type { Component } from "solid-js";
import DropDownMenu from "../UI/DropDownMenu";
import { MenuDialogSection } from "../UI/MenuDialogWrapper";
import { DepsUsageKind, DEPS_USAGE } from "./use/dirUsage";
import { DirDeps } from "./use/exploreDir";

interface Props {
  deps: DirDeps;
  selectedUsage: DepsUsageKind;
  onChange: (opt: string) => void;
}

const FileImportEachDeps: Component<Props> = (props) => {
  return (
    <MenuDialogSection
      label={`第${props.deps.deps}階層 (sample: ${props.deps.name})`}
    >
      <div class="flex items-center gap-4">
        <div>用途: </div>
        <DropDownMenu
          options={[...DEPS_USAGE]}
          onChange={props.onChange}
          selectedOption={props.selectedUsage}
        />
      </div>
    </MenuDialogSection>
  );
};

export default FileImportEachDeps;
