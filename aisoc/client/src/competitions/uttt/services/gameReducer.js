export default class GameState {
  constructor() {
    // List of moves
    // Each move is a list with at least one element (the placement event)
    // If there is another event e.g. a grid won then it counts as part of the previous move
    this.moves = [];
    this.index = -1;
    this.state = initialState();
  }

  // Adds the event to the list of events
  // If the event type is tile placement it will create a new move, otherwise it will append it to the previous one
  // If we are currently at the end of the state it will also apply it automatically to the live game state
  addEvent(event) {
    const upToDate = this.index + 1 === this.moves.length;

    if ('t' in event) {
      // tile placement
      this.moves.push([event]);
    } else {
      this.moves[this.moves.length - 1].push(event);
    }

    if (upToDate) {
      this.state = apply(this.state, event);
      // Depending on the event type there may nor may not be an additional move, in either case our index is the latest
      this.index = this.moves.length - 1;
    }
  }

  addManyEvents(events) {
    for (let event of events) {
      this.addEvent(event);
    }
  }

  getGrid() {
    return this.state;
  }

  next() {
    if (this.index + 1 === this.moves.length) {
      return false;
    }  

    for (let move of this.moves[this.index + 1]) {
      this.state = apply(this.state, move);
    }

    this.index++;

    return true;
  }

  previous() {
    if (this.index === -1) {
      return false;
    }  

    for (let move of this.moves[this.index]) {
      this.state = undo(this.state, move);
    }

    this.index--;

    return true;
  }

  goToBeginning() {
    this.index = -1;
    this.state = initialState();
  }

  goToEnd() {
    while (this.index + 1 < this.moves.length) {
      this.next();
    }
  }

  goToPosition(pos) {
    if (pos < -1 || pos >= this.moves.length) {
      throw new Error('Position out of bounds');
    }

    let delta = pos - this.index;

    if (-delta > pos + 1) {
      // Go forwards instead
      delta = pos + 1;
      this.goToBeginning();
    }

    while (delta != 0) {
      if (delta < 0) {
        if (!this.previous()) {
          throw new Error('Already at start');
        }

        delta++;
      } else {
        if (!this.next()) {
          throw new Error('Already at end');
        }

        delta--;
      }
    }

    if (!this.index == pos) {
      throw new Error(`goToPosition failed, pos (${pos}) did not equal index (${this.index})`);
    }
  }

  getLength() {
    return this.moves.length;
  }

  // A position of -1 means we are in the initial state.
  getPosition() {
    return this.index;
  }

  // Generates an array where each element is an object containing `{ t: number, g: number }`
  // representing the played moves
  toMoveList() {
    return this.moves.map(actions => actions.filter(action => 't' in action && 'g' in action)[0]);
  }
}

function initialState() {
  return {
    winner: null,
    nextPlayer: 'R',
    nextGrid: null,
    // To allow for undoing we need to store each "nextGrid" that occured
    // This contains the previous "nextGrid"s not the current one
    pastNextGrids: [],
    subGridsWon: Array(9).fill(null),
    subGrids: Array(9).fill(Array(9).fill(null))
  };
}

function apply(state, action) {
  if (!state) {
    state = initialState();
  }

  state = { ...state };

  if ('overall' in action) {
    state.winner = action.overall;
  } else if ('t' in action) {
    state.subGrids = [...state.subGrids];
    state.subGrids[action.g] = [...state.subGrids[action.g]];
    state.subGrids[action.g][action.t] = state.nextPlayer;
    state.nextPlayer = state.nextPlayer === 'R'? 'B': 'R';

    // Store the previous "nextGrid"
    state.pastNextGrids.push(state.nextGrid);

    if (state.subGridsWon[action.t] === null) {
      state.nextGrid = action.t;
    } else {
      // Any non-taken grid is allowed
      state.nextGrid = null;
    }
  } else if ('w' in action) {
    state.subGridsWon = [...state.subGridsWon];
    state.subGridsWon[action.g] = action.w;
  } else {
    throw new Error('Unknown action ' + JSON.stringify(action));
  }


  return state;
}

function undo(state, action) {
  if (!state) {
    throw Error('Tried to undo when state was not set');
  }

  state = { ...state };

  if ('overall' in action) {
    state.winner = null;
  } else if ('t' in action) {
    state.subGrids = [...state.subGrids];
    state.subGrids[action.g] = [...state.subGrids[action.g]];
    state.subGrids[action.g][action.t] = null;
    state.nextPlayer = state.nextPlayer === 'R'? 'B': 'R';
    state.nextGrid = state.pastNextGrids.pop();

  } else if ('w' in action) {
    state.subGridsWon = [...state.subGridsWon];
    state.subGridsWon[action.g] = null;
  } else {
    throw new Error('Unknown action ' + action);
  }

  return state;
}