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

use super::*;

use primitives::{H256, Blake2Hasher};
use runtime_primitives::{
	BuildStorage,
	testing::{Digest, DigestItem, Header},
	traits::{BlakeTwo256, IdentityLookup},
};
use support::{assert_ok, impl_outer_origin};
use runtime_io::with_externalities;

impl_outer_origin! {
	pub enum Origin for Test {}
}

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of modules we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;

impl system::Trait for Test {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type Digest = Digest;
	type Log = DigestItem;
}
impl Trait for Test {
	type Event = ();
}
type Portablegabi = Module<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
	system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
}

#[test]
fn check_add_accumulator() {
	with_externalities(&mut new_test_ext(), || {
		assert_ok!(Portablegabi::update_accumulator(
			Origin::signed(1),
			vec![1u8, 2u8, 3u8]
		));
		assert_ok!(Portablegabi::update_accumulator(
			Origin::signed(1),
			vec![4u8, 5u8, 6u8]
		));
		assert_ok!(Portablegabi::update_accumulator(
			Origin::signed(1),
			vec![7u8, 8u8, 9u8]
		));

		// There should be three accumulators inside the store
		assert_eq!(Portablegabi::accumulator_count(1), 3);

		// asserting that the stored value is equal to what we stored
		assert_eq!(Portablegabi::accumulator_list((1, 0)), vec![1u8, 2u8, 3u8]);
		assert_eq!(Portablegabi::accumulator_list((1, 1)), vec![4u8, 5u8, 6u8]);
		assert_eq!(Portablegabi::accumulator_list((1, 2)), vec![7u8, 8u8, 9u8]);
	});
}
