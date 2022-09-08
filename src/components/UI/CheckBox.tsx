import { Component } from "solid-js";

interface Props {
  label: string;
  isChecked: boolean;
  check: () => void;
}

const CheckBox: Component<Props> = (props) => {
  return (
    <label class="flex items-center gap-2">
      <input type="checkbox" checked={props.isChecked} onchange={props.check} />
      {props.label}
    </label>
  );
};

export default CheckBox;
