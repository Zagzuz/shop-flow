use crate::catalog_proto::Item as ProtoItem;
use compact_str::CompactString;

#[derive(Default)]
pub struct Catalog {
    items: Vec<Item>,
}

impl Catalog {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn list_items(&self) -> &Vec<Item> {
        &self.items
    }
}

#[derive(Clone)]
pub struct Item {
    pub title: CompactString,
    pub price: u32,
    pub count: u32,
}

#[allow(clippy::from_over_into)]
impl Into<ProtoItem> for Item {
    fn into(self) -> ProtoItem {
        ProtoItem {
            title: self.title.into(),
            price: self.price,
            count: self.count,
        }
    }
}
