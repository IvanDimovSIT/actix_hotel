use sea_orm::prelude::Date;
use sea_orm::prelude::DateTime;
use sea_orm::prelude::StringLen;
use sea_orm::sea_query::any;
use sea_orm::ActiveModelBehavior;
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::ConnectionTrait;
use sea_orm::DbErr;
use sea_orm::DeriveActiveEnum;
use sea_orm::DeriveEntityModel;
use sea_orm::DerivePrimaryKey;
use sea_orm::DeriveRelation;
use sea_orm::EntityTrait;
use sea_orm::EnumIter;
use sea_orm::FromQueryResult;
use sea_orm::PrimaryKeyTrait;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use sea_orm::Related;
use sea_orm::RelationTrait;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::persistence::booking;

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
pub enum BookingStatus {
    #[default]
    #[sea_orm(string_value = "Unpaid")]
    Unpaid,
    #[sea_orm(string_value = "Paid")]
    Paid,
    #[sea_orm(string_value = "Canceled")]
    Canceled,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, DeriveEntityModel)]
#[sea_orm(table_name = "bookings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub main_guest_id: Uuid,
    pub room_id: Uuid,
    pub admin_id: Uuid,
    pub user_id: Option<Uuid>,
    pub booking_time: DateTime,
    pub payment_time: Option<DateTime>,
    pub start_date: Date,
    pub end_date: Date,
    pub total_price: i64,
    pub status: BookingStatus,
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
        belongs_to = "super::guest::Entity",
        from = "Column::MainGuestId",
        to = "super::guest::Column::Id"
    )]
    MainGuest,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::AdminId",
        to = "super::user::Column::Id"
    )]
    Admin,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    GuestUser,
    #[sea_orm(has_many = "super::guest::Entity")]
    Guest,
    #[sea_orm(has_many = "super::booking_guest::Entity")]
    BookingGuest,
}
impl Related<super::room::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Room.def()
    }
}
impl Related<super::booking_guest::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::BookingGuest.def()
    }
}
impl Related<super::guest::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        super::booking_guest::Relation::Guest.def()
    }

    fn via() -> Option<sea_orm::RelationDef> {
        Some(super::booking_guest::Relation::Booking.def().rev())
    }
}
impl Related<super::user::Entity> for Entity {
    fn to() -> sea_orm::RelationDef {
        Relation::Admin.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

fn check_room_is_occupied_in_range(start_date: Date, end_date: Date) -> Condition {
    any![
        Condition::all()
            .add(booking::Column::StartDate.gte(start_date))
            .add(booking::Column::StartDate.lte(end_date)),
        Condition::all()
            .add(booking::Column::EndDate.gte(start_date))
            .add(booking::Column::EndDate.lte(end_date)),
        Condition::all()
            .add(booking::Column::StartDate.lte(start_date))
            .add(booking::Column::EndDate.gte(end_date)),
    ]
}

pub async fn is_room_occupied_for_period<T>(
    db: &T,
    room_id: Uuid,
    start_date: Date,
    end_date: Date,
) -> Result<bool, DbErr>
where
    T: ConnectionTrait,
{
    Ok(booking::Entity::find()
        .filter(booking::Column::RoomId.eq(room_id))
        .filter(booking::Column::Status.eq(BookingStatus::Canceled).not())
        .filter(check_room_is_occupied_in_range(start_date, end_date))
        .one(db)
        .await?
        .is_some())
}

pub async fn get_bookings_for_user<T>(
    db: &T,
    user_id: Uuid,
    include_canceled: bool,
    include_paid: bool,
) -> Result<Vec<Uuid>, DbErr>
where
    T: ConnectionTrait,
{
    #[derive(Debug, FromQueryResult)]
    struct BookingId {
        b_id: Uuid,
    }

    let mut base_query = Entity::find()
        .select_only()
        .column_as(Column::Id, "b_id")
        .filter(Column::UserId.eq(user_id));

    if !include_canceled {
        base_query = base_query.filter(Column::Status.eq(BookingStatus::Canceled).not())
    }
    if !include_paid {
        base_query = base_query.filter(Column::Status.eq(BookingStatus::Paid).not())
    }

    Ok(base_query
        .into_model::<BookingId>()
        .all(db)
        .await?
        .iter()
        .map(|b| b.b_id)
        .collect())
}

pub async fn user_has_booking_for_room<T>(
    db: &T,
    user_id: Uuid,
    room_id: Uuid,
) -> Result<bool, DbErr>
where
    T: ConnectionTrait,
{
    Ok(Entity::find()
        .filter(Column::UserId.eq(user_id))
        .filter(Column::RoomId.eq(room_id))
        .one(db)
        .await?
        .is_some())
}
