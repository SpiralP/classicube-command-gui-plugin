import { Card, H5, Spinner } from "@blueprintjs/core";
import React, { useContext, useEffect, useState } from "react";
import { RenderedText } from "../components/RenderedText";
import { ConnectionContext } from "../Connection";
import { CommandInfo } from "../types";

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

function CommandCard({ command }: { command: CommandInfo & { name: string } }) {
  const { name, help } = command;

  return (
    <Card interactive key={name}>
      <H5>
        <RenderedText>{name}</RenderedText>
      </H5>

      {help.map((line, i) => (
        // eslint-disable-next-line react/no-array-index-key
        <p key={i} style={{ margin: 0, overflow: "hidden" }}>
          <RenderedText shadow>{line}</RenderedText>
        </p>
      ))}
    </Card>
  );
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
          <CommandCard key={name} command={{ ...info, name }} />
        ))
      ) : (
        <Spinner />
      )}
    </>
  );
}
