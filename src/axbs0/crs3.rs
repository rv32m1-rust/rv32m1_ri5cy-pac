#[doc = "Reader of register CRS3"]
pub type R = crate::R<u32, super::CRS3>;
#[doc = "Writer for register CRS3"]
pub type W = crate::W<u32, super::CRS3>;
#[doc = "Register CRS3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CRS3 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Park\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARK_A {
  #[doc = "0: Park on master port M0"]
  PARK_0,
  #[doc = "1: Park on master port M1"]
  PARK_1,
  #[doc = "2: Park on master port M2"]
  PARK_2,
  #[doc = "3: Park on master port M3"]
  PARK_3,
  #[doc = "4: Park on master port M4"]
  PARK_4,
  #[doc = "5: Park on master port M5"]
  PARK_5,
  #[doc = "6: Park on master port M6"]
  PARK_6,
  #[doc = "7: Park on master port M7"]
  PARK_7,
}
impl From<PARK_A> for u8 {
  #[inline(always)]
  fn from(variant: PARK_A) -> Self {
    match variant {
      PARK_A::PARK_0 => 0,
      PARK_A::PARK_1 => 1,
      PARK_A::PARK_2 => 2,
      PARK_A::PARK_3 => 3,
      PARK_A::PARK_4 => 4,
      PARK_A::PARK_5 => 5,
      PARK_A::PARK_6 => 6,
      PARK_A::PARK_7 => 7,
    }
  }
}
#[doc = "Reader of field `PARK`"]
pub type PARK_R = crate::R<u8, PARK_A>;
impl PARK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PARK_A {
    match self.bits {
      0 => PARK_A::PARK_0,
      1 => PARK_A::PARK_1,
      2 => PARK_A::PARK_2,
      3 => PARK_A::PARK_3,
      4 => PARK_A::PARK_4,
      5 => PARK_A::PARK_5,
      6 => PARK_A::PARK_6,
      7 => PARK_A::PARK_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `PARK_0`"]
  #[inline(always)]
  pub fn is_park_0(&self) -> bool {
    *self == PARK_A::PARK_0
  }
  #[doc = "Checks if the value of the field is `PARK_1`"]
  #[inline(always)]
  pub fn is_park_1(&self) -> bool {
    *self == PARK_A::PARK_1
  }
  #[doc = "Checks if the value of the field is `PARK_2`"]
  #[inline(always)]
  pub fn is_park_2(&self) -> bool {
    *self == PARK_A::PARK_2
  }
  #[doc = "Checks if the value of the field is `PARK_3`"]
  #[inline(always)]
  pub fn is_park_3(&self) -> bool {
    *self == PARK_A::PARK_3
  }
  #[doc = "Checks if the value of the field is `PARK_4`"]
  #[inline(always)]
  pub fn is_park_4(&self) -> bool {
    *self == PARK_A::PARK_4
  }
  #[doc = "Checks if the value of the field is `PARK_5`"]
  #[inline(always)]
  pub fn is_park_5(&self) -> bool {
    *self == PARK_A::PARK_5
  }
  #[doc = "Checks if the value of the field is `PARK_6`"]
  #[inline(always)]
  pub fn is_park_6(&self) -> bool {
    *self == PARK_A::PARK_6
  }
  #[doc = "Checks if the value of the field is `PARK_7`"]
  #[inline(always)]
  pub fn is_park_7(&self) -> bool {
    *self == PARK_A::PARK_7
  }
}
#[doc = "Write proxy for field `PARK`"]
pub struct PARK_W<'a> {
  w: &'a mut W,
}
impl<'a> PARK_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PARK_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Park on master port M0"]
  #[inline(always)]
  pub fn park_0(self) -> &'a mut W {
    self.variant(PARK_A::PARK_0)
  }
  #[doc = "Park on master port M1"]
  #[inline(always)]
  pub fn park_1(self) -> &'a mut W {
    self.variant(PARK_A::PARK_1)
  }
  #[doc = "Park on master port M2"]
  #[inline(always)]
  pub fn park_2(self) -> &'a mut W {
    self.variant(PARK_A::PARK_2)
  }
  #[doc = "Park on master port M3"]
  #[inline(always)]
  pub fn park_3(self) -> &'a mut W {
    self.variant(PARK_A::PARK_3)
  }
  #[doc = "Park on master port M4"]
  #[inline(always)]
  pub fn park_4(self) -> &'a mut W {
    self.variant(PARK_A::PARK_4)
  }
  #[doc = "Park on master port M5"]
  #[inline(always)]
  pub fn park_5(self) -> &'a mut W {
    self.variant(PARK_A::PARK_5)
  }
  #[doc = "Park on master port M6"]
  #[inline(always)]
  pub fn park_6(self) -> &'a mut W {
    self.variant(PARK_A::PARK_6)
  }
  #[doc = "Park on master port M7"]
  #[inline(always)]
  pub fn park_7(self) -> &'a mut W {
    self.variant(PARK_A::PARK_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
    self.w
  }
}
#[doc = "Parking Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCTL_A {
  #[doc = "0: When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK field"]
  PCTL_0,
  #[doc = "1: When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port"]
  PCTL_1,
  #[doc = "2: When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state"]
  PCTL_2,
}
impl From<PCTL_A> for u8 {
  #[inline(always)]
  fn from(variant: PCTL_A) -> Self {
    match variant {
      PCTL_A::PCTL_0 => 0,
      PCTL_A::PCTL_1 => 1,
      PCTL_A::PCTL_2 => 2,
    }
  }
}
#[doc = "Reader of field `PCTL`"]
pub type PCTL_R = crate::R<u8, PCTL_A>;
impl PCTL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, PCTL_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(PCTL_A::PCTL_0),
      1 => Val(PCTL_A::PCTL_1),
      2 => Val(PCTL_A::PCTL_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `PCTL_0`"]
  #[inline(always)]
  pub fn is_pctl_0(&self) -> bool {
    *self == PCTL_A::PCTL_0
  }
  #[doc = "Checks if the value of the field is `PCTL_1`"]
  #[inline(always)]
  pub fn is_pctl_1(&self) -> bool {
    *self == PCTL_A::PCTL_1
  }
  #[doc = "Checks if the value of the field is `PCTL_2`"]
  #[inline(always)]
  pub fn is_pctl_2(&self) -> bool {
    *self == PCTL_A::PCTL_2
  }
}
#[doc = "Write proxy for field `PCTL`"]
pub struct PCTL_W<'a> {
  w: &'a mut W,
}
impl<'a> PCTL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PCTL_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK field"]
  #[inline(always)]
  pub fn pctl_0(self) -> &'a mut W {
    self.variant(PCTL_A::PCTL_0)
  }
  #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port"]
  #[inline(always)]
  pub fn pctl_1(self) -> &'a mut W {
    self.variant(PCTL_A::PCTL_1)
  }
  #[doc = "When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state"]
  #[inline(always)]
  pub fn pctl_2(self) -> &'a mut W {
    self.variant(PCTL_A::PCTL_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
    self.w
  }
}
#[doc = "Arbitration Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARB_A {
  #[doc = "0: Fixed priority"]
  ARB_0,
  #[doc = "1: Round-robin, or rotating, priority"]
  ARB_1,
}
impl From<ARB_A> for u8 {
  #[inline(always)]
  fn from(variant: ARB_A) -> Self {
    match variant {
      ARB_A::ARB_0 => 0,
      ARB_A::ARB_1 => 1,
    }
  }
}
#[doc = "Reader of field `ARB`"]
pub type ARB_R = crate::R<u8, ARB_A>;
impl ARB_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, ARB_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(ARB_A::ARB_0),
      1 => Val(ARB_A::ARB_1),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `ARB_0`"]
  #[inline(always)]
  pub fn is_arb_0(&self) -> bool {
    *self == ARB_A::ARB_0
  }
  #[doc = "Checks if the value of the field is `ARB_1`"]
  #[inline(always)]
  pub fn is_arb_1(&self) -> bool {
    *self == ARB_A::ARB_1
  }
}
#[doc = "Write proxy for field `ARB`"]
pub struct ARB_W<'a> {
  w: &'a mut W,
}
impl<'a> ARB_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ARB_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Fixed priority"]
  #[inline(always)]
  pub fn arb_0(self) -> &'a mut W {
    self.variant(ARB_A::ARB_0)
  }
  #[doc = "Round-robin, or rotating, priority"]
  #[inline(always)]
  pub fn arb_1(self) -> &'a mut W {
    self.variant(ARB_A::ARB_1)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
    self.w
  }
}
#[doc = "Halt Low Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HLP_A {
  #[doc = "0: The low power mode request has the highest priority for arbitration on this slave port"]
  HLP_0,
  #[doc = "1: The low power mode request has the lowest initial priority for arbitration on this slave port"]
  HLP_1,
}
impl From<HLP_A> for bool {
  #[inline(always)]
  fn from(variant: HLP_A) -> Self {
    match variant {
      HLP_A::HLP_0 => false,
      HLP_A::HLP_1 => true,
    }
  }
}
#[doc = "Reader of field `HLP`"]
pub type HLP_R = crate::R<bool, HLP_A>;
impl HLP_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> HLP_A {
    match self.bits {
      false => HLP_A::HLP_0,
      true => HLP_A::HLP_1,
    }
  }
  #[doc = "Checks if the value of the field is `HLP_0`"]
  #[inline(always)]
  pub fn is_hlp_0(&self) -> bool {
    *self == HLP_A::HLP_0
  }
  #[doc = "Checks if the value of the field is `HLP_1`"]
  #[inline(always)]
  pub fn is_hlp_1(&self) -> bool {
    *self == HLP_A::HLP_1
  }
}
#[doc = "Write proxy for field `HLP`"]
pub struct HLP_W<'a> {
  w: &'a mut W,
}
impl<'a> HLP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: HLP_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The low power mode request has the highest priority for arbitration on this slave port"]
  #[inline(always)]
  pub fn hlp_0(self) -> &'a mut W {
    self.variant(HLP_A::HLP_0)
  }
  #[doc = "The low power mode request has the lowest initial priority for arbitration on this slave port"]
  #[inline(always)]
  pub fn hlp_1(self) -> &'a mut W {
    self.variant(HLP_A::HLP_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
    self.w
  }
}
#[doc = "Read Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO_A {
  #[doc = "0: The slave port's registers are writeable"]
  RO_0,
  #[doc = "1: The slave port's registers are read-only and cannot be written. Attempted writes have no effect on the registers and result in a bus error response."]
  RO_1,
}
impl From<RO_A> for bool {
  #[inline(always)]
  fn from(variant: RO_A) -> Self {
    match variant {
      RO_A::RO_0 => false,
      RO_A::RO_1 => true,
    }
  }
}
#[doc = "Reader of field `RO`"]
pub type RO_R = crate::R<bool, RO_A>;
impl RO_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RO_A {
    match self.bits {
      false => RO_A::RO_0,
      true => RO_A::RO_1,
    }
  }
  #[doc = "Checks if the value of the field is `RO_0`"]
  #[inline(always)]
  pub fn is_ro_0(&self) -> bool {
    *self == RO_A::RO_0
  }
  #[doc = "Checks if the value of the field is `RO_1`"]
  #[inline(always)]
  pub fn is_ro_1(&self) -> bool {
    *self == RO_A::RO_1
  }
}
#[doc = "Write proxy for field `RO`"]
pub struct RO_W<'a> {
  w: &'a mut W,
}
impl<'a> RO_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RO_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The slave port's registers are writeable"]
  #[inline(always)]
  pub fn ro_0(self) -> &'a mut W {
    self.variant(RO_A::RO_0)
  }
  #[doc = "The slave port's registers are read-only and cannot be written. Attempted writes have no effect on the registers and result in a bus error response."]
  #[inline(always)]
  pub fn ro_1(self) -> &'a mut W {
    self.variant(RO_A::RO_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:2 - Park"]
  #[inline(always)]
  pub fn park(&self) -> PARK_R {
    PARK_R::new((self.bits & 0x07) as u8)
  }
  #[doc = "Bits 4:5 - Parking Control"]
  #[inline(always)]
  pub fn pctl(&self) -> PCTL_R {
    PCTL_R::new(((self.bits >> 4) & 0x03) as u8)
  }
  #[doc = "Bits 8:9 - Arbitration Mode"]
  #[inline(always)]
  pub fn arb(&self) -> ARB_R {
    ARB_R::new(((self.bits >> 8) & 0x03) as u8)
  }
  #[doc = "Bit 30 - Halt Low Priority"]
  #[inline(always)]
  pub fn hlp(&self) -> HLP_R {
    HLP_R::new(((self.bits >> 30) & 0x01) != 0)
  }
  #[doc = "Bit 31 - Read Only"]
  #[inline(always)]
  pub fn ro(&self) -> RO_R {
    RO_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:2 - Park"]
  #[inline(always)]
  pub fn park(&mut self) -> PARK_W {
    PARK_W { w: self }
  }
  #[doc = "Bits 4:5 - Parking Control"]
  #[inline(always)]
  pub fn pctl(&mut self) -> PCTL_W {
    PCTL_W { w: self }
  }
  #[doc = "Bits 8:9 - Arbitration Mode"]
  #[inline(always)]
  pub fn arb(&mut self) -> ARB_W {
    ARB_W { w: self }
  }
  #[doc = "Bit 30 - Halt Low Priority"]
  #[inline(always)]
  pub fn hlp(&mut self) -> HLP_W {
    HLP_W { w: self }
  }
  #[doc = "Bit 31 - Read Only"]
  #[inline(always)]
  pub fn ro(&mut self) -> RO_W {
    RO_W { w: self }
  }
}
