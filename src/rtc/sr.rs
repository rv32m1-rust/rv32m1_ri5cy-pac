#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x01
  }
}
#[doc = "Time Invalid Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF_A {
  #[doc = "0: Time is valid."]
  TIF_0,
  #[doc = "1: Time is invalid and time counter is read as zero."]
  TIF_1,
}
impl From<TIF_A> for bool {
  #[inline(always)]
  fn from(variant: TIF_A) -> Self {
    match variant {
      TIF_A::TIF_0 => false,
      TIF_A::TIF_1 => true,
    }
  }
}
#[doc = "Reader of field `TIF`"]
pub type TIF_R = crate::R<bool, TIF_A>;
impl TIF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TIF_A {
    match self.bits {
      false => TIF_A::TIF_0,
      true => TIF_A::TIF_1,
    }
  }
  #[doc = "Checks if the value of the field is `TIF_0`"]
  #[inline(always)]
  pub fn is_tif_0(&self) -> bool {
    *self == TIF_A::TIF_0
  }
  #[doc = "Checks if the value of the field is `TIF_1`"]
  #[inline(always)]
  pub fn is_tif_1(&self) -> bool {
    *self == TIF_A::TIF_1
  }
}
#[doc = "Time Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOF_A {
  #[doc = "0: Time overflow has not occurred."]
  TOF_0,
  #[doc = "1: Time overflow has occurred and time counter is read as zero."]
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
#[doc = "Time Alarm Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAF_A {
  #[doc = "0: Time alarm has not occurred."]
  TAF_0,
  #[doc = "1: Time alarm has occurred."]
  TAF_1,
}
impl From<TAF_A> for bool {
  #[inline(always)]
  fn from(variant: TAF_A) -> Self {
    match variant {
      TAF_A::TAF_0 => false,
      TAF_A::TAF_1 => true,
    }
  }
}
#[doc = "Reader of field `TAF`"]
pub type TAF_R = crate::R<bool, TAF_A>;
impl TAF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TAF_A {
    match self.bits {
      false => TAF_A::TAF_0,
      true => TAF_A::TAF_1,
    }
  }
  #[doc = "Checks if the value of the field is `TAF_0`"]
  #[inline(always)]
  pub fn is_taf_0(&self) -> bool {
    *self == TAF_A::TAF_0
  }
  #[doc = "Checks if the value of the field is `TAF_1`"]
  #[inline(always)]
  pub fn is_taf_1(&self) -> bool {
    *self == TAF_A::TAF_1
  }
}
#[doc = "Monotonic Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOF_A {
  #[doc = "0: Monotonic counter overflow has not occurred."]
  MOF_0,
  #[doc = "1: Monotonic counter overflow has occurred and monotonic counter is read as zero."]
  MOF_1,
}
impl From<MOF_A> for bool {
  #[inline(always)]
  fn from(variant: MOF_A) -> Self {
    match variant {
      MOF_A::MOF_0 => false,
      MOF_A::MOF_1 => true,
    }
  }
}
#[doc = "Reader of field `MOF`"]
pub type MOF_R = crate::R<bool, MOF_A>;
impl MOF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MOF_A {
    match self.bits {
      false => MOF_A::MOF_0,
      true => MOF_A::MOF_1,
    }
  }
  #[doc = "Checks if the value of the field is `MOF_0`"]
  #[inline(always)]
  pub fn is_mof_0(&self) -> bool {
    *self == MOF_A::MOF_0
  }
  #[doc = "Checks if the value of the field is `MOF_1`"]
  #[inline(always)]
  pub fn is_mof_1(&self) -> bool {
    *self == MOF_A::MOF_1
  }
}
#[doc = "Time Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCE_A {
  #[doc = "0: Time counter is disabled."]
  TCE_0,
  #[doc = "1: Time counter is enabled."]
  TCE_1,
}
impl From<TCE_A> for bool {
  #[inline(always)]
  fn from(variant: TCE_A) -> Self {
    match variant {
      TCE_A::TCE_0 => false,
      TCE_A::TCE_1 => true,
    }
  }
}
#[doc = "Reader of field `TCE`"]
pub type TCE_R = crate::R<bool, TCE_A>;
impl TCE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TCE_A {
    match self.bits {
      false => TCE_A::TCE_0,
      true => TCE_A::TCE_1,
    }
  }
  #[doc = "Checks if the value of the field is `TCE_0`"]
  #[inline(always)]
  pub fn is_tce_0(&self) -> bool {
    *self == TCE_A::TCE_0
  }
  #[doc = "Checks if the value of the field is `TCE_1`"]
  #[inline(always)]
  pub fn is_tce_1(&self) -> bool {
    *self == TCE_A::TCE_1
  }
}
#[doc = "Write proxy for field `TCE`"]
pub struct TCE_W<'a> {
  w: &'a mut W,
}
impl<'a> TCE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TCE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Time counter is disabled."]
  #[inline(always)]
  pub fn tce_0(self) -> &'a mut W {
    self.variant(TCE_A::TCE_0)
  }
  #[doc = "Time counter is enabled."]
  #[inline(always)]
  pub fn tce_1(self) -> &'a mut W {
    self.variant(TCE_A::TCE_1)
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
#[doc = "Tamper Interrupt Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIDF_A {
  #[doc = "0: Tamper interrupt has not asserted."]
  TIDF_0,
  #[doc = "1: Tamper interrupt has asserted."]
  TIDF_1,
}
impl From<TIDF_A> for bool {
  #[inline(always)]
  fn from(variant: TIDF_A) -> Self {
    match variant {
      TIDF_A::TIDF_0 => false,
      TIDF_A::TIDF_1 => true,
    }
  }
}
#[doc = "Reader of field `TIDF`"]
pub type TIDF_R = crate::R<bool, TIDF_A>;
impl TIDF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TIDF_A {
    match self.bits {
      false => TIDF_A::TIDF_0,
      true => TIDF_A::TIDF_1,
    }
  }
  #[doc = "Checks if the value of the field is `TIDF_0`"]
  #[inline(always)]
  pub fn is_tidf_0(&self) -> bool {
    *self == TIDF_A::TIDF_0
  }
  #[doc = "Checks if the value of the field is `TIDF_1`"]
  #[inline(always)]
  pub fn is_tidf_1(&self) -> bool {
    *self == TIDF_A::TIDF_1
  }
}
impl R {
  #[doc = "Bit 0 - Time Invalid Flag"]
  #[inline(always)]
  pub fn tif(&self) -> TIF_R {
    TIF_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Time Overflow Flag"]
  #[inline(always)]
  pub fn tof(&self) -> TOF_R {
    TOF_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Time Alarm Flag"]
  #[inline(always)]
  pub fn taf(&self) -> TAF_R {
    TAF_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Monotonic Overflow Flag"]
  #[inline(always)]
  pub fn mof(&self) -> MOF_R {
    MOF_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Time Counter Enable"]
  #[inline(always)]
  pub fn tce(&self) -> TCE_R {
    TCE_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Tamper Interrupt Detect Flag"]
  #[inline(always)]
  pub fn tidf(&self) -> TIDF_R {
    TIDF_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 4 - Time Counter Enable"]
  #[inline(always)]
  pub fn tce(&mut self) -> TCE_W {
    TCE_W { w: self }
  }
}
