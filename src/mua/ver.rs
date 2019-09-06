#[doc = "Reader of register VER"]
pub type R = crate::R<u32, super::VER>;
#[doc = "Feature Specification Number\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEATURE_A {
  #[doc = "0: Standard features implemented"]
  FEATURE_0,
  #[doc = "32768: Core Control and Status Registers are implemented in both MUA and MUB."]
  FEATURE_4,
}
impl From<FEATURE_A> for u16 {
  #[inline(always)]
  fn from(variant: FEATURE_A) -> Self {
    match variant {
      FEATURE_A::FEATURE_0 => 0,
      FEATURE_A::FEATURE_4 => 32768,
    }
  }
}
#[doc = "Reader of field `FEATURE`"]
pub type FEATURE_R = crate::R<u16, FEATURE_A>;
impl FEATURE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u16, FEATURE_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(FEATURE_A::FEATURE_0),
      32768 => Val(FEATURE_A::FEATURE_4),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `FEATURE_0`"]
  #[inline(always)]
  pub fn is_feature_0(&self) -> bool {
    *self == FEATURE_A::FEATURE_0
  }
  #[doc = "Checks if the value of the field is `FEATURE_4`"]
  #[inline(always)]
  pub fn is_feature_4(&self) -> bool {
    *self == FEATURE_A::FEATURE_4
  }
}
#[doc = "Reader of field `MINOR`"]
pub type MINOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAJOR`"]
pub type MAJOR_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:15 - Feature Specification Number"]
  #[inline(always)]
  pub fn feature(&self) -> FEATURE_R {
    FEATURE_R::new((self.bits & 0xffff) as u16)
  }
  #[doc = "Bits 16:23 - Minor Version Number"]
  #[inline(always)]
  pub fn minor(&self) -> MINOR_R {
    MINOR_R::new(((self.bits >> 16) & 0xff) as u8)
  }
  #[doc = "Bits 24:31 - Major Version Number"]
  #[inline(always)]
  pub fn major(&self) -> MAJOR_R {
    MAJOR_R::new(((self.bits >> 24) & 0xff) as u8)
  }
}
