import React, { useMemo } from 'react'

import useLoadData from './useLoadData';
import RaffleContext from './RaffleContext';

const RaffleProvider = ({ contractId, children }) => {
  const { data, isLoading, contract, participate } = useLoadData({ contractId });

  const raffleData = useMemo(() => {
    return {
      data,
      isLoading,
      contract,
      participate,
    }
  }, [data, isLoading, contract, participate]);

  return (
    <RaffleContext.Provider value={raffleData}>
      {children}
    </RaffleContext.Provider>
  )
}

export default RaffleProvider
