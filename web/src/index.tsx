import React from "react";
import ReactDOM from "react-dom";
import { App } from "./containers/App";
import { ConnectionArgs } from "./types";

let args: ConnectionArgs | undefined;
try {
  args = JSON.parse(atob(window.location.hash.slice(1)));
  // location.hash = "";
  // history.pushState(
  //   "",
  //   document.title,
  //   window.location.pathname + window.location.search
  // );
} catch (e) {
  console.warn(e);
}

ReactDOM.render(<App connectionArgs={args} />, document.getElementById("root"));
