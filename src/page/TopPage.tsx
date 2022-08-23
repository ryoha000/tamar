import { Component } from "solid-js";
import Header from "../components/TopPage/Header";
import WorkList from "../components/TopPage/WorkList";

const TopPage: Component = () => {
  return (
    <div class="flex p-4 pt-14">
      <Header />
      <WorkList />
    </div>
  );
};

export default TopPage;
