import React from 'react'
import cn from 'classnames'

import styles from './Checkbox.module.scss'

const Checkbox = ({ className, checked, onChange, ...rest }) => {
  return (
    <input
      className={cn([styles.checkbox, className])}
      type="checkbox"
      checked={checked}
      onChange={onChange}
      {...rest}
    />
  )
}

export default Checkbox
