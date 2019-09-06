#[doc = "Reader of register CSC"]
pub type R = crate::R<u32, super::CSC>;
#[doc = "Writer for register CSC"]
pub type W = crate::W<u32, super::CSC>;
#[doc = "Register CSC `reset()`'s with value 0"]
impl crate::ResetValue for super::CSC {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
  #[doc = "0: Disable DMA transfers."]
  DMA_0,
  #[doc = "1: Enable DMA transfers."]
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
  #[doc = "Disable DMA transfers."]
  #[inline(always)]
  pub fn dma_0(self) -> &'a mut W {
    self.variant(DMA_A::DMA_0)
  }
  #[doc = "Enable DMA transfers."]
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
    self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
    self.w
  }
}
#[doc = "Reader of field `ELSA`"]
pub type ELSA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ELSA`"]
pub struct ELSA_W<'a> {
  w: &'a mut W,
}
impl<'a> ELSA_W<'a> {
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
#[doc = "Reader of field `ELSB`"]
pub type ELSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ELSB`"]
pub struct ELSB_W<'a> {
  w: &'a mut W,
}
impl<'a> ELSB_W<'a> {
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
#[doc = "Reader of field `MSA`"]
pub type MSA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSA`"]
pub struct MSA_W<'a> {
  w: &'a mut W,
}
impl<'a> MSA_W<'a> {
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
#[doc = "Reader of field `MSB`"]
pub type MSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSB`"]
pub struct MSB_W<'a> {
  w: &'a mut W,
}
impl<'a> MSB_W<'a> {
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
#[doc = "Channel Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHIE_A {
  #[doc = "0: Disable channel interrupts."]
  CHIE_0,
  #[doc = "1: Enable channel interrupts."]
  CHIE_1,
}
impl From<CHIE_A> for bool {
  #[inline(always)]
  fn from(variant: CHIE_A) -> Self {
    match variant {
      CHIE_A::CHIE_0 => false,
      CHIE_A::CHIE_1 => true,
    }
  }
}
#[doc = "Reader of field `CHIE`"]
pub type CHIE_R = crate::R<bool, CHIE_A>;
impl CHIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CHIE_A {
    match self.bits {
      false => CHIE_A::CHIE_0,
      true => CHIE_A::CHIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `CHIE_0`"]
  #[inline(always)]
  pub fn is_chie_0(&self) -> bool {
    *self == CHIE_A::CHIE_0
  }
  #[doc = "Checks if the value of the field is `CHIE_1`"]
  #[inline(always)]
  pub fn is_chie_1(&self) -> bool {
    *self == CHIE_A::CHIE_1
  }
}
#[doc = "Write proxy for field `CHIE`"]
pub struct CHIE_W<'a> {
  w: &'a mut W,
}
impl<'a> CHIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CHIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable channel interrupts."]
  #[inline(always)]
  pub fn chie_0(self) -> &'a mut W {
    self.variant(CHIE_A::CHIE_0)
  }
  #[doc = "Enable channel interrupts."]
  #[inline(always)]
  pub fn chie_1(self) -> &'a mut W {
    self.variant(CHIE_A::CHIE_1)
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
#[doc = "Channel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHF_A {
  #[doc = "0: No channel event has occurred."]
  CHF_0,
  #[doc = "1: A channel event has occurred."]
  CHF_1,
}
impl From<CHF_A> for bool {
  #[inline(always)]
  fn from(variant: CHF_A) -> Self {
    match variant {
      CHF_A::CHF_0 => false,
      CHF_A::CHF_1 => true,
    }
  }
}
#[doc = "Reader of field `CHF`"]
pub type CHF_R = crate::R<bool, CHF_A>;
impl CHF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CHF_A {
    match self.bits {
      false => CHF_A::CHF_0,
      true => CHF_A::CHF_1,
    }
  }
  #[doc = "Checks if the value of the field is `CHF_0`"]
  #[inline(always)]
  pub fn is_chf_0(&self) -> bool {
    *self == CHF_A::CHF_0
  }
  #[doc = "Checks if the value of the field is `CHF_1`"]
  #[inline(always)]
  pub fn is_chf_1(&self) -> bool {
    *self == CHF_A::CHF_1
  }
}
#[doc = "Write proxy for field `CHF`"]
pub struct CHF_W<'a> {
  w: &'a mut W,
}
impl<'a> CHF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CHF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No channel event has occurred."]
  #[inline(always)]
  pub fn chf_0(self) -> &'a mut W {
    self.variant(CHF_A::CHF_0)
  }
  #[doc = "A channel event has occurred."]
  #[inline(always)]
  pub fn chf_1(self) -> &'a mut W {
    self.variant(CHF_A::CHF_1)
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
impl R {
  #[doc = "Bit 0 - DMA Enable"]
  #[inline(always)]
  pub fn dma(&self) -> DMA_R {
    DMA_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 2 - Edge or Level Select"]
  #[inline(always)]
  pub fn elsa(&self) -> ELSA_R {
    ELSA_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Edge or Level Select"]
  #[inline(always)]
  pub fn elsb(&self) -> ELSB_R {
    ELSB_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Channel Mode Select"]
  #[inline(always)]
  pub fn msa(&self) -> MSA_R {
    MSA_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Channel Mode Select"]
  #[inline(always)]
  pub fn msb(&self) -> MSB_R {
    MSB_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Channel Interrupt Enable"]
  #[inline(always)]
  pub fn chie(&self) -> CHIE_R {
    CHIE_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Channel Flag"]
  #[inline(always)]
  pub fn chf(&self) -> CHF_R {
    CHF_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - DMA Enable"]
  #[inline(always)]
  pub fn dma(&mut self) -> DMA_W {
    DMA_W { w: self }
  }
  #[doc = "Bit 2 - Edge or Level Select"]
  #[inline(always)]
  pub fn elsa(&mut self) -> ELSA_W {
    ELSA_W { w: self }
  }
  #[doc = "Bit 3 - Edge or Level Select"]
  #[inline(always)]
  pub fn elsb(&mut self) -> ELSB_W {
    ELSB_W { w: self }
  }
  #[doc = "Bit 4 - Channel Mode Select"]
  #[inline(always)]
  pub fn msa(&mut self) -> MSA_W {
    MSA_W { w: self }
  }
  #[doc = "Bit 5 - Channel Mode Select"]
  #[inline(always)]
  pub fn msb(&mut self) -> MSB_W {
    MSB_W { w: self }
  }
  #[doc = "Bit 6 - Channel Interrupt Enable"]
  #[inline(always)]
  pub fn chie(&mut self) -> CHIE_W {
    CHIE_W { w: self }
  }
  #[doc = "Bit 7 - Channel Flag"]
  #[inline(always)]
  pub fn chf(&mut self) -> CHF_W {
    CHF_W { w: self }
  }
}
