import { publicKey, u64 } from '@solana/buffer-layout-utils';
import { u8, struct } from '@solana/buffer-layout';


export const ESCROW_ACCOUNT_DATA_LAYOUT = struct([
  u8("isInitialized"),
  publicKey("initializerPubkey"),
  publicKey("initializerTempTokenAccountPubkey"),
  publicKey("initializerReceivingTokenAccountPubkey"),
  u64("expectedAmount"),
]);

export interface EscrowLayout {
  isInitialized: number,
  initializerPubkey: Uint8Array,
  initializerReceivingTokenAccountPubkey: Uint8Array,
  initializerTempTokenAccountPubkey: Uint8Array,
  expectedAmount: Uint8Array
}