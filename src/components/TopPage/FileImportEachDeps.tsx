import type { Component } from "solid-js";
import DropDownMenu from "../UI/DropDownMenu";
import { DepsUsageKind, DEPS_USAGE } from "./use/dirUsage";
import { DirDeps } from "./use/exploreDir";

interface Props {
  deps: DirDeps;
  selectedUsage: DepsUsageKind;
  onChange: (opt: string) => void;
}

const FileImportEachDeps: Component<Props> = (props) => {
  return (
    <div>
      <div>
        第{props.deps.deps}階層 (sample:{" "}
        <code class="text-sm">{props.deps.name}</code>)
      </div>
      <div class="flex items-center">
        <div>用途: </div>
        <DropDownMenu
          options={[...DEPS_USAGE]}
          onChange={props.onChange}
          selectedOption={props.selectedUsage}
        />
      </div>
    </div>
  );
};

export default FileImportEachDeps;
