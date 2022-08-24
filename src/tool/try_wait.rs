// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

use std::{sync::Arc, time::Duration};

use tokio::{sync::RwLock, time};

pub async fn try_data<T, F, P>(f: F, pred: P) -> Option<T>
where
    T: Clone,
    F: Fn() -> Arc<RwLock<T>>,
    P: Fn(&T) -> bool,
{
    const MAX_RETRY: usize = 3;

    let v = {
        let arc = f();
        let lock = arc.read().await;
        Clone::clone(&*lock)
    };

    if pred(&v) {
        return Some(v);
    }

    for _ in 0..MAX_RETRY {
        time::sleep(Duration::from_secs(5)).await;

        let v = {
            let arc = f();
            let lock = arc.read().await;
            Clone::clone(&*lock)
        };

        if pred(&v) {
            return Some(v);
        }
    }

    None
}
