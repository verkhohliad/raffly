import React from 'react'
import cn from 'classnames'

import styles from './Icon.module.scss'

const Icon = ({ src, onClick, className }) => {
  return (
    <div
      className={cn({
        [styles.iconWrapper]: true,
        [className]: className,
        [styles.clickable]: Boolean(onClick),
      })}
      onClick={onClick}
    >
      <img src={src} alt="icon" className={styles.icon} />
    </div>
  )
}

export default Icon
