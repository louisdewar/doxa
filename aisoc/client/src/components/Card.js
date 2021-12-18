import './Card.scss';

export default function Card({ children, darker, className }) {
  return <div className={`card ${darker ? 'card-darker' : ''} ${className}`}>{children}</div>;
}
