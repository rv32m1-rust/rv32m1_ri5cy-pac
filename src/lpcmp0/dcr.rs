#[doc = "Reader of register DCR"]
pub type R = crate::R<u32, super::DCR>;
#[doc = "Writer for register DCR"]
pub type W = crate::W<u32, super::DCR>;
#[doc = "Register DCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DCR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "DAC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC_EN_A {
  #[doc = "0: DAC is disabled."]
  DAC_EN_0,
  #[doc = "1: DAC is enabled."]
  DAC_EN_1,
}
impl From<DAC_EN_A> for bool {
  #[inline(always)]
  fn from(variant: DAC_EN_A) -> Self {
    match variant {
      DAC_EN_A::DAC_EN_0 => false,
      DAC_EN_A::DAC_EN_1 => true,
    }
  }
}
#[doc = "Reader of field `DAC_EN`"]
pub type DAC_EN_R = crate::R<bool, DAC_EN_A>;
impl DAC_EN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DAC_EN_A {
    match self.bits {
      false => DAC_EN_A::DAC_EN_0,
      true => DAC_EN_A::DAC_EN_1,
    }
  }
  #[doc = "Checks if the value of the field is `DAC_EN_0`"]
  #[inline(always)]
  pub fn is_dac_en_0(&self) -> bool {
    *self == DAC_EN_A::DAC_EN_0
  }
  #[doc = "Checks if the value of the field is `DAC_EN_1`"]
  #[inline(always)]
  pub fn is_dac_en_1(&self) -> bool {
    *self == DAC_EN_A::DAC_EN_1
  }
}
#[doc = "Write proxy for field `DAC_EN`"]
pub struct DAC_EN_W<'a> {
  w: &'a mut W,
}
impl<'a> DAC_EN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DAC_EN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "DAC is disabled."]
  #[inline(always)]
  pub fn dac_en_0(self) -> &'a mut W {
    self.variant(DAC_EN_A::DAC_EN_0)
  }
  #[doc = "DAC is enabled."]
  #[inline(always)]
  pub fn dac_en_1(self) -> &'a mut W {
    self.variant(DAC_EN_A::DAC_EN_1)
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
#[doc = "DAC High Power Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC_HPMD_A {
  #[doc = "0: DAC high power mode is not enabled."]
  DAC_HPMD_0,
  #[doc = "1: DAC high power mode is enabled."]
  DAC_HPMD_1,
}
impl From<DAC_HPMD_A> for bool {
  #[inline(always)]
  fn from(variant: DAC_HPMD_A) -> Self {
    match variant {
      DAC_HPMD_A::DAC_HPMD_0 => false,
      DAC_HPMD_A::DAC_HPMD_1 => true,
    }
  }
}
#[doc = "Reader of field `DAC_HPMD`"]
pub type DAC_HPMD_R = crate::R<bool, DAC_HPMD_A>;
impl DAC_HPMD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DAC_HPMD_A {
    match self.bits {
      false => DAC_HPMD_A::DAC_HPMD_0,
      true => DAC_HPMD_A::DAC_HPMD_1,
    }
  }
  #[doc = "Checks if the value of the field is `DAC_HPMD_0`"]
  #[inline(always)]
  pub fn is_dac_hpmd_0(&self) -> bool {
    *self == DAC_HPMD_A::DAC_HPMD_0
  }
  #[doc = "Checks if the value of the field is `DAC_HPMD_1`"]
  #[inline(always)]
  pub fn is_dac_hpmd_1(&self) -> bool {
    *self == DAC_HPMD_A::DAC_HPMD_1
  }
}
#[doc = "Write proxy for field `DAC_HPMD`"]
pub struct DAC_HPMD_W<'a> {
  w: &'a mut W,
}
impl<'a> DAC_HPMD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DAC_HPMD_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "DAC high power mode is not enabled."]
  #[inline(always)]
  pub fn dac_hpmd_0(self) -> &'a mut W {
    self.variant(DAC_HPMD_A::DAC_HPMD_0)
  }
  #[doc = "DAC high power mode is enabled."]
  #[inline(always)]
  pub fn dac_hpmd_1(self) -> &'a mut W {
    self.variant(DAC_HPMD_A::DAC_HPMD_1)
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
#[doc = "Supply Voltage Reference Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRSEL_A {
  #[doc = "0: vrefh_int is selected as resistor ladder network supply reference Vin."]
  VRSEL_0,
  #[doc = "1: vrefh_ext is selected as resistor ladder network supply reference Vin."]
  VRSEL_1,
}
impl From<VRSEL_A> for bool {
  #[inline(always)]
  fn from(variant: VRSEL_A) -> Self {
    match variant {
      VRSEL_A::VRSEL_0 => false,
      VRSEL_A::VRSEL_1 => true,
    }
  }
}
#[doc = "Reader of field `VRSEL`"]
pub type VRSEL_R = crate::R<bool, VRSEL_A>;
impl VRSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VRSEL_A {
    match self.bits {
      false => VRSEL_A::VRSEL_0,
      true => VRSEL_A::VRSEL_1,
    }
  }
  #[doc = "Checks if the value of the field is `VRSEL_0`"]
  #[inline(always)]
  pub fn is_vrsel_0(&self) -> bool {
    *self == VRSEL_A::VRSEL_0
  }
  #[doc = "Checks if the value of the field is `VRSEL_1`"]
  #[inline(always)]
  pub fn is_vrsel_1(&self) -> bool {
    *self == VRSEL_A::VRSEL_1
  }
}
#[doc = "Write proxy for field `VRSEL`"]
pub struct VRSEL_W<'a> {
  w: &'a mut W,
}
impl<'a> VRSEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VRSEL_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "vrefh_int is selected as resistor ladder network supply reference Vin."]
  #[inline(always)]
  pub fn vrsel_0(self) -> &'a mut W {
    self.variant(VRSEL_A::VRSEL_0)
  }
  #[doc = "vrefh_ext is selected as resistor ladder network supply reference Vin."]
  #[inline(always)]
  pub fn vrsel_1(self) -> &'a mut W {
    self.variant(VRSEL_A::VRSEL_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
    self.w
  }
}
#[doc = "Reader of field `DAC_DATA`"]
pub type DAC_DATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAC_DATA`"]
pub struct DAC_DATA_W<'a> {
  w: &'a mut W,
}
impl<'a> DAC_DATA_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - DAC Enable"]
  #[inline(always)]
  pub fn dac_en(&self) -> DAC_EN_R {
    DAC_EN_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - DAC High Power Mode Select"]
  #[inline(always)]
  pub fn dac_hpmd(&self) -> DAC_HPMD_R {
    DAC_HPMD_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Supply Voltage Reference Source Select"]
  #[inline(always)]
  pub fn vrsel(&self) -> VRSEL_R {
    VRSEL_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bits 16:21 - DAC Output Voltage Select"]
  #[inline(always)]
  pub fn dac_data(&self) -> DAC_DATA_R {
    DAC_DATA_R::new(((self.bits >> 16) & 0x3f) as u8)
  }
}
impl W {
  #[doc = "Bit 0 - DAC Enable"]
  #[inline(always)]
  pub fn dac_en(&mut self) -> DAC_EN_W {
    DAC_EN_W { w: self }
  }
  #[doc = "Bit 1 - DAC High Power Mode Select"]
  #[inline(always)]
  pub fn dac_hpmd(&mut self) -> DAC_HPMD_W {
    DAC_HPMD_W { w: self }
  }
  #[doc = "Bit 8 - Supply Voltage Reference Source Select"]
  #[inline(always)]
  pub fn vrsel(&mut self) -> VRSEL_W {
    VRSEL_W { w: self }
  }
  #[doc = "Bits 16:21 - DAC Output Voltage Select"]
  #[inline(always)]
  pub fn dac_data(&mut self) -> DAC_DATA_W {
    DAC_DATA_W { w: self }
  }
}
