import classNames from 'classnames';
import './Container.scss';

export default function Container({ children, padTop = true, padBottom = true, ...args }) {
  return <div className={classNames('container', { 'container-with-top-margin': padTop, 'container-with-bottom-margin': padBottom })} {...args}>
    {children}
  </div>;
}
