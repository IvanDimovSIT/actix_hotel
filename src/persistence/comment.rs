use sea_orm::prelude::DateTime;
use sea_orm::prelude::StringLen;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::DbErr;
use sea_orm::DerivePrimaryKey;
use sea_orm::EntityTrait;
use sea_orm::PaginatorTrait;
use sea_orm::PrimaryKeyTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
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

pub async fn get_paged_comments<T>(
    db: &T,
    room_id: Uuid,
    page: u64,
    size: u64,
) -> Result<(u64, Vec<Model>), DbErr>
where
    T: ConnectionTrait,
{
    let comments = Entity::find()
        .filter(Column::RoomId.eq(room_id))
        .order_by(Column::PostedTime, sea_orm::Order::Desc)
        .paginate(db, size)
        .fetch_page(page)
        .await?;

    let count = Entity::find()
        .filter(Column::RoomId.eq(room_id))
        .count(db)
        .await?;

    Ok((count, comments))
}
