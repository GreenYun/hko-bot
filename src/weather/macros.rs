// Copyright (c) 2022 GreenYun Organizaiton
// SPDX-License-identifier: MIT

macro_rules! impl_update {
    ($self:ident) => {
        pub async fn update() {
            use hko::{common::Lang, fetch};
            use paste::paste;

            use crate::tool::macros::unwrap_or_execute;

            let chinese = unwrap_or_execute!(fetch(Lang::TC).await, |e| {
                log::error!("{}", e);
                return;
            });

            let english = unwrap_or_execute!(fetch(Lang::EN).await, |e| {
                log::error!("{}", e);
                return;
            });

            paste! { let b = [< $self:camel >] ::new(chinese, english); }

            {
                paste! { let arc = super:: [< $self:lower >] (); }
                let mut lock = arc.write().await;
                *lock = b;
            }
        }
    };
}

pub(super) use impl_update;
