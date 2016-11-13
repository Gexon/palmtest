use tinyecs::*;

use components::*;


/// Система создает объекты в мире.
pub struct SpawnSystem;

impl System for SpawnSystem {
    // Обрабатываем сущности содержащие компоненты "SpawnPoint"
    // Аспект - список сущностей, содержащих выбранные компоненты.
    fn aspect(&self) -> Aspect {
        aspect_all!(SpawnPoint)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    // обработчик, вызывается при update
    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, world: &mut WorldHandle, data: &mut DataList) {
        let ground = data.unwrap_entity();
        let mut last_id = ground.get_component::<WorldLastId>();

        // перебираем все сущности
        for entity in entities {
            // создаем объект Пальма.
            let entity_object = world.entity_manager.create_entity();
            entity_object.add_component(FloraClass);
            entity_object.add_component(IdHerb { id: last_id.flora_id });
            entity_object.refresh();
            let id_herb = entity_object.get_component::<IdHerb>();
            println!("Создаем пальму {}", id_herb.id);
            last_id.flora_id += 1;


            entity.remove_component::<SpawnPoint>(); // удаляем компонент "Точка спавна/spawn_point"
            entity.refresh();
        }
    }
}