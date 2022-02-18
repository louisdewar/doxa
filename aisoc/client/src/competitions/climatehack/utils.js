

export function roundScore(score) {
  return Math.round((score + Number.EPSILON) * 100000) / 100000;
}
