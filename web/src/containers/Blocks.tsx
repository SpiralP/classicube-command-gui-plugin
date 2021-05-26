import { Button, Card, H5, Spinner, Switch } from "@blueprintjs/core";
import { Popover2 } from "@blueprintjs/popover2";
import React, { useEffect, useState } from "react";
import { useInView } from "react-intersection-observer";
import { useConnection } from "../Connection";
import {
  CollideType,
  DrawType,
  JsonBlock,
  JsonEvent,
  SoundType,
} from "../types";
import { BlockProperties } from "./BlockProperties";
import { SelectEnumItems } from "./SelectEnumItems";

export function useBlocks() {
  const connection = useConnection();

  const [blocks, setBlocks] = useState<JsonBlock[] | undefined>(undefined);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    function listener(obj: JsonEvent) {
      if (obj.type === "blocks") {
        setBlocks(obj.data);
      }
    }
    connection.addListener(listener);
    connection.send({ type: "askBlocks" });

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

const SoundTypeEnumItems = [
  { label: "None", value: SoundType.SOUND_NONE },
  { label: "Wood", value: SoundType.SOUND_WOOD },
  { label: "Gravel", value: SoundType.SOUND_GRAVEL },
  { label: "Grass", value: SoundType.SOUND_GRASS },
  { label: "Stone", value: SoundType.SOUND_STONE },
  { label: "Metal", value: SoundType.SOUND_METAL },
  { label: "Glass", value: SoundType.SOUND_GLASS },
  { label: "Cloth", value: SoundType.SOUND_CLOTH },
  { label: "Sand", value: SoundType.SOUND_SAND },
  { label: "Snow", value: SoundType.SOUND_SNOW },
];

const DrawTypeEnumItems = [
  { label: "Opaque", value: DrawType.DRAW_OPAQUE },
  { label: "Transparent", value: DrawType.DRAW_TRANSPARENT },
  {
    label: "Transparent_thick",
    value: DrawType.DRAW_TRANSPARENT_THICK,
  },
  { label: "Translucent", value: DrawType.DRAW_TRANSLUCENT },
  { label: "Gas", value: DrawType.DRAW_GAS },
  { label: "Sprite", value: DrawType.DRAW_SPRITE },
];

const CollideTypeEnumItems = [
  { label: "Gas", value: CollideType.COLLIDE_GAS },
  { label: "Liquid", value: CollideType.COLLIDE_LIQUID },
  { label: "Solid", value: CollideType.COLLIDE_SOLID },
  { label: "Ice", value: CollideType.COLLIDE_ICE },
  {
    label: "Slippery ice",
    value: CollideType.COLLIDE_SLIPPERY_ICE,
  },
  {
    label: "Liquid water",
    value: CollideType.COLLIDE_LIQUID_WATER,
  },
  {
    label: "Liquid lava",
    value: CollideType.COLLIDE_LIQUID_LAVA,
  },
  { label: "Climb rope", value: CollideType.COLLIDE_CLIMB_ROPE },
];

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

  const { ref: containerRef, inView } = useInView();

  let el = null;
  if (inView) {
    el = (
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
        <SelectEnumItems<SoundType>
          text="digSounds"
          value={block.digSounds}
          enumItems={SoundTypeEnumItems}
          onItemSelect={({ value }) => {
            changeBlockProp(block.id, "digSounds", value);
          }}
          disabled={disabled}
        />
        <SelectEnumItems<SoundType>
          text="stepSounds"
          value={block.stepSounds}
          enumItems={SoundTypeEnumItems}
          onItemSelect={({ value }) => {
            changeBlockProp(block.id, "stepSounds", value);
          }}
          disabled={disabled}
        />
        <SelectEnumItems<DrawType>
          text="draw"
          value={block.draw}
          enumItems={DrawTypeEnumItems}
          onItemSelect={({ value }) => {
            changeBlockProp(block.id, "draw", value);
          }}
          disabled={disabled}
        />
        <SelectEnumItems<CollideType>
          text="collide"
          value={block.collide}
          enumItems={CollideTypeEnumItems}
          onItemSelect={({ value }) => {
            changeBlockProp(block.id, "collide", value);
          }}
          disabled={disabled}
        />
        <Popover2
          interactionKind="click"
          content={
            <div>
              <BlockProperties id={`${block.id}`} />
            </div>
          }
          renderTarget={({ isOpen, ref, ...targetProps }) => (
            <Button
              // eslint-disable-next-line react/jsx-props-no-spreading
              {...targetProps}
              elementRef={ref || undefined}
              text="Properties"
            />
          )}
        />
      </>
    );
  }

  return (
    <Card interactive>
      <div ref={containerRef}>{el}</div>
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
