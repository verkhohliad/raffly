import { utils } from 'near-api-js'

export const nanoToMilli = (nanoSecs) => {
  return nanoSecs / 10**6
}

export const milliToYears = (milliSecs) => {
  return milliSecs / 1000 / 60 / 60 / 24 / 365
}

export const yoctoNearToNear = (yoctoNear) => {
  return yoctoNear / 10**24
}

export const yoctoNearToString = (yoctoNear) => {
  return BigInt(yoctoNear).toString()
}

export const yoctoToNear = (yocto) => {
  return utils.format.formatNearAmount(yocto)
}

export const nearToYocto = (near) => {
  return utils.format.parseNearAmount(near)
}
