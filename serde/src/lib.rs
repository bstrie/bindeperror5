pub trait Serialize {
    fn serialize<S: Serializer>(&self, serializer: S);
}

pub trait Serializer {
    fn serialize_field<T: Serialize>(&self, value: &T);
}
