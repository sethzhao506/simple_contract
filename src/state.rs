use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map, Index, IndexList, IndexedMap, Item, MultiIndex, UniqueIndex};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
  pub owner: Addr,
  pub score: i32,
  pub identifier: u8, // <---- unique value
}

// TokenIndexes structs keeps a list of indexers
pub struct StateIndexes<'a> {
  // token.identifier
  pub identifier: UniqueIndex<'a, U8Key, State>,
}

// IndexList is just boilerplate code for fetching a struct's indexes
impl<'a> IndexList<State> for StateIndexes<'a> {
  fn get_indexes(&'_ self) -> Box<dyn Iterator<Item=&'_ dyn Index<State>> + '_> {
    let v: Vec<&dyn Index<State>> = vec![&self.identifier];
    Box::new(v.into_iter())
  }
}

// storage access function.
pub fn states<'a>() -> IndexedMap<'a, &'a [u8], State, StateIndexes<'a>> {
  let indexes = StateIndexes {
    identifier: UniqueIndex::new(|d| U8Key::new(d.identifier), "state_identifier"),
  };
  IndexedMap::new(TOKEN_NAMESPACE, indexes)
}

pub const STATE: Item<State> = Item::new("state");
