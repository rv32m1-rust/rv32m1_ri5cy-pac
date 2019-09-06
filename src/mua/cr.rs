#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Fn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FN_A {
  #[doc = "0: Clears the Fn bit in the SR register."]
  FN_0,
  #[doc = "1: Sets the Fn bit in the SR register."]
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
#[doc = "Write proxy for field `Fn`"]
pub struct FN_W<'a> {
  w: &'a mut W,
}
impl<'a> FN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FN_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Clears the Fn bit in the SR register."]
  #[inline(always)]
  pub fn fn_0(self) -> &'a mut W {
    self.variant(FN_A::FN_0)
  }
  #[doc = "Sets the Fn bit in the SR register."]
  #[inline(always)]
  pub fn fn_1(self) -> &'a mut W {
    self.variant(FN_A::FN_1)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
    self.w
  }
}
#[doc = "NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMI_A {
  #[doc = "0: Non-maskable interrupt is not issued to the Processor B by the Processor A (default)."]
  NMI_0,
  #[doc = "1: Non-maskable interrupt is issued to the Processor B by the Processor A."]
  NMI_1,
}
impl From<NMI_A> for bool {
  #[inline(always)]
  fn from(variant: NMI_A) -> Self {
    match variant {
      NMI_A::NMI_0 => false,
      NMI_A::NMI_1 => true,
    }
  }
}
#[doc = "Reader of field `NMI`"]
pub type NMI_R = crate::R<bool, NMI_A>;
impl NMI_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> NMI_A {
    match self.bits {
      false => NMI_A::NMI_0,
      true => NMI_A::NMI_1,
    }
  }
  #[doc = "Checks if the value of the field is `NMI_0`"]
  #[inline(always)]
  pub fn is_nmi_0(&self) -> bool {
    *self == NMI_A::NMI_0
  }
  #[doc = "Checks if the value of the field is `NMI_1`"]
  #[inline(always)]
  pub fn is_nmi_1(&self) -> bool {
    *self == NMI_A::NMI_1
  }
}
#[doc = "Write proxy for field `NMI`"]
pub struct NMI_W<'a> {
  w: &'a mut W,
}
impl<'a> NMI_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: NMI_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Non-maskable interrupt is not issued to the Processor B by the Processor A (default)."]
  #[inline(always)]
  pub fn nmi_0(self) -> &'a mut W {
    self.variant(NMI_A::NMI_0)
  }
  #[doc = "Non-maskable interrupt is issued to the Processor B by the Processor A."]
  #[inline(always)]
  pub fn nmi_1(self) -> &'a mut W {
    self.variant(NMI_A::NMI_1)
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
#[doc = "MUR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUR_A {
  #[doc = "0: N/A. Self clearing bit (default)."]
  MUR_0,
  #[doc = "1: Asserts the MU reset."]
  MUR_1,
}
impl From<MUR_A> for bool {
  #[inline(always)]
  fn from(variant: MUR_A) -> Self {
    match variant {
      MUR_A::MUR_0 => false,
      MUR_A::MUR_1 => true,
    }
  }
}
#[doc = "Reader of field `MUR`"]
pub type MUR_R = crate::R<bool, MUR_A>;
impl MUR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MUR_A {
    match self.bits {
      false => MUR_A::MUR_0,
      true => MUR_A::MUR_1,
    }
  }
  #[doc = "Checks if the value of the field is `MUR_0`"]
  #[inline(always)]
  pub fn is_mur_0(&self) -> bool {
    *self == MUR_A::MUR_0
  }
  #[doc = "Checks if the value of the field is `MUR_1`"]
  #[inline(always)]
  pub fn is_mur_1(&self) -> bool {
    *self == MUR_A::MUR_1
  }
}
#[doc = "Write proxy for field `MUR`"]
pub struct MUR_W<'a> {
  w: &'a mut W,
}
impl<'a> MUR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MUR_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "N/A. Self clearing bit (default)."]
  #[inline(always)]
  pub fn mur_0(self) -> &'a mut W {
    self.variant(MUR_A::MUR_0)
  }
  #[doc = "Asserts the MU reset."]
  #[inline(always)]
  pub fn mur_1(self) -> &'a mut W {
    self.variant(MUR_A::MUR_1)
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
#[doc = "RDIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDIE_A {
  #[doc = "0: Disables Processor A General Purpose Interrupt 3 request due to Processor B reset de-assertion."]
  RDIE_0,
  #[doc = "1: Enables Processor A General Purpose Interrupt 3 request due to Processor B reset de-assertion."]
  RDIE_1,
}
impl From<RDIE_A> for bool {
  #[inline(always)]
  fn from(variant: RDIE_A) -> Self {
    match variant {
      RDIE_A::RDIE_0 => false,
      RDIE_A::RDIE_1 => true,
    }
  }
}
#[doc = "Reader of field `RDIE`"]
pub type RDIE_R = crate::R<bool, RDIE_A>;
impl RDIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RDIE_A {
    match self.bits {
      false => RDIE_A::RDIE_0,
      true => RDIE_A::RDIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `RDIE_0`"]
  #[inline(always)]
  pub fn is_rdie_0(&self) -> bool {
    *self == RDIE_A::RDIE_0
  }
  #[doc = "Checks if the value of the field is `RDIE_1`"]
  #[inline(always)]
  pub fn is_rdie_1(&self) -> bool {
    *self == RDIE_A::RDIE_1
  }
}
#[doc = "Write proxy for field `RDIE`"]
pub struct RDIE_W<'a> {
  w: &'a mut W,
}
impl<'a> RDIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RDIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disables Processor A General Purpose Interrupt 3 request due to Processor B reset de-assertion."]
  #[inline(always)]
  pub fn rdie_0(self) -> &'a mut W {
    self.variant(RDIE_A::RDIE_0)
  }
  #[doc = "Enables Processor A General Purpose Interrupt 3 request due to Processor B reset de-assertion."]
  #[inline(always)]
  pub fn rdie_1(self) -> &'a mut W {
    self.variant(RDIE_A::RDIE_1)
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
#[doc = "Processor A hardware reset interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRIE_A {
  #[doc = "0: Disables Processor A General Purpose Interrupt 3 request due to Processor B issued HR to Processor A."]
  HRIE_0,
  #[doc = "1: Enables Processor A General Purpose Interrupt 3 request due to Processor B issued HR to Processor A."]
  HRIE_1,
}
impl From<HRIE_A> for bool {
  #[inline(always)]
  fn from(variant: HRIE_A) -> Self {
    match variant {
      HRIE_A::HRIE_0 => false,
      HRIE_A::HRIE_1 => true,
    }
  }
}
#[doc = "Reader of field `HRIE`"]
pub type HRIE_R = crate::R<bool, HRIE_A>;
impl HRIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> HRIE_A {
    match self.bits {
      false => HRIE_A::HRIE_0,
      true => HRIE_A::HRIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `HRIE_0`"]
  #[inline(always)]
  pub fn is_hrie_0(&self) -> bool {
    *self == HRIE_A::HRIE_0
  }
  #[doc = "Checks if the value of the field is `HRIE_1`"]
  #[inline(always)]
  pub fn is_hrie_1(&self) -> bool {
    *self == HRIE_A::HRIE_1
  }
}
#[doc = "Write proxy for field `HRIE`"]
pub struct HRIE_W<'a> {
  w: &'a mut W,
}
impl<'a> HRIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: HRIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disables Processor A General Purpose Interrupt 3 request due to Processor B issued HR to Processor A."]
  #[inline(always)]
  pub fn hrie_0(self) -> &'a mut W {
    self.variant(HRIE_A::HRIE_0)
  }
  #[doc = "Enables Processor A General Purpose Interrupt 3 request due to Processor B issued HR to Processor A."]
  #[inline(always)]
  pub fn hrie_1(self) -> &'a mut W {
    self.variant(HRIE_A::HRIE_1)
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
#[doc = "MURIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MURIE_A {
  #[doc = "0: Disables Processor A-side General Purpose Interrupt 3 request due to MU reset issued by MUB."]
  MURIE_0,
  #[doc = "1: Enables Processor A-side General Purpose Interrupt 3 request due to MU reset issued by MUB."]
  MURIE_1,
}
impl From<MURIE_A> for bool {
  #[inline(always)]
  fn from(variant: MURIE_A) -> Self {
    match variant {
      MURIE_A::MURIE_0 => false,
      MURIE_A::MURIE_1 => true,
    }
  }
}
#[doc = "Reader of field `MURIE`"]
pub type MURIE_R = crate::R<bool, MURIE_A>;
impl MURIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MURIE_A {
    match self.bits {
      false => MURIE_A::MURIE_0,
      true => MURIE_A::MURIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `MURIE_0`"]
  #[inline(always)]
  pub fn is_murie_0(&self) -> bool {
    *self == MURIE_A::MURIE_0
  }
  #[doc = "Checks if the value of the field is `MURIE_1`"]
  #[inline(always)]
  pub fn is_murie_1(&self) -> bool {
    *self == MURIE_A::MURIE_1
  }
}
#[doc = "Write proxy for field `MURIE`"]
pub struct MURIE_W<'a> {
  w: &'a mut W,
}
impl<'a> MURIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MURIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disables Processor A-side General Purpose Interrupt 3 request due to MU reset issued by MUB."]
  #[inline(always)]
  pub fn murie_0(self) -> &'a mut W {
    self.variant(MURIE_A::MURIE_0)
  }
  #[doc = "Enables Processor A-side General Purpose Interrupt 3 request due to MU reset issued by MUB."]
  #[inline(always)]
  pub fn murie_1(self) -> &'a mut W {
    self.variant(MURIE_A::MURIE_1)
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
#[doc = "RAIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAIE_A {
  #[doc = "0: Disables Processor A-side General Purpose Interrupt 3 request due to Processor B reset assertion."]
  RAIE_0,
  #[doc = "1: Enables Processor A-side General Purpose Interrupt 3 request due to Processor B reset assertion."]
  RAIE_1,
}
impl From<RAIE_A> for bool {
  #[inline(always)]
  fn from(variant: RAIE_A) -> Self {
    match variant {
      RAIE_A::RAIE_0 => false,
      RAIE_A::RAIE_1 => true,
    }
  }
}
#[doc = "Reader of field `RAIE`"]
pub type RAIE_R = crate::R<bool, RAIE_A>;
impl RAIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RAIE_A {
    match self.bits {
      false => RAIE_A::RAIE_0,
      true => RAIE_A::RAIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `RAIE_0`"]
  #[inline(always)]
  pub fn is_raie_0(&self) -> bool {
    *self == RAIE_A::RAIE_0
  }
  #[doc = "Checks if the value of the field is `RAIE_1`"]
  #[inline(always)]
  pub fn is_raie_1(&self) -> bool {
    *self == RAIE_A::RAIE_1
  }
}
#[doc = "Write proxy for field `RAIE`"]
pub struct RAIE_W<'a> {
  w: &'a mut W,
}
impl<'a> RAIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RAIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disables Processor A-side General Purpose Interrupt 3 request due to Processor B reset assertion."]
  #[inline(always)]
  pub fn raie_0(self) -> &'a mut W {
    self.variant(RAIE_A::RAIE_0)
  }
  #[doc = "Enables Processor A-side General Purpose Interrupt 3 request due to Processor B reset assertion."]
  #[inline(always)]
  pub fn raie_1(self) -> &'a mut W {
    self.variant(RAIE_A::RAIE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
    self.w
  }
}
#[doc = "GIRn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIRN_A {
  #[doc = "0: MUA General Interrupt n is not requested to the MUB (default)."]
  GIRN_0,
  #[doc = "1: MUA General Interrupt n is requested to the MUB."]
  GIRN_1,
}
impl From<GIRN_A> for u8 {
  #[inline(always)]
  fn from(variant: GIRN_A) -> Self {
    match variant {
      GIRN_A::GIRN_0 => 0,
      GIRN_A::GIRN_1 => 1,
    }
  }
}
#[doc = "Reader of field `GIRn`"]
pub type GIRN_R = crate::R<u8, GIRN_A>;
impl GIRN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, GIRN_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(GIRN_A::GIRN_0),
      1 => Val(GIRN_A::GIRN_1),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `GIRN_0`"]
  #[inline(always)]
  pub fn is_girn_0(&self) -> bool {
    *self == GIRN_A::GIRN_0
  }
  #[doc = "Checks if the value of the field is `GIRN_1`"]
  #[inline(always)]
  pub fn is_girn_1(&self) -> bool {
    *self == GIRN_A::GIRN_1
  }
}
#[doc = "Write proxy for field `GIRn`"]
pub struct GIRN_W<'a> {
  w: &'a mut W,
}
impl<'a> GIRN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GIRN_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "MUA General Interrupt n is not requested to the MUB (default)."]
  #[inline(always)]
  pub fn girn_0(self) -> &'a mut W {
    self.variant(GIRN_A::GIRN_0)
  }
  #[doc = "MUA General Interrupt n is requested to the MUB."]
  #[inline(always)]
  pub fn girn_1(self) -> &'a mut W {
    self.variant(GIRN_A::GIRN_1)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
    self.w
  }
}
#[doc = "TIEn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIEN_A {
  #[doc = "0: Disables MUA Transmit Interrupt n. (default)"]
  TIEN_0,
  #[doc = "1: Enables MUA Transmit Interrupt n."]
  TIEN_1,
}
impl From<TIEN_A> for u8 {
  #[inline(always)]
  fn from(variant: TIEN_A) -> Self {
    match variant {
      TIEN_A::TIEN_0 => 0,
      TIEN_A::TIEN_1 => 1,
    }
  }
}
#[doc = "Reader of field `TIEn`"]
pub type TIEN_R = crate::R<u8, TIEN_A>;
impl TIEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, TIEN_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(TIEN_A::TIEN_0),
      1 => Val(TIEN_A::TIEN_1),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `TIEN_0`"]
  #[inline(always)]
  pub fn is_tien_0(&self) -> bool {
    *self == TIEN_A::TIEN_0
  }
  #[doc = "Checks if the value of the field is `TIEN_1`"]
  #[inline(always)]
  pub fn is_tien_1(&self) -> bool {
    *self == TIEN_A::TIEN_1
  }
}
#[doc = "Write proxy for field `TIEn`"]
pub struct TIEN_W<'a> {
  w: &'a mut W,
}
impl<'a> TIEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TIEN_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Disables MUA Transmit Interrupt n. (default)"]
  #[inline(always)]
  pub fn tien_0(self) -> &'a mut W {
    self.variant(TIEN_A::TIEN_0)
  }
  #[doc = "Enables MUA Transmit Interrupt n."]
  #[inline(always)]
  pub fn tien_1(self) -> &'a mut W {
    self.variant(TIEN_A::TIEN_1)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
    self.w
  }
}
#[doc = "RIEn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIEN_A {
  #[doc = "0: Disables MUA Receive Interrupt n. (default)"]
  RIEN_0,
  #[doc = "1: Enables MUA Receive Interrupt n."]
  RIEN_1,
}
impl From<RIEN_A> for u8 {
  #[inline(always)]
  fn from(variant: RIEN_A) -> Self {
    match variant {
      RIEN_A::RIEN_0 => 0,
      RIEN_A::RIEN_1 => 1,
    }
  }
}
#[doc = "Reader of field `RIEn`"]
pub type RIEN_R = crate::R<u8, RIEN_A>;
impl RIEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, RIEN_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(RIEN_A::RIEN_0),
      1 => Val(RIEN_A::RIEN_1),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `RIEN_0`"]
  #[inline(always)]
  pub fn is_rien_0(&self) -> bool {
    *self == RIEN_A::RIEN_0
  }
  #[doc = "Checks if the value of the field is `RIEN_1`"]
  #[inline(always)]
  pub fn is_rien_1(&self) -> bool {
    *self == RIEN_A::RIEN_1
  }
}
#[doc = "Write proxy for field `RIEn`"]
pub struct RIEN_W<'a> {
  w: &'a mut W,
}
impl<'a> RIEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RIEN_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Disables MUA Receive Interrupt n. (default)"]
  #[inline(always)]
  pub fn rien_0(self) -> &'a mut W {
    self.variant(RIEN_A::RIEN_0)
  }
  #[doc = "Enables MUA Receive Interrupt n."]
  #[inline(always)]
  pub fn rien_1(self) -> &'a mut W {
    self.variant(RIEN_A::RIEN_1)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
    self.w
  }
}
#[doc = "GIEn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIEN_A {
  #[doc = "0: Disables MUA General Interrupt n. (default)"]
  GIEN_0,
  #[doc = "1: Enables MUA General Interrupt n."]
  GIEN_1,
}
impl From<GIEN_A> for u8 {
  #[inline(always)]
  fn from(variant: GIEN_A) -> Self {
    match variant {
      GIEN_A::GIEN_0 => 0,
      GIEN_A::GIEN_1 => 1,
    }
  }
}
#[doc = "Reader of field `GIEn`"]
pub type GIEN_R = crate::R<u8, GIEN_A>;
impl GIEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, GIEN_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(GIEN_A::GIEN_0),
      1 => Val(GIEN_A::GIEN_1),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `GIEN_0`"]
  #[inline(always)]
  pub fn is_gien_0(&self) -> bool {
    *self == GIEN_A::GIEN_0
  }
  #[doc = "Checks if the value of the field is `GIEN_1`"]
  #[inline(always)]
  pub fn is_gien_1(&self) -> bool {
    *self == GIEN_A::GIEN_1
  }
}
#[doc = "Write proxy for field `GIEn`"]
pub struct GIEN_W<'a> {
  w: &'a mut W,
}
impl<'a> GIEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GIEN_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Disables MUA General Interrupt n. (default)"]
  #[inline(always)]
  pub fn gien_0(self) -> &'a mut W {
    self.variant(GIEN_A::GIEN_0)
  }
  #[doc = "Enables MUA General Interrupt n."]
  #[inline(always)]
  pub fn gien_1(self) -> &'a mut W {
    self.variant(GIEN_A::GIEN_1)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:2 - Fn"]
  #[inline(always)]
  pub fn fn_(&self) -> FN_R {
    FN_R::new((self.bits & 0x07) as u8)
  }
  #[doc = "Bit 3 - NMI"]
  #[inline(always)]
  pub fn nmi(&self) -> NMI_R {
    NMI_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 5 - MUR"]
  #[inline(always)]
  pub fn mur(&self) -> MUR_R {
    MUR_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - RDIE"]
  #[inline(always)]
  pub fn rdie(&self) -> RDIE_R {
    RDIE_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Processor A hardware reset interrupt enable"]
  #[inline(always)]
  pub fn hrie(&self) -> HRIE_R {
    HRIE_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 11 - MURIE"]
  #[inline(always)]
  pub fn murie(&self) -> MURIE_R {
    MURIE_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bit 12 - RAIE"]
  #[inline(always)]
  pub fn raie(&self) -> RAIE_R {
    RAIE_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bits 16:19 - GIRn"]
  #[inline(always)]
  pub fn girn(&self) -> GIRN_R {
    GIRN_R::new(((self.bits >> 16) & 0x0f) as u8)
  }
  #[doc = "Bits 20:23 - TIEn"]
  #[inline(always)]
  pub fn tien(&self) -> TIEN_R {
    TIEN_R::new(((self.bits >> 20) & 0x0f) as u8)
  }
  #[doc = "Bits 24:27 - RIEn"]
  #[inline(always)]
  pub fn rien(&self) -> RIEN_R {
    RIEN_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
  #[doc = "Bits 28:31 - GIEn"]
  #[inline(always)]
  pub fn gien(&self) -> GIEN_R {
    GIEN_R::new(((self.bits >> 28) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bits 0:2 - Fn"]
  #[inline(always)]
  pub fn fn_(&mut self) -> FN_W {
    FN_W { w: self }
  }
  #[doc = "Bit 3 - NMI"]
  #[inline(always)]
  pub fn nmi(&mut self) -> NMI_W {
    NMI_W { w: self }
  }
  #[doc = "Bit 5 - MUR"]
  #[inline(always)]
  pub fn mur(&mut self) -> MUR_W {
    MUR_W { w: self }
  }
  #[doc = "Bit 6 - RDIE"]
  #[inline(always)]
  pub fn rdie(&mut self) -> RDIE_W {
    RDIE_W { w: self }
  }
  #[doc = "Bit 7 - Processor A hardware reset interrupt enable"]
  #[inline(always)]
  pub fn hrie(&mut self) -> HRIE_W {
    HRIE_W { w: self }
  }
  #[doc = "Bit 11 - MURIE"]
  #[inline(always)]
  pub fn murie(&mut self) -> MURIE_W {
    MURIE_W { w: self }
  }
  #[doc = "Bit 12 - RAIE"]
  #[inline(always)]
  pub fn raie(&mut self) -> RAIE_W {
    RAIE_W { w: self }
  }
  #[doc = "Bits 16:19 - GIRn"]
  #[inline(always)]
  pub fn girn(&mut self) -> GIRN_W {
    GIRN_W { w: self }
  }
  #[doc = "Bits 20:23 - TIEn"]
  #[inline(always)]
  pub fn tien(&mut self) -> TIEN_W {
    TIEN_W { w: self }
  }
  #[doc = "Bits 24:27 - RIEn"]
  #[inline(always)]
  pub fn rien(&mut self) -> RIEN_W {
    RIEN_W { w: self }
  }
  #[doc = "Bits 28:31 - GIEn"]
  #[inline(always)]
  pub fn gien(&mut self) -> GIEN_W {
    GIEN_W { w: self }
  }
}
