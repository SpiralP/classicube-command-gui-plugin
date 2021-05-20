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
    <div>
      {Object.entries(players).map(([id, p]) => (
        <pre key={id} title={p.realName}>{`${p.nickName} ${p.group}`}</pre>
      ))}
    </div>
  );
}

function send(connection: WebSocket, obj: JsonMessage) {
  return connection.send(JSON.stringify(obj));
}
