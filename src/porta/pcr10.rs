#[doc = "Reader of register PCR10"]
pub type R = crate::R<u32, super::PCR10>;
#[doc = "Writer for register PCR10"]
pub type W = crate::W<u32, super::PCR10>;
#[doc = "Register PCR10 `reset()`'s with value 0"]
impl crate::ResetValue for super::PCR10 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Pull Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_A {
  #[doc = "0: Internal pulldown resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
  PS_0,
  #[doc = "1: Internal pullup resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
  PS_1,
}
impl From<PS_A> for bool {
  #[inline(always)]
  fn from(variant: PS_A) -> Self {
    match variant {
      PS_A::PS_0 => false,
      PS_A::PS_1 => true,
    }
  }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<bool, PS_A>;
impl PS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PS_A {
    match self.bits {
      false => PS_A::PS_0,
      true => PS_A::PS_1,
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
      self.bit(variant.into())
    }
  }
  #[doc = "Internal pulldown resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
  #[inline(always)]
  pub fn ps_0(self) -> &'a mut W {
    self.variant(PS_A::PS_0)
  }
  #[doc = "Internal pullup resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
  #[inline(always)]
  pub fn ps_1(self) -> &'a mut W {
    self.variant(PS_A::PS_1)
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
#[doc = "Pull Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
  #[doc = "0: no description available"]
  PE_0,
  #[doc = "1: no description available"]
  PE_1,
}
impl From<PE_A> for bool {
  #[inline(always)]
  fn from(variant: PE_A) -> Self {
    match variant {
      PE_A::PE_0 => false,
      PE_A::PE_1 => true,
    }
  }
}
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, PE_A>;
impl PE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PE_A {
    match self.bits {
      false => PE_A::PE_0,
      true => PE_A::PE_1,
    }
  }
  #[doc = "Checks if the value of the field is `PE_0`"]
  #[inline(always)]
  pub fn is_pe_0(&self) -> bool {
    *self == PE_A::PE_0
  }
  #[doc = "Checks if the value of the field is `PE_1`"]
  #[inline(always)]
  pub fn is_pe_1(&self) -> bool {
    *self == PE_A::PE_1
  }
}
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
  w: &'a mut W,
}
impl<'a> PE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn pe_0(self) -> &'a mut W {
    self.variant(PE_A::PE_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn pe_1(self) -> &'a mut W {
    self.variant(PE_A::PE_1)
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
#[doc = "Slew Rate Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRE_A {
  #[doc = "0: Fast slew rate is configured on the corresponding pin, if the pin is configured as a digital output."]
  SRE_0,
  #[doc = "1: Slow slew rate is configured on the corresponding pin, if the pin is configured as a digital output."]
  SRE_1,
}
impl From<SRE_A> for bool {
  #[inline(always)]
  fn from(variant: SRE_A) -> Self {
    match variant {
      SRE_A::SRE_0 => false,
      SRE_A::SRE_1 => true,
    }
  }
}
#[doc = "Reader of field `SRE`"]
pub type SRE_R = crate::R<bool, SRE_A>;
impl SRE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SRE_A {
    match self.bits {
      false => SRE_A::SRE_0,
      true => SRE_A::SRE_1,
    }
  }
  #[doc = "Checks if the value of the field is `SRE_0`"]
  #[inline(always)]
  pub fn is_sre_0(&self) -> bool {
    *self == SRE_A::SRE_0
  }
  #[doc = "Checks if the value of the field is `SRE_1`"]
  #[inline(always)]
  pub fn is_sre_1(&self) -> bool {
    *self == SRE_A::SRE_1
  }
}
#[doc = "Write proxy for field `SRE`"]
pub struct SRE_W<'a> {
  w: &'a mut W,
}
impl<'a> SRE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SRE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Fast slew rate is configured on the corresponding pin, if the pin is configured as a digital output."]
  #[inline(always)]
  pub fn sre_0(self) -> &'a mut W {
    self.variant(SRE_A::SRE_0)
  }
  #[doc = "Slow slew rate is configured on the corresponding pin, if the pin is configured as a digital output."]
  #[inline(always)]
  pub fn sre_1(self) -> &'a mut W {
    self.variant(SRE_A::SRE_1)
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
#[doc = "Open Drain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODE_A {
  #[doc = "0: Open drain output is disabled on the corresponding pin."]
  ODE_0,
  #[doc = "1: Open drain output is enabled on the corresponding pin, if the pin is configured as a digital output."]
  ODE_1,
}
impl From<ODE_A> for bool {
  #[inline(always)]
  fn from(variant: ODE_A) -> Self {
    match variant {
      ODE_A::ODE_0 => false,
      ODE_A::ODE_1 => true,
    }
  }
}
#[doc = "Reader of field `ODE`"]
pub type ODE_R = crate::R<bool, ODE_A>;
impl ODE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ODE_A {
    match self.bits {
      false => ODE_A::ODE_0,
      true => ODE_A::ODE_1,
    }
  }
  #[doc = "Checks if the value of the field is `ODE_0`"]
  #[inline(always)]
  pub fn is_ode_0(&self) -> bool {
    *self == ODE_A::ODE_0
  }
  #[doc = "Checks if the value of the field is `ODE_1`"]
  #[inline(always)]
  pub fn is_ode_1(&self) -> bool {
    *self == ODE_A::ODE_1
  }
}
#[doc = "Write proxy for field `ODE`"]
pub struct ODE_W<'a> {
  w: &'a mut W,
}
impl<'a> ODE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ODE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Open drain output is disabled on the corresponding pin."]
  #[inline(always)]
  pub fn ode_0(self) -> &'a mut W {
    self.variant(ODE_A::ODE_0)
  }
  #[doc = "Open drain output is enabled on the corresponding pin, if the pin is configured as a digital output."]
  #[inline(always)]
  pub fn ode_1(self) -> &'a mut W {
    self.variant(ODE_A::ODE_1)
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
#[doc = "Pin Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX_A {
  #[doc = "0: Pin disabled (Alternative 0) (analog)."]
  MUX_0,
  #[doc = "1: Alternative 1 (GPIO)."]
  MUX_1,
  #[doc = "2: Alternative 2 (chip-specific)."]
  MUX_2,
  #[doc = "3: Alternative 3 (chip-specific)."]
  MUX_3,
  #[doc = "4: Alternative 4 (chip-specific)."]
  MUX_4,
  #[doc = "5: Alternative 5 (chip-specific)."]
  MUX_5,
  #[doc = "6: Alternative 6 (chip-specific)."]
  MUX_6,
  #[doc = "7: Alternative 7 (chip-specific)."]
  MUX_7,
}
impl From<MUX_A> for u8 {
  #[inline(always)]
  fn from(variant: MUX_A) -> Self {
    match variant {
      MUX_A::MUX_0 => 0,
      MUX_A::MUX_1 => 1,
      MUX_A::MUX_2 => 2,
      MUX_A::MUX_3 => 3,
      MUX_A::MUX_4 => 4,
      MUX_A::MUX_5 => 5,
      MUX_A::MUX_6 => 6,
      MUX_A::MUX_7 => 7,
    }
  }
}
#[doc = "Reader of field `MUX`"]
pub type MUX_R = crate::R<u8, MUX_A>;
impl MUX_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MUX_A {
    match self.bits {
      0 => MUX_A::MUX_0,
      1 => MUX_A::MUX_1,
      2 => MUX_A::MUX_2,
      3 => MUX_A::MUX_3,
      4 => MUX_A::MUX_4,
      5 => MUX_A::MUX_5,
      6 => MUX_A::MUX_6,
      7 => MUX_A::MUX_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `MUX_0`"]
  #[inline(always)]
  pub fn is_mux_0(&self) -> bool {
    *self == MUX_A::MUX_0
  }
  #[doc = "Checks if the value of the field is `MUX_1`"]
  #[inline(always)]
  pub fn is_mux_1(&self) -> bool {
    *self == MUX_A::MUX_1
  }
  #[doc = "Checks if the value of the field is `MUX_2`"]
  #[inline(always)]
  pub fn is_mux_2(&self) -> bool {
    *self == MUX_A::MUX_2
  }
  #[doc = "Checks if the value of the field is `MUX_3`"]
  #[inline(always)]
  pub fn is_mux_3(&self) -> bool {
    *self == MUX_A::MUX_3
  }
  #[doc = "Checks if the value of the field is `MUX_4`"]
  #[inline(always)]
  pub fn is_mux_4(&self) -> bool {
    *self == MUX_A::MUX_4
  }
  #[doc = "Checks if the value of the field is `MUX_5`"]
  #[inline(always)]
  pub fn is_mux_5(&self) -> bool {
    *self == MUX_A::MUX_5
  }
  #[doc = "Checks if the value of the field is `MUX_6`"]
  #[inline(always)]
  pub fn is_mux_6(&self) -> bool {
    *self == MUX_A::MUX_6
  }
  #[doc = "Checks if the value of the field is `MUX_7`"]
  #[inline(always)]
  pub fn is_mux_7(&self) -> bool {
    *self == MUX_A::MUX_7
  }
}
#[doc = "Write proxy for field `MUX`"]
pub struct MUX_W<'a> {
  w: &'a mut W,
}
impl<'a> MUX_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MUX_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Pin disabled (Alternative 0) (analog)."]
  #[inline(always)]
  pub fn mux_0(self) -> &'a mut W {
    self.variant(MUX_A::MUX_0)
  }
  #[doc = "Alternative 1 (GPIO)."]
  #[inline(always)]
  pub fn mux_1(self) -> &'a mut W {
    self.variant(MUX_A::MUX_1)
  }
  #[doc = "Alternative 2 (chip-specific)."]
  #[inline(always)]
  pub fn mux_2(self) -> &'a mut W {
    self.variant(MUX_A::MUX_2)
  }
  #[doc = "Alternative 3 (chip-specific)."]
  #[inline(always)]
  pub fn mux_3(self) -> &'a mut W {
    self.variant(MUX_A::MUX_3)
  }
  #[doc = "Alternative 4 (chip-specific)."]
  #[inline(always)]
  pub fn mux_4(self) -> &'a mut W {
    self.variant(MUX_A::MUX_4)
  }
  #[doc = "Alternative 5 (chip-specific)."]
  #[inline(always)]
  pub fn mux_5(self) -> &'a mut W {
    self.variant(MUX_A::MUX_5)
  }
  #[doc = "Alternative 6 (chip-specific)."]
  #[inline(always)]
  pub fn mux_6(self) -> &'a mut W {
    self.variant(MUX_A::MUX_6)
  }
  #[doc = "Alternative 7 (chip-specific)."]
  #[inline(always)]
  pub fn mux_7(self) -> &'a mut W {
    self.variant(MUX_A::MUX_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
    self.w
  }
}
#[doc = "Lock Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK_A {
  #[doc = "0: Pin Control Register is not locked."]
  LK_0,
  #[doc = "1: Pin Control Register is locked and cannot be updated until the next system reset."]
  LK_1,
}
impl From<LK_A> for bool {
  #[inline(always)]
  fn from(variant: LK_A) -> Self {
    match variant {
      LK_A::LK_0 => false,
      LK_A::LK_1 => true,
    }
  }
}
#[doc = "Reader of field `LK`"]
pub type LK_R = crate::R<bool, LK_A>;
impl LK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LK_A {
    match self.bits {
      false => LK_A::LK_0,
      true => LK_A::LK_1,
    }
  }
  #[doc = "Checks if the value of the field is `LK_0`"]
  #[inline(always)]
  pub fn is_lk_0(&self) -> bool {
    *self == LK_A::LK_0
  }
  #[doc = "Checks if the value of the field is `LK_1`"]
  #[inline(always)]
  pub fn is_lk_1(&self) -> bool {
    *self == LK_A::LK_1
  }
}
#[doc = "Write proxy for field `LK`"]
pub struct LK_W<'a> {
  w: &'a mut W,
}
impl<'a> LK_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LK_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Pin Control Register is not locked."]
  #[inline(always)]
  pub fn lk_0(self) -> &'a mut W {
    self.variant(LK_A::LK_0)
  }
  #[doc = "Pin Control Register is locked and cannot be updated until the next system reset."]
  #[inline(always)]
  pub fn lk_1(self) -> &'a mut W {
    self.variant(LK_A::LK_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
    self.w
  }
}
#[doc = "Interrupt Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRQC_A {
  #[doc = "0: Interrupt Status Flag (ISF) is disabled."]
  IRQC_0,
  #[doc = "1: ISF flag and DMA request on rising edge."]
  IRQC_1,
  #[doc = "2: ISF flag and DMA request on falling edge."]
  IRQC_2,
  #[doc = "3: ISF flag and DMA request on either edge."]
  IRQC_3,
  #[doc = "5: no description available"]
  IRQC_5,
  #[doc = "6: no description available"]
  IRQC_6,
  #[doc = "7: no description available"]
  IRQC_7,
  #[doc = "8: ISF flag and Interrupt when logic 0."]
  IRQC_8,
  #[doc = "9: ISF flag and Interrupt on rising-edge."]
  IRQC_9,
  #[doc = "10: ISF flag and Interrupt on falling-edge."]
  IRQC_10,
  #[doc = "11: ISF flag and Interrupt on either edge."]
  IRQC_11,
  #[doc = "12: ISF flag and Interrupt when logic 1."]
  IRQC_12,
  #[doc = "13: no description available"]
  IRQC_13,
  #[doc = "14: no description available"]
  IRQC_14,
}
impl From<IRQC_A> for u8 {
  #[inline(always)]
  fn from(variant: IRQC_A) -> Self {
    match variant {
      IRQC_A::IRQC_0 => 0,
      IRQC_A::IRQC_1 => 1,
      IRQC_A::IRQC_2 => 2,
      IRQC_A::IRQC_3 => 3,
      IRQC_A::IRQC_5 => 5,
      IRQC_A::IRQC_6 => 6,
      IRQC_A::IRQC_7 => 7,
      IRQC_A::IRQC_8 => 8,
      IRQC_A::IRQC_9 => 9,
      IRQC_A::IRQC_10 => 10,
      IRQC_A::IRQC_11 => 11,
      IRQC_A::IRQC_12 => 12,
      IRQC_A::IRQC_13 => 13,
      IRQC_A::IRQC_14 => 14,
    }
  }
}
#[doc = "Reader of field `IRQC`"]
pub type IRQC_R = crate::R<u8, IRQC_A>;
impl IRQC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, IRQC_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(IRQC_A::IRQC_0),
      1 => Val(IRQC_A::IRQC_1),
      2 => Val(IRQC_A::IRQC_2),
      3 => Val(IRQC_A::IRQC_3),
      5 => Val(IRQC_A::IRQC_5),
      6 => Val(IRQC_A::IRQC_6),
      7 => Val(IRQC_A::IRQC_7),
      8 => Val(IRQC_A::IRQC_8),
      9 => Val(IRQC_A::IRQC_9),
      10 => Val(IRQC_A::IRQC_10),
      11 => Val(IRQC_A::IRQC_11),
      12 => Val(IRQC_A::IRQC_12),
      13 => Val(IRQC_A::IRQC_13),
      14 => Val(IRQC_A::IRQC_14),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `IRQC_0`"]
  #[inline(always)]
  pub fn is_irqc_0(&self) -> bool {
    *self == IRQC_A::IRQC_0
  }
  #[doc = "Checks if the value of the field is `IRQC_1`"]
  #[inline(always)]
  pub fn is_irqc_1(&self) -> bool {
    *self == IRQC_A::IRQC_1
  }
  #[doc = "Checks if the value of the field is `IRQC_2`"]
  #[inline(always)]
  pub fn is_irqc_2(&self) -> bool {
    *self == IRQC_A::IRQC_2
  }
  #[doc = "Checks if the value of the field is `IRQC_3`"]
  #[inline(always)]
  pub fn is_irqc_3(&self) -> bool {
    *self == IRQC_A::IRQC_3
  }
  #[doc = "Checks if the value of the field is `IRQC_5`"]
  #[inline(always)]
  pub fn is_irqc_5(&self) -> bool {
    *self == IRQC_A::IRQC_5
  }
  #[doc = "Checks if the value of the field is `IRQC_6`"]
  #[inline(always)]
  pub fn is_irqc_6(&self) -> bool {
    *self == IRQC_A::IRQC_6
  }
  #[doc = "Checks if the value of the field is `IRQC_7`"]
  #[inline(always)]
  pub fn is_irqc_7(&self) -> bool {
    *self == IRQC_A::IRQC_7
  }
  #[doc = "Checks if the value of the field is `IRQC_8`"]
  #[inline(always)]
  pub fn is_irqc_8(&self) -> bool {
    *self == IRQC_A::IRQC_8
  }
  #[doc = "Checks if the value of the field is `IRQC_9`"]
  #[inline(always)]
  pub fn is_irqc_9(&self) -> bool {
    *self == IRQC_A::IRQC_9
  }
  #[doc = "Checks if the value of the field is `IRQC_10`"]
  #[inline(always)]
  pub fn is_irqc_10(&self) -> bool {
    *self == IRQC_A::IRQC_10
  }
  #[doc = "Checks if the value of the field is `IRQC_11`"]
  #[inline(always)]
  pub fn is_irqc_11(&self) -> bool {
    *self == IRQC_A::IRQC_11
  }
  #[doc = "Checks if the value of the field is `IRQC_12`"]
  #[inline(always)]
  pub fn is_irqc_12(&self) -> bool {
    *self == IRQC_A::IRQC_12
  }
  #[doc = "Checks if the value of the field is `IRQC_13`"]
  #[inline(always)]
  pub fn is_irqc_13(&self) -> bool {
    *self == IRQC_A::IRQC_13
  }
  #[doc = "Checks if the value of the field is `IRQC_14`"]
  #[inline(always)]
  pub fn is_irqc_14(&self) -> bool {
    *self == IRQC_A::IRQC_14
  }
}
#[doc = "Write proxy for field `IRQC`"]
pub struct IRQC_W<'a> {
  w: &'a mut W,
}
impl<'a> IRQC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: IRQC_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Interrupt Status Flag (ISF) is disabled."]
  #[inline(always)]
  pub fn irqc_0(self) -> &'a mut W {
    self.variant(IRQC_A::IRQC_0)
  }
  #[doc = "ISF flag and DMA request on rising edge."]
  #[inline(always)]
  pub fn irqc_1(self) -> &'a mut W {
    self.variant(IRQC_A::IRQC_1)
  }
  #[doc = "ISF flag and DMA request on falling edge."]
  #[inline(always)]
  pub fn irqc_2(self) -> &'a mut W {
    self.variant(IRQC_A::IRQC_2)
  }
  #[doc = "ISF flag and DMA request on either edge."]
  #[inline(always)]
  pub fn irqc_3(self) -> &'a mut W {
    self.variant(IRQC_A::IRQC_3)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn irqc_5(self) -> &'a mut W {
    self.variant(IRQC_A::IRQC_5)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn irqc_6(self) -> &'a mut W {
    self.variant(IRQC_A::IRQC_6)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn irqc_7(self) -> &'a mut W {
    self.variant(IRQC_A::IRQC_7)
  }
  #[doc = "ISF flag and Interrupt when logic 0."]
  #[inline(always)]
  pub fn irqc_8(self) -> &'a mut W {
    self.variant(IRQC_A::IRQC_8)
  }
  #[doc = "ISF flag and Interrupt on rising-edge."]
  #[inline(always)]
  pub fn irqc_9(self) -> &'a mut W {
    self.variant(IRQC_A::IRQC_9)
  }
  #[doc = "ISF flag and Interrupt on falling-edge."]
  #[inline(always)]
  pub fn irqc_10(self) -> &'a mut W {
    self.variant(IRQC_A::IRQC_10)
  }
  #[doc = "ISF flag and Interrupt on either edge."]
  #[inline(always)]
  pub fn irqc_11(self) -> &'a mut W {
    self.variant(IRQC_A::IRQC_11)
  }
  #[doc = "ISF flag and Interrupt when logic 1."]
  #[inline(always)]
  pub fn irqc_12(self) -> &'a mut W {
    self.variant(IRQC_A::IRQC_12)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn irqc_13(self) -> &'a mut W {
    self.variant(IRQC_A::IRQC_13)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn irqc_14(self) -> &'a mut W {
    self.variant(IRQC_A::IRQC_14)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
    self.w
  }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF_A {
  #[doc = "0: Configured interrupt is not detected."]
  ISF_0,
  #[doc = "1: no description available"]
  ISF_1,
}
impl From<ISF_A> for bool {
  #[inline(always)]
  fn from(variant: ISF_A) -> Self {
    match variant {
      ISF_A::ISF_0 => false,
      ISF_A::ISF_1 => true,
    }
  }
}
#[doc = "Reader of field `ISF`"]
pub type ISF_R = crate::R<bool, ISF_A>;
impl ISF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ISF_A {
    match self.bits {
      false => ISF_A::ISF_0,
      true => ISF_A::ISF_1,
    }
  }
  #[doc = "Checks if the value of the field is `ISF_0`"]
  #[inline(always)]
  pub fn is_isf_0(&self) -> bool {
    *self == ISF_A::ISF_0
  }
  #[doc = "Checks if the value of the field is `ISF_1`"]
  #[inline(always)]
  pub fn is_isf_1(&self) -> bool {
    *self == ISF_A::ISF_1
  }
}
#[doc = "Write proxy for field `ISF`"]
pub struct ISF_W<'a> {
  w: &'a mut W,
}
impl<'a> ISF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ISF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Configured interrupt is not detected."]
  #[inline(always)]
  pub fn isf_0(self) -> &'a mut W {
    self.variant(ISF_A::ISF_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn isf_1(self) -> &'a mut W {
    self.variant(ISF_A::ISF_1)
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
impl R {
  #[doc = "Bit 0 - Pull Select"]
  #[inline(always)]
  pub fn ps(&self) -> PS_R {
    PS_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Pull Enable"]
  #[inline(always)]
  pub fn pe(&self) -> PE_R {
    PE_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Slew Rate Enable"]
  #[inline(always)]
  pub fn sre(&self) -> SRE_R {
    SRE_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Open Drain Enable"]
  #[inline(always)]
  pub fn ode(&self) -> ODE_R {
    ODE_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bits 8:10 - Pin Mux Control"]
  #[inline(always)]
  pub fn mux(&self) -> MUX_R {
    MUX_R::new(((self.bits >> 8) & 0x07) as u8)
  }
  #[doc = "Bit 15 - Lock Register"]
  #[inline(always)]
  pub fn lk(&self) -> LK_R {
    LK_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bits 16:19 - Interrupt Configuration"]
  #[inline(always)]
  pub fn irqc(&self) -> IRQC_R {
    IRQC_R::new(((self.bits >> 16) & 0x0f) as u8)
  }
  #[doc = "Bit 24 - Interrupt Status Flag"]
  #[inline(always)]
  pub fn isf(&self) -> ISF_R {
    ISF_R::new(((self.bits >> 24) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Pull Select"]
  #[inline(always)]
  pub fn ps(&mut self) -> PS_W {
    PS_W { w: self }
  }
  #[doc = "Bit 1 - Pull Enable"]
  #[inline(always)]
  pub fn pe(&mut self) -> PE_W {
    PE_W { w: self }
  }
  #[doc = "Bit 2 - Slew Rate Enable"]
  #[inline(always)]
  pub fn sre(&mut self) -> SRE_W {
    SRE_W { w: self }
  }
  #[doc = "Bit 5 - Open Drain Enable"]
  #[inline(always)]
  pub fn ode(&mut self) -> ODE_W {
    ODE_W { w: self }
  }
  #[doc = "Bits 8:10 - Pin Mux Control"]
  #[inline(always)]
  pub fn mux(&mut self) -> MUX_W {
    MUX_W { w: self }
  }
  #[doc = "Bit 15 - Lock Register"]
  #[inline(always)]
  pub fn lk(&mut self) -> LK_W {
    LK_W { w: self }
  }
  #[doc = "Bits 16:19 - Interrupt Configuration"]
  #[inline(always)]
  pub fn irqc(&mut self) -> IRQC_W {
    IRQC_W { w: self }
  }
  #[doc = "Bit 24 - Interrupt Status Flag"]
  #[inline(always)]
  pub fn isf(&mut self) -> ISF_W {
    ISF_W { w: self }
  }
}
