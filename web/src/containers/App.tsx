/* eslint-disable react/jsx-no-bind */

import { Button, NonIdealState } from "@blueprintjs/core";
import React, { useContext, useState } from "react";
import { ConnectionContext, useSetupConnection } from "../Connection";
import { ConnectionArgs } from "../types";
import { Blocks } from "./Blocks";
import { Commands } from "./Commands";
import { TabList } from "./TabList";

function Body() {
  const [mode, setMode] = useState<"none" | "commands" | "blocks">("none");
  const connection = useContext(ConnectionContext);

  let mainElement = null;
  if (mode === "commands") {
    mainElement = <Commands />;
  } else if (mode === "blocks") {
    mainElement = <Blocks />;
  }

  return (
    <>
      <Button
        text="Show commands"
        onClick={() => {
          setMode("commands");
        }}
      />
      <Button
        text="Show blocks"
        disabled={connection == null}
        onClick={() => {
          setMode("blocks");
        }}
      />

      {mainElement}
    </>
  );
}

export function App({ connectionArgs }: { connectionArgs?: ConnectionArgs }) {
  const connection = useSetupConnection(connectionArgs);

  return (
    <ConnectionContext.Provider value={connection}>
      <div
        style={{
          width: "100%",
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-between",
        }}
      >
        <div
          style={{
            minWidth: "500px",
          }}
        >
          <Body />
        </div>
        <div
          style={{
            width: "400px",
          }}
        >
          {connection ? (
            <TabList />
          ) : (
            <>
              <style>
                {`
                  .non-ideal {
                    justify-content: normal;
                  }
                `}
              </style>
              <NonIdealState
                className="non-ideal"
                title="No connection"
                icon="offline"
              />
            </>
          )}
        </div>
      </div>
    </ConnectionContext.Provider>
  );
}
