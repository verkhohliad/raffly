import { useEffect, useState, useCallback } from 'react'
import axios from 'axios'

import useNear, { NEAR_METRICS_ENDPOINT, callViewMethodViaProvider } from 'services/near'
import { ONE_MILLI_NEAR, DEFAULT_GAS } from 'utils/constants'
import { yoctoToNear, nearToYocto } from 'utils/converter'
import logger from 'utils/logger'

const DEFAULT_APY = 10

const useLoadData = ({ contractId }) => {
  const [contract, setContract] = useState(null)
  const [data, setData] = useState({})
  const [isLoading, setIsLoading] = useState(true)

  const { getContract, wallet, accountId } = useNear()

  useEffect(() => {
    const loadData = async () => {
      setIsLoading(true)

      let data

      if (wallet.isSignedIn()) {
        const contract = getContract(contractId)

        setContract(contract)

        data = await contract.get_state()
      } else {
        data = await callViewMethodViaProvider({ methodName: 'get_state', args: {}, accountId: contractId })
      }

      const res = await axios.get(NEAR_METRICS_ENDPOINT)

      const formattedData = {
        ...data,
        apy: res?.data?.st_near_30_day_apy ?? DEFAULT_APY,
        myTickets: data.tickets.reduce((acc, [number, account_id]) => {
          if (account_id === accountId) {
            acc.push(number)
          }

          return acc
        }, [])
      }

      logger.log('loaded data', formattedData)

      setData(formattedData)

      setIsLoading(false)
    }

    if (wallet) {
      loadData()
    }
  }, [wallet])

  const participate = useCallback((ticketAmount, isAutoProlong) => {
    const amount = ticketAmount * yoctoToNear(data.ticket_price) + ticketAmount * ONE_MILLI_NEAR

    logger.log('participate', { ticketAmount, ticket_price: data.ticket_price, isAutoProlong, amount: nearToYocto(amount.toString()) })

    // redirecting to approving transaction
    contract.participate({
      args: {
        is_auto_prolong: isAutoProlong,
      },
      gas: DEFAULT_GAS,
      amount: nearToYocto(String(amount)),
    })
  }, [contract, data])

  return {
    data,
    contract,
    isLoading,
    participate,
  }
}

export default useLoadData
