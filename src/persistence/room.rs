use std::error::Error;

use sea_orm::prelude::StringLen;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::DbErr;
use sea_orm::DerivePrimaryKey;
use sea_orm::EntityTrait;
use sea_orm::ModelTrait;
use sea_orm::PrimaryKeyTrait;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use sea_orm::Related;
use sea_orm::RelationDef;
use sea_orm::RelationTrait;
use sea_orm::{ActiveModelBehavior, DeriveActiveEnum, DeriveEntityModel, DeriveRelation, EnumIter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::persistence::bed;

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
    ToSchema,
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

impl Related<super::bed::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bed.def()
    }
}

pub async fn find_by_room_number<T>(db: &T, room_number: &str) -> Result<Option<Model>, DbErr>
where
    T: ConnectionTrait,
{
    let room = crate::persistence::room::Entity::find()
        .filter(crate::persistence::room::Column::RoomNumber.eq(room_number))
        .one(db)
        .await?;

    Ok(room)
}

pub async fn find_room_by_id<T>(
    db: &T,
    id: Uuid,
) -> Result<Option<(Model, Vec<crate::persistence::bed::Model>)>, DbErr>
where
    T: ConnectionTrait,
{
    let room_option = Entity::find_by_id(id).one(db).await?;

    if room_option.is_none() {
        return Ok(None);
    }

    let room = room_option.unwrap();

    let beds = room.find_related(bed::Entity).all(db).await?;

    Ok(Some((room, beds)))
}
