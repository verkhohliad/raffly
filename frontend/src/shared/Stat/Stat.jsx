import React from 'react'

import Icon from 'shared/Icon'

import styles from './Stat.module.scss'

const Stat = ({ label, icon, value, CustomComponent }) => {
  return (
    <div className={styles.stat}>
      <div className={styles.label}>{label}</div>

      {CustomComponent ? CustomComponent : (
        <div className={styles.valueWrapper}>
          <Icon src={icon} />

          <div className={styles.value}>{value}</div>
        </div>
      )}
    </div>
  )
}

export default Stat
