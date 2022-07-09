pub struct IndexMap;

#[cfg(feature = "serde")]
impl serde::Serialize for IndexMap {
    fn serialize<T: serde::Serializer>(&self, _: T) {}
}
