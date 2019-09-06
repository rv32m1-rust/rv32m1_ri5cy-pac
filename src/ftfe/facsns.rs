#[doc = "Reader of register FACSNS"]
pub type R = crate::R<u8, super::FACSNS>;
#[doc = "Number of Segments Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUMSG_S_A {
  #[doc = "16: Secondary Program flash memory is divided into 16 segments"]
  NUMSG_S_16,
}
impl From<NUMSG_S_A> for u8 {
  #[inline(always)]
  fn from(variant: NUMSG_S_A) -> Self {
    match variant {
      NUMSG_S_A::NUMSG_S_16 => 16,
    }
  }
}
#[doc = "Reader of field `NUMSG_S`"]
pub type NUMSG_S_R = crate::R<u8, NUMSG_S_A>;
impl NUMSG_S_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, NUMSG_S_A> {
    use crate::Variant::*;
    match self.bits {
      16 => Val(NUMSG_S_A::NUMSG_S_16),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `NUMSG_S_16`"]
  #[inline(always)]
  pub fn is_numsg_s_16(&self) -> bool {
    *self == NUMSG_S_A::NUMSG_S_16
  }
}
impl R {
  #[doc = "Bits 0:7 - Number of Segments Indicator"]
  #[inline(always)]
  pub fn numsg_s(&self) -> NUMSG_S_R {
    NUMSG_S_R::new((self.bits & 0xff) as u8)
  }
}
