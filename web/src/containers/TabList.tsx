import { Menu, MenuItem, PopoverInteractionKind } from "@blueprintjs/core";
import React, { useEffect, useState } from "react";
import { RenderedText } from "../components/RenderedText";
import { Connection } from "../Connection";

export function usePlayers({
  connection,
}: {
  connection: Connection;
}): [Record<number, JsonPlayer>, Record<string, string>] {
  const [players, setPlayers] = useState<Record<number, JsonPlayer>>({});
  const [colorCodes, setColorCodes] = useState<Record<string, string>>({});

  useEffect(() => {
    function listener(obj: JsonEvent) {
      if (obj.type === "newPlayers") {
        setPlayers(Object.fromEntries(obj.data.map((p) => [p.id, p])));
      } else if (obj.type === "playerAdded" || obj.type === "playerChanged") {
        setPlayers((players) => ({ ...players, [obj.data.id]: obj.data }));
      } else if (obj.type === "playerRemoved") {
        setPlayers((players) => {
          const o = { ...players };
          delete o[obj.data.id];
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
    connection.addListener(listener);
    connection.send({ type: "tabListSubscribe" });

    return () => {
      connection.removeListener(listener);
    };
  }, []);

  return [players, colorCodes];
}

export function TabList({ connection }: { connection: Connection }) {
  const [players, colorCodes] = usePlayers({ connection });

  return (
    <div>
      {/* <Rendered text="h&cell&ao" connection={connection} /> */}
      <Menu>
        {Object.entries(players).map(([id, p]) => (
          <MenuItem
            key={id}
            title={p.realName}
            text={
              <RenderedText connection={connection}>{p.nickName}</RenderedText>
            }
            label={p.group}
            popoverProps={{
              interactionKind: PopoverInteractionKind.CLICK,
            }}
          >
            <MenuItem
              text="TP"
              onClick={() => {
                connection.send({
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
    </div>
  );
}
