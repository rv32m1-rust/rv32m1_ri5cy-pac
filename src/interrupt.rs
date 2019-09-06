#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
  #[doc = "0 - CTI0_MCM0"]
  CTI0_MCM0,
  #[doc = "1 - DMA0"]
  DMA0,
  #[doc = "2 - DMA1"]
  DMA1,
  #[doc = "3 - DMA2"]
  DMA2,
  #[doc = "4 - DMA3"]
  DMA3,
  #[doc = "5 - DMA4"]
  DMA4,
  #[doc = "6 - DMA5"]
  DMA5,
  #[doc = "7 - DMA6"]
  DMA6,
  #[doc = "8 - DMA7"]
  DMA7,
  #[doc = "9 - DMA8"]
  DMA8,
  #[doc = "10 - DMA9"]
  DMA9,
  #[doc = "11 - DMA10"]
  DMA10,
  #[doc = "12 - DMA11"]
  DMA11,
  #[doc = "13 - DMA12"]
  DMA12,
  #[doc = "14 - DMA13"]
  DMA13,
  #[doc = "15 - DMA14"]
  DMA14,
  #[doc = "16 - DMA15"]
  DMA15,
  #[doc = "17 - DMA0_Error"]
  DMA0_ERROR,
  #[doc = "18 - CMC0"]
  CMC0,
  #[doc = "19 - EWM"]
  EWM,
  #[doc = "20 - FTFE_Command_Complete"]
  FTFE_COMMAND_COMPLETE,
  #[doc = "21 - FTFE_Read_Collision"]
  FTFE_READ_COLLISION,
  #[doc = "22 - LLWU0"]
  LLWU0,
  #[doc = "23 - MUA"]
  MUA,
  #[doc = "24 - SPM"]
  SPM,
  #[doc = "25 - WDOG0"]
  WDOG0,
  #[doc = "26 - SCG"]
  SCG,
  #[doc = "27 - LPIT0"]
  LPIT0,
  #[doc = "28 - RTC"]
  RTC,
  #[doc = "29 - LPTMR0"]
  LPTMR0,
  #[doc = "30 - LPTMR1"]
  LPTMR1,
  #[doc = "31 - TPM0"]
  TPM0,
  #[doc = "32 - TPM1"]
  TPM1,
  #[doc = "33 - TPM2"]
  TPM2,
  #[doc = "34 - EMVSIM0"]
  EMVSIM0,
  #[doc = "35 - FLEXIO0"]
  FLEXIO0,
  #[doc = "36 - LPI2C0"]
  LPI2C0,
  #[doc = "37 - LPI2C1"]
  LPI2C1,
  #[doc = "38 - LPI2C2"]
  LPI2C2,
  #[doc = "39 - I2S0"]
  I2S0,
  #[doc = "40 - USDHC0"]
  USDHC0,
  #[doc = "41 - LPSPI0"]
  LPSPI0,
  #[doc = "42 - LPSPI1"]
  LPSPI1,
  #[doc = "43 - LPSPI2"]
  LPSPI2,
  #[doc = "44 - LPUART0"]
  LPUART0,
  #[doc = "45 - LPUART1"]
  LPUART1,
  #[doc = "46 - LPUART2"]
  LPUART2,
  #[doc = "47 - USB0"]
  USB0,
  #[doc = "48 - PORTA"]
  PORTA,
  #[doc = "49 - PORTB"]
  PORTB,
  #[doc = "50 - PORTC"]
  PORTC,
  #[doc = "51 - PORTD"]
  PORTD,
  #[doc = "52 - ADC0"]
  ADC0,
  #[doc = "53 - LPCMP0"]
  LPCMP0,
  #[doc = "54 - LPDAC0"]
  LPDAC0,
  #[doc = "55 - CAU3_Task_Complete"]
  CAU3_TASK_COMPLETE,
  #[doc = "56 - CAU3_Security_Violation"]
  CAU3_SECURITY_VIOLATION,
  #[doc = "57 - TRNG"]
  TRNG,
  #[doc = "58 - LPIT1"]
  LPIT1,
  #[doc = "59 - LPTMR2"]
  LPTMR2,
  #[doc = "60 - TPM3"]
  TPM3,
  #[doc = "61 - LPI2C3"]
  LPI2C3,
  #[doc = "62 - LPSPI3"]
  LPSPI3,
  #[doc = "63 - LPUART3"]
  LPUART3,
  #[doc = "64 - PORTE"]
  PORTE,
  #[doc = "65 - LPCMP1"]
  LPCMP1,
}
unsafe impl bare_metal::Nr for Interrupt {
  #[inline]
  fn nr(&self) -> u8 {
    match *self {
      Interrupt::CTI0_MCM0 => 0,
      Interrupt::DMA0 => 1,
      Interrupt::DMA1 => 2,
      Interrupt::DMA2 => 3,
      Interrupt::DMA3 => 4,
      Interrupt::DMA4 => 5,
      Interrupt::DMA5 => 6,
      Interrupt::DMA6 => 7,
      Interrupt::DMA7 => 8,
      Interrupt::DMA8 => 9,
      Interrupt::DMA9 => 10,
      Interrupt::DMA10 => 11,
      Interrupt::DMA11 => 12,
      Interrupt::DMA12 => 13,
      Interrupt::DMA13 => 14,
      Interrupt::DMA14 => 15,
      Interrupt::DMA15 => 16,
      Interrupt::DMA0_ERROR => 17,
      Interrupt::CMC0 => 18,
      Interrupt::EWM => 19,
      Interrupt::FTFE_COMMAND_COMPLETE => 20,
      Interrupt::FTFE_READ_COLLISION => 21,
      Interrupt::LLWU0 => 22,
      Interrupt::MUA => 23,
      Interrupt::SPM => 24,
      Interrupt::WDOG0 => 25,
      Interrupt::SCG => 26,
      Interrupt::LPIT0 => 27,
      Interrupt::RTC => 28,
      Interrupt::LPTMR0 => 29,
      Interrupt::LPTMR1 => 30,
      Interrupt::TPM0 => 31,
      Interrupt::TPM1 => 32,
      Interrupt::TPM2 => 33,
      Interrupt::EMVSIM0 => 34,
      Interrupt::FLEXIO0 => 35,
      Interrupt::LPI2C0 => 36,
      Interrupt::LPI2C1 => 37,
      Interrupt::LPI2C2 => 38,
      Interrupt::I2S0 => 39,
      Interrupt::USDHC0 => 40,
      Interrupt::LPSPI0 => 41,
      Interrupt::LPSPI1 => 42,
      Interrupt::LPSPI2 => 43,
      Interrupt::LPUART0 => 44,
      Interrupt::LPUART1 => 45,
      Interrupt::LPUART2 => 46,
      Interrupt::USB0 => 47,
      Interrupt::PORTA => 48,
      Interrupt::PORTB => 49,
      Interrupt::PORTC => 50,
      Interrupt::PORTD => 51,
      Interrupt::ADC0 => 52,
      Interrupt::LPCMP0 => 53,
      Interrupt::LPDAC0 => 54,
      Interrupt::CAU3_TASK_COMPLETE => 55,
      Interrupt::CAU3_SECURITY_VIOLATION => 56,
      Interrupt::TRNG => 57,
      Interrupt::LPIT1 => 58,
      Interrupt::LPTMR2 => 59,
      Interrupt::TPM3 => 60,
      Interrupt::LPI2C3 => 61,
      Interrupt::LPSPI3 => 62,
      Interrupt::LPUART3 => 63,
      Interrupt::PORTE => 64,
      Interrupt::LPCMP1 => 65,
    }
  }
}
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
  #[inline]
  pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
    match value {
      0 => Ok(Interrupt::CTI0_MCM0),
      1 => Ok(Interrupt::DMA0),
      2 => Ok(Interrupt::DMA1),
      3 => Ok(Interrupt::DMA2),
      4 => Ok(Interrupt::DMA3),
      5 => Ok(Interrupt::DMA4),
      6 => Ok(Interrupt::DMA5),
      7 => Ok(Interrupt::DMA6),
      8 => Ok(Interrupt::DMA7),
      9 => Ok(Interrupt::DMA8),
      10 => Ok(Interrupt::DMA9),
      11 => Ok(Interrupt::DMA10),
      12 => Ok(Interrupt::DMA11),
      13 => Ok(Interrupt::DMA12),
      14 => Ok(Interrupt::DMA13),
      15 => Ok(Interrupt::DMA14),
      16 => Ok(Interrupt::DMA15),
      17 => Ok(Interrupt::DMA0_ERROR),
      18 => Ok(Interrupt::CMC0),
      19 => Ok(Interrupt::EWM),
      20 => Ok(Interrupt::FTFE_COMMAND_COMPLETE),
      21 => Ok(Interrupt::FTFE_READ_COLLISION),
      22 => Ok(Interrupt::LLWU0),
      23 => Ok(Interrupt::MUA),
      24 => Ok(Interrupt::SPM),
      25 => Ok(Interrupt::WDOG0),
      26 => Ok(Interrupt::SCG),
      27 => Ok(Interrupt::LPIT0),
      28 => Ok(Interrupt::RTC),
      29 => Ok(Interrupt::LPTMR0),
      30 => Ok(Interrupt::LPTMR1),
      31 => Ok(Interrupt::TPM0),
      32 => Ok(Interrupt::TPM1),
      33 => Ok(Interrupt::TPM2),
      34 => Ok(Interrupt::EMVSIM0),
      35 => Ok(Interrupt::FLEXIO0),
      36 => Ok(Interrupt::LPI2C0),
      37 => Ok(Interrupt::LPI2C1),
      38 => Ok(Interrupt::LPI2C2),
      39 => Ok(Interrupt::I2S0),
      40 => Ok(Interrupt::USDHC0),
      41 => Ok(Interrupt::LPSPI0),
      42 => Ok(Interrupt::LPSPI1),
      43 => Ok(Interrupt::LPSPI2),
      44 => Ok(Interrupt::LPUART0),
      45 => Ok(Interrupt::LPUART1),
      46 => Ok(Interrupt::LPUART2),
      47 => Ok(Interrupt::USB0),
      48 => Ok(Interrupt::PORTA),
      49 => Ok(Interrupt::PORTB),
      50 => Ok(Interrupt::PORTC),
      51 => Ok(Interrupt::PORTD),
      52 => Ok(Interrupt::ADC0),
      53 => Ok(Interrupt::LPCMP0),
      54 => Ok(Interrupt::LPDAC0),
      55 => Ok(Interrupt::CAU3_TASK_COMPLETE),
      56 => Ok(Interrupt::CAU3_SECURITY_VIOLATION),
      57 => Ok(Interrupt::TRNG),
      58 => Ok(Interrupt::LPIT1),
      59 => Ok(Interrupt::LPTMR2),
      60 => Ok(Interrupt::TPM3),
      61 => Ok(Interrupt::LPI2C3),
      62 => Ok(Interrupt::LPSPI3),
      63 => Ok(Interrupt::LPUART3),
      64 => Ok(Interrupt::PORTE),
      65 => Ok(Interrupt::LPCMP1),
      _ => Err(TryFromInterruptError(())),
    }
  }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
