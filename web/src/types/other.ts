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
  collide: CollideType;
  extendedCollide: number;
  speedMultiplier: number;
  lightOffset: number;
  draw: DrawType;
  digSounds: SoundType;
  stepSounds: SoundType;
  tinted: boolean;
  fullOpaque: boolean;
  spriteOffset: number;
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

export enum SoundType {
  SOUND_NONE,
  SOUND_WOOD,
  SOUND_GRAVEL,
  SOUND_GRASS,
  SOUND_STONE,
  SOUND_METAL,
  SOUND_GLASS,
  SOUND_CLOTH,
  SOUND_SAND,
  SOUND_SNOW,
}

/* Describes how a block is rendered in the world. */
export enum DrawType {
  DRAW_OPAQUE /* Completely covers blocks behind (e.g. dirt). */,
  DRAW_TRANSPARENT /* Blocks behind show (e.g. glass). Pixels either fully visible or invisible. */,
  DRAW_TRANSPARENT_THICK /* Same as Transparent, but all neighbour faces show. (e.g. leaves) */,
  DRAW_TRANSLUCENT /* Blocks behind show (e.g. water). Pixels blend with other blocks behind. */,
  DRAW_GAS /* Does not show (e.g. air). Can still be collided with. */,
  DRAW_SPRITE /* Block renders as an X (e.g. sapling). Pixels either fully visible or invisible. */,
}

/* Describes the interaction a block has with a player when they collide with it. */
export enum CollideType {
  COLLIDE_GAS /* No interaction when player collides. */,
  COLLIDE_LIQUID /* 'swimming'/'bobbing' interaction when player collides. */,
  COLLIDE_SOLID /* Block completely stops the player when they are moving. */,
  COLLIDE_ICE /* Block is solid and partially slidable on. */,
  COLLIDE_SLIPPERY_ICE /* Block is solid and fully slidable on. */,
  COLLIDE_LIQUID_WATER /* Water style 'swimming'/'bobbing' interaction when player collides. */,
  COLLIDE_LIQUID_LAVA /* Lava style 'swimming'/'bobbing' interaction when player collides. */,
  COLLIDE_CLIMB_ROPE /* Rope/Ladder style climbing interaction when player collides. */,
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

export interface JsonBlockProperties {
  basic: BasicBlockProperties;
  complex: ComplexBlockProperties;
}

export interface BasicBlockProperties {
  deathMessage: string | null;
  killerBlock: boolean;
  isTDoor: boolean;
  isDoor: boolean;
  oDoorBlock: boolean;
  isMessageBlock: boolean;
  isPortal: boolean;
  waterKills: boolean;
  lavaKills: boolean;
  opBlock: boolean;
  isRails: boolean;
  animalAi: AnimalAI | null;
  stackBlock: string | null;
  drownable: boolean;
  grassBlock: string | null;
  dirtBlock: string | null;
}

export enum AnimalAI {
  None,
  Fly,
  FleeAir,
  KillerAir,
  FleeWater,
  KillerWater,
  FleeLava,
  KillerLava,
}

export interface ComplexBlockProperties {
  basicBlockName: string | null;
  lightPass: boolean;
  needRestart: boolean;
  physics: boolean;
  allowBreak: boolean;
  walkthrough: boolean;
  walkthroughActivated: boolean;
}
