const log = console.log
console.warn = () => {
}
const {ApiPromise, WsProvider, Keyring} = require('@polkadot/api')
const {encodeAddress, cryptoWaitReady} = require('@polkadot/util-crypto')
const assert = require('assert')
const {stringToU8a} = require('@polkadot/util')
const BigNumber = require('bignumber.js')

const gcd = (a, b) => (b.isZero() ? a : gcd(b, a.modulo(b)));

const ACCOUNT_SECRET = process.env.ACCOUNT_SECRET || '//Alice'
const RPC = process.env.RPC_SERVER || 'ws://127.0.0.1:9988'

let proxyIndex = 2000
const UNIT = 1000000000000

/*
multisig 2/3
  TODO
 */
const multisig = '7KNL7wgm4RoALDP2UPfTXAJhHMacmd4Hbv1jCpWuYerjjgN8' //TODO


const period = 11250;
const daysToBlocks = (days) => Math.floor(days * 24 * 60 * 60 / 6);
const daysToPeriodCount = (days) => Math.floor(days * 24 * 60 * 60 / 6 / period);

const vesting = {
  start: 13517962,
  period,
  per_period: '',
  period_count: daysToPeriodCount(30*12),
}

const teamVesting = {
  start: vesting.start + daysToBlocks(30*6),
  period,
  per_period: '',
  period_count: daysToPeriodCount(30*24),
}

const allocation = {
  angel: [
    ['202500000', vesting]
  ],
  seed: [
    ['45000000', vesting],
    ['45000000', vesting],
    ['45000000', vesting],
    ['22500000', vesting],
    ['22500000', vesting],
    ['22500000', vesting],
    ['22500000', vesting],
    ['18000000', vesting],
    ['16875000', vesting],
    ['14625000', vesting],
    ['11250000', vesting],
    ['11250000', vesting],
    ['11250000', vesting],
    ['11250000', vesting],
    ['11250000', vesting],
    ['4500000', vesting],
    ['2250000', vesting]
  ],
  founders: [
    ['153326250', teamVesting],
    ['144000000', teamVesting],
    ['119362500', teamVesting],
    ['9686250', teamVesting],
    ['107100000', teamVesting]
  ],
  advisors: [
    ['6750000', teamVesting],
    ['6750000', teamVesting],
    ['13500000', teamVesting],
    ['6750000', teamVesting],
    ['1350000', teamVesting]
  ],
  strategic: [
    ['50561797.752809', vesting],
    ['9363295.88014981', vesting],
    ['14981273.4082397', vesting],
    ['13108614.2322097', vesting],
    ['9363295.88014981', vesting],
    ['7490636.70411985', vesting],
    ['3745318.35205993', vesting],
    ['3745318.35205993', vesting],
    ['3745318.35205993', vesting],
    ['1872659.17602996', vesting],
    ['1872659.17602996', vesting],
    ['1872659.17602996', vesting],
    ['1872659.17602996', vesting],
    ['1872659.17602996', vesting],
    ['1872659.17602996', vesting],
    ['1872659.17602996', vesting],
    ['1872659.17602996', vesting],
    ['1498127.34082397', vesting],
    ['374531.835205993', vesting],
    ['1498127.34082397', vesting],
    ['3745318.35205993', vesting]
  ],
  employees: [
    ['33750000', teamVesting],
    ['27000000', teamVesting],
    ['6750000', teamVesting],
    ['6750000', teamVesting],
    ['8100000', teamVesting],
    ['13500000', teamVesting],
    ['1350000', teamVesting],
    ['3375000', teamVesting],
    ['6750000', teamVesting],
    ['6750000', teamVesting],
    ['3375000', teamVesting],
    ['6750000', teamVesting],
    ['3375000', teamVesting],
    ['10125000', teamVesting],
    ['6750000', teamVesting],
    ['3375000', teamVesting],
    ['6750000', teamVesting],
    ['10125000', teamVesting],
    ['76698176.503760355067', teamVesting]
  ]
}

const strategicTotal = '150000000';

const total = vestings => vestings.reduce((acc, [amount]) => acc.plus(amount), new BigNumber(0));

const strategicRemainder = new BigNumber(strategicTotal).minus(total(allocation.strategic));
allocation.strategic.push([strategicRemainder.toFixed(), vesting]);

const totals = {
  angel: total(allocation.angel).toFixed(),
  seed: total(allocation.seed).toFixed(),
  founders: total(allocation.founders).plus(total(allocation.advisors)).toFixed(),
  strategic: total(allocation.strategic).toFixed(),
  employees: total(allocation.employees).toFixed(),
};

const grandTotal = total(Object.values(allocation).flat());

assert.equal(grandTotal, '1499973176.503760355067');
assert.equal(totals.angel, '202500000');
assert.equal(totals.seed, '337500000');
assert.equal(totals.founders, '568575000');
assert.equal(totals.strategic, '150000000');
assert.equal(totals.employees, '241398176.503760355067');

function calculateSchedule([amount, {start, period, period_count}]) {
  const total = new BigNumber(amount).multipliedBy(UNIT)

  const per_period = total
    .div(period_count)
    .decimalPlaces(0, BigNumber.ROUND_FLOOR)
    .toFixed()
  const remainder = total.mod(period_count).toFixed()

  //console.log({total: total.toFixed(), remainder})

  return {
    remainder,
    schedule: {
      start,
      period,
      per_period,
      period_count,
    },
  }
}

const distribution = Object.values(allocation).flat().map(calculateSchedule)

const totalDistributed = distribution
  .reduce(
    (acc, {schedule: {per_period, period_count}, remainder}) =>
      acc
        .plus(remainder)
        .plus(new BigNumber(per_period).multipliedBy(period_count)),
    new BigNumber(0),
  )
  .toFixed()

assert.equal(new BigNumber(grandTotal).multipliedBy(UNIT).toFixed(), totalDistributed, 'total distributed does not match')
//distribution.forEach(({remainder}) => assert.equal(remainder, 0, 'remainder is not zero'));

const hdxAddress = (pubKey) => encodeAddress(pubKey, 42)
const sendAndWait = (from, tx, nonce = -1) =>
  new Promise(async (resolve, reject) => {
    try {
      await tx.signAndSend(from, {nonce}, (receipt) => {
        if (receipt.status.isInBlock) {
          resolve(receipt)
        }
      })
    } catch (e) {
      reject(e)
    }
  })

async function main() {
  await cryptoWaitReady()
  const provider = new WsProvider(RPC)
  const keyring = new Keyring({type: 'sr25519'})
  const api = await ApiPromise.create({provider})
  const [chain, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.version(),
  ])
  log(`connected to ${RPC} (${chain} ${nodeVersion})`)
  const from = keyring.addFromUri(ACCOUNT_SECRET)
  const activeAccount = hdxAddress(from.addressRaw)
  log('active account:', activeAccount)
  const vestingPubKey = stringToU8a('modlpy/vstng'.padEnd(32, '\0'))
  const vestingAddress = hdxAddress(vestingPubKey)
  log('vestingAddress account:', vestingAddress)
  log('controller multisig:', multisig)
  log('total to be distributed:', grandTotal.toFixed())

  const grandTotalTotal = grandTotal.multipliedBy(UNIT).toFixed()
  log(grandTotalTotal)

  log('creating anonymous proxies...')
  const proxies = distribution.map(() =>
    api.tx.proxy.anonymous('Any', 0, proxyIndex++),
  )
  const receipt1 = await sendAndWait(from, api.tx.utility.batchAll(proxies))
  const anonymousProxies = receipt1.events
    .filter(({event}) => event.method === 'AnonymousCreated')
    .map(({event}) => event.data.anonymous.toHuman())
  assert.equal(
    anonymousProxies.length,
    distribution.length,
    'not all proxies created',
  )
  log('proxies created:', anonymousProxies)
  log('gc proxy:', anonymousProxies[anonymousProxies.length - 1])

  log(`funding proxies ${anonymousProxies.length}...`)
  const transfers = anonymousProxies.map((anon) =>
    api.tx.balances.forceTransfer(activeAccount, anon, 1000 * UNIT),
  )
  const receipt2 = await sendAndWait(
    from,
    api.tx.sudo.sudo(api.tx.utility.batchAll(transfers)),
  )
  const transferEvents = receipt2.events.filter(
    ({event}) => event.method === 'Transfer',
  )
  assert.equal(
    transferEvents.length,
    anonymousProxies.length,
    'not all proxies funded',
  )
  log('all proxies funded')

  log('changing delegate to multisig...')
  const changes = anonymousProxies.map((anon) =>
    api.tx.proxy.proxy(
      anon,
      null,
      api.tx.utility.batchAll([
        api.tx.proxy.removeProxy(activeAccount, 'Any', 0),
        api.tx.proxy.addProxy(multisig, 'Any', 0),
      ]),
    ),
  )
  const receipt3 = await sendAndWait(from, api.tx.utility.batchAll(changes))
  const newDelegates = receipt3.events
    .filter(({event}) => event.method === 'ProxyAdded')
    .map(({event}) => event.data.delegatee.toHuman())
  newDelegates.forEach((delegate) =>
    assert.equal(delegate, multisig, 'not all proxies delegated to multisig'),
  )
  log('all proxies delegated to multisig')

  log('distributing funds...')
  const toVestingAddress = api.tx.sudo.sudo(api.tx.balances.forceTransfer(activeAccount, vestingAddress, grandTotalTotal))
  const vestings = distribution
    .map(({remainder, schedule}, i) =>
      api.tx.sudo.sudo(
        api.tx.vesting.vestedTransfer(anonymousProxies[i], schedule),
      ));
  const receipt4 = await sendAndWait(
    from,
    api.tx.utility.batchAll([toVestingAddress, ...vestings]),
  )
  const transferred = receipt4.events
    .filter(({event}) => event.method === 'Transfer')
    .map(({event}) => event.data.amount.toString())
    .reduce((a, num) => a.plus(num), new BigNumber(0))
    .minus(grandTotalTotal)
    .toFixed()
  assert.equal(transferred, grandTotalTotal, 'difference between total and transferred')
  log('funds distributed:', transferred)
}

main()
  .then(() => {
    process.exit(0)
  })
  .catch((e) => {
    console.error(e)
    process.exit(1)
  })
