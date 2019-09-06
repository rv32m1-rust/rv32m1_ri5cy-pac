#[doc = "Reader of register HOST_CTRL_CAP"]
pub type R = crate::R<u32, super::HOST_CTRL_CAP>;
#[doc = "Reader of field `DDR50_SUPPORT`"]
pub type DDR50_SUPPORT_R = crate::R<bool, bool>;
#[doc = "Max Block Length\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBL_A {
  #[doc = "0: 512 bytes"]
  MBL_0,
  #[doc = "1: 1024 bytes"]
  MBL_1,
  #[doc = "2: 2048 bytes"]
  MBL_2,
  #[doc = "3: 4096 bytes"]
  MBL_3,
}
impl From<MBL_A> for u8 {
  #[inline(always)]
  fn from(variant: MBL_A) -> Self {
    match variant {
      MBL_A::MBL_0 => 0,
      MBL_A::MBL_1 => 1,
      MBL_A::MBL_2 => 2,
      MBL_A::MBL_3 => 3,
    }
  }
}
#[doc = "Reader of field `MBL`"]
pub type MBL_R = crate::R<u8, MBL_A>;
impl MBL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, MBL_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(MBL_A::MBL_0),
      1 => Val(MBL_A::MBL_1),
      2 => Val(MBL_A::MBL_2),
      3 => Val(MBL_A::MBL_3),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `MBL_0`"]
  #[inline(always)]
  pub fn is_mbl_0(&self) -> bool {
    *self == MBL_A::MBL_0
  }
  #[doc = "Checks if the value of the field is `MBL_1`"]
  #[inline(always)]
  pub fn is_mbl_1(&self) -> bool {
    *self == MBL_A::MBL_1
  }
  #[doc = "Checks if the value of the field is `MBL_2`"]
  #[inline(always)]
  pub fn is_mbl_2(&self) -> bool {
    *self == MBL_A::MBL_2
  }
  #[doc = "Checks if the value of the field is `MBL_3`"]
  #[inline(always)]
  pub fn is_mbl_3(&self) -> bool {
    *self == MBL_A::MBL_3
  }
}
#[doc = "ADMA Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADMAS_A {
  #[doc = "0: Advanced DMA Not supported"]
  ADMAS_0,
  #[doc = "1: Advanced DMA Supported"]
  ADMAS_1,
}
impl From<ADMAS_A> for bool {
  #[inline(always)]
  fn from(variant: ADMAS_A) -> Self {
    match variant {
      ADMAS_A::ADMAS_0 => false,
      ADMAS_A::ADMAS_1 => true,
    }
  }
}
#[doc = "Reader of field `ADMAS`"]
pub type ADMAS_R = crate::R<bool, ADMAS_A>;
impl ADMAS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ADMAS_A {
    match self.bits {
      false => ADMAS_A::ADMAS_0,
      true => ADMAS_A::ADMAS_1,
    }
  }
  #[doc = "Checks if the value of the field is `ADMAS_0`"]
  #[inline(always)]
  pub fn is_admas_0(&self) -> bool {
    *self == ADMAS_A::ADMAS_0
  }
  #[doc = "Checks if the value of the field is `ADMAS_1`"]
  #[inline(always)]
  pub fn is_admas_1(&self) -> bool {
    *self == ADMAS_A::ADMAS_1
  }
}
#[doc = "High Speed Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSS_A {
  #[doc = "0: High Speed Not Supported"]
  HSS_0,
  #[doc = "1: High Speed Supported"]
  HSS_1,
}
impl From<HSS_A> for bool {
  #[inline(always)]
  fn from(variant: HSS_A) -> Self {
    match variant {
      HSS_A::HSS_0 => false,
      HSS_A::HSS_1 => true,
    }
  }
}
#[doc = "Reader of field `HSS`"]
pub type HSS_R = crate::R<bool, HSS_A>;
impl HSS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> HSS_A {
    match self.bits {
      false => HSS_A::HSS_0,
      true => HSS_A::HSS_1,
    }
  }
  #[doc = "Checks if the value of the field is `HSS_0`"]
  #[inline(always)]
  pub fn is_hss_0(&self) -> bool {
    *self == HSS_A::HSS_0
  }
  #[doc = "Checks if the value of the field is `HSS_1`"]
  #[inline(always)]
  pub fn is_hss_1(&self) -> bool {
    *self == HSS_A::HSS_1
  }
}
#[doc = "DMA Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAS_A {
  #[doc = "0: DMA not supported"]
  DMAS_0,
  #[doc = "1: DMA Supported"]
  DMAS_1,
}
impl From<DMAS_A> for bool {
  #[inline(always)]
  fn from(variant: DMAS_A) -> Self {
    match variant {
      DMAS_A::DMAS_0 => false,
      DMAS_A::DMAS_1 => true,
    }
  }
}
#[doc = "Reader of field `DMAS`"]
pub type DMAS_R = crate::R<bool, DMAS_A>;
impl DMAS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DMAS_A {
    match self.bits {
      false => DMAS_A::DMAS_0,
      true => DMAS_A::DMAS_1,
    }
  }
  #[doc = "Checks if the value of the field is `DMAS_0`"]
  #[inline(always)]
  pub fn is_dmas_0(&self) -> bool {
    *self == DMAS_A::DMAS_0
  }
  #[doc = "Checks if the value of the field is `DMAS_1`"]
  #[inline(always)]
  pub fn is_dmas_1(&self) -> bool {
    *self == DMAS_A::DMAS_1
  }
}
#[doc = "Suspend / Resume Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRS_A {
  #[doc = "0: Not supported"]
  SRS_0,
  #[doc = "1: Supported"]
  SRS_1,
}
impl From<SRS_A> for bool {
  #[inline(always)]
  fn from(variant: SRS_A) -> Self {
    match variant {
      SRS_A::SRS_0 => false,
      SRS_A::SRS_1 => true,
    }
  }
}
#[doc = "Reader of field `SRS`"]
pub type SRS_R = crate::R<bool, SRS_A>;
impl SRS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SRS_A {
    match self.bits {
      false => SRS_A::SRS_0,
      true => SRS_A::SRS_1,
    }
  }
  #[doc = "Checks if the value of the field is `SRS_0`"]
  #[inline(always)]
  pub fn is_srs_0(&self) -> bool {
    *self == SRS_A::SRS_0
  }
  #[doc = "Checks if the value of the field is `SRS_1`"]
  #[inline(always)]
  pub fn is_srs_1(&self) -> bool {
    *self == SRS_A::SRS_1
  }
}
#[doc = "Voltage Support 3.3V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VS33_A {
  #[doc = "0: 3.3V not supported"]
  VS33_0,
  #[doc = "1: 3.3V supported"]
  VS33_1,
}
impl From<VS33_A> for bool {
  #[inline(always)]
  fn from(variant: VS33_A) -> Self {
    match variant {
      VS33_A::VS33_0 => false,
      VS33_A::VS33_1 => true,
    }
  }
}
#[doc = "Reader of field `VS33`"]
pub type VS33_R = crate::R<bool, VS33_A>;
impl VS33_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VS33_A {
    match self.bits {
      false => VS33_A::VS33_0,
      true => VS33_A::VS33_1,
    }
  }
  #[doc = "Checks if the value of the field is `VS33_0`"]
  #[inline(always)]
  pub fn is_vs33_0(&self) -> bool {
    *self == VS33_A::VS33_0
  }
  #[doc = "Checks if the value of the field is `VS33_1`"]
  #[inline(always)]
  pub fn is_vs33_1(&self) -> bool {
    *self == VS33_A::VS33_1
  }
}
#[doc = "Voltage Support 3.0 V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VS30_A {
  #[doc = "0: 3.0V not supported"]
  VS30_0,
  #[doc = "1: 3.0V supported"]
  VS30_1,
}
impl From<VS30_A> for bool {
  #[inline(always)]
  fn from(variant: VS30_A) -> Self {
    match variant {
      VS30_A::VS30_0 => false,
      VS30_A::VS30_1 => true,
    }
  }
}
#[doc = "Reader of field `VS30`"]
pub type VS30_R = crate::R<bool, VS30_A>;
impl VS30_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VS30_A {
    match self.bits {
      false => VS30_A::VS30_0,
      true => VS30_A::VS30_1,
    }
  }
  #[doc = "Checks if the value of the field is `VS30_0`"]
  #[inline(always)]
  pub fn is_vs30_0(&self) -> bool {
    *self == VS30_A::VS30_0
  }
  #[doc = "Checks if the value of the field is `VS30_1`"]
  #[inline(always)]
  pub fn is_vs30_1(&self) -> bool {
    *self == VS30_A::VS30_1
  }
}
#[doc = "Voltage Support 1.8 V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VS18_A {
  #[doc = "0: 1.8V not supported"]
  VS18_0,
  #[doc = "1: 1.8V supported"]
  VS18_1,
}
impl From<VS18_A> for bool {
  #[inline(always)]
  fn from(variant: VS18_A) -> Self {
    match variant {
      VS18_A::VS18_0 => false,
      VS18_A::VS18_1 => true,
    }
  }
}
#[doc = "Reader of field `VS18`"]
pub type VS18_R = crate::R<bool, VS18_A>;
impl VS18_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VS18_A {
    match self.bits {
      false => VS18_A::VS18_0,
      true => VS18_A::VS18_1,
    }
  }
  #[doc = "Checks if the value of the field is `VS18_0`"]
  #[inline(always)]
  pub fn is_vs18_0(&self) -> bool {
    *self == VS18_A::VS18_0
  }
  #[doc = "Checks if the value of the field is `VS18_1`"]
  #[inline(always)]
  pub fn is_vs18_1(&self) -> bool {
    *self == VS18_A::VS18_1
  }
}
impl R {
  #[doc = "Bit 2 - DDR50 support"]
  #[inline(always)]
  pub fn ddr50_support(&self) -> DDR50_SUPPORT_R {
    DDR50_SUPPORT_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bits 16:18 - Max Block Length"]
  #[inline(always)]
  pub fn mbl(&self) -> MBL_R {
    MBL_R::new(((self.bits >> 16) & 0x07) as u8)
  }
  #[doc = "Bit 20 - ADMA Support"]
  #[inline(always)]
  pub fn admas(&self) -> ADMAS_R {
    ADMAS_R::new(((self.bits >> 20) & 0x01) != 0)
  }
  #[doc = "Bit 21 - High Speed Support"]
  #[inline(always)]
  pub fn hss(&self) -> HSS_R {
    HSS_R::new(((self.bits >> 21) & 0x01) != 0)
  }
  #[doc = "Bit 22 - DMA Support"]
  #[inline(always)]
  pub fn dmas(&self) -> DMAS_R {
    DMAS_R::new(((self.bits >> 22) & 0x01) != 0)
  }
  #[doc = "Bit 23 - Suspend / Resume Support"]
  #[inline(always)]
  pub fn srs(&self) -> SRS_R {
    SRS_R::new(((self.bits >> 23) & 0x01) != 0)
  }
  #[doc = "Bit 24 - Voltage Support 3.3V"]
  #[inline(always)]
  pub fn vs33(&self) -> VS33_R {
    VS33_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - Voltage Support 3.0 V"]
  #[inline(always)]
  pub fn vs30(&self) -> VS30_R {
    VS30_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - Voltage Support 1.8 V"]
  #[inline(always)]
  pub fn vs18(&self) -> VS18_R {
    VS18_R::new(((self.bits >> 26) & 0x01) != 0)
  }
}
