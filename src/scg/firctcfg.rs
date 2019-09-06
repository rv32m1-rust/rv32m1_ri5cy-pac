#[doc = "Reader of register FIRCTCFG"]
pub type R = crate::R<u32, super::FIRCTCFG>;
#[doc = "Writer for register FIRCTCFG"]
pub type W = crate::W<u32, super::FIRCTCFG>;
#[doc = "Register FIRCTCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FIRCTCFG {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Trim Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIMSRC_A {
  #[doc = "2: no description available"]
  TRIMSRC_2,
  #[doc = "3: no description available"]
  TRIMSRC_3,
}
impl From<TRIMSRC_A> for u8 {
  #[inline(always)]
  fn from(variant: TRIMSRC_A) -> Self {
    match variant {
      TRIMSRC_A::TRIMSRC_2 => 2,
      TRIMSRC_A::TRIMSRC_3 => 3,
    }
  }
}
#[doc = "Reader of field `TRIMSRC`"]
pub type TRIMSRC_R = crate::R<u8, TRIMSRC_A>;
impl TRIMSRC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, TRIMSRC_A> {
    use crate::Variant::*;
    match self.bits {
      2 => Val(TRIMSRC_A::TRIMSRC_2),
      3 => Val(TRIMSRC_A::TRIMSRC_3),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `TRIMSRC_2`"]
  #[inline(always)]
  pub fn is_trimsrc_2(&self) -> bool {
    *self == TRIMSRC_A::TRIMSRC_2
  }
  #[doc = "Checks if the value of the field is `TRIMSRC_3`"]
  #[inline(always)]
  pub fn is_trimsrc_3(&self) -> bool {
    *self == TRIMSRC_A::TRIMSRC_3
  }
}
#[doc = "Write proxy for field `TRIMSRC`"]
pub struct TRIMSRC_W<'a> {
  w: &'a mut W,
}
impl<'a> TRIMSRC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRIMSRC_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn trimsrc_2(self) -> &'a mut W {
    self.variant(TRIMSRC_A::TRIMSRC_2)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn trimsrc_3(self) -> &'a mut W {
    self.variant(TRIMSRC_A::TRIMSRC_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
    self.w
  }
}
#[doc = "Fast IRC Trim Predivide\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIMDIV_A {
  #[doc = "0: Divide by 1"]
  TRIMDIV_0,
  #[doc = "1: Divide by 128"]
  TRIMDIV_1,
  #[doc = "2: Divide by 256"]
  TRIMDIV_2,
  #[doc = "3: Divide by 512"]
  TRIMDIV_3,
  #[doc = "4: Divide by 1024"]
  TRIMDIV_4,
  #[doc = "5: Divide by 2048"]
  TRIMDIV_5,
}
impl From<TRIMDIV_A> for u8 {
  #[inline(always)]
  fn from(variant: TRIMDIV_A) -> Self {
    match variant {
      TRIMDIV_A::TRIMDIV_0 => 0,
      TRIMDIV_A::TRIMDIV_1 => 1,
      TRIMDIV_A::TRIMDIV_2 => 2,
      TRIMDIV_A::TRIMDIV_3 => 3,
      TRIMDIV_A::TRIMDIV_4 => 4,
      TRIMDIV_A::TRIMDIV_5 => 5,
    }
  }
}
#[doc = "Reader of field `TRIMDIV`"]
pub type TRIMDIV_R = crate::R<u8, TRIMDIV_A>;
impl TRIMDIV_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, TRIMDIV_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(TRIMDIV_A::TRIMDIV_0),
      1 => Val(TRIMDIV_A::TRIMDIV_1),
      2 => Val(TRIMDIV_A::TRIMDIV_2),
      3 => Val(TRIMDIV_A::TRIMDIV_3),
      4 => Val(TRIMDIV_A::TRIMDIV_4),
      5 => Val(TRIMDIV_A::TRIMDIV_5),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `TRIMDIV_0`"]
  #[inline(always)]
  pub fn is_trimdiv_0(&self) -> bool {
    *self == TRIMDIV_A::TRIMDIV_0
  }
  #[doc = "Checks if the value of the field is `TRIMDIV_1`"]
  #[inline(always)]
  pub fn is_trimdiv_1(&self) -> bool {
    *self == TRIMDIV_A::TRIMDIV_1
  }
  #[doc = "Checks if the value of the field is `TRIMDIV_2`"]
  #[inline(always)]
  pub fn is_trimdiv_2(&self) -> bool {
    *self == TRIMDIV_A::TRIMDIV_2
  }
  #[doc = "Checks if the value of the field is `TRIMDIV_3`"]
  #[inline(always)]
  pub fn is_trimdiv_3(&self) -> bool {
    *self == TRIMDIV_A::TRIMDIV_3
  }
  #[doc = "Checks if the value of the field is `TRIMDIV_4`"]
  #[inline(always)]
  pub fn is_trimdiv_4(&self) -> bool {
    *self == TRIMDIV_A::TRIMDIV_4
  }
  #[doc = "Checks if the value of the field is `TRIMDIV_5`"]
  #[inline(always)]
  pub fn is_trimdiv_5(&self) -> bool {
    *self == TRIMDIV_A::TRIMDIV_5
  }
}
#[doc = "Write proxy for field `TRIMDIV`"]
pub struct TRIMDIV_W<'a> {
  w: &'a mut W,
}
impl<'a> TRIMDIV_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRIMDIV_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Divide by 1"]
  #[inline(always)]
  pub fn trimdiv_0(self) -> &'a mut W {
    self.variant(TRIMDIV_A::TRIMDIV_0)
  }
  #[doc = "Divide by 128"]
  #[inline(always)]
  pub fn trimdiv_1(self) -> &'a mut W {
    self.variant(TRIMDIV_A::TRIMDIV_1)
  }
  #[doc = "Divide by 256"]
  #[inline(always)]
  pub fn trimdiv_2(self) -> &'a mut W {
    self.variant(TRIMDIV_A::TRIMDIV_2)
  }
  #[doc = "Divide by 512"]
  #[inline(always)]
  pub fn trimdiv_3(self) -> &'a mut W {
    self.variant(TRIMDIV_A::TRIMDIV_3)
  }
  #[doc = "Divide by 1024"]
  #[inline(always)]
  pub fn trimdiv_4(self) -> &'a mut W {
    self.variant(TRIMDIV_A::TRIMDIV_4)
  }
  #[doc = "Divide by 2048"]
  #[inline(always)]
  pub fn trimdiv_5(self) -> &'a mut W {
    self.variant(TRIMDIV_A::TRIMDIV_5)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:1 - Trim Source"]
  #[inline(always)]
  pub fn trimsrc(&self) -> TRIMSRC_R {
    TRIMSRC_R::new((self.bits & 0x03) as u8)
  }
  #[doc = "Bits 8:10 - Fast IRC Trim Predivide"]
  #[inline(always)]
  pub fn trimdiv(&self) -> TRIMDIV_R {
    TRIMDIV_R::new(((self.bits >> 8) & 0x07) as u8)
  }
}
impl W {
  #[doc = "Bits 0:1 - Trim Source"]
  #[inline(always)]
  pub fn trimsrc(&mut self) -> TRIMSRC_W {
    TRIMSRC_W { w: self }
  }
  #[doc = "Bits 8:10 - Fast IRC Trim Predivide"]
  #[inline(always)]
  pub fn trimdiv(&mut self) -> TRIMDIV_W {
    TRIMDIV_W { w: self }
  }
}
