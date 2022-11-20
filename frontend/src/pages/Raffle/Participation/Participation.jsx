import React, { useState, useCallback, useMemo } from 'react'
import { utils } from 'near-api-js'

import Input from 'shared/Input'
import Button from 'shared/Button'
import Checkbox from 'shared/Checkbox'

import useRaffle from '../useRaffle'

import styles from './Participation.module.scss'

const Participation = () => {
  const [ticketAmount, setTicketAmount] = useState()
  const [isAutoProlong, setIsAutoProlong] = useState(true)

  const { data: { ticket_price, is_raffle_on }, participate, contract } = useRaffle()

  const handleParticipate = useCallback((ev) => {
    ev.preventDefault()

    participate(ticketAmount, isAutoProlong)
  }, [participate, ticketAmount, isAutoProlong])

  const onGoingParticipateDescription = useMemo(() => {
    if (!is_raffle_on) return ''

    if (!isAutoProlong)
      return '. You can participate in the next raffle only with auto prolongation now.'

    return '. You can participate in the next raffle.'
  }, [is_raffle_on, isAutoProlong])

  return (
    <div className={styles.participation}>
      <div className={styles.title}>
        <div className={styles.title__text}>
          Participate
        </div>
        <div className={styles.title__description}>
          1 ticket = {utils.format.formatNearAmount(ticket_price)} Ⓝ{onGoingParticipateDescription}
        </div>
      </div>

      <form className={styles.ticket__form} onSubmit={handleParticipate}>
        <Input
          label="Number of tickets"
          placeholder="0"
          type="number"
          value={ticketAmount}
          onChange={e => setTicketAmount(Math.abs(e.target.value))}
          className={styles.ticket__input}
          disabled={is_raffle_on && !isAutoProlong}
        />

        <Button
          className={styles.ticket__button}
          type="submit"
          disabled={!ticketAmount || !contract || (is_raffle_on && !isAutoProlong)}
        >
          Buy
        </Button>
      </form>

      <div className={styles.auto_prolong}>
        <Checkbox
          className={styles.auto_prolong__checkbox}
          checked={isAutoProlong}
          onChange={() => setIsAutoProlong(!isAutoProlong)}
        />
        <label className={styles.auto_prolong__label}>Auto prolong</label>
      </div>
    </div>
  )
}

export default Participation
