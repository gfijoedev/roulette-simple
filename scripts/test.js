import * as dotenv from 'dotenv';
dotenv.config();

const { NEAR_ACCOUNT_ID, NEAR_SEED_PHRASE, NEAR_CONTRACT_ID } = process.env;

import { readFileSync } from 'fs';
import { Account } from '@near-js/accounts';
import { JsonRpcProvider } from '@near-js/providers';
import { KeyPairSigner } from '@near-js/signers';
import { KeyPair } from '@near-js/crypto';
import { parseNearAmount, formatNearAmount } from '@near-js/utils';
import { parseSeedPhrase } from 'near-seed-phrase';

const wait = async (s = 500) => await new Promise((r) => setTimeout(r, s));

// near setup
const provider = new JsonRpcProvider({
  url: 'https://test.rpc.fastnear.com',
});
const { secretKey } = parseSeedPhrase(NEAR_SEED_PHRASE);
const keyPair = KeyPair.fromString(secretKey);
let signer = new KeyPairSigner(keyPair);

async function deleteAccount(accountId) {
  const account = new Account(accountId, provider, signer);
  try {
    await account.deleteAccount(NEAR_ACCOUNT_ID);
    console.log('Account deleted', accountId);
  } catch (e) {
    console.log('Error deleting account', e);
  }
}

async function createAccount(accountId) {
  const account = new Account(NEAR_ACCOUNT_ID, provider, signer);
  try {
    await account.createAccount(
      accountId,
      (await signer.getPublicKey()).toString(),
      parseNearAmount('100'),
    );
    console.log('Account created', accountId);
  } catch (e) {
    console.log('Error creating account', e);
  }
}

async function deployContract(accountId, path) {
  const account = new Account(accountId, provider, signer);
  try {
    await account.deployContract(await readFileSync(path));
    console.log('Contract deployed', path);
  } catch (e) {
    console.log('Error deploying contract', e);
  }
}

async function view({ contractId = NEAR_CONTRACT_ID, methodName, args = {} }) {
  try {
    const res = await provider.callFunction(contractId, methodName, args);
    return res;
  } catch (e) {
    console.log('Error calling', methodName, e);
  }
}

// contract call

async function call({
  contractId = NEAR_CONTRACT_ID,
  methodName,
  args,
  deposit = 0n,
  gas = 18000000000000n,
}) {
  const account = new Account(NEAR_ACCOUNT_ID, provider, signer);

  try {
    const res = await account.callFunction({
      contractId,
      methodName,
      args,
      deposit,
      gas,
    });
    // console.log('Call result:', res === '' ? 'no return value' : res);
    return res;
  } catch (e) {
    console.log('Error calling', methodName, e);
  }
}

// test run const

const VERBOSE = false;
const REDEPLOY_CONTRACT = process.env.DEPLOY_CONTRACT || false;

async function getStats() {
  // get stats
  const [spins, bets, house, payout] = await view({
    methodName: 'stats',
  });
  console.log(
    'bets:',
    bets,
    '\t',
    'wagered:',
    bets / 10,
    '\t',
    'house:',
    formatNearAmount(house, 4),
    '\t',
    'payout:',
    formatNearAmount(payout, 4),
  );
}

async function test() {
  if (REDEPLOY_CONTRACT) {
    await deleteAccount(NEAR_CONTRACT_ID);
    await wait();
    await createAccount(NEAR_CONTRACT_ID);
    await wait();
    await deployContract(
      NEAR_CONTRACT_ID,
      './contract/target/near/contract_rs.wasm',
    );
    await wait();
  }

  // test fts

  const balanceContract = await view({
    contractId: 'usdc.fakes.testnet',
    methodName: 'ft_balance_of',
    args: { account_id: NEAR_ACCOUNT_ID },
  });
  console.log('ft_balance in contract', balanceContract);

  await call({
    contractId: 'usdc.fakes.testnet',
    methodName: 'ft_transfer_call',
    args: {
      receiver_id: NEAR_CONTRACT_ID,
      amount: '1000000000',
      msg: '',
    },
    gas: 300000000000000n,
    deposit: 1n,
  });

  await wait();

  const balance = await view({
    methodName: 'usdc_balance',
    args: { account_id: NEAR_ACCOUNT_ID },
  });
  console.log('ft_balance', balance);

  return;

  // test calls

  let rounds = 0;
  while (true && rounds < 1000) {
    rounds++;

    const bets = [
      // Inside Bets
      {
        kind: 'Straight',
        number: 1,
        amount: parseNearAmount('0.1'),
      },
      {
        kind: 'Split',
        number: 0,
        amount: parseNearAmount('0.1'),
      },
      {
        kind: 'Street',
        number: 0,
        amount: parseNearAmount('0.1'),
      },
      {
        kind: 'Corner',
        number: 0,
        amount: parseNearAmount('0.1'),
      },
      {
        kind: 'SixLine',
        number: 0,
        amount: parseNearAmount('0.1'),
      },
      // Outside Bets
      {
        kind: 'Column',
        number: 0,
        amount: parseNearAmount('0.1'),
      },
      {
        kind: 'Dozen',
        number: 0,
        amount: parseNearAmount('0.1'),
      },
      {
        kind: 'Red',
        number: 0,
        amount: parseNearAmount('0.1'),
      },
      {
        kind: 'Black',
        number: 0,
        amount: parseNearAmount('0.1'),
      },
      {
        kind: 'Odd',
        number: 0,
        amount: parseNearAmount('0.1'),
      },
      {
        kind: 'Even',
        number: 0,
        amount: parseNearAmount('0.1'),
      },
      {
        kind: 'Low',
        number: 0,
        amount: parseNearAmount('0.1'),
      },
      {
        kind: 'High',
        number: 0,
        amount: parseNearAmount('0.1'),
      },
    ];
    const spins = [bets, bets, bets, bets];

    let deposit = BigInt('0');
    for (const bets of spins) {
      for (const bet of bets) {
        deposit += BigInt(bet.amount);
      }
    }
    const spinResults = await call({
      methodName: 'spin',
      args: { spins, callback_gas: 3 },
      deposit,
    });
    let totalMultiple = 0;
    for (const [i, betResults] of spinResults.entries()) {
      for (const [j, betResult] of betResults.entries()) {
        const [win, number, red, multiple] = betResult;
        const payoutMultiple = win ? multiple + 1 : 0;
        if (VERBOSE) {
          console.log(
            'Bet:',
            spins[i][j].kind,
            '\t\tResult:',
            number,
            red ? 'red' : 'black',
            '\t\tPayout:',
            payoutMultiple,
            'x bet',
          );
        }
        totalMultiple += payoutMultiple;
      }
    }
    console.log('total payout: ', totalMultiple);

    await wait();
    await getStats();
  }
}

test();
