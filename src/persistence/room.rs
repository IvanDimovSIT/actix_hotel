use std::error::Error;

use sea_orm::prelude::StringLen;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DerivePrimaryKey;
use sea_orm::EntityTrait;
use sea_orm::PrimaryKeyTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelBehavior, DeriveActiveEnum, DeriveEntityModel, DeriveRelation, EnumIter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Clone, Debug, PartialEq, Eq, Default, EnumIter, DeriveActiveEnum, Serialize, Deserialize,
)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(16))")]
pub enum BathroomType {
    #[default]
    #[sea_orm(string_value = "Private")]
    Private,
    #[sea_orm(string_value = "Shared")]
    Shared,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, DeriveEntityModel)]
#[sea_orm(table_name = "rooms")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub price: i64,
    pub floor: i16,
    #[sea_orm(column_type = "String(StringLen::N(16))", unique)]
    pub room_number: String,
    pub bathroom_type: BathroomType,
    pub is_deleted: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::bed::Entity")]
    Bed,
}
impl ActiveModelBehavior for ActiveModel {}

pub async fn find_by_room_number(
    db: &DatabaseConnection,
    room_number: &str,
) -> Result<Option<Model>, Box<dyn Error>> {
    let room = crate::persistence::room::Entity::find()
        .filter(crate::persistence::room::Column::RoomNumber.eq(room_number))
        .one(db)
        .await?;

    Ok(room)
}
