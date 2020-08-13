use dash_rs::{
    model::{
        creator::Creator,
        level::{DemonRating, Featured::Featured, Level, LevelData, LevelLength, LevelRating, Password},
        song::{MainSong, NewgroundsSong},
        user::{
            profile::{Profile, Twitter, Youtube},
            Color, ModLevel,
        },
        GameVersion,
    },
    Base64Decoded, HasRobtopFormat, PercentDecoded, Thunk,
};
use std::borrow::Cow;

const DARK_REALM_DATA: &str =
    "1:11774780:2:Dark \
     Realm:5:2:6:2073761:8:10:9:30:10:90786:12:0:13:20:14:10974:17:1:43:0:25::18:10:19:11994:42:0:45:0:3:\
     TXkgYmVzdCBsZXZlbCB5ZXQuIFZpZGVvIG9uIG15IFlvdVR1YmUuIEhhdmUgZnVuIGluIHRoaXMgZmFzdC1wYWNlZCBERU1PTiA-OikgdjIgRml4ZWQgc29tZSB0aGluZ3M=:\
     15:3:30:0:31:0:37:3:38:1:39:10:46:1:47:2:35:444085";

const CREO_DUNE_DATA: &str = "1~|~771277~|~2~|~Creo - \
                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.03~|~6~|~~|~10~|~https%3A%2F%2Faudio.ngfiles.com%2F771000%\
                              2F771277_Creo---Dune.mp3%3Ff1508708604~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1";

/// Testing data for newgrounds song (de)serialization
///
/// This is the data provided by the Geometry Dash servers for the song "Dune" by Creo, except that
/// its fields have been reordered
const CREO_DUNE_DATA_ORDERED: &str = "1~|~771277~|~2~|~Creo - \
                                      Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.\
                                      03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%2F%2Faudio.ngfiles.com%2F771000%\
                                      2F771277_Creo---Dune.mp3%3Ff1508708604";

const CREO_DUNE_DATA_TOO_MANY_FIELDS: &str = "1~|~771277~|~54~|~should be ignored~|~2~|~Creo - \
                                              Dune~|~3~|~50531~|~4~|~CreoMusic~|~5~|~8.\
                                              03~|~6~|~~|~7~|~UCsCWA3Y3JppL6feQiMRgm6Q~|~8~|~1~|~10~|~https%3A%2F%2Faudio.ngfiles.com%\
                                              2F771000%2F771277_Creo---Dune.mp3%3Ff1508708604~|~9~|~should be ignored";

const PROFILE_STARDUST1971_DATA: &str = "1:stardust1971:2:2073761:13:149:17:498:10:9:11:10:3:13723:46:2312:4:484:8:19:18:0:19:0:50:0:20:\
                                         stardust19710:21:95:22:48:23:33:24:18:25:11:26:10:28:1:43:2:48:13:30:0:16:8451:31:0:44:\
                                         stadust1971:45::49:0:38:0:39:579:40:0:29:1";

const CREO_DUNE: NewgroundsSong<'static> = NewgroundsSong {
    song_id: 771277,
    name: Cow::Borrowed("Creo - Dune"),
    index_3: 50531,
    artist: Cow::Borrowed("CreoMusic"),
    filesize: 8.03,
    index_6: None,
    index_7: Some(Cow::Borrowed("UCsCWA3Y3JppL6feQiMRgm6Q")),
    index_8: Cow::Borrowed("1"),
    link: Thunk::Processed(PercentDecoded(Cow::Borrowed(
        "https://audio.ngfiles.com/771000/771277_Creo---Dune.mp3?f1508708604",
    ))),
};

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

const PROFILE_STARDUST1971: Profile = Profile {
    name: Cow::Borrowed("stardust1971"),
    user_id: 2073761,
    stars: 13723,
    demons: 484,
    creator_points: 19,
    primary_color: Color::Known(255, 0, 0),
    secondary_color: Color::Known(255, 125, 0),
    secret_coins: 149,
    account_id: 8451,
    user_coins: 498,
    index_18: Cow::Borrowed("0"),
    index_19: Cow::Borrowed("0"),
    youtube_url: Some(Youtube(Cow::Borrowed("stardust19710"))),
    cube_index: 95,
    ship_index: 48,
    ball_index: 33,
    ufo_index: 18,
    wave_index: 11,
    robot_index: 10,
    has_glow: true,
    index_29: Cow::Borrowed("1"),
    global_rank: Some(0),
    index_31: Cow::Borrowed("0"),
    spider_index: 2,
    twitter_url: Some(Twitter(Cow::Borrowed("stadust1971"))),
    twitch_url: None,
    diamonds: 2312,
    death_effect_index: 13,
    mod_level: ModLevel::None,
    index_50: Cow::Borrowed("0"),
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
fn serialize_song() {
    init_log();

    let mut buf: Vec<u8> = Vec::new();
    let result = CREO_DUNE.write_robtop_data(&mut buf);

    assert!(result.is_ok());
    assert_eq!(buf, CREO_DUNE_DATA_ORDERED.as_bytes());
}

#[test]
fn deserialize_song() {
    init_log();

    let song = NewgroundsSong::from_robtop_str(CREO_DUNE_DATA);

    assert!(song.is_ok(), "{:?}", song.unwrap_err());

    let mut song = song.unwrap();

    assert!(song.link.process().is_ok());
    assert_eq!(song, CREO_DUNE);
}

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
fn deserialize_profile() {
    init_log();

    let profile = Profile::from_robtop_str(PROFILE_STARDUST1971_DATA);

    assert!(profile.is_ok(), "{:?}", profile.unwrap_err());
    assert_eq!(profile.unwrap(), PROFILE_STARDUST1971);
}

#[test]
fn profile_roundtrip() {
    init_log();

    let data = PROFILE_STARDUST1971.to_robtop_string();

    assert!(data.is_ok(), "{:?}", data.unwrap_err());

    let data = data.unwrap();

    let profile = Profile::from_robtop_str(&data);

    assert!(profile.is_ok(), "{:?}", profile.unwrap_err());
    assert_eq!(profile.unwrap(), PROFILE_STARDUST1971);
}

fn init_log() {
    if let Err(err) = env_logger::builder().is_test(true).try_init() {
        // nothing to make the tests fail over
        eprintln!("Error setting up env_logger: {:?}", err)
    }
}
