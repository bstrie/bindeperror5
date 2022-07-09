use indexmap::IndexMap;
use serde::{Serialize, Serializer};

struct Module {
    exports: IndexMap,
}

impl Serialize for Module {
    fn serialize<T: Serializer>(&self, serializer: T) {
        serializer.serialize_field(&self.exports);
    }
}
