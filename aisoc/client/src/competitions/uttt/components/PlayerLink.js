import './PlayerLink.scss';
import { Link } from 'react-router-dom';

export default function PlayerLink({ baseUrl, username, playerClass }) {
  return <Link to={`${baseUrl}user/${username}`} className={`PlayerLink player-${playerClass}`}>{username}</Link>;
}
