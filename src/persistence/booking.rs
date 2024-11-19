use sea_orm::prelude::Date;
use sea_orm::prelude::DateTime;
use sea_orm::prelude::StringLen;
use sea_orm::ActiveModelBehavior;
use sea_orm::DeriveActiveEnum;
use sea_orm::DeriveEntityModel;
use sea_orm::DerivePrimaryKey;
use sea_orm::DeriveRelation;
use sea_orm::EntityTrait;
use sea_orm::EnumIter;
use sea_orm::PrimaryKeyTrait;
use sea_orm::Related;
use sea_orm::RelationTrait;
use serde::Deserialize;
use serde::Serialize;
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
pub enum BookingStatus  {
    #[default]
    #[sea_orm(string_value = "Unpaid")]
    Unpaid,
    #[sea_orm(string_value = "Paid")]
    Paid,
    #[sea_orm(string_value = "Canceled")]
    Canceled
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
    pub start_date: Date,
    pub end_date: Date,
    pub total_price: i64,
    pub status: BookingStatus
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
