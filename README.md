## Cross Program Invocations (Anchor)

Small Anchor example showing one program (`master`) performing a CPI into another program (`child`) to update a stored `u64`. The TypeScript test drives both programs locally.

### What the programs do
- `child`: owns a `Data` account with `data: u64` and an `authority` pubkey. Only the authority can change the stored value.
- `master`: builds a CPI context to call `child::set_data` and asserts the value becomes `42` after the call.
- `tests/child.ts`: initializes the `child` account, then has `master` set the value over CPI, and checks the result.

### Prerequisites
- Rust + Cargo
- Node 16+ with yarn
- Anchor CLI
- Solana CLI with a local validator (`solana-test-validator`) or access to a cluster

### Install deps
```bash
yarn install
cargo build-sbf
```

### Run the test suite
```bash
anchor test
```

### Deploy manually (optional)
```bash
anchor build
anchor deploy
```

### Key files
- `programs/child/src/lib.rs` – stores the data and enforces the authority.
- `programs/master/src/lib.rs` – performs the CPI into `child`.
- `tests/child.ts` – end-to-end test that exercises the CPI flow.

