import React from 'react'
import cn from 'classnames'

import styles from './Input.module.scss'

const Input = ({ onChange, label, className, type = 'text', value, ...rest }) => {
  return (
    <div>
      <label className={styles.label}>{label}</label>
      <input
        className={cn([styles.input, className])}
        type={type}
        onChange={onChange}
        onWheel={(e) => e.target.blur()}
        {...rest}
      />
    </div>
  )
}

export default Input
