import type { Component } from "solid-js";
import Header from "../components/TopPage/Header";
// import { useToast } from "../components/UI/use/toast";
import { toast } from "solid-toast";

const TopPage: Component = () => {
  // const aaa = useToast();
  const click = () => {
    const rand = Math.floor(Math.random() * 100) % 2;
    if (rand === 0) {
      toast.success("success message", { position: "bottom-right" });
    }
    if (rand === 1) {
      toast.error("error message", { position: "bottom-right" });
    }
  };
  return (
    <div>
      <Header />
      <p class="text-4xl text-green-700 text-center py-20">Top Page</p>
      <button onclick={click}>add toast</button>
    </div>
  );
};

export default TopPage;
