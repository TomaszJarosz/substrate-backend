#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::*;
    use frame_support::{pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::error]
    pub enum Error<T> {
        StorageOverflow,
        ForbiddenFive,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        ValueStored { value: u32, who: T::AccountId },
    }

    #[pallet::storage]
    #[pallet::getter(fn adding_machine_storage)]
    pub type AddingMachineStorage<T> = StorageValue<_, u32>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1_000)]
        pub fn reset_to_0(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            <AddingMachineStorage<T>>::put(0);
            Self::deposit_event(Event::ValueStored { value: 0, who });
            Ok(())
        }

        #[pallet::weight(1_000)]
        pub fn add(origin: OriginFor<T>, value: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            match <AddingMachineStorage<T>>::get() {
                None => {
                    <AddingMachineStorage<T>>::put(value);
                    Self::deposit_event(Event::ValueStored { value, who });
                }
                Some(old) => {
                    let new = old.checked_add(value).ok_or(Error::<T>::StorageOverflow)?;
                    <AddingMachineStorage<T>>::put(new);
                    Self::deposit_event(Event::ValueStored { value: new, who });
                }
            };
            Ok(())
        }

        #[pallet::weight(1_000)]
        pub fn add_but_five_is_forbidden(origin: OriginFor<T>, value: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(value != 5,  <Error<T>>::ForbiddenFive);

            match <AddingMachineStorage<T>>::get() {
                None => {
                    <AddingMachineStorage<T>>::put(value);
                    Self::deposit_event(Event::ValueStored { value, who });
                }
                Some(old) => {
                    let new = old.checked_add(value).ok_or(Error::<T>::StorageOverflow)?;
                    <AddingMachineStorage<T>>::put(new);
                    Self::deposit_event(Event::ValueStored { value: new, who });
                }
            };
            Ok(())
        }
    }
}
