export type MancalaTurn = 0 | 1 | null;
export type Pit = 1 | 2 | 3 | 4 | 5 | 6;
export type Hint = {
  value: number;
  hintType: number;
  min: number;
  max: number;
};
export type SimulationResult = {
  hints: Hint[];
  min: number;
  max: number;
  ends: number;
};
export type HintData = {
  hints: Hint[];
  min: number;
  max: number;
};
