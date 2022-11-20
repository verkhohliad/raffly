export const ROOT_CONTRACT_NAME = process.env.ROOT_CONTRACT_NAME || 'raffly-root.near'

export const NEAR_METRICS_ENDPOINT = 'https://validators.narwallets.com/metrics_json'

const getNearConfig = (env) => {
  switch (env) {
    case 'development':
    case 'production':
    case 'mainnet':
      return {
        networkId: 'mainnet',
        nodeUrl: 'https://rpc.mainnet.near.org',
        contractName: ROOT_CONTRACT_NAME,
        walletUrl: 'https://wallet.near.org',
        helperUrl: 'https://helper.mainnet.near.org',
        explorerUrl: 'https://explorer.mainnet.near.org',
      }
    // case 'development':
    case 'testnet':
      return {
        networkId: 'testnet',
        nodeUrl: 'https://rpc.testnet.near.org',
        contractName: ROOT_CONTRACT_NAME,
        walletUrl: 'https://wallet.testnet.near.org',
        helperUrl: 'https://helper.testnet.near.org',
        explorerUrl: 'https://explorer.testnet.near.org',
      }
    default:
      throw Error(`Unconfigured environment '${env}'. Can be configured in src/config.js.`)
  }
}

export const nearConfig = getNearConfig(process.env.NODE_ENV)
