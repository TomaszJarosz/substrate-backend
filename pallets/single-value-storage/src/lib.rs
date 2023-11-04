#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;

pub use weights::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::storage]
    #[pallet::getter(fn stored_value)]
    pub type SingleValueStorage<T> = StorageValue<_, u32>;

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        ValueStored { value: u32, who: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::call_index(0)]
        #[pallet::weight(1_000)]
        pub fn set_value(origin: OriginFor<T>, value: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Update storage.
            <SingleValueStorage<T>>::put(&value);

            // Emit an event.
            Self::deposit_event(Event::ValueStored { value, who });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(1_000)]
        pub fn increment_value_by_1(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Read a value from storage.
            match <SingleValueStorage<T>>::get() {
                // Return an error if the value has not been set.
                None => return Err(Error::<T>::NoneValue.into()),
                Some(old) => {
                    // Increment the value read from storage; will error in the event of overflow.
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    // Update the value in storage with the incremented result.
                    <SingleValueStorage<T>>::put(&new);
                    // Emit an event.
                    Self::deposit_event(Event::ValueStored { value: new, who });
                    Ok(())
                }
            }
        }
    }
}
