import React, { useCallback } from 'react'
import cn from 'classnames'
import { confirmAlert } from 'react-confirm-alert'
import 'react-confirm-alert/src/react-confirm-alert.css'

import { yoctoToNear } from 'utils/converter'
import winnerLogoSrc from 'assets/images/winner-logo.svg'

import useRaffle from '../useRaffle'

import styles from './Winner.module.scss'

const Winner = () => {
  const { data: { winner_ticket, winner, winner_amount } } = useRaffle()

  const disabled = winner_amount == 0

  const openModal = useCallback(() => {
    if (disabled) return

    confirmAlert({
      childrenElement: () => <div className={styles.modal}>
        <div className={styles.modalLogoWrapper}>
          <img src={winnerLogoSrc} alt="Winner" className={styles.modalLogo} />
        </div>

        <div className={styles.pair}>
          Winner <span className={styles.value}>{winner}</span>
        </div>

        <div className={styles.pair}>
          Ticket <span className={styles.value}>#{winner_ticket}</span>
        </div>

        <div className={styles.pair}>
          Amount <span className={styles.value}>{yoctoToNear(winner_amount)}Ⓝ</span>
        </div>
      </div>,
      overlayClassName: styles.overlay,
    })
  }, [])

  return (
    <div className={styles.winnerWrapper}>
      <div
        className={cn({
          [styles.winner]: true,
          [styles.disabled]: disabled
        })}
        onClick={openModal}
      >
        <div className={styles.winnerLogoWrapper}>
          <img src={winnerLogoSrc} alt="Winner" className={styles.winnerLogo} />
        </div>

        <div className={styles.label}>
          Latest raffle winner
        </div>
      </div>
    </div>
  )
}

export default Winner
