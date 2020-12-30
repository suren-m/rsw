// player and health

// players - collection

struct Inventory {}

enum BodyType {
    Light,
    Medium,
    Heavy,
}

const MAX_HEALTH: u8 = 100;
struct Player {
    id: i32,
    name: String,
    health: u8,
    inventory: Inventory,
    body_type: BodyType,
    avatar: char,
}

// state
// collection of players
