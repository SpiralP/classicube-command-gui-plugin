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

export type JsonMessage =
  | ChatCommandJsonMessage
  | TabListSubscribeJsonMessage
  | AskColorCodesJsonMessage
  | RenderTextJsonMessage;