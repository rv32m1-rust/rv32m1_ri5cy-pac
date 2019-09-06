#[doc = "Reader of register GLOBAL"]
pub type R = crate::R<u32, super::GLOBAL>;
#[doc = "Writer for register GLOBAL"]
pub type W = crate::W<u32, super::GLOBAL>;
#[doc = "Register GLOBAL `reset()`'s with value 0"]
impl crate::ResetValue for super::GLOBAL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "No Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOUPDATE_A {
  #[doc = "0: Internal double buffered registers update as normal."]
  NOUPDATE_0,
  #[doc = "1: Internal double buffered registers do not update."]
  NOUPDATE_1,
}
impl From<NOUPDATE_A> for bool {
  #[inline(always)]
  fn from(variant: NOUPDATE_A) -> Self {
    match variant {
      NOUPDATE_A::NOUPDATE_0 => false,
      NOUPDATE_A::NOUPDATE_1 => true,
    }
  }
}
#[doc = "Reader of field `NOUPDATE`"]
pub type NOUPDATE_R = crate::R<bool, NOUPDATE_A>;
impl NOUPDATE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> NOUPDATE_A {
    match self.bits {
      false => NOUPDATE_A::NOUPDATE_0,
      true => NOUPDATE_A::NOUPDATE_1,
    }
  }
  #[doc = "Checks if the value of the field is `NOUPDATE_0`"]
  #[inline(always)]
  pub fn is_noupdate_0(&self) -> bool {
    *self == NOUPDATE_A::NOUPDATE_0
  }
  #[doc = "Checks if the value of the field is `NOUPDATE_1`"]
  #[inline(always)]
  pub fn is_noupdate_1(&self) -> bool {
    *self == NOUPDATE_A::NOUPDATE_1
  }
}
#[doc = "Write proxy for field `NOUPDATE`"]
pub struct NOUPDATE_W<'a> {
  w: &'a mut W,
}
impl<'a> NOUPDATE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: NOUPDATE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Internal double buffered registers update as normal."]
  #[inline(always)]
  pub fn noupdate_0(self) -> &'a mut W {
    self.variant(NOUPDATE_A::NOUPDATE_0)
  }
  #[doc = "Internal double buffered registers do not update."]
  #[inline(always)]
  pub fn noupdate_1(self) -> &'a mut W {
    self.variant(NOUPDATE_A::NOUPDATE_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
    self.w
  }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_A {
  #[doc = "0: Module is not reset."]
  RST_0,
  #[doc = "1: Module is reset."]
  RST_1,
}
impl From<RST_A> for bool {
  #[inline(always)]
  fn from(variant: RST_A) -> Self {
    match variant {
      RST_A::RST_0 => false,
      RST_A::RST_1 => true,
    }
  }
}
#[doc = "Reader of field `RST`"]
pub type RST_R = crate::R<bool, RST_A>;
impl RST_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RST_A {
    match self.bits {
      false => RST_A::RST_0,
      true => RST_A::RST_1,
    }
  }
  #[doc = "Checks if the value of the field is `RST_0`"]
  #[inline(always)]
  pub fn is_rst_0(&self) -> bool {
    *self == RST_A::RST_0
  }
  #[doc = "Checks if the value of the field is `RST_1`"]
  #[inline(always)]
  pub fn is_rst_1(&self) -> bool {
    *self == RST_A::RST_1
  }
}
#[doc = "Write proxy for field `RST`"]
pub struct RST_W<'a> {
  w: &'a mut W,
}
impl<'a> RST_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RST_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Module is not reset."]
  #[inline(always)]
  pub fn rst_0(self) -> &'a mut W {
    self.variant(RST_A::RST_0)
  }
  #[doc = "Module is reset."]
  #[inline(always)]
  pub fn rst_1(self) -> &'a mut W {
    self.variant(RST_A::RST_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - No Update"]
  #[inline(always)]
  pub fn noupdate(&self) -> NOUPDATE_R {
    NOUPDATE_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Software Reset"]
  #[inline(always)]
  pub fn rst(&self) -> RST_R {
    RST_R::new(((self.bits >> 1) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - No Update"]
  #[inline(always)]
  pub fn noupdate(&mut self) -> NOUPDATE_W {
    NOUPDATE_W { w: self }
  }
  #[doc = "Bit 1 - Software Reset"]
  #[inline(always)]
  pub fn rst(&mut self) -> RST_W {
    RST_W { w: self }
  }
}
