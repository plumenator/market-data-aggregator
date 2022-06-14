#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Exchange {
    Binance,
    Bitstamp,
}

impl std::fmt::Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Currency {
    Aave,
    Ada,
    Algo,
    Alpha,
    Amp,
    Ant,
    Audio,
    Avax,
    Axs,
    Band,
    Bat,
    Bch,
    Btc,
    Cel,
    Chz,
    Comp,
    Crv,
    Ctsi,
    Cvx,
    Dai,
    Dydx,
    Enj,
    Ens,
    Eth,
    Eth2,
    Eur,
    Eurt,
    Fet,
    Ftm,
    Ftt,
    Gala,
    Gbp,
    Gods,
    Grt,
    Gusd,
    Hbar,
    Imx,
    Inj,
    Knc,
    Link,
    Lrc,
    Ltc,
    Mana,
    Matic,
    Mkr,
    Nexo,
    Omg,
    OneInch,
    Pax,
    Perp,
    Rad,
    Rgt,
    Rly,
    Rndr,
    Sand,
    Sgb,
    Shib,
    Skl,
    Slp,
    Snx,
    Storj,
    Sushi,
    Sxp,
    Uma,
    Uni,
    Usd,
    Usdc,
    Usdt,
    Ust,
    Vega,
    Wbtc,
    Xlm,
    Xrp,
    Yfi,
    Zrx,
    Aca,
    Ach,
    Acm,
    Adx,
    Ae,
    Aergo,
    Agi,
    Agix,
    Agld,
    Aion,
    Akro,
    Alcx,
    Alice,
    Alpaca,
    Alpine,
    Amb,
    Anc,
    Ankr,
    Any,
    Ape,
    Api3,
    Appc,
    Ar,
    Ardr,
    Ark,
    Arn,
    Arpa,
    Asr,
    Ast,
    Astr,
    Ata,
    Atm,
    Atom,
    Auction,
    Aud,
    Auto,
    Ava,
    B,
    Badger,
    Bake,
    Bal,
    Bar,
    Bcc,
    Bcd,
    Bcha,
    Bchabc,
    Bchsv,
    Bcn,
    Bcpt,
    Bdot,
    Beam,
    Bear,
    Bel,
    Beta,
    Beth,
    Bico,
    Bidr,
    Bifi,
    Bkrw,
    Blz,
    Bnb,
    Bnt,
    Bnx,
    Bond,
    Bot,
    Bqx,
    Brd,
    Brl,
    Bsw,
    Btcb,
    Btcst,
    Btg,
    Bts,
    Btt,
    Bttc,
    Bull,
    Burger,
    Busd,
    Bvnd,
    Bzrx,
    C98,
    Cake,
    Cdt,
    Celo,
    Celr,
    Cfx,
    Chat,
    Chess,
    Chr,
    City,
    Ckb,
    Cloak,
    Clv,
    Cmt,
    Cnd,
    Cocos,
    Cos,
    Coti,
    Cover,
    Cream,
    Ctk,
    Ctxc,
    Cvc,
    Cvp,
    Dar,
    Dash,
    Data,
    Dcr,
    Dego,
    Dent,
    Dexe,
    Df,
    Dgb,
    Dgd,
    Dia,
    Dlt,
    Dnt,
    Dock,
    Dodo,
    Doge,
    Dot,
    Down,
    Drep,
    Dusk,
    Easy,
    Edo,
    Egld,
    Elf,
    Eng,
    Eos,
    Eps,
    Epx,
    Erd,
    Ern,
    Etc,
    Evx,
    Ez,
    Farm,
    Fida,
    Fil,
    Fio,
    Firo,
    Fis,
    Flm,
    Flow,
    Flux,
    For,
    Forth,
    Front,
    Fuel,
    Fun,
    Fxs,
    Gal,
    Gas,
    Ghst,
    Glm,
    Glmr,
    Gmt,
    Gno,
    Gnt,
    Go,
    Grs,
    Gtc,
    Gto,
    Gvt,
    Gxs,
    Gyen,
    Hard,
    Hc,
    Hegic,
    High,
    Hive,
    Hnt,
    Hot,
    Hsr,
    Icn,
    Icp,
    Icx,
    Idex,
    Idrt,
    Ilv,
    Ins,
    Iost,
    Iota,
    Iotx,
    Iq,
    Iris,
    Jasmy,
    Joe,
    Jst,
    Juv,
    Kava,
    Kda,
    Keep,
    Key,
    Klay,
    Kmd,
    Kp3r,
    Ksm,
    Lazio,
    Ldo,
    Lend,
    Lina,
    Lit,
    Loka,
    Loom,
    Lpt,
    Lsk,
    Lto,
    Lun,
    Luna,
    Lunc,
    Mask,
    Mbl,
    Mbox,
    Mc,
    Mco,
    Mda,
    Mdt,
    Mdx,
    Mft,
    Mina,
    Mir,
    Mith,
    Mln,
    Mob,
    Mod,
    Movr,
    Mth,
    Mtl,
    Multi,
    Nano,
    Nas,
    Nav,
    Nbs,
    Nbt,
    Ncash,
    Near,
    Nebl,
    Neo,
    Ngn,
    Nkn,
    Nmr,
    Npxs,
    Nu,
    Nuls,
    Nxs,
    Oax,
    Ocean,
    Og,
    Ogn,
    Om,
    One,
    Ong,
    Ont,
    Ooki,
    Op,
    Orn,
    Ost,
    Oxt,
    Paxg,
    People,
    Perl,
    Pha,
    Phb,
    Phx,
    Pivx,
    Pla,
    Pnt,
    Poa,
    Poe,
    Pols,
    Poly,
    Pond,
    Porto,
    Powr,
    Ppt,
    Prom,
    Pros,
    Psg,
    Pundix,
    Pyr,
    Qi,
    Qkc,
    Qlc,
    Qnt,
    Qsp,
    Qtum,
    Quick,
    Ramp,
    Rare,
    Ray,
    Rcn,
    Rdn,
    Reef,
    Rei,
    Ren,
    Rep,
    Req,
    Rif,
    Rlc,
    Rose,
    Rpx,
    Rsr,
    Rub,
    Rune,
    Rvn,
    Salt,
    Santos,
    Sc,
    Scrt,
    Sfp,
    Sky,
    Sngls,
    Snm,
    Snt,
    Sol,
    Sparta,
    Spell,
    Srm,
    Ssv,
    Steem,
    Stmx,
    Storm,
    Stpt,
    Strat,
    Strax,
    Stx,
    Sub,
    Sun,
    Super,
    Susd,
    Swrv,
    Sys,
    T,
    Tct,
    Tfuel,
    Theta,
    Tko,
    Tlm,
    Tnb,
    Tnt,
    Tomo,
    Torn,
    Trb,
    Tribe,
    Trig,
    Troy,
    Tru,
    Trx,
    Try,
    Tusd,
    Tvk,
    Twt,
    Uah,
    Uft,
    Unfi,
    Up,
    Usdp,
    Usds,
    Usdsb,
    Ustc,
    Utk,
    Vai,
    Ven,
    Vet,
    Vgx,
    Via,
    Vib,
    Vibe,
    Vidt,
    Vite,
    Voxel,
    Vtho,
    Wabi,
    Wan,
    Waves,
    Waxp,
    Win,
    Wing,
    Wings,
    Wnxm,
    Woo,
    Wpr,
    Wrx,
    Wtc,
    Xec,
    Xem,
    Xmr,
    Xno,
    Xtz,
    Xvg,
    Xvs,
    Xzc,
    Yfii,
    Ygg,
    Yoyo,
    Zar,
    Zec,
    Zen,
    Zil,
}

pub const CURRENCY_VARIANTS: [&str; 476] = [
    "aave", "ada", "algo", "alpha", "amp", "ant", "audio", "avax", "axs", "band", "bat", "bch",
    "btc", "cel", "chz", "comp", "crv", "ctsi", "cvx", "dai", "dydx", "enj", "ens", "eth", "eth2",
    "eur", "eurt", "fet", "ftm", "ftt", "gala", "gbp", "gods", "grt", "gusd", "hbar", "imx", "inj",
    "knc", "link", "lrc", "ltc", "mana", "matic", "mkr", "nexo", "omg", "1inch", "pax", "perp",
    "rad", "rgt", "rly", "rndr", "sand", "sgb", "shib", "skl", "slp", "snx", "storj", "sushi",
    "sxp", "uma", "uni", "usd", "usdc", "usdt", "ust", "vega", "wbtc", "xlm", "xrp", "yfi", "zrx",
    "aca", "ach", "acm", "adx", "ae", "aergo", "agi", "agix", "agld", "aion", "akro", "alcx",
    "alice", "alpaca", "alpine", "amb", "anc", "ankr", "any", "ape", "api3", "appc", "ar", "ardr",
    "ark", "arn", "arpa", "asr", "ast", "astr", "ata", "atm", "atom", "auction", "aud", "auto",
    "ava", "b", "badger", "bake", "bal", "bar", "bcc", "bcd", "bcha", "bchabc", "bchsv", "bcn",
    "bcpt", "bdot", "beam", "bear", "bel", "beta", "beth", "bico", "bidr", "bifi", "bkrw", "blz",
    "bnb", "bnt", "bnx", "bond", "bot", "bqx", "brd", "brl", "bsw", "btcb", "btcst", "btg", "bts",
    "btt", "bttc", "bull", "burger", "busd", "bvnd", "bzrx", "c98", "cake", "cdt", "celo", "celr",
    "cfx", "chat", "chess", "chr", "city", "ckb", "cloak", "clv", "cmt", "cnd", "cocos", "cos",
    "coti", "cover", "cream", "ctk", "ctxc", "cvc", "cvp", "dar", "dash", "data", "dcr", "dego",
    "dent", "dexe", "df", "dgb", "dgd", "dia", "dlt", "dnt", "dock", "dodo", "doge", "dot", "down",
    "drep", "dusk", "easy", "edo", "egld", "elf", "eng", "eos", "eps", "epx", "erd", "ern", "etc",
    "evx", "ez", "farm", "fida", "fil", "fio", "firo", "fis", "flm", "flow", "flux", "for",
    "forth", "front", "fuel", "fun", "fxs", "gal", "gas", "ghst", "glm", "glmr", "gmt", "gno",
    "gnt", "go", "grs", "gtc", "gto", "gvt", "gxs", "gyen", "hard", "hc", "hegic", "high", "hive",
    "hnt", "hot", "hsr", "icn", "icp", "icx", "idex", "idrt", "ilv", "ins", "iost", "iota", "iotx",
    "iq", "iris", "jasmy", "joe", "jst", "juv", "kava", "kda", "keep", "key", "klay", "kmd",
    "kp3r", "ksm", "lazio", "ldo", "lend", "lina", "lit", "loka", "loom", "lpt", "lsk", "lto",
    "lun", "luna", "lunc", "mask", "mbl", "mbox", "mc", "mco", "mda", "mdt", "mdx", "mft", "mina",
    "mir", "mith", "mln", "mob", "mod", "movr", "mth", "mtl", "multi", "nano", "nas", "nav", "nbs",
    "nbt", "ncash", "near", "nebl", "neo", "ngn", "nkn", "nmr", "npxs", "nu", "nuls", "nxs", "oax",
    "ocean", "og", "ogn", "om", "one", "ong", "ont", "ooki", "op", "orn", "ost", "oxt", "paxg",
    "people", "perl", "pha", "phb", "phx", "pivx", "pla", "pnt", "poa", "poe", "pols", "poly",
    "pond", "porto", "powr", "ppt", "prom", "pros", "psg", "pundix", "pyr", "qi", "qkc", "qlc",
    "qnt", "qsp", "qtum", "quick", "ramp", "rare", "ray", "rcn", "rdn", "reef", "rei", "ren",
    "rep", "req", "rif", "rlc", "rose", "rpx", "rsr", "rub", "rune", "rvn", "salt", "santos", "sc",
    "scrt", "sfp", "sky", "sngls", "snm", "snt", "sol", "sparta", "spell", "srm", "ssv", "steem",
    "stmx", "storm", "stpt", "strat", "strax", "stx", "sub", "sun", "super", "susd", "swrv", "sys",
    "t", "tct", "tfuel", "theta", "tko", "tlm", "tnb", "tnt", "tomo", "torn", "trb", "tribe",
    "trig", "troy", "tru", "trx", "try", "tusd", "tvk", "twt", "uah", "uft", "unfi", "up", "usdp",
    "usds", "usdsb", "ustc", "utk", "vai", "ven", "vet", "vgx", "via", "vib", "vibe", "vidt",
    "vite", "voxel", "vtho", "wabi", "wan", "waves", "waxp", "win", "wing", "wings", "wnxm", "woo",
    "wpr", "wrx", "wtc", "xec", "xem", "xmr", "xno", "xtz", "xvg", "xvs", "xzc", "yfii", "ygg",
    "yoyo", "zar", "zec", "zen", "zil",
];

impl std::str::FromStr for Currency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Currency::*;
        if s.to_lowercase() == CURRENCY_VARIANTS[0] {
            Ok(Aave)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[1] {
            Ok(Ada)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[2] {
            Ok(Algo)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[3] {
            Ok(Alpha)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[4] {
            Ok(Amp)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[5] {
            Ok(Ant)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[6] {
            Ok(Audio)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[7] {
            Ok(Avax)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[8] {
            Ok(Axs)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[9] {
            Ok(Band)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[10] {
            Ok(Bat)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[11] {
            Ok(Bch)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[12] {
            Ok(Btc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[13] {
            Ok(Cel)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[14] {
            Ok(Chz)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[15] {
            Ok(Comp)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[16] {
            Ok(Crv)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[17] {
            Ok(Ctsi)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[18] {
            Ok(Cvx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[19] {
            Ok(Dai)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[20] {
            Ok(Dydx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[21] {
            Ok(Enj)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[22] {
            Ok(Ens)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[23] {
            Ok(Eth)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[24] {
            Ok(Eth2)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[25] {
            Ok(Eur)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[26] {
            Ok(Eurt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[27] {
            Ok(Fet)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[28] {
            Ok(Ftm)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[29] {
            Ok(Ftt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[30] {
            Ok(Gala)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[31] {
            Ok(Gbp)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[32] {
            Ok(Gods)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[33] {
            Ok(Grt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[34] {
            Ok(Gusd)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[35] {
            Ok(Hbar)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[36] {
            Ok(Imx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[37] {
            Ok(Inj)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[38] {
            Ok(Knc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[39] {
            Ok(Link)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[40] {
            Ok(Lrc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[41] {
            Ok(Ltc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[42] {
            Ok(Mana)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[43] {
            Ok(Matic)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[44] {
            Ok(Mkr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[45] {
            Ok(Nexo)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[46] {
            Ok(Omg)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[47] {
            Ok(OneInch)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[48] {
            Ok(Pax)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[49] {
            Ok(Perp)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[50] {
            Ok(Rad)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[51] {
            Ok(Rgt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[52] {
            Ok(Rly)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[53] {
            Ok(Rndr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[54] {
            Ok(Sand)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[55] {
            Ok(Sgb)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[56] {
            Ok(Shib)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[57] {
            Ok(Skl)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[58] {
            Ok(Slp)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[59] {
            Ok(Snx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[60] {
            Ok(Storj)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[61] {
            Ok(Sushi)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[62] {
            Ok(Sxp)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[63] {
            Ok(Uma)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[64] {
            Ok(Uni)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[65] {
            Ok(Usd)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[66] {
            Ok(Usdc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[67] {
            Ok(Usdt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[68] {
            Ok(Ust)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[69] {
            Ok(Vega)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[70] {
            Ok(Wbtc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[71] {
            Ok(Xlm)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[72] {
            Ok(Xrp)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[73] {
            Ok(Yfi)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[74] {
            Ok(Zrx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[75] {
            Ok(Aca)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[76] {
            Ok(Ach)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[77] {
            Ok(Acm)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[78] {
            Ok(Adx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[79] {
            Ok(Ae)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[80] {
            Ok(Aergo)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[81] {
            Ok(Agi)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[82] {
            Ok(Agix)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[83] {
            Ok(Agld)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[84] {
            Ok(Aion)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[85] {
            Ok(Akro)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[86] {
            Ok(Alcx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[87] {
            Ok(Alice)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[88] {
            Ok(Alpaca)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[89] {
            Ok(Alpine)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[90] {
            Ok(Amb)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[91] {
            Ok(Anc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[92] {
            Ok(Ankr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[93] {
            Ok(Any)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[94] {
            Ok(Ape)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[95] {
            Ok(Api3)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[96] {
            Ok(Appc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[97] {
            Ok(Ar)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[98] {
            Ok(Ardr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[99] {
            Ok(Ark)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[100] {
            Ok(Arn)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[101] {
            Ok(Arpa)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[102] {
            Ok(Asr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[103] {
            Ok(Ast)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[104] {
            Ok(Astr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[105] {
            Ok(Ata)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[106] {
            Ok(Atm)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[107] {
            Ok(Atom)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[108] {
            Ok(Auction)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[109] {
            Ok(Aud)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[110] {
            Ok(Auto)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[111] {
            Ok(Ava)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[112] {
            Ok(B)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[113] {
            Ok(Badger)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[114] {
            Ok(Bake)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[115] {
            Ok(Bal)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[116] {
            Ok(Bar)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[117] {
            Ok(Bcc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[118] {
            Ok(Bcd)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[119] {
            Ok(Bcha)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[120] {
            Ok(Bchabc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[121] {
            Ok(Bchsv)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[122] {
            Ok(Bcn)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[123] {
            Ok(Bcpt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[124] {
            Ok(Bdot)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[125] {
            Ok(Beam)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[126] {
            Ok(Bear)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[127] {
            Ok(Bel)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[128] {
            Ok(Beta)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[129] {
            Ok(Beth)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[130] {
            Ok(Bico)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[131] {
            Ok(Bidr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[132] {
            Ok(Bifi)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[133] {
            Ok(Bkrw)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[134] {
            Ok(Blz)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[135] {
            Ok(Bnb)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[136] {
            Ok(Bnt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[137] {
            Ok(Bnx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[138] {
            Ok(Bond)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[139] {
            Ok(Bot)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[140] {
            Ok(Bqx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[141] {
            Ok(Brd)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[142] {
            Ok(Brl)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[143] {
            Ok(Bsw)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[144] {
            Ok(Btcb)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[145] {
            Ok(Btcst)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[146] {
            Ok(Btg)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[147] {
            Ok(Bts)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[148] {
            Ok(Btt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[149] {
            Ok(Bttc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[150] {
            Ok(Bull)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[151] {
            Ok(Burger)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[152] {
            Ok(Busd)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[153] {
            Ok(Bvnd)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[154] {
            Ok(Bzrx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[155] {
            Ok(C98)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[156] {
            Ok(Cake)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[157] {
            Ok(Cdt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[158] {
            Ok(Celo)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[159] {
            Ok(Celr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[160] {
            Ok(Cfx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[161] {
            Ok(Chat)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[162] {
            Ok(Chess)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[163] {
            Ok(Chr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[164] {
            Ok(City)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[165] {
            Ok(Ckb)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[166] {
            Ok(Cloak)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[167] {
            Ok(Clv)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[168] {
            Ok(Cmt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[169] {
            Ok(Cnd)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[170] {
            Ok(Cocos)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[171] {
            Ok(Cos)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[172] {
            Ok(Coti)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[173] {
            Ok(Cover)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[174] {
            Ok(Cream)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[175] {
            Ok(Ctk)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[176] {
            Ok(Ctxc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[177] {
            Ok(Cvc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[178] {
            Ok(Cvp)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[179] {
            Ok(Dar)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[180] {
            Ok(Dash)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[181] {
            Ok(Data)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[182] {
            Ok(Dcr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[183] {
            Ok(Dego)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[184] {
            Ok(Dent)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[185] {
            Ok(Dexe)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[186] {
            Ok(Df)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[187] {
            Ok(Dgb)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[188] {
            Ok(Dgd)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[189] {
            Ok(Dia)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[190] {
            Ok(Dlt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[191] {
            Ok(Dnt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[192] {
            Ok(Dock)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[193] {
            Ok(Dodo)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[194] {
            Ok(Doge)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[195] {
            Ok(Dot)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[196] {
            Ok(Down)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[197] {
            Ok(Drep)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[198] {
            Ok(Dusk)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[199] {
            Ok(Easy)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[200] {
            Ok(Edo)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[201] {
            Ok(Egld)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[202] {
            Ok(Elf)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[203] {
            Ok(Eng)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[204] {
            Ok(Eos)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[205] {
            Ok(Eps)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[206] {
            Ok(Epx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[207] {
            Ok(Erd)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[208] {
            Ok(Ern)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[209] {
            Ok(Etc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[210] {
            Ok(Evx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[211] {
            Ok(Ez)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[212] {
            Ok(Farm)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[213] {
            Ok(Fida)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[214] {
            Ok(Fil)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[215] {
            Ok(Fio)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[216] {
            Ok(Firo)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[217] {
            Ok(Fis)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[218] {
            Ok(Flm)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[219] {
            Ok(Flow)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[220] {
            Ok(Flux)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[221] {
            Ok(For)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[222] {
            Ok(Forth)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[223] {
            Ok(Front)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[224] {
            Ok(Fuel)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[225] {
            Ok(Fun)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[226] {
            Ok(Fxs)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[227] {
            Ok(Gal)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[228] {
            Ok(Gas)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[229] {
            Ok(Ghst)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[230] {
            Ok(Glm)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[231] {
            Ok(Glmr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[232] {
            Ok(Gmt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[233] {
            Ok(Gno)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[234] {
            Ok(Gnt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[235] {
            Ok(Go)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[236] {
            Ok(Grs)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[237] {
            Ok(Gtc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[238] {
            Ok(Gto)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[239] {
            Ok(Gvt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[240] {
            Ok(Gxs)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[241] {
            Ok(Gyen)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[242] {
            Ok(Hard)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[243] {
            Ok(Hc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[244] {
            Ok(Hegic)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[245] {
            Ok(High)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[246] {
            Ok(Hive)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[247] {
            Ok(Hnt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[248] {
            Ok(Hot)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[249] {
            Ok(Hsr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[250] {
            Ok(Icn)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[251] {
            Ok(Icp)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[252] {
            Ok(Icx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[253] {
            Ok(Idex)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[254] {
            Ok(Idrt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[255] {
            Ok(Ilv)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[256] {
            Ok(Ins)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[257] {
            Ok(Iost)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[258] {
            Ok(Iota)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[259] {
            Ok(Iotx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[260] {
            Ok(Iq)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[261] {
            Ok(Iris)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[262] {
            Ok(Jasmy)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[263] {
            Ok(Joe)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[264] {
            Ok(Jst)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[265] {
            Ok(Juv)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[266] {
            Ok(Kava)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[267] {
            Ok(Kda)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[268] {
            Ok(Keep)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[269] {
            Ok(Key)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[270] {
            Ok(Klay)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[271] {
            Ok(Kmd)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[272] {
            Ok(Kp3r)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[273] {
            Ok(Ksm)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[274] {
            Ok(Lazio)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[275] {
            Ok(Ldo)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[276] {
            Ok(Lend)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[277] {
            Ok(Lina)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[278] {
            Ok(Lit)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[279] {
            Ok(Loka)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[280] {
            Ok(Loom)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[281] {
            Ok(Lpt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[282] {
            Ok(Lsk)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[283] {
            Ok(Lto)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[284] {
            Ok(Lun)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[285] {
            Ok(Luna)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[286] {
            Ok(Lunc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[287] {
            Ok(Mask)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[288] {
            Ok(Mbl)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[289] {
            Ok(Mbox)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[290] {
            Ok(Mc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[291] {
            Ok(Mco)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[292] {
            Ok(Mda)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[293] {
            Ok(Mdt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[294] {
            Ok(Mdx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[295] {
            Ok(Mft)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[296] {
            Ok(Mina)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[297] {
            Ok(Mir)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[298] {
            Ok(Mith)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[299] {
            Ok(Mln)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[300] {
            Ok(Mob)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[301] {
            Ok(Mod)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[302] {
            Ok(Movr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[303] {
            Ok(Mth)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[304] {
            Ok(Mtl)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[305] {
            Ok(Multi)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[306] {
            Ok(Nano)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[307] {
            Ok(Nas)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[308] {
            Ok(Nav)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[309] {
            Ok(Nbs)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[310] {
            Ok(Nbt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[311] {
            Ok(Ncash)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[312] {
            Ok(Near)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[313] {
            Ok(Nebl)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[314] {
            Ok(Neo)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[315] {
            Ok(Ngn)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[316] {
            Ok(Nkn)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[317] {
            Ok(Nmr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[318] {
            Ok(Npxs)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[319] {
            Ok(Nu)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[320] {
            Ok(Nuls)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[321] {
            Ok(Nxs)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[322] {
            Ok(Oax)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[323] {
            Ok(Ocean)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[324] {
            Ok(Og)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[325] {
            Ok(Ogn)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[326] {
            Ok(Om)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[327] {
            Ok(One)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[328] {
            Ok(Ong)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[329] {
            Ok(Ont)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[330] {
            Ok(Ooki)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[331] {
            Ok(Op)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[332] {
            Ok(Orn)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[333] {
            Ok(Ost)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[334] {
            Ok(Oxt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[335] {
            Ok(Paxg)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[336] {
            Ok(People)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[337] {
            Ok(Perl)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[338] {
            Ok(Pha)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[339] {
            Ok(Phb)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[340] {
            Ok(Phx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[341] {
            Ok(Pivx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[342] {
            Ok(Pla)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[343] {
            Ok(Pnt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[344] {
            Ok(Poa)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[345] {
            Ok(Poe)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[346] {
            Ok(Pols)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[347] {
            Ok(Poly)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[348] {
            Ok(Pond)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[349] {
            Ok(Porto)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[350] {
            Ok(Powr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[351] {
            Ok(Ppt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[352] {
            Ok(Prom)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[353] {
            Ok(Pros)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[354] {
            Ok(Psg)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[355] {
            Ok(Pundix)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[356] {
            Ok(Pyr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[357] {
            Ok(Qi)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[358] {
            Ok(Qkc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[359] {
            Ok(Qlc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[360] {
            Ok(Qnt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[361] {
            Ok(Qsp)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[362] {
            Ok(Qtum)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[363] {
            Ok(Quick)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[364] {
            Ok(Ramp)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[365] {
            Ok(Rare)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[366] {
            Ok(Ray)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[367] {
            Ok(Rcn)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[368] {
            Ok(Rdn)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[369] {
            Ok(Reef)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[370] {
            Ok(Rei)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[371] {
            Ok(Ren)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[372] {
            Ok(Rep)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[373] {
            Ok(Req)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[374] {
            Ok(Rif)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[375] {
            Ok(Rlc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[376] {
            Ok(Rose)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[377] {
            Ok(Rpx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[378] {
            Ok(Rsr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[379] {
            Ok(Rub)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[380] {
            Ok(Rune)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[381] {
            Ok(Rvn)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[382] {
            Ok(Salt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[383] {
            Ok(Santos)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[384] {
            Ok(Sc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[385] {
            Ok(Scrt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[386] {
            Ok(Sfp)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[387] {
            Ok(Sky)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[388] {
            Ok(Sngls)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[389] {
            Ok(Snm)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[390] {
            Ok(Snt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[391] {
            Ok(Sol)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[392] {
            Ok(Sparta)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[393] {
            Ok(Spell)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[394] {
            Ok(Srm)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[395] {
            Ok(Ssv)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[396] {
            Ok(Steem)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[397] {
            Ok(Stmx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[398] {
            Ok(Storm)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[399] {
            Ok(Stpt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[400] {
            Ok(Strat)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[401] {
            Ok(Strax)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[402] {
            Ok(Stx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[403] {
            Ok(Sub)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[404] {
            Ok(Sun)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[405] {
            Ok(Super)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[406] {
            Ok(Susd)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[407] {
            Ok(Swrv)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[408] {
            Ok(Sys)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[409] {
            Ok(T)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[410] {
            Ok(Tct)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[411] {
            Ok(Tfuel)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[412] {
            Ok(Theta)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[413] {
            Ok(Tko)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[414] {
            Ok(Tlm)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[415] {
            Ok(Tnb)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[416] {
            Ok(Tnt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[417] {
            Ok(Tomo)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[418] {
            Ok(Torn)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[419] {
            Ok(Trb)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[420] {
            Ok(Tribe)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[421] {
            Ok(Trig)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[422] {
            Ok(Troy)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[423] {
            Ok(Tru)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[424] {
            Ok(Trx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[425] {
            Ok(Try)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[426] {
            Ok(Tusd)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[427] {
            Ok(Tvk)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[428] {
            Ok(Twt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[429] {
            Ok(Uah)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[430] {
            Ok(Uft)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[431] {
            Ok(Unfi)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[432] {
            Ok(Up)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[433] {
            Ok(Usdp)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[434] {
            Ok(Usds)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[435] {
            Ok(Usdsb)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[436] {
            Ok(Ustc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[437] {
            Ok(Utk)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[438] {
            Ok(Vai)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[439] {
            Ok(Ven)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[440] {
            Ok(Vet)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[441] {
            Ok(Vgx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[442] {
            Ok(Via)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[443] {
            Ok(Vib)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[444] {
            Ok(Vibe)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[445] {
            Ok(Vidt)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[446] {
            Ok(Vite)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[447] {
            Ok(Voxel)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[448] {
            Ok(Vtho)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[449] {
            Ok(Wabi)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[450] {
            Ok(Wan)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[451] {
            Ok(Waves)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[452] {
            Ok(Waxp)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[453] {
            Ok(Win)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[454] {
            Ok(Wing)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[455] {
            Ok(Wings)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[456] {
            Ok(Wnxm)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[457] {
            Ok(Woo)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[458] {
            Ok(Wpr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[459] {
            Ok(Wrx)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[460] {
            Ok(Wtc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[461] {
            Ok(Xec)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[462] {
            Ok(Xem)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[463] {
            Ok(Xmr)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[464] {
            Ok(Xno)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[465] {
            Ok(Xtz)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[466] {
            Ok(Xvg)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[467] {
            Ok(Xvs)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[468] {
            Ok(Xzc)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[469] {
            Ok(Yfii)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[470] {
            Ok(Ygg)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[471] {
            Ok(Yoyo)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[472] {
            Ok(Zar)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[473] {
            Ok(Zec)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[474] {
            Ok(Zen)
        } else if s.to_lowercase() == CURRENCY_VARIANTS[475] {
            Ok(Zil)
        } else {
            Err("invalid currency".into())
        }
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::OneInch => write!(f, "1inch"),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Symbol {
    pub base: Currency,
    pub quote: Currency,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Market {
    exchange: Exchange,
    symbol: Symbol,
}
