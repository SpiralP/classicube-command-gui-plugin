import { NonIdealState } from "@blueprintjs/core";
import React from "react";
import { ConnectionContext, useSetupConnection } from "../Connection";
import { ConnectionArgs } from "../types";
import { Commands } from "./Commands";
import { TabList } from "./TabList";

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
          <Commands />
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
