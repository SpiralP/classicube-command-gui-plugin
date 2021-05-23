import { Card, H5, Spinner } from "@blueprintjs/core";
import React, { useContext, useEffect, useState } from "react";
import { RenderedText } from "../components/RenderedText";
import { ConnectionContext } from "../Connection";

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

export function Commands() {
  const connection = useContext(ConnectionContext);
  const commands = useCommands();

  connection?.addListener(() => {
    //
  });

  return (
    <>
      {commands ? (
        Object.entries(commands).map(([name, info]) => (
          <Card interactive key={name}>
            <H5>
              <RenderedText>{name}</RenderedText>
            </H5>
            <p>
              <RenderedText>{info.help[0]}</RenderedText>
            </p>
          </Card>
        ))
      ) : (
        <Spinner />
      )}
    </>
  );
}
