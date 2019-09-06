#[doc = "Reader of register LVDSC2"]
pub type R = crate::R<u32, super::LVDSC2>;
#[doc = "Writer for register LVDSC2"]
pub type W = crate::W<u32, super::LVDSC2>;
#[doc = "Register LVDSC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::LVDSC2 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "VDD Low-Voltage Warning Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDD_LVWV_A {
  #[doc = "0: no description available"]
  VDD_LVWV_0,
  #[doc = "1: no description available"]
  VDD_LVWV_1,
  #[doc = "2: no description available"]
  VDD_LVWV_2,
  #[doc = "3: no description available"]
  VDD_LVWV_3,
}
impl From<VDD_LVWV_A> for u8 {
  #[inline(always)]
  fn from(variant: VDD_LVWV_A) -> Self {
    match variant {
      VDD_LVWV_A::VDD_LVWV_0 => 0,
      VDD_LVWV_A::VDD_LVWV_1 => 1,
      VDD_LVWV_A::VDD_LVWV_2 => 2,
      VDD_LVWV_A::VDD_LVWV_3 => 3,
    }
  }
}
#[doc = "Reader of field `VDD_LVWV`"]
pub type VDD_LVWV_R = crate::R<u8, VDD_LVWV_A>;
impl VDD_LVWV_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VDD_LVWV_A {
    match self.bits {
      0 => VDD_LVWV_A::VDD_LVWV_0,
      1 => VDD_LVWV_A::VDD_LVWV_1,
      2 => VDD_LVWV_A::VDD_LVWV_2,
      3 => VDD_LVWV_A::VDD_LVWV_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `VDD_LVWV_0`"]
  #[inline(always)]
  pub fn is_vdd_lvwv_0(&self) -> bool {
    *self == VDD_LVWV_A::VDD_LVWV_0
  }
  #[doc = "Checks if the value of the field is `VDD_LVWV_1`"]
  #[inline(always)]
  pub fn is_vdd_lvwv_1(&self) -> bool {
    *self == VDD_LVWV_A::VDD_LVWV_1
  }
  #[doc = "Checks if the value of the field is `VDD_LVWV_2`"]
  #[inline(always)]
  pub fn is_vdd_lvwv_2(&self) -> bool {
    *self == VDD_LVWV_A::VDD_LVWV_2
  }
  #[doc = "Checks if the value of the field is `VDD_LVWV_3`"]
  #[inline(always)]
  pub fn is_vdd_lvwv_3(&self) -> bool {
    *self == VDD_LVWV_A::VDD_LVWV_3
  }
}
#[doc = "Write proxy for field `VDD_LVWV`"]
pub struct VDD_LVWV_W<'a> {
  w: &'a mut W,
}
impl<'a> VDD_LVWV_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VDD_LVWV_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn vdd_lvwv_0(self) -> &'a mut W {
    self.variant(VDD_LVWV_A::VDD_LVWV_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn vdd_lvwv_1(self) -> &'a mut W {
    self.variant(VDD_LVWV_A::VDD_LVWV_1)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn vdd_lvwv_2(self) -> &'a mut W {
    self.variant(VDD_LVWV_A::VDD_LVWV_2)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn vdd_lvwv_3(self) -> &'a mut W {
    self.variant(VDD_LVWV_A::VDD_LVWV_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
    self.w
  }
}
#[doc = "VDD Low-Voltage Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDD_LVWIE_A {
  #[doc = "0: Hardware interrupt disabled (use polling)"]
  VDD_LVWIE_0,
  #[doc = "1: Request a hardware interrupt when VDD_LVWF = 1"]
  VDD_LVWIE_1,
}
impl From<VDD_LVWIE_A> for bool {
  #[inline(always)]
  fn from(variant: VDD_LVWIE_A) -> Self {
    match variant {
      VDD_LVWIE_A::VDD_LVWIE_0 => false,
      VDD_LVWIE_A::VDD_LVWIE_1 => true,
    }
  }
}
#[doc = "Reader of field `VDD_LVWIE`"]
pub type VDD_LVWIE_R = crate::R<bool, VDD_LVWIE_A>;
impl VDD_LVWIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VDD_LVWIE_A {
    match self.bits {
      false => VDD_LVWIE_A::VDD_LVWIE_0,
      true => VDD_LVWIE_A::VDD_LVWIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `VDD_LVWIE_0`"]
  #[inline(always)]
  pub fn is_vdd_lvwie_0(&self) -> bool {
    *self == VDD_LVWIE_A::VDD_LVWIE_0
  }
  #[doc = "Checks if the value of the field is `VDD_LVWIE_1`"]
  #[inline(always)]
  pub fn is_vdd_lvwie_1(&self) -> bool {
    *self == VDD_LVWIE_A::VDD_LVWIE_1
  }
}
#[doc = "Write proxy for field `VDD_LVWIE`"]
pub struct VDD_LVWIE_W<'a> {
  w: &'a mut W,
}
impl<'a> VDD_LVWIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VDD_LVWIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Hardware interrupt disabled (use polling)"]
  #[inline(always)]
  pub fn vdd_lvwie_0(self) -> &'a mut W {
    self.variant(VDD_LVWIE_A::VDD_LVWIE_0)
  }
  #[doc = "Request a hardware interrupt when VDD_LVWF = 1"]
  #[inline(always)]
  pub fn vdd_lvwie_1(self) -> &'a mut W {
    self.variant(VDD_LVWIE_A::VDD_LVWIE_1)
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
#[doc = "Write proxy for field `VDD_LVWACK`"]
pub struct VDD_LVWACK_W<'a> {
  w: &'a mut W,
}
impl<'a> VDD_LVWACK_W<'a> {
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
#[doc = "VDD Low-Voltage Warning Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDD_LVWF_A {
  #[doc = "0: Low-voltage warning event not detected"]
  VDD_LVWF_0,
  #[doc = "1: Low-voltage warning event detected"]
  VDD_LVWF_1,
}
impl From<VDD_LVWF_A> for bool {
  #[inline(always)]
  fn from(variant: VDD_LVWF_A) -> Self {
    match variant {
      VDD_LVWF_A::VDD_LVWF_0 => false,
      VDD_LVWF_A::VDD_LVWF_1 => true,
    }
  }
}
#[doc = "Reader of field `VDD_LVWF`"]
pub type VDD_LVWF_R = crate::R<bool, VDD_LVWF_A>;
impl VDD_LVWF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VDD_LVWF_A {
    match self.bits {
      false => VDD_LVWF_A::VDD_LVWF_0,
      true => VDD_LVWF_A::VDD_LVWF_1,
    }
  }
  #[doc = "Checks if the value of the field is `VDD_LVWF_0`"]
  #[inline(always)]
  pub fn is_vdd_lvwf_0(&self) -> bool {
    *self == VDD_LVWF_A::VDD_LVWF_0
  }
  #[doc = "Checks if the value of the field is `VDD_LVWF_1`"]
  #[inline(always)]
  pub fn is_vdd_lvwf_1(&self) -> bool {
    *self == VDD_LVWF_A::VDD_LVWF_1
  }
}
impl R {
  #[doc = "Bits 16:17 - VDD Low-Voltage Warning Voltage Select"]
  #[inline(always)]
  pub fn vdd_lvwv(&self) -> VDD_LVWV_R {
    VDD_LVWV_R::new(((self.bits >> 16) & 0x03) as u8)
  }
  #[doc = "Bit 21 - VDD Low-Voltage Warning Interrupt Enable"]
  #[inline(always)]
  pub fn vdd_lvwie(&self) -> VDD_LVWIE_R {
    VDD_LVWIE_R::new(((self.bits >> 21) & 0x01) != 0)
  }
  #[doc = "Bit 23 - VDD Low-Voltage Warning Flag"]
  #[inline(always)]
  pub fn vdd_lvwf(&self) -> VDD_LVWF_R {
    VDD_LVWF_R::new(((self.bits >> 23) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 16:17 - VDD Low-Voltage Warning Voltage Select"]
  #[inline(always)]
  pub fn vdd_lvwv(&mut self) -> VDD_LVWV_W {
    VDD_LVWV_W { w: self }
  }
  #[doc = "Bit 21 - VDD Low-Voltage Warning Interrupt Enable"]
  #[inline(always)]
  pub fn vdd_lvwie(&mut self) -> VDD_LVWIE_W {
    VDD_LVWIE_W { w: self }
  }
  #[doc = "Bit 22 - VDD Low-Voltage Warning Acknowledge"]
  #[inline(always)]
  pub fn vdd_lvwack(&mut self) -> VDD_LVWACK_W {
    VDD_LVWACK_W { w: self }
  }
}
