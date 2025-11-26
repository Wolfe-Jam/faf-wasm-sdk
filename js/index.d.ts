// FAF WASM SDK - TypeScript Definitions

export function initialize(): Promise<void>;
export function createFAF(yamlContent: string): FAF;
export function isValidFAF(yamlContent: string): boolean;
export function scoreFAF(yamlContent: string): FafScoreJson;
export function version(): string;

export function init(): Promise<void>;
export function score_weights(weights: Float32Array, base: number): number;
export function score_weights_fast(weights: Float32Array, values: Float32Array): number;
export const WEIGHTS_F32: Float32Array;

export interface FafScoreJson {
  completeness: number;
  clarity: number;
  structure: number;
  metadata: number;
  weighted: number;
  truth: number;
  tier: string;
  truth_tier: string;
}

export class FAF {
  constructor(yamlContent: string);
  
  readonly name: string | null;
  readonly stack: string | null;
  readonly weighted_score: number;
  readonly truth_score: number;
  readonly tier: string;
  readonly completeness: number;
  readonly clarity: number;
  readonly structure: number;
  readonly metadata: number;
  
  score_with_bonus(language: string): number;
  display(): string;
  dna_json(): string;
  score_json(): string;
  
  static validate(yamlContent: string): boolean;
  static version(): string;
}
