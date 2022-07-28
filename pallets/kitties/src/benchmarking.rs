//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;
use frame_benchmarking::vec;

benchmarks! {
	// tên của benchmark
	create_kitty {
		// khởi tạo các tham số cho extrinsic benchmark
		let dnas : Vec<u8> = b"lienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlq".to_vec();

		let caller: T::AccountId = whitelisted_caller();
	}: create_kitty (RawOrigin::Signed(caller), dnas)

	// kiểm tra lại trạng thái storage khi thực hiện extrinsic xem đúng chưa
	verify {
		assert_eq!(KittyId::<T>::get(), 1);
	}

	transfer {
		let dnas : Vec<u8> = b"lienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlqlienlq".to_vec();

		let caller: T::AccountId = whitelisted_caller();
		Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into(), dnas);
		let kitty = Kitties::<T>::kitty_owned(caller.clone());
		let transfer_dnas : T::Hash = *kitty.iter().next().unwrap();

		let to_account: T::AccountId = account("Tien Trinh", 1, 1);
	}: transfer (RawOrigin::Signed(caller), to_account, transfer_dnas)

	verify {
		assert_eq!(KittyId::<T>::get(), 1);

		let account: T::AccountId = account("Tien Trinh", 1, 1);
		let kitty = Kitties::<T>::kitty_owned(account);
		assert_eq!(kitty.len(), 1);
	}

	// thực hiện benchmark với mock runtime, storage ban đầu.
	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
