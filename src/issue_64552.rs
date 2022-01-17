// This fails because there is a custom Send impl on a NodeRef type inside of
// BTreeMap which relates some regions.
// I tried to store the region predicates and augment them in the param-env,
// but I don't think we can relate late bound regions at all.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

fn needs_send<T: Send>(_val: T) {}

async fn async_fn_a(_num: u32) {}

async fn async_fn_b(map: Arc<BTreeMap<u32, &'static u32>>) {
    for (_i, v) in &*map {
        async_fn_a(**v).await;
    }
}

fn test() {
    let map: Arc<BTreeMap<u32, &'static u32>> = Arc::new(BTreeMap::new());
    needs_send(async_fn_b(map.clone()));
}
