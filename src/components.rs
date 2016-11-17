use tinyecs::*;


// платформа
pub struct ClassGround;

impl Component for ClassGround {}

// храним последние id, глобальное.
pub struct WorldLastId {
    pub flora_id: i64,
}

impl Component for WorldLastId {}

// спавнер
pub struct SpawnPoint;

impl Component for SpawnPoint {}

// метка принадлежности к классу арстений.
pub struct FloraClass;

impl Component for FloraClass {}

// метка для удаления сущности.
pub struct Dead;

impl Component for Dead {}

// храним текущий id, для конкретного растения.
pub struct IdHerb {
    pub id: i64,
}

impl Component for IdHerb {}