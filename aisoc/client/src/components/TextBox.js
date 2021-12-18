import './TextBox.scss';

export default function TextBox({ value, setValue, placeholder, type, className, disabled }) {

  const handleValueChange = e => {
    setValue(e.target.value);
  };

  return <input
    type={type}
    className={`textbox ${className}`}
    placeholder={placeholder}
    value={value}
    onChange={handleValueChange}
    disabled={disabled}
  />;
}
