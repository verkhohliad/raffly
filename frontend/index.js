import React from 'react'
import { createRoot } from 'react-dom/client'

import App from './App'

import "@near-wallet-selector/modal-ui/styles.css"
import 'styles/root.scss'

const container = document.querySelector('#root')
const root = createRoot(container)

root.render(<App />)
