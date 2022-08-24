import { ParentComponent } from "solid-js";
import DialogBase from "./DialogBase";

interface Props {
  isOpen: boolean;
  close: () => void;
}

const Dialog: ParentComponent<Props> = (props) => {
  return (
    <DialogBase
      isOpen={props.isOpen}
      close={props.close}
      withCurtain={true}
      align="center"
    >
      {props.children}
    </DialogBase>
  );
};

export default Dialog;
