#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::*;
    use frame_support::{pallet_prelude::*, sp_runtime::print, log::*};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1_000)]
        pub fn say_hello(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            print("Hello World");
            info!("called by {:?}", who);

            Ok(())
        }
    }
}
