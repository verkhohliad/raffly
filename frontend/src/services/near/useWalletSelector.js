import { useEffect, useState } from 'react'
import { setupWalletSelector } from "@near-wallet-selector/core"
import { setupModal } from "@near-wallet-selector/modal-ui"
import { setupNearWallet } from "@near-wallet-selector/near-wallet"
import { setupMyNearWallet } from "@near-wallet-selector/my-near-wallet"
import { setupSender } from "@near-wallet-selector/sender"
import { setupMathWallet } from "@near-wallet-selector/math-wallet"
import { setupNightly } from "@near-wallet-selector/nightly"
import { setupMeteorWallet } from "@near-wallet-selector/meteor-wallet"
import { setupLedger } from "@near-wallet-selector/ledger"
import { setupWalletConnect } from "@near-wallet-selector/wallet-connect"
import { setupNightlyConnect } from "@near-wallet-selector/nightly-connect"
import { setupDefaultWallets } from "@near-wallet-selector/default-wallets"

import { nearConfig, ROOT_CONTRACT_NAME } from './config'

const useWalletSelector = () => {
  const [modal, setModal] = useState(null)

  useEffect(() => {
    const init = async () => {
      const selector = await setupWalletSelector({
        network: nearConfig.networkId,
        modules: [
          ...(await setupDefaultWallets()),
          setupNearWallet(),
          setupMyNearWallet(),
          setupSender(),
          setupMathWallet(),
          setupNightly(),
          setupMeteorWallet(),
          setupLedger(),
          // todo: change params from default
          setupWalletConnect({
            projectId: "c4f79cc...",
            metadata: {
              name: "NEAR Wallet Selector",
              description: "Example dApp used by NEAR Wallet Selector",
              url: "https://github.com/near/wallet-selector",
              icons: ["https://avatars.githubusercontent.com/u/37784886"],
            },
          }),
          setupNightlyConnect({
            url: "wss://ncproxy.nightly.app/app",
            appMetadata: {
              additionalInfo: "",
              application: "NEAR Wallet Selector",
              description: "Example dApp used by NEAR Wallet Selector",
              icon: "https://near.org/wp-content/uploads/2020/09/cropped-favicon-192x192.png",
            },
          }),
        ],
      })

      setModal(
        setupModal(selector, {
          contractId: ROOT_CONTRACT_NAME,
        })
      )
    }

    init()
  }, [])

  return modal
}

export default useWalletSelector
