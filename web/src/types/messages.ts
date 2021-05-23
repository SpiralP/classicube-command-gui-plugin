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

export type JsonMessage =
  | ChatCommandJsonMessage
  | TabListSubscribeJsonMessage
  | AskColorCodesJsonMessage
  | RenderTextJsonMessage;
