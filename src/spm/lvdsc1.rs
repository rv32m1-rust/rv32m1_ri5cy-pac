#[doc = "Reader of register LVDSC1"]
pub type R = crate::R<u32, super::LVDSC1>;
#[doc = "Writer for register LVDSC1"]
pub type W = crate::W<u32, super::LVDSC1>;
#[doc = "Register LVDSC1 `reset()`'s with value 0x0010_0010"]
impl crate::ResetValue for super::LVDSC1 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0010_0010
  }
}
#[doc = "Core Low-Voltage Detect Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COREVDD_LVDRE_A {
  #[doc = "0: COREVDD_LVDF does not generate hardware resets"]
  COREVDD_LVDRE_0,
  #[doc = "1: Force an MCU reset when CORE_LVDF = 1"]
  COREVDD_LVDRE_1,
}
impl From<COREVDD_LVDRE_A> for bool {
  #[inline(always)]
  fn from(variant: COREVDD_LVDRE_A) -> Self {
    match variant {
      COREVDD_LVDRE_A::COREVDD_LVDRE_0 => false,
      COREVDD_LVDRE_A::COREVDD_LVDRE_1 => true,
    }
  }
}
#[doc = "Reader of field `COREVDD_LVDRE`"]
pub type COREVDD_LVDRE_R = crate::R<bool, COREVDD_LVDRE_A>;
impl COREVDD_LVDRE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> COREVDD_LVDRE_A {
    match self.bits {
      false => COREVDD_LVDRE_A::COREVDD_LVDRE_0,
      true => COREVDD_LVDRE_A::COREVDD_LVDRE_1,
    }
  }
  #[doc = "Checks if the value of the field is `COREVDD_LVDRE_0`"]
  #[inline(always)]
  pub fn is_corevdd_lvdre_0(&self) -> bool {
    *self == COREVDD_LVDRE_A::COREVDD_LVDRE_0
  }
  #[doc = "Checks if the value of the field is `COREVDD_LVDRE_1`"]
  #[inline(always)]
  pub fn is_corevdd_lvdre_1(&self) -> bool {
    *self == COREVDD_LVDRE_A::COREVDD_LVDRE_1
  }
}
#[doc = "Write proxy for field `COREVDD_LVDRE`"]
pub struct COREVDD_LVDRE_W<'a> {
  w: &'a mut W,
}
impl<'a> COREVDD_LVDRE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: COREVDD_LVDRE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "COREVDD_LVDF does not generate hardware resets"]
  #[inline(always)]
  pub fn corevdd_lvdre_0(self) -> &'a mut W {
    self.variant(COREVDD_LVDRE_A::COREVDD_LVDRE_0)
  }
  #[doc = "Force an MCU reset when CORE_LVDF = 1"]
  #[inline(always)]
  pub fn corevdd_lvdre_1(self) -> &'a mut W {
    self.variant(COREVDD_LVDRE_A::COREVDD_LVDRE_1)
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
#[doc = "Low-Voltage Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COREVDD_LVDIE_A {
  #[doc = "0: Hardware interrupt disabled (use polling)"]
  COREVDD_LVDIE_0,
  #[doc = "1: Request a hardware interrupt when LVDF = 1"]
  COREVDD_LVDIE_1,
}
impl From<COREVDD_LVDIE_A> for bool {
  #[inline(always)]
  fn from(variant: COREVDD_LVDIE_A) -> Self {
    match variant {
      COREVDD_LVDIE_A::COREVDD_LVDIE_0 => false,
      COREVDD_LVDIE_A::COREVDD_LVDIE_1 => true,
    }
  }
}
#[doc = "Reader of field `COREVDD_LVDIE`"]
pub type COREVDD_LVDIE_R = crate::R<bool, COREVDD_LVDIE_A>;
impl COREVDD_LVDIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> COREVDD_LVDIE_A {
    match self.bits {
      false => COREVDD_LVDIE_A::COREVDD_LVDIE_0,
      true => COREVDD_LVDIE_A::COREVDD_LVDIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `COREVDD_LVDIE_0`"]
  #[inline(always)]
  pub fn is_corevdd_lvdie_0(&self) -> bool {
    *self == COREVDD_LVDIE_A::COREVDD_LVDIE_0
  }
  #[doc = "Checks if the value of the field is `COREVDD_LVDIE_1`"]
  #[inline(always)]
  pub fn is_corevdd_lvdie_1(&self) -> bool {
    *self == COREVDD_LVDIE_A::COREVDD_LVDIE_1
  }
}
#[doc = "Write proxy for field `COREVDD_LVDIE`"]
pub struct COREVDD_LVDIE_W<'a> {
  w: &'a mut W,
}
impl<'a> COREVDD_LVDIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: COREVDD_LVDIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Hardware interrupt disabled (use polling)"]
  #[inline(always)]
  pub fn corevdd_lvdie_0(self) -> &'a mut W {
    self.variant(COREVDD_LVDIE_A::COREVDD_LVDIE_0)
  }
  #[doc = "Request a hardware interrupt when LVDF = 1"]
  #[inline(always)]
  pub fn corevdd_lvdie_1(self) -> &'a mut W {
    self.variant(COREVDD_LVDIE_A::COREVDD_LVDIE_1)
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
#[doc = "Write proxy for field `COREVDD_LVDACK`"]
pub struct COREVDD_LVDACK_W<'a> {
  w: &'a mut W,
}
impl<'a> COREVDD_LVDACK_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
    self.w
  }
}
#[doc = "Low-Voltage Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COREVDD_LVDF_A {
  #[doc = "0: Low-voltage event not detected"]
  COREVDD_LVDF_0,
  #[doc = "1: Low-voltage event detected"]
  COREVDD_LVDF_1,
}
impl From<COREVDD_LVDF_A> for bool {
  #[inline(always)]
  fn from(variant: COREVDD_LVDF_A) -> Self {
    match variant {
      COREVDD_LVDF_A::COREVDD_LVDF_0 => false,
      COREVDD_LVDF_A::COREVDD_LVDF_1 => true,
    }
  }
}
#[doc = "Reader of field `COREVDD_LVDF`"]
pub type COREVDD_LVDF_R = crate::R<bool, COREVDD_LVDF_A>;
impl COREVDD_LVDF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> COREVDD_LVDF_A {
    match self.bits {
      false => COREVDD_LVDF_A::COREVDD_LVDF_0,
      true => COREVDD_LVDF_A::COREVDD_LVDF_1,
    }
  }
  #[doc = "Checks if the value of the field is `COREVDD_LVDF_0`"]
  #[inline(always)]
  pub fn is_corevdd_lvdf_0(&self) -> bool {
    *self == COREVDD_LVDF_A::COREVDD_LVDF_0
  }
  #[doc = "Checks if the value of the field is `COREVDD_LVDF_1`"]
  #[inline(always)]
  pub fn is_corevdd_lvdf_1(&self) -> bool {
    *self == COREVDD_LVDF_A::COREVDD_LVDF_1
  }
}
#[doc = "VDD Low-Voltage Detect Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDD_LVDV_A {
  #[doc = "0: no description available"]
  VDD_LVDV_0,
  #[doc = "1: no description available"]
  VDD_LVDV_1,
}
impl From<VDD_LVDV_A> for u8 {
  #[inline(always)]
  fn from(variant: VDD_LVDV_A) -> Self {
    match variant {
      VDD_LVDV_A::VDD_LVDV_0 => 0,
      VDD_LVDV_A::VDD_LVDV_1 => 1,
    }
  }
}
#[doc = "Reader of field `VDD_LVDV`"]
pub type VDD_LVDV_R = crate::R<u8, VDD_LVDV_A>;
impl VDD_LVDV_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, VDD_LVDV_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(VDD_LVDV_A::VDD_LVDV_0),
      1 => Val(VDD_LVDV_A::VDD_LVDV_1),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `VDD_LVDV_0`"]
  #[inline(always)]
  pub fn is_vdd_lvdv_0(&self) -> bool {
    *self == VDD_LVDV_A::VDD_LVDV_0
  }
  #[doc = "Checks if the value of the field is `VDD_LVDV_1`"]
  #[inline(always)]
  pub fn is_vdd_lvdv_1(&self) -> bool {
    *self == VDD_LVDV_A::VDD_LVDV_1
  }
}
#[doc = "Write proxy for field `VDD_LVDV`"]
pub struct VDD_LVDV_W<'a> {
  w: &'a mut W,
}
impl<'a> VDD_LVDV_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VDD_LVDV_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn vdd_lvdv_0(self) -> &'a mut W {
    self.variant(VDD_LVDV_A::VDD_LVDV_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn vdd_lvdv_1(self) -> &'a mut W {
    self.variant(VDD_LVDV_A::VDD_LVDV_1)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
    self.w
  }
}
#[doc = "VDD Low-Voltage Detect Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDD_LVDRE_A {
  #[doc = "0: VDD_LVDF does not generate hardware resets"]
  VDD_LVDRE_0,
  #[doc = "1: Force an MCU reset when VDD_LVDF = 1"]
  VDD_LVDRE_1,
}
impl From<VDD_LVDRE_A> for bool {
  #[inline(always)]
  fn from(variant: VDD_LVDRE_A) -> Self {
    match variant {
      VDD_LVDRE_A::VDD_LVDRE_0 => false,
      VDD_LVDRE_A::VDD_LVDRE_1 => true,
    }
  }
}
#[doc = "Reader of field `VDD_LVDRE`"]
pub type VDD_LVDRE_R = crate::R<bool, VDD_LVDRE_A>;
impl VDD_LVDRE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VDD_LVDRE_A {
    match self.bits {
      false => VDD_LVDRE_A::VDD_LVDRE_0,
      true => VDD_LVDRE_A::VDD_LVDRE_1,
    }
  }
  #[doc = "Checks if the value of the field is `VDD_LVDRE_0`"]
  #[inline(always)]
  pub fn is_vdd_lvdre_0(&self) -> bool {
    *self == VDD_LVDRE_A::VDD_LVDRE_0
  }
  #[doc = "Checks if the value of the field is `VDD_LVDRE_1`"]
  #[inline(always)]
  pub fn is_vdd_lvdre_1(&self) -> bool {
    *self == VDD_LVDRE_A::VDD_LVDRE_1
  }
}
#[doc = "Write proxy for field `VDD_LVDRE`"]
pub struct VDD_LVDRE_W<'a> {
  w: &'a mut W,
}
impl<'a> VDD_LVDRE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VDD_LVDRE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "VDD_LVDF does not generate hardware resets"]
  #[inline(always)]
  pub fn vdd_lvdre_0(self) -> &'a mut W {
    self.variant(VDD_LVDRE_A::VDD_LVDRE_0)
  }
  #[doc = "Force an MCU reset when VDD_LVDF = 1"]
  #[inline(always)]
  pub fn vdd_lvdre_1(self) -> &'a mut W {
    self.variant(VDD_LVDRE_A::VDD_LVDRE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
    self.w
  }
}
#[doc = "VDD Low-Voltage Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDD_LVDIE_A {
  #[doc = "0: Hardware interrupt disabled (use polling)"]
  VDD_LVDIE_0,
  #[doc = "1: Request a hardware interrupt when VDD_LVDF = 1"]
  VDD_LVDIE_1,
}
impl From<VDD_LVDIE_A> for bool {
  #[inline(always)]
  fn from(variant: VDD_LVDIE_A) -> Self {
    match variant {
      VDD_LVDIE_A::VDD_LVDIE_0 => false,
      VDD_LVDIE_A::VDD_LVDIE_1 => true,
    }
  }
}
#[doc = "Reader of field `VDD_LVDIE`"]
pub type VDD_LVDIE_R = crate::R<bool, VDD_LVDIE_A>;
impl VDD_LVDIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VDD_LVDIE_A {
    match self.bits {
      false => VDD_LVDIE_A::VDD_LVDIE_0,
      true => VDD_LVDIE_A::VDD_LVDIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `VDD_LVDIE_0`"]
  #[inline(always)]
  pub fn is_vdd_lvdie_0(&self) -> bool {
    *self == VDD_LVDIE_A::VDD_LVDIE_0
  }
  #[doc = "Checks if the value of the field is `VDD_LVDIE_1`"]
  #[inline(always)]
  pub fn is_vdd_lvdie_1(&self) -> bool {
    *self == VDD_LVDIE_A::VDD_LVDIE_1
  }
}
#[doc = "Write proxy for field `VDD_LVDIE`"]
pub struct VDD_LVDIE_W<'a> {
  w: &'a mut W,
}
impl<'a> VDD_LVDIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VDD_LVDIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Hardware interrupt disabled (use polling)"]
  #[inline(always)]
  pub fn vdd_lvdie_0(self) -> &'a mut W {
    self.variant(VDD_LVDIE_A::VDD_LVDIE_0)
  }
  #[doc = "Request a hardware interrupt when VDD_LVDF = 1"]
  #[inline(always)]
  pub fn vdd_lvdie_1(self) -> &'a mut W {
    self.variant(VDD_LVDIE_A::VDD_LVDIE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
    self.w
  }
}
#[doc = "Write proxy for field `VDD_LVDACK`"]
pub struct VDD_LVDACK_W<'a> {
  w: &'a mut W,
}
impl<'a> VDD_LVDACK_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
    self.w
  }
}
#[doc = "VDD Low-Voltage Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDD_LVDF_A {
  #[doc = "0: Low-voltage event not detected"]
  VDD_LVDF_0,
  #[doc = "1: Low-voltage event detected"]
  VDD_LVDF_1,
}
impl From<VDD_LVDF_A> for bool {
  #[inline(always)]
  fn from(variant: VDD_LVDF_A) -> Self {
    match variant {
      VDD_LVDF_A::VDD_LVDF_0 => false,
      VDD_LVDF_A::VDD_LVDF_1 => true,
    }
  }
}
#[doc = "Reader of field `VDD_LVDF`"]
pub type VDD_LVDF_R = crate::R<bool, VDD_LVDF_A>;
impl VDD_LVDF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VDD_LVDF_A {
    match self.bits {
      false => VDD_LVDF_A::VDD_LVDF_0,
      true => VDD_LVDF_A::VDD_LVDF_1,
    }
  }
  #[doc = "Checks if the value of the field is `VDD_LVDF_0`"]
  #[inline(always)]
  pub fn is_vdd_lvdf_0(&self) -> bool {
    *self == VDD_LVDF_A::VDD_LVDF_0
  }
  #[doc = "Checks if the value of the field is `VDD_LVDF_1`"]
  #[inline(always)]
  pub fn is_vdd_lvdf_1(&self) -> bool {
    *self == VDD_LVDF_A::VDD_LVDF_1
  }
}
impl R {
  #[doc = "Bit 4 - Core Low-Voltage Detect Reset Enable"]
  #[inline(always)]
  pub fn corevdd_lvdre(&self) -> COREVDD_LVDRE_R {
    COREVDD_LVDRE_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Low-Voltage Detect Interrupt Enable"]
  #[inline(always)]
  pub fn corevdd_lvdie(&self) -> COREVDD_LVDIE_R {
    COREVDD_LVDIE_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Low-Voltage Detect Flag"]
  #[inline(always)]
  pub fn corevdd_lvdf(&self) -> COREVDD_LVDF_R {
    COREVDD_LVDF_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bits 16:17 - VDD Low-Voltage Detect Voltage Select"]
  #[inline(always)]
  pub fn vdd_lvdv(&self) -> VDD_LVDV_R {
    VDD_LVDV_R::new(((self.bits >> 16) & 0x03) as u8)
  }
  #[doc = "Bit 20 - VDD Low-Voltage Detect Reset Enable"]
  #[inline(always)]
  pub fn vdd_lvdre(&self) -> VDD_LVDRE_R {
    VDD_LVDRE_R::new(((self.bits >> 20) & 0x01) != 0)
  }
  #[doc = "Bit 21 - VDD Low-Voltage Detect Interrupt Enable"]
  #[inline(always)]
  pub fn vdd_lvdie(&self) -> VDD_LVDIE_R {
    VDD_LVDIE_R::new(((self.bits >> 21) & 0x01) != 0)
  }
  #[doc = "Bit 23 - VDD Low-Voltage Detect Flag"]
  #[inline(always)]
  pub fn vdd_lvdf(&self) -> VDD_LVDF_R {
    VDD_LVDF_R::new(((self.bits >> 23) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 4 - Core Low-Voltage Detect Reset Enable"]
  #[inline(always)]
  pub fn corevdd_lvdre(&mut self) -> COREVDD_LVDRE_W {
    COREVDD_LVDRE_W { w: self }
  }
  #[doc = "Bit 5 - Low-Voltage Detect Interrupt Enable"]
  #[inline(always)]
  pub fn corevdd_lvdie(&mut self) -> COREVDD_LVDIE_W {
    COREVDD_LVDIE_W { w: self }
  }
  #[doc = "Bit 6 - Low-Voltage Detect Acknowledge"]
  #[inline(always)]
  pub fn corevdd_lvdack(&mut self) -> COREVDD_LVDACK_W {
    COREVDD_LVDACK_W { w: self }
  }
  #[doc = "Bits 16:17 - VDD Low-Voltage Detect Voltage Select"]
  #[inline(always)]
  pub fn vdd_lvdv(&mut self) -> VDD_LVDV_W {
    VDD_LVDV_W { w: self }
  }
  #[doc = "Bit 20 - VDD Low-Voltage Detect Reset Enable"]
  #[inline(always)]
  pub fn vdd_lvdre(&mut self) -> VDD_LVDRE_W {
    VDD_LVDRE_W { w: self }
  }
  #[doc = "Bit 21 - VDD Low-Voltage Detect Interrupt Enable"]
  #[inline(always)]
  pub fn vdd_lvdie(&mut self) -> VDD_LVDIE_W {
    VDD_LVDIE_W { w: self }
  }
  #[doc = "Bit 22 - VDD Low-Voltage Detect Acknowledge"]
  #[inline(always)]
  pub fn vdd_lvdack(&mut self) -> VDD_LVDACK_W {
    VDD_LVDACK_W { w: self }
  }
}
