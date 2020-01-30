// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019  BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org


//! portablegabi: handles the accumulators for the portablegabi library.

/// Test module for attestations
#[cfg(test)]
mod tests;

use rstd::prelude::*;
use support::{ensure, dispatch::Result, StorageMap, decl_module, decl_storage, decl_event};
use system::{self, ensure_signed};

/// The portablegabi trait
pub trait Trait: system::Trait {
	/// portablegabi specific event type
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event!(
	/// Events for portablegabi
	pub enum Event<T> where <T as system::Trait>::AccountId {
		/// An accumulator has been updated. Therefore an attestation has be revoked
		Updated(AccountId, u64, Vec<u8>),
	}
);

decl_module! {
	/// The portablegabi runtime module
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		/// Deposit events
		fn deposit_event<T>() = default;

		/// Updates the attestation
		/// origin - the origin of the transaction
		/// accumulator - the updated accumulator
		pub fn update_accumulator(origin, accumulator: Vec<u8>) -> Result {
			// origin of the transaction needs to be a signed sender account
			let attester = ensure_signed(origin)?;

			// check how many accumulators are already stored for the sender
			// if the sender has no entry in the counter map, the sender has no
			// accumulators.
			let counter = if !<AccumulatorCount<T>>::exists(&attester) {
				0
			} else {
				<AccumulatorCount<T>>::get(&attester)
			};
			
			// increase the accumulator counter and ensure that there is no
			// accumulator with `index` stored.
			let next = counter.checked_add(1).ok_or("Overflow increasing accumulator index")?;
			ensure!(!<AccumulatorList<T>>::exists((attester.clone(), counter)),
					"Inconsistent accumulator counter");

			// store the Accumulator and update the counter
			<AccumulatorList<T>>::insert((attester.clone(), counter), &accumulator);
			<AccumulatorCount<T>>::insert(&attester, next);

			// deposit an event that the accumulator was updated
			Self::deposit_event(RawEvent::Updated(attester, next, accumulator));
			Ok(())
		}
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as DID {
		/// The AccumulatorList contains all accumulator. It is a map which
		/// maps an account id and an index to an accumulator
		/// AccumulatorList: (account-id, index) -> accumulator
		AccumulatorList get(accumulator_list): map (T::AccountId, u64) => Vec<u8>;

		/// The AccumulatorCounter stores for each attester the number of
		/// accumulator updates.
		/// AccumulatorCount: account-id -> counter
		AccumulatorCount get(accumulator_count): map T::AccountId => u64;
	}
}

