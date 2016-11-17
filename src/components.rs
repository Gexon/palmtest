use tinyecs::*;


// платформа
pub struct _ClassGround;

impl Component for _ClassGround {}

// храним последние id, глобальное.
pub struct _WorldLastId {
    pub flora_id: i64,
}

impl Component for _WorldLastId {}

// спавнер
pub struct SpawnPoint;

impl Component for SpawnPoint {}

// метка принадлежности к классу арстений.
pub struct FloraClass;

impl Component for FloraClass {}

// храним текущий id, для конкретного растения.
pub struct _IdHerb {
    pub id: i64,
}

impl Component for _IdHerb {}