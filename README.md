# offer_letter_escrow

## Project Title
offer_letter_escrow

## Project Description
`offer_letter_escrow` is a Soroban smart contract that turns a signed job offer letter into a self-custody escrow on the Stellar network. The employer locks the candidate's signing bonus inside the contract the moment the offer is funded, and the funds can only move to one of two outcomes: the candidate (when they actually show up for day one) or back to the employer (when they don't). Every state transition is signed by the right party, timestamped by the ledger, and emitted as a public event, so neither side can quietly walk away with the other side's money.

The whole flow is small on purpose — one `fund_offer`, one `accept`, one `report_first_day`, and a single `release` or `refund` to close it out — so a candidate or an HR manager can read the contract, understand it in five minutes, and trust it without trusting a third party.

## Project Vision
The vision is to make every job-offer letter a small, transparent financial agreement on-chain. We want to give candidates the confidence to relocate for a new role and give employers the assurance that a no-show candidate will not silently walk away with a bonus. In the long term, `offer_letter_escrow` can become the trust layer for global remote hiring, contractor onboarding, internship placements, university recruitment, and even apprenticeship programs — anywhere that a written offer can benefit from a programmable, neutral, and auditable payment escrow.

## Key Features
- **Deterministic lifecycle escrow** — every offer moves through a fixed `funded -> accepted -> reported -> released | refunded` state machine, with each transition gated by `require_auth` for the correct party.
- **Split-protected signing bonus** — the bonus is locked in the contract at `fund_offer`, the candidate commits with `accept`, and the funds can only be released after the employer confirms first-day attendance, or refunded on a no-show.
- **On-chain evidence trail** — the first-day report carries a short `evidence_hash` (e.g. a SHA-256 of a badge scan, an HR system record id, or a hash of an attestation URI) so the off-chain proof of attendance is auditable directly from the ledger.
- **Strict authorization** — `require_auth` is enforced on the employer for `fund_offer`, `report_first_day`, `release`, and `refund`, and on the candidate for `accept`, so no one can mutate an offer they do not own.
- **Public status view** — `get_status` returns a single integer status code (0–5) for any `offer_id`, so a frontend or audit tool can render the current state of the offer without re-deriving the lifecycle.
- **Event-driven integration** — every transition publishes a topic+data event (`offer_funded`, `offer_accepted`, `first_day_reported`, `bonus_released`, `bonus_refunded`) so an off-chain relayer, indexer, or HR dashboard can subscribe to the lifecycle.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** work dApp — see `contracts/offer_letter_escrow/src/lib.rs` for the full offer_letter_escrow business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `CD4LHGLFKQYA6EUL645VWCEJ2GARLMLHUTCIZNRABW5UQ6KH5JWJBBGI`
- **Explorer template:** `https://stellar.expert/explorer/testnet/tx/b7cb8b7ad3eb301d833bc27fda8a66352ed3c4662dd2a167830a23272c1b8ab1`

## Future Scope
- **Real asset settlement** — wire `release` and `refund` to a Stellar asset contract (USDC or a custom payroll token) so the bonus is actually moved on-chain instead of just signaled through events.
- **Time-locked auto-refund** — add a `start_date` ledger timestamp to each offer so the contract can auto-refund if the candidate never reports by a deadline, removing the need for the employer to actively call `refund`.
- **Dispute window** — give the candidate a short challenge window after `report_first_day` to dispute the report before funds are released, with a simple employer/candidate/arbiter resolution path.
- **Multi-party offers** — extend `Offer` to support co-signers (e.g. a recruitment agency that paid a referral fee) and split the bonus across multiple payouts in one release.
- **Frontend dApp** — build a minimal web UI using Freighter that lets a candidate accept an offer and an employer fund / report / release / refund with one click, reading status via `get_status`.
- **Indexed event feed** — index the contract's events (`offer_funded`, `offer_accepted`, `bonus_released`, `bonus_refunded`) into a small dashboard so HR teams can watch the status of every offer in real time.
- **Testnet end-to-end demo** — deploy to Stellar Testnet, run a full `fund -> accept -> report -> release` flow, and capture the contract ID plus the invoke transaction hash for the README.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `offer_letter_escrow` (work)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
