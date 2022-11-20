import React from 'react'

import Stat from 'shared/Stat'
import CountDown from 'shared/CountDown'
import { toPercent } from 'utils/formatter'
import { nanoToMilli, yoctoToNear } from 'utils/converter';
import participatedAmountIcon from 'assets/images/icon-circle-chart.svg'
import participantsNumberIcon from 'assets/images/icon-people.svg'
import winChanceIcon from 'assets/images/icon-gift.svg'

import useRaffle from '../../useRaffle';

import styles from './Stats.module.scss'

const Stats = () => {
  const { data: { locked_amount, locked_assets, ticket_counter, end_time, is_raffle_on, starting_period, start_time, ending_period, myTickets } } = useRaffle()

  return (
    <div className={styles.stats}>
      <div className={styles.row}>
        <Stat
          label="Participated Amount"
          value={`${yoctoToNear(locked_amount)} Ⓝ`}
          icon={participatedAmountIcon}
        />

        <Stat
          label="Number of Participants"
          value={locked_assets?.length}
          icon={participantsNumberIcon}
        />
      </div>

      <div className={styles.row}>
        <Stat
          label={`Chance of Win for ${myTickets.length > 0 ? myTickets.length : 1} Ticket${myTickets.length > 1 ? 's' : ''}`}
          value={toPercent((myTickets.length || 1) / (ticket_counter))}
          icon={winChanceIcon}
        />

        <Stat
          label={`Raffle will ${is_raffle_on ? 'end' : 'start'} in`}
          CustomComponent={<CountDown time={nanoToMilli(is_raffle_on ? start_time + ending_period : end_time + starting_period)} />}
        />
      </div>
    </div>
  )
}

export default Stats
