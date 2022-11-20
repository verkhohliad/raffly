import React from 'react'

import PrizeItem from './PrizeItem';
import RaffleLogo from './RaffleLogo';

import styles from './Prize.module.scss'

const Prize = () => {
  return (
    <div className={styles.prize}>
      <PrizeItem />

      <RaffleLogo />
    </div>
  )
}

export default Prize
