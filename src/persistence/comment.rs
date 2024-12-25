use sea_orm::prelude::DateTime;
use sea_orm::prelude::StringLen;
use sea_orm::DerivePrimaryKey;
use sea_orm::EntityTrait;
use sea_orm::PrimaryKeyTrait;
use sea_orm::Related;
use sea_orm::RelationDef;
use sea_orm::RelationTrait;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Default, DeriveEntityModel)]
#[sea_orm(table_name = "comments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub room_id: Uuid,
    pub user_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(256))")]
    pub content: String,
    pub posted_time: DateTime,
    pub updated_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::room::Entity",
        from = "Column::RoomId",
        to = "super::room::Column::Id"
    )]
    Room,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}
impl ActiveModelBehavior for ActiveModel {}

impl Related<super::room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
