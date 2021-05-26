import { Card, H5, Spinner, Switch } from "@blueprintjs/core";
import React, { useEffect, useState } from "react";
import { useInView } from "react-intersection-observer";
import { useConnection } from "../Connection";
import { JsonBlock, JsonEvent } from "../types";

export function useBlocks() {
  const connection = useConnection();

  const [blocks, setBlocks] = useState<JsonBlock[] | undefined>(undefined);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    console.log("fetching");
    function listener(obj: JsonEvent) {
      if (obj.type === "blocks") {
        setBlocks(obj.data);
        setLoading(false);
      }
    }
    connection.addListener(listener);
    connection.send({ type: "askBlocks" });
    setLoading(true);

    return () => {
      connection.removeListener(listener);
    };
  }, [connection]);

  return {
    blocks,
    loading,
    changeBlockProp: (blockId: number, propName: string, value: any) => {
      setLoading(true);

      // TODO
      console.log("changing", blockId, propName, value);

      setTimeout(() => {
        setBlocks((prev) =>
          prev
            ? [
                ...prev.map((block) =>
                  block.id === blockId ? { ...block, [propName]: value } : block
                ),
              ]
            : undefined
        );
        setLoading(false);
      }, 1000);
    },
  };
}

function BlockCard({
  block,
  disabled,
  changeBlockProp,
}: {
  block: JsonBlock;
  disabled: boolean;
  changeBlockProp: (blockId: number, propName: string, value: any) => void;
}) {
  const booleans: Array<[string, boolean]> = [
    ["isLiquid", block.isLiquid],
    ["blocksLight", block.blocksLight],
    ["fullBright", block.fullBright],
    ["tinted", block.tinted],
    ["fullOpaque", block.fullOpaque],
    ["canPlace", block.canPlace],
    ["canDelete", block.canDelete],
  ];

  const { ref, inView } = useInView();

  return (
    <Card interactive>
      <div ref={ref}>
        {inView ? (
          <>
            <H5>{`${block.name} ${block.id}`}</H5>
            {booleans.map(([propName, value]) => (
              <Switch
                disabled={disabled}
                inline
                key={propName}
                checked={value}
                innerLabel={propName}
                large
                onChange={(event) => {
                  const { checked } = event.target as HTMLInputElement;
                  changeBlockProp(block.id, propName, checked);
                }}
              />
            ))}
          </>
        ) : null}
      </div>
    </Card>
  );
}

export function Blocks() {
  const { blocks, loading, changeBlockProp } = useBlocks();

  return (
    <>
      {blocks ? (
        blocks.map((block) => (
          <BlockCard
            key={block.id}
            block={block}
            disabled={loading}
            changeBlockProp={changeBlockProp}
          />
        ))
      ) : (
        <Spinner />
      )}
    </>
  );
}
