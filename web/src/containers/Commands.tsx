import { Card, H5, Spinner } from "@blueprintjs/core";
import React, { useEffect, useState } from "react";
import { Connection } from "../Connection";

interface CommandInfo {
  type: string;
  shortcut: string;
  defaultRank: string;
  help: string[];
}

function useCommands() {
  const [commands, setCommands] =
    useState<Record<string, CommandInfo> | undefined>(undefined);

  useEffect(() => {
    let cancel = false;

    (async () => {
      const response = await fetch("/commands.json");
      const obj: Record<string, CommandInfo> = await response.json();

      if (cancel) return;

      setCommands(obj);
    })();

    return () => {
      cancel = true;
    };
  }, []);

  return commands;
}

export function Commands({ connection }: { connection?: Connection }) {
  const commands = useCommands();

  connection?.addListener(() => {
    //
  });

  return (
    <>
      {commands ? (
        <div>
          {Object.entries(commands).map(([name, info]) => (
            <Card interactive key={name}>
              <H5>{name}</H5>
              <p>{info.type}</p>
            </Card>
          ))}
        </div>
      ) : (
        <Spinner />
      )}
    </>
  );
}
