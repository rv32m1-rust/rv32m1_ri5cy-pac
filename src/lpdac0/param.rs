#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "FIFO size\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOSZ_A {
  #[doc = "1: FIFO depth is 4"]
  FIFOSZ_1,
  #[doc = "2: FIFO depth is 8"]
  FIFOSZ_2,
  #[doc = "3: FIFO depth is 16"]
  FIFOSZ_3,
  #[doc = "4: FIFO depth is 32"]
  FIFOSZ_4,
  #[doc = "5: FIFO depth is 64"]
  FIFOSZ_5,
  #[doc = "6: FIFO depth is 128"]
  FIFOSZ_6,
  #[doc = "7: FIFO depth is 256"]
  FIFOSZ_7,
}
impl From<FIFOSZ_A> for u8 {
  #[inline(always)]
  fn from(variant: FIFOSZ_A) -> Self {
    match variant {
      FIFOSZ_A::FIFOSZ_1 => 1,
      FIFOSZ_A::FIFOSZ_2 => 2,
      FIFOSZ_A::FIFOSZ_3 => 3,
      FIFOSZ_A::FIFOSZ_4 => 4,
      FIFOSZ_A::FIFOSZ_5 => 5,
      FIFOSZ_A::FIFOSZ_6 => 6,
      FIFOSZ_A::FIFOSZ_7 => 7,
    }
  }
}
#[doc = "Reader of field `FIFOSZ`"]
pub type FIFOSZ_R = crate::R<u8, FIFOSZ_A>;
impl FIFOSZ_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, FIFOSZ_A> {
    use crate::Variant::*;
    match self.bits {
      1 => Val(FIFOSZ_A::FIFOSZ_1),
      2 => Val(FIFOSZ_A::FIFOSZ_2),
      3 => Val(FIFOSZ_A::FIFOSZ_3),
      4 => Val(FIFOSZ_A::FIFOSZ_4),
      5 => Val(FIFOSZ_A::FIFOSZ_5),
      6 => Val(FIFOSZ_A::FIFOSZ_6),
      7 => Val(FIFOSZ_A::FIFOSZ_7),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `FIFOSZ_1`"]
  #[inline(always)]
  pub fn is_fifosz_1(&self) -> bool {
    *self == FIFOSZ_A::FIFOSZ_1
  }
  #[doc = "Checks if the value of the field is `FIFOSZ_2`"]
  #[inline(always)]
  pub fn is_fifosz_2(&self) -> bool {
    *self == FIFOSZ_A::FIFOSZ_2
  }
  #[doc = "Checks if the value of the field is `FIFOSZ_3`"]
  #[inline(always)]
  pub fn is_fifosz_3(&self) -> bool {
    *self == FIFOSZ_A::FIFOSZ_3
  }
  #[doc = "Checks if the value of the field is `FIFOSZ_4`"]
  #[inline(always)]
  pub fn is_fifosz_4(&self) -> bool {
    *self == FIFOSZ_A::FIFOSZ_4
  }
  #[doc = "Checks if the value of the field is `FIFOSZ_5`"]
  #[inline(always)]
  pub fn is_fifosz_5(&self) -> bool {
    *self == FIFOSZ_A::FIFOSZ_5
  }
  #[doc = "Checks if the value of the field is `FIFOSZ_6`"]
  #[inline(always)]
  pub fn is_fifosz_6(&self) -> bool {
    *self == FIFOSZ_A::FIFOSZ_6
  }
  #[doc = "Checks if the value of the field is `FIFOSZ_7`"]
  #[inline(always)]
  pub fn is_fifosz_7(&self) -> bool {
    *self == FIFOSZ_A::FIFOSZ_7
  }
}
impl R {
  #[doc = "Bits 0:2 - FIFO size"]
  #[inline(always)]
  pub fn fifosz(&self) -> FIFOSZ_R {
    FIFOSZ_R::new((self.bits & 0x07) as u8)
  }
}
