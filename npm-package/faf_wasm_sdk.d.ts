/* tslint:disable */
/* eslint-disable */
/**
 * Standalone validate function
 */
export function validate_faf(yaml_content: string): boolean;
/**
 * Get SDK version
 */
export function sdk_version(): string;
/**
 * Standalone score function - returns JSON
 */
export function score_faf(yaml_content: string): string;
export function score_weights_fast(weights: Float32Array, values: Float32Array): number;
export function score_weights(weights: Float32Array, base: number): number;
/**
 * FAF - Main entry point for WASM
 */
export class FAF {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Export score as JSON
   */
  score_json(): string;
  /**
   * Get Mk3 display string
   */
  mk3_display(): string;
  /**
   * Get Mk3 breakdown string
   */
  mk3_breakdown(): string;
  /**
   * Get score with language bonus
   */
  score_with_bonus(language: string): number;
  /**
   * Create FAF from YAML content
   */
  constructor(yaml_content: string);
  /**
   * Get display string
   */
  display(): string;
  /**
   * Get version
   */
  static version(): string;
  /**
   * Validate FAF content (returns true if valid)
   */
  static validate(yaml_content: string): boolean;
  /**
   * Get Mk3 filled slots count
   */
  readonly mk3_filled: number;
  /**
   * Get project name
   */
  readonly name: string | undefined;
  /**
   * Get project stack
   */
  readonly stack: string | undefined;
  /**
   * Get Mk3 tier emoji
   */
  readonly mk3_tier: string;
  /**
   * Get Mk3 slot-based score (0-100)
   */
  readonly mk3_score: number;
  /**
   * Get Mk3 total slots count
   */
  readonly mk3_total: number;
}
/**
 * Mk3 Compiler Engine Tier System (OFFICIAL - DO NOT CHANGE)
 * 100%: Championship | 99%+: Gold | 95%+: Silver | 85%+: Bronze
 * 70%+: Green | 55%+: Yellow | <55%: Red
 * FAF Score result - fully transparent
 */
export class FafScore {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Get truth tier emoji (Mk3 Compiler Engine - OFFICIAL)
   */
  truth_tier(): string;
  /**
   * Apply language bonus
   */
  with_bonus(language: string): number;
  /**
   * Create new score
   */
  static new(completeness: number, clarity: number, structure: number, metadata: number): FafScore;
  /**
   * Get tier emoji (Mk3 Compiler Engine - OFFICIAL)
   */
  tier(): string;
  /**
   * Get truth score (unweighted average)
   */
  truth(): number;
  /**
   * Get full display string
   */
  display(): string;
  /**
   * Export as JSON for JS
   */
  to_json(): string;
  /**
   * Calculate weighted score
   */
  weighted(): number;
  /**
   * Get completeness score (0-100)
   */
  readonly completeness: number;
  /**
   * Get clarity score (0-100)
   */
  readonly clarity: number;
  /**
   * Get metadata score (0-100)
   */
  readonly metadata: number;
  /**
   * Get structure score (0-100)
   */
  readonly structure: number;
}
/**
 * Mk3 Score result - slot-based (filled/total)
 */
export class Mk3Score {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Display string
   */
  display(): string;
  /**
   * Get breakdown as string
   */
  breakdown(): string;
  /**
   * Get tier emoji (Mk3 official)
   */
  readonly tier: string;
  /**
   * Get percentage score (0-100)
   */
  readonly score: number;
  /**
   * Get total slot count
   */
  readonly total: number;
  /**
   * Get filled slot count
   */
  readonly filled: number;
}
