import GameState from 'competitions/uttt/components/Grid/GameState';


export function getPlayableTiles(state) {
  if (state.winner !== null) return [];

  return (state.nextGrid != null ? [state.nextGrid] : [...Array(9).keys()].filter(grid => !state.subGridsWon[grid]))
    .flatMap(grid => [...Array(9).keys()]
      .filter(tile => state.subGrids[grid][tile] === null)
      .map(tile => ({ g: grid, t: tile })));
}

function isBoardWon(subGrid) {
  return {
    // Rows
    [[subGrid[0], subGrid[1], subGrid[2]].every((e, _, a) => a[0] && e == a[0])]: subGrid[0],
    [[subGrid[3], subGrid[4], subGrid[5]].every((e, _, a) => a[0] && e == a[0])]: subGrid[3],
    [[subGrid[6], subGrid[7], subGrid[8]].every((e, _, a) => a[0] && e == a[0])]: subGrid[6],

    // Columns
    [[subGrid[0], subGrid[3], subGrid[6]].every((e, _, a) => a[0] && e == a[0])]: subGrid[0],
    [[subGrid[1], subGrid[4], subGrid[7]].every((e, _, a) => a[0] && e == a[0])]: subGrid[1],
    [[subGrid[2], subGrid[5], subGrid[8]].every((e, _, a) => a[0] && e == a[0])]: subGrid[2],

    // Diagonals
    [[subGrid[0], subGrid[4], subGrid[8]].every((e, _, a) => a[0] && e == a[0])]: subGrid[0],
    [[subGrid[2], subGrid[4], subGrid[6]].every((e, _, a) => a[0] && e == a[0])]: subGrid[2],
  }[true];
}

export default class PlayableGameController {
  constructor(agent, agentPlaysFirst = true) {
    this.gameState = new GameState();
    this.agent = agent;
    this.agentPlaysFirst = agentPlaysFirst;

    if (this.agentPlaysFirst) {
      this.gameState.addEvent(this.agent.getNextMove(this.gameState.state));
    }
  }

  reset() {
    this.gameState.reset();
    this.agent.reset();

    if (this.agentPlaysFirst) {
      this.gameState.addEvent(this.agent.getNextMove(this.gameState.state));
    }

    return this.gameState.getGrid();
  }

  getGrid() {
    return this.gameState.getGrid();
  }

  findWins() {
    for (const i in this.gameState.state.subGrids) {
      const w = isBoardWon(this.gameState.state.subGrids[i]);
      if (w !== undefined && this.gameState.state.subGrids[i] != null) {
        this.gameState.addEvent({ g: i, w });

        const overall = isBoardWon(this.gameState.state.subGridsWon);
        if (overall) {
          this.gameState.addEvent({ overall });
          return true;
        }
      }
    }
    return false;
  }

  *placeTile(grid, tile) {
    if (this.gameState.state.winner) return;

    const playableTiles = getPlayableTiles(this.gameState.state);
    if (!playableTiles.find(x => x.g == grid && x.t == tile)) return;

    this.gameState.addEvent({ g: grid, t: tile });
    yield this.gameState.getGrid();

    if (!this.findWins()) {
      yield this.gameState.getGrid();
      this.gameState.addEvent(this.agent.getNextMove(this.gameState.state));
      this.findWins();
    }

    yield this.gameState.getGrid();
  }
}
