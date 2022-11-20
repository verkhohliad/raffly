import { createContext } from 'react'

const RaffleContext = createContext({
  data: {},
  contract: {},
  isLoading: false,
  participate: () => {},
})

export default RaffleContext
