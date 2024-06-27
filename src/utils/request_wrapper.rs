use std::collections::HashMap;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use tonic::metadata::{KeyAndValueRef, MetadataMap};

pub struct RequestMetadata<'a, T> {
    pub(crate) metadata: &'a MetadataMap,
    pub(crate) request: T,
}

fn metadata_map_to_hashmap(metadata: &MetadataMap) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for entry in metadata.iter() {
        match entry {
            KeyAndValueRef::Ascii(key, value) => {
                let key_str = key.as_str().to_string();
                if key_str == "timezone" {
                    let value_str = value.to_str().unwrap_or("").to_string();
                    map.insert(key_str, value_str);
                }
            }
            _ => {}
        }
    }
    map
}

impl<T> Serialize for RequestMetadata<'_, T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let metadata_map = metadata_map_to_hashmap(self.metadata);
        let mut state = serializer.serialize_struct("RequestMetadata", 2)?;
        state.serialize_field("metadata", &metadata_map)?;
        state.serialize_field("request", &self.request)?;
        state.end()
    }
}

impl<T> Clone for RequestMetadata<'_, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        RequestMetadata {
            metadata: self.metadata,
            request: self.request.clone(),
        }
    }
}
