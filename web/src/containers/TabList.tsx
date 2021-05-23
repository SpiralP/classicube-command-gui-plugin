import { Menu, MenuItem, PopoverInteractionKind } from "@blueprintjs/core";
import React, { useContext, useEffect, useState } from "react";
import { RenderedText } from "../components/RenderedText";
import { Connection, ConnectionContext } from "../Connection";
import { JsonEvent, JsonPlayer } from "../types";

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
        setPlayers((prev) => ({ ...prev, [obj.data.id]: obj.data }));
      } else if (obj.type === "playerRemoved") {
        setPlayers((prev) => {
          const o = { ...prev };
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
  }, [connection]);

  return [players, colorCodes];
}

export function TabList() {
  const connection = useContext(ConnectionContext);
  if (!connection) throw new Error("!connection");

  const [players] = usePlayers({ connection });

  const tpOnClick = (p: JsonPlayer) => () => {
    connection.send({
      type: "chatCommand",
      data: `TP ${p.realName}`,
    });
  };

  return (
    <Menu>
      {Object.entries(players).map(([id, p]) => (
        <MenuItem
          key={id}
          title={p.realName}
          text={<RenderedText>{p.nickName}</RenderedText>}
          labelElement={<RenderedText size={12}>{p.group}</RenderedText>}
          popoverProps={{
            interactionKind: PopoverInteractionKind.CLICK,
          }}
        >
          <MenuItem text="TP" onClick={tpOnClick(p)} />
          <MenuItem text="Child two" />
          <MenuItem text="Child three" />
        </MenuItem>
      ))}
    </Menu>
  );
}
