// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

#[derive(Debug)]
struct Luggage<TState> {
    id: usize,
    state: TState,
}

impl<TState> Luggage<TState> {
    fn transition<TNextState>(self, next_state: TNextState) -> Luggage<TNextState> {
        Luggage {
            id: self.id,
            state: next_state,
        }
    }
}

struct CheckIn;
struct OnLoading;
struct Offloading;
#[derive(Debug)]
struct AwaitingPickup;
#[derive(Debug)]
struct EndCustody;

impl Luggage<CheckIn> {
    pub fn new(id: usize) -> Self {
        Self { id, state: CheckIn }
    }

    pub fn load_luggage(self) -> Luggage<OnLoading> {
        self.transition(OnLoading)
    }
}
impl Luggage<OnLoading> {
    pub fn offload(self) -> Luggage<Offloading> {
        self.transition(Offloading)
    }
}
impl Luggage<Offloading> {
    pub fn await_pickup(self) -> Luggage<AwaitingPickup> {
        self.transition(AwaitingPickup)
    }
}
impl Luggage<AwaitingPickup> {
    pub fn pick_luggage(self) -> usize {
        self.id
    }
}

fn main() {
    let awaiting_pickup = Luggage::new(30).load_luggage().offload().await_pickup();

    println!("Awaiting pickup: {awaiting_pickup:?}");
    let picked_up_id = awaiting_pickup.pick_luggage();
    println!("Picked up id: {picked_up_id:?}");
}
