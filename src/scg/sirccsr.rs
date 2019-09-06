#[doc = "Reader of register SIRCCSR"]
pub type R = crate::R<u32, super::SIRCCSR>;
#[doc = "Writer for register SIRCCSR"]
pub type W = crate::W<u32, super::SIRCCSR>;
#[doc = "Register SIRCCSR `reset()`'s with value 0x05"]
impl crate::ResetValue for super::SIRCCSR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x05
  }
}
#[doc = "Slow IRC Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCEN_A {
  #[doc = "0: Slow IRC is disabled"]
  SIRCEN_0,
  #[doc = "1: Slow IRC is enabled"]
  SIRCEN_1,
}
impl From<SIRCEN_A> for bool {
  #[inline(always)]
  fn from(variant: SIRCEN_A) -> Self {
    match variant {
      SIRCEN_A::SIRCEN_0 => false,
      SIRCEN_A::SIRCEN_1 => true,
    }
  }
}
#[doc = "Reader of field `SIRCEN`"]
pub type SIRCEN_R = crate::R<bool, SIRCEN_A>;
impl SIRCEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SIRCEN_A {
    match self.bits {
      false => SIRCEN_A::SIRCEN_0,
      true => SIRCEN_A::SIRCEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `SIRCEN_0`"]
  #[inline(always)]
  pub fn is_sircen_0(&self) -> bool {
    *self == SIRCEN_A::SIRCEN_0
  }
  #[doc = "Checks if the value of the field is `SIRCEN_1`"]
  #[inline(always)]
  pub fn is_sircen_1(&self) -> bool {
    *self == SIRCEN_A::SIRCEN_1
  }
}
#[doc = "Write proxy for field `SIRCEN`"]
pub struct SIRCEN_W<'a> {
  w: &'a mut W,
}
impl<'a> SIRCEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SIRCEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Slow IRC is disabled"]
  #[inline(always)]
  pub fn sircen_0(self) -> &'a mut W {
    self.variant(SIRCEN_A::SIRCEN_0)
  }
  #[doc = "Slow IRC is enabled"]
  #[inline(always)]
  pub fn sircen_1(self) -> &'a mut W {
    self.variant(SIRCEN_A::SIRCEN_1)
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
#[doc = "Slow IRC Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCSTEN_A {
  #[doc = "0: Slow IRC is disabled in Stop modes"]
  SIRCSTEN_0,
  #[doc = "1: Slow IRC is enabled in Stop modes"]
  SIRCSTEN_1,
}
impl From<SIRCSTEN_A> for bool {
  #[inline(always)]
  fn from(variant: SIRCSTEN_A) -> Self {
    match variant {
      SIRCSTEN_A::SIRCSTEN_0 => false,
      SIRCSTEN_A::SIRCSTEN_1 => true,
    }
  }
}
#[doc = "Reader of field `SIRCSTEN`"]
pub type SIRCSTEN_R = crate::R<bool, SIRCSTEN_A>;
impl SIRCSTEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SIRCSTEN_A {
    match self.bits {
      false => SIRCSTEN_A::SIRCSTEN_0,
      true => SIRCSTEN_A::SIRCSTEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `SIRCSTEN_0`"]
  #[inline(always)]
  pub fn is_sircsten_0(&self) -> bool {
    *self == SIRCSTEN_A::SIRCSTEN_0
  }
  #[doc = "Checks if the value of the field is `SIRCSTEN_1`"]
  #[inline(always)]
  pub fn is_sircsten_1(&self) -> bool {
    *self == SIRCSTEN_A::SIRCSTEN_1
  }
}
#[doc = "Write proxy for field `SIRCSTEN`"]
pub struct SIRCSTEN_W<'a> {
  w: &'a mut W,
}
impl<'a> SIRCSTEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SIRCSTEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Slow IRC is disabled in Stop modes"]
  #[inline(always)]
  pub fn sircsten_0(self) -> &'a mut W {
    self.variant(SIRCSTEN_A::SIRCSTEN_0)
  }
  #[doc = "Slow IRC is enabled in Stop modes"]
  #[inline(always)]
  pub fn sircsten_1(self) -> &'a mut W {
    self.variant(SIRCSTEN_A::SIRCSTEN_1)
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
#[doc = "Slow IRC Low Power Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCLPEN_A {
  #[doc = "0: Slow IRC is disabled in VLP modes"]
  SIRCLPEN_0,
  #[doc = "1: Slow IRC is enabled in VLP modes"]
  SIRCLPEN_1,
}
impl From<SIRCLPEN_A> for bool {
  #[inline(always)]
  fn from(variant: SIRCLPEN_A) -> Self {
    match variant {
      SIRCLPEN_A::SIRCLPEN_0 => false,
      SIRCLPEN_A::SIRCLPEN_1 => true,
    }
  }
}
#[doc = "Reader of field `SIRCLPEN`"]
pub type SIRCLPEN_R = crate::R<bool, SIRCLPEN_A>;
impl SIRCLPEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SIRCLPEN_A {
    match self.bits {
      false => SIRCLPEN_A::SIRCLPEN_0,
      true => SIRCLPEN_A::SIRCLPEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `SIRCLPEN_0`"]
  #[inline(always)]
  pub fn is_sirclpen_0(&self) -> bool {
    *self == SIRCLPEN_A::SIRCLPEN_0
  }
  #[doc = "Checks if the value of the field is `SIRCLPEN_1`"]
  #[inline(always)]
  pub fn is_sirclpen_1(&self) -> bool {
    *self == SIRCLPEN_A::SIRCLPEN_1
  }
}
#[doc = "Write proxy for field `SIRCLPEN`"]
pub struct SIRCLPEN_W<'a> {
  w: &'a mut W,
}
impl<'a> SIRCLPEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SIRCLPEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Slow IRC is disabled in VLP modes"]
  #[inline(always)]
  pub fn sirclpen_0(self) -> &'a mut W {
    self.variant(SIRCLPEN_A::SIRCLPEN_0)
  }
  #[doc = "Slow IRC is enabled in VLP modes"]
  #[inline(always)]
  pub fn sirclpen_1(self) -> &'a mut W {
    self.variant(SIRCLPEN_A::SIRCLPEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
    self.w
  }
}
#[doc = "Lock Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK_A {
  #[doc = "0: Control Status Register can be written."]
  LK_0,
  #[doc = "1: Control Status Register cannot be written."]
  LK_1,
}
impl From<LK_A> for bool {
  #[inline(always)]
  fn from(variant: LK_A) -> Self {
    match variant {
      LK_A::LK_0 => false,
      LK_A::LK_1 => true,
    }
  }
}
#[doc = "Reader of field `LK`"]
pub type LK_R = crate::R<bool, LK_A>;
impl LK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LK_A {
    match self.bits {
      false => LK_A::LK_0,
      true => LK_A::LK_1,
    }
  }
  #[doc = "Checks if the value of the field is `LK_0`"]
  #[inline(always)]
  pub fn is_lk_0(&self) -> bool {
    *self == LK_A::LK_0
  }
  #[doc = "Checks if the value of the field is `LK_1`"]
  #[inline(always)]
  pub fn is_lk_1(&self) -> bool {
    *self == LK_A::LK_1
  }
}
#[doc = "Write proxy for field `LK`"]
pub struct LK_W<'a> {
  w: &'a mut W,
}
impl<'a> LK_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LK_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Control Status Register can be written."]
  #[inline(always)]
  pub fn lk_0(self) -> &'a mut W {
    self.variant(LK_A::LK_0)
  }
  #[doc = "Control Status Register cannot be written."]
  #[inline(always)]
  pub fn lk_1(self) -> &'a mut W {
    self.variant(LK_A::LK_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
    self.w
  }
}
#[doc = "Slow IRC Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCVLD_A {
  #[doc = "0: Slow IRC is not enabled or clock is not valid"]
  SIRCVLD_0,
  #[doc = "1: Slow IRC is enabled and output clock is valid"]
  SIRCVLD_1,
}
impl From<SIRCVLD_A> for bool {
  #[inline(always)]
  fn from(variant: SIRCVLD_A) -> Self {
    match variant {
      SIRCVLD_A::SIRCVLD_0 => false,
      SIRCVLD_A::SIRCVLD_1 => true,
    }
  }
}
#[doc = "Reader of field `SIRCVLD`"]
pub type SIRCVLD_R = crate::R<bool, SIRCVLD_A>;
impl SIRCVLD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SIRCVLD_A {
    match self.bits {
      false => SIRCVLD_A::SIRCVLD_0,
      true => SIRCVLD_A::SIRCVLD_1,
    }
  }
  #[doc = "Checks if the value of the field is `SIRCVLD_0`"]
  #[inline(always)]
  pub fn is_sircvld_0(&self) -> bool {
    *self == SIRCVLD_A::SIRCVLD_0
  }
  #[doc = "Checks if the value of the field is `SIRCVLD_1`"]
  #[inline(always)]
  pub fn is_sircvld_1(&self) -> bool {
    *self == SIRCVLD_A::SIRCVLD_1
  }
}
#[doc = "Slow IRC Selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCSEL_A {
  #[doc = "0: Slow IRC is not the system clock source"]
  SIRCSEL_0,
  #[doc = "1: Slow IRC is the system clock source"]
  SIRCSEL_1,
}
impl From<SIRCSEL_A> for bool {
  #[inline(always)]
  fn from(variant: SIRCSEL_A) -> Self {
    match variant {
      SIRCSEL_A::SIRCSEL_0 => false,
      SIRCSEL_A::SIRCSEL_1 => true,
    }
  }
}
#[doc = "Reader of field `SIRCSEL`"]
pub type SIRCSEL_R = crate::R<bool, SIRCSEL_A>;
impl SIRCSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SIRCSEL_A {
    match self.bits {
      false => SIRCSEL_A::SIRCSEL_0,
      true => SIRCSEL_A::SIRCSEL_1,
    }
  }
  #[doc = "Checks if the value of the field is `SIRCSEL_0`"]
  #[inline(always)]
  pub fn is_sircsel_0(&self) -> bool {
    *self == SIRCSEL_A::SIRCSEL_0
  }
  #[doc = "Checks if the value of the field is `SIRCSEL_1`"]
  #[inline(always)]
  pub fn is_sircsel_1(&self) -> bool {
    *self == SIRCSEL_A::SIRCSEL_1
  }
}
impl R {
  #[doc = "Bit 0 - Slow IRC Enable"]
  #[inline(always)]
  pub fn sircen(&self) -> SIRCEN_R {
    SIRCEN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Slow IRC Stop Enable"]
  #[inline(always)]
  pub fn sircsten(&self) -> SIRCSTEN_R {
    SIRCSTEN_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Slow IRC Low Power Enable"]
  #[inline(always)]
  pub fn sirclpen(&self) -> SIRCLPEN_R {
    SIRCLPEN_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 23 - Lock Register"]
  #[inline(always)]
  pub fn lk(&self) -> LK_R {
    LK_R::new(((self.bits >> 23) & 0x01) != 0)
  }
  #[doc = "Bit 24 - Slow IRC Valid"]
  #[inline(always)]
  pub fn sircvld(&self) -> SIRCVLD_R {
    SIRCVLD_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - Slow IRC Selected"]
  #[inline(always)]
  pub fn sircsel(&self) -> SIRCSEL_R {
    SIRCSEL_R::new(((self.bits >> 25) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Slow IRC Enable"]
  #[inline(always)]
  pub fn sircen(&mut self) -> SIRCEN_W {
    SIRCEN_W { w: self }
  }
  #[doc = "Bit 1 - Slow IRC Stop Enable"]
  #[inline(always)]
  pub fn sircsten(&mut self) -> SIRCSTEN_W {
    SIRCSTEN_W { w: self }
  }
  #[doc = "Bit 2 - Slow IRC Low Power Enable"]
  #[inline(always)]
  pub fn sirclpen(&mut self) -> SIRCLPEN_W {
    SIRCLPEN_W { w: self }
  }
  #[doc = "Bit 23 - Lock Register"]
  #[inline(always)]
  pub fn lk(&mut self) -> LK_W {
    LK_W { w: self }
  }
}
