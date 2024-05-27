### About

I found that 2 available generators [Maldon](https://github.com/flood-protocol/maldon) and [AmanRaj](https://github.com/AmanRaj1608/create3/tree/master) does not work well with[ LiFi create3 factory](https://github.com/lifinance/create3-factory/blob/main), since LiFi factory is hashing input salt with msg.sender to mitigate possible frontrun of smart contract deployment under known address, which seems to not be included in those approaches.

### Usage

FACTORY address and PROXY_BYTECODE are default to be compatible with LiFi factory, but you can specify your own.

```sh
Usage: {} <num_threads> <deployer_address> <prefix> [FACTORY | PROXY_BYTECODE]
```

### Example

```sh
❯ cargo run 16 0xd8da6bf26964af9d7eed9e03e53415d37aa96045 0x333
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/my-create3 16 0xd8da6bf26964af9d7eed9e03e53415d37aa96045 0x333`
Found salt: 0x31b28b6d48af50aeda39b5a1f1994fdfa5b214be720bedcb36f4aff75a0db2c0
Deployed address: 0x333015bf3a8e2c1d1216b6cda9bcf8cd05893c4f
Time taken: 14.915125ms
```
