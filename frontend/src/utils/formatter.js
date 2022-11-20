export const formatAmount = (amount) => {
  return amount.toLocaleString('en-US', {
    currency: 'USD',
  })
}

export const toPercent = (decimal, fixed = 0) => {
  return `${(decimal * 100).toFixed(fixed)}%`
}
