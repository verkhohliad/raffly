import 'regenerator-runtime/runtime'
import React from 'react'

import Layout from 'pages/Layout'
import { NearProvider } from 'services/near'

const App = () => {
  return (
    <NearProvider>
      <Layout />
    </NearProvider>
  )
};

export default App
