import React from 'react'

import Icon from 'shared/Icon'
import iconInfoSrc from 'assets/images/icon-info.svg'
import iconShareSrc from 'assets/images/icon-share.svg'

import styles from './AdditionalActions.module.scss'

const AdditionalActions = () => {
  return (
    <div className={styles.additionalActions}>
      <Icon
        className={styles.icon}
        src={iconShareSrc}
        onClick={() => console.log('share')}
      />
      <Icon className={styles.icon} src={iconInfoSrc} />
    </div>
  )
}

export default AdditionalActions
