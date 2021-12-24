import PlayableGame from '../components/PlayableGame';

export default function OverviewTab() {
  return <div style={{ boxSizing: 'border-box', display: 'flow-root' }}>
    <h2>Overview</h2>
    <div style={{ float: 'right', width: '45%', backgroundColor: '#1F2937', borderRadius: '3px', margin: 'auto', marginLeft: '1rem' }}>
      <PlayableGame />
    </div>

    <p>
      Ultimate tic-tac-toe is two-player game based on a 3×3 grid of regular tic-tac-toe boards. To win the overall game, a player must win a horizontal, vertical or diagonal row of smaller tic-tac-toe boards — but there is a twist!
    </p>
    <p>
      The tile position a player chooses in the smaller board determines the board the next player must play in. For example, if you pick the top-right tile of a board, your opponent must then play in the top-right board.
    </p>
    <p>
      To get to grips with the rules, feel free to play against the agent on the right. Your challenge is then to develop an ultimate tic-tac-toe agent that beats everybody else{'\''}s.
    </p>
  </div >;
}
