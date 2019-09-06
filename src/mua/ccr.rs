#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Writer for register CCR"]
pub type W = crate::W<u32, super::CCR>;
#[doc = "Register CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "HR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HR_A {
  #[doc = "0: De-assert Hardware reset to the Processor B. (default)"]
  HR_0,
  #[doc = "1: Assert Hardware reset to the Processor B."]
  HR_1,
}
impl From<HR_A> for bool {
  #[inline(always)]
  fn from(variant: HR_A) -> Self {
    match variant {
      HR_A::HR_0 => false,
      HR_A::HR_1 => true,
    }
  }
}
#[doc = "Reader of field `HR`"]
pub type HR_R = crate::R<bool, HR_A>;
impl HR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> HR_A {
    match self.bits {
      false => HR_A::HR_0,
      true => HR_A::HR_1,
    }
  }
  #[doc = "Checks if the value of the field is `HR_0`"]
  #[inline(always)]
  pub fn is_hr_0(&self) -> bool {
    *self == HR_A::HR_0
  }
  #[doc = "Checks if the value of the field is `HR_1`"]
  #[inline(always)]
  pub fn is_hr_1(&self) -> bool {
    *self == HR_A::HR_1
  }
}
#[doc = "Write proxy for field `HR`"]
pub struct HR_W<'a> {
  w: &'a mut W,
}
impl<'a> HR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: HR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "De-assert Hardware reset to the Processor B. (default)"]
  #[inline(always)]
  pub fn hr_0(self) -> &'a mut W {
    self.variant(HR_A::HR_0)
  }
  #[doc = "Assert Hardware reset to the Processor B."]
  #[inline(always)]
  pub fn hr_1(self) -> &'a mut W {
    self.variant(HR_A::HR_1)
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
#[doc = "When set, HR bit in MUB CCR has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRM_A {
  #[doc = "0: HR bit in MUB CCR is not masked, enables the hardware reset to the Processor A (default after hardware reset)."]
  HRM_0,
  #[doc = "1: HR bit in MUB CCR is masked, disables the hardware reset request to the Processor A."]
  HRM_1,
}
impl From<HRM_A> for bool {
  #[inline(always)]
  fn from(variant: HRM_A) -> Self {
    match variant {
      HRM_A::HRM_0 => false,
      HRM_A::HRM_1 => true,
    }
  }
}
#[doc = "Reader of field `HRM`"]
pub type HRM_R = crate::R<bool, HRM_A>;
impl HRM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> HRM_A {
    match self.bits {
      false => HRM_A::HRM_0,
      true => HRM_A::HRM_1,
    }
  }
  #[doc = "Checks if the value of the field is `HRM_0`"]
  #[inline(always)]
  pub fn is_hrm_0(&self) -> bool {
    *self == HRM_A::HRM_0
  }
  #[doc = "Checks if the value of the field is `HRM_1`"]
  #[inline(always)]
  pub fn is_hrm_1(&self) -> bool {
    *self == HRM_A::HRM_1
  }
}
#[doc = "Write proxy for field `HRM`"]
pub struct HRM_W<'a> {
  w: &'a mut W,
}
impl<'a> HRM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: HRM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "HR bit in MUB CCR is not masked, enables the hardware reset to the Processor A (default after hardware reset)."]
  #[inline(always)]
  pub fn hrm_0(self) -> &'a mut W {
    self.variant(HRM_A::HRM_0)
  }
  #[doc = "HR bit in MUB CCR is masked, disables the hardware reset request to the Processor A."]
  #[inline(always)]
  pub fn hrm_1(self) -> &'a mut W {
    self.variant(HRM_A::HRM_1)
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
#[doc = "Processor B Reset Hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTH_A {
  #[doc = "0: Release Processor B from reset"]
  RSTH_0,
  #[doc = "1: Hold Processor B in reset"]
  RSTH_1,
}
impl From<RSTH_A> for bool {
  #[inline(always)]
  fn from(variant: RSTH_A) -> Self {
    match variant {
      RSTH_A::RSTH_0 => false,
      RSTH_A::RSTH_1 => true,
    }
  }
}
#[doc = "Reader of field `RSTH`"]
pub type RSTH_R = crate::R<bool, RSTH_A>;
impl RSTH_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RSTH_A {
    match self.bits {
      false => RSTH_A::RSTH_0,
      true => RSTH_A::RSTH_1,
    }
  }
  #[doc = "Checks if the value of the field is `RSTH_0`"]
  #[inline(always)]
  pub fn is_rsth_0(&self) -> bool {
    *self == RSTH_A::RSTH_0
  }
  #[doc = "Checks if the value of the field is `RSTH_1`"]
  #[inline(always)]
  pub fn is_rsth_1(&self) -> bool {
    *self == RSTH_A::RSTH_1
  }
}
#[doc = "Write proxy for field `RSTH`"]
pub struct RSTH_W<'a> {
  w: &'a mut W,
}
impl<'a> RSTH_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RSTH_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Release Processor B from reset"]
  #[inline(always)]
  pub fn rsth_0(self) -> &'a mut W {
    self.variant(RSTH_A::RSTH_0)
  }
  #[doc = "Hold Processor B in reset"]
  #[inline(always)]
  pub fn rsth_1(self) -> &'a mut W {
    self.variant(RSTH_A::RSTH_1)
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
#[doc = "MUB clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKE_A {
  #[doc = "0: MUB platform clock gated when MUB-side enters a stop mode."]
  CLKE_0,
  #[doc = "1: MUB platform clock kept running after MUB-side enters a stop mode, until MUA also enters a stop mode."]
  CLKE_1,
}
impl From<CLKE_A> for bool {
  #[inline(always)]
  fn from(variant: CLKE_A) -> Self {
    match variant {
      CLKE_A::CLKE_0 => false,
      CLKE_A::CLKE_1 => true,
    }
  }
}
#[doc = "Reader of field `CLKE`"]
pub type CLKE_R = crate::R<bool, CLKE_A>;
impl CLKE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CLKE_A {
    match self.bits {
      false => CLKE_A::CLKE_0,
      true => CLKE_A::CLKE_1,
    }
  }
  #[doc = "Checks if the value of the field is `CLKE_0`"]
  #[inline(always)]
  pub fn is_clke_0(&self) -> bool {
    *self == CLKE_A::CLKE_0
  }
  #[doc = "Checks if the value of the field is `CLKE_1`"]
  #[inline(always)]
  pub fn is_clke_1(&self) -> bool {
    *self == CLKE_A::CLKE_1
  }
}
#[doc = "Write proxy for field `CLKE`"]
pub struct CLKE_W<'a> {
  w: &'a mut W,
}
impl<'a> CLKE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CLKE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "MUB platform clock gated when MUB-side enters a stop mode."]
  #[inline(always)]
  pub fn clke_0(self) -> &'a mut W {
    self.variant(CLKE_A::CLKE_0)
  }
  #[doc = "MUB platform clock kept running after MUB-side enters a stop mode, until MUA also enters a stop mode."]
  #[inline(always)]
  pub fn clke_1(self) -> &'a mut W {
    self.variant(CLKE_A::CLKE_1)
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
#[doc = "Slave Processor B Boot Config.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_A {
  #[doc = "0: Boot from Dflash base"]
  BOOT_0,
  #[doc = "2: Boot from CM0+ RAM base"]
  BOOT_2,
}
impl From<BOOT_A> for u8 {
  #[inline(always)]
  fn from(variant: BOOT_A) -> Self {
    match variant {
      BOOT_A::BOOT_0 => 0,
      BOOT_A::BOOT_2 => 2,
    }
  }
}
#[doc = "Reader of field `BOOT`"]
pub type BOOT_R = crate::R<u8, BOOT_A>;
impl BOOT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, BOOT_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(BOOT_A::BOOT_0),
      2 => Val(BOOT_A::BOOT_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `BOOT_0`"]
  #[inline(always)]
  pub fn is_boot_0(&self) -> bool {
    *self == BOOT_A::BOOT_0
  }
  #[doc = "Checks if the value of the field is `BOOT_2`"]
  #[inline(always)]
  pub fn is_boot_2(&self) -> bool {
    *self == BOOT_A::BOOT_2
  }
}
#[doc = "Write proxy for field `BOOT`"]
pub struct BOOT_W<'a> {
  w: &'a mut W,
}
impl<'a> BOOT_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: BOOT_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Boot from Dflash base"]
  #[inline(always)]
  pub fn boot_0(self) -> &'a mut W {
    self.variant(BOOT_A::BOOT_0)
  }
  #[doc = "Boot from CM0+ RAM base"]
  #[inline(always)]
  pub fn boot_2(self) -> &'a mut W {
    self.variant(BOOT_A::BOOT_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - HR"]
  #[inline(always)]
  pub fn hr(&self) -> HR_R {
    HR_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - When set, HR bit in MUB CCR has no effect"]
  #[inline(always)]
  pub fn hrm(&self) -> HRM_R {
    HRM_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Processor B Reset Hold"]
  #[inline(always)]
  pub fn rsth(&self) -> RSTH_R {
    RSTH_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - MUB clock enable"]
  #[inline(always)]
  pub fn clke(&self) -> CLKE_R {
    CLKE_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bits 4:5 - Slave Processor B Boot Config."]
  #[inline(always)]
  pub fn boot(&self) -> BOOT_R {
    BOOT_R::new(((self.bits >> 4) & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bit 0 - HR"]
  #[inline(always)]
  pub fn hr(&mut self) -> HR_W {
    HR_W { w: self }
  }
  #[doc = "Bit 1 - When set, HR bit in MUB CCR has no effect"]
  #[inline(always)]
  pub fn hrm(&mut self) -> HRM_W {
    HRM_W { w: self }
  }
  #[doc = "Bit 2 - Processor B Reset Hold"]
  #[inline(always)]
  pub fn rsth(&mut self) -> RSTH_W {
    RSTH_W { w: self }
  }
  #[doc = "Bit 3 - MUB clock enable"]
  #[inline(always)]
  pub fn clke(&mut self) -> CLKE_W {
    CLKE_W { w: self }
  }
  #[doc = "Bits 4:5 - Slave Processor B Boot Config."]
  #[inline(always)]
  pub fn boot(&mut self) -> BOOT_W {
    BOOT_W { w: self }
  }
}
