use dash_rs::{
    model::{
        creator::Creator,
        level::{DemonRating, Featured::Featured, Level, LevelData, LevelLength, LevelRating, Password},
        song::{MainSong, NewgroundsSong},
        user::{
            searched::SearchedUser,
            Color, IconType,
        },
        GameVersion,
    },
    Base64Decoded, HasRobtopFormat, Thunk,
};
use std::borrow::Cow;

const DARK_REALM_DATA: &str =
    "1:11774780:2:Dark \
     Realm:5:2:6:2073761:8:10:9:30:10:90786:12:0:13:20:14:10974:17:1:43:0:25::18:10:19:11994:42:0:45:0:3:\
     TXkgYmVzdCBsZXZlbCB5ZXQuIFZpZGVvIG9uIG15IFlvdVR1YmUuIEhhdmUgZnVuIGluIHRoaXMgZmFzdC1wYWNlZCBERU1PTiA-OikgdjIgRml4ZWQgc29tZSB0aGluZ3M=:\
     15:3:30:0:31:0:37:3:38:1:39:10:46:1:47:2:35:444085";

const CREO_DUNE_DATA_TOO_MANY_FIELDS: &str = "1~|~771277~|~54~|~should be ignored~|~2~|~Creo - \
                                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.\
                                              03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%2F%2Faudio.ngfiles.com%\
                                              2F771000%2F771277_Creo---Dune.mp3%3Ff1508708604~|~9~|~should be ignored";

const SEARCHED_MICHIGUN_DATA: &str = "1:Michigun:2:703929:13:149:17:12312:6::9:22:10:15:11:12:14:0:15:2:16:34499:3:61161:8:16:4:997";

const CREATOR_REGISTERED_DATA: &str = "4170784:Serponge:119741";
const _CREATOR_REGISTERED_DATA_TOO_MANY_FIELDS: &str = "4170784:Serponge:119741:34:fda:32:asd:3";

const CREATOR_REGISTERED: Creator = Creator {
    user_id: 4170784,
    name: Cow::Borrowed("Serponge"),
    account_id: Some(119741),
};

const CREATOR_UNREGISTERED_DATA: &str = "4170784:Serponge:0";

const CREATOR_UNREGISTERED: Creator = Creator {
    user_id: 4170784,
    name: Cow::Borrowed("Serponge"),
    account_id: None,
};

const SEARCHED_MICHIGUN: SearchedUser = SearchedUser {
    name: Cow::Borrowed("Michigun"),
    user_id: 703929,
    stars: 61161,
    demons: 997,
    index_6: None,
    creator_points: 16,
    icon_index: 22,
    primary_color: Color::Known(0, 0, 0),
    secondary_color: Color::Known(255, 255, 255),
    secret_coins: 149,
    icon_type: IconType::Cube,
    has_glow: true,
    account_id: 34499,
    user_coins: 12312,
};

const TIME_PRESSURE: Level<Option<u64>, u64> = Level {
    level_id: 897837,
    name: Cow::Borrowed("time pressure"),
    description: Some(Thunk::Processed(Base64Decoded(Cow::Borrowed(
        "please rate and like  8-9 stars mabye?",
    )))),
    version: 2,
    creator: 842519,
    difficulty: LevelRating::Demon(DemonRating::Easy),
    downloads: 3189574,
    main_song: Some(MainSong {
        main_song_id: 14,
        name: "Electrodynamix",
        artist: "DJ-Nate",
    }),
    gd_version: GameVersion::Unknown,
    likes: 198542,
    length: LevelLength::Long,
    stars: 10,
    featured: Featured(700),
    copy_of: None,
    index_31: Some(Cow::Borrowed("0")),
    custom_song: None,
    coin_amount: 0,
    coins_verified: false,
    stars_requested: None,
    index_40: None,
    is_epic: false,
    index_43: Cow::Borrowed("3"),
    object_amount: None,
    index_46: None,
    index_47: None,
    level_data: Some(LevelData {
        level_data: Thunk::Unprocessed("REMOVED"),
        password: Password::PasswordCopy(3101),
        time_since_upload: Cow::Borrowed("5 years"),
        time_since_update: Cow::Borrowed("5 years"),
        index_36: None,
    }),
};

#[test]
fn serialize_registered_creator() {
    init_log();

    let mut buf: Vec<u8> = Vec::new();
    let result = CREATOR_REGISTERED.write_robtop_data(&mut buf);

    assert!(result.is_ok());
    assert_eq!(buf, CREATOR_REGISTERED_DATA.as_bytes());
}

#[test]
fn serialize_unregistered_creator() {
    init_log();

    let mut buf: Vec<u8> = Vec::new();
    let result = CREATOR_UNREGISTERED.write_robtop_data(&mut buf);

    assert!(result.is_ok());
    assert_eq!(buf, CREATOR_UNREGISTERED_DATA.as_bytes());
}

#[test]
fn deserialize_registered_creator() {
    init_log();

    let creator = Creator::from_robtop_str(CREATOR_REGISTERED_DATA);

    assert!(creator.is_ok(), "{:?}", creator.unwrap_err());
    assert_eq!(creator.unwrap(), CREATOR_REGISTERED);
}

#[test]
fn deserialize_unregistered_creator() {
    init_log();

    let creator = Creator::from_robtop_str(CREATOR_UNREGISTERED_DATA);

    assert!(creator.is_ok(), "{:?}", creator.unwrap_err());
    assert_eq!(creator.unwrap(), CREATOR_UNREGISTERED);
}

#[test]
fn deserialize_too_many_fields() {
    init_log();

    let song = NewgroundsSong::from_robtop_str(CREO_DUNE_DATA_TOO_MANY_FIELDS);

    assert!(song.is_ok(), "{:?}", song.unwrap_err());
}

#[test]
fn deserialize_partial_level() {
    init_log();

    let level = Level::from_robtop_str(DARK_REALM_DATA);

    assert!(level.is_ok(), "{:?}", level.unwrap_err());

    let mut level = level.unwrap();

    assert!(level.description.as_mut().unwrap().process().is_ok());
}

#[test]
fn deserialize_level() {
    init_log();

    let level = Level::from_robtop_str(include_str!("data/11774780_dark_realm_gjdownload_response"));

    let mut level = level.unwrap();

    assert!(level.description.as_mut().unwrap().process().is_ok());
    assert!(level.level_data.is_some());
}

#[test]
fn deserialize_level2() {
    init_log();

    let level = Level::from_robtop_str(include_str!("data/897837_time_pressure_gjdownload_response"));

    let mut level = level.unwrap();

    assert!(level.description.as_mut().unwrap().process().is_ok());
    assert!(level.level_data.is_some());

    level.level_data.as_mut().unwrap().level_data.process().unwrap();

    level.level_data.as_mut().unwrap().level_data = Thunk::Unprocessed("REMOVED");

    assert_eq!(level, TIME_PRESSURE);
}

#[test]
fn deserialize_searched_user() {
    init_log();

    let user = SearchedUser::from_robtop_str(SEARCHED_MICHIGUN_DATA);

    assert!(user.is_ok(), "{:?}", user.unwrap_err());
    assert_eq!(user.unwrap(), SEARCHED_MICHIGUN);
}

#[test]
fn searched_user_roundtrip() {
    init_log();

    let data = SEARCHED_MICHIGUN.to_robtop_string();

    assert!(data.is_ok(), "{:?}", data.unwrap_err());

    let data = data.unwrap();

    let user = SearchedUser::from_robtop_str(&data);

    assert!(user.is_ok(), "{:?}", user.unwrap_err());
    assert_eq!(user.unwrap(), SEARCHED_MICHIGUN);
}

fn init_log() {
    if let Err(err) = env_logger::builder().is_test(true).try_init() {
        // nothing to make the tests fail over
        eprintln!("Error setting up env_logger: {:?}", err)
    }
}
