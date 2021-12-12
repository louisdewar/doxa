import './Button.scss';

export default function Button({ children }) {
  return <button className="button">
    {children}
  </button>;
}
