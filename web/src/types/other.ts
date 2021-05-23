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

export interface CommandInfo {
  name: string;
  type: string;
  shortcut: string | null;
  defaultRank: number;
  help: string[];
  aliases: CommandInfoAlias[];
  extraPerms: CommandInfoPerm[];
}

export interface CommandInfoAlias {
  trigger: string;
  format: string;
}

export interface CommandInfoPerm {
  perm: string;
  description: string;
}
