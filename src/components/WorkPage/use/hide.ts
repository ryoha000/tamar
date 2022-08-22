import { createSignal, onMount } from "solid-js";

const HIDDEN_MS = 2000;
const useHide = () => {
  const [hidden, setHidden] = createSignal(false);
  const [timer, setTimer] = createSignal(0);

  onMount(() => {
    setHiddenTimer();
  });
  const setHiddenTimer = () => {
    setTimer(setTimeout(() => setHidden(true), HIDDEN_MS));
  };
  const actionStart = () => {
    clearTimeout(timer());
    setHidden(false);
  };
  const actionEnd = () => {
    setHiddenTimer();
  };

  return { actionStart, actionEnd, hidden}
};

export default useHide
