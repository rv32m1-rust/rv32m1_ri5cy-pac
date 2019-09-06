#[doc = "Reader of register PCR[%s]"]
pub type R = crate::R<u32, super::PCR>;
#[doc = "Writer for register PCR[%s]"]
pub type W = crate::W<u32, super::PCR>;
#[doc = "Register PCR[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::PCR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Tamper Pull Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPE_A {
  #[doc = "0: Pull resistor is disabled on tamper pin."]
  TPE_0,
  #[doc = "1: Pull resistor is enabled on tamper pin."]
  TPE_1,
}
impl From<TPE_A> for bool {
  #[inline(always)]
  fn from(variant: TPE_A) -> Self {
    match variant {
      TPE_A::TPE_0 => false,
      TPE_A::TPE_1 => true,
    }
  }
}
#[doc = "Reader of field `TPE`"]
pub type TPE_R = crate::R<bool, TPE_A>;
impl TPE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TPE_A {
    match self.bits {
      false => TPE_A::TPE_0,
      true => TPE_A::TPE_1,
    }
  }
  #[doc = "Checks if the value of the field is `TPE_0`"]
  #[inline(always)]
  pub fn is_tpe_0(&self) -> bool {
    *self == TPE_A::TPE_0
  }
  #[doc = "Checks if the value of the field is `TPE_1`"]
  #[inline(always)]
  pub fn is_tpe_1(&self) -> bool {
    *self == TPE_A::TPE_1
  }
}
#[doc = "Write proxy for field `TPE`"]
pub struct TPE_W<'a> {
  w: &'a mut W,
}
impl<'a> TPE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TPE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Pull resistor is disabled on tamper pin."]
  #[inline(always)]
  pub fn tpe_0(self) -> &'a mut W {
    self.variant(TPE_A::TPE_0)
  }
  #[doc = "Pull resistor is enabled on tamper pin."]
  #[inline(always)]
  pub fn tpe_1(self) -> &'a mut W {
    self.variant(TPE_A::TPE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
    self.w
  }
}
#[doc = "Tamper Pull Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPS_A {
  #[doc = "0: Tamper pin pull resistor direction will assert the tamper pin."]
  TPS_0,
  #[doc = "1: Tamper pin pull resistor direction will negate the tamper pin."]
  TPS_1,
}
impl From<TPS_A> for bool {
  #[inline(always)]
  fn from(variant: TPS_A) -> Self {
    match variant {
      TPS_A::TPS_0 => false,
      TPS_A::TPS_1 => true,
    }
  }
}
#[doc = "Reader of field `TPS`"]
pub type TPS_R = crate::R<bool, TPS_A>;
impl TPS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TPS_A {
    match self.bits {
      false => TPS_A::TPS_0,
      true => TPS_A::TPS_1,
    }
  }
  #[doc = "Checks if the value of the field is `TPS_0`"]
  #[inline(always)]
  pub fn is_tps_0(&self) -> bool {
    *self == TPS_A::TPS_0
  }
  #[doc = "Checks if the value of the field is `TPS_1`"]
  #[inline(always)]
  pub fn is_tps_1(&self) -> bool {
    *self == TPS_A::TPS_1
  }
}
#[doc = "Write proxy for field `TPS`"]
pub struct TPS_W<'a> {
  w: &'a mut W,
}
impl<'a> TPS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TPS_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Tamper pin pull resistor direction will assert the tamper pin."]
  #[inline(always)]
  pub fn tps_0(self) -> &'a mut W {
    self.variant(TPS_A::TPS_0)
  }
  #[doc = "Tamper pin pull resistor direction will negate the tamper pin."]
  #[inline(always)]
  pub fn tps_1(self) -> &'a mut W {
    self.variant(TPS_A::TPS_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
    self.w
  }
}
#[doc = "Tamper Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFE_A {
  #[doc = "0: Input filter is disabled on the tamper pin."]
  TFE_0,
  #[doc = "1: Input filter is enabled on the tamper pin."]
  TFE_1,
}
impl From<TFE_A> for bool {
  #[inline(always)]
  fn from(variant: TFE_A) -> Self {
    match variant {
      TFE_A::TFE_0 => false,
      TFE_A::TFE_1 => true,
    }
  }
}
#[doc = "Reader of field `TFE`"]
pub type TFE_R = crate::R<bool, TFE_A>;
impl TFE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TFE_A {
    match self.bits {
      false => TFE_A::TFE_0,
      true => TFE_A::TFE_1,
    }
  }
  #[doc = "Checks if the value of the field is `TFE_0`"]
  #[inline(always)]
  pub fn is_tfe_0(&self) -> bool {
    *self == TFE_A::TFE_0
  }
  #[doc = "Checks if the value of the field is `TFE_1`"]
  #[inline(always)]
  pub fn is_tfe_1(&self) -> bool {
    *self == TFE_A::TFE_1
  }
}
#[doc = "Write proxy for field `TFE`"]
pub struct TFE_W<'a> {
  w: &'a mut W,
}
impl<'a> TFE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TFE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Input filter is disabled on the tamper pin."]
  #[inline(always)]
  pub fn tfe_0(self) -> &'a mut W {
    self.variant(TFE_A::TFE_0)
  }
  #[doc = "Input filter is enabled on the tamper pin."]
  #[inline(always)]
  pub fn tfe_1(self) -> &'a mut W {
    self.variant(TFE_A::TFE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
    self.w
  }
}
#[doc = "Tamper Pin Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPP_A {
  #[doc = "0: Tamper pin is active high."]
  TPP_0,
  #[doc = "1: Tamper pin is active low."]
  TPP_1,
}
impl From<TPP_A> for bool {
  #[inline(always)]
  fn from(variant: TPP_A) -> Self {
    match variant {
      TPP_A::TPP_0 => false,
      TPP_A::TPP_1 => true,
    }
  }
}
#[doc = "Reader of field `TPP`"]
pub type TPP_R = crate::R<bool, TPP_A>;
impl TPP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TPP_A {
    match self.bits {
      false => TPP_A::TPP_0,
      true => TPP_A::TPP_1,
    }
  }
  #[doc = "Checks if the value of the field is `TPP_0`"]
  #[inline(always)]
  pub fn is_tpp_0(&self) -> bool {
    *self == TPP_A::TPP_0
  }
  #[doc = "Checks if the value of the field is `TPP_1`"]
  #[inline(always)]
  pub fn is_tpp_1(&self) -> bool {
    *self == TPP_A::TPP_1
  }
}
#[doc = "Write proxy for field `TPP`"]
pub struct TPP_W<'a> {
  w: &'a mut W,
}
impl<'a> TPP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TPP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Tamper pin is active high."]
  #[inline(always)]
  pub fn tpp_0(self) -> &'a mut W {
    self.variant(TPP_A::TPP_0)
  }
  #[doc = "Tamper pin is active low."]
  #[inline(always)]
  pub fn tpp_1(self) -> &'a mut W {
    self.variant(TPP_A::TPP_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
    self.w
  }
}
#[doc = "Tamper Pin Input Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPID_A {
  #[doc = "0: Tamper pin input data is logic zero."]
  TPID_0,
  #[doc = "1: Tamper pin input data is logic one."]
  TPID_1,
}
impl From<TPID_A> for bool {
  #[inline(always)]
  fn from(variant: TPID_A) -> Self {
    match variant {
      TPID_A::TPID_0 => false,
      TPID_A::TPID_1 => true,
    }
  }
}
#[doc = "Reader of field `TPID`"]
pub type TPID_R = crate::R<bool, TPID_A>;
impl TPID_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TPID_A {
    match self.bits {
      false => TPID_A::TPID_0,
      true => TPID_A::TPID_1,
    }
  }
  #[doc = "Checks if the value of the field is `TPID_0`"]
  #[inline(always)]
  pub fn is_tpid_0(&self) -> bool {
    *self == TPID_A::TPID_0
  }
  #[doc = "Checks if the value of the field is `TPID_1`"]
  #[inline(always)]
  pub fn is_tpid_1(&self) -> bool {
    *self == TPID_A::TPID_1
  }
}
impl R {
  #[doc = "Bit 24 - Tamper Pull Enable"]
  #[inline(always)]
  pub fn tpe(&self) -> TPE_R {
    TPE_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - Tamper Pull Select"]
  #[inline(always)]
  pub fn tps(&self) -> TPS_R {
    TPS_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - Tamper Filter Enable"]
  #[inline(always)]
  pub fn tfe(&self) -> TFE_R {
    TFE_R::new(((self.bits >> 26) & 0x01) != 0)
  }
  #[doc = "Bit 27 - Tamper Pin Polarity"]
  #[inline(always)]
  pub fn tpp(&self) -> TPP_R {
    TPP_R::new(((self.bits >> 27) & 0x01) != 0)
  }
  #[doc = "Bit 31 - Tamper Pin Input Data"]
  #[inline(always)]
  pub fn tpid(&self) -> TPID_R {
    TPID_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 24 - Tamper Pull Enable"]
  #[inline(always)]
  pub fn tpe(&mut self) -> TPE_W {
    TPE_W { w: self }
  }
  #[doc = "Bit 25 - Tamper Pull Select"]
  #[inline(always)]
  pub fn tps(&mut self) -> TPS_W {
    TPS_W { w: self }
  }
  #[doc = "Bit 26 - Tamper Filter Enable"]
  #[inline(always)]
  pub fn tfe(&mut self) -> TFE_W {
    TFE_W { w: self }
  }
  #[doc = "Bit 27 - Tamper Pin Polarity"]
  #[inline(always)]
  pub fn tpp(&mut self) -> TPP_W {
    TPP_W { w: self }
  }
}
