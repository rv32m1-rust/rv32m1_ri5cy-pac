#[doc = "Reader of register SC"]
pub type R = crate::R<u32, super::SC>;
#[doc = "Writer for register SC"]
pub type W = crate::W<u32, super::SC>;
#[doc = "Register SC `reset()`'s with value 0"]
impl crate::ResetValue for super::SC {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Prescale Factor Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_A {
  #[doc = "0: Divide by 1"]
  PS_0,
  #[doc = "1: Divide by 2"]
  PS_1,
  #[doc = "2: Divide by 4"]
  PS_2,
  #[doc = "3: Divide by 8"]
  PS_3,
  #[doc = "4: Divide by 16"]
  PS_4,
  #[doc = "5: Divide by 32"]
  PS_5,
  #[doc = "6: Divide by 64"]
  PS_6,
  #[doc = "7: Divide by 128"]
  PS_7,
}
impl From<PS_A> for u8 {
  #[inline(always)]
  fn from(variant: PS_A) -> Self {
    match variant {
      PS_A::PS_0 => 0,
      PS_A::PS_1 => 1,
      PS_A::PS_2 => 2,
      PS_A::PS_3 => 3,
      PS_A::PS_4 => 4,
      PS_A::PS_5 => 5,
      PS_A::PS_6 => 6,
      PS_A::PS_7 => 7,
    }
  }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<u8, PS_A>;
impl PS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PS_A {
    match self.bits {
      0 => PS_A::PS_0,
      1 => PS_A::PS_1,
      2 => PS_A::PS_2,
      3 => PS_A::PS_3,
      4 => PS_A::PS_4,
      5 => PS_A::PS_5,
      6 => PS_A::PS_6,
      7 => PS_A::PS_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `PS_0`"]
  #[inline(always)]
  pub fn is_ps_0(&self) -> bool {
    *self == PS_A::PS_0
  }
  #[doc = "Checks if the value of the field is `PS_1`"]
  #[inline(always)]
  pub fn is_ps_1(&self) -> bool {
    *self == PS_A::PS_1
  }
  #[doc = "Checks if the value of the field is `PS_2`"]
  #[inline(always)]
  pub fn is_ps_2(&self) -> bool {
    *self == PS_A::PS_2
  }
  #[doc = "Checks if the value of the field is `PS_3`"]
  #[inline(always)]
  pub fn is_ps_3(&self) -> bool {
    *self == PS_A::PS_3
  }
  #[doc = "Checks if the value of the field is `PS_4`"]
  #[inline(always)]
  pub fn is_ps_4(&self) -> bool {
    *self == PS_A::PS_4
  }
  #[doc = "Checks if the value of the field is `PS_5`"]
  #[inline(always)]
  pub fn is_ps_5(&self) -> bool {
    *self == PS_A::PS_5
  }
  #[doc = "Checks if the value of the field is `PS_6`"]
  #[inline(always)]
  pub fn is_ps_6(&self) -> bool {
    *self == PS_A::PS_6
  }
  #[doc = "Checks if the value of the field is `PS_7`"]
  #[inline(always)]
  pub fn is_ps_7(&self) -> bool {
    *self == PS_A::PS_7
  }
}
#[doc = "Write proxy for field `PS`"]
pub struct PS_W<'a> {
  w: &'a mut W,
}
impl<'a> PS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PS_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Divide by 1"]
  #[inline(always)]
  pub fn ps_0(self) -> &'a mut W {
    self.variant(PS_A::PS_0)
  }
  #[doc = "Divide by 2"]
  #[inline(always)]
  pub fn ps_1(self) -> &'a mut W {
    self.variant(PS_A::PS_1)
  }
  #[doc = "Divide by 4"]
  #[inline(always)]
  pub fn ps_2(self) -> &'a mut W {
    self.variant(PS_A::PS_2)
  }
  #[doc = "Divide by 8"]
  #[inline(always)]
  pub fn ps_3(self) -> &'a mut W {
    self.variant(PS_A::PS_3)
  }
  #[doc = "Divide by 16"]
  #[inline(always)]
  pub fn ps_4(self) -> &'a mut W {
    self.variant(PS_A::PS_4)
  }
  #[doc = "Divide by 32"]
  #[inline(always)]
  pub fn ps_5(self) -> &'a mut W {
    self.variant(PS_A::PS_5)
  }
  #[doc = "Divide by 64"]
  #[inline(always)]
  pub fn ps_6(self) -> &'a mut W {
    self.variant(PS_A::PS_6)
  }
  #[doc = "Divide by 128"]
  #[inline(always)]
  pub fn ps_7(self) -> &'a mut W {
    self.variant(PS_A::PS_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
    self.w
  }
}
#[doc = "Clock Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMOD_A {
  #[doc = "0: TPM counter is disabled"]
  CMOD_0,
  #[doc = "1: TPM counter increments on every TPM counter clock"]
  CMOD_1,
  #[doc = "2: TPM counter increments on rising edge of TPM_EXTCLK synchronized to the TPM counter clock"]
  CMOD_2,
  #[doc = "3: no description available"]
  CMOD_3,
}
impl From<CMOD_A> for u8 {
  #[inline(always)]
  fn from(variant: CMOD_A) -> Self {
    match variant {
      CMOD_A::CMOD_0 => 0,
      CMOD_A::CMOD_1 => 1,
      CMOD_A::CMOD_2 => 2,
      CMOD_A::CMOD_3 => 3,
    }
  }
}
#[doc = "Reader of field `CMOD`"]
pub type CMOD_R = crate::R<u8, CMOD_A>;
impl CMOD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CMOD_A {
    match self.bits {
      0 => CMOD_A::CMOD_0,
      1 => CMOD_A::CMOD_1,
      2 => CMOD_A::CMOD_2,
      3 => CMOD_A::CMOD_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `CMOD_0`"]
  #[inline(always)]
  pub fn is_cmod_0(&self) -> bool {
    *self == CMOD_A::CMOD_0
  }
  #[doc = "Checks if the value of the field is `CMOD_1`"]
  #[inline(always)]
  pub fn is_cmod_1(&self) -> bool {
    *self == CMOD_A::CMOD_1
  }
  #[doc = "Checks if the value of the field is `CMOD_2`"]
  #[inline(always)]
  pub fn is_cmod_2(&self) -> bool {
    *self == CMOD_A::CMOD_2
  }
  #[doc = "Checks if the value of the field is `CMOD_3`"]
  #[inline(always)]
  pub fn is_cmod_3(&self) -> bool {
    *self == CMOD_A::CMOD_3
  }
}
#[doc = "Write proxy for field `CMOD`"]
pub struct CMOD_W<'a> {
  w: &'a mut W,
}
impl<'a> CMOD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CMOD_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "TPM counter is disabled"]
  #[inline(always)]
  pub fn cmod_0(self) -> &'a mut W {
    self.variant(CMOD_A::CMOD_0)
  }
  #[doc = "TPM counter increments on every TPM counter clock"]
  #[inline(always)]
  pub fn cmod_1(self) -> &'a mut W {
    self.variant(CMOD_A::CMOD_1)
  }
  #[doc = "TPM counter increments on rising edge of TPM_EXTCLK synchronized to the TPM counter clock"]
  #[inline(always)]
  pub fn cmod_2(self) -> &'a mut W {
    self.variant(CMOD_A::CMOD_2)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn cmod_3(self) -> &'a mut W {
    self.variant(CMOD_A::CMOD_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
    self.w
  }
}
#[doc = "Center-Aligned PWM Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPWMS_A {
  #[doc = "0: no description available"]
  CPWMS_0,
  #[doc = "1: no description available"]
  CPWMS_1,
}
impl From<CPWMS_A> for bool {
  #[inline(always)]
  fn from(variant: CPWMS_A) -> Self {
    match variant {
      CPWMS_A::CPWMS_0 => false,
      CPWMS_A::CPWMS_1 => true,
    }
  }
}
#[doc = "Reader of field `CPWMS`"]
pub type CPWMS_R = crate::R<bool, CPWMS_A>;
impl CPWMS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CPWMS_A {
    match self.bits {
      false => CPWMS_A::CPWMS_0,
      true => CPWMS_A::CPWMS_1,
    }
  }
  #[doc = "Checks if the value of the field is `CPWMS_0`"]
  #[inline(always)]
  pub fn is_cpwms_0(&self) -> bool {
    *self == CPWMS_A::CPWMS_0
  }
  #[doc = "Checks if the value of the field is `CPWMS_1`"]
  #[inline(always)]
  pub fn is_cpwms_1(&self) -> bool {
    *self == CPWMS_A::CPWMS_1
  }
}
#[doc = "Write proxy for field `CPWMS`"]
pub struct CPWMS_W<'a> {
  w: &'a mut W,
}
impl<'a> CPWMS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CPWMS_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn cpwms_0(self) -> &'a mut W {
    self.variant(CPWMS_A::CPWMS_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn cpwms_1(self) -> &'a mut W {
    self.variant(CPWMS_A::CPWMS_1)
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
#[doc = "Timer Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOIE_A {
  #[doc = "0: no description available"]
  TOIE_0,
  #[doc = "1: Enable TOF interrupts. An interrupt is generated when TOF equals one."]
  TOIE_1,
}
impl From<TOIE_A> for bool {
  #[inline(always)]
  fn from(variant: TOIE_A) -> Self {
    match variant {
      TOIE_A::TOIE_0 => false,
      TOIE_A::TOIE_1 => true,
    }
  }
}
#[doc = "Reader of field `TOIE`"]
pub type TOIE_R = crate::R<bool, TOIE_A>;
impl TOIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TOIE_A {
    match self.bits {
      false => TOIE_A::TOIE_0,
      true => TOIE_A::TOIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `TOIE_0`"]
  #[inline(always)]
  pub fn is_toie_0(&self) -> bool {
    *self == TOIE_A::TOIE_0
  }
  #[doc = "Checks if the value of the field is `TOIE_1`"]
  #[inline(always)]
  pub fn is_toie_1(&self) -> bool {
    *self == TOIE_A::TOIE_1
  }
}
#[doc = "Write proxy for field `TOIE`"]
pub struct TOIE_W<'a> {
  w: &'a mut W,
}
impl<'a> TOIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TOIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn toie_0(self) -> &'a mut W {
    self.variant(TOIE_A::TOIE_0)
  }
  #[doc = "Enable TOF interrupts. An interrupt is generated when TOF equals one."]
  #[inline(always)]
  pub fn toie_1(self) -> &'a mut W {
    self.variant(TOIE_A::TOIE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
    self.w
  }
}
#[doc = "Timer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOF_A {
  #[doc = "0: no description available"]
  TOF_0,
  #[doc = "1: no description available"]
  TOF_1,
}
impl From<TOF_A> for bool {
  #[inline(always)]
  fn from(variant: TOF_A) -> Self {
    match variant {
      TOF_A::TOF_0 => false,
      TOF_A::TOF_1 => true,
    }
  }
}
#[doc = "Reader of field `TOF`"]
pub type TOF_R = crate::R<bool, TOF_A>;
impl TOF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TOF_A {
    match self.bits {
      false => TOF_A::TOF_0,
      true => TOF_A::TOF_1,
    }
  }
  #[doc = "Checks if the value of the field is `TOF_0`"]
  #[inline(always)]
  pub fn is_tof_0(&self) -> bool {
    *self == TOF_A::TOF_0
  }
  #[doc = "Checks if the value of the field is `TOF_1`"]
  #[inline(always)]
  pub fn is_tof_1(&self) -> bool {
    *self == TOF_A::TOF_1
  }
}
#[doc = "Write proxy for field `TOF`"]
pub struct TOF_W<'a> {
  w: &'a mut W,
}
impl<'a> TOF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TOF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn tof_0(self) -> &'a mut W {
    self.variant(TOF_A::TOF_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn tof_1(self) -> &'a mut W {
    self.variant(TOF_A::TOF_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
    self.w
  }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
  #[doc = "0: Disables DMA transfers."]
  DMA_0,
  #[doc = "1: Enables DMA transfers."]
  DMA_1,
}
impl From<DMA_A> for bool {
  #[inline(always)]
  fn from(variant: DMA_A) -> Self {
    match variant {
      DMA_A::DMA_0 => false,
      DMA_A::DMA_1 => true,
    }
  }
}
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, DMA_A>;
impl DMA_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DMA_A {
    match self.bits {
      false => DMA_A::DMA_0,
      true => DMA_A::DMA_1,
    }
  }
  #[doc = "Checks if the value of the field is `DMA_0`"]
  #[inline(always)]
  pub fn is_dma_0(&self) -> bool {
    *self == DMA_A::DMA_0
  }
  #[doc = "Checks if the value of the field is `DMA_1`"]
  #[inline(always)]
  pub fn is_dma_1(&self) -> bool {
    *self == DMA_A::DMA_1
  }
}
#[doc = "Write proxy for field `DMA`"]
pub struct DMA_W<'a> {
  w: &'a mut W,
}
impl<'a> DMA_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DMA_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disables DMA transfers."]
  #[inline(always)]
  pub fn dma_0(self) -> &'a mut W {
    self.variant(DMA_A::DMA_0)
  }
  #[doc = "Enables DMA transfers."]
  #[inline(always)]
  pub fn dma_1(self) -> &'a mut W {
    self.variant(DMA_A::DMA_1)
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
impl R {
  #[doc = "Bits 0:2 - Prescale Factor Selection"]
  #[inline(always)]
  pub fn ps(&self) -> PS_R {
    PS_R::new((self.bits & 0x07) as u8)
  }
  #[doc = "Bits 3:4 - Clock Mode Selection"]
  #[inline(always)]
  pub fn cmod(&self) -> CMOD_R {
    CMOD_R::new(((self.bits >> 3) & 0x03) as u8)
  }
  #[doc = "Bit 5 - Center-Aligned PWM Select"]
  #[inline(always)]
  pub fn cpwms(&self) -> CPWMS_R {
    CPWMS_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Timer Overflow Interrupt Enable"]
  #[inline(always)]
  pub fn toie(&self) -> TOIE_R {
    TOIE_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Timer Overflow Flag"]
  #[inline(always)]
  pub fn tof(&self) -> TOF_R {
    TOF_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 8 - DMA Enable"]
  #[inline(always)]
  pub fn dma(&self) -> DMA_R {
    DMA_R::new(((self.bits >> 8) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:2 - Prescale Factor Selection"]
  #[inline(always)]
  pub fn ps(&mut self) -> PS_W {
    PS_W { w: self }
  }
  #[doc = "Bits 3:4 - Clock Mode Selection"]
  #[inline(always)]
  pub fn cmod(&mut self) -> CMOD_W {
    CMOD_W { w: self }
  }
  #[doc = "Bit 5 - Center-Aligned PWM Select"]
  #[inline(always)]
  pub fn cpwms(&mut self) -> CPWMS_W {
    CPWMS_W { w: self }
  }
  #[doc = "Bit 6 - Timer Overflow Interrupt Enable"]
  #[inline(always)]
  pub fn toie(&mut self) -> TOIE_W {
    TOIE_W { w: self }
  }
  #[doc = "Bit 7 - Timer Overflow Flag"]
  #[inline(always)]
  pub fn tof(&mut self) -> TOF_W {
    TOF_W { w: self }
  }
  #[doc = "Bit 8 - DMA Enable"]
  #[inline(always)]
  pub fn dma(&mut self) -> DMA_W {
    DMA_W { w: self }
  }
}
