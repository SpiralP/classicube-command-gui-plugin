import { H5, Menu, MenuItem, PopoverInteractionKind } from "@blueprintjs/core";
import React, { useContext, useEffect, useState } from "react";
import { RenderedText } from "../components/RenderedText";
import { Connection, ConnectionContext } from "../Connection";
import { JsonEvent, JsonPlayer, Rank } from "../types";

export function usePlayers({ connection }: { connection: Connection }) {
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

  const [ranks, setRanks] = useState<Rank[]>([]);

  useEffect(() => {
    function listener(obj: JsonEvent) {
      if (obj.type === "ranks") {
        setRanks(obj.data);
      }
    }
    connection.addListener(listener);

    return () => {
      connection.removeListener(listener);
    };
  }, [connection]);

  useEffect(() => {
    connection.send({ type: "askRanks" });
  }, [connection]);

  return { players, colorCodes, ranks };
}

export function TabList() {
  const connection = useContext(ConnectionContext);
  if (!connection) throw new Error("!connection");

  const { players, ranks } = usePlayers({ connection });

  return (
    <Menu>
      {Object.entries(players).map(([id, p]) => (
        <PlayerMenuItem
          key={id}
          player={p}
          ranks={ranks}
          connection={connection}
        />
      ))}
    </Menu>
  );
}

function PlayerMenuItem({
  player: p,
  ranks,
  connection,
}: {
  player: JsonPlayer;
  ranks: Rank[];
  connection: Connection;
}) {
  const rank = ranks.find((r) => r.permission === 120 - p.rank);
  const tpOnClick = () => () => {
    connection.send({
      type: "chatCommand",
      data: `TP ${p.realName}`,
    });
  };

  return (
    <MenuItem
      title={p.realName}
      text={<RenderedText>{p.nickName}</RenderedText>}
      labelElement={
        <RenderedText size={12} shadow={false}>
          {p.group}
        </RenderedText>
      }
      popoverProps={{
        interactionKind: PopoverInteractionKind.CLICK,
      }}
    >
      <H5>
        {rank ? (
          <RenderedText>{`&${rank.colorCode}${rank.rankName}`}</RenderedText>
        ) : null}
      </H5>
      <MenuItem text="TP" onClick={tpOnClick()} />
      <MenuItem text="Child two" />
      <MenuItem text="Child three" />
    </MenuItem>
  );
}
