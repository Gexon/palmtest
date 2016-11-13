#[macro_use] extern crate tinyecs;

fn main() {
    let mut dk_world = World::new(); // создаем мир компонентной системы.
    init(&mut dk_world);     // инициализация растений.
    loop {dk_world.update()}        // основной цикл ECS.
}

pub fn init(dk_world: &mut World) {
    // добавляем в мир систему спавна.
    dk_world.set_system(SpawnSystem);

    {
        // вносим в этот мир немного земли
        let mut entity_manager = dk_world.entity_manager();
        let entity = entity_manager.create_entity();

        entity.add_component(ClassGround);
        entity.add_component(WorldLastId { flora_id: 0 });
        entity.refresh();
    }


    // добавляем в мир систему роста растений.

    for x in 0..100 {
        //println!("Спавним пальму {}", x); // x: i32

        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint);
        entity_spawner.refresh();
    }
}