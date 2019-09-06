#[doc = "Reader of register MCFG"]
pub type R = crate::R<u32, super::MCFG>;
#[doc = "Data RAM Size\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRAM_SZ_A {
  #[doc = "0: No memory module"]
  DRAM_SZ_0,
  #[doc = "4: 2K bytes"]
  DRAM_SZ_4,
  #[doc = "5: 3K bytes"]
  DRAM_SZ_5,
  #[doc = "6: 4K bytes"]
  DRAM_SZ_6,
  #[doc = "7: 6K bytes"]
  DRAM_SZ_7,
  #[doc = "8: 8K bytes"]
  DRAM_SZ_8,
  #[doc = "9: 12K bytes"]
  DRAM_SZ_9,
  #[doc = "10: 16K bytes"]
  DRAM_SZ_10,
  #[doc = "11: 24K bytes"]
  DRAM_SZ_11,
  #[doc = "12: 32K bytes"]
  DRAM_SZ_12,
  #[doc = "13: 48K bytes"]
  DRAM_SZ_13,
  #[doc = "14: 64K bytes"]
  DRAM_SZ_14,
  #[doc = "15: 96K bytes"]
  DRAM_SZ_15,
}
impl From<DRAM_SZ_A> for u8 {
  #[inline(always)]
  fn from(variant: DRAM_SZ_A) -> Self {
    match variant {
      DRAM_SZ_A::DRAM_SZ_0 => 0,
      DRAM_SZ_A::DRAM_SZ_4 => 4,
      DRAM_SZ_A::DRAM_SZ_5 => 5,
      DRAM_SZ_A::DRAM_SZ_6 => 6,
      DRAM_SZ_A::DRAM_SZ_7 => 7,
      DRAM_SZ_A::DRAM_SZ_8 => 8,
      DRAM_SZ_A::DRAM_SZ_9 => 9,
      DRAM_SZ_A::DRAM_SZ_10 => 10,
      DRAM_SZ_A::DRAM_SZ_11 => 11,
      DRAM_SZ_A::DRAM_SZ_12 => 12,
      DRAM_SZ_A::DRAM_SZ_13 => 13,
      DRAM_SZ_A::DRAM_SZ_14 => 14,
      DRAM_SZ_A::DRAM_SZ_15 => 15,
    }
  }
}
#[doc = "Reader of field `DRAM_SZ`"]
pub type DRAM_SZ_R = crate::R<u8, DRAM_SZ_A>;
impl DRAM_SZ_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, DRAM_SZ_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(DRAM_SZ_A::DRAM_SZ_0),
      4 => Val(DRAM_SZ_A::DRAM_SZ_4),
      5 => Val(DRAM_SZ_A::DRAM_SZ_5),
      6 => Val(DRAM_SZ_A::DRAM_SZ_6),
      7 => Val(DRAM_SZ_A::DRAM_SZ_7),
      8 => Val(DRAM_SZ_A::DRAM_SZ_8),
      9 => Val(DRAM_SZ_A::DRAM_SZ_9),
      10 => Val(DRAM_SZ_A::DRAM_SZ_10),
      11 => Val(DRAM_SZ_A::DRAM_SZ_11),
      12 => Val(DRAM_SZ_A::DRAM_SZ_12),
      13 => Val(DRAM_SZ_A::DRAM_SZ_13),
      14 => Val(DRAM_SZ_A::DRAM_SZ_14),
      15 => Val(DRAM_SZ_A::DRAM_SZ_15),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `DRAM_SZ_0`"]
  #[inline(always)]
  pub fn is_dram_sz_0(&self) -> bool {
    *self == DRAM_SZ_A::DRAM_SZ_0
  }
  #[doc = "Checks if the value of the field is `DRAM_SZ_4`"]
  #[inline(always)]
  pub fn is_dram_sz_4(&self) -> bool {
    *self == DRAM_SZ_A::DRAM_SZ_4
  }
  #[doc = "Checks if the value of the field is `DRAM_SZ_5`"]
  #[inline(always)]
  pub fn is_dram_sz_5(&self) -> bool {
    *self == DRAM_SZ_A::DRAM_SZ_5
  }
  #[doc = "Checks if the value of the field is `DRAM_SZ_6`"]
  #[inline(always)]
  pub fn is_dram_sz_6(&self) -> bool {
    *self == DRAM_SZ_A::DRAM_SZ_6
  }
  #[doc = "Checks if the value of the field is `DRAM_SZ_7`"]
  #[inline(always)]
  pub fn is_dram_sz_7(&self) -> bool {
    *self == DRAM_SZ_A::DRAM_SZ_7
  }
  #[doc = "Checks if the value of the field is `DRAM_SZ_8`"]
  #[inline(always)]
  pub fn is_dram_sz_8(&self) -> bool {
    *self == DRAM_SZ_A::DRAM_SZ_8
  }
  #[doc = "Checks if the value of the field is `DRAM_SZ_9`"]
  #[inline(always)]
  pub fn is_dram_sz_9(&self) -> bool {
    *self == DRAM_SZ_A::DRAM_SZ_9
  }
  #[doc = "Checks if the value of the field is `DRAM_SZ_10`"]
  #[inline(always)]
  pub fn is_dram_sz_10(&self) -> bool {
    *self == DRAM_SZ_A::DRAM_SZ_10
  }
  #[doc = "Checks if the value of the field is `DRAM_SZ_11`"]
  #[inline(always)]
  pub fn is_dram_sz_11(&self) -> bool {
    *self == DRAM_SZ_A::DRAM_SZ_11
  }
  #[doc = "Checks if the value of the field is `DRAM_SZ_12`"]
  #[inline(always)]
  pub fn is_dram_sz_12(&self) -> bool {
    *self == DRAM_SZ_A::DRAM_SZ_12
  }
  #[doc = "Checks if the value of the field is `DRAM_SZ_13`"]
  #[inline(always)]
  pub fn is_dram_sz_13(&self) -> bool {
    *self == DRAM_SZ_A::DRAM_SZ_13
  }
  #[doc = "Checks if the value of the field is `DRAM_SZ_14`"]
  #[inline(always)]
  pub fn is_dram_sz_14(&self) -> bool {
    *self == DRAM_SZ_A::DRAM_SZ_14
  }
  #[doc = "Checks if the value of the field is `DRAM_SZ_15`"]
  #[inline(always)]
  pub fn is_dram_sz_15(&self) -> bool {
    *self == DRAM_SZ_A::DRAM_SZ_15
  }
}
#[doc = "Instruction ROM Size\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IROM_SZ_A {
  #[doc = "0: No memory module"]
  IROM_SZ_0,
  #[doc = "4: 2K bytes"]
  IROM_SZ_4,
  #[doc = "5: 3K bytes"]
  IROM_SZ_5,
  #[doc = "6: 4K bytes"]
  IROM_SZ_6,
  #[doc = "7: 6K bytes"]
  IROM_SZ_7,
  #[doc = "8: 8K bytes"]
  IROM_SZ_8,
  #[doc = "9: 12K bytes"]
  IROM_SZ_9,
  #[doc = "10: 16K bytes"]
  IROM_SZ_10,
  #[doc = "11: 24K bytes"]
  IROM_SZ_11,
  #[doc = "12: 32K bytes"]
  IROM_SZ_12,
  #[doc = "13: 48K bytes"]
  IROM_SZ_13,
  #[doc = "14: 64K bytes"]
  IROM_SZ_14,
  #[doc = "15: 96K bytes"]
  IROM_SZ_15,
}
impl From<IROM_SZ_A> for u8 {
  #[inline(always)]
  fn from(variant: IROM_SZ_A) -> Self {
    match variant {
      IROM_SZ_A::IROM_SZ_0 => 0,
      IROM_SZ_A::IROM_SZ_4 => 4,
      IROM_SZ_A::IROM_SZ_5 => 5,
      IROM_SZ_A::IROM_SZ_6 => 6,
      IROM_SZ_A::IROM_SZ_7 => 7,
      IROM_SZ_A::IROM_SZ_8 => 8,
      IROM_SZ_A::IROM_SZ_9 => 9,
      IROM_SZ_A::IROM_SZ_10 => 10,
      IROM_SZ_A::IROM_SZ_11 => 11,
      IROM_SZ_A::IROM_SZ_12 => 12,
      IROM_SZ_A::IROM_SZ_13 => 13,
      IROM_SZ_A::IROM_SZ_14 => 14,
      IROM_SZ_A::IROM_SZ_15 => 15,
    }
  }
}
#[doc = "Reader of field `IROM_SZ`"]
pub type IROM_SZ_R = crate::R<u8, IROM_SZ_A>;
impl IROM_SZ_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, IROM_SZ_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(IROM_SZ_A::IROM_SZ_0),
      4 => Val(IROM_SZ_A::IROM_SZ_4),
      5 => Val(IROM_SZ_A::IROM_SZ_5),
      6 => Val(IROM_SZ_A::IROM_SZ_6),
      7 => Val(IROM_SZ_A::IROM_SZ_7),
      8 => Val(IROM_SZ_A::IROM_SZ_8),
      9 => Val(IROM_SZ_A::IROM_SZ_9),
      10 => Val(IROM_SZ_A::IROM_SZ_10),
      11 => Val(IROM_SZ_A::IROM_SZ_11),
      12 => Val(IROM_SZ_A::IROM_SZ_12),
      13 => Val(IROM_SZ_A::IROM_SZ_13),
      14 => Val(IROM_SZ_A::IROM_SZ_14),
      15 => Val(IROM_SZ_A::IROM_SZ_15),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `IROM_SZ_0`"]
  #[inline(always)]
  pub fn is_irom_sz_0(&self) -> bool {
    *self == IROM_SZ_A::IROM_SZ_0
  }
  #[doc = "Checks if the value of the field is `IROM_SZ_4`"]
  #[inline(always)]
  pub fn is_irom_sz_4(&self) -> bool {
    *self == IROM_SZ_A::IROM_SZ_4
  }
  #[doc = "Checks if the value of the field is `IROM_SZ_5`"]
  #[inline(always)]
  pub fn is_irom_sz_5(&self) -> bool {
    *self == IROM_SZ_A::IROM_SZ_5
  }
  #[doc = "Checks if the value of the field is `IROM_SZ_6`"]
  #[inline(always)]
  pub fn is_irom_sz_6(&self) -> bool {
    *self == IROM_SZ_A::IROM_SZ_6
  }
  #[doc = "Checks if the value of the field is `IROM_SZ_7`"]
  #[inline(always)]
  pub fn is_irom_sz_7(&self) -> bool {
    *self == IROM_SZ_A::IROM_SZ_7
  }
  #[doc = "Checks if the value of the field is `IROM_SZ_8`"]
  #[inline(always)]
  pub fn is_irom_sz_8(&self) -> bool {
    *self == IROM_SZ_A::IROM_SZ_8
  }
  #[doc = "Checks if the value of the field is `IROM_SZ_9`"]
  #[inline(always)]
  pub fn is_irom_sz_9(&self) -> bool {
    *self == IROM_SZ_A::IROM_SZ_9
  }
  #[doc = "Checks if the value of the field is `IROM_SZ_10`"]
  #[inline(always)]
  pub fn is_irom_sz_10(&self) -> bool {
    *self == IROM_SZ_A::IROM_SZ_10
  }
  #[doc = "Checks if the value of the field is `IROM_SZ_11`"]
  #[inline(always)]
  pub fn is_irom_sz_11(&self) -> bool {
    *self == IROM_SZ_A::IROM_SZ_11
  }
  #[doc = "Checks if the value of the field is `IROM_SZ_12`"]
  #[inline(always)]
  pub fn is_irom_sz_12(&self) -> bool {
    *self == IROM_SZ_A::IROM_SZ_12
  }
  #[doc = "Checks if the value of the field is `IROM_SZ_13`"]
  #[inline(always)]
  pub fn is_irom_sz_13(&self) -> bool {
    *self == IROM_SZ_A::IROM_SZ_13
  }
  #[doc = "Checks if the value of the field is `IROM_SZ_14`"]
  #[inline(always)]
  pub fn is_irom_sz_14(&self) -> bool {
    *self == IROM_SZ_A::IROM_SZ_14
  }
  #[doc = "Checks if the value of the field is `IROM_SZ_15`"]
  #[inline(always)]
  pub fn is_irom_sz_15(&self) -> bool {
    *self == IROM_SZ_A::IROM_SZ_15
  }
}
#[doc = "Instruction RAM Size\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRAM_SZ_A {
  #[doc = "0: No memory module"]
  IRAM_SZ_0,
  #[doc = "4: 2K bytes"]
  IRAM_SZ_4,
  #[doc = "5: 3K bytes"]
  IRAM_SZ_5,
  #[doc = "6: 4K bytes"]
  IRAM_SZ_6,
  #[doc = "7: 6K bytes"]
  IRAM_SZ_7,
  #[doc = "8: 8K bytes"]
  IRAM_SZ_8,
  #[doc = "9: 12K bytes"]
  IRAM_SZ_9,
  #[doc = "10: 16K bytes"]
  IRAM_SZ_10,
  #[doc = "11: 24K bytes"]
  IRAM_SZ_11,
  #[doc = "12: 32K bytes"]
  IRAM_SZ_12,
  #[doc = "13: 48K bytes"]
  IRAM_SZ_13,
  #[doc = "14: 64K bytes"]
  IRAM_SZ_14,
  #[doc = "15: 96K bytes"]
  IRAM_SZ_15,
}
impl From<IRAM_SZ_A> for u8 {
  #[inline(always)]
  fn from(variant: IRAM_SZ_A) -> Self {
    match variant {
      IRAM_SZ_A::IRAM_SZ_0 => 0,
      IRAM_SZ_A::IRAM_SZ_4 => 4,
      IRAM_SZ_A::IRAM_SZ_5 => 5,
      IRAM_SZ_A::IRAM_SZ_6 => 6,
      IRAM_SZ_A::IRAM_SZ_7 => 7,
      IRAM_SZ_A::IRAM_SZ_8 => 8,
      IRAM_SZ_A::IRAM_SZ_9 => 9,
      IRAM_SZ_A::IRAM_SZ_10 => 10,
      IRAM_SZ_A::IRAM_SZ_11 => 11,
      IRAM_SZ_A::IRAM_SZ_12 => 12,
      IRAM_SZ_A::IRAM_SZ_13 => 13,
      IRAM_SZ_A::IRAM_SZ_14 => 14,
      IRAM_SZ_A::IRAM_SZ_15 => 15,
    }
  }
}
#[doc = "Reader of field `IRAM_SZ`"]
pub type IRAM_SZ_R = crate::R<u8, IRAM_SZ_A>;
impl IRAM_SZ_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, IRAM_SZ_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(IRAM_SZ_A::IRAM_SZ_0),
      4 => Val(IRAM_SZ_A::IRAM_SZ_4),
      5 => Val(IRAM_SZ_A::IRAM_SZ_5),
      6 => Val(IRAM_SZ_A::IRAM_SZ_6),
      7 => Val(IRAM_SZ_A::IRAM_SZ_7),
      8 => Val(IRAM_SZ_A::IRAM_SZ_8),
      9 => Val(IRAM_SZ_A::IRAM_SZ_9),
      10 => Val(IRAM_SZ_A::IRAM_SZ_10),
      11 => Val(IRAM_SZ_A::IRAM_SZ_11),
      12 => Val(IRAM_SZ_A::IRAM_SZ_12),
      13 => Val(IRAM_SZ_A::IRAM_SZ_13),
      14 => Val(IRAM_SZ_A::IRAM_SZ_14),
      15 => Val(IRAM_SZ_A::IRAM_SZ_15),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `IRAM_SZ_0`"]
  #[inline(always)]
  pub fn is_iram_sz_0(&self) -> bool {
    *self == IRAM_SZ_A::IRAM_SZ_0
  }
  #[doc = "Checks if the value of the field is `IRAM_SZ_4`"]
  #[inline(always)]
  pub fn is_iram_sz_4(&self) -> bool {
    *self == IRAM_SZ_A::IRAM_SZ_4
  }
  #[doc = "Checks if the value of the field is `IRAM_SZ_5`"]
  #[inline(always)]
  pub fn is_iram_sz_5(&self) -> bool {
    *self == IRAM_SZ_A::IRAM_SZ_5
  }
  #[doc = "Checks if the value of the field is `IRAM_SZ_6`"]
  #[inline(always)]
  pub fn is_iram_sz_6(&self) -> bool {
    *self == IRAM_SZ_A::IRAM_SZ_6
  }
  #[doc = "Checks if the value of the field is `IRAM_SZ_7`"]
  #[inline(always)]
  pub fn is_iram_sz_7(&self) -> bool {
    *self == IRAM_SZ_A::IRAM_SZ_7
  }
  #[doc = "Checks if the value of the field is `IRAM_SZ_8`"]
  #[inline(always)]
  pub fn is_iram_sz_8(&self) -> bool {
    *self == IRAM_SZ_A::IRAM_SZ_8
  }
  #[doc = "Checks if the value of the field is `IRAM_SZ_9`"]
  #[inline(always)]
  pub fn is_iram_sz_9(&self) -> bool {
    *self == IRAM_SZ_A::IRAM_SZ_9
  }
  #[doc = "Checks if the value of the field is `IRAM_SZ_10`"]
  #[inline(always)]
  pub fn is_iram_sz_10(&self) -> bool {
    *self == IRAM_SZ_A::IRAM_SZ_10
  }
  #[doc = "Checks if the value of the field is `IRAM_SZ_11`"]
  #[inline(always)]
  pub fn is_iram_sz_11(&self) -> bool {
    *self == IRAM_SZ_A::IRAM_SZ_11
  }
  #[doc = "Checks if the value of the field is `IRAM_SZ_12`"]
  #[inline(always)]
  pub fn is_iram_sz_12(&self) -> bool {
    *self == IRAM_SZ_A::IRAM_SZ_12
  }
  #[doc = "Checks if the value of the field is `IRAM_SZ_13`"]
  #[inline(always)]
  pub fn is_iram_sz_13(&self) -> bool {
    *self == IRAM_SZ_A::IRAM_SZ_13
  }
  #[doc = "Checks if the value of the field is `IRAM_SZ_14`"]
  #[inline(always)]
  pub fn is_iram_sz_14(&self) -> bool {
    *self == IRAM_SZ_A::IRAM_SZ_14
  }
  #[doc = "Checks if the value of the field is `IRAM_SZ_15`"]
  #[inline(always)]
  pub fn is_iram_sz_15(&self) -> bool {
    *self == IRAM_SZ_A::IRAM_SZ_15
  }
}
impl R {
  #[doc = "Bits 8:11 - Data RAM Size"]
  #[inline(always)]
  pub fn dram_sz(&self) -> DRAM_SZ_R {
    DRAM_SZ_R::new(((self.bits >> 8) & 0x0f) as u8)
  }
  #[doc = "Bits 16:19 - Instruction ROM Size"]
  #[inline(always)]
  pub fn irom_sz(&self) -> IROM_SZ_R {
    IROM_SZ_R::new(((self.bits >> 16) & 0x0f) as u8)
  }
  #[doc = "Bits 24:27 - Instruction RAM Size"]
  #[inline(always)]
  pub fn iram_sz(&self) -> IRAM_SZ_R {
    IRAM_SZ_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
}
