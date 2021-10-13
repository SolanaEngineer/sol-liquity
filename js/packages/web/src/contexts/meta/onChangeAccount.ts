import {
  KeyedAccountInfo,
  ProgramAccountChangeCallback,
} from '@solana/web3.js';
import { ProcessAccountsFunc, UpdateStateValueFunc } from './types';

export const onChangeAccount =
  (
    process: ProcessAccountsFunc,
    setter: UpdateStateValueFunc,
    all: boolean,
  ): ProgramAccountChangeCallback =>
  async info => {
    const pubkey = pubkeyByAccountInfo(info);
    await process(
      {
        pubkey,
        account: info.accountInfo,
      },
      setter,
      all,
    );
  };

const pubkeyByAccountInfo = (info: KeyedAccountInfo) => {
  return typeof info.accountId === 'string'
    ? info.accountId
    : info.accountId.toBase58();
};
