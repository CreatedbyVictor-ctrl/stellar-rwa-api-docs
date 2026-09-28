//! Tests for `holder_position::get`.

use axum::extract::{Path, State};

use super::holder_position::get;
use super::test_support::{asset, distribution, state_with};
use crate::indexer::Snapshot;
use crate::models::Holder;

#[tokio::test]
async fn aggregates_positions_and_claimable() {
    let mut a = asset(1);
    a.total_supply = "1000".to_string();
    a.valuation_usd = 200.0;
    let mut d = distribution(7, 5);
    d.total_amount = "1000".to_string();
    d.distributed = "200".to_string();
    let mut snapshot = Snapshot { assets: vec![a], ..Default::default() };
    snapshot.holders.insert(
        1,
        vec![Holder { address: "GA".into(), balance: "250".into(), share_percent: 25.0 }],
    );
    snapshot.dividends.insert(1, vec![d]);

    let position = get(State(state_with(snapshot)), Path("GA".into())).await.0;
    assert_eq!(position.asset_count, 1);
    assert_eq!(position.total_value_usd, 50.0);
    assert_eq!(position.positions[0].claimable[0].amount, "200");
}

#[tokio::test]
async fn unknown_address_has_empty_position() {
    let position = get(State(state_with(Snapshot::default())), Path("GX".into())).await.0;
    assert_eq!(position.asset_count, 0);
    assert!(position.positions.is_empty());
}
