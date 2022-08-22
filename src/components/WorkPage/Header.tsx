import { Link, useParams } from "@solidjs/router";
import { Component } from "solid-js";

const Header: Component = () => {
  const params = useParams();

  return (
    <div
      class="h-12 bg-opacity-50 bg-slate-500 fixed z-header w-full"
      tabIndex={-1}
    >
      ここにヘッダー {`page: ${params["page"]}`}
      <Link href={`/work/${params["id"]}/${+params["page"] + 1}`}>link</Link>
    </div>
  );
};

export default Header;
