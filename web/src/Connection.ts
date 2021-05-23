import { useEffect, useState } from "react";

export class Connection {
  ws: WebSocket;
  listeners: Set<(message: JsonEvent) => void> = new Set();
  constructor(ws: WebSocket) {
    this.ws = ws;
  }

  send(message: JsonMessage) {
    return this.ws.send(JSON.stringify(message));
  }

  addListener(listener: (message: JsonEvent) => void) {
    this.listeners.add(listener);
  }

  removeListener(listener: (message: JsonEvent) => void) {
    this.listeners.delete(listener);
  }

  handleEvent(event: JsonEvent) {
    this.listeners.forEach((listener) => {
      listener(event);
    });
  }
}

export function useConnection(connectionArgs?: ConnectionArgs) {
  const [connection, setConnection] =
    useState<Connection | undefined>(undefined);

  useEffect(() => {
    if (!connectionArgs) return;

    const ws = new WebSocket(
      `ws://127.0.0.1:${connectionArgs.port}/${connectionArgs.path}`
    );
    ws.addEventListener("open", () => {
      console.log("connected!");

      setConnection(new Connection(ws));
    });
    ws.addEventListener("message", ({ data }) => {
      if (!connection || typeof data !== "string") return;

      const event = JSON.parse(data);
      console.log(event);
      connection.handleEvent(event);
    });

    ws.addEventListener("close", () => {
      ws.close();
      setConnection(undefined);
      window.close();
    });
    ws.addEventListener("error", () => {
      ws.close();
      setConnection(undefined);
      window.close();
    });

    return () => {
      ws.close();
      setConnection(undefined);
    };
  }, []);

  return connection;
}
