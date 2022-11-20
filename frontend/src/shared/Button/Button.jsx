import React from 'react'
import cn from 'classnames'

import styles from './Button.module.scss'

const Button = ({ children, onClick, className, ...rest }) => {
  return (
    <button
      onClick={onClick}
      className={cn([styles.button, className])}
      {...rest}
    >
      {children}
    </button>
  )
}

export default Button
