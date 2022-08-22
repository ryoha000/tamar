import type { ParentComponent } from "solid-js";

interface Props {
  label: string;
}

const TooltipWrapper: ParentComponent<Props> = (props) => {
  return (
    <span class="relative group">
      <span
        class={[
          "whitespace-nowrap",
          "rounded",
          "bg-white",
          "border border-secondary",
          "px-2",
          "py-1",
          "absolute",
          "top-8",
          "left-1/2",
          "-translate-x-1/2",
          "before:content-['']",
          "before:absolute",
          "before:-translate-x-1/2",
          "before:left-1/2",
          "before:-top-2",
          "before:border-4",
          "before:border-transparent",
          "before:border-b-secondary",
          "opacity-0",
          "group-hover:opacity-100",
          "transition",
          "pointer-events-none",
        ].join(" ")}
      >
        {props.label}
      </span>
      {props.children}
    </span>
  );
};

export default TooltipWrapper;
