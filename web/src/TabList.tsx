import { Menu, MenuItem, PopoverInteractionKind } from "@blueprintjs/core";
import React, { useEffect, useState } from "react";

export function usePlayers({
  connection,
}: {
  connection: WebSocket;
}): [Record<number, JsonPlayer>, Record<string, string>] {
  const [players, setPlayers] = useState<Record<number, JsonPlayer>>({});
  const [colorCodes, setColorCodes] = useState<Record<string, string>>({});

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
      } else if (obj.type === "colorCodes") {
        setColorCodes(
          Object.fromEntries(obj.data.map(({ char, color }) => [char, color]))
        );
      }
    }
    connection.addEventListener("message", listener);
    send(connection, { type: "tabListSubscribe" });
    return () => {
      connection.removeEventListener("message", listener);
    };
  }, []);

  return [players, colorCodes];
}

function Colored({
  text,
  colorCodes,
}: {
  text: string;
  colorCodes: Record<string, string>;
}) {
  const parts: JSX.Element[] = [];
  let hadCodeSymbol = false;
  let currentColor = "000000";
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (c === "&") {
      hadCodeSymbol = true;
      continue;
    }
    if (hadCodeSymbol) {
      hadCodeSymbol = false;
      const color = colorCodes[c];
      if (color) {
        currentColor = color;
        continue;
      }
    }

    console.log(currentColor);
    parts.push(
      <span style={{ fontWeight: "bold", color: `#${currentColor}` }}>{c}</span>
    );
  }

  // &aasdf
  // <pre color={a}>asdf</pre>

  return <div>{parts}</div>;
}

export function TabList({ connection }: { connection: WebSocket }) {
  const [players, colorCodes] = usePlayers({ connection });

  return (
    <Menu>
      {Object.entries(players).map(([id, p]) => (
        <MenuItem
          key={id}
          title={p.realName}
          text={<Colored text={p.nickName} colorCodes={colorCodes} />}
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
