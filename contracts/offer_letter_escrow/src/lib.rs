#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, symbol_short, Address,
    Env, Symbol,
};

pub const STATUS_NONE: u32 = 0;
pub const STATUS_FUNDED: u32 = 1;
pub const STATUS_ACCEPTED: u32 = 2;
pub const STATUS_REPORTED: u32 = 3;
pub const STATUS_RELEASED: u32 = 4;
pub const STATUS_REFUNDED: u32 = 5;

#[contract]
pub struct OfferLetterEscrow;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Offer {
    pub employer: Address,
    pub candidate: Address,
    pub signing_bonus: i128,
    pub status: u32,
    pub evidence_hash: Symbol,
    pub reason: Symbol,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Offer(Symbol),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    OfferAlreadyExists = 1,
    OfferNotFound = 2,
    InvalidSigningBonus = 3,
    SameEmployerAndCandidate = 4,
    UnauthorizedCandidate = 5,
    UnauthorizedEmployer = 6,
    InvalidStatus = 7,
}

#[contractimpl]
impl OfferLetterEscrow {
    pub fn fund_offer(
        env: Env,
        employer: Address,
        candidate: Address,
        offer_id: Symbol,
        signing_bonus: i128,
    ) {
        employer.require_auth();

        if signing_bonus <= 0 {
            panic_with_error!(&env, Error::InvalidSigningBonus);
        }

        if employer == candidate {
            panic_with_error!(&env, Error::SameEmployerAndCandidate);
        }

        let offer_key = DataKey::Offer(offer_id.clone());

        if env.storage().persistent().has(&offer_key) {
            panic_with_error!(&env, Error::OfferAlreadyExists);
        }

        let offer = Offer {
            employer: employer.clone(),
            candidate: candidate.clone(),
            signing_bonus,
            status: STATUS_FUNDED,
            evidence_hash: symbol_short!("none"),
            reason: symbol_short!("none"),
        };

        env.storage().persistent().set(&offer_key, &offer);

        env.events().publish(
            (symbol_short!("funded"), offer_id),
            (employer, candidate, signing_bonus),
        );
    }

    pub fn accept(env: Env, candidate: Address, offer_id: Symbol) {
        candidate.require_auth();

        let offer_key = DataKey::Offer(offer_id.clone());
        let mut offer = Self::read_offer(&env, offer_id.clone());

        if offer.candidate != candidate {
            panic_with_error!(&env, Error::UnauthorizedCandidate);
        }

        if offer.status != STATUS_FUNDED {
            panic_with_error!(&env, Error::InvalidStatus);
        }

        offer.status = STATUS_ACCEPTED;

        env.storage().persistent().set(&offer_key, &offer);

        env.events()
            .publish((symbol_short!("accepted"), offer_id), candidate);
    }

    pub fn report_first_day(
        env: Env,
        employer: Address,
        offer_id: Symbol,
        evidence_hash: Symbol,
    ) {
        employer.require_auth();

        let offer_key = DataKey::Offer(offer_id.clone());
        let mut offer = Self::read_offer(&env, offer_id.clone());

        if offer.employer != employer {
            panic_with_error!(&env, Error::UnauthorizedEmployer);
        }

        if offer.status != STATUS_ACCEPTED {
            panic_with_error!(&env, Error::InvalidStatus);
        }

        offer.status = STATUS_REPORTED;
        offer.evidence_hash = evidence_hash.clone();

        env.storage().persistent().set(&offer_key, &offer);

        env.events()
            .publish((symbol_short!("reported"), offer_id), (employer, evidence_hash));
    }

    pub fn release(env: Env, employer: Address, offer_id: Symbol) {
        employer.require_auth();

        let offer_key = DataKey::Offer(offer_id.clone());
        let mut offer = Self::read_offer(&env, offer_id.clone());

        if offer.employer != employer {
            panic_with_error!(&env, Error::UnauthorizedEmployer);
        }

        if offer.status != STATUS_REPORTED {
            panic_with_error!(&env, Error::InvalidStatus);
        }

        offer.status = STATUS_RELEASED;

        let candidate = offer.candidate.clone();
        let signing_bonus = offer.signing_bonus;

        env.storage().persistent().set(&offer_key, &offer);

        env.events()
            .publish((symbol_short!("released"), offer_id), (candidate, signing_bonus));
    }

    pub fn refund(env: Env, employer: Address, offer_id: Symbol, reason: Symbol) {
        employer.require_auth();

        let offer_key = DataKey::Offer(offer_id.clone());
        let mut offer = Self::read_offer(&env, offer_id.clone());

        if offer.employer != employer {
            panic_with_error!(&env, Error::UnauthorizedEmployer);
        }

        if offer.status != STATUS_ACCEPTED && offer.status != STATUS_REPORTED {
            panic_with_error!(&env, Error::InvalidStatus);
        }

        offer.status = STATUS_REFUNDED;
        offer.reason = reason.clone();

        env.storage().persistent().set(&offer_key, &offer);

        env.events()
            .publish((symbol_short!("refunded"), offer_id), (employer, reason));
    }

    pub fn get_status(env: Env, offer_id: Symbol) -> u32 {
        let offer_key = DataKey::Offer(offer_id);

        let offer: Option<Offer> = env.storage().persistent().get(&offer_key);

        match offer {
            Some(o) => o.status,
            None => STATUS_NONE,
        }
    }

    pub fn get_offer(env: Env, offer_id: Symbol) -> Offer {
        Self::read_offer(&env, offer_id)
    }
}

impl OfferLetterEscrow {
    fn read_offer(env: &Env, offer_id: Symbol) -> Offer {
        match env.storage().persistent().get(&DataKey::Offer(offer_id)) {
            Some(offer) => offer,
            None => panic_with_error!(env, Error::OfferNotFound),
        }
    }
}