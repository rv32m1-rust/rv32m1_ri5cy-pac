#[doc = "Reader of register VERID"]
pub type R = crate::R<u32, super::VERID>;
#[doc = "Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RES_A {
  #[doc = "0: Up to 13-bit differential/12-bit single ended resolution supported."]
  RES_0,
  #[doc = "1: Up to 16-bit differential/15-bit single ended resolution supported."]
  RES_1,
}
impl From<RES_A> for bool {
  #[inline(always)]
  fn from(variant: RES_A) -> Self {
    match variant {
      RES_A::RES_0 => false,
      RES_A::RES_1 => true,
    }
  }
}
#[doc = "Reader of field `RES`"]
pub type RES_R = crate::R<bool, RES_A>;
impl RES_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RES_A {
    match self.bits {
      false => RES_A::RES_0,
      true => RES_A::RES_1,
    }
  }
  #[doc = "Checks if the value of the field is `RES_0`"]
  #[inline(always)]
  pub fn is_res_0(&self) -> bool {
    *self == RES_A::RES_0
  }
  #[doc = "Checks if the value of the field is `RES_1`"]
  #[inline(always)]
  pub fn is_res_1(&self) -> bool {
    *self == RES_A::RES_1
  }
}
#[doc = "Differential Supported\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFFEN_A {
  #[doc = "0: Differential operation not supported."]
  DIFFEN_0,
  #[doc = "1: Differential operation supported. CMDLa\\[DIFF\\] and CMDLa\\[ABSEL\\] control fields implemented."]
  DIFFEN_1,
}
impl From<DIFFEN_A> for bool {
  #[inline(always)]
  fn from(variant: DIFFEN_A) -> Self {
    match variant {
      DIFFEN_A::DIFFEN_0 => false,
      DIFFEN_A::DIFFEN_1 => true,
    }
  }
}
#[doc = "Reader of field `DIFFEN`"]
pub type DIFFEN_R = crate::R<bool, DIFFEN_A>;
impl DIFFEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DIFFEN_A {
    match self.bits {
      false => DIFFEN_A::DIFFEN_0,
      true => DIFFEN_A::DIFFEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `DIFFEN_0`"]
  #[inline(always)]
  pub fn is_diffen_0(&self) -> bool {
    *self == DIFFEN_A::DIFFEN_0
  }
  #[doc = "Checks if the value of the field is `DIFFEN_1`"]
  #[inline(always)]
  pub fn is_diffen_1(&self) -> bool {
    *self == DIFFEN_A::DIFFEN_1
  }
}
#[doc = "Multi Vref Implemented\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MVI_A {
  #[doc = "0: Single voltage reference high (VREFH) input supported."]
  MVI_0,
  #[doc = "1: Multiple voltage reference high (VREFH) inputs supported."]
  MVI_1,
}
impl From<MVI_A> for bool {
  #[inline(always)]
  fn from(variant: MVI_A) -> Self {
    match variant {
      MVI_A::MVI_0 => false,
      MVI_A::MVI_1 => true,
    }
  }
}
#[doc = "Reader of field `MVI`"]
pub type MVI_R = crate::R<bool, MVI_A>;
impl MVI_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MVI_A {
    match self.bits {
      false => MVI_A::MVI_0,
      true => MVI_A::MVI_1,
    }
  }
  #[doc = "Checks if the value of the field is `MVI_0`"]
  #[inline(always)]
  pub fn is_mvi_0(&self) -> bool {
    *self == MVI_A::MVI_0
  }
  #[doc = "Checks if the value of the field is `MVI_1`"]
  #[inline(always)]
  pub fn is_mvi_1(&self) -> bool {
    *self == MVI_A::MVI_1
  }
}
#[doc = "Channel Scale Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSW_A {
  #[doc = "0: Channel scaling not supported."]
  CSW_0,
  #[doc = "1: Channel scaling supported. 1-bit CSCALE control field."]
  CSW_1,
  #[doc = "6: Channel scaling supported. 6-bit CSCALE control field."]
  CSW_6,
}
impl From<CSW_A> for u8 {
  #[inline(always)]
  fn from(variant: CSW_A) -> Self {
    match variant {
      CSW_A::CSW_0 => 0,
      CSW_A::CSW_1 => 1,
      CSW_A::CSW_6 => 6,
    }
  }
}
#[doc = "Reader of field `CSW`"]
pub type CSW_R = crate::R<u8, CSW_A>;
impl CSW_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, CSW_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(CSW_A::CSW_0),
      1 => Val(CSW_A::CSW_1),
      6 => Val(CSW_A::CSW_6),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `CSW_0`"]
  #[inline(always)]
  pub fn is_csw_0(&self) -> bool {
    *self == CSW_A::CSW_0
  }
  #[doc = "Checks if the value of the field is `CSW_1`"]
  #[inline(always)]
  pub fn is_csw_1(&self) -> bool {
    *self == CSW_A::CSW_1
  }
  #[doc = "Checks if the value of the field is `CSW_6`"]
  #[inline(always)]
  pub fn is_csw_6(&self) -> bool {
    *self == CSW_A::CSW_6
  }
}
#[doc = "Voltage Reference 1 Range Control Bit Implemented\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VR1RNGI_A {
  #[doc = "0: Range control not required. CFG\\[VREF1RNG\\] is not implemented."]
  VR1RNGI_0,
  #[doc = "1: Range control required. CFG\\[VREF1RNG\\] is implemented."]
  VR1RNGI_1,
}
impl From<VR1RNGI_A> for bool {
  #[inline(always)]
  fn from(variant: VR1RNGI_A) -> Self {
    match variant {
      VR1RNGI_A::VR1RNGI_0 => false,
      VR1RNGI_A::VR1RNGI_1 => true,
    }
  }
}
#[doc = "Reader of field `VR1RNGI`"]
pub type VR1RNGI_R = crate::R<bool, VR1RNGI_A>;
impl VR1RNGI_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VR1RNGI_A {
    match self.bits {
      false => VR1RNGI_A::VR1RNGI_0,
      true => VR1RNGI_A::VR1RNGI_1,
    }
  }
  #[doc = "Checks if the value of the field is `VR1RNGI_0`"]
  #[inline(always)]
  pub fn is_vr1rngi_0(&self) -> bool {
    *self == VR1RNGI_A::VR1RNGI_0
  }
  #[doc = "Checks if the value of the field is `VR1RNGI_1`"]
  #[inline(always)]
  pub fn is_vr1rngi_1(&self) -> bool {
    *self == VR1RNGI_A::VR1RNGI_1
  }
}
#[doc = "Internal ADC Clock implemented\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IADCKI_A {
  #[doc = "0: Internal clock source not implemented."]
  IADCKI_0,
  #[doc = "1: Internal clock source (and CFG\\[ADCKEN\\]) implemented."]
  IADCKI_1,
}
impl From<IADCKI_A> for bool {
  #[inline(always)]
  fn from(variant: IADCKI_A) -> Self {
    match variant {
      IADCKI_A::IADCKI_0 => false,
      IADCKI_A::IADCKI_1 => true,
    }
  }
}
#[doc = "Reader of field `IADCKI`"]
pub type IADCKI_R = crate::R<bool, IADCKI_A>;
impl IADCKI_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> IADCKI_A {
    match self.bits {
      false => IADCKI_A::IADCKI_0,
      true => IADCKI_A::IADCKI_1,
    }
  }
  #[doc = "Checks if the value of the field is `IADCKI_0`"]
  #[inline(always)]
  pub fn is_iadcki_0(&self) -> bool {
    *self == IADCKI_A::IADCKI_0
  }
  #[doc = "Checks if the value of the field is `IADCKI_1`"]
  #[inline(always)]
  pub fn is_iadcki_1(&self) -> bool {
    *self == IADCKI_A::IADCKI_1
  }
}
#[doc = "Calibration Offset Function Implemented\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALOFSI_A {
  #[doc = "0: Offset calibration and offset trimming not implemented."]
  CALOFSI_0,
  #[doc = "1: Offset calibration and offset trimming implemented."]
  CALOFSI_1,
}
impl From<CALOFSI_A> for bool {
  #[inline(always)]
  fn from(variant: CALOFSI_A) -> Self {
    match variant {
      CALOFSI_A::CALOFSI_0 => false,
      CALOFSI_A::CALOFSI_1 => true,
    }
  }
}
#[doc = "Reader of field `CALOFSI`"]
pub type CALOFSI_R = crate::R<bool, CALOFSI_A>;
impl CALOFSI_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CALOFSI_A {
    match self.bits {
      false => CALOFSI_A::CALOFSI_0,
      true => CALOFSI_A::CALOFSI_1,
    }
  }
  #[doc = "Checks if the value of the field is `CALOFSI_0`"]
  #[inline(always)]
  pub fn is_calofsi_0(&self) -> bool {
    *self == CALOFSI_A::CALOFSI_0
  }
  #[doc = "Checks if the value of the field is `CALOFSI_1`"]
  #[inline(always)]
  pub fn is_calofsi_1(&self) -> bool {
    *self == CALOFSI_A::CALOFSI_1
  }
}
#[doc = "Reader of field `MINOR`"]
pub type MINOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAJOR`"]
pub type MAJOR_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bit 0 - Resolution"]
  #[inline(always)]
  pub fn res(&self) -> RES_R {
    RES_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Differential Supported"]
  #[inline(always)]
  pub fn diffen(&self) -> DIFFEN_R {
    DIFFEN_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Multi Vref Implemented"]
  #[inline(always)]
  pub fn mvi(&self) -> MVI_R {
    MVI_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bits 4:6 - Channel Scale Width"]
  #[inline(always)]
  pub fn csw(&self) -> CSW_R {
    CSW_R::new(((self.bits >> 4) & 0x07) as u8)
  }
  #[doc = "Bit 8 - Voltage Reference 1 Range Control Bit Implemented"]
  #[inline(always)]
  pub fn vr1rngi(&self) -> VR1RNGI_R {
    VR1RNGI_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - Internal ADC Clock implemented"]
  #[inline(always)]
  pub fn iadcki(&self) -> IADCKI_R {
    IADCKI_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - Calibration Offset Function Implemented"]
  #[inline(always)]
  pub fn calofsi(&self) -> CALOFSI_R {
    CALOFSI_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bits 16:23 - Minor Version Number"]
  #[inline(always)]
  pub fn minor(&self) -> MINOR_R {
    MINOR_R::new(((self.bits >> 16) & 0xff) as u8)
  }
  #[doc = "Bits 24:31 - Major Version Number"]
  #[inline(always)]
  pub fn major(&self) -> MAJOR_R {
    MAJOR_R::new(((self.bits >> 24) & 0xff) as u8)
  }
}
