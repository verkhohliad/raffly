import React from 'react'

import useRaffle from '../useRaffle';
import Stats from './Stats'

import styles from './Details.module.scss'

const Details = () => {
  const { data: { raffle_name, raffle_description, round } } = useRaffle()

  return (
    <div className={styles.details}>
      <div className={styles.title}>
        <div className={styles.titleText}>
          {raffle_name} <span className={styles.round}>{round}</span>
        </div>

        <div className={styles.description}>
          {raffle_description}
        </div>
      </div>

      <Stats />
    </div>
  )
}

export default Details
