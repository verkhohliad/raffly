import React from 'react'

import Raffle from 'pages/Raffle'

import Header from './Header'

const Layout = () => {
return (
    <>
      <Header />

      <Raffle contractId="liquid.raffly-root.near" />
    </>
  )
}

export default Layout
