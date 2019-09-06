#[doc = "Reader of register PRES_STATE"]
pub type R = crate::R<u32, super::PRES_STATE>;
#[doc = "Command Inhibit (CMD)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIHB_A {
  #[doc = "0: Can issue command using only CMD line"]
  CIHB_0,
  #[doc = "1: Cannot issue command"]
  CIHB_1,
}
impl From<CIHB_A> for bool {
  #[inline(always)]
  fn from(variant: CIHB_A) -> Self {
    match variant {
      CIHB_A::CIHB_0 => false,
      CIHB_A::CIHB_1 => true,
    }
  }
}
#[doc = "Reader of field `CIHB`"]
pub type CIHB_R = crate::R<bool, CIHB_A>;
impl CIHB_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CIHB_A {
    match self.bits {
      false => CIHB_A::CIHB_0,
      true => CIHB_A::CIHB_1,
    }
  }
  #[doc = "Checks if the value of the field is `CIHB_0`"]
  #[inline(always)]
  pub fn is_cihb_0(&self) -> bool {
    *self == CIHB_A::CIHB_0
  }
  #[doc = "Checks if the value of the field is `CIHB_1`"]
  #[inline(always)]
  pub fn is_cihb_1(&self) -> bool {
    *self == CIHB_A::CIHB_1
  }
}
#[doc = "Command Inhibit (DATA)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDIHB_A {
  #[doc = "0: Can issue command which uses the DATA line"]
  CDIHB_0,
  #[doc = "1: Cannot issue command which uses the DATA line"]
  CDIHB_1,
}
impl From<CDIHB_A> for bool {
  #[inline(always)]
  fn from(variant: CDIHB_A) -> Self {
    match variant {
      CDIHB_A::CDIHB_0 => false,
      CDIHB_A::CDIHB_1 => true,
    }
  }
}
#[doc = "Reader of field `CDIHB`"]
pub type CDIHB_R = crate::R<bool, CDIHB_A>;
impl CDIHB_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CDIHB_A {
    match self.bits {
      false => CDIHB_A::CDIHB_0,
      true => CDIHB_A::CDIHB_1,
    }
  }
  #[doc = "Checks if the value of the field is `CDIHB_0`"]
  #[inline(always)]
  pub fn is_cdihb_0(&self) -> bool {
    *self == CDIHB_A::CDIHB_0
  }
  #[doc = "Checks if the value of the field is `CDIHB_1`"]
  #[inline(always)]
  pub fn is_cdihb_1(&self) -> bool {
    *self == CDIHB_A::CDIHB_1
  }
}
#[doc = "Data Line Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLA_A {
  #[doc = "0: DATA Line Inactive"]
  DLA_0,
  #[doc = "1: DATA Line Active"]
  DLA_1,
}
impl From<DLA_A> for bool {
  #[inline(always)]
  fn from(variant: DLA_A) -> Self {
    match variant {
      DLA_A::DLA_0 => false,
      DLA_A::DLA_1 => true,
    }
  }
}
#[doc = "Reader of field `DLA`"]
pub type DLA_R = crate::R<bool, DLA_A>;
impl DLA_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DLA_A {
    match self.bits {
      false => DLA_A::DLA_0,
      true => DLA_A::DLA_1,
    }
  }
  #[doc = "Checks if the value of the field is `DLA_0`"]
  #[inline(always)]
  pub fn is_dla_0(&self) -> bool {
    *self == DLA_A::DLA_0
  }
  #[doc = "Checks if the value of the field is `DLA_1`"]
  #[inline(always)]
  pub fn is_dla_1(&self) -> bool {
    *self == DLA_A::DLA_1
  }
}
#[doc = "SD Clock Stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDSTB_A {
  #[doc = "0: Clock is changing frequency and not stable."]
  SDSTB_0,
  #[doc = "1: Clock is stable."]
  SDSTB_1,
}
impl From<SDSTB_A> for bool {
  #[inline(always)]
  fn from(variant: SDSTB_A) -> Self {
    match variant {
      SDSTB_A::SDSTB_0 => false,
      SDSTB_A::SDSTB_1 => true,
    }
  }
}
#[doc = "Reader of field `SDSTB`"]
pub type SDSTB_R = crate::R<bool, SDSTB_A>;
impl SDSTB_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SDSTB_A {
    match self.bits {
      false => SDSTB_A::SDSTB_0,
      true => SDSTB_A::SDSTB_1,
    }
  }
  #[doc = "Checks if the value of the field is `SDSTB_0`"]
  #[inline(always)]
  pub fn is_sdstb_0(&self) -> bool {
    *self == SDSTB_A::SDSTB_0
  }
  #[doc = "Checks if the value of the field is `SDSTB_1`"]
  #[inline(always)]
  pub fn is_sdstb_1(&self) -> bool {
    *self == SDSTB_A::SDSTB_1
  }
}
#[doc = "IPG_CLK Gated Off Internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPGOFF_A {
  #[doc = "0: IPG_CLK is active."]
  IPGOFF_0,
  #[doc = "1: IPG_CLK is gated off."]
  IPGOFF_1,
}
impl From<IPGOFF_A> for bool {
  #[inline(always)]
  fn from(variant: IPGOFF_A) -> Self {
    match variant {
      IPGOFF_A::IPGOFF_0 => false,
      IPGOFF_A::IPGOFF_1 => true,
    }
  }
}
#[doc = "Reader of field `IPGOFF`"]
pub type IPGOFF_R = crate::R<bool, IPGOFF_A>;
impl IPGOFF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> IPGOFF_A {
    match self.bits {
      false => IPGOFF_A::IPGOFF_0,
      true => IPGOFF_A::IPGOFF_1,
    }
  }
  #[doc = "Checks if the value of the field is `IPGOFF_0`"]
  #[inline(always)]
  pub fn is_ipgoff_0(&self) -> bool {
    *self == IPGOFF_A::IPGOFF_0
  }
  #[doc = "Checks if the value of the field is `IPGOFF_1`"]
  #[inline(always)]
  pub fn is_ipgoff_1(&self) -> bool {
    *self == IPGOFF_A::IPGOFF_1
  }
}
#[doc = "HCLK Gated Off Internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCKOFF_A {
  #[doc = "0: HCLK is active."]
  HCKOFF_0,
  #[doc = "1: HCLK is gated off."]
  HCKOFF_1,
}
impl From<HCKOFF_A> for bool {
  #[inline(always)]
  fn from(variant: HCKOFF_A) -> Self {
    match variant {
      HCKOFF_A::HCKOFF_0 => false,
      HCKOFF_A::HCKOFF_1 => true,
    }
  }
}
#[doc = "Reader of field `HCKOFF`"]
pub type HCKOFF_R = crate::R<bool, HCKOFF_A>;
impl HCKOFF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> HCKOFF_A {
    match self.bits {
      false => HCKOFF_A::HCKOFF_0,
      true => HCKOFF_A::HCKOFF_1,
    }
  }
  #[doc = "Checks if the value of the field is `HCKOFF_0`"]
  #[inline(always)]
  pub fn is_hckoff_0(&self) -> bool {
    *self == HCKOFF_A::HCKOFF_0
  }
  #[doc = "Checks if the value of the field is `HCKOFF_1`"]
  #[inline(always)]
  pub fn is_hckoff_1(&self) -> bool {
    *self == HCKOFF_A::HCKOFF_1
  }
}
#[doc = "IPG_PERCLK Gated Off Internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEROFF_A {
  #[doc = "0: IPG_PERCLK is active."]
  PEROFF_0,
  #[doc = "1: IPG_PERCLK is gated off."]
  PEROFF_1,
}
impl From<PEROFF_A> for bool {
  #[inline(always)]
  fn from(variant: PEROFF_A) -> Self {
    match variant {
      PEROFF_A::PEROFF_0 => false,
      PEROFF_A::PEROFF_1 => true,
    }
  }
}
#[doc = "Reader of field `PEROFF`"]
pub type PEROFF_R = crate::R<bool, PEROFF_A>;
impl PEROFF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PEROFF_A {
    match self.bits {
      false => PEROFF_A::PEROFF_0,
      true => PEROFF_A::PEROFF_1,
    }
  }
  #[doc = "Checks if the value of the field is `PEROFF_0`"]
  #[inline(always)]
  pub fn is_peroff_0(&self) -> bool {
    *self == PEROFF_A::PEROFF_0
  }
  #[doc = "Checks if the value of the field is `PEROFF_1`"]
  #[inline(always)]
  pub fn is_peroff_1(&self) -> bool {
    *self == PEROFF_A::PEROFF_1
  }
}
#[doc = "SD Clock Gated Off Internally\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDOFF_A {
  #[doc = "0: SD Clock is active."]
  SDOFF_0,
  #[doc = "1: SD Clock is gated off."]
  SDOFF_1,
}
impl From<SDOFF_A> for bool {
  #[inline(always)]
  fn from(variant: SDOFF_A) -> Self {
    match variant {
      SDOFF_A::SDOFF_0 => false,
      SDOFF_A::SDOFF_1 => true,
    }
  }
}
#[doc = "Reader of field `SDOFF`"]
pub type SDOFF_R = crate::R<bool, SDOFF_A>;
impl SDOFF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SDOFF_A {
    match self.bits {
      false => SDOFF_A::SDOFF_0,
      true => SDOFF_A::SDOFF_1,
    }
  }
  #[doc = "Checks if the value of the field is `SDOFF_0`"]
  #[inline(always)]
  pub fn is_sdoff_0(&self) -> bool {
    *self == SDOFF_A::SDOFF_0
  }
  #[doc = "Checks if the value of the field is `SDOFF_1`"]
  #[inline(always)]
  pub fn is_sdoff_1(&self) -> bool {
    *self == SDOFF_A::SDOFF_1
  }
}
#[doc = "Write Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTA_A {
  #[doc = "0: No valid data"]
  WTA_0,
  #[doc = "1: Transferring data"]
  WTA_1,
}
impl From<WTA_A> for bool {
  #[inline(always)]
  fn from(variant: WTA_A) -> Self {
    match variant {
      WTA_A::WTA_0 => false,
      WTA_A::WTA_1 => true,
    }
  }
}
#[doc = "Reader of field `WTA`"]
pub type WTA_R = crate::R<bool, WTA_A>;
impl WTA_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WTA_A {
    match self.bits {
      false => WTA_A::WTA_0,
      true => WTA_A::WTA_1,
    }
  }
  #[doc = "Checks if the value of the field is `WTA_0`"]
  #[inline(always)]
  pub fn is_wta_0(&self) -> bool {
    *self == WTA_A::WTA_0
  }
  #[doc = "Checks if the value of the field is `WTA_1`"]
  #[inline(always)]
  pub fn is_wta_1(&self) -> bool {
    *self == WTA_A::WTA_1
  }
}
#[doc = "Read Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTA_A {
  #[doc = "0: No valid data"]
  RTA_0,
  #[doc = "1: Transferring data"]
  RTA_1,
}
impl From<RTA_A> for bool {
  #[inline(always)]
  fn from(variant: RTA_A) -> Self {
    match variant {
      RTA_A::RTA_0 => false,
      RTA_A::RTA_1 => true,
    }
  }
}
#[doc = "Reader of field `RTA`"]
pub type RTA_R = crate::R<bool, RTA_A>;
impl RTA_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RTA_A {
    match self.bits {
      false => RTA_A::RTA_0,
      true => RTA_A::RTA_1,
    }
  }
  #[doc = "Checks if the value of the field is `RTA_0`"]
  #[inline(always)]
  pub fn is_rta_0(&self) -> bool {
    *self == RTA_A::RTA_0
  }
  #[doc = "Checks if the value of the field is `RTA_1`"]
  #[inline(always)]
  pub fn is_rta_1(&self) -> bool {
    *self == RTA_A::RTA_1
  }
}
#[doc = "Buffer Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWEN_A {
  #[doc = "0: Write disable"]
  BWEN_0,
  #[doc = "1: Write enable"]
  BWEN_1,
}
impl From<BWEN_A> for bool {
  #[inline(always)]
  fn from(variant: BWEN_A) -> Self {
    match variant {
      BWEN_A::BWEN_0 => false,
      BWEN_A::BWEN_1 => true,
    }
  }
}
#[doc = "Reader of field `BWEN`"]
pub type BWEN_R = crate::R<bool, BWEN_A>;
impl BWEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BWEN_A {
    match self.bits {
      false => BWEN_A::BWEN_0,
      true => BWEN_A::BWEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `BWEN_0`"]
  #[inline(always)]
  pub fn is_bwen_0(&self) -> bool {
    *self == BWEN_A::BWEN_0
  }
  #[doc = "Checks if the value of the field is `BWEN_1`"]
  #[inline(always)]
  pub fn is_bwen_1(&self) -> bool {
    *self == BWEN_A::BWEN_1
  }
}
#[doc = "Buffer Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BREN_A {
  #[doc = "0: Read disable"]
  BREN_0,
  #[doc = "1: Read enable"]
  BREN_1,
}
impl From<BREN_A> for bool {
  #[inline(always)]
  fn from(variant: BREN_A) -> Self {
    match variant {
      BREN_A::BREN_0 => false,
      BREN_A::BREN_1 => true,
    }
  }
}
#[doc = "Reader of field `BREN`"]
pub type BREN_R = crate::R<bool, BREN_A>;
impl BREN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> BREN_A {
    match self.bits {
      false => BREN_A::BREN_0,
      true => BREN_A::BREN_1,
    }
  }
  #[doc = "Checks if the value of the field is `BREN_0`"]
  #[inline(always)]
  pub fn is_bren_0(&self) -> bool {
    *self == BREN_A::BREN_0
  }
  #[doc = "Checks if the value of the field is `BREN_1`"]
  #[inline(always)]
  pub fn is_bren_1(&self) -> bool {
    *self == BREN_A::BREN_1
  }
}
#[doc = "Card Inserted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINST_A {
  #[doc = "0: Power on Reset or No Card"]
  CINST_0,
  #[doc = "1: Card Inserted"]
  CINST_1,
}
impl From<CINST_A> for bool {
  #[inline(always)]
  fn from(variant: CINST_A) -> Self {
    match variant {
      CINST_A::CINST_0 => false,
      CINST_A::CINST_1 => true,
    }
  }
}
#[doc = "Reader of field `CINST`"]
pub type CINST_R = crate::R<bool, CINST_A>;
impl CINST_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CINST_A {
    match self.bits {
      false => CINST_A::CINST_0,
      true => CINST_A::CINST_1,
    }
  }
  #[doc = "Checks if the value of the field is `CINST_0`"]
  #[inline(always)]
  pub fn is_cinst_0(&self) -> bool {
    *self == CINST_A::CINST_0
  }
  #[doc = "Checks if the value of the field is `CINST_1`"]
  #[inline(always)]
  pub fn is_cinst_1(&self) -> bool {
    *self == CINST_A::CINST_1
  }
}
#[doc = "Card Detect Pin Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDPL_A {
  #[doc = "0: No card present (CD_B = 1)"]
  CDPL_0,
  #[doc = "1: Card present (CD_B = 0)"]
  CDPL_1,
}
impl From<CDPL_A> for bool {
  #[inline(always)]
  fn from(variant: CDPL_A) -> Self {
    match variant {
      CDPL_A::CDPL_0 => false,
      CDPL_A::CDPL_1 => true,
    }
  }
}
#[doc = "Reader of field `CDPL`"]
pub type CDPL_R = crate::R<bool, CDPL_A>;
impl CDPL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> CDPL_A {
    match self.bits {
      false => CDPL_A::CDPL_0,
      true => CDPL_A::CDPL_1,
    }
  }
  #[doc = "Checks if the value of the field is `CDPL_0`"]
  #[inline(always)]
  pub fn is_cdpl_0(&self) -> bool {
    *self == CDPL_A::CDPL_0
  }
  #[doc = "Checks if the value of the field is `CDPL_1`"]
  #[inline(always)]
  pub fn is_cdpl_1(&self) -> bool {
    *self == CDPL_A::CDPL_1
  }
}
#[doc = "Write Protect Switch Pin Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPSPL_A {
  #[doc = "0: Write protected (WP = 1)"]
  WPSPL_0,
  #[doc = "1: Write enabled (WP = 0)"]
  WPSPL_1,
}
impl From<WPSPL_A> for bool {
  #[inline(always)]
  fn from(variant: WPSPL_A) -> Self {
    match variant {
      WPSPL_A::WPSPL_0 => false,
      WPSPL_A::WPSPL_1 => true,
    }
  }
}
#[doc = "Reader of field `WPSPL`"]
pub type WPSPL_R = crate::R<bool, WPSPL_A>;
impl WPSPL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WPSPL_A {
    match self.bits {
      false => WPSPL_A::WPSPL_0,
      true => WPSPL_A::WPSPL_1,
    }
  }
  #[doc = "Checks if the value of the field is `WPSPL_0`"]
  #[inline(always)]
  pub fn is_wpspl_0(&self) -> bool {
    *self == WPSPL_A::WPSPL_0
  }
  #[doc = "Checks if the value of the field is `WPSPL_1`"]
  #[inline(always)]
  pub fn is_wpspl_1(&self) -> bool {
    *self == WPSPL_A::WPSPL_1
  }
}
#[doc = "Reader of field `CLSL`"]
pub type CLSL_R = crate::R<bool, bool>;
#[doc = "DATA\\[7:0\\] Line Signal Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLSL_A {
  #[doc = "0: Data 0 line signal level"]
  DATA0,
  #[doc = "1: Data 1 line signal level"]
  DATA1,
  #[doc = "2: Data 2 line signal level"]
  DATA2,
  #[doc = "3: Data 3 line signal level"]
  DATA3,
  #[doc = "4: Data 4 line signal level"]
  DATA4,
  #[doc = "5: Data 5 line signal level"]
  DATA5,
  #[doc = "6: Data 6 line signal level"]
  DATA6,
  #[doc = "7: Data 7 line signal level"]
  DATA7,
}
impl From<DLSL_A> for u8 {
  #[inline(always)]
  fn from(variant: DLSL_A) -> Self {
    match variant {
      DLSL_A::DATA0 => 0,
      DLSL_A::DATA1 => 1,
      DLSL_A::DATA2 => 2,
      DLSL_A::DATA3 => 3,
      DLSL_A::DATA4 => 4,
      DLSL_A::DATA5 => 5,
      DLSL_A::DATA6 => 6,
      DLSL_A::DATA7 => 7,
    }
  }
}
#[doc = "Reader of field `DLSL`"]
pub type DLSL_R = crate::R<u8, DLSL_A>;
impl DLSL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, DLSL_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(DLSL_A::DATA0),
      1 => Val(DLSL_A::DATA1),
      2 => Val(DLSL_A::DATA2),
      3 => Val(DLSL_A::DATA3),
      4 => Val(DLSL_A::DATA4),
      5 => Val(DLSL_A::DATA5),
      6 => Val(DLSL_A::DATA6),
      7 => Val(DLSL_A::DATA7),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `DATA0`"]
  #[inline(always)]
  pub fn is_data0(&self) -> bool {
    *self == DLSL_A::DATA0
  }
  #[doc = "Checks if the value of the field is `DATA1`"]
  #[inline(always)]
  pub fn is_data1(&self) -> bool {
    *self == DLSL_A::DATA1
  }
  #[doc = "Checks if the value of the field is `DATA2`"]
  #[inline(always)]
  pub fn is_data2(&self) -> bool {
    *self == DLSL_A::DATA2
  }
  #[doc = "Checks if the value of the field is `DATA3`"]
  #[inline(always)]
  pub fn is_data3(&self) -> bool {
    *self == DLSL_A::DATA3
  }
  #[doc = "Checks if the value of the field is `DATA4`"]
  #[inline(always)]
  pub fn is_data4(&self) -> bool {
    *self == DLSL_A::DATA4
  }
  #[doc = "Checks if the value of the field is `DATA5`"]
  #[inline(always)]
  pub fn is_data5(&self) -> bool {
    *self == DLSL_A::DATA5
  }
  #[doc = "Checks if the value of the field is `DATA6`"]
  #[inline(always)]
  pub fn is_data6(&self) -> bool {
    *self == DLSL_A::DATA6
  }
  #[doc = "Checks if the value of the field is `DATA7`"]
  #[inline(always)]
  pub fn is_data7(&self) -> bool {
    *self == DLSL_A::DATA7
  }
}
impl R {
  #[doc = "Bit 0 - Command Inhibit (CMD)"]
  #[inline(always)]
  pub fn cihb(&self) -> CIHB_R {
    CIHB_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Command Inhibit (DATA)"]
  #[inline(always)]
  pub fn cdihb(&self) -> CDIHB_R {
    CDIHB_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Data Line Active"]
  #[inline(always)]
  pub fn dla(&self) -> DLA_R {
    DLA_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - SD Clock Stable"]
  #[inline(always)]
  pub fn sdstb(&self) -> SDSTB_R {
    SDSTB_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - IPG_CLK Gated Off Internally"]
  #[inline(always)]
  pub fn ipgoff(&self) -> IPGOFF_R {
    IPGOFF_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - HCLK Gated Off Internally"]
  #[inline(always)]
  pub fn hckoff(&self) -> HCKOFF_R {
    HCKOFF_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - IPG_PERCLK Gated Off Internally"]
  #[inline(always)]
  pub fn peroff(&self) -> PEROFF_R {
    PEROFF_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - SD Clock Gated Off Internally"]
  #[inline(always)]
  pub fn sdoff(&self) -> SDOFF_R {
    SDOFF_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Write Transfer Active"]
  #[inline(always)]
  pub fn wta(&self) -> WTA_R {
    WTA_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - Read Transfer Active"]
  #[inline(always)]
  pub fn rta(&self) -> RTA_R {
    RTA_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - Buffer Write Enable"]
  #[inline(always)]
  pub fn bwen(&self) -> BWEN_R {
    BWEN_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 11 - Buffer Read Enable"]
  #[inline(always)]
  pub fn bren(&self) -> BREN_R {
    BREN_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bit 16 - Card Inserted"]
  #[inline(always)]
  pub fn cinst(&self) -> CINST_R {
    CINST_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 18 - Card Detect Pin Level"]
  #[inline(always)]
  pub fn cdpl(&self) -> CDPL_R {
    CDPL_R::new(((self.bits >> 18) & 0x01) != 0)
  }
  #[doc = "Bit 19 - Write Protect Switch Pin Level"]
  #[inline(always)]
  pub fn wpspl(&self) -> WPSPL_R {
    WPSPL_R::new(((self.bits >> 19) & 0x01) != 0)
  }
  #[doc = "Bit 23 - CMD Line Signal Level"]
  #[inline(always)]
  pub fn clsl(&self) -> CLSL_R {
    CLSL_R::new(((self.bits >> 23) & 0x01) != 0)
  }
  #[doc = "Bits 24:31 - DATA\\[7:0\\] Line Signal Level"]
  #[inline(always)]
  pub fn dlsl(&self) -> DLSL_R {
    DLSL_R::new(((self.bits >> 24) & 0xff) as u8)
  }
}
