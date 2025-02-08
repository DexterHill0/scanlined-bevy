use std::collections::BTreeMap;

use bevy::ecs::{
    change_detection::DetectChanges,
    component::Tick,
    entity::Entity,
    observer::{Observer, Trigger},
    system::{Commands, Query, ResMut, Resource, StaticSystemParam, SystemChangeTick, SystemParam},
    world::{OnRemove, Ref, World},
};
use bevy_mod_index::{
    index::IndexInfo,
    prelude::IndexRefreshPolicy,
    storage::{HashmapStorage, IndexStorage},
};

use crate::{PixelMarker, PixelPosition, UserPixelMarker};

// type ComponentsQuery<'w, 's, T> =
//     Query<'w, 's, (Entity, Ref<'static, <T as IndexInfo>::Component>)>;

// #[derive(SystemParam)]
// pub struct BTreeMapStorageRefreshData<'w, 's, I: IndexInfo> {
//     components: ComponentsQuery<'w, 's, I>,
//     ticks: SystemChangeTick,
// }

// struct BTreeMapStorageIter<'a, V> {
//     inner: Option<&'a V>,
// }
// impl<'a, V> Iterator for BTreeMapStorageIter<'a, V> {
//     type Item = &'a V;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.inner.take()
//     }
// }

// #[derive(Default, Resource)]
// pub struct BTreeMapStorage<I: IndexInfo> {
//     map: BTreeMap<I::Value, Entity>,
//     last_refresh_tick: Tick,
// }

// impl<I: IndexInfo + Default> IndexStorage<I> for BTreeMapStorage<I>
// where
//     <I as IndexInfo>::Value: Default + Ord,
// {
//     type RefreshData<'w, 's> = BTreeMapStorageRefreshData<'w, 's, I>;

//     fn lookup<'w, 's>(
//         &mut self,
//         val: &<I as IndexInfo>::Value,
//         data: &mut StaticSystemParam<Self::RefreshData<'w, 's>>,
//     ) -> impl Iterator<Item = Entity> {
//         if I::REFRESH_POLICY == IndexRefreshPolicy::WhenUsed {
//             self.refresh(data);
//         }

//         BTreeMapStorageIter {
//             inner: self.map.get(val),
//         }
//         .copied()
//     }

//     fn refresh(&mut self, data: &mut StaticSystemParam<Self::RefreshData<'_, '_>>) {
//         if self.last_refresh_tick != data.ticks.this_run() {
//             self.force_refresh(data);
//         }
//     }

//     fn force_refresh(&mut self, data: &mut StaticSystemParam<Self::RefreshData<'_, '_>>) {
//         for (entity, component) in &data.components {
//             if component.last_changed().is_newer_than(
//                 Tick::new(self.last_refresh_tick.get().wrapping_sub(1)),
//                 data.ticks.this_run(),
//             ) {
//                 self.map.insert(I::value(&component), entity);
//             }
//         }
//         self.last_refresh_tick = data.ticks.this_run();
//     }

//     fn removal_observer() -> Option<Observer> {
//         Some(Observer::new(
//             |trigger: Trigger<OnRemove, I::Component>,
//              mut query: Query<&mut I::Component>,
//              mut storage: ResMut<Self>| {
//                 if let Ok(component) = query.get_mut(trigger.entity()) {
//                     storage.map.remove(&I::value(&component));
//                 }
//             },
//         ))
//     }
// }

// impl IndexInfo for UserPixelMarker {
//     type Component = UserPixelMarker;
//     type Value = PixelPosition;

//     type Storage = BTreeMapStorage<Self>;

//     const REFRESH_POLICY: IndexRefreshPolicy = IndexRefreshPolicy::WhenRun;

//     fn value(c: &Self::Component) -> Self::Value {
//         c.pos
//     }
// }

impl IndexInfo for PixelMarker {
    type Component = PixelMarker;
    type Value = PixelPosition;

    type Storage = HashmapStorage<Self>;

    const REFRESH_POLICY: IndexRefreshPolicy = IndexRefreshPolicy::WhenRun;

    fn value(c: &Self::Component) -> Self::Value {
        c.pos
    }
}
