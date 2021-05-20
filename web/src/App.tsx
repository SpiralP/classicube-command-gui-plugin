import { MenuItem, useHotkeys } from "@blueprintjs/core";
import { Omnibar } from "@blueprintjs/select";
import React, { useEffect, useMemo, useState } from "react";
import { TabList } from "./TabList";

interface CommandInfo {
  type: string;
  shortcut: string;
  defaultRank: string;
  help: string[];
}

interface CommandInfoWithName extends CommandInfo {
  name: string;
}

const CommandOmnibar = Omnibar.ofType<CommandInfoWithName>();

export default function App({
  connectionArgs,
}: {
  connectionArgs?: ConnectionArgs;
}) {
  const commands = useCommands();
  const connection = useConnection(connectionArgs);
  const [showOmnibar, setShowOmnibar] = useState(false);
  useHotkeys(
    useMemo(
      () => [
        {
          label: "Show omnibar",
          combo: "shift",
          global: true,
          onKeyDown: () => {
            setShowOmnibar(true);
          },
        },
      ],
      []
    )
  );

  return (
    <div>
      {commands ? (
        <CommandOmnibar
          isOpen={showOmnibar}
          items={Object.entries(commands).map(
            ([name, info]) => ({ ...info, name } as CommandInfoWithName)
          )}
          itemRenderer={(command, { handleClick, modifiers, query }) => {
            if (!modifiers.matchesPredicate) {
              return null;
            }

            return (
              <MenuItem
                active={modifiers.active}
                disabled={modifiers.disabled}
                key={command.name}
                onClick={handleClick}
                text={command.name}
                label={command.help.join(" ").slice(0, 30)}
              />
            );
          }}
          onItemSelect={(item) => {
            //
          }}
        />
      ) : null}
      {connection ? (
        <>
          <h1>connected!</h1>
          <TabList connection={connection} />
        </>
      ) : (
        "no connection"
      )}
    </div>
  );
}

function useConnection(connectionArgs?: ConnectionArgs) {
  const [connection, setConnection] =
    useState<WebSocket | undefined>(undefined);

  useEffect(() => {
    if (!connectionArgs) return;

    const connection = new WebSocket(
      `ws://127.0.0.1:${connectionArgs.port}/${connectionArgs.path}`
    );
    connection.addEventListener("open", () => {
      console.log("connected!");

      setConnection(connection);
    });
    connection.addEventListener("message", ({ data }) => {
      if (typeof data !== "string") return;

      const obj = JSON.parse(data);
      console.log(obj);
    });

    connection.addEventListener("close", () => {
      window.close();
    });
    connection.addEventListener("error", () => {
      window.close();
    });

    return () => {
      connection.close();
      setConnection(undefined);
    };
  }, []);

  return connection;
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
