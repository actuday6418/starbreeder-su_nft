use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Trixel {
  pub x: u32,
  pub y: u32,
  pub z: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub dso_name: String,
    pub dso_coordinate: Trixel,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");
