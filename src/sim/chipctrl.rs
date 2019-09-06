#[doc = "Reader of register CHIPCTRL"]
pub type R = crate::R<u32, super::CHIPCTRL>;
#[doc = "Writer for register CHIPCTRL"]
pub type W = crate::W<u32, super::CHIPCTRL>;
#[doc = "Register CHIPCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CHIPCTRL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "FLEXBUS security level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBSL_A {
  #[doc = "0: All off-chip access(instruction and data) via the Flexbus or sdram are disallowed"]
  FBSL_0,
  #[doc = "1: All off-chip access(instruction and data) via the Flexbus or sdram are disallowed"]
  FBSL_1,
  #[doc = "2: off-chip instruction access are disallowed, data access are allowed"]
  FBSL_2,
  #[doc = "3: off-chip instruction access and data access are allowed"]
  FBSL_3,
}
impl From<FBSL_A> for u8 {
  #[inline(always)]
  fn from(variant: FBSL_A) -> Self {
    match variant {
      FBSL_A::FBSL_0 => 0,
      FBSL_A::FBSL_1 => 1,
      FBSL_A::FBSL_2 => 2,
      FBSL_A::FBSL_3 => 3,
    }
  }
}
#[doc = "Reader of field `FBSL`"]
pub type FBSL_R = crate::R<u8, FBSL_A>;
impl FBSL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FBSL_A {
    match self.bits {
      0 => FBSL_A::FBSL_0,
      1 => FBSL_A::FBSL_1,
      2 => FBSL_A::FBSL_2,
      3 => FBSL_A::FBSL_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `FBSL_0`"]
  #[inline(always)]
  pub fn is_fbsl_0(&self) -> bool {
    *self == FBSL_A::FBSL_0
  }
  #[doc = "Checks if the value of the field is `FBSL_1`"]
  #[inline(always)]
  pub fn is_fbsl_1(&self) -> bool {
    *self == FBSL_A::FBSL_1
  }
  #[doc = "Checks if the value of the field is `FBSL_2`"]
  #[inline(always)]
  pub fn is_fbsl_2(&self) -> bool {
    *self == FBSL_A::FBSL_2
  }
  #[doc = "Checks if the value of the field is `FBSL_3`"]
  #[inline(always)]
  pub fn is_fbsl_3(&self) -> bool {
    *self == FBSL_A::FBSL_3
  }
}
#[doc = "Write proxy for field `FBSL`"]
pub struct FBSL_W<'a> {
  w: &'a mut W,
}
impl<'a> FBSL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FBSL_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "All off-chip access(instruction and data) via the Flexbus or sdram are disallowed"]
  #[inline(always)]
  pub fn fbsl_0(self) -> &'a mut W {
    self.variant(FBSL_A::FBSL_0)
  }
  #[doc = "All off-chip access(instruction and data) via the Flexbus or sdram are disallowed"]
  #[inline(always)]
  pub fn fbsl_1(self) -> &'a mut W {
    self.variant(FBSL_A::FBSL_1)
  }
  #[doc = "off-chip instruction access are disallowed, data access are allowed"]
  #[inline(always)]
  pub fn fbsl_2(self) -> &'a mut W {
    self.variant(FBSL_A::FBSL_2)
  }
  #[doc = "off-chip instruction access and data access are allowed"]
  #[inline(always)]
  pub fn fbsl_3(self) -> &'a mut W {
    self.variant(FBSL_A::FBSL_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
    self.w
  }
}
impl R {
  #[doc = "Bits 8:9 - FLEXBUS security level"]
  #[inline(always)]
  pub fn fbsl(&self) -> FBSL_R {
    FBSL_R::new(((self.bits >> 8) & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bits 8:9 - FLEXBUS security level"]
  #[inline(always)]
  pub fn fbsl(&mut self) -> FBSL_W {
    FBSL_W { w: self }
  }
}
