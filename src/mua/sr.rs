#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0x00f0_0000"]
impl crate::ResetValue for super::SR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x00f0_0000
  }
}
#[doc = "Fn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FN_A {
  #[doc = "0: Fn bit in the MUB CR register is written 0 (default)."]
  FN_0,
  #[doc = "1: Fn bit in the MUB CR register is written 1."]
  FN_1,
}
impl From<FN_A> for u8 {
  #[inline(always)]
  fn from(variant: FN_A) -> Self {
    match variant {
      FN_A::FN_0 => 0,
      FN_A::FN_1 => 1,
    }
  }
}
#[doc = "Reader of field `Fn`"]
pub type FN_R = crate::R<u8, FN_A>;
impl FN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, FN_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(FN_A::FN_0),
      1 => Val(FN_A::FN_1),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `FN_0`"]
  #[inline(always)]
  pub fn is_fn_0(&self) -> bool {
    *self == FN_A::FN_0
  }
  #[doc = "Checks if the value of the field is `FN_1`"]
  #[inline(always)]
  pub fn is_fn_1(&self) -> bool {
    *self == FN_A::FN_1
  }
}
#[doc = "NMIC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIC_A {
  #[doc = "0: Default"]
  NMIC_0,
  #[doc = "1: Writing \"1\" clears the NMI bit in the MUB CR register."]
  NMIC_1,
}
impl From<NMIC_A> for bool {
  #[inline(always)]
  fn from(variant: NMIC_A) -> Self {
    match variant {
      NMIC_A::NMIC_0 => false,
      NMIC_A::NMIC_1 => true,
    }
  }
}
#[doc = "Reader of field `NMIC`"]
pub type NMIC_R = crate::R<bool, NMIC_A>;
impl NMIC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> NMIC_A {
    match self.bits {
      false => NMIC_A::NMIC_0,
      true => NMIC_A::NMIC_1,
    }
  }
  #[doc = "Checks if the value of the field is `NMIC_0`"]
  #[inline(always)]
  pub fn is_nmic_0(&self) -> bool {
    *self == NMIC_A::NMIC_0
  }
  #[doc = "Checks if the value of the field is `NMIC_1`"]
  #[inline(always)]
  pub fn is_nmic_1(&self) -> bool {
    *self == NMIC_A::NMIC_1
  }
}
#[doc = "Write proxy for field `NMIC`"]
pub struct NMIC_W<'a> {
  w: &'a mut W,
}
impl<'a> NMIC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: NMIC_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Default"]
  #[inline(always)]
  pub fn nmic_0(self) -> &'a mut W {
    self.variant(NMIC_A::NMIC_0)
  }
  #[doc = "Writing \"1\" clears the NMI bit in the MUB CR register."]
  #[inline(always)]
  pub fn nmic_1(self) -> &'a mut W {
    self.variant(NMIC_A::NMIC_1)
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
#[doc = "EP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_A {
  #[doc = "0: The MUA side event is not pending (default)."]
  EP_0,
  #[doc = "1: The MUA side event is pending."]
  EP_1,
}
impl From<EP_A> for bool {
  #[inline(always)]
  fn from(variant: EP_A) -> Self {
    match variant {
      EP_A::EP_0 => false,
      EP_A::EP_1 => true,
    }
  }
}
#[doc = "Reader of field `EP`"]
pub type EP_R = crate::R<bool, EP_A>;
impl EP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EP_A {
    match self.bits {
      false => EP_A::EP_0,
      true => EP_A::EP_1,
    }
  }
  #[doc = "Checks if the value of the field is `EP_0`"]
  #[inline(always)]
  pub fn is_ep_0(&self) -> bool {
    *self == EP_A::EP_0
  }
  #[doc = "Checks if the value of the field is `EP_1`"]
  #[inline(always)]
  pub fn is_ep_1(&self) -> bool {
    *self == EP_A::EP_1
  }
}
#[doc = "HRIP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRIP_A {
  #[doc = "0: MUB didn't issue hardware reset to Processor A"]
  HRIP_0,
  #[doc = "1: MUB had initiated a hardware reset to Processor A through HR bit."]
  HRIP_1,
}
impl From<HRIP_A> for bool {
  #[inline(always)]
  fn from(variant: HRIP_A) -> Self {
    match variant {
      HRIP_A::HRIP_0 => false,
      HRIP_A::HRIP_1 => true,
    }
  }
}
#[doc = "Reader of field `HRIP`"]
pub type HRIP_R = crate::R<bool, HRIP_A>;
impl HRIP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> HRIP_A {
    match self.bits {
      false => HRIP_A::HRIP_0,
      true => HRIP_A::HRIP_1,
    }
  }
  #[doc = "Checks if the value of the field is `HRIP_0`"]
  #[inline(always)]
  pub fn is_hrip_0(&self) -> bool {
    *self == HRIP_A::HRIP_0
  }
  #[doc = "Checks if the value of the field is `HRIP_1`"]
  #[inline(always)]
  pub fn is_hrip_1(&self) -> bool {
    *self == HRIP_A::HRIP_1
  }
}
#[doc = "Write proxy for field `HRIP`"]
pub struct HRIP_W<'a> {
  w: &'a mut W,
}
impl<'a> HRIP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: HRIP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "MUB didn't issue hardware reset to Processor A"]
  #[inline(always)]
  pub fn hrip_0(self) -> &'a mut W {
    self.variant(HRIP_A::HRIP_0)
  }
  #[doc = "MUB had initiated a hardware reset to Processor A through HR bit."]
  #[inline(always)]
  pub fn hrip_1(self) -> &'a mut W {
    self.variant(HRIP_A::HRIP_1)
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
#[doc = "FUP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUP_A {
  #[doc = "0: No flags updated, initiated by the MUA, in progress (default)"]
  FUP_0,
  #[doc = "1: MUA initiated flags update, processing"]
  FUP_1,
}
impl From<FUP_A> for bool {
  #[inline(always)]
  fn from(variant: FUP_A) -> Self {
    match variant {
      FUP_A::FUP_0 => false,
      FUP_A::FUP_1 => true,
    }
  }
}
#[doc = "Reader of field `FUP`"]
pub type FUP_R = crate::R<bool, FUP_A>;
impl FUP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FUP_A {
    match self.bits {
      false => FUP_A::FUP_0,
      true => FUP_A::FUP_1,
    }
  }
  #[doc = "Checks if the value of the field is `FUP_0`"]
  #[inline(always)]
  pub fn is_fup_0(&self) -> bool {
    *self == FUP_A::FUP_0
  }
  #[doc = "Checks if the value of the field is `FUP_1`"]
  #[inline(always)]
  pub fn is_fup_1(&self) -> bool {
    *self == FUP_A::FUP_1
  }
}
#[doc = "RDIP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDIP_A {
  #[doc = "0: Processor B did not exit reset"]
  RDIP_0,
  #[doc = "1: Processor B exited from reset"]
  RDIP_1,
}
impl From<RDIP_A> for bool {
  #[inline(always)]
  fn from(variant: RDIP_A) -> Self {
    match variant {
      RDIP_A::RDIP_0 => false,
      RDIP_A::RDIP_1 => true,
    }
  }
}
#[doc = "Reader of field `RDIP`"]
pub type RDIP_R = crate::R<bool, RDIP_A>;
impl RDIP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RDIP_A {
    match self.bits {
      false => RDIP_A::RDIP_0,
      true => RDIP_A::RDIP_1,
    }
  }
  #[doc = "Checks if the value of the field is `RDIP_0`"]
  #[inline(always)]
  pub fn is_rdip_0(&self) -> bool {
    *self == RDIP_A::RDIP_0
  }
  #[doc = "Checks if the value of the field is `RDIP_1`"]
  #[inline(always)]
  pub fn is_rdip_1(&self) -> bool {
    *self == RDIP_A::RDIP_1
  }
}
#[doc = "Write proxy for field `RDIP`"]
pub struct RDIP_W<'a> {
  w: &'a mut W,
}
impl<'a> RDIP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RDIP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Processor B did not exit reset"]
  #[inline(always)]
  pub fn rdip_0(self) -> &'a mut W {
    self.variant(RDIP_A::RDIP_0)
  }
  #[doc = "Processor B exited from reset"]
  #[inline(always)]
  pub fn rdip_1(self) -> &'a mut W {
    self.variant(RDIP_A::RDIP_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
    self.w
  }
}
#[doc = "RAIP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAIP_A {
  #[doc = "0: Processor B did not enter reset"]
  RAIP_0,
  #[doc = "1: Processor B entered reset"]
  RAIP_1,
}
impl From<RAIP_A> for bool {
  #[inline(always)]
  fn from(variant: RAIP_A) -> Self {
    match variant {
      RAIP_A::RAIP_0 => false,
      RAIP_A::RAIP_1 => true,
    }
  }
}
#[doc = "Reader of field `RAIP`"]
pub type RAIP_R = crate::R<bool, RAIP_A>;
impl RAIP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RAIP_A {
    match self.bits {
      false => RAIP_A::RAIP_0,
      true => RAIP_A::RAIP_1,
    }
  }
  #[doc = "Checks if the value of the field is `RAIP_0`"]
  #[inline(always)]
  pub fn is_raip_0(&self) -> bool {
    *self == RAIP_A::RAIP_0
  }
  #[doc = "Checks if the value of the field is `RAIP_1`"]
  #[inline(always)]
  pub fn is_raip_1(&self) -> bool {
    *self == RAIP_A::RAIP_1
  }
}
#[doc = "Write proxy for field `RAIP`"]
pub struct RAIP_W<'a> {
  w: &'a mut W,
}
impl<'a> RAIP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RAIP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Processor B did not enter reset"]
  #[inline(always)]
  pub fn raip_0(self) -> &'a mut W {
    self.variant(RAIP_A::RAIP_0)
  }
  #[doc = "Processor B entered reset"]
  #[inline(always)]
  pub fn raip_1(self) -> &'a mut W {
    self.variant(RAIP_A::RAIP_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
    self.w
  }
}
#[doc = "MURIP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MURIP_A {
  #[doc = "0: Processor B did not issue MU reset"]
  MURIP_0,
  #[doc = "1: Processor B issued MU reset"]
  MURIP_1,
}
impl From<MURIP_A> for bool {
  #[inline(always)]
  fn from(variant: MURIP_A) -> Self {
    match variant {
      MURIP_A::MURIP_0 => false,
      MURIP_A::MURIP_1 => true,
    }
  }
}
#[doc = "Reader of field `MURIP`"]
pub type MURIP_R = crate::R<bool, MURIP_A>;
impl MURIP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MURIP_A {
    match self.bits {
      false => MURIP_A::MURIP_0,
      true => MURIP_A::MURIP_1,
    }
  }
  #[doc = "Checks if the value of the field is `MURIP_0`"]
  #[inline(always)]
  pub fn is_murip_0(&self) -> bool {
    *self == MURIP_A::MURIP_0
  }
  #[doc = "Checks if the value of the field is `MURIP_1`"]
  #[inline(always)]
  pub fn is_murip_1(&self) -> bool {
    *self == MURIP_A::MURIP_1
  }
}
#[doc = "Write proxy for field `MURIP`"]
pub struct MURIP_W<'a> {
  w: &'a mut W,
}
impl<'a> MURIP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MURIP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Processor B did not issue MU reset"]
  #[inline(always)]
  pub fn murip_0(self) -> &'a mut W {
    self.variant(MURIP_A::MURIP_0)
  }
  #[doc = "Processor B issued MU reset"]
  #[inline(always)]
  pub fn murip_1(self) -> &'a mut W {
    self.variant(MURIP_A::MURIP_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
    self.w
  }
}
#[doc = "PM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PM_A {
  #[doc = "0: The MUB processor is in Run Mode."]
  RUN,
  #[doc = "1: The MUB processor is in COO Mode."]
  COO,
  #[doc = "2: The MUB processor is in WAIT Mode."]
  WAIT,
  #[doc = "3: no description available"]
  STOP,
  #[doc = "4: no description available"]
  DSM,
}
impl From<PM_A> for u8 {
  #[inline(always)]
  fn from(variant: PM_A) -> Self {
    match variant {
      PM_A::RUN => 0,
      PM_A::COO => 1,
      PM_A::WAIT => 2,
      PM_A::STOP => 3,
      PM_A::DSM => 4,
    }
  }
}
#[doc = "Reader of field `PM`"]
pub type PM_R = crate::R<u8, PM_A>;
impl PM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, PM_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(PM_A::RUN),
      1 => Val(PM_A::COO),
      2 => Val(PM_A::WAIT),
      3 => Val(PM_A::STOP),
      4 => Val(PM_A::DSM),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `RUN`"]
  #[inline(always)]
  pub fn is_run(&self) -> bool {
    *self == PM_A::RUN
  }
  #[doc = "Checks if the value of the field is `COO`"]
  #[inline(always)]
  pub fn is_coo(&self) -> bool {
    *self == PM_A::COO
  }
  #[doc = "Checks if the value of the field is `WAIT`"]
  #[inline(always)]
  pub fn is_wait(&self) -> bool {
    *self == PM_A::WAIT
  }
  #[doc = "Checks if the value of the field is `STOP`"]
  #[inline(always)]
  pub fn is_stop(&self) -> bool {
    *self == PM_A::STOP
  }
  #[doc = "Checks if the value of the field is `DSM`"]
  #[inline(always)]
  pub fn is_dsm(&self) -> bool {
    *self == PM_A::DSM
  }
}
#[doc = "TEn\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN_A {
  #[doc = "0: MUA TRn register is not empty."]
  TEN_0,
  #[doc = "1: MUA TRn register is empty (default)."]
  TEN_1,
}
impl From<TEN_A> for u8 {
  #[inline(always)]
  fn from(variant: TEN_A) -> Self {
    match variant {
      TEN_A::TEN_0 => 0,
      TEN_A::TEN_1 => 1,
    }
  }
}
#[doc = "Reader of field `TEn`"]
pub type TEN_R = crate::R<u8, TEN_A>;
impl TEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, TEN_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(TEN_A::TEN_0),
      1 => Val(TEN_A::TEN_1),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `TEN_0`"]
  #[inline(always)]
  pub fn is_ten_0(&self) -> bool {
    *self == TEN_A::TEN_0
  }
  #[doc = "Checks if the value of the field is `TEN_1`"]
  #[inline(always)]
  pub fn is_ten_1(&self) -> bool {
    *self == TEN_A::TEN_1
  }
}
#[doc = "RFn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFN_A {
  #[doc = "0: MUA RRn register is not full (default)."]
  RFN_0,
  #[doc = "1: MUA RRn register has received data from MUB TRn register and is ready to be read by the MUA."]
  RFN_1,
}
impl From<RFN_A> for u8 {
  #[inline(always)]
  fn from(variant: RFN_A) -> Self {
    match variant {
      RFN_A::RFN_0 => 0,
      RFN_A::RFN_1 => 1,
    }
  }
}
#[doc = "Reader of field `RFn`"]
pub type RFN_R = crate::R<u8, RFN_A>;
impl RFN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, RFN_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(RFN_A::RFN_0),
      1 => Val(RFN_A::RFN_1),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `RFN_0`"]
  #[inline(always)]
  pub fn is_rfn_0(&self) -> bool {
    *self == RFN_A::RFN_0
  }
  #[doc = "Checks if the value of the field is `RFN_1`"]
  #[inline(always)]
  pub fn is_rfn_1(&self) -> bool {
    *self == RFN_A::RFN_1
  }
}
#[doc = "GIPn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIPN_A {
  #[doc = "0: MUA general purpose interrupt n is not pending. (default)"]
  GIPN_0,
  #[doc = "1: MUA general purpose interrupt n is pending."]
  GIPN_1,
}
impl From<GIPN_A> for u8 {
  #[inline(always)]
  fn from(variant: GIPN_A) -> Self {
    match variant {
      GIPN_A::GIPN_0 => 0,
      GIPN_A::GIPN_1 => 1,
    }
  }
}
#[doc = "Reader of field `GIPn`"]
pub type GIPN_R = crate::R<u8, GIPN_A>;
impl GIPN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, GIPN_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(GIPN_A::GIPN_0),
      1 => Val(GIPN_A::GIPN_1),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `GIPN_0`"]
  #[inline(always)]
  pub fn is_gipn_0(&self) -> bool {
    *self == GIPN_A::GIPN_0
  }
  #[doc = "Checks if the value of the field is `GIPN_1`"]
  #[inline(always)]
  pub fn is_gipn_1(&self) -> bool {
    *self == GIPN_A::GIPN_1
  }
}
impl R {
  #[doc = "Bits 0:2 - Fn"]
  #[inline(always)]
  pub fn fn_(&self) -> FN_R {
    FN_R::new((self.bits & 0x07) as u8)
  }
  #[doc = "Bit 3 - NMIC"]
  #[inline(always)]
  pub fn nmic(&self) -> NMIC_R {
    NMIC_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - EP"]
  #[inline(always)]
  pub fn ep(&self) -> EP_R {
    EP_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 7 - HRIP"]
  #[inline(always)]
  pub fn hrip(&self) -> HRIP_R {
    HRIP_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 8 - FUP"]
  #[inline(always)]
  pub fn fup(&self) -> FUP_R {
    FUP_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - RDIP"]
  #[inline(always)]
  pub fn rdip(&self) -> RDIP_R {
    RDIP_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - RAIP"]
  #[inline(always)]
  pub fn raip(&self) -> RAIP_R {
    RAIP_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 11 - MURIP"]
  #[inline(always)]
  pub fn murip(&self) -> MURIP_R {
    MURIP_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bits 12:14 - PM"]
  #[inline(always)]
  pub fn pm(&self) -> PM_R {
    PM_R::new(((self.bits >> 12) & 0x07) as u8)
  }
  #[doc = "Bits 20:23 - TEn"]
  #[inline(always)]
  pub fn ten(&self) -> TEN_R {
    TEN_R::new(((self.bits >> 20) & 0x0f) as u8)
  }
  #[doc = "Bits 24:27 - RFn"]
  #[inline(always)]
  pub fn rfn(&self) -> RFN_R {
    RFN_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
  #[doc = "Bits 28:31 - GIPn"]
  #[inline(always)]
  pub fn gipn(&self) -> GIPN_R {
    GIPN_R::new(((self.bits >> 28) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bit 3 - NMIC"]
  #[inline(always)]
  pub fn nmic(&mut self) -> NMIC_W {
    NMIC_W { w: self }
  }
  #[doc = "Bit 7 - HRIP"]
  #[inline(always)]
  pub fn hrip(&mut self) -> HRIP_W {
    HRIP_W { w: self }
  }
  #[doc = "Bit 9 - RDIP"]
  #[inline(always)]
  pub fn rdip(&mut self) -> RDIP_W {
    RDIP_W { w: self }
  }
  #[doc = "Bit 10 - RAIP"]
  #[inline(always)]
  pub fn raip(&mut self) -> RAIP_W {
    RAIP_W { w: self }
  }
  #[doc = "Bit 11 - MURIP"]
  #[inline(always)]
  pub fn murip(&mut self) -> MURIP_W {
    MURIP_W { w: self }
  }
}
