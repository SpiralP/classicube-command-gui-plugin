import { NonIdealState } from "@blueprintjs/core";
import React from "react";
import { useConnection } from "../Connection";
import { Commands } from "./Commands";
import { TabList } from "./TabList";

export default function App({
  connectionArgs,
}: {
  connectionArgs?: ConnectionArgs;
}) {
  const connection = useConnection(connectionArgs);

  return (
    <div>
      <Commands connection={connection} />
      {connection ? (
        <>
          <TabList connection={connection} />
        </>
      ) : (
        <NonIdealState title="No connection" icon="offline" />
      )}
    </div>
  );
}
