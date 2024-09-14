// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

macro_rules! weather_mods {
	(@ {
		($($m:ident),* $(,)?)
		($($total:tt)*)
		($all_updaters:ident, $updater_type:ident, $count:ident)
	}) => {
		$(
			#[allow(non_snake_case)]
			pub mod $m;

			::paste::paste!{ pub use $crate::weather::$m::[<$m:camel>]; }
		)*

		#[allow(non_camel_case_types)]
		type $updater_type = fn() -> ::tokio::task::JoinHandle<()>;

		#[allow(non_upper_case_globals)]
		const $count: usize = $($total)*;

		#[allow(non_upper_case_globals)]
		const $all_updaters: [&$updater_type; $count] = {
			::paste::paste! {
				$(
					const [<$m:lower _ updater>]: $updater_type = || {
						::tokio::spawn($crate::weather::update_data::<$m::[<$m:camel>]>())
					};
				)+

				[$(&[<$m:lower _ updater>]),+]
			}
		};
	};

	(@ {
		($($m:ident),* $(,)?)
		($($total:tt)*)
		($all_updaters:ident, $updater_type:ident, $count:ident)
	} $x:ident, $($r:tt)* ) => {
		$crate::weather::macros::weather_mods! {
			@ {
				($($m,)* $x,)
				($($total)* + 1)
				($all_updaters, $updater_type, $count)
			}
			$($r)*
		}
	};

	// Entry point
	{
		$(pub mod $x:ident;)+
		const $all_updaters:ident: [&$updater_type:ident; $count:ident];
	} => {
		$crate::weather::macros::weather_mods! {
			@ { () (0) ($all_updaters, $updater_type, $count) }
			$($x,)+
		}
	};
}

pub(super) use weather_mods;
