import { getPlayableTiles } from './PlayableGameController';


function getRandomMove(playableTiles) {
  return playableTiles[Math.floor(Math.random() * playableTiles.length)];
}

export default class RandomAgent {
  getNextMove(state) {
    return getRandomMove(getPlayableTiles(state));
  }

  reset() {
    return;
  }
}
