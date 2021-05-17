interface ConnectionArgs {
  port: number;
  path: string;
}

interface ChatCommandJsonMessage {
  type: "chatCommand";
  data: string;
}

type JsonMessage = ChatCommandJsonMessage;
