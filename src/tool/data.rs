// Copyright (c) 2022 - 2024 GreenYun Organization
// SPDX-License-identifier: MIT

// use std::{sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
// use tokio::{sync::RwLock, time};

pub fn out_dated(last_record: DateTime<Utc>) -> bool {
	let now = Utc::now();
	let update_time = last_record.to_utc();
	(now - update_time).num_days() >= 1
}

// pub fn out_minuted(last_record: DateTime<Utc>) -> Option<DateTime<Utc>> {
// 	let now = Utc::now();
// 	let update_time = last_record.to_utc();
// 	((now - update_time).num_minutes() >= 1).then_some(now)
// }

// pub async fn try_data<T, F, P>(f: F, pred: P) -> Option<T>
// where
//     T: Clone + Send + Sync,
//     F: Fn() -> Arc<RwLock<T>> + Send,
//     P: Fn(&T) -> bool + Send,
// {
//     const MAX_RETRY: usize = 3;

//     for i in 0..=MAX_RETRY {
//         let v = {
//             let arc = f();
//             let lock = arc.read().await;
//             Clone::clone(&*lock)
//         };

//         if pred(&v) {
//             return Some(v);
//         }

//         if i != MAX_RETRY {
//             time::sleep(Duration::from_secs(5)).await;
//         }
//     }

//     None
// }
