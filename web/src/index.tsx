import React from "react";
import ReactDOM from "react-dom";
import App from "./App";

let args: ConnectionArgs | undefined;
try {
  args = JSON.parse(atob(location.hash.slice(1)));
  // location.hash = "";
  // history.pushState(
  //   "",
  //   document.title,
  //   window.location.pathname + window.location.search
  // );
} catch (e) {
  console.error(e);
}

ReactDOM.render(<App connectionArgs={args} />, document.getElementById("root"));