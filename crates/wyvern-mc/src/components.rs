pub trait ComponentHolder<R: ComponentRegistry<Self>>: Sized {
    fn get<C: Component<Self, R, V>, V>(&self, component: &C) -> Option<V> {
        component.get_component(self)
    }

    fn set<C: Component<Self, R, V>, V>(&mut self, component: &C, value: V) {
        component.insert_component(self, value);
    }

    fn with<C: Component<Self, R, V>, V>(mut self, component: &C, value: V) -> Self {
        component.insert_component(&mut self, value);
        self
    }
}

pub trait ComponentRegistry<H: ComponentHolder<Self>>: Sized {}

pub trait Component<H: ComponentHolder<R>, R: ComponentRegistry<H>, V> {
    fn insert_component(&self, holder: &mut H, value: V);
    fn get_component(&self, holder: &H) -> Option<V>;
}

#[cfg(test)]
mod tests {
    use crate::{dimension::entity::EntityType, values::Key};

    use super::{Component, ComponentHolder, ComponentRegistry};

    struct Entity {
        name: String,
        kind: Key<EntityType>,
    }

    impl ComponentHolder<EntityComponents> for Entity {}

    struct EntityComponents {}

    impl ComponentRegistry<Entity> for EntityComponents {}

    impl EntityComponents {
        const CUSTOM_NAME: CustomName = CustomName;
        const ENTITY_KIND: EntityKind = EntityKind;
    }

    struct CustomName;

    impl Component<Entity, EntityComponents, String> for CustomName {
        fn insert_component(&self, holder: &mut Entity, value: String) {
            holder.name = value;
        }

        fn get_component(&self, holder: &Entity) -> Option<String> {
            Some(holder.name.clone())
        }
    }

    struct EntityKind;

    impl Component<Entity, EntityComponents, Key<EntityType>> for EntityKind {
        fn insert_component(&self, holder: &mut Entity, value: Key<EntityType>) {
            holder.kind = value;
        }

        fn get_component(&self, holder: &Entity) -> Option<Key<EntityType>> {
            Some(holder.kind.clone())
        }
    }

    #[test]
    fn entity() {
        let entity = Entity {
            name: "Zombie".to_string(),
            kind: Key::new("minecraft", "zombie"),
        };
        let kind = entity.get(&EntityComponents::ENTITY_KIND);
        assert_eq!(kind, Some(Key::new("minecraft", "zombie")));
        let name = entity.get(&EntityComponents::CUSTOM_NAME);
        assert_eq!(name, Some("Zombie".to_string()));
    }
}
