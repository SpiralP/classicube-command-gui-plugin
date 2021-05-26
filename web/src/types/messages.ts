export interface ChatCommandJsonMessage {
  type: "chatCommand";
  data: string;
}
export interface TabListSubscribeJsonMessage {
  type: "tabListSubscribe";
}
export interface AskColorCodesJsonMessage {
  type: "askColorCodes";
}
export interface RenderTextJsonMessage {
  type: "renderText";
  data: {
    text: string;
    size: number;
    shadow: boolean;
  };
}
export interface AskRanksJsonMessage {
  type: "askRanks";
}
export interface AskBlocksJsonMessage {
  type: "askBlocks";
}
export interface AskBlockPropertiesJsonMessage {
  type: "askBlockProperties";
  /**
   * id or name
   */
  data: string;
}

export type JsonMessage =
  | ChatCommandJsonMessage
  | TabListSubscribeJsonMessage
  | AskColorCodesJsonMessage
  | RenderTextJsonMessage
  | AskRanksJsonMessage
  | AskBlocksJsonMessage
  | AskBlockPropertiesJsonMessage;
