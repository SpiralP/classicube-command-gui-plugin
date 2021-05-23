export interface ConnectionArgs {
  port: number;
  path: string;
}

export interface JsonPlayer {
  id: number;
  realName: string;
  nickName: string;
  group: string;
  rank: number;
}

export interface ColorCode {
  char: string;
  color: string;
}
