import { Component } from "solid-js";
import Header from "../components/TopPage/Header";
import { useStore } from "../lib/store";

const ViewWorkPage: Component = () => {
  const store = useStore();
  if (!store) {
    return <div>loading</div>;
  }

  return (
    <div class="flex p-4 pt-14">
      <Header />
      <div>ViewWorkPage</div>
    </div>
  );
};

export default ViewWorkPage;
