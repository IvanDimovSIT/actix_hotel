use sea_orm::prelude::StringLen;
use sea_orm::DerivePrimaryKey;
use sea_orm::EntityTrait;
use sea_orm::PrimaryKeyTrait;
use sea_orm::Related;
use sea_orm::RelationDef;
use sea_orm::RelationTrait;
use sea_orm::{ActiveModelBehavior, DeriveActiveEnum, DeriveEntityModel, DeriveRelation, EnumIter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Default,
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
    Hash,
    ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(16))")]
pub enum BedSize {
    #[default]
    #[sea_orm(string_value = "Single")]
    Single,
    #[sea_orm(string_value = "SmallDouble")]
    SmallDouble,
    #[sea_orm(string_value = "Double")]
    Double,
    #[sea_orm(string_value = "KingSize")]
    KingSize,
}
impl BedSize {
    pub fn get_size(&self) -> i16 {
        match self {
            Self::Single => 1,
            Self::SmallDouble | Self::Double | Self::KingSize => 2,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, DeriveEntityModel)]
#[sea_orm(table_name = "beds")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub room_id: Uuid,
    pub bed_size: BedSize,
    pub count: i16,
    pub total_capacity: i16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::room::Entity",
        from = "Column::RoomId",
        to = "super::room::Column::Id"
    )]
    Room,
}
impl ActiveModelBehavior for ActiveModel {}

impl Related<super::room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}
