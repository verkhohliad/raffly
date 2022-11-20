import { createContext } from 'react'

const NearContext = createContext({
  wallet: null,
  isLoggedIn: false,
  accountId: '',
  getContract: () => {},
  signIn: () => {},
  signOut: () => {},
})

export default NearContext
