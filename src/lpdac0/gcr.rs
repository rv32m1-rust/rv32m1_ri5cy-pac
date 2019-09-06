#[doc = "Reader of register GCR"]
pub type R = crate::R<u32, super::GCR>;
#[doc = "Writer for register GCR"]
pub type W = crate::W<u32, super::GCR>;
#[doc = "Register GCR `reset()`'s with value 0"]
impl crate::ResetValue for super::GCR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "DAC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACEN_A {
  #[doc = "0: The DAC system is disabled."]
  DACEN_0,
  #[doc = "1: The DAC system is enabled."]
  DACEN_1,
}
impl From<DACEN_A> for bool {
  #[inline(always)]
  fn from(variant: DACEN_A) -> Self {
    match variant {
      DACEN_A::DACEN_0 => false,
      DACEN_A::DACEN_1 => true,
    }
  }
}
#[doc = "Reader of field `DACEN`"]
pub type DACEN_R = crate::R<bool, DACEN_A>;
impl DACEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DACEN_A {
    match self.bits {
      false => DACEN_A::DACEN_0,
      true => DACEN_A::DACEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `DACEN_0`"]
  #[inline(always)]
  pub fn is_dacen_0(&self) -> bool {
    *self == DACEN_A::DACEN_0
  }
  #[doc = "Checks if the value of the field is `DACEN_1`"]
  #[inline(always)]
  pub fn is_dacen_1(&self) -> bool {
    *self == DACEN_A::DACEN_1
  }
}
#[doc = "Write proxy for field `DACEN`"]
pub struct DACEN_W<'a> {
  w: &'a mut W,
}
impl<'a> DACEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DACEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The DAC system is disabled."]
  #[inline(always)]
  pub fn dacen_0(self) -> &'a mut W {
    self.variant(DACEN_A::DACEN_0)
  }
  #[doc = "The DAC system is enabled."]
  #[inline(always)]
  pub fn dacen_1(self) -> &'a mut W {
    self.variant(DACEN_A::DACEN_1)
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
#[doc = "DAC Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACRFS_A {
  #[doc = "0: The DAC selects VREFH_INT as the reference voltage."]
  DACRFS_0,
  #[doc = "1: The DAC selects VREFH_EXT as the reference voltage."]
  DACRFS_1,
}
impl From<DACRFS_A> for bool {
  #[inline(always)]
  fn from(variant: DACRFS_A) -> Self {
    match variant {
      DACRFS_A::DACRFS_0 => false,
      DACRFS_A::DACRFS_1 => true,
    }
  }
}
#[doc = "Reader of field `DACRFS`"]
pub type DACRFS_R = crate::R<bool, DACRFS_A>;
impl DACRFS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DACRFS_A {
    match self.bits {
      false => DACRFS_A::DACRFS_0,
      true => DACRFS_A::DACRFS_1,
    }
  }
  #[doc = "Checks if the value of the field is `DACRFS_0`"]
  #[inline(always)]
  pub fn is_dacrfs_0(&self) -> bool {
    *self == DACRFS_A::DACRFS_0
  }
  #[doc = "Checks if the value of the field is `DACRFS_1`"]
  #[inline(always)]
  pub fn is_dacrfs_1(&self) -> bool {
    *self == DACRFS_A::DACRFS_1
  }
}
#[doc = "Write proxy for field `DACRFS`"]
pub struct DACRFS_W<'a> {
  w: &'a mut W,
}
impl<'a> DACRFS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DACRFS_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The DAC selects VREFH_INT as the reference voltage."]
  #[inline(always)]
  pub fn dacrfs_0(self) -> &'a mut W {
    self.variant(DACRFS_A::DACRFS_0)
  }
  #[doc = "The DAC selects VREFH_EXT as the reference voltage."]
  #[inline(always)]
  pub fn dacrfs_1(self) -> &'a mut W {
    self.variant(DACRFS_A::DACRFS_1)
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
#[doc = "Low Power Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPEN_A {
  #[doc = "0: High-Power mode"]
  LPEN_0,
  #[doc = "1: Low-Power mode"]
  LPEN_1,
}
impl From<LPEN_A> for bool {
  #[inline(always)]
  fn from(variant: LPEN_A) -> Self {
    match variant {
      LPEN_A::LPEN_0 => false,
      LPEN_A::LPEN_1 => true,
    }
  }
}
#[doc = "Reader of field `LPEN`"]
pub type LPEN_R = crate::R<bool, LPEN_A>;
impl LPEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LPEN_A {
    match self.bits {
      false => LPEN_A::LPEN_0,
      true => LPEN_A::LPEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `LPEN_0`"]
  #[inline(always)]
  pub fn is_lpen_0(&self) -> bool {
    *self == LPEN_A::LPEN_0
  }
  #[doc = "Checks if the value of the field is `LPEN_1`"]
  #[inline(always)]
  pub fn is_lpen_1(&self) -> bool {
    *self == LPEN_A::LPEN_1
  }
}
#[doc = "Write proxy for field `LPEN`"]
pub struct LPEN_W<'a> {
  w: &'a mut W,
}
impl<'a> LPEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LPEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "High-Power mode"]
  #[inline(always)]
  pub fn lpen_0(self) -> &'a mut W {
    self.variant(LPEN_A::LPEN_0)
  }
  #[doc = "Low-Power mode"]
  #[inline(always)]
  pub fn lpen_1(self) -> &'a mut W {
    self.variant(LPEN_A::LPEN_1)
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
#[doc = "FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOEN_A {
  #[doc = "0: FIFO mode is disabled and buffer mode is enabled. Any data written to DATA\\[DATA\\] goes to buffer then goes to conversion."]
  FIFOEN_0,
  #[doc = "1: FIFO mode is enabled. Data will be first read from FIFO to buffer then goes to conversion"]
  FIFOEN_1,
}
impl From<FIFOEN_A> for bool {
  #[inline(always)]
  fn from(variant: FIFOEN_A) -> Self {
    match variant {
      FIFOEN_A::FIFOEN_0 => false,
      FIFOEN_A::FIFOEN_1 => true,
    }
  }
}
#[doc = "Reader of field `FIFOEN`"]
pub type FIFOEN_R = crate::R<bool, FIFOEN_A>;
impl FIFOEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FIFOEN_A {
    match self.bits {
      false => FIFOEN_A::FIFOEN_0,
      true => FIFOEN_A::FIFOEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `FIFOEN_0`"]
  #[inline(always)]
  pub fn is_fifoen_0(&self) -> bool {
    *self == FIFOEN_A::FIFOEN_0
  }
  #[doc = "Checks if the value of the field is `FIFOEN_1`"]
  #[inline(always)]
  pub fn is_fifoen_1(&self) -> bool {
    *self == FIFOEN_A::FIFOEN_1
  }
}
#[doc = "Write proxy for field `FIFOEN`"]
pub struct FIFOEN_W<'a> {
  w: &'a mut W,
}
impl<'a> FIFOEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FIFOEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "FIFO mode is disabled and buffer mode is enabled. Any data written to DATA\\[DATA\\] goes to buffer then goes to conversion."]
  #[inline(always)]
  pub fn fifoen_0(self) -> &'a mut W {
    self.variant(FIFOEN_A::FIFOEN_0)
  }
  #[doc = "FIFO mode is enabled. Data will be first read from FIFO to buffer then goes to conversion"]
  #[inline(always)]
  pub fn fifoen_1(self) -> &'a mut W {
    self.variant(FIFOEN_A::FIFOEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
    self.w
  }
}
#[doc = "Swing Back Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWMD_A {
  #[doc = "0: Swing back mode disable"]
  SWMD_0,
  #[doc = "1: Swing back mode enable"]
  SWMD_1,
}
impl From<SWMD_A> for bool {
  #[inline(always)]
  fn from(variant: SWMD_A) -> Self {
    match variant {
      SWMD_A::SWMD_0 => false,
      SWMD_A::SWMD_1 => true,
    }
  }
}
#[doc = "Reader of field `SWMD`"]
pub type SWMD_R = crate::R<bool, SWMD_A>;
impl SWMD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SWMD_A {
    match self.bits {
      false => SWMD_A::SWMD_0,
      true => SWMD_A::SWMD_1,
    }
  }
  #[doc = "Checks if the value of the field is `SWMD_0`"]
  #[inline(always)]
  pub fn is_swmd_0(&self) -> bool {
    *self == SWMD_A::SWMD_0
  }
  #[doc = "Checks if the value of the field is `SWMD_1`"]
  #[inline(always)]
  pub fn is_swmd_1(&self) -> bool {
    *self == SWMD_A::SWMD_1
  }
}
#[doc = "Write proxy for field `SWMD`"]
pub struct SWMD_W<'a> {
  w: &'a mut W,
}
impl<'a> SWMD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SWMD_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Swing back mode disable"]
  #[inline(always)]
  pub fn swmd_0(self) -> &'a mut W {
    self.variant(SWMD_A::SWMD_0)
  }
  #[doc = "Swing back mode enable"]
  #[inline(always)]
  pub fn swmd_1(self) -> &'a mut W {
    self.variant(SWMD_A::SWMD_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
    self.w
  }
}
#[doc = "DAC Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSEL_A {
  #[doc = "0: The DAC hardware trigger is selected."]
  TRGSEL_0,
  #[doc = "1: The DAC software trigger is selected."]
  TRGSEL_1,
}
impl From<TRGSEL_A> for bool {
  #[inline(always)]
  fn from(variant: TRGSEL_A) -> Self {
    match variant {
      TRGSEL_A::TRGSEL_0 => false,
      TRGSEL_A::TRGSEL_1 => true,
    }
  }
}
#[doc = "Reader of field `TRGSEL`"]
pub type TRGSEL_R = crate::R<bool, TRGSEL_A>;
impl TRGSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TRGSEL_A {
    match self.bits {
      false => TRGSEL_A::TRGSEL_0,
      true => TRGSEL_A::TRGSEL_1,
    }
  }
  #[doc = "Checks if the value of the field is `TRGSEL_0`"]
  #[inline(always)]
  pub fn is_trgsel_0(&self) -> bool {
    *self == TRGSEL_A::TRGSEL_0
  }
  #[doc = "Checks if the value of the field is `TRGSEL_1`"]
  #[inline(always)]
  pub fn is_trgsel_1(&self) -> bool {
    *self == TRGSEL_A::TRGSEL_1
  }
}
#[doc = "Write proxy for field `TRGSEL`"]
pub struct TRGSEL_W<'a> {
  w: &'a mut W,
}
impl<'a> TRGSEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRGSEL_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The DAC hardware trigger is selected."]
  #[inline(always)]
  pub fn trgsel_0(self) -> &'a mut W {
    self.variant(TRGSEL_A::TRGSEL_0)
  }
  #[doc = "The DAC software trigger is selected."]
  #[inline(always)]
  pub fn trgsel_1(self) -> &'a mut W {
    self.variant(TRGSEL_A::TRGSEL_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - DAC Enable"]
  #[inline(always)]
  pub fn dacen(&self) -> DACEN_R {
    DACEN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - DAC Reference Select"]
  #[inline(always)]
  pub fn dacrfs(&self) -> DACRFS_R {
    DACRFS_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Low Power Enable"]
  #[inline(always)]
  pub fn lpen(&self) -> LPEN_R {
    LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - FIFO Enable"]
  #[inline(always)]
  pub fn fifoen(&self) -> FIFOEN_R {
    FIFOEN_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Swing Back Mode"]
  #[inline(always)]
  pub fn swmd(&self) -> SWMD_R {
    SWMD_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - DAC Trigger Select"]
  #[inline(always)]
  pub fn trgsel(&self) -> TRGSEL_R {
    TRGSEL_R::new(((self.bits >> 5) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - DAC Enable"]
  #[inline(always)]
  pub fn dacen(&mut self) -> DACEN_W {
    DACEN_W { w: self }
  }
  #[doc = "Bit 1 - DAC Reference Select"]
  #[inline(always)]
  pub fn dacrfs(&mut self) -> DACRFS_W {
    DACRFS_W { w: self }
  }
  #[doc = "Bit 2 - Low Power Enable"]
  #[inline(always)]
  pub fn lpen(&mut self) -> LPEN_W {
    LPEN_W { w: self }
  }
  #[doc = "Bit 3 - FIFO Enable"]
  #[inline(always)]
  pub fn fifoen(&mut self) -> FIFOEN_W {
    FIFOEN_W { w: self }
  }
  #[doc = "Bit 4 - Swing Back Mode"]
  #[inline(always)]
  pub fn swmd(&mut self) -> SWMD_W {
    SWMD_W { w: self }
  }
  #[doc = "Bit 5 - DAC Trigger Select"]
  #[inline(always)]
  pub fn trgsel(&mut self) -> TRGSEL_W {
    TRGSEL_W { w: self }
  }
}
