import {
  ColorCode,
  JsonBlock,
  JsonBlockProperties,
  JsonPlayer,
  JsonRank,
} from "./other";

export interface NewPlayersJsonEvent {
  type: "newPlayers";
  data: JsonPlayer[];
}
export interface PlayerAddedJsonEvent {
  type: "playerAdded";
  data: JsonPlayer;
}
export interface PlayerRemovedJsonEvent {
  type: "playerRemoved";
  data: {
    id: number;
  };
}
export interface PlayerChangedJsonEvent {
  type: "playerChanged";
  data: JsonPlayer;
}
export interface WeDisconnectedJsonEvent {
  type: "weDisconnected";
}
export interface ColorCodesJsonEvent {
  type: "colorCodes";
  data: ColorCode[];
}
export interface RenderedTextJsonEvent {
  type: "renderedText";
  data: {
    text: string;
    size: number;
    shadow: boolean;
    /**
     * R G B A order
     */
    pixels: number[];
    width: number;
    height: number;
  };
}
export interface RanksJsonMessage {
  type: "ranks";
  data: JsonRank[];
}
export interface BlocksJsonMessage {
  type: "blocks";
  data: JsonBlock[];
}
export interface BlockPropertiesJsonMessage {
  type: "blockProperties";
  data: JsonBlockProperties;
}

export type JsonEvent =
  | NewPlayersJsonEvent
  | PlayerAddedJsonEvent
  | PlayerRemovedJsonEvent
  | PlayerChangedJsonEvent
  | WeDisconnectedJsonEvent
  | ColorCodesJsonEvent
  | RenderedTextJsonEvent
  | RanksJsonMessage
  | BlocksJsonMessage;
