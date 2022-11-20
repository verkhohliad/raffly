import React, { useState, useEffect } from 'react'

import styles from './CountDown.module.scss'

// in milliseconds
const CountDown = ({ time }) => {
  const [days, setDays] = useState(0)
  const [hours, setHours] = useState(0)
  const [minutes, setMinutes] = useState(0)
  const [seconds, setSeconds] = useState(0)

  useEffect(() => {
    const interval = setInterval(() => {
      const now = new Date().getTime()
      const distance = time - now

      const days = Math.floor(distance / (1000 * 60 * 60 * 24))
      const hours = Math.floor((distance % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60))
      const minutes = Math.floor((distance % (1000 * 60 * 60)) / (1000 * 60))
      const seconds = Math.floor((distance % (1000 * 60)) / 1000)

      setDays(days > 0 ? days : 0)
      setHours(hours > 0 ? hours : 0)
      setMinutes(minutes > 0 ? minutes : 0)
      setSeconds(seconds > 0 ? seconds : 0)

      if (distance < 0) {
        clearInterval(interval)
      }
    }, 1000)

    return () => clearInterval(interval)
  }, [time])

  return (
    <div className={styles.countdown}>
      <div className={styles.countdown__item}>
        <div className={styles.countdown__number}>{days}</div>
        <div className={styles.countdown__text}>days</div>
      </div>

      <div className={styles.countdown__divider}>:</div>

      <div className={styles.countdown__item}>
        <div className={styles.countdown__number}>{hours}</div>
        <div className={styles.countdown__text}>hours</div>
      </div>

      <div className={styles.countdown__divider}>:</div>

      <div className={styles.countdown__item}>
        <div className={styles.countdown__number}>{minutes}</div>
        <div className={styles.countdown__text}>min</div>
      </div>

      <div className={styles.countdown__divider}>:</div>

      <div className={styles.countdown__item}>
        <div className={styles.countdown__number}>{seconds}</div>
        <div className={styles.countdown__text}>sec</div>
      </div>
    </div>
  )
}

export default CountDown
