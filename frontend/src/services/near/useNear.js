import { useContext } from 'react'

import NearContext from './NearContext'

const useNear = () => {
  return useContext(NearContext)
}

export default useNear