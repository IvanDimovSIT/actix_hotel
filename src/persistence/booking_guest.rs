use sea_orm::ActiveModelBehavior;
use sea_orm::DeriveEntityModel;
use sea_orm::DerivePrimaryKey;
use sea_orm::DeriveRelation;
use sea_orm::EntityTrait;
use sea_orm::EnumIter;
use sea_orm::PrimaryKeyTrait;
use sea_orm::Related;
use sea_orm::RelationDef;
use sea_orm::RelationTrait;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Default, DeriveEntityModel)]
#[sea_orm(table_name = "bookings_guests")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub guest_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub booking_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::booking::Entity",
        from = "Column::BookingId",
        to = "super::booking::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Booking,
    #[sea_orm(
        belongs_to = "super::guest::Entity",
        from = "Column::GuestId",
        to = "super::guest::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Guest,
}

impl Related<super::booking::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Booking.def()
    }
}

impl Related<super::guest::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Guest.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
