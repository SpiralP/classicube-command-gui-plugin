import React, { useEffect } from "react";

interface ConnectionArgs {
  port: number;
  path: string;
}

export default function App() {
  useEffect(() => {
    console.log(location.hash.slice(1));
    const args: ConnectionArgs = JSON.parse(atob(location.hash.slice(1)));
    console.log(args);

    const ws = new WebSocket(`ws://127.0.0.1:${args.port}/${args.path}`);
    ws.addEventListener("open", () => {
      console.log("connected!");
      ws.send("hello!");
    });

    ws.addEventListener("close", () => {
      window.close();
    });
    ws.addEventListener("error", () => {
      window.close();
    });

    return () => {
      //   disconnect();
    };
  }, []);

  return <h2>yea</h2>;
}
