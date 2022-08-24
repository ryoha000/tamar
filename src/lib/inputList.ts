// https://stackoverflow.com/a/65073572
export const useInputList = (
  inputCallback: (e: HTMLInputElement) => void,
  listCallback: (e: HTMLInputElement) => void
) => {
  let eventSource: "input" | "list" | null = null;
  const keydown = (e: KeyboardEvent) => {
    eventSource = e.key ? "input" : "list";
  };
  const input = (e: InputEvent) => {
    if (!e.target || !(e.target instanceof HTMLInputElement)) {
      return;
    }
    if (eventSource === "list") {
      listCallback(e.target);
    } else {
      inputCallback(e.target);
    }
  };

  return { keydown, input };
};

export default useInputList;
