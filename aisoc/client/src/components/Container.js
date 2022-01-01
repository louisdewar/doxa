import classNames from 'classnames';
import './Container.scss';

export default function Container({ children, padTop = true, ...args }) {
  return <div className={classNames('container', { 'container-with-top-margin': padTop })} {...args}>
    {children}
  </div>;
}
