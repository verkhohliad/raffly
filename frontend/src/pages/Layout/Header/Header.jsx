import React, { useCallback } from 'react'

import useNear from 'services/near'
import Button from 'shared/Button';
import logoSrc from 'assets/images/logo.svg'

import styles from './Header.module.scss'

const Header = () => {
  const { signIn, signOut, accountId } = useNear()

  const handleSignIn = useCallback(() => {
    accountId ? signOut() : signIn();
  }, [signIn, accountId]);

  return (
    <div className={styles.header}>
      <img className={styles.logo} src={logoSrc} alt="Logo" />

      <Button
        onClick={handleSignIn}
        className={styles.connect}
      >
        {accountId ? accountId : 'Connect Wallet'}
      </Button>
    </div>
  )
}

export default Header
