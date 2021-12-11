import Matches from 'competitions/uttt/components/Matches.js';
import Navbar from 'competitions/uttt/components/NavBar.js';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import UTTTAPI from '../api';
import './User.scss';


export default function User({ competitionBaseUrl }) {
  let { username } = useParams();

  const [score, setScore] = useState(null);

  useEffect(() => {
    UTTTAPI.getUserScore(username).then(data => {
      // If a score is null the user has not yet had any match results so we default to 0
      setScore(data.score || 0);
    }).catch(err => {
      console.error(err);
    });
  }, []);

  return (
    <div>
      <Navbar competitionName='Ultimate Tic-Tac-Toe' homepageUrl={competitionBaseUrl} />
      <div className="main">
        <div className="user-info">
          <div className="user-header">
            <h1>{username}</h1>
          </div>
          <div className="user-stats">
            <span className="score">{score} pts</span>
          </div>
        </div>
        <Matches username={username} competitionBaseUrl={competitionBaseUrl} />
      </div>
    </div>
  );
}
