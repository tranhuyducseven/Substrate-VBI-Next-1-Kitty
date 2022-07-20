#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::inherent::Vec;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;
	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		dna: Vec<u8>,
		owner: T::AccountId,
		price: u32,
		gender: Gender,
	}
	pub type Amount = u32;
	pub type Dna = Vec<u8>;

	#[derive(TypeInfo, Encode, Decode, Debug)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender {
		fn default() -> Self {
			Gender::Male
		}
	}
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);
	#[pallet::storage]
	#[pallet::getter(fn number_of_kittens)]
	pub type NumberOfKittens<T> = StorageValue<_, Amount, ValueQuery>;

	// key : dna
	//value : kitty
	#[pallet::storage]
	#[pallet::getter(fn kitty)]
	pub(super) type KittyList<T: Config> =
		StorageMap<_, Blake2_128Concat, Dna, Kitty<T>, OptionQuery>;

	// key : accountId
	//value : dna
	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub(super) type KittyOfOwnerList<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Dna>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyStored(Vec<u8>, u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		StorageOverflow,
		NotOwner,
	}

	//extrinsic
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let gender = Self::gen_gender(dna.clone())?;
			let kitty = Kitty { dna: dna.clone(), owner: who.clone(), price, gender };
			// add a new kitty
			<KittyList<T>>::insert(dna.clone(), kitty);

			//update number_of_kittens
			let mut current_number_of_kittens = <NumberOfKittens<T>>::get();
			current_number_of_kittens += 1;
			<NumberOfKittens<T>>::put(current_number_of_kittens);

			//update kitty of owner
			<KittyOfOwnerList<T>>::mutate(who.clone(), |dna_list| {
				match dna_list {
					Some(dna_list) => dna_list.push(dna.clone()),
					None => {
						let mut list = Vec::new();
						list.push(dna.clone());
						*dna_list = Some(list);
					},
				};
			});

			// Emit an event.
			Self::deposit_event(Event::KittyStored(dna, price));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn swap_owner(
			origin: OriginFor<T>,
			dna: Dna,
			new_owner: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			//get kitty
			let kitty = <KittyList<T>>::get(dna.clone());
			//check owner

			//update owner
			match kitty {
				Some(mut tmp_kitty) => {
					ensure!(tmp_kitty.owner == who, Error::<T>::NotOwner);
					tmp_kitty.owner = new_owner.clone();
					<KittyList<T>>::insert(dna.clone(), tmp_kitty);
					//remove old owner's list
					<KittyOfOwnerList<T>>::mutate(who.clone(), |dna_list| {
						match dna_list {
							Some(dna_list) => {
								dna_list.retain(|x| x != &dna);
							},
							None => {},
						};
					});

					//update new owner's list
					<KittyOfOwnerList<T>>::mutate(new_owner.clone(), |dna_list| {
						match dna_list {
							Some(dna_list) => dna_list.push(dna.clone()),
							None => {
								let mut list = Vec::new();
								list.push(dna.clone());
								*dna_list = Some(list);
							},
						};
					});
				},
				None => {},
			}

			Ok(())
		}
	}
}

// helper function

impl<T> Pallet<T> {
	fn gen_gender(dna: Vec<u8>) -> Result<Gender, Error<T>> {
		let mut res = Gender::Female;
		if dna.len() % 2 == 0 {
			res = Gender::Male;
		}
		Ok(res)
	}
}
