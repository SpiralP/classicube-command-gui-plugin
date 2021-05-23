import { createContext, useEffect, useState } from "react";
import { ConnectionArgs, JsonEvent, JsonMessage } from "./types";

export class Connection {
  private ws: WebSocket;

  private listeners: Set<(event: JsonEvent) => void> = new Set();

  constructor(ws: WebSocket) {
    this.ws = ws;
  }

  public send(message: JsonMessage) {
    return this.ws.send(JSON.stringify(message));
  }

  public addListener(listener: (event: JsonEvent) => void) {
    this.listeners.add(listener);
  }

  public removeListener(listener: (event: JsonEvent) => void) {
    this.listeners.delete(listener);
  }

  public handleEvent(event: JsonEvent) {
    this.listeners.forEach((listener) => {
      listener(event);
    });
  }
}

export const ConnectionContext =
  createContext<Connection | undefined>(undefined);

export function useSetupConnection(connectionArgs?: ConnectionArgs) {
  const [connection, setConnection] =
    useState<Connection | undefined>(undefined);

  useEffect(() => {
    if (!connectionArgs) return undefined;

    const ws = new WebSocket(
      `ws://127.0.0.1:${connectionArgs.port}/${connectionArgs.path}`
    );
    ws.addEventListener("open", () => {
      const newConnection = new Connection(ws);
      ws.addEventListener("message", ({ data }) => {
        if (typeof data !== "string") return;

        const event = JSON.parse(data);
        newConnection.handleEvent(event);
      });

      setConnection(newConnection);
    });

    ws.addEventListener("close", () => {
      ws.close();
      setConnection(undefined);
      // window.close();
    });
    ws.addEventListener("error", () => {
      ws.close();
      setConnection(undefined);
      // window.close();
    });

    return () => {
      ws.close();
      setConnection(undefined);
    };
  }, [connectionArgs]);

  return connection;
}
