import React, { useState, useEffect, useMemo, useCallback } from 'react'
import queryString from 'query-string'

import logger from 'utils/logger'

import NearContext from './NearContext'
import { connectWallet, setupContract, signOut as nearSignOut } from './utils'
import useWalletSelector from './useWalletSelector'

const NearProvider = ({ children }) => {
  const [wallet, setWallet] = useState(null)
  const walletSelectorModal = useWalletSelector()

  useEffect(() => {
    const setup = async () => {
      const wallet = await connectWallet()

      setWallet(wallet)

      if (wallet.isSignedIn()) {
        // preload smth
      }
    }

    const parsedQuery = queryString.parse(window.location.search)

    if (parsedQuery.signedNear) {
      // handle moving back from success transaction
      // window.location.replace(window.location.origin)
    }

    setup().catch(logger.error)
  }, [])

  const getContract = useCallback((contractId) => {
    return setupContract(wallet, contractId);
  }, [wallet]);

  const value = useMemo(() => {
    return {
      wallet,
      isLoggedIn: wallet?.isSignedIn?.(),
      accountId: wallet?.account?.()?.accountId,
      getContract,
      signIn: () => walletSelectorModal.show(),
      signOut: () => {
        nearSignOut(wallet)

        window.location.replace(window.location.origin + window.location.pathname)
      },
    }
  }, [wallet, walletSelectorModal])

  return (
    <NearContext.Provider value={value}>
      {children}
    </NearContext.Provider>
  )
}

export default NearProvider
