import GameState from './gameReducer';

export default class LiveSocketManager {
  constructor(url) {
    this.url = url;
    this.gameState = new GameState();
    this.humanPlayer = null;
    this.availableGrids = null;
  }

  connect() {
    this.socket = new WebSocket(this.url);
    this.socket.onmessage = this.handleSocketMessage.bind(this);
  }

  handleSocketMessage(e) {
    let message = JSON.parse(e.data);
    let parts;

    switch (message.event[0]) {
    case 'S':
      // Start game
      this.setHumanPlayer(message.event.slice(2));
      break;
    case 'R':
      // Request action
      //let availableGrids = message.event.split(' ')[1];
      break;
    case 'P':
      // Tile was placed
      parts = message.event.split(' ');
      this.gameState.addEvent({ 'g': parts[2], 't': parts[3] });
      break;
    case 'G':
      // Grid was won
      parts = message.event.split(' ');
      this.gameState.addEvent({ 'g': parts[2], 'w': parts[1] });
      break;
    default:
      console.error('Unrecognised socket message: ' + message.event);
    }
  }

  setHumanPlayer(colour) {
    if (this.humanPlayer !== null) {
      console.error('Recieved start game event while a game was in progress');
      return;
    }
    if (colour === 'R') {
      this.humanPlayer = 'red';
    } else if (colour === 'B') {
      this.humanPlayer = 'blue';
    } else {
      console.error('Invalid player colour: ' + colour);
    }
  }
}
