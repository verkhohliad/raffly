import React from 'react'

import { nanoToMilli, milliToYears, yoctoToNear } from 'utils/converter'
import { formatAmount } from 'utils/formatter'
import logger from 'utils/logger'
import prizeImgLeftSrc from 'assets/images/prize-left.svg'
import prizeImgRightSrc from 'assets/images/prize-right.svg'

import useRaffle from '../../useRaffle';

import styles from './PrizeItem.module.scss'

const PrizeItem = () => {
  const { data: { apy, locked_amount, ending_period } } = useRaffle()

  const prize = yoctoToNear(locked_amount) * apy / 100 * milliToYears(nanoToMilli(ending_period))

  logger.log('aprox. prize calc', { apy, locked_amount, ending_period, prize })

  return (
    <div className={styles.prizeItem}>
      <img className={styles.leftImg} src={prizeImgLeftSrc} alt="Prize Image Left" />

      <div className={styles.prizeDetails}>
        <div className={styles.prizeTitle}>Winning Amount</div>
        <div className={styles.prizeAmount}>~{formatAmount(prize)}Ⓝ</div>
      </div>

      <img className={styles.rightImg} src={prizeImgRightSrc} alt="Prize Image Right" />
    </div>
  )
}

export default PrizeItem
