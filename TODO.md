# TODOs

- [ ] billing_app refactor
  - [ ] project store should use IntoIterator &mut trait for the hash map
  - [ ] research if `dispatch` and the reducer pattern make sense in Rust
  - [ ] use threads & channels to emit events to mutate the store instead of passing the &mut store around? Store could be read only outside of main()?
- [ ] a32 refactor
  - [ ] get_orders function should initialize the hash map succinctly
  - [ ] get_indexes should not need the entire vector slice
