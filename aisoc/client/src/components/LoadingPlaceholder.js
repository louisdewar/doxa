import './LoadingPlaceholder.scss';

function toLength(val) {
  if (typeof val === 'number') {
    return `${val}px`;
  } else {
    return val;
  }
}

export default function LoadingPlaceholder({ width, height }) {
  width = toLength(width);
  height = toLength(height || 15);

  return <div className="LoadingPlaceholder" style={{ width, height }}></div>;
}
