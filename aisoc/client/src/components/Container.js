import './Container.scss';

export default function Container({ children, ...args }) {
  return <div className='container' {...args}>
    {children}
  </div>;
}
