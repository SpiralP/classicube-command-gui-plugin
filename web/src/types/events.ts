import { ColorCode, JsonPlayer } from "./other";

interface NewPlayersJsonEvent {
  type: "newPlayers";
  data: JsonPlayer[];
}
interface PlayerAddedJsonEvent {
  type: "playerAdded";
  data: JsonPlayer;
}
interface PlayerRemovedJsonEvent {
  type: "playerRemoved";
  data: {
    id: number;
  };
}
interface PlayerChangedJsonEvent {
  type: "playerChanged";
  data: JsonPlayer;
}
interface WeDisconnectedJsonEvent {
  type: "weDisconnected";
}
interface ColorCodesJsonEvent {
  type: "colorCodes";
  data: ColorCode[];
}
interface RenderedTextJsonEvent {
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

export type JsonEvent =
  | NewPlayersJsonEvent
  | PlayerAddedJsonEvent
  | PlayerRemovedJsonEvent
  | PlayerChangedJsonEvent
  | WeDisconnectedJsonEvent
  | ColorCodesJsonEvent
  | RenderedTextJsonEvent;
