#[macro_use] extern crate tinyecs;

use tinyecs::*;

use systems::*;
use components::*;

mod components;
mod systems;


fn main() {
    let mut dk_world = World::new(); // создаем мир компонентной системы.
    init(&mut dk_world);     // инициализация растений.
    println!("Инициализация завершена, переходим к update");
    loop { dk_world.update() }        // основной цикл ECS.
}

pub fn init(dk_world: &mut World) {
    // добавляем в мир систему спавна.
    dk_world.set_system(SpawnSystem);
    dk_world.set_system(MarkerSystem);
    dk_world.set_system(DeleteSystem);

    {
        // вносим в этот глобальный/единственный базовый элемент "платформа/ground".
        let mut entity_manager = dk_world.entity_manager();
        let entity = entity_manager.create_entity();

        entity.add_component(ClassGround);
        entity.add_component(WorldLastId { flora_id: 0 });
        entity.refresh();
    }

    // добавляем в мир систему роста растений.
    for _x in 0..1_000 {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint);
        entity_spawner.refresh();
        //println!("Создаем спавнер {} для пальмы.", _x);
    }
}