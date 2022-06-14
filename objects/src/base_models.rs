extern crate enum_iterator;
use enum_iterator::Sequence;
/// This enum stands for the type of each Hai in the player's hand.
/// M = 萬子
/// P = 筒子
/// S = 索子
/// Z = 字牌
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Hai {
  M(u8),
  P(u8),
  S(u8),
  Z(u8),
}

#[derive(PartialEq, Clone)]
pub enum MenzuType {
  Ankou,
  Minkou,
  Shunzu,
}

#[derive(PartialEq, Clone)]
pub struct Menzu {
  menzu_type: MenzuType,
  content: Vec<Hai>,
  fu: u8,
}

pub struct Atama {
  content: Hai,
  fu: u8,
}

/// How the player wins.
/// Ron means another player discards a Hai that the player wants.
/// Tsumo means the player herself fetched the needed Hai.
pub enum GoOutType {
  Ron(Hai),
  Tsumo(Hai),
}

/// Yaku's that has one han.
#[derive(Debug, Enum, Sequence, PartialEq)]
pub enum Yaku {
  // 立直
  Riichi,
  // 自摸
  Tsumo,
  // 一発
  Ippatsu,
  // Sangenhai 三元牌の役
  // A.k.a. W / G / R Dragon.
  // 白
  Haku,
  // 発
  Hatsu,
  // 中
  Chun,
  // Self-wind or Round-wind.
  // 自風
  Jikaze,
  // 門風
  Monfon,
  // 断幺
  Tanyao,
  // 一盃口
  Iipeikou,
  // 平和
  Pinfu,
  // 槍槓
  Chankan,
  // 嶺上開花
  Rinshan,
  // 海底撈月
  Haitei,
  // 河底撈鱼
  Houtei,
  // ダブリー
  Daburii,
  // 七対子
  Chiitoi,
  // 対々和
  Toitoi,
  // 三暗刻
  Sanankou,
  // 三色同刻
  Sanshaku,
  // 三色同順
  Sanshuu,
  // 混老頭
  Honroutou,
  // 一気通貫
  Iitsuu,
  // 混全带幺九
  Chantaa,
  // 小三元
  Shousangen,
  // 三槓子
  Sanganzu,
  // 混一色
  Honiitsu,
  // 純全带幺九
  Chunchan,
  // 二盃口
  Ryanpeikou,
  // 流し満貫
  Nagashimankan,
  // 清一色
  Chiniitsu,
  // 天和
  Tenhou,
  // 地和
  Chiihou,
  // 緑一色
  Ryouiisou,
  // 国士無双
  Kokushimusou,
  // 九蓮宝燈
  Chuureiboutou,
  // 大三元
  Daisangen,
  // 小四喜
  Shousuushii,
  // 字一色
  Zuuiisou,
  // 清老頭
  Chinroutou,
  // 四暗刻
  Suuankou,
  // 四槓子
  Suukanzu,
  // 九蓮宝燈九面待ち
  Chuureikyuumenmachi,
  // 国士無双十三面待ち
  Kokushijuusanmenmachi,
  // 大四喜
  Daisuushii,
  // 四暗刻単騎
  Suuankoutanki,
}

pub const MANKAN: u8 = 5;

/// Maps from yaku to its corresponding han.
pub fn init_yaku_han_lookup() -> enum_map::EnumMap<Yaku, u8> {
  let mut yaku_han_lookup: enum_map::EnumMap<Yaku, u8> = enum_map::EnumMap::default();

  let yaku_vec = enum_iterator::all::<Yaku>().collect::<Vec<_>>();
  let mut cur_han: u8 = 1;
  for yaku in yaku_vec {
    // Change the current han.
    match yaku {
      Yaku::Daburii => cur_han = 2,
      Yaku::Honiitsu => cur_han = 3,
      Yaku::Nagashimankan => cur_han = 5,
      Yaku::Chiniitsu => cur_han = 6,
      Yaku::Tenhou => cur_han = 13,
      Yaku::Chuureikyuumenmachi => cur_han = 26,
      _ => (),
    };

    yaku_han_lookup[yaku] = cur_han;
  }

  yaku_han_lookup
}

pub fn hai_type_eq(lhs: &Hai, rhs: &Hai) -> bool {
  std::mem::discriminant(lhs) == std::mem::discriminant(rhs)
}

pub fn is_incremental(lhs: &Hai, rhs: &Hai) -> bool {
  // Do a sanity check.
  assert_eq!(hai_type_eq(lhs, rhs), true);

  let lhs_val = match lhs {
    Hai::M(val) => val,
    Hai::P(val) => val,
    Hai::S(val) => val,
    _ => return false,
  };

  let rhs_val = match rhs {
    Hai::M(val) => val,
    Hai::P(val) => val,
    Hai::S(val) => val,
    _ => return false,
  };

  *lhs_val + 1 == *rhs_val
}
