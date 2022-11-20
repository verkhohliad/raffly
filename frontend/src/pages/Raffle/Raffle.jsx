import React from 'react'

import Prize from './Prize'
import Details from './Details'
import Participation from './Participation'
import Tickets from './Tickets'
import AdditionalActions from './AdditonalActions'
import Winner from './Winner'
import useRaffle from './useRaffle'

import styles from './Raffle.module.scss'

const Raffle = () => {
  const { isLoading, data: { myTickets } } = useRaffle()

  return !isLoading && (
    <div className={styles.raffle}>
      <Prize />

      <div className={styles.form}>
        <Winner />

        <Details />

        <Participation />

        {myTickets.length > 0 && <Tickets />}
      </div>

      {false && (<AdditionalActions />)}
    </div>
  )
}

export default Raffle
