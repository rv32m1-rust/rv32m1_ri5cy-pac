#[doc = "Reader of register HVDSC1"]
pub type R = crate::R<u32, super::HVDSC1>;
#[doc = "Writer for register HVDSC1"]
pub type W = crate::W<u32, super::HVDSC1>;
#[doc = "Register HVDSC1 `reset()`'s with value 0x0001_0000"]
impl crate::ResetValue for super::HVDSC1 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0001_0000
  }
}
#[doc = "VDD High-Voltage Detect Voltage Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDD_HVDV_A {
  #[doc = "0: no description available"]
  VDD_HVDV_0,
  #[doc = "1: no description available"]
  VDD_HVDV_1,
}
impl From<VDD_HVDV_A> for bool {
  #[inline(always)]
  fn from(variant: VDD_HVDV_A) -> Self {
    match variant {
      VDD_HVDV_A::VDD_HVDV_0 => false,
      VDD_HVDV_A::VDD_HVDV_1 => true,
    }
  }
}
#[doc = "Reader of field `VDD_HVDV`"]
pub type VDD_HVDV_R = crate::R<bool, VDD_HVDV_A>;
impl VDD_HVDV_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VDD_HVDV_A {
    match self.bits {
      false => VDD_HVDV_A::VDD_HVDV_0,
      true => VDD_HVDV_A::VDD_HVDV_1,
    }
  }
  #[doc = "Checks if the value of the field is `VDD_HVDV_0`"]
  #[inline(always)]
  pub fn is_vdd_hvdv_0(&self) -> bool {
    *self == VDD_HVDV_A::VDD_HVDV_0
  }
  #[doc = "Checks if the value of the field is `VDD_HVDV_1`"]
  #[inline(always)]
  pub fn is_vdd_hvdv_1(&self) -> bool {
    *self == VDD_HVDV_A::VDD_HVDV_1
  }
}
#[doc = "Write proxy for field `VDD_HVDV`"]
pub struct VDD_HVDV_W<'a> {
  w: &'a mut W,
}
impl<'a> VDD_HVDV_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VDD_HVDV_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn vdd_hvdv_0(self) -> &'a mut W {
    self.variant(VDD_HVDV_A::VDD_HVDV_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn vdd_hvdv_1(self) -> &'a mut W {
    self.variant(VDD_HVDV_A::VDD_HVDV_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
    self.w
  }
}
#[doc = "VDD High-Voltage Detect Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDD_HVDRE_A {
  #[doc = "0: VDD HVDF does not generate hardware resets"]
  VDD_HVDRE_0,
  #[doc = "1: Force an MCU reset when VDD_HVDF = 1"]
  VDD_HVDRE_1,
}
impl From<VDD_HVDRE_A> for bool {
  #[inline(always)]
  fn from(variant: VDD_HVDRE_A) -> Self {
    match variant {
      VDD_HVDRE_A::VDD_HVDRE_0 => false,
      VDD_HVDRE_A::VDD_HVDRE_1 => true,
    }
  }
}
#[doc = "Reader of field `VDD_HVDRE`"]
pub type VDD_HVDRE_R = crate::R<bool, VDD_HVDRE_A>;
impl VDD_HVDRE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VDD_HVDRE_A {
    match self.bits {
      false => VDD_HVDRE_A::VDD_HVDRE_0,
      true => VDD_HVDRE_A::VDD_HVDRE_1,
    }
  }
  #[doc = "Checks if the value of the field is `VDD_HVDRE_0`"]
  #[inline(always)]
  pub fn is_vdd_hvdre_0(&self) -> bool {
    *self == VDD_HVDRE_A::VDD_HVDRE_0
  }
  #[doc = "Checks if the value of the field is `VDD_HVDRE_1`"]
  #[inline(always)]
  pub fn is_vdd_hvdre_1(&self) -> bool {
    *self == VDD_HVDRE_A::VDD_HVDRE_1
  }
}
#[doc = "Write proxy for field `VDD_HVDRE`"]
pub struct VDD_HVDRE_W<'a> {
  w: &'a mut W,
}
impl<'a> VDD_HVDRE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VDD_HVDRE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "VDD HVDF does not generate hardware resets"]
  #[inline(always)]
  pub fn vdd_hvdre_0(self) -> &'a mut W {
    self.variant(VDD_HVDRE_A::VDD_HVDRE_0)
  }
  #[doc = "Force an MCU reset when VDD_HVDF = 1"]
  #[inline(always)]
  pub fn vdd_hvdre_1(self) -> &'a mut W {
    self.variant(VDD_HVDRE_A::VDD_HVDRE_1)
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
#[doc = "VDD High-Voltage Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDD_HVDIE_A {
  #[doc = "0: Hardware interrupt disabled (use polling)"]
  VDD_HVDIE_0,
  #[doc = "1: Request a hardware interrupt when HVDF = 1"]
  VDD_HVDIE_1,
}
impl From<VDD_HVDIE_A> for bool {
  #[inline(always)]
  fn from(variant: VDD_HVDIE_A) -> Self {
    match variant {
      VDD_HVDIE_A::VDD_HVDIE_0 => false,
      VDD_HVDIE_A::VDD_HVDIE_1 => true,
    }
  }
}
#[doc = "Reader of field `VDD_HVDIE`"]
pub type VDD_HVDIE_R = crate::R<bool, VDD_HVDIE_A>;
impl VDD_HVDIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VDD_HVDIE_A {
    match self.bits {
      false => VDD_HVDIE_A::VDD_HVDIE_0,
      true => VDD_HVDIE_A::VDD_HVDIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `VDD_HVDIE_0`"]
  #[inline(always)]
  pub fn is_vdd_hvdie_0(&self) -> bool {
    *self == VDD_HVDIE_A::VDD_HVDIE_0
  }
  #[doc = "Checks if the value of the field is `VDD_HVDIE_1`"]
  #[inline(always)]
  pub fn is_vdd_hvdie_1(&self) -> bool {
    *self == VDD_HVDIE_A::VDD_HVDIE_1
  }
}
#[doc = "Write proxy for field `VDD_HVDIE`"]
pub struct VDD_HVDIE_W<'a> {
  w: &'a mut W,
}
impl<'a> VDD_HVDIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VDD_HVDIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Hardware interrupt disabled (use polling)"]
  #[inline(always)]
  pub fn vdd_hvdie_0(self) -> &'a mut W {
    self.variant(VDD_HVDIE_A::VDD_HVDIE_0)
  }
  #[doc = "Request a hardware interrupt when HVDF = 1"]
  #[inline(always)]
  pub fn vdd_hvdie_1(self) -> &'a mut W {
    self.variant(VDD_HVDIE_A::VDD_HVDIE_1)
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
#[doc = "Write proxy for field `VDD_HVDACK`"]
pub struct VDD_HVDACK_W<'a> {
  w: &'a mut W,
}
impl<'a> VDD_HVDACK_W<'a> {
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
#[doc = "VDD High-Voltage Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDD_HVDF_A {
  #[doc = "0: Vdd High-voltage event not detected"]
  VDD_HVDF_0,
  #[doc = "1: Vdd High-voltage event detected"]
  VDD_HVDF_1,
}
impl From<VDD_HVDF_A> for bool {
  #[inline(always)]
  fn from(variant: VDD_HVDF_A) -> Self {
    match variant {
      VDD_HVDF_A::VDD_HVDF_0 => false,
      VDD_HVDF_A::VDD_HVDF_1 => true,
    }
  }
}
#[doc = "Reader of field `VDD_HVDF`"]
pub type VDD_HVDF_R = crate::R<bool, VDD_HVDF_A>;
impl VDD_HVDF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VDD_HVDF_A {
    match self.bits {
      false => VDD_HVDF_A::VDD_HVDF_0,
      true => VDD_HVDF_A::VDD_HVDF_1,
    }
  }
  #[doc = "Checks if the value of the field is `VDD_HVDF_0`"]
  #[inline(always)]
  pub fn is_vdd_hvdf_0(&self) -> bool {
    *self == VDD_HVDF_A::VDD_HVDF_0
  }
  #[doc = "Checks if the value of the field is `VDD_HVDF_1`"]
  #[inline(always)]
  pub fn is_vdd_hvdf_1(&self) -> bool {
    *self == VDD_HVDF_A::VDD_HVDF_1
  }
}
impl R {
  #[doc = "Bit 16 - VDD High-Voltage Detect Voltage Select"]
  #[inline(always)]
  pub fn vdd_hvdv(&self) -> VDD_HVDV_R {
    VDD_HVDV_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 20 - VDD High-Voltage Detect Reset Enable"]
  #[inline(always)]
  pub fn vdd_hvdre(&self) -> VDD_HVDRE_R {
    VDD_HVDRE_R::new(((self.bits >> 20) & 0x01) != 0)
  }
  #[doc = "Bit 21 - VDD High-Voltage Detect Interrupt Enable"]
  #[inline(always)]
  pub fn vdd_hvdie(&self) -> VDD_HVDIE_R {
    VDD_HVDIE_R::new(((self.bits >> 21) & 0x01) != 0)
  }
  #[doc = "Bit 23 - VDD High-Voltage Detect Flag"]
  #[inline(always)]
  pub fn vdd_hvdf(&self) -> VDD_HVDF_R {
    VDD_HVDF_R::new(((self.bits >> 23) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 16 - VDD High-Voltage Detect Voltage Select"]
  #[inline(always)]
  pub fn vdd_hvdv(&mut self) -> VDD_HVDV_W {
    VDD_HVDV_W { w: self }
  }
  #[doc = "Bit 20 - VDD High-Voltage Detect Reset Enable"]
  #[inline(always)]
  pub fn vdd_hvdre(&mut self) -> VDD_HVDRE_W {
    VDD_HVDRE_W { w: self }
  }
  #[doc = "Bit 21 - VDD High-Voltage Detect Interrupt Enable"]
  #[inline(always)]
  pub fn vdd_hvdie(&mut self) -> VDD_HVDIE_W {
    VDD_HVDIE_W { w: self }
  }
  #[doc = "Bit 22 - VDD High-Voltage Detect Acknowledge"]
  #[inline(always)]
  pub fn vdd_hvdack(&mut self) -> VDD_HVDACK_W {
    VDD_HVDACK_W { w: self }
  }
}
