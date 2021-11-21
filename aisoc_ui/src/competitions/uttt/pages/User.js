import api from 'common/api';
import Matches from 'competitions/uttt/components/Matches.js';
import Navbar from 'components/NavBar.js';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import './User.scss';


export default function User() {
  let { username } = useParams();

  const [score, setScore] = useState(null);

  useEffect(() => {
    api.user.getScore(username).then(data => {
      // If a score is null the user has not yet had any match results so we default to 0
      setScore(data.score || 0);
    }).catch(err => {
      console.error(err);
    });
  }, []);

  return (
    <div>
      <Navbar competitionName='Ultimate Tic-Tac-Toe' homepageUrl='/c/uttt/' />
      <div className="main">
        <div className="user-info">
          <div className="user-header">
            <h1>{username}</h1>
          </div>
          <div className="user-stats">
            <span className="score">{score} pts</span>
          </div>
        </div>
        <Matches username={username} />
      </div>
    </div>
  );
}
