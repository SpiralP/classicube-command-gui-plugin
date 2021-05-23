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
