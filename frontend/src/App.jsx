import { useEffect, useState } from 'react';
import './App.css';

import { setupWalletSelector } from '@near-wallet-selector/core';
import { setupModal } from '@near-wallet-selector/modal-ui';
import { setupMyNearWallet } from '@near-wallet-selector/my-near-wallet';
import '@near-wallet-selector/modal-ui/styles.css';

import Wheel from './components/Wheel';

let modal, wallet;

function App() {
  const [{ win, number, red, multiple }, setState] = useState({
    // win: true,
    // number: 3,
    // red: true,
    // multiple: 1,
  });

  async function setup() {
    const selector = await setupWalletSelector({
      network: 'testnet',
      modules: [setupMyNearWallet()],
    });

    modal = setupModal(selector, {
      contractId: 'test123456789.gfijoe.testnet',
    });

    wallet = await selector.wallet('my-near-wallet');
    const accounts = await wallet.getAccounts();
    console.log(accounts);
  }

  useEffect(() => {
    setup();
  }, []);

  // app functions

  async function showModal() {
    modal.show();
  }

  async function spin() {
    const res = await wallet.signAndSendTransaction({
      actions: [
        {
          type: 'FunctionCall',
          params: {
            methodName: 'spin_with_near',
            args: {
              spins: [
                [
                  {
                    kind: 'Red',
                    number: 0,
                    amount: '100000000000000000000000', // 0.1 near
                  },
                ],
              ],
              callback_tgas: 3,
            },
            gas: '30000000000000', // 30 tgas
            deposit: '100000000000000000000000', // 0.1 near
          },
        },
      ],
    });
    console.log(JSON.parse(atob(res.status.SuccessValue)));

    const [win, number, red, multiple] = JSON.parse(
      atob(res.status.SuccessValue),
    )[0][0];

    setState({ win, number, red, multiple });
  }

  return (
    <>
      <div className="card">
        <button onClick={showModal}>Connect Wallet</button>
        <button onClick={spin}>Spin</button>
      </div>
      <div style={{ position: 'relative' }}>
        <Wheel {...{ win, number, red, multiple }} />
      </div>
    </>
  );
}

export default App;
