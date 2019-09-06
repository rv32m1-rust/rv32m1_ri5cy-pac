#[doc = "Reader of register SDID"]
pub type R = crate::R<u32, super::SDID>;
#[doc = "PINID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINID_A {
  #[doc = "8: 176-pin"]
  PINID_8,
  #[doc = "13: 191-pin"]
  PINID_13,
}
impl From<PINID_A> for u8 {
  #[inline(always)]
  fn from(variant: PINID_A) -> Self {
    match variant {
      PINID_A::PINID_8 => 8,
      PINID_A::PINID_13 => 13,
    }
  }
}
#[doc = "Reader of field `PINID`"]
pub type PINID_R = crate::R<u8, PINID_A>;
impl PINID_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, PINID_A> {
    use crate::Variant::*;
    match self.bits {
      8 => Val(PINID_A::PINID_8),
      13 => Val(PINID_A::PINID_13),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `PINID_8`"]
  #[inline(always)]
  pub fn is_pinid_8(&self) -> bool {
    *self == PINID_A::PINID_8
  }
  #[doc = "Checks if the value of the field is `PINID_13`"]
  #[inline(always)]
  pub fn is_pinid_13(&self) -> bool {
    *self == PINID_A::PINID_13
  }
}
#[doc = "Reader of field `DIEID`"]
pub type DIEID_R = crate::R<u8, u8>;
#[doc = "Reader of field `REVID`"]
pub type REVID_R = crate::R<u8, u8>;
#[doc = "Reader of field `SERIESID`"]
pub type SERIESID_R = crate::R<u8, u8>;
#[doc = "SUBFAMID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUBFAMID_A {
  #[doc = "2: 02"]
  SUBFAMID_2,
  #[doc = "3: 03"]
  SUBFAMID_3,
  #[doc = "4: 04"]
  SUBFAMID_4,
}
impl From<SUBFAMID_A> for u8 {
  #[inline(always)]
  fn from(variant: SUBFAMID_A) -> Self {
    match variant {
      SUBFAMID_A::SUBFAMID_2 => 2,
      SUBFAMID_A::SUBFAMID_3 => 3,
      SUBFAMID_A::SUBFAMID_4 => 4,
    }
  }
}
#[doc = "Reader of field `SUBFAMID`"]
pub type SUBFAMID_R = crate::R<u8, SUBFAMID_A>;
impl SUBFAMID_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, SUBFAMID_A> {
    use crate::Variant::*;
    match self.bits {
      2 => Val(SUBFAMID_A::SUBFAMID_2),
      3 => Val(SUBFAMID_A::SUBFAMID_3),
      4 => Val(SUBFAMID_A::SUBFAMID_4),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `SUBFAMID_2`"]
  #[inline(always)]
  pub fn is_subfamid_2(&self) -> bool {
    *self == SUBFAMID_A::SUBFAMID_2
  }
  #[doc = "Checks if the value of the field is `SUBFAMID_3`"]
  #[inline(always)]
  pub fn is_subfamid_3(&self) -> bool {
    *self == SUBFAMID_A::SUBFAMID_3
  }
  #[doc = "Checks if the value of the field is `SUBFAMID_4`"]
  #[inline(always)]
  pub fn is_subfamid_4(&self) -> bool {
    *self == SUBFAMID_A::SUBFAMID_4
  }
}
#[doc = "FAMID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAMID_A {
  #[doc = "0: RV32M1"]
  FAMID_0,
}
impl From<FAMID_A> for u8 {
  #[inline(always)]
  fn from(variant: FAMID_A) -> Self {
    match variant {
      FAMID_A::FAMID_0 => 0,
    }
  }
}
#[doc = "Reader of field `FAMID`"]
pub type FAMID_R = crate::R<u8, FAMID_A>;
impl FAMID_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, FAMID_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(FAMID_A::FAMID_0),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `FAMID_0`"]
  #[inline(always)]
  pub fn is_famid_0(&self) -> bool {
    *self == FAMID_A::FAMID_0
  }
}
impl R {
  #[doc = "Bits 0:3 - PINID"]
  #[inline(always)]
  pub fn pinid(&self) -> PINID_R {
    PINID_R::new((self.bits & 0x0f) as u8)
  }
  #[doc = "Bits 7:11 - DIEID"]
  #[inline(always)]
  pub fn dieid(&self) -> DIEID_R {
    DIEID_R::new(((self.bits >> 7) & 0x1f) as u8)
  }
  #[doc = "Bits 12:15 - REVID"]
  #[inline(always)]
  pub fn revid(&self) -> REVID_R {
    REVID_R::new(((self.bits >> 12) & 0x0f) as u8)
  }
  #[doc = "Bits 20:23 - SERIESID"]
  #[inline(always)]
  pub fn seriesid(&self) -> SERIESID_R {
    SERIESID_R::new(((self.bits >> 20) & 0x0f) as u8)
  }
  #[doc = "Bits 24:27 - SUBFAMID"]
  #[inline(always)]
  pub fn subfamid(&self) -> SUBFAMID_R {
    SUBFAMID_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
  #[doc = "Bits 28:31 - FAMID"]
  #[inline(always)]
  pub fn famid(&self) -> FAMID_R {
    FAMID_R::new(((self.bits >> 28) & 0x0f) as u8)
  }
}
