use serde::{Deserialize, Serialize};
use sqlx::Type;
use strum_macros::{Display, EnumString};

pub enum Zone {
    ChennaiCentral,
    ChennaiNorth,
    ChennaiSouth,
    CoimbatoreCity,
    MaduraiCity,
    TrichyCity,
    SalemCity,
    TirunelveliCity,
    Ooty,
    Coonoor,
    Kodaikanal,
    Yercaud,
    Valparai,
    Yelagiri,
    Sirumalai,
    Mahabalipuram,
    PondicherryBorder,
    Rameswaram,
    Kanyakumari,
    ThoothukudiCoast,
    NagapattinamCoast,
    MaduraiTempleArea,
    RameswaramTempleArea,
    Tiruvannamalai,
    Chidambaram,
    Srivilliputhur,
    ThanjavurTempleBelt,
    Palani,
    Chettinad,
    ThanjavurHeritageZone,
    Kumbakonam,
    Srirangam,
    Hogenakkal,
    Courtallam,
    Pollachi,
    Mayiladuthurai,
    Dhanushkodi,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display, Type)]
#[sqlx(type_name = "text")]
#[strum(serialize_all = "snake_case")]
pub enum AuthProvider {
    Google,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display, Type)]
#[sqlx(type_name = "text")]
#[strum(serialize_all = "snake_case")]
pub enum UserAccountStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display, Type, Deserialize, Serialize)]
#[sqlx(type_name = "text")]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum HotelStatus {
    Active,
    Inactive,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display, Type, Deserialize, Serialize)]
#[sqlx(type_name = "text")]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AmenityCategory {
    Basic,
    Services,
    FoodDrink,
    SafetySecurity,
    Wellness,
    Business,
    RoomFeatures,
    Bathroom,
    Views,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display, Type, Deserialize, Serialize)]
#[sqlx(type_name = "text")]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum HotelAmenity {
    Wifi,
    AirConditioning,
    PowerBackup,
    Elevator,
    RoomService,
    Laundry,
    FrontDesk24hr,
    Housekeeping,
    Restaurant,
    BreakfastIncluded,
    Bar,
    Cctv,
    Security24hr,
    SwimmingPool,
    Gym,
    Spa,
    ConferenceRoom,
    BanquetHall,
    Television,
    MiniFridge,
    WorkDesk,
    Wardrobe,
    Bathtub,
    HotWater,
    Balcony,
    CityView,
    MountainView,
}
