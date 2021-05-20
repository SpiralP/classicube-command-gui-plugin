import { Menu, MenuItem, PopoverInteractionKind } from "@blueprintjs/core";
import React, { useEffect, useState } from "react";

export function usePlayers({ connection }: { connection: WebSocket }) {
  const [players, setPlayers] = useState<Record<number, JsonPlayer>>({});

  useEffect(() => {
    function listener({ data }: MessageEvent<any>) {
      if (typeof data !== "string") return;

      const obj: JsonEvent = JSON.parse(data);

      if (obj.type === "newPlayers") {
        setPlayers(Object.fromEntries(obj.data.map((p) => [p.id, p])));
      } else if (obj.type === "playerAdded" || obj.type === "playerChanged") {
        console.log({ ...players, [obj.data.id]: obj.data });
        setPlayers((players) => ({ ...players, [obj.data.id]: obj.data }));
      } else if (obj.type === "playerRemoved") {
        setPlayers((players) => {
          const o = { ...players };
          delete o[obj.data];
          return o;
        });
      } else if (obj.type === "weDisconnected") {
        setPlayers({});
      }
    }
    connection.addEventListener("message", listener);
    send(connection, { type: "tabListSubscribe" });
    return () => {
      connection.removeEventListener("message", listener);
    };
  }, []);

  return players;
}

export function TabList({ connection }: { connection: WebSocket }) {
  const players = usePlayers({ connection });
  return (
    <Menu>
      {Object.entries(players).map(([id, p]) => (
        <MenuItem
          key={id}
          title={p.realName}
          text={p.nickName}
          label={p.group}
          popoverProps={{
            interactionKind: PopoverInteractionKind.CLICK,
          }}
        >
          <MenuItem
            text="TP"
            onClick={() => {
              send(connection, {
                type: "chatCommand",
                data: `TP ${p.realName}`,
              });
            }}
          />
          <MenuItem text="Child two" />
          <MenuItem text="Child three" />
        </MenuItem>
      ))}
    </Menu>
  );
}

function send(connection: WebSocket, obj: JsonMessage) {
  return connection.send(JSON.stringify(obj));
}
