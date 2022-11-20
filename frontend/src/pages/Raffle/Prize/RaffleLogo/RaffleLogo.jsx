import React from 'react'

import raffleLogoSrc from 'assets/images/raffle-liquid-logo.svg'

import styles from './RaffleLogo.module.scss'

const RaffleLogo = () => {
  return (
    <div className={styles.raffleLogoWrapper}>
      <img className={styles.raffleLogo} src={raffleLogoSrc} alt="Raffle Logo" />
    </div>
  )
}

export default RaffleLogo
