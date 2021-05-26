import { Card, H5, Spinner } from "@blueprintjs/core";
import React, { useEffect, useState } from "react";
import { RenderedText } from "../components/RenderedText";
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

function CommandCard({ command }: { command: CommandInfo }) {
  const { name, help } = command;

  return (
    <Card interactive>
      <H5>
        <RenderedText shadow={false}>{name}</RenderedText>
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
  const commands = useCommands();

  return (
    <>
      {commands ? (
        Object.values(commands).map((command) => (
          <CommandCard key={command.name} command={command} />
        ))
      ) : (
        <Spinner />
      )}
    </>
  );
}
