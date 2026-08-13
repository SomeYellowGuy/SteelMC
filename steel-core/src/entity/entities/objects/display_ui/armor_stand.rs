use crate::entity::{Entity, EntityBase, EntityBaseLoad, LivingEntity, LivingEntityBase};
use crate::world::World;
use glam::DVec3;
use std::sync::Weak;
use steel_macros::entity_behavior;
use steel_registry::entity_type::EntityTypeRef;
use steel_registry::vanilla_entity_data::ArmorStandEntityData;
use steel_utils::locks::SyncMutex;
use steel_utils::{DowncastType, DowncastTypeKey};

/// The vanilla armor stand entity.
#[entity_behavior(class = "ArmorStand")]
pub struct ArmorStandEntity {
    base: EntityBase,
    living_base: LivingEntityBase,
    entity_type: EntityTypeRef,
    entity_data: SyncMutex<ArmorStandEntityData>,
}

// SAFETY: This key is owned by Steel and uniquely identifies `ArmorStandEntity`.
unsafe impl DowncastType for ArmorStandEntity {
    const TYPE_KEY: DowncastTypeKey = DowncastTypeKey::new("steel:entity/armor_stand");
}

impl ArmorStandEntity {
    /// Creates a new Armor Stand entity.
    #[must_use]
    pub fn new(entity_type: EntityTypeRef, id: i32, position: DVec3, world: Weak<World>) -> Self {
        Self {
            base: EntityBase::new(id, position, entity_type.dimensions, world),
            living_base: LivingEntityBase::new(entity_type),
            entity_type,
            entity_data: SyncMutex::new(ArmorStandEntityData::new()),
        }
    }

    /// Creates an Armor Stand entity from saved data.
    #[must_use]
    pub fn from_saved(entity_type: EntityTypeRef, load: EntityBaseLoad) -> Self {
        Self {
            base: EntityBase::from_load(load, entity_type.dimensions),
            living_base: LivingEntityBase::new(entity_type),
            entity_type,
            entity_data: SyncMutex::new(ArmorStandEntityData::new()),
        }
    }
}

impl Entity for ArmorStandEntity {
    fn base(&self) -> &EntityBase {
        &self.base
    }

    fn entity_type(&self) -> EntityTypeRef {
        self.entity_type
    }
}

impl LivingEntity for ArmorStandEntity {
    fn living_base(&self) -> &LivingEntityBase {
        &self.living_base
    }

    fn get_health(&self) -> f32 {
        *self.entity_data.lock().living_entity().health.get()
    }

    fn set_health(&self, health: f32) {
        let max_health = self.get_max_health();
        let clamped = health.clamp(0.0, max_health);
        self.entity_data
            .lock()
            .living_entity_mut()
            .health
            .set(clamped);
    }
}
