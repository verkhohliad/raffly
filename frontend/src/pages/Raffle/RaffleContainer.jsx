import React from 'react'

import Raffle from './Raffle'
import RaffleProvider from './RaffleProvider';

const RaffleContainer = ({ contractId }) => {
  return (
    <RaffleProvider contractId={contractId}>
      <Raffle />
    </RaffleProvider>
  )
}

export default RaffleContainer
