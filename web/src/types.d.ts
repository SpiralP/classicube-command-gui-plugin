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

type JsonMessage = ChatCommandJsonMessage | TabListSubscribeJsonMessage;

interface JsonPlayer {
  id: number;
  realName: string;
  nickName: string;
  group: string;
  rank: number;
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
  /**
   * id
   */
  data: number;
}
interface PlayerChangedJsonEvent {
  type: "playerChanged";
  data: JsonPlayer;
}
interface WeDisconnectedJsonEvent {
  type: "weDisconnected";
}

type JsonEvent =
  | NewPlayersJsonEvent
  | PlayerAddedJsonEvent
  | PlayerRemovedJsonEvent
  | PlayerChangedJsonEvent
  | WeDisconnectedJsonEvent;
