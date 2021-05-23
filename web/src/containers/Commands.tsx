import { Card, Spinner } from "@blueprintjs/core";
import React, { useEffect, useState } from "react";
import { Connection } from "../Connection";

export function Commands({ connection }: { connection?: Connection }) {
  const commands = useCommands();

  return (
    <>
      {commands ? (
        <div>
          {Object.entries(commands).map(([name, info]) => (
            <Card interactive>
              <h5>{name}</h5>
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
      const commands: Record<string, CommandInfo> = await response.json();

      if (cancel) return;

      setCommands(commands);
    })();

    return () => {
      cancel = true;
    };
  }, []);

  return commands;
}
