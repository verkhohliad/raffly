import React from 'react'

import useRaffle from '../useRaffle'
import styles from './Tickets.module.scss'

const Tickets = () => {
  const { data: { myTickets } } = useRaffle()

  return (
    <div className={styles.tickets}>
      <div className={styles.title}>Tickets</div>

      <div className={styles.ticketsWrapper}>
        {myTickets.map((ticket) => (
          <div
            className={styles.ticket}
            key={ticket}
          >
            #{ticket}
          </div>
        ))}
      </div>
    </div>
  )
}

export default Tickets
