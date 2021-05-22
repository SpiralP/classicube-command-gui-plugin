interface ConnectionArgs {
  port: number;
  path: string;
}

interface ChatCommandJsonMessage {
  type: "chatCommand";
  data: string;
}
interface TabListSubscribeJsonMessage {
  type: "tabListSubscribe";
}
interface AskColorCodesJsonMessage {
  type: "askColorCodes";
}
interface RenderTextJsonMessage {
  type: "renderText";
  data: string;
}

type JsonMessage =
  | ChatCommandJsonMessage
  | TabListSubscribeJsonMessage
  | AskColorCodesJsonMessage
  | RenderTextJsonMessage;

interface JsonPlayer {
  id: number;
  realName: string;
  nickName: string;
  group: string;
  rank: number;
}

interface ColorCode {
  char: string;
  color: string;
}

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
    /**
     * R G B A order
     */
    pixels: number[];
    width: number;
    height: number;
  };
}

type JsonEvent =
  | NewPlayersJsonEvent
  | PlayerAddedJsonEvent
  | PlayerRemovedJsonEvent
  | PlayerChangedJsonEvent
  | WeDisconnectedJsonEvent
  | ColorCodesJsonEvent
  | RenderedTextJsonEvent;
