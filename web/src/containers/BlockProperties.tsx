import { Divider, H5, Spinner, Tag } from "@blueprintjs/core";
import React, { useEffect, useState } from "react";
import { useInView } from "react-intersection-observer";
import { useConnection } from "../Connection";
import { JsonBlockProperties, JsonEvent } from "../types";

function useBlockProperties({ id, inView }: { id: string; inView: boolean }) {
  const connection = useConnection();
  const [blockProperties, setBlockProperties] =
    useState<JsonBlockProperties | undefined>(undefined);

  useEffect(() => {
    if (!inView) return undefined;

    console.log("fetching blockProperties", id);
    function listener(obj: JsonEvent) {
      if (obj.type === "blockProperties" && obj.data.id === id) {
        setBlockProperties(obj.data);
      }
    }
    connection.addListener(listener);
    connection.send({ type: "askBlockProperties", data: id });

    return () => {
      connection.removeListener(listener);
    };
  }, [connection, id, inView]);

  return blockProperties;
}

export function BlockProperties({ id }: { id: string }) {
  const { ref, inView } = useInView();

  const p = useBlockProperties({ id, inView });

  let el = null;
  if (p) {
    const basicItems: Array<[string, any]> = [
      ["deathMessage", p.basic.deathMessage],
      ["killerBlock", p.basic.killerBlock],
      ["isTDoor", p.basic.isTDoor],
      ["isDoor", p.basic.isDoor],
      ["oDoorBlock", p.basic.oDoorBlock],
      ["isMessageBlock", p.basic.isMessageBlock],
      ["isPortal", p.basic.isPortal],
      ["waterKills", p.basic.waterKills],
      ["lavaKills", p.basic.lavaKills],
      ["opBlock", p.basic.opBlock],
      ["isRails", p.basic.isRails],
      ["animalAi", p.basic.animalAi],
      ["stackBlock", p.basic.stackBlock],
      ["drownable", p.basic.drownable],
      ["grassBlock", p.basic.grassBlock],
      ["dirtBlock", p.basic.dirtBlock],
    ];

    const basicEls = basicItems
      .filter(([, value]) => value)
      .map(([label, value]) => (
        <Tag
          key={label}
          rightIcon={
            typeof value === "string" ? <span>{value}</span> : undefined
          }
        >
          {label}
        </Tag>
      ));

    const complexItems: Array<[string, any]> = [
      ["basicBlockName", p.complex.basicBlockName],
      ["lightPass", p.complex.lightPass],
      ["needRestart", p.complex.needRestart],
      ["physics", p.complex.physics],
      ["allowBreak", p.complex.allowBreak],
      ["walkthrough", p.complex.walkthrough],
      ["walkthroughActivated", p.complex.walkthroughActivated],
    ];

    const complexEls = complexItems
      .filter(([, value]) => value)
      .map(([label]) => <Tag key={label}>{label}</Tag>);

    if (!basicEls.length && !complexEls.length) {
      el = <H5>None</H5>;
    } else {
      el = (
        <>
          {basicEls.length ? (
            <>
              <H5>Basic</H5>
              {basicEls}
            </>
          ) : null}
          {basicEls.length && complexEls.length ? <Divider /> : null}
          {complexEls.length ? (
            <>
              <H5>Complex</H5>
              {complexEls}
            </>
          ) : null}
        </>
      );
    }
  } else if (inView) {
    el = <Spinner />;
  }

  return <div ref={ref}>{el}</div>;
}
