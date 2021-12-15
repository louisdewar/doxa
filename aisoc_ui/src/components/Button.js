import './Button.scss';

export default function Button({ children, buttonProps, success, failure, disabled }) {
  let className = 'button';
  if (disabled) {
    className += ' button-disabled';
  } else {
    if (success) className += ' button-success';
    if (failure) className += ' button-failure';
  }


  return <button className={className}   {...buttonProps}   >
    {children}
  </button >;
}
