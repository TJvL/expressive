use expressive_data::value::Value;
use std::collections::HashMap;

pub struct ModelDefinition {
    pub name: String,
    pub entities: HashMap<String, EntityDefinition>,
}

pub struct ModelInstance {
    pub definition: ModelDefinition,
    pub entities: HashMap<String, HashMap<String, EntityInstance>>,
}

pub enum RelationType {
    OneToOne {
        entity_left_name: String,
        entity_right_name: String,
    },
    OneToMany {
        entity_left_name: String,
        entity_right_name: String,
    },
    ZeroToMany {
        entity_left_name: String,
        entity_right_name: String,
    },
}

pub struct RelationDefinition {
    pub name: String,
    pub relation_type: RelationType,
}

pub struct RelationInstance {
    pub definition: RelationDefinition,
}

pub struct EntityDefinition {
    pub name: String,
    pub attributes: HashMap<String, AttributeDefinition>,
    pub relations: HashMap<String, RelationDefinition>,
}

pub struct EntityInstance {
    pub definition: EntityDefinition,
    pub attributes: HashMap<String, AttributeInstance>,
}

pub enum ValueType {
    PreciseDecimal,
    PreciseInteger,
    FastDecimal,
    FastInteger,
}

pub struct AttributeDefinition {
    pub name: String,
    pub value_type: ValueType,
}

pub struct AttributeInstance {
    pub definition: AttributeDefinition,
    pub value: Value,
}
