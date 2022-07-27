//! Benchmarking setup for pallet-template

use super::*;
use frame_benchmarking::account;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	// tên của benchmark
	create_kitty {
		// khởi tạo các tham số cho extrinsic benchmark
		let dna : Vec<u8> = b"lienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlq".to_vec();

		let caller: T::AccountId = whitelisted_caller();
	}: create_kitty (RawOrigin::Signed(caller), dna)

	// kiểm tra lại trạng thái storage khi thực hiện extrinsic xem đúng chưa
	verify {
		assert_eq!(KittyId::<T>::get(), 1);
	}

	//benchmark transfer
	transfer {
		let caller: T::AccountId = whitelisted_caller();
		let bob: T::AccountId = account("bob", 0, 0);
		let dna : Vec<u8> = b"lienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlq".to_vec();
		Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into(), dna)?;

		let dna_of_kitty = KittiesOwned::<T>::get(&caller)[0];


	}: transfer(RawOrigin::Signed(caller.clone()), bob.clone(), dna_of_kitty)

	verify {
		assert_eq!(KittiesOwned::<T>::get(&bob)[0], dna_of_kitty);
	}

	// thực hiện benchmark với mock runtime, storage ban đầu.
	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);

}
