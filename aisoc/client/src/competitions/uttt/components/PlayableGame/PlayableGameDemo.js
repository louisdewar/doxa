import { faGamepad } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useRef, useState } from 'react';
import PlayableGame from './PlayableGame';
import './PlayableGameDemo.scss';
import RandomAgent from './RandomAgent';


export default function PlayableGameDemo() {
  const [resets, setResets] = useState(0);
  const randomAgent = useRef(new RandomAgent());

  return <>
    <div className='playable-game-demo'>
      <PlayableGame resets={resets} agent={randomAgent.current} />
    </div>
    <div className='playable-game-demo-footer'>
      <FontAwesomeIcon icon={faGamepad} fixedWidth size='sm' />
      <span className='playable-game-demo-agent-name'>
        Random agent
      </span>
      <a onClick={e => {
        e.preventDefault();
        setResets(resets + 1);
      }} href="#" className='playable-game-demo-reset-link'>RESET</a>
    </div>
  </>;
}
