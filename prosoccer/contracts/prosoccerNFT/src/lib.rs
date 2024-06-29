#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, Map, Symbol};

pub struct ProsoccerCollection {
    token_id: i64,
    listings: Map<i64, NFTListing>,
}

pub struct NFTListing {
    token_id: i64,
    price: i64,
    seller: Address,
    token_uri: Symbol,
}

#[contract]
pub struct ProsoccerContract;

pub struct MintEvent {
    token_id: i64,
    minter: Address,
    price: i64,
    token_uri: Symbol,
}

pub struct BuyEvent {
    token_id: i64,
    buyer: Address,
    seller: Address,
    price: i64,
}

pub struct DelistEvent {
    token_id: i64,
    seller: Address,
}

#[contractimpl]
impl ProsoccerContract {
    pub fn init(env: Env, initial_owner: Address, name: Symbol, symbol: Symbol) {
        // Initialize contract with owner, name, and symbol
        env.storage().persistent().set(b"owner", &initial_owner);
        env.storage().persistent().set(b"name", &name.clone());
        env.storage().persistent().set(b"symbol", &symbol.clone());

        // Initialize the token_id_counter to 0
        env.storage().persistent().set(b"token_id_counter", &0i64);
    }

    pub fn mint(env: Env, token_uri: Symbol, initial_price: i64, minter: Address) {
        if minter != env.current_contract_address() {
            minter.require_auth();
        }

        // Generate a unique token ID
        let mut token_id_counter: i64 = env
            .storage()
            .persistent()
            .get(b"token_id_counter")
            .unwrap_or(0);
        token_id_counter += 1;
        let token_id = token_id_counter;

        // Create the NFTListing struct
        let listing = NFTListing {
            token_id,
            price: initial_price,
            seller: minter.clone(),
            token_uri: token_uri.clone(),
        };

        // Add the listing to the `listings` map
        let mut listings: Map<i64, NFTListing> = env
            .storage()
            .instance()
            .get(b"listings")
            .unwrap_or_default();
        listings.insert(token_id, listing);
        env.storage().instance().set(b"listings", listings);

        // Update the token ID counter
        env.storage()
            .persistent()
            .set(b"token_id_counter", token_id_counter);

        // Emit MintEvent
        let event = MintEvent {
            token_id,
            minter,
            price: initial_price,
            token_uri,
        };
        env.events().publish(event);
    }

    pub fn delist(env: Env, seller: Address, token_id: i64) {
        seller.require_auth();

        let mut listings: Map<i64, NFTListing> = env
            .storage()
            .instance()
            .get(b"listings")
            .unwrap_or_default();

        if let Some(listing) = listings.get(&token_id) {
            if listing.seller != seller {
                panic!("Only the seller can delist this token.");
            }

            listings.remove(&token_id);
            env.storage().instance().set(b"listings", listings);

            // Emit DelistEvent
            let event = DelistEvent {
                token_id,
                seller: seller.clone(),
            };
            env.events().publish(event);
        } else {
            panic!("Token ID not found in listings.");
        }
    }

    pub fn buy(env: Env, buyer: Address, token_id: i64) {
        buyer.require_auth();

        let mut listings: Map<i64, NFTListing> = env
            .storage()
            .instance()
            .get(b"listings")
            .unwrap_or_default();

        if let Some(listing) = listings.get(&token_id) {
            if !Self::check_balance(env.clone(), &buyer, listing.price as u64) {
                panic!("Buyer does not have enough balance.");
            }

            if Self::transfer_funds(env.clone(), &buyer, &listing.seller, listing.price as u64) {
                listings.remove(&token_id);
                env.storage().instance().set(b"listings", listings);

                // Emit BuyEvent
                let event = BuyEvent {
                    token_id,
                    buyer: buyer.clone(),
                    seller: listing.seller.clone(),
                    price: listing.price,
                };
                env.events().publish(event);
            } else {
                panic!("Failed to transfer funds.");
            }
        } else {
            panic!("Token ID not found in listings.");
        }
    }

    fn check_balance(env: Env, buyer: &Address, price: u64) -> bool {
        // Placeholder
        return true;
    }

    fn transfer_funds(env: Env, buyer: &Address, seller: &Address, amount: u64) -> bool {
        // Placeholder
        return true;
    }
}
