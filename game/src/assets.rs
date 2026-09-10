use crate::Id;
use anyhow::Result;
use include_dir::{Dir, include_dir};
use inventory::collect;
use itertools::Itertools;
use polymap::PolyMap;
use serde::{Deserialize, Serialize};
use std::any::{Any, TypeId};
use std::collections::HashMap;

static ASSETS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets");

pub trait Identifiable {
    fn id(&self) -> Id<Self>;
}

pub(crate) struct AssetType {
    pub kind: &'static str,
    pub type_id: TypeId, // to cast back from
    pub load: &'static (dyn Send + Sync + Fn(&AssetType, &'_ mut PolyMap<TypeId>) -> Result<()>),
    pub display: &'static (dyn Send + Sync + Fn(&PolyMap<TypeId>) -> String),
}

inventory::collect!(AssetType);

fn load_one<T: Identifiable + Any + for<'a> Deserialize<'a>>(
    contents: &str,
    file: &dyn std::fmt::Debug,
    items: &mut HashMap<Id<T>, T>,
) -> Result<()> {
    let item = toml::from_str::<T>(contents);
    match item {
        Ok(item) => {
            let t = toml::from_str::<T>(contents)?;
            items.insert(t.id(), t);
        }
        Err(err) => println!("{:?} {}", file, err),
    }
    Ok(())
}

fn load<T: Identifiable + Any + for<'a> Deserialize<'a>>(
    asset: &AssetType,
    store: &mut PolyMap<TypeId>,
) -> Result<()> {
    let items: &mut HashMap<Id<T>, T> = store.entry(TypeId::of::<T>()).or_insert_with(HashMap::new);

    // TODO(feat): Load from user dirs ? #3
    // TODO(correctness): Error handling improvements #4
    let dir = ASSETS_DIR.get_dir(asset.kind.to_lowercase());
    if let Some(dir) = dir {
        for file in dir.files() {
            let contents = file.contents_utf8();
            if let Some(contents) = contents {
                load_one::<T>(contents, file, items)?;
            }
        }
    }
    Ok(())
}

fn display<T: std::fmt::Debug + Any + for<'a> Deserialize<'a>>(store: &PolyMap<TypeId>) -> String {
    let Some(items) = store.get::<TypeId, HashMap<Id<T>, T>>(&TypeId::of::<T>()) else {
        return "?".to_string();
    };
    let out = items.values().map(|item| format!("{:#?}", item))
        .join(", ");
    format!("[{}]", out)
}

impl AssetType {
    pub const fn new<T: Identifiable + std::fmt::Debug + Any + for<'a> Deserialize<'a>>(
        kind: &'static str,
    ) -> Self {
        Self {
            kind,
            type_id: TypeId::of::<T>(),
            load: &load::<T>,
            display: &display::<T>,
        }
    }
    pub fn load_all(self: &AssetType, store: &mut PolyMap<TypeId>) -> Result<()> {
        (self.load)(self, store)
    }
}

#[macro_export]
macro_rules! asset(
    { $key:ident } => {
        inventory::submit! {
            $crate::assets::AssetType::new::<$key>(stringify!($key))
        }
        impl $crate::assets::Identifiable for $key {
            fn id(&self) -> $crate::Id<Self> {
                $crate::Id::new(self.name.to_string())
            }
        }
    };
);
