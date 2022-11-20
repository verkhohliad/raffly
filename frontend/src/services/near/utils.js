import { connect, keyStores, WalletConnection, Contract, providers } from 'near-api-js'

import { ROOT_CONTRACT_NAME, nearConfig } from './config'

export const DEFAULT_GAS = 300000000000000

const provider = new providers.JsonRpcProvider(nearConfig.nodeUrl)

// export const readMethods = [
//   ['get_state', 'state'],
//   ['get_ticket_price', 'ticket_price'],
//   ['get_locked_amount', 'locked_amount'],
//   ['get_prolonged_amount', 'prolonged_amount'],
//   ['get_ticker_counter', 'ticket_counter'],
//   ['get_winner', 'winner'],
//   ['get_winner_ticket', 'winner_ticket'],
//   ['get_is_raffle_on', 'is_raffle_on'],
//   ['get_start_time', 'start_time'],
//   ['get_end_time', 'end_time'],
//   ['get_starting_period', 'starting_period'],
//   ['get_ending_period', 'ending_period'],
//   ['get_prolonged_assets', 'prolonged_assets'],
//   ['get_locked_assets', 'locked_assets'],
//   ['get_claimable_assets', 'claimable_assets'],
//   ['get_tickets', 'tickets'],
//   ['get_raffle_owner', 'raffle_owner'],
//   ['get_owner', 'owner'],
//   ['get_start_st_near_price', 'start_st_near_price'],
//   ['get_end_st_near_price', 'end_st_near_price'],
//   ['get_round', 'round'],
//   ['get_raffle_name', 'raffle_name'],
//   ['get_raffle_description', 'raffle_description'],
// ]

const viewMethods = ['get_state']
const changeMethods = ["claim", "remove_prolongation", "participate"]

export const connectWallet = async () => {
  const near = await connect({
    ...nearConfig,
    keyStore: new keyStores.BrowserLocalStorageKeyStore(),
  })

  return new WalletConnection(near)
}

export const signOut = (wallet) => {
  wallet.signOut()
}

export const setupContract = (wallet, contractId = ROOT_CONTRACT_NAME) => {
  if (wallet.isSignedIn()) {
    return new Contract(
      wallet.account(),
      contractId,
      {
        viewMethods,
        changeMethods,
        sender: wallet.account(),
      }
    )
  }

  return null
}

export const callViewMethodViaProvider = async ({ methodName, args, accountId = ROOT_CONTRACT_NAME }) => {
  const resultRaw = await provider.query({
    request_type: "call_function",
    account_id: accountId,
    method_name: methodName,
    args_base64: btoa(JSON.stringify(args)),
    finality: "optimistic",
  })

  return JSON.parse(Buffer.from(resultRaw.result).toString())
}
