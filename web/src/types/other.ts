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

export interface JsonRank {
  colorCode: string;
  rankName: string;
  drawLimit: number;
  permission: number;
  maxRealms: number;
}

export interface JsonBlock {
  id: number;
  name: string;
  isLiquid: boolean;
  blocksLight: boolean;
  fullBright: boolean;
  fogCol: JsonColor;
  fogDensity: number;
  collide: number;
  extendedCollide: number;
  speedMultiplier: number;
  lightOffset: number;
  draw: number;
  digSounds: number;
  stepSounds: number;
  tinted: boolean;
  fullOpaque: boolean;
  spriteOffset: number;
  // TODO is this correct camel case?
  minBb: JsonVec3;
  maxBb: JsonVec3;
  renderMinBb: JsonVec3;
  renderMaxBb: JsonVec3;
  /* Texture ids of each face of blocks. */
  // textures: [TextureLoc; 4608],
  canPlace: boolean;
  canDelete: boolean;
  /* Bit flags of faces hidden of two neighbouring blocks. */
  // hidden: [number; 589824],
  canStretch: number;
}

export interface JsonVec3 {
  x: number;
  y: number;
  z: number;
}

export interface JsonColor {
  r: number;
  g: number;
  b: number;
  a: number;
}
