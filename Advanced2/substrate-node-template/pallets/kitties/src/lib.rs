#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use codec::{Decode, Encode};
	use frame_support::{
		dispatch::DispatchResult,
		pallet_prelude::*,
		traits::{Currency, Randomness},
	};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_io::hashing::blake2_128;

	#[derive(Encode, Decode, TypeInfo)]
	pub struct Kitty(pub [u8; 16]);

	type KittyIndex = u32;
	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	// type BalanceOf<T> =
	// <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new Kitty was sucessfully created. \[sender, kitty_id\]
		Created(T::AccountId, KittyIndex),
		// /// Kitty price was sucessfully set. \[sender, kitty_id, new_price\]
		// PriceSet(T::AccountId, T::Hash, Option<BalanceOf<T>>),
		/// A Kitty was sucessfully transferred. \[from, to, kitty_id\]
		Transferred(T::AccountId, T::AccountId, KittyIndex),
		// /// A Kitty was sucessfully bought. \[buyer, seller, kitty_id, bid_price\]
		// Bought(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),
	}

	#[pallet::storage]
	#[pallet::getter(fn kitties_count)]
	pub type KittiesCount<T: Config> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> =
		StorageMap<_, Blake2_128Concat, KittyIndex, Option<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> =
		StorageMap<_, Blake2_128Concat, KittyIndex, Option<Kitty>, ValueQuery>;

	#[pallet::error]
	pub enum Error<T> {
		/// Handles arithemtic overflow when incrementing the Kitty counter.
		KittiesCountOverflow,
		InvalidKittyIndex,
		// /// An account cannot own more Kitties than `MaxKittyCount`.
		// ExceedMaxKittyOwned,
		// /// Buyer cannot be the owner.
		// BuyerIsKittyOwner,
		// /// Handles checking whether the Kitty exists.
		// KittyNotExist,
		/// Handles checking that the Kitty is owned by the account transferring, buying or setting a price for it.
		NotKittyOwner,
		// /// Ensures the Kitty is for sale.
		// KittyNotForSale,
		// /// Ensures that the buying price is greater than the asking price.
		// KittyBidPriceTooLow,
		// /// Ensures that an account has enough funds to purchase a Kitty.
		// NotEnoughBalance,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty_id = match Self::kitties_count() {
				Some(id) => {
					ensure!(id != KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				}
				None => 1,
			};

			let dna = Self::random_value(&who);
			Kitties::<T>::insert(kitty_id, Some(Kitty(dna)));
			Owner::<T>::insert(kitty_id, Some(who.clone()));
			KittiesCount::<T>::put(kitty_id + 1);
			Self::deposit_event(Event::Created(who, kitty_id));
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn transfer(origin: OriginFor<T>, new_owner: T::AccountId, kitty_id: KittyIndex,) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Some(who.clone()) == Owner::<T>::get(kitty_id), <Error<T>>::NotKittyOwner);
			Owner::<T>::insert(kitty_id, Some(new_owner.clone()));

			Self::deposit_event(Event::Transferred(who, new_owner, kitty_id));
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn breed(origin: OriginFor<T>, kitty_id_1: KittyIndex, kitty_id_2: KittyIndex,) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyIndex)?;
			let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyIndex)?;

			let kitty_id = match Self::kitties_count() {
				Some(id) => {
					ensure!(id != KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
					id
				},
				None => {
					1
				}
			};

			let dna1 = kitty1.0;
			let dna2 = kitty2.0;

			let selector = Self::random_value(&who);
			let mut new_dna = [0u8; 16];

			for i in 0..dna1.len() {
				new_dna[i] = (selector[i] & dna1[i]) | (!selector[i] & dna2[i]);
			}

			Kitties::<T>::insert(kitty_id, Some(Kitty(new_dna)));
			Owner::<T>::insert(kitty_id, Some(who.clone()));
			KittiesCount::<T>::put(kitty_id+1);
			Self::deposit_event(Event::Created(who, kitty_id));

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
		}
	}
}
