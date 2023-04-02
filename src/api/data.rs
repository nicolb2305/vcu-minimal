use std::str::FromStr;

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Default, Copy, Clone, PartialEq)]
#[repr(i32)]
#[non_exhaustive]
pub enum Champion {
    Unknown = -1,
    #[default]
    None = 0,
    Annie = 1,
    Olaf = 2,
    Galio = 3,
    TwistedFate = 4,
    XinZhao = 5,
    Urgot = 6,
    LeBlanc = 7,
    Vladimir = 8,
    Fiddlesticks = 9,
    Kayle = 10,
    MasterYi = 11,
    Alistar = 12,
    Ryze = 13,
    Sion = 14,
    Sivir = 15,
    Soraka = 16,
    Teemo = 17,
    Tristana = 18,
    Warwick = 19,
    NunuWillump = 20,
    MissFortune = 21,
    Ashe = 22,
    Tryndamere = 23,
    Jax = 24,
    Morgana = 25,
    Zilean = 26,
    Singed = 27,
    Evelynn = 28,
    Twitch = 29,
    Karthus = 30,
    ChoGath = 31,
    Amumu = 32,
    Rammus = 33,
    Anivia = 34,
    Shaco = 35,
    DrMundo = 36,
    Sona = 37,
    Kassadin = 38,
    Irelia = 39,
    Janna = 40,
    Gangplank = 41,
    Corki = 42,
    Karma = 43,
    Taric = 44,
    Veigar = 45,
    Trundle = 48,
    Swain = 50,
    Caitlyn = 51,
    Blitzcrank = 53,
    Malphite = 54,
    Katarina = 55,
    Nocturne = 56,
    Maokai = 57,
    Renekton = 58,
    JarvanIV = 59,
    Elise = 60,
    Orianna = 61,
    Wukong = 62,
    Brand = 63,
    LeeSin = 64,
    Vayne = 67,
    Rumble = 68,
    Cassiopeia = 69,
    Skarner = 72,
    Heimerdinger = 74,
    Nasus = 75,
    Nidalee = 76,
    Udyr = 77,
    Poppy = 78,
    Gragas = 79,
    Pantheon = 80,
    Ezreal = 81,
    Mordekaiser = 82,
    Yorick = 83,
    Akali = 84,
    Kennen = 85,
    Garen = 86,
    Leona = 89,
    Malzahar = 90,
    Talon = 91,
    Riven = 92,
    KogMaw = 96,
    Shen = 98,
    Lux = 99,
    Xerath = 101,
    Shyvana = 102,
    Ahri = 103,
    Graves = 104,
    Fizz = 105,
    Volibear = 106,
    Rengar = 107,
    Varus = 110,
    Nautilus = 111,
    Viktor = 112,
    Sejuani = 113,
    Fiora = 114,
    Ziggs = 115,
    Lulu = 117,
    Draven = 119,
    Hecarim = 120,
    KhaZix = 121,
    Darius = 122,
    Jayce = 126,
    Lissandra = 127,
    Diana = 131,
    Quinn = 133,
    Syndra = 134,
    AurelionSol = 136,
    Kayn = 141,
    Zoe = 142,
    Zyra = 143,
    KaiSa = 145,
    Seraphine = 147,
    Gnar = 150,
    Zac = 154,
    Yasuo = 157,
    VelKoz = 161,
    Taliyah = 163,
    Camille = 164,
    Akshan = 166,
    BelVeth = 200,
    Braum = 201,
    Jhin = 202,
    Kindred = 203,
    Zeri = 221,
    Jinx = 222,
    TahmKench = 223,
    Viego = 234,
    Senna = 235,
    Lucian = 236,
    Zed = 238,
    Kled = 240,
    Ekko = 245,
    Qiyana = 246,
    Vi = 254,
    Aatrox = 266,
    Nami = 267,
    Azir = 268,
    Yuumi = 350,
    Samira = 360,
    Thresh = 412,
    Illaoi = 420,
    RekSai = 421,
    Ivern = 427,
    Kalista = 429,
    Bard = 432,
    Rakan = 497,
    Xayah = 498,
    Ornn = 516,
    Sylas = 517,
    Neeko = 518,
    Aphelios = 523,
    Rell = 526,
    Pyke = 555,
    Vex = 711,
    Yone = 777,
    Sett = 875,
    Lillia = 876,
    Gwen = 887,
    RenataGlasc = 888,
    Nilah = 895,
    KSante = 897,
    Milio = 902,
}

impl FromStr for Champion {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
        Ok(match normalized.to_lowercase().as_str() {
            "aatrox" => Self::Aatrox,
            "ahri" => Self::Ahri,
            "akali" => Self::Akali,
            "akshan" => Self::Akshan,
            "alistar" => Self::Alistar,
            "amumu" => Self::Amumu,
            "anivia" => Self::Anivia,
            "annie" => Self::Annie,
            "aphelios" => Self::Aphelios,
            "ashe" => Self::Ashe,
            "aurelion sol" | "aurelionsol" | "asol" => Self::AurelionSol,
            "azir" => Self::Azir,
            "bard" => Self::Bard,
            "belveth" | "bel'veth" => Self::BelVeth,
            "blitzcrank" | "blitz" => Self::Blitzcrank,
            "brand" => Self::Brand,
            "braum" => Self::Braum,
            "darius" => Self::Darius,
            "diana" => Self::Diana,
            "draven" => Self::Draven,
            "drmundo" | "mundo" => Self::DrMundo,
            "ekko" => Self::Ekko,
            "elise" => Self::Elise,
            "evelynn" | "eve" => Self::Evelynn,
            "ezreal" | "ez" => Self::Ezreal,
            "fiddlesticks" | "fiddle" => Self::Fiddlesticks,
            "fiora" => Self::Fiora,
            "fizz" => Self::Fizz,
            "galio" => Self::Galio,
            "gangplank" | "gp" => Self::Gangplank,
            "garen" => Self::Garen,
            "gnar" => Self::Gnar,
            "gragas" => Self::Gragas,
            "graves" => Self::Graves,
            "gwen" => Self::Gwen,
            "hecarim" | "heca" => Self::Hecarim,
            "heimerdinger" | "heimer" => Self::Heimerdinger,
            "illaoi" => Self::Illaoi,
            "irelia" => Self::Irelia,
            "ivern" => Self::Ivern,
            "janna" => Self::Janna,
            "jarvaniv" | "jarvan" | "j4" => Self::JarvanIV,
            "jax" => Self::Jax,
            "jayce" => Self::Jayce,
            "jhin" => Self::Jhin,
            "jinx" => Self::Jinx,
            "kaisa" => Self::KaiSa,
            "kalista" => Self::Kalista,
            "karma" => Self::Karma,
            "karthus" => Self::Karthus,
            "kassadin" | "kass" => Self::Kassadin,
            "katarina" | "kat" | "kata" => Self::Katarina,
            "kayle" => Self::Kayle,
            "kayn" | "rhaast" => Self::Kayn,
            "kennen" => Self::Kennen,
            "khazix" | "kha" => Self::KhaZix,
            "kindred" => Self::Kindred,
            "kled" => Self::Kled,
            "kogmaw" | "kog" => Self::KogMaw,
            "ksante" => Self::KSante,
            "leblanc" | "lb" => Self::LeBlanc,
            "leesin" | "lee" => Self::LeeSin,
            "leona" => Self::Leona,
            "lillia" => Self::Lillia,
            "lissandra" => Self::Lissandra,
            "lucian" => Self::Lucian,
            "lulu" => Self::Lulu,
            "lux" => Self::Lux,
            "malphite" | "malph" => Self::Malphite,
            "malzahar" | "malz" => Self::Malzahar,
            "maokai" => Self::Maokai,
            "masteryi" | "master" | "yi" => Self::MasterYi,
            "milio" => Self::Milio,
            "missfortune" | "mf" => Self::MissFortune,
            "mordekaiser" | "mord" | "morde" => Self::Mordekaiser,
            "morgana" | "morg" => Self::Morgana,
            "nami" => Self::Nami,
            "nasus" | "susan" => Self::Nasus,
            "nautilus" | "naut" | "nauti" => Self::Nautilus,
            "neeko" => Self::Neeko,
            "nidalee" | "nid" | "nida" => Self::Nidalee,
            "nilah" => Self::Nilah,
            "nocturne" | "noc" => Self::Nocturne,
            "nunuwillump" | "nunu" => Self::NunuWillump,
            "olaf" => Self::Olaf,
            "orianna" | "ori" => Self::Orianna,
            "ornn" => Self::Ornn,
            "pantheon" | "panth" => Self::Pantheon,
            "poppy" => Self::Poppy,
            "pyke" => Self::Pyke,
            "qiyana" => Self::Qiyana,
            "quinn" => Self::Quinn,
            "rakan" => Self::Rakan,
            "rammus" => Self::Rammus,
            "reksai" => Self::RekSai,
            "rell" => Self::Rell,
            "renataglasc" | "renata" => Self::RenataGlasc,
            "renekton" | "renek" => Self::Renekton,
            "rengar" => Self::Rengar,
            "riven" => Self::Riven,
            "rumble" => Self::Rumble,
            "ryze" => Self::Ryze,
            "samira" => Self::Samira,
            "sejuani" | "sej" => Self::Sejuani,
            "senna" => Self::Senna,
            "seraphine" | "sera" => Self::Seraphine,
            "sett" => Self::Sett,
            "shaco" => Self::Shaco,
            "shen" => Self::Shen,
            "shyvana" | "shyv" => Self::Shyvana,
            "singed" => Self::Singed,
            "sion" => Self::Sion,
            "sivir" => Self::Sivir,
            "skarner" | "skarn" => Self::Skarner,
            "sona" => Self::Sona,
            "soraka" | "raka" => Self::Soraka,
            "swain" => Self::Swain,
            "sylas" => Self::Sylas,
            "syndra" => Self::Syndra,
            "tahmkench" | "tahm" | "tk" => Self::TahmKench,
            "taliyah" | "tali" => Self::Taliyah,
            "talon" => Self::Talon,
            "taric" => Self::Taric,
            "teemo" => Self::Teemo,
            "thresh" => Self::Thresh,
            "tristana" | "trist" => Self::Tristana,
            "trundle" => Self::Trundle,
            "tryndamere" | "trynd" => Self::Tryndamere,
            "twistedfate" | "tf" => Self::TwistedFate,
            "twitch" => Self::Twitch,
            "udyr" => Self::Udyr,
            "urgot" => Self::Urgot,
            "varus" => Self::Varus,
            "vayne" => Self::Vayne,
            "veigar" | "veig" => Self::Veigar,
            "velkoz" | "vel" => Self::VelKoz,
            "vex" => Self::Vex,
            "vi" => Self::Vi,
            "viego" => Self::Viego,
            "viktor" | "vik" => Self::Viktor,
            "vladimir" | "vlad" => Self::Vladimir,
            "volibear" | "voli" => Self::Volibear,
            "warwick" | "ww" => Self::Warwick,
            "wukong" | "wu" => Self::Wukong,
            "xayah" => Self::Xayah,
            "xerath" | "xer" => Self::Xerath,
            "xinzhao" | "xin" => Self::XinZhao,
            "yasuo" | "yas" => Self::Yasuo,
            "yone" => Self::Yone,
            "yorick" => Self::Yorick,
            "yuumi" => Self::Yuumi,
            "zac" => Self::Zac,
            "zed" => Self::Zed,
            "zeri" => Self::Zeri,
            "ziggs" => Self::Ziggs,
            "zilean" | "zil" => Self::Zilean,
            "zoe" => Self::Zoe,
            "zyra" => Self::Zyra,
            "" => Self::None,
            _ => return Err("Could not find champion"),
        })
    }
}