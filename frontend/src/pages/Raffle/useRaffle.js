import { useContext } from 'react'

import RaffleContext from './RaffleContext'

const useRaffle = () => {
  return useContext(RaffleContext)
}

export default useRaffle
