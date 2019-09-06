#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - Control Register"]
  pub cr: CR,
  _reserved1: [u8; 236usize],
  #[doc = "0xf0 - Hardware Configuration Register 0"]
  pub hwcfg0: HWCFG0,
  #[doc = "0xf4 - Hardware Configuration Register 1"]
  pub hwcfg1: HWCFG1,
  #[doc = "0xf8 - Hardware Configuration Register 2"]
  pub hwcfg2: HWCFG2,
  #[doc = "0xfc - Hardware Configuration Register 3"]
  pub hwcfg3: HWCFG3,
  #[doc = "0x100 - Master Domain Assignment Configuration Register"]
  pub mdacfg0: MDACFG0,
  #[doc = "0x101 - Master Domain Assignment Configuration Register"]
  pub mdacfg1: MDACFG1,
  #[doc = "0x102 - Master Domain Assignment Configuration Register"]
  pub mdacfg2: MDACFG2,
  #[doc = "0x103 - Master Domain Assignment Configuration Register"]
  pub mdacfg3: MDACFG3,
  #[doc = "0x104 - Master Domain Assignment Configuration Register"]
  pub mdacfg4: MDACFG4,
  _reserved10: [u8; 27usize],
  #[doc = "0x120 - Master Domain Assignment Configuration Register"]
  pub mdacfg32: MDACFG32,
  #[doc = "0x121 - Master Domain Assignment Configuration Register"]
  pub mdacfg33: MDACFG33,
  #[doc = "0x122 - Master Domain Assignment Configuration Register"]
  pub mdacfg34: MDACFG34,
  _reserved13: [u8; 29usize],
  #[doc = "0x140 - Memory Region Configuration Register"]
  pub mrcfg: [MRCFG; 2],
  _reserved14: [u8; 186usize],
  #[doc = "0x1fc - Fault Domain ID"]
  pub fdid: FDID,
  #[doc = "0x200 - Domain Error Location Register"]
  pub derrloc: [DERRLOC; 3],
  _reserved16: [u8; 500usize],
  #[doc = "0x400 - Domain Error Word0 Register"]
  pub derr_w0_0: DERR_W0_0,
  #[doc = "0x404 - Domain Error Word1 Register"]
  pub derr_w1_0: DERR_W1_0,
  _reserved18: [u8; 4usize],
  #[doc = "0x40c - Domain Error Word3 Register"]
  pub derr_w3_0: DERR_W3_0,
  #[doc = "0x410 - Domain Error Word0 Register"]
  pub derr_w0_1: DERR_W0_1,
  #[doc = "0x414 - Domain Error Word1 Register"]
  pub derr_w1_1: DERR_W1_1,
  _reserved21: [u8; 4usize],
  #[doc = "0x41c - Domain Error Word3 Register"]
  pub derr_w3_1: DERR_W3_1,
  _reserved22: [u8; 224usize],
  #[doc = "0x500 - Domain Error Word0 Register"]
  pub derr_w0_16: DERR_W0_16,
  #[doc = "0x504 - Domain Error Word1 Register"]
  pub derr_w1_16: DERR_W1_16,
  _reserved24: [u8; 4usize],
  #[doc = "0x50c - Domain Error Word3 Register"]
  pub derr_w3_16: DERR_W3_16,
  #[doc = "0x510 - Domain Error Word0 Register"]
  pub derr_w0_17: DERR_W0_17,
  #[doc = "0x514 - Domain Error Word1 Register"]
  pub derr_w1_17: DERR_W1_17,
  _reserved27: [u8; 4usize],
  #[doc = "0x51c - Domain Error Word3 Register"]
  pub derr_w3_17: DERR_W3_17,
  #[doc = "0x520 - Domain Error Word0 Register"]
  pub derr_w0_18: DERR_W0_18,
  #[doc = "0x524 - Domain Error Word1 Register"]
  pub derr_w1_18: DERR_W1_18,
  _reserved30: [u8; 4usize],
  #[doc = "0x52c - Domain Error Word3 Register"]
  pub derr_w3_18: DERR_W3_18,
  _reserved31: [u8; 464usize],
  #[doc = "0x700 - Process Identifier"]
  pub pid0: PID0,
  #[doc = "0x704 - Process Identifier"]
  pub pid1: PID1,
  _reserved33: [u8; 120usize],
  #[doc = "0x780 - Process Identifier"]
  pub pid32: PID32,
  _reserved34: [u8; 124usize],
  #[doc = "0x800 - Master Domain Assignment"]
  pub mda_w0_0_dfmt0: MDA_W0_0_DFMT0,
  #[doc = "0x804 - Master Domain Assignment"]
  pub mda_w1_0_dfmt0: MDA_W1_0_DFMT0,
  _reserved36: [u8; 24usize],
  #[doc = "0x820 - Master Domain Assignment"]
  pub mda_w0_1_dfmt0: MDA_W0_1_DFMT0,
  #[doc = "0x824 - Master Domain Assignment"]
  pub mda_w1_1_dfmt0: MDA_W1_1_DFMT0,
  _reserved38: [u8; 24usize],
  #[doc = "0x840 - Master Domain Assignment"]
  pub mda_w0_2_dfmt1: MDA_W0_2_DFMT1,
  _reserved39: [u8; 28usize],
  #[doc = "0x860 - Master Domain Assignment"]
  pub mda_w0_3_dfmt1: MDA_W0_3_DFMT1,
  _reserved40: [u8; 28usize],
  #[doc = "0x880 - Master Domain Assignment"]
  pub mda_w0_4_dfmt1: MDA_W0_4_DFMT1,
  _reserved41: [u8; 892usize],
  #[doc = "0xc00 - Master Domain Assignment"]
  pub mda_w0_32_dfmt0: MDA_W0_32_DFMT0,
  #[doc = "0xc04 - Master Domain Assignment"]
  pub mda_w1_32_dfmt0: MDA_W1_32_DFMT0,
  _reserved43: [u8; 24usize],
  #[doc = "0xc20 - Master Domain Assignment"]
  pub mda_w0_33_dfmt1: MDA_W0_33_DFMT1,
  _reserved44: [u8; 28usize],
  #[doc = "0xc40 - Master Domain Assignment"]
  pub mda_w0_34_dfmt1: MDA_W0_34_DFMT1,
  _reserved45: [u8; 964usize],
  #[doc = "0x1008 - Peripheral Domain Access Control"]
  pub pdac_w0_0_1: PDAC_W0_0_1,
  #[doc = "0x100c - Peripheral Domain Access Control"]
  pub pdac_w1_0_1: PDAC_W1_0_1,
  _reserved47: [u8; 8usize],
  #[doc = "0x1018 - Peripheral Domain Access Control"]
  pub pdac_w0_0_3: PDAC_W0_0_3,
  #[doc = "0x101c - Peripheral Domain Access Control"]
  pub pdac_w1_0_3: PDAC_W1_0_3,
  #[doc = "0x1020 - Peripheral Domain Access Control"]
  pub pdac_w0_0_4: PDAC_W0_0_4,
  #[doc = "0x1024 - Peripheral Domain Access Control"]
  pub pdac_w1_0_4: PDAC_W1_0_4,
  _reserved51: [u8; 24usize],
  #[doc = "0x1040 - Peripheral Domain Access Control"]
  pub pdac_w0_0_8: PDAC_W0_0_8,
  #[doc = "0x1044 - Peripheral Domain Access Control"]
  pub pdac_w1_0_8: PDAC_W1_0_8,
  #[doc = "0x1048 - Peripheral Domain Access Control"]
  pub pdac_w0_0_9: PDAC_W0_0_9,
  #[doc = "0x104c - Peripheral Domain Access Control"]
  pub pdac_w1_0_9: PDAC_W1_0_9,
  _reserved55: [u8; 16usize],
  #[doc = "0x1060 - Peripheral Domain Access Control"]
  pub pdac_w0_0_12: PDAC_W0_0_12,
  #[doc = "0x1064 - Peripheral Domain Access Control"]
  pub pdac_w1_0_12: PDAC_W1_0_12,
  _reserved57: [u8; 16usize],
  #[doc = "0x1078 - Peripheral Domain Access Control"]
  pub pdac_w0_0_15: PDAC_W0_0_15,
  #[doc = "0x107c - Peripheral Domain Access Control"]
  pub pdac_w1_0_15: PDAC_W1_0_15,
  _reserved59: [u8; 32usize],
  #[doc = "0x10a0 - Peripheral Domain Access Control"]
  pub pdac_w0_0_20: PDAC_W0_0_20,
  #[doc = "0x10a4 - Peripheral Domain Access Control"]
  pub pdac_w1_0_20: PDAC_W1_0_20,
  #[doc = "0x10a8 - Peripheral Domain Access Control"]
  pub pdac_w0_0_21: PDAC_W0_0_21,
  #[doc = "0x10ac - Peripheral Domain Access Control"]
  pub pdac_w1_0_21: PDAC_W1_0_21,
  #[doc = "0x10b0 - Peripheral Domain Access Control"]
  pub pdac_w0_0_22: PDAC_W0_0_22,
  #[doc = "0x10b4 - Peripheral Domain Access Control"]
  pub pdac_w1_0_22: PDAC_W1_0_22,
  #[doc = "0x10b8 - Peripheral Domain Access Control"]
  pub pdac_w0_0_23: PDAC_W0_0_23,
  #[doc = "0x10bc - Peripheral Domain Access Control"]
  pub pdac_w1_0_23: PDAC_W1_0_23,
  _reserved67: [u8; 24usize],
  #[doc = "0x10d8 - Peripheral Domain Access Control"]
  pub pdac_w0_0_27: PDAC_W0_0_27,
  #[doc = "0x10dc - Peripheral Domain Access Control"]
  pub pdac_w1_0_27: PDAC_W1_0_27,
  _reserved69: [u8; 32usize],
  #[doc = "0x1100 - Peripheral Domain Access Control"]
  pub pdac_w0_0_32: PDAC_W0_0_32,
  #[doc = "0x1104 - Peripheral Domain Access Control"]
  pub pdac_w1_0_32: PDAC_W1_0_32,
  #[doc = "0x1108 - Peripheral Domain Access Control"]
  pub pdac_w0_0_33: PDAC_W0_0_33,
  #[doc = "0x110c - Peripheral Domain Access Control"]
  pub pdac_w1_0_33: PDAC_W1_0_33,
  #[doc = "0x1110 - Peripheral Domain Access Control"]
  pub pdac_w0_0_34: PDAC_W0_0_34,
  #[doc = "0x1114 - Peripheral Domain Access Control"]
  pub pdac_w1_0_34: PDAC_W1_0_34,
  #[doc = "0x1118 - Peripheral Domain Access Control"]
  pub pdac_w0_0_35: PDAC_W0_0_35,
  #[doc = "0x111c - Peripheral Domain Access Control"]
  pub pdac_w1_0_35: PDAC_W1_0_35,
  #[doc = "0x1120 - Peripheral Domain Access Control"]
  pub pdac_w0_0_36: PDAC_W0_0_36,
  #[doc = "0x1124 - Peripheral Domain Access Control"]
  pub pdac_w1_0_36: PDAC_W1_0_36,
  #[doc = "0x1128 - Peripheral Domain Access Control"]
  pub pdac_w0_0_37: PDAC_W0_0_37,
  #[doc = "0x112c - Peripheral Domain Access Control"]
  pub pdac_w1_0_37: PDAC_W1_0_37,
  #[doc = "0x1130 - Peripheral Domain Access Control"]
  pub pdac_w0_0_38: PDAC_W0_0_38,
  #[doc = "0x1134 - Peripheral Domain Access Control"]
  pub pdac_w1_0_38: PDAC_W1_0_38,
  #[doc = "0x1138 - Peripheral Domain Access Control"]
  pub pdac_w0_0_39: PDAC_W0_0_39,
  #[doc = "0x113c - Peripheral Domain Access Control"]
  pub pdac_w1_0_39: PDAC_W1_0_39,
  #[doc = "0x1140 - Peripheral Domain Access Control"]
  pub pdac_w0_0_40: PDAC_W0_0_40,
  #[doc = "0x1144 - Peripheral Domain Access Control"]
  pub pdac_w1_0_40: PDAC_W1_0_40,
  #[doc = "0x1148 - Peripheral Domain Access Control"]
  pub pdac_w0_0_41: PDAC_W0_0_41,
  #[doc = "0x114c - Peripheral Domain Access Control"]
  pub pdac_w1_0_41: PDAC_W1_0_41,
  #[doc = "0x1150 - Peripheral Domain Access Control"]
  pub pdac_w0_0_42: PDAC_W0_0_42,
  #[doc = "0x1154 - Peripheral Domain Access Control"]
  pub pdac_w1_0_42: PDAC_W1_0_42,
  #[doc = "0x1158 - Peripheral Domain Access Control"]
  pub pdac_w0_0_43: PDAC_W0_0_43,
  #[doc = "0x115c - Peripheral Domain Access Control"]
  pub pdac_w1_0_43: PDAC_W1_0_43,
  #[doc = "0x1160 - Peripheral Domain Access Control"]
  pub pdac_w0_0_44: PDAC_W0_0_44,
  #[doc = "0x1164 - Peripheral Domain Access Control"]
  pub pdac_w1_0_44: PDAC_W1_0_44,
  #[doc = "0x1168 - Peripheral Domain Access Control"]
  pub pdac_w0_0_45: PDAC_W0_0_45,
  #[doc = "0x116c - Peripheral Domain Access Control"]
  pub pdac_w1_0_45: PDAC_W1_0_45,
  #[doc = "0x1170 - Peripheral Domain Access Control"]
  pub pdac_w0_0_46: PDAC_W0_0_46,
  #[doc = "0x1174 - Peripheral Domain Access Control"]
  pub pdac_w1_0_46: PDAC_W1_0_46,
  #[doc = "0x1178 - Peripheral Domain Access Control"]
  pub pdac_w0_0_47: PDAC_W0_0_47,
  #[doc = "0x117c - Peripheral Domain Access Control"]
  pub pdac_w1_0_47: PDAC_W1_0_47,
  #[doc = "0x1180 - Peripheral Domain Access Control"]
  pub pdac_w0_0_48: PDAC_W0_0_48,
  #[doc = "0x1184 - Peripheral Domain Access Control"]
  pub pdac_w1_0_48: PDAC_W1_0_48,
  #[doc = "0x1188 - Peripheral Domain Access Control"]
  pub pdac_w0_0_49: PDAC_W0_0_49,
  #[doc = "0x118c - Peripheral Domain Access Control"]
  pub pdac_w1_0_49: PDAC_W1_0_49,
  #[doc = "0x1190 - Peripheral Domain Access Control"]
  pub pdac_w0_0_50: PDAC_W0_0_50,
  #[doc = "0x1194 - Peripheral Domain Access Control"]
  pub pdac_w1_0_50: PDAC_W1_0_50,
  #[doc = "0x1198 - Peripheral Domain Access Control"]
  pub pdac_w0_0_51: PDAC_W0_0_51,
  #[doc = "0x119c - Peripheral Domain Access Control"]
  pub pdac_w1_0_51: PDAC_W1_0_51,
  #[doc = "0x11a0 - Peripheral Domain Access Control"]
  pub pdac_w0_0_52: PDAC_W0_0_52,
  #[doc = "0x11a4 - Peripheral Domain Access Control"]
  pub pdac_w1_0_52: PDAC_W1_0_52,
  #[doc = "0x11a8 - Peripheral Domain Access Control"]
  pub pdac_w0_0_53: PDAC_W0_0_53,
  #[doc = "0x11ac - Peripheral Domain Access Control"]
  pub pdac_w1_0_53: PDAC_W1_0_53,
  #[doc = "0x11b0 - Peripheral Domain Access Control"]
  pub pdac_w0_0_54: PDAC_W0_0_54,
  #[doc = "0x11b4 - Peripheral Domain Access Control"]
  pub pdac_w1_0_54: PDAC_W1_0_54,
  #[doc = "0x11b8 - Peripheral Domain Access Control"]
  pub pdac_w0_0_55: PDAC_W0_0_55,
  #[doc = "0x11bc - Peripheral Domain Access Control"]
  pub pdac_w1_0_55: PDAC_W1_0_55,
  #[doc = "0x11c0 - Peripheral Domain Access Control"]
  pub pdac_w0_0_56: PDAC_W0_0_56,
  #[doc = "0x11c4 - Peripheral Domain Access Control"]
  pub pdac_w1_0_56: PDAC_W1_0_56,
  #[doc = "0x11c8 - Peripheral Domain Access Control"]
  pub pdac_w0_0_57: PDAC_W0_0_57,
  #[doc = "0x11cc - Peripheral Domain Access Control"]
  pub pdac_w1_0_57: PDAC_W1_0_57,
  #[doc = "0x11d0 - Peripheral Domain Access Control"]
  pub pdac_w0_0_58: PDAC_W0_0_58,
  #[doc = "0x11d4 - Peripheral Domain Access Control"]
  pub pdac_w1_0_58: PDAC_W1_0_58,
  #[doc = "0x11d8 - Peripheral Domain Access Control"]
  pub pdac_w0_0_59: PDAC_W0_0_59,
  #[doc = "0x11dc - Peripheral Domain Access Control"]
  pub pdac_w1_0_59: PDAC_W1_0_59,
  #[doc = "0x11e0 - Peripheral Domain Access Control"]
  pub pdac_w0_0_60: PDAC_W0_0_60,
  #[doc = "0x11e4 - Peripheral Domain Access Control"]
  pub pdac_w1_0_60: PDAC_W1_0_60,
  #[doc = "0x11e8 - Peripheral Domain Access Control"]
  pub pdac_w0_0_61: PDAC_W0_0_61,
  #[doc = "0x11ec - Peripheral Domain Access Control"]
  pub pdac_w1_0_61: PDAC_W1_0_61,
  #[doc = "0x11f0 - Peripheral Domain Access Control"]
  pub pdac_w0_0_62: PDAC_W0_0_62,
  #[doc = "0x11f4 - Peripheral Domain Access Control"]
  pub pdac_w1_0_62: PDAC_W1_0_62,
  #[doc = "0x11f8 - Peripheral Domain Access Control"]
  pub pdac_w0_0_63: PDAC_W0_0_63,
  #[doc = "0x11fc - Peripheral Domain Access Control"]
  pub pdac_w1_0_63: PDAC_W1_0_63,
  #[doc = "0x1200 - Peripheral Domain Access Control"]
  pub pdac_w0_0_64: PDAC_W0_0_64,
  #[doc = "0x1204 - Peripheral Domain Access Control"]
  pub pdac_w1_0_64: PDAC_W1_0_64,
  #[doc = "0x1208 - Peripheral Domain Access Control"]
  pub pdac_w0_0_65: PDAC_W0_0_65,
  #[doc = "0x120c - Peripheral Domain Access Control"]
  pub pdac_w1_0_65: PDAC_W1_0_65,
  #[doc = "0x1210 - Peripheral Domain Access Control"]
  pub pdac_w0_0_66: PDAC_W0_0_66,
  #[doc = "0x1214 - Peripheral Domain Access Control"]
  pub pdac_w1_0_66: PDAC_W1_0_66,
  #[doc = "0x1218 - Peripheral Domain Access Control"]
  pub pdac_w0_0_67: PDAC_W0_0_67,
  #[doc = "0x121c - Peripheral Domain Access Control"]
  pub pdac_w1_0_67: PDAC_W1_0_67,
  #[doc = "0x1220 - Peripheral Domain Access Control"]
  pub pdac_w0_0_68: PDAC_W0_0_68,
  #[doc = "0x1224 - Peripheral Domain Access Control"]
  pub pdac_w1_0_68: PDAC_W1_0_68,
  #[doc = "0x1228 - Peripheral Domain Access Control"]
  pub pdac_w0_0_69: PDAC_W0_0_69,
  #[doc = "0x122c - Peripheral Domain Access Control"]
  pub pdac_w1_0_69: PDAC_W1_0_69,
  #[doc = "0x1230 - Peripheral Domain Access Control"]
  pub pdac_w0_0_70: PDAC_W0_0_70,
  #[doc = "0x1234 - Peripheral Domain Access Control"]
  pub pdac_w1_0_70: PDAC_W1_0_70,
  #[doc = "0x1238 - Peripheral Domain Access Control"]
  pub pdac_w0_0_71: PDAC_W0_0_71,
  #[doc = "0x123c - Peripheral Domain Access Control"]
  pub pdac_w1_0_71: PDAC_W1_0_71,
  #[doc = "0x1240 - Peripheral Domain Access Control"]
  pub pdac_w0_0_72: PDAC_W0_0_72,
  #[doc = "0x1244 - Peripheral Domain Access Control"]
  pub pdac_w1_0_72: PDAC_W1_0_72,
  #[doc = "0x1248 - Peripheral Domain Access Control"]
  pub pdac_w0_0_73: PDAC_W0_0_73,
  #[doc = "0x124c - Peripheral Domain Access Control"]
  pub pdac_w1_0_73: PDAC_W1_0_73,
  #[doc = "0x1250 - Peripheral Domain Access Control"]
  pub pdac_w0_0_74: PDAC_W0_0_74,
  #[doc = "0x1254 - Peripheral Domain Access Control"]
  pub pdac_w1_0_74: PDAC_W1_0_74,
  #[doc = "0x1258 - Peripheral Domain Access Control"]
  pub pdac_w0_0_75: PDAC_W0_0_75,
  #[doc = "0x125c - Peripheral Domain Access Control"]
  pub pdac_w1_0_75: PDAC_W1_0_75,
  #[doc = "0x1260 - Peripheral Domain Access Control"]
  pub pdac_w0_0_76: PDAC_W0_0_76,
  #[doc = "0x1264 - Peripheral Domain Access Control"]
  pub pdac_w1_0_76: PDAC_W1_0_76,
  #[doc = "0x1268 - Peripheral Domain Access Control"]
  pub pdac_w0_0_77: PDAC_W0_0_77,
  #[doc = "0x126c - Peripheral Domain Access Control"]
  pub pdac_w1_0_77: PDAC_W1_0_77,
  #[doc = "0x1270 - Peripheral Domain Access Control"]
  pub pdac_w0_0_78: PDAC_W0_0_78,
  #[doc = "0x1274 - Peripheral Domain Access Control"]
  pub pdac_w1_0_78: PDAC_W1_0_78,
  _reserved163: [u8; 368usize],
  #[doc = "0x13e8 - Peripheral Domain Access Control"]
  pub pdac_w0_0_125: PDAC_W0_0_125,
  #[doc = "0x13ec - Peripheral Domain Access Control"]
  pub pdac_w1_0_125: PDAC_W1_0_125,
  #[doc = "0x13f0 - Peripheral Domain Access Control"]
  pub pdac_w0_0_126: PDAC_W0_0_126,
  #[doc = "0x13f4 - Peripheral Domain Access Control"]
  pub pdac_w1_0_126: PDAC_W1_0_126,
  #[doc = "0x13f8 - Peripheral Domain Access Control"]
  pub pdac_w0_0_127: PDAC_W0_0_127,
  #[doc = "0x13fc - Peripheral Domain Access Control"]
  pub pdac_w1_0_127: PDAC_W1_0_127,
  _reserved169: [u8; 64usize],
  #[doc = "0x1440 - Peripheral Domain Access Control"]
  pub pdac_w0_1_8: PDAC_W0_1_8,
  #[doc = "0x1444 - Peripheral Domain Access Control"]
  pub pdac_w1_1_8: PDAC_W1_1_8,
  #[doc = "0x1448 - Peripheral Domain Access Control"]
  pub pdac_w0_1_9: PDAC_W0_1_9,
  #[doc = "0x144c - Peripheral Domain Access Control"]
  pub pdac_w1_1_9: PDAC_W1_1_9,
  _reserved173: [u8; 40usize],
  #[doc = "0x1478 - Peripheral Domain Access Control"]
  pub pdac_w0_1_15: PDAC_W0_1_15,
  #[doc = "0x147c - Peripheral Domain Access Control"]
  pub pdac_w1_1_15: PDAC_W1_1_15,
  _reserved175: [u8; 88usize],
  #[doc = "0x14d8 - Peripheral Domain Access Control"]
  pub pdac_w0_1_27: PDAC_W0_1_27,
  #[doc = "0x14dc - Peripheral Domain Access Control"]
  pub pdac_w1_1_27: PDAC_W1_1_27,
  _reserved177: [u8; 32usize],
  #[doc = "0x1500 - Peripheral Domain Access Control"]
  pub pdac_w0_1_32: PDAC_W0_1_32,
  #[doc = "0x1504 - Peripheral Domain Access Control"]
  pub pdac_w1_1_32: PDAC_W1_1_32,
  #[doc = "0x1508 - Peripheral Domain Access Control"]
  pub pdac_w0_1_33: PDAC_W0_1_33,
  #[doc = "0x150c - Peripheral Domain Access Control"]
  pub pdac_w1_1_33: PDAC_W1_1_33,
  #[doc = "0x1510 - Peripheral Domain Access Control"]
  pub pdac_w0_1_34: PDAC_W0_1_34,
  #[doc = "0x1514 - Peripheral Domain Access Control"]
  pub pdac_w1_1_34: PDAC_W1_1_34,
  #[doc = "0x1518 - Peripheral Domain Access Control"]
  pub pdac_w0_1_35: PDAC_W0_1_35,
  #[doc = "0x151c - Peripheral Domain Access Control"]
  pub pdac_w1_1_35: PDAC_W1_1_35,
  #[doc = "0x1520 - Peripheral Domain Access Control"]
  pub pdac_w0_1_36: PDAC_W0_1_36,
  #[doc = "0x1524 - Peripheral Domain Access Control"]
  pub pdac_w1_1_36: PDAC_W1_1_36,
  #[doc = "0x1528 - Peripheral Domain Access Control"]
  pub pdac_w0_1_37: PDAC_W0_1_37,
  #[doc = "0x152c - Peripheral Domain Access Control"]
  pub pdac_w1_1_37: PDAC_W1_1_37,
  #[doc = "0x1530 - Peripheral Domain Access Control"]
  pub pdac_w0_1_38: PDAC_W0_1_38,
  #[doc = "0x1534 - Peripheral Domain Access Control"]
  pub pdac_w1_1_38: PDAC_W1_1_38,
  #[doc = "0x1538 - Peripheral Domain Access Control"]
  pub pdac_w0_1_39: PDAC_W0_1_39,
  #[doc = "0x153c - Peripheral Domain Access Control"]
  pub pdac_w1_1_39: PDAC_W1_1_39,
  #[doc = "0x1540 - Peripheral Domain Access Control"]
  pub pdac_w0_1_40: PDAC_W0_1_40,
  #[doc = "0x1544 - Peripheral Domain Access Control"]
  pub pdac_w1_1_40: PDAC_W1_1_40,
  #[doc = "0x1548 - Peripheral Domain Access Control"]
  pub pdac_w0_1_41: PDAC_W0_1_41,
  #[doc = "0x154c - Peripheral Domain Access Control"]
  pub pdac_w1_1_41: PDAC_W1_1_41,
  #[doc = "0x1550 - Peripheral Domain Access Control"]
  pub pdac_w0_1_42: PDAC_W0_1_42,
  #[doc = "0x1554 - Peripheral Domain Access Control"]
  pub pdac_w1_1_42: PDAC_W1_1_42,
  #[doc = "0x1558 - Peripheral Domain Access Control"]
  pub pdac_w0_1_43: PDAC_W0_1_43,
  #[doc = "0x155c - Peripheral Domain Access Control"]
  pub pdac_w1_1_43: PDAC_W1_1_43,
  #[doc = "0x1560 - Peripheral Domain Access Control"]
  pub pdac_w0_1_44: PDAC_W0_1_44,
  #[doc = "0x1564 - Peripheral Domain Access Control"]
  pub pdac_w1_1_44: PDAC_W1_1_44,
  #[doc = "0x1568 - Peripheral Domain Access Control"]
  pub pdac_w0_1_45: PDAC_W0_1_45,
  #[doc = "0x156c - Peripheral Domain Access Control"]
  pub pdac_w1_1_45: PDAC_W1_1_45,
  #[doc = "0x1570 - Peripheral Domain Access Control"]
  pub pdac_w0_1_46: PDAC_W0_1_46,
  #[doc = "0x1574 - Peripheral Domain Access Control"]
  pub pdac_w1_1_46: PDAC_W1_1_46,
  #[doc = "0x1578 - Peripheral Domain Access Control"]
  pub pdac_w0_1_47: PDAC_W0_1_47,
  #[doc = "0x157c - Peripheral Domain Access Control"]
  pub pdac_w1_1_47: PDAC_W1_1_47,
  #[doc = "0x1580 - Peripheral Domain Access Control"]
  pub pdac_w0_1_48: PDAC_W0_1_48,
  #[doc = "0x1584 - Peripheral Domain Access Control"]
  pub pdac_w1_1_48: PDAC_W1_1_48,
  #[doc = "0x1588 - Peripheral Domain Access Control"]
  pub pdac_w0_1_49: PDAC_W0_1_49,
  #[doc = "0x158c - Peripheral Domain Access Control"]
  pub pdac_w1_1_49: PDAC_W1_1_49,
  #[doc = "0x1590 - Peripheral Domain Access Control"]
  pub pdac_w0_1_50: PDAC_W0_1_50,
  #[doc = "0x1594 - Peripheral Domain Access Control"]
  pub pdac_w1_1_50: PDAC_W1_1_50,
  #[doc = "0x1598 - Peripheral Domain Access Control"]
  pub pdac_w0_1_51: PDAC_W0_1_51,
  #[doc = "0x159c - Peripheral Domain Access Control"]
  pub pdac_w1_1_51: PDAC_W1_1_51,
  #[doc = "0x15a0 - Peripheral Domain Access Control"]
  pub pdac_w0_1_52: PDAC_W0_1_52,
  #[doc = "0x15a4 - Peripheral Domain Access Control"]
  pub pdac_w1_1_52: PDAC_W1_1_52,
  #[doc = "0x15a8 - Peripheral Domain Access Control"]
  pub pdac_w0_1_53: PDAC_W0_1_53,
  #[doc = "0x15ac - Peripheral Domain Access Control"]
  pub pdac_w1_1_53: PDAC_W1_1_53,
  #[doc = "0x15b0 - Peripheral Domain Access Control"]
  pub pdac_w0_1_54: PDAC_W0_1_54,
  #[doc = "0x15b4 - Peripheral Domain Access Control"]
  pub pdac_w1_1_54: PDAC_W1_1_54,
  #[doc = "0x15b8 - Peripheral Domain Access Control"]
  pub pdac_w0_1_55: PDAC_W0_1_55,
  #[doc = "0x15bc - Peripheral Domain Access Control"]
  pub pdac_w1_1_55: PDAC_W1_1_55,
  #[doc = "0x15c0 - Peripheral Domain Access Control"]
  pub pdac_w0_1_56: PDAC_W0_1_56,
  #[doc = "0x15c4 - Peripheral Domain Access Control"]
  pub pdac_w1_1_56: PDAC_W1_1_56,
  _reserved227: [u8; 696usize],
  #[doc = "0x1880 - Peripheral Domain Access Control"]
  pub pdac_w0_2_16: PDAC_W0_2_16,
  #[doc = "0x1884 - Peripheral Domain Access Control"]
  pub pdac_w1_2_16: PDAC_W1_2_16,
  _reserved229: [u8; 120usize],
  #[doc = "0x1900 - Peripheral Domain Access Control"]
  pub pdac_w0_2_32: PDAC_W0_2_32,
  #[doc = "0x1904 - Peripheral Domain Access Control"]
  pub pdac_w1_2_32: PDAC_W1_2_32,
  _reserved231: [u8; 1784usize],
  #[doc = "0x2000 - Memory Region Descriptor"]
  pub mrgd_w0_0_0: MRGD_W0_0_0,
  #[doc = "0x2004 - Memory Region Descriptor"]
  pub mrgd_w1_0_0: MRGD_W1_0_0,
  #[doc = "0x2008 - Memory Region Descriptor"]
  pub mrgd_w2_0_0: MRGD_W2_0_0,
  #[doc = "0x200c - Memory Region Descriptor"]
  pub mrgd_w3_0_0: MRGD_W3_0_0,
  #[doc = "0x2010 - Memory Region Descriptor"]
  pub mrgd_w4_0_0: MRGD_W4_0_0,
  _reserved236: [u8; 12usize],
  #[doc = "0x2020 - Memory Region Descriptor"]
  pub mrgd_w0_0_1: MRGD_W0_0_1,
  #[doc = "0x2024 - Memory Region Descriptor"]
  pub mrgd_w1_0_1: MRGD_W1_0_1,
  #[doc = "0x2028 - Memory Region Descriptor"]
  pub mrgd_w2_0_1: MRGD_W2_0_1,
  #[doc = "0x202c - Memory Region Descriptor"]
  pub mrgd_w3_0_1: MRGD_W3_0_1,
  #[doc = "0x2030 - Memory Region Descriptor"]
  pub mrgd_w4_0_1: MRGD_W4_0_1,
  _reserved241: [u8; 12usize],
  #[doc = "0x2040 - Memory Region Descriptor"]
  pub mrgd_w0_0_2: MRGD_W0_0_2,
  #[doc = "0x2044 - Memory Region Descriptor"]
  pub mrgd_w1_0_2: MRGD_W1_0_2,
  #[doc = "0x2048 - Memory Region Descriptor"]
  pub mrgd_w2_0_2: MRGD_W2_0_2,
  #[doc = "0x204c - Memory Region Descriptor"]
  pub mrgd_w3_0_2: MRGD_W3_0_2,
  #[doc = "0x2050 - Memory Region Descriptor"]
  pub mrgd_w4_0_2: MRGD_W4_0_2,
  _reserved246: [u8; 12usize],
  #[doc = "0x2060 - Memory Region Descriptor"]
  pub mrgd_w0_0_3: MRGD_W0_0_3,
  #[doc = "0x2064 - Memory Region Descriptor"]
  pub mrgd_w1_0_3: MRGD_W1_0_3,
  #[doc = "0x2068 - Memory Region Descriptor"]
  pub mrgd_w2_0_3: MRGD_W2_0_3,
  #[doc = "0x206c - Memory Region Descriptor"]
  pub mrgd_w3_0_3: MRGD_W3_0_3,
  #[doc = "0x2070 - Memory Region Descriptor"]
  pub mrgd_w4_0_3: MRGD_W4_0_3,
  _reserved251: [u8; 12usize],
  #[doc = "0x2080 - Memory Region Descriptor"]
  pub mrgd_w0_0_4: MRGD_W0_0_4,
  #[doc = "0x2084 - Memory Region Descriptor"]
  pub mrgd_w1_0_4: MRGD_W1_0_4,
  #[doc = "0x2088 - Memory Region Descriptor"]
  pub mrgd_w2_0_4: MRGD_W2_0_4,
  #[doc = "0x208c - Memory Region Descriptor"]
  pub mrgd_w3_0_4: MRGD_W3_0_4,
  #[doc = "0x2090 - Memory Region Descriptor"]
  pub mrgd_w4_0_4: MRGD_W4_0_4,
  _reserved256: [u8; 12usize],
  #[doc = "0x20a0 - Memory Region Descriptor"]
  pub mrgd_w0_0_5: MRGD_W0_0_5,
  #[doc = "0x20a4 - Memory Region Descriptor"]
  pub mrgd_w1_0_5: MRGD_W1_0_5,
  #[doc = "0x20a8 - Memory Region Descriptor"]
  pub mrgd_w2_0_5: MRGD_W2_0_5,
  #[doc = "0x20ac - Memory Region Descriptor"]
  pub mrgd_w3_0_5: MRGD_W3_0_5,
  #[doc = "0x20b0 - Memory Region Descriptor"]
  pub mrgd_w4_0_5: MRGD_W4_0_5,
  _reserved261: [u8; 12usize],
  #[doc = "0x20c0 - Memory Region Descriptor"]
  pub mrgd_w0_0_6: MRGD_W0_0_6,
  #[doc = "0x20c4 - Memory Region Descriptor"]
  pub mrgd_w1_0_6: MRGD_W1_0_6,
  #[doc = "0x20c8 - Memory Region Descriptor"]
  pub mrgd_w2_0_6: MRGD_W2_0_6,
  #[doc = "0x20cc - Memory Region Descriptor"]
  pub mrgd_w3_0_6: MRGD_W3_0_6,
  #[doc = "0x20d0 - Memory Region Descriptor"]
  pub mrgd_w4_0_6: MRGD_W4_0_6,
  _reserved266: [u8; 12usize],
  #[doc = "0x20e0 - Memory Region Descriptor"]
  pub mrgd_w0_0_7: MRGD_W0_0_7,
  #[doc = "0x20e4 - Memory Region Descriptor"]
  pub mrgd_w1_0_7: MRGD_W1_0_7,
  #[doc = "0x20e8 - Memory Region Descriptor"]
  pub mrgd_w2_0_7: MRGD_W2_0_7,
  #[doc = "0x20ec - Memory Region Descriptor"]
  pub mrgd_w3_0_7: MRGD_W3_0_7,
  #[doc = "0x20f0 - Memory Region Descriptor"]
  pub mrgd_w4_0_7: MRGD_W4_0_7,
  _reserved271: [u8; 268usize],
  #[doc = "0x2200 - Memory Region Descriptor"]
  pub mrgd_w0_1_0: MRGD_W0_1_0,
  #[doc = "0x2204 - Memory Region Descriptor"]
  pub mrgd_w1_1_0: MRGD_W1_1_0,
  #[doc = "0x2208 - Memory Region Descriptor"]
  pub mrgd_w2_1_0: MRGD_W2_1_0,
  #[doc = "0x220c - Memory Region Descriptor"]
  pub mrgd_w3_1_0: MRGD_W3_1_0,
  #[doc = "0x2210 - Memory Region Descriptor"]
  pub mrgd_w4_1_0: MRGD_W4_1_0,
  _reserved276: [u8; 12usize],
  #[doc = "0x2220 - Memory Region Descriptor"]
  pub mrgd_w0_1_1: MRGD_W0_1_1,
  #[doc = "0x2224 - Memory Region Descriptor"]
  pub mrgd_w1_1_1: MRGD_W1_1_1,
  #[doc = "0x2228 - Memory Region Descriptor"]
  pub mrgd_w2_1_1: MRGD_W2_1_1,
  #[doc = "0x222c - Memory Region Descriptor"]
  pub mrgd_w3_1_1: MRGD_W3_1_1,
  #[doc = "0x2230 - Memory Region Descriptor"]
  pub mrgd_w4_1_1: MRGD_W4_1_1,
  _reserved281: [u8; 12usize],
  #[doc = "0x2240 - Memory Region Descriptor"]
  pub mrgd_w0_1_2: MRGD_W0_1_2,
  #[doc = "0x2244 - Memory Region Descriptor"]
  pub mrgd_w1_1_2: MRGD_W1_1_2,
  #[doc = "0x2248 - Memory Region Descriptor"]
  pub mrgd_w2_1_2: MRGD_W2_1_2,
  #[doc = "0x224c - Memory Region Descriptor"]
  pub mrgd_w3_1_2: MRGD_W3_1_2,
  #[doc = "0x2250 - Memory Region Descriptor"]
  pub mrgd_w4_1_2: MRGD_W4_1_2,
  _reserved286: [u8; 12usize],
  #[doc = "0x2260 - Memory Region Descriptor"]
  pub mrgd_w0_1_3: MRGD_W0_1_3,
  #[doc = "0x2264 - Memory Region Descriptor"]
  pub mrgd_w1_1_3: MRGD_W1_1_3,
  #[doc = "0x2268 - Memory Region Descriptor"]
  pub mrgd_w2_1_3: MRGD_W2_1_3,
  #[doc = "0x226c - Memory Region Descriptor"]
  pub mrgd_w3_1_3: MRGD_W3_1_3,
  #[doc = "0x2270 - Memory Region Descriptor"]
  pub mrgd_w4_1_3: MRGD_W4_1_3,
  _reserved291: [u8; 12usize],
  #[doc = "0x2280 - Memory Region Descriptor"]
  pub mrgd_w0_1_4: MRGD_W0_1_4,
  #[doc = "0x2284 - Memory Region Descriptor"]
  pub mrgd_w1_1_4: MRGD_W1_1_4,
  #[doc = "0x2288 - Memory Region Descriptor"]
  pub mrgd_w2_1_4: MRGD_W2_1_4,
  #[doc = "0x228c - Memory Region Descriptor"]
  pub mrgd_w3_1_4: MRGD_W3_1_4,
  #[doc = "0x2290 - Memory Region Descriptor"]
  pub mrgd_w4_1_4: MRGD_W4_1_4,
  _reserved296: [u8; 12usize],
  #[doc = "0x22a0 - Memory Region Descriptor"]
  pub mrgd_w0_1_5: MRGD_W0_1_5,
  #[doc = "0x22a4 - Memory Region Descriptor"]
  pub mrgd_w1_1_5: MRGD_W1_1_5,
  #[doc = "0x22a8 - Memory Region Descriptor"]
  pub mrgd_w2_1_5: MRGD_W2_1_5,
  #[doc = "0x22ac - Memory Region Descriptor"]
  pub mrgd_w3_1_5: MRGD_W3_1_5,
  #[doc = "0x22b0 - Memory Region Descriptor"]
  pub mrgd_w4_1_5: MRGD_W4_1_5,
  _reserved301: [u8; 12usize],
  #[doc = "0x22c0 - Memory Region Descriptor"]
  pub mrgd_w0_1_6: MRGD_W0_1_6,
  #[doc = "0x22c4 - Memory Region Descriptor"]
  pub mrgd_w1_1_6: MRGD_W1_1_6,
  #[doc = "0x22c8 - Memory Region Descriptor"]
  pub mrgd_w2_1_6: MRGD_W2_1_6,
  #[doc = "0x22cc - Memory Region Descriptor"]
  pub mrgd_w3_1_6: MRGD_W3_1_6,
  #[doc = "0x22d0 - Memory Region Descriptor"]
  pub mrgd_w4_1_6: MRGD_W4_1_6,
  _reserved306: [u8; 12usize],
  #[doc = "0x22e0 - Memory Region Descriptor"]
  pub mrgd_w0_1_7: MRGD_W0_1_7,
  #[doc = "0x22e4 - Memory Region Descriptor"]
  pub mrgd_w1_1_7: MRGD_W1_1_7,
  #[doc = "0x22e8 - Memory Region Descriptor"]
  pub mrgd_w2_1_7: MRGD_W2_1_7,
  #[doc = "0x22ec - Memory Region Descriptor"]
  pub mrgd_w3_1_7: MRGD_W3_1_7,
  #[doc = "0x22f0 - Memory Region Descriptor"]
  pub mrgd_w4_1_7: MRGD_W4_1_7,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Hardware Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hwcfg0](hwcfg0) module"]
pub type HWCFG0 = crate::Reg<u32, _HWCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFG0;
#[doc = "`read()` method returns [hwcfg0::R](hwcfg0::R) reader structure"]
impl crate::Readable for HWCFG0 {}
#[doc = "Hardware Configuration Register 0"]
pub mod hwcfg0;
#[doc = "Hardware Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hwcfg1](hwcfg1) module"]
pub type HWCFG1 = crate::Reg<u32, _HWCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFG1;
#[doc = "`read()` method returns [hwcfg1::R](hwcfg1::R) reader structure"]
impl crate::Readable for HWCFG1 {}
#[doc = "Hardware Configuration Register 1"]
pub mod hwcfg1;
#[doc = "Hardware Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hwcfg2](hwcfg2) module"]
pub type HWCFG2 = crate::Reg<u32, _HWCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFG2;
#[doc = "`read()` method returns [hwcfg2::R](hwcfg2::R) reader structure"]
impl crate::Readable for HWCFG2 {}
#[doc = "Hardware Configuration Register 2"]
pub mod hwcfg2;
#[doc = "Hardware Configuration Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hwcfg3](hwcfg3) module"]
pub type HWCFG3 = crate::Reg<u32, _HWCFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFG3;
#[doc = "`read()` method returns [hwcfg3::R](hwcfg3::R) reader structure"]
impl crate::Readable for HWCFG3 {}
#[doc = "Hardware Configuration Register 3"]
pub mod hwcfg3;
#[doc = "Master Domain Assignment Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdacfg0](mdacfg0) module"]
pub type MDACFG0 = crate::Reg<u8, _MDACFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDACFG0;
#[doc = "`read()` method returns [mdacfg0::R](mdacfg0::R) reader structure"]
impl crate::Readable for MDACFG0 {}
#[doc = "Master Domain Assignment Configuration Register"]
pub mod mdacfg0;
#[doc = "Master Domain Assignment Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdacfg1](mdacfg1) module"]
pub type MDACFG1 = crate::Reg<u8, _MDACFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDACFG1;
#[doc = "`read()` method returns [mdacfg1::R](mdacfg1::R) reader structure"]
impl crate::Readable for MDACFG1 {}
#[doc = "Master Domain Assignment Configuration Register"]
pub mod mdacfg1;
#[doc = "Master Domain Assignment Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdacfg2](mdacfg2) module"]
pub type MDACFG2 = crate::Reg<u8, _MDACFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDACFG2;
#[doc = "`read()` method returns [mdacfg2::R](mdacfg2::R) reader structure"]
impl crate::Readable for MDACFG2 {}
#[doc = "Master Domain Assignment Configuration Register"]
pub mod mdacfg2;
#[doc = "Master Domain Assignment Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdacfg3](mdacfg3) module"]
pub type MDACFG3 = crate::Reg<u8, _MDACFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDACFG3;
#[doc = "`read()` method returns [mdacfg3::R](mdacfg3::R) reader structure"]
impl crate::Readable for MDACFG3 {}
#[doc = "Master Domain Assignment Configuration Register"]
pub mod mdacfg3;
#[doc = "Master Domain Assignment Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdacfg4](mdacfg4) module"]
pub type MDACFG4 = crate::Reg<u8, _MDACFG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDACFG4;
#[doc = "`read()` method returns [mdacfg4::R](mdacfg4::R) reader structure"]
impl crate::Readable for MDACFG4 {}
#[doc = "Master Domain Assignment Configuration Register"]
pub mod mdacfg4;
#[doc = "Master Domain Assignment Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdacfg32](mdacfg32) module"]
pub type MDACFG32 = crate::Reg<u8, _MDACFG32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDACFG32;
#[doc = "`read()` method returns [mdacfg32::R](mdacfg32::R) reader structure"]
impl crate::Readable for MDACFG32 {}
#[doc = "Master Domain Assignment Configuration Register"]
pub mod mdacfg32;
#[doc = "Master Domain Assignment Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdacfg33](mdacfg33) module"]
pub type MDACFG33 = crate::Reg<u8, _MDACFG33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDACFG33;
#[doc = "`read()` method returns [mdacfg33::R](mdacfg33::R) reader structure"]
impl crate::Readable for MDACFG33 {}
#[doc = "Master Domain Assignment Configuration Register"]
pub mod mdacfg33;
#[doc = "Master Domain Assignment Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mdacfg34](mdacfg34) module"]
pub type MDACFG34 = crate::Reg<u8, _MDACFG34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDACFG34;
#[doc = "`read()` method returns [mdacfg34::R](mdacfg34::R) reader structure"]
impl crate::Readable for MDACFG34 {}
#[doc = "Master Domain Assignment Configuration Register"]
pub mod mdacfg34;
#[doc = "Memory Region Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrcfg](mrcfg) module"]
pub type MRCFG = crate::Reg<u8, _MRCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRCFG;
#[doc = "`read()` method returns [mrcfg::R](mrcfg::R) reader structure"]
impl crate::Readable for MRCFG {}
#[doc = "Memory Region Configuration Register"]
pub mod mrcfg;
#[doc = "Fault Domain ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fdid](fdid) module"]
pub type FDID = crate::Reg<u32, _FDID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDID;
#[doc = "`read()` method returns [fdid::R](fdid::R) reader structure"]
impl crate::Readable for FDID {}
#[doc = "`write(|w| ..)` method takes [fdid::W](fdid::W) writer structure"]
impl crate::Writable for FDID {}
#[doc = "Fault Domain ID"]
pub mod fdid;
#[doc = "Domain Error Location Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [derrloc](derrloc) module"]
pub type DERRLOC = crate::Reg<u32, _DERRLOC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DERRLOC;
#[doc = "`read()` method returns [derrloc::R](derrloc::R) reader structure"]
impl crate::Readable for DERRLOC {}
#[doc = "Domain Error Location Register"]
pub mod derrloc;
#[doc = "Domain Error Word0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [derr_w0_0](derr_w0_0) module"]
pub type DERR_W0_0 = crate::Reg<u32, _DERR_W0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DERR_W0_0;
#[doc = "`read()` method returns [derr_w0_0::R](derr_w0_0::R) reader structure"]
impl crate::Readable for DERR_W0_0 {}
#[doc = "Domain Error Word0 Register"]
pub mod derr_w0_0;
#[doc = "Domain Error Word1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [derr_w1_0](derr_w1_0) module"]
pub type DERR_W1_0 = crate::Reg<u32, _DERR_W1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DERR_W1_0;
#[doc = "`read()` method returns [derr_w1_0::R](derr_w1_0::R) reader structure"]
impl crate::Readable for DERR_W1_0 {}
#[doc = "Domain Error Word1 Register"]
pub mod derr_w1_0;
#[doc = "Domain Error Word3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [derr_w3_0](derr_w3_0) module"]
pub type DERR_W3_0 = crate::Reg<u32, _DERR_W3_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DERR_W3_0;
#[doc = "`read()` method returns [derr_w3_0::R](derr_w3_0::R) reader structure"]
impl crate::Readable for DERR_W3_0 {}
#[doc = "`write(|w| ..)` method takes [derr_w3_0::W](derr_w3_0::W) writer structure"]
impl crate::Writable for DERR_W3_0 {}
#[doc = "Domain Error Word3 Register"]
pub mod derr_w3_0;
#[doc = "Domain Error Word0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [derr_w0_1](derr_w0_1) module"]
pub type DERR_W0_1 = crate::Reg<u32, _DERR_W0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DERR_W0_1;
#[doc = "`read()` method returns [derr_w0_1::R](derr_w0_1::R) reader structure"]
impl crate::Readable for DERR_W0_1 {}
#[doc = "Domain Error Word0 Register"]
pub mod derr_w0_1;
#[doc = "Domain Error Word1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [derr_w1_1](derr_w1_1) module"]
pub type DERR_W1_1 = crate::Reg<u32, _DERR_W1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DERR_W1_1;
#[doc = "`read()` method returns [derr_w1_1::R](derr_w1_1::R) reader structure"]
impl crate::Readable for DERR_W1_1 {}
#[doc = "Domain Error Word1 Register"]
pub mod derr_w1_1;
#[doc = "Domain Error Word3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [derr_w3_1](derr_w3_1) module"]
pub type DERR_W3_1 = crate::Reg<u32, _DERR_W3_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DERR_W3_1;
#[doc = "`read()` method returns [derr_w3_1::R](derr_w3_1::R) reader structure"]
impl crate::Readable for DERR_W3_1 {}
#[doc = "`write(|w| ..)` method takes [derr_w3_1::W](derr_w3_1::W) writer structure"]
impl crate::Writable for DERR_W3_1 {}
#[doc = "Domain Error Word3 Register"]
pub mod derr_w3_1;
#[doc = "Domain Error Word0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [derr_w0_16](derr_w0_16) module"]
pub type DERR_W0_16 = crate::Reg<u32, _DERR_W0_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DERR_W0_16;
#[doc = "`read()` method returns [derr_w0_16::R](derr_w0_16::R) reader structure"]
impl crate::Readable for DERR_W0_16 {}
#[doc = "Domain Error Word0 Register"]
pub mod derr_w0_16;
#[doc = "Domain Error Word1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [derr_w1_16](derr_w1_16) module"]
pub type DERR_W1_16 = crate::Reg<u32, _DERR_W1_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DERR_W1_16;
#[doc = "`read()` method returns [derr_w1_16::R](derr_w1_16::R) reader structure"]
impl crate::Readable for DERR_W1_16 {}
#[doc = "Domain Error Word1 Register"]
pub mod derr_w1_16;
#[doc = "Domain Error Word3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [derr_w3_16](derr_w3_16) module"]
pub type DERR_W3_16 = crate::Reg<u32, _DERR_W3_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DERR_W3_16;
#[doc = "`read()` method returns [derr_w3_16::R](derr_w3_16::R) reader structure"]
impl crate::Readable for DERR_W3_16 {}
#[doc = "`write(|w| ..)` method takes [derr_w3_16::W](derr_w3_16::W) writer structure"]
impl crate::Writable for DERR_W3_16 {}
#[doc = "Domain Error Word3 Register"]
pub mod derr_w3_16;
#[doc = "Domain Error Word0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [derr_w0_17](derr_w0_17) module"]
pub type DERR_W0_17 = crate::Reg<u32, _DERR_W0_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DERR_W0_17;
#[doc = "`read()` method returns [derr_w0_17::R](derr_w0_17::R) reader structure"]
impl crate::Readable for DERR_W0_17 {}
#[doc = "Domain Error Word0 Register"]
pub mod derr_w0_17;
#[doc = "Domain Error Word1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [derr_w1_17](derr_w1_17) module"]
pub type DERR_W1_17 = crate::Reg<u32, _DERR_W1_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DERR_W1_17;
#[doc = "`read()` method returns [derr_w1_17::R](derr_w1_17::R) reader structure"]
impl crate::Readable for DERR_W1_17 {}
#[doc = "Domain Error Word1 Register"]
pub mod derr_w1_17;
#[doc = "Domain Error Word3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [derr_w3_17](derr_w3_17) module"]
pub type DERR_W3_17 = crate::Reg<u32, _DERR_W3_17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DERR_W3_17;
#[doc = "`read()` method returns [derr_w3_17::R](derr_w3_17::R) reader structure"]
impl crate::Readable for DERR_W3_17 {}
#[doc = "`write(|w| ..)` method takes [derr_w3_17::W](derr_w3_17::W) writer structure"]
impl crate::Writable for DERR_W3_17 {}
#[doc = "Domain Error Word3 Register"]
pub mod derr_w3_17;
#[doc = "Domain Error Word0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [derr_w0_18](derr_w0_18) module"]
pub type DERR_W0_18 = crate::Reg<u32, _DERR_W0_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DERR_W0_18;
#[doc = "`read()` method returns [derr_w0_18::R](derr_w0_18::R) reader structure"]
impl crate::Readable for DERR_W0_18 {}
#[doc = "Domain Error Word0 Register"]
pub mod derr_w0_18;
#[doc = "Domain Error Word1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [derr_w1_18](derr_w1_18) module"]
pub type DERR_W1_18 = crate::Reg<u32, _DERR_W1_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DERR_W1_18;
#[doc = "`read()` method returns [derr_w1_18::R](derr_w1_18::R) reader structure"]
impl crate::Readable for DERR_W1_18 {}
#[doc = "Domain Error Word1 Register"]
pub mod derr_w1_18;
#[doc = "Domain Error Word3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [derr_w3_18](derr_w3_18) module"]
pub type DERR_W3_18 = crate::Reg<u32, _DERR_W3_18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DERR_W3_18;
#[doc = "`read()` method returns [derr_w3_18::R](derr_w3_18::R) reader structure"]
impl crate::Readable for DERR_W3_18 {}
#[doc = "`write(|w| ..)` method takes [derr_w3_18::W](derr_w3_18::W) writer structure"]
impl crate::Writable for DERR_W3_18 {}
#[doc = "Domain Error Word3 Register"]
pub mod derr_w3_18;
#[doc = "Process Identifier\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pid0](pid0) module"]
pub type PID0 = crate::Reg<u32, _PID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID0;
#[doc = "`read()` method returns [pid0::R](pid0::R) reader structure"]
impl crate::Readable for PID0 {}
#[doc = "`write(|w| ..)` method takes [pid0::W](pid0::W) writer structure"]
impl crate::Writable for PID0 {}
#[doc = "Process Identifier"]
pub mod pid0;
#[doc = "Process Identifier\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pid1](pid1) module"]
pub type PID1 = crate::Reg<u32, _PID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID1;
#[doc = "`read()` method returns [pid1::R](pid1::R) reader structure"]
impl crate::Readable for PID1 {}
#[doc = "`write(|w| ..)` method takes [pid1::W](pid1::W) writer structure"]
impl crate::Writable for PID1 {}
#[doc = "Process Identifier"]
pub mod pid1;
#[doc = "Process Identifier\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pid32](pid32) module"]
pub type PID32 = crate::Reg<u32, _PID32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID32;
#[doc = "`read()` method returns [pid32::R](pid32::R) reader structure"]
impl crate::Readable for PID32 {}
#[doc = "`write(|w| ..)` method takes [pid32::W](pid32::W) writer structure"]
impl crate::Writable for PID32 {}
#[doc = "Process Identifier"]
pub mod pid32;
#[doc = "Master Domain Assignment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mda_w0_0_dfmt0](mda_w0_0_dfmt0) module"]
pub type MDA_W0_0_DFMT0 = crate::Reg<u32, _MDA_W0_0_DFMT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDA_W0_0_DFMT0;
#[doc = "`read()` method returns [mda_w0_0_dfmt0::R](mda_w0_0_dfmt0::R) reader structure"]
impl crate::Readable for MDA_W0_0_DFMT0 {}
#[doc = "`write(|w| ..)` method takes [mda_w0_0_dfmt0::W](mda_w0_0_dfmt0::W) writer structure"]
impl crate::Writable for MDA_W0_0_DFMT0 {}
#[doc = "Master Domain Assignment"]
pub mod mda_w0_0_dfmt0;
#[doc = "Master Domain Assignment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mda_w1_0_dfmt0](mda_w1_0_dfmt0) module"]
pub type MDA_W1_0_DFMT0 = crate::Reg<u32, _MDA_W1_0_DFMT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDA_W1_0_DFMT0;
#[doc = "`read()` method returns [mda_w1_0_dfmt0::R](mda_w1_0_dfmt0::R) reader structure"]
impl crate::Readable for MDA_W1_0_DFMT0 {}
#[doc = "`write(|w| ..)` method takes [mda_w1_0_dfmt0::W](mda_w1_0_dfmt0::W) writer structure"]
impl crate::Writable for MDA_W1_0_DFMT0 {}
#[doc = "Master Domain Assignment"]
pub mod mda_w1_0_dfmt0;
#[doc = "Master Domain Assignment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mda_w0_1_dfmt0](mda_w0_1_dfmt0) module"]
pub type MDA_W0_1_DFMT0 = crate::Reg<u32, _MDA_W0_1_DFMT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDA_W0_1_DFMT0;
#[doc = "`read()` method returns [mda_w0_1_dfmt0::R](mda_w0_1_dfmt0::R) reader structure"]
impl crate::Readable for MDA_W0_1_DFMT0 {}
#[doc = "`write(|w| ..)` method takes [mda_w0_1_dfmt0::W](mda_w0_1_dfmt0::W) writer structure"]
impl crate::Writable for MDA_W0_1_DFMT0 {}
#[doc = "Master Domain Assignment"]
pub mod mda_w0_1_dfmt0;
#[doc = "Master Domain Assignment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mda_w1_1_dfmt0](mda_w1_1_dfmt0) module"]
pub type MDA_W1_1_DFMT0 = crate::Reg<u32, _MDA_W1_1_DFMT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDA_W1_1_DFMT0;
#[doc = "`read()` method returns [mda_w1_1_dfmt0::R](mda_w1_1_dfmt0::R) reader structure"]
impl crate::Readable for MDA_W1_1_DFMT0 {}
#[doc = "`write(|w| ..)` method takes [mda_w1_1_dfmt0::W](mda_w1_1_dfmt0::W) writer structure"]
impl crate::Writable for MDA_W1_1_DFMT0 {}
#[doc = "Master Domain Assignment"]
pub mod mda_w1_1_dfmt0;
#[doc = "Master Domain Assignment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mda_w0_2_dfmt1](mda_w0_2_dfmt1) module"]
pub type MDA_W0_2_DFMT1 = crate::Reg<u32, _MDA_W0_2_DFMT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDA_W0_2_DFMT1;
#[doc = "`read()` method returns [mda_w0_2_dfmt1::R](mda_w0_2_dfmt1::R) reader structure"]
impl crate::Readable for MDA_W0_2_DFMT1 {}
#[doc = "`write(|w| ..)` method takes [mda_w0_2_dfmt1::W](mda_w0_2_dfmt1::W) writer structure"]
impl crate::Writable for MDA_W0_2_DFMT1 {}
#[doc = "Master Domain Assignment"]
pub mod mda_w0_2_dfmt1;
#[doc = "Master Domain Assignment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mda_w0_3_dfmt1](mda_w0_3_dfmt1) module"]
pub type MDA_W0_3_DFMT1 = crate::Reg<u32, _MDA_W0_3_DFMT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDA_W0_3_DFMT1;
#[doc = "`read()` method returns [mda_w0_3_dfmt1::R](mda_w0_3_dfmt1::R) reader structure"]
impl crate::Readable for MDA_W0_3_DFMT1 {}
#[doc = "`write(|w| ..)` method takes [mda_w0_3_dfmt1::W](mda_w0_3_dfmt1::W) writer structure"]
impl crate::Writable for MDA_W0_3_DFMT1 {}
#[doc = "Master Domain Assignment"]
pub mod mda_w0_3_dfmt1;
#[doc = "Master Domain Assignment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mda_w0_4_dfmt1](mda_w0_4_dfmt1) module"]
pub type MDA_W0_4_DFMT1 = crate::Reg<u32, _MDA_W0_4_DFMT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDA_W0_4_DFMT1;
#[doc = "`read()` method returns [mda_w0_4_dfmt1::R](mda_w0_4_dfmt1::R) reader structure"]
impl crate::Readable for MDA_W0_4_DFMT1 {}
#[doc = "`write(|w| ..)` method takes [mda_w0_4_dfmt1::W](mda_w0_4_dfmt1::W) writer structure"]
impl crate::Writable for MDA_W0_4_DFMT1 {}
#[doc = "Master Domain Assignment"]
pub mod mda_w0_4_dfmt1;
#[doc = "Master Domain Assignment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mda_w0_32_dfmt0](mda_w0_32_dfmt0) module"]
pub type MDA_W0_32_DFMT0 = crate::Reg<u32, _MDA_W0_32_DFMT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDA_W0_32_DFMT0;
#[doc = "`read()` method returns [mda_w0_32_dfmt0::R](mda_w0_32_dfmt0::R) reader structure"]
impl crate::Readable for MDA_W0_32_DFMT0 {}
#[doc = "`write(|w| ..)` method takes [mda_w0_32_dfmt0::W](mda_w0_32_dfmt0::W) writer structure"]
impl crate::Writable for MDA_W0_32_DFMT0 {}
#[doc = "Master Domain Assignment"]
pub mod mda_w0_32_dfmt0;
#[doc = "Master Domain Assignment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mda_w1_32_dfmt0](mda_w1_32_dfmt0) module"]
pub type MDA_W1_32_DFMT0 = crate::Reg<u32, _MDA_W1_32_DFMT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDA_W1_32_DFMT0;
#[doc = "`read()` method returns [mda_w1_32_dfmt0::R](mda_w1_32_dfmt0::R) reader structure"]
impl crate::Readable for MDA_W1_32_DFMT0 {}
#[doc = "`write(|w| ..)` method takes [mda_w1_32_dfmt0::W](mda_w1_32_dfmt0::W) writer structure"]
impl crate::Writable for MDA_W1_32_DFMT0 {}
#[doc = "Master Domain Assignment"]
pub mod mda_w1_32_dfmt0;
#[doc = "Master Domain Assignment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mda_w0_33_dfmt1](mda_w0_33_dfmt1) module"]
pub type MDA_W0_33_DFMT1 = crate::Reg<u32, _MDA_W0_33_DFMT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDA_W0_33_DFMT1;
#[doc = "`read()` method returns [mda_w0_33_dfmt1::R](mda_w0_33_dfmt1::R) reader structure"]
impl crate::Readable for MDA_W0_33_DFMT1 {}
#[doc = "`write(|w| ..)` method takes [mda_w0_33_dfmt1::W](mda_w0_33_dfmt1::W) writer structure"]
impl crate::Writable for MDA_W0_33_DFMT1 {}
#[doc = "Master Domain Assignment"]
pub mod mda_w0_33_dfmt1;
#[doc = "Master Domain Assignment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mda_w0_34_dfmt1](mda_w0_34_dfmt1) module"]
pub type MDA_W0_34_DFMT1 = crate::Reg<u32, _MDA_W0_34_DFMT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDA_W0_34_DFMT1;
#[doc = "`read()` method returns [mda_w0_34_dfmt1::R](mda_w0_34_dfmt1::R) reader structure"]
impl crate::Readable for MDA_W0_34_DFMT1 {}
#[doc = "`write(|w| ..)` method takes [mda_w0_34_dfmt1::W](mda_w0_34_dfmt1::W) writer structure"]
impl crate::Writable for MDA_W0_34_DFMT1 {}
#[doc = "Master Domain Assignment"]
pub mod mda_w0_34_dfmt1;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_1](pdac_w0_0_1) module"]
pub type PDAC_W0_0_1 = crate::Reg<u32, _PDAC_W0_0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_1;
#[doc = "`read()` method returns [pdac_w0_0_1::R](pdac_w0_0_1::R) reader structure"]
impl crate::Readable for PDAC_W0_0_1 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_1::W](pdac_w0_0_1::W) writer structure"]
impl crate::Writable for PDAC_W0_0_1 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_1;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_1](pdac_w1_0_1) module"]
pub type PDAC_W1_0_1 = crate::Reg<u32, _PDAC_W1_0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_1;
#[doc = "`read()` method returns [pdac_w1_0_1::R](pdac_w1_0_1::R) reader structure"]
impl crate::Readable for PDAC_W1_0_1 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_1::W](pdac_w1_0_1::W) writer structure"]
impl crate::Writable for PDAC_W1_0_1 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_1;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_3](pdac_w0_0_3) module"]
pub type PDAC_W0_0_3 = crate::Reg<u32, _PDAC_W0_0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_3;
#[doc = "`read()` method returns [pdac_w0_0_3::R](pdac_w0_0_3::R) reader structure"]
impl crate::Readable for PDAC_W0_0_3 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_3::W](pdac_w0_0_3::W) writer structure"]
impl crate::Writable for PDAC_W0_0_3 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_3;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_3](pdac_w1_0_3) module"]
pub type PDAC_W1_0_3 = crate::Reg<u32, _PDAC_W1_0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_3;
#[doc = "`read()` method returns [pdac_w1_0_3::R](pdac_w1_0_3::R) reader structure"]
impl crate::Readable for PDAC_W1_0_3 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_3::W](pdac_w1_0_3::W) writer structure"]
impl crate::Writable for PDAC_W1_0_3 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_3;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_4](pdac_w0_0_4) module"]
pub type PDAC_W0_0_4 = crate::Reg<u32, _PDAC_W0_0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_4;
#[doc = "`read()` method returns [pdac_w0_0_4::R](pdac_w0_0_4::R) reader structure"]
impl crate::Readable for PDAC_W0_0_4 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_4::W](pdac_w0_0_4::W) writer structure"]
impl crate::Writable for PDAC_W0_0_4 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_4;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_4](pdac_w1_0_4) module"]
pub type PDAC_W1_0_4 = crate::Reg<u32, _PDAC_W1_0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_4;
#[doc = "`read()` method returns [pdac_w1_0_4::R](pdac_w1_0_4::R) reader structure"]
impl crate::Readable for PDAC_W1_0_4 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_4::W](pdac_w1_0_4::W) writer structure"]
impl crate::Writable for PDAC_W1_0_4 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_4;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_8](pdac_w0_0_8) module"]
pub type PDAC_W0_0_8 = crate::Reg<u32, _PDAC_W0_0_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_8;
#[doc = "`read()` method returns [pdac_w0_0_8::R](pdac_w0_0_8::R) reader structure"]
impl crate::Readable for PDAC_W0_0_8 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_8::W](pdac_w0_0_8::W) writer structure"]
impl crate::Writable for PDAC_W0_0_8 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_8;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_8](pdac_w1_0_8) module"]
pub type PDAC_W1_0_8 = crate::Reg<u32, _PDAC_W1_0_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_8;
#[doc = "`read()` method returns [pdac_w1_0_8::R](pdac_w1_0_8::R) reader structure"]
impl crate::Readable for PDAC_W1_0_8 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_8::W](pdac_w1_0_8::W) writer structure"]
impl crate::Writable for PDAC_W1_0_8 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_8;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_9](pdac_w0_0_9) module"]
pub type PDAC_W0_0_9 = crate::Reg<u32, _PDAC_W0_0_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_9;
#[doc = "`read()` method returns [pdac_w0_0_9::R](pdac_w0_0_9::R) reader structure"]
impl crate::Readable for PDAC_W0_0_9 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_9::W](pdac_w0_0_9::W) writer structure"]
impl crate::Writable for PDAC_W0_0_9 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_9;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_9](pdac_w1_0_9) module"]
pub type PDAC_W1_0_9 = crate::Reg<u32, _PDAC_W1_0_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_9;
#[doc = "`read()` method returns [pdac_w1_0_9::R](pdac_w1_0_9::R) reader structure"]
impl crate::Readable for PDAC_W1_0_9 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_9::W](pdac_w1_0_9::W) writer structure"]
impl crate::Writable for PDAC_W1_0_9 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_9;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_12](pdac_w0_0_12) module"]
pub type PDAC_W0_0_12 = crate::Reg<u32, _PDAC_W0_0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_12;
#[doc = "`read()` method returns [pdac_w0_0_12::R](pdac_w0_0_12::R) reader structure"]
impl crate::Readable for PDAC_W0_0_12 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_12::W](pdac_w0_0_12::W) writer structure"]
impl crate::Writable for PDAC_W0_0_12 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_12;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_12](pdac_w1_0_12) module"]
pub type PDAC_W1_0_12 = crate::Reg<u32, _PDAC_W1_0_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_12;
#[doc = "`read()` method returns [pdac_w1_0_12::R](pdac_w1_0_12::R) reader structure"]
impl crate::Readable for PDAC_W1_0_12 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_12::W](pdac_w1_0_12::W) writer structure"]
impl crate::Writable for PDAC_W1_0_12 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_12;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_15](pdac_w0_0_15) module"]
pub type PDAC_W0_0_15 = crate::Reg<u32, _PDAC_W0_0_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_15;
#[doc = "`read()` method returns [pdac_w0_0_15::R](pdac_w0_0_15::R) reader structure"]
impl crate::Readable for PDAC_W0_0_15 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_15::W](pdac_w0_0_15::W) writer structure"]
impl crate::Writable for PDAC_W0_0_15 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_15;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_15](pdac_w1_0_15) module"]
pub type PDAC_W1_0_15 = crate::Reg<u32, _PDAC_W1_0_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_15;
#[doc = "`read()` method returns [pdac_w1_0_15::R](pdac_w1_0_15::R) reader structure"]
impl crate::Readable for PDAC_W1_0_15 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_15::W](pdac_w1_0_15::W) writer structure"]
impl crate::Writable for PDAC_W1_0_15 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_15;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_20](pdac_w0_0_20) module"]
pub type PDAC_W0_0_20 = crate::Reg<u32, _PDAC_W0_0_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_20;
#[doc = "`read()` method returns [pdac_w0_0_20::R](pdac_w0_0_20::R) reader structure"]
impl crate::Readable for PDAC_W0_0_20 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_20::W](pdac_w0_0_20::W) writer structure"]
impl crate::Writable for PDAC_W0_0_20 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_20;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_20](pdac_w1_0_20) module"]
pub type PDAC_W1_0_20 = crate::Reg<u32, _PDAC_W1_0_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_20;
#[doc = "`read()` method returns [pdac_w1_0_20::R](pdac_w1_0_20::R) reader structure"]
impl crate::Readable for PDAC_W1_0_20 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_20::W](pdac_w1_0_20::W) writer structure"]
impl crate::Writable for PDAC_W1_0_20 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_20;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_21](pdac_w0_0_21) module"]
pub type PDAC_W0_0_21 = crate::Reg<u32, _PDAC_W0_0_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_21;
#[doc = "`read()` method returns [pdac_w0_0_21::R](pdac_w0_0_21::R) reader structure"]
impl crate::Readable for PDAC_W0_0_21 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_21::W](pdac_w0_0_21::W) writer structure"]
impl crate::Writable for PDAC_W0_0_21 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_21;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_21](pdac_w1_0_21) module"]
pub type PDAC_W1_0_21 = crate::Reg<u32, _PDAC_W1_0_21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_21;
#[doc = "`read()` method returns [pdac_w1_0_21::R](pdac_w1_0_21::R) reader structure"]
impl crate::Readable for PDAC_W1_0_21 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_21::W](pdac_w1_0_21::W) writer structure"]
impl crate::Writable for PDAC_W1_0_21 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_21;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_22](pdac_w0_0_22) module"]
pub type PDAC_W0_0_22 = crate::Reg<u32, _PDAC_W0_0_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_22;
#[doc = "`read()` method returns [pdac_w0_0_22::R](pdac_w0_0_22::R) reader structure"]
impl crate::Readable for PDAC_W0_0_22 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_22::W](pdac_w0_0_22::W) writer structure"]
impl crate::Writable for PDAC_W0_0_22 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_22;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_22](pdac_w1_0_22) module"]
pub type PDAC_W1_0_22 = crate::Reg<u32, _PDAC_W1_0_22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_22;
#[doc = "`read()` method returns [pdac_w1_0_22::R](pdac_w1_0_22::R) reader structure"]
impl crate::Readable for PDAC_W1_0_22 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_22::W](pdac_w1_0_22::W) writer structure"]
impl crate::Writable for PDAC_W1_0_22 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_22;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_23](pdac_w0_0_23) module"]
pub type PDAC_W0_0_23 = crate::Reg<u32, _PDAC_W0_0_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_23;
#[doc = "`read()` method returns [pdac_w0_0_23::R](pdac_w0_0_23::R) reader structure"]
impl crate::Readable for PDAC_W0_0_23 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_23::W](pdac_w0_0_23::W) writer structure"]
impl crate::Writable for PDAC_W0_0_23 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_23;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_23](pdac_w1_0_23) module"]
pub type PDAC_W1_0_23 = crate::Reg<u32, _PDAC_W1_0_23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_23;
#[doc = "`read()` method returns [pdac_w1_0_23::R](pdac_w1_0_23::R) reader structure"]
impl crate::Readable for PDAC_W1_0_23 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_23::W](pdac_w1_0_23::W) writer structure"]
impl crate::Writable for PDAC_W1_0_23 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_23;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_27](pdac_w0_0_27) module"]
pub type PDAC_W0_0_27 = crate::Reg<u32, _PDAC_W0_0_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_27;
#[doc = "`read()` method returns [pdac_w0_0_27::R](pdac_w0_0_27::R) reader structure"]
impl crate::Readable for PDAC_W0_0_27 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_27::W](pdac_w0_0_27::W) writer structure"]
impl crate::Writable for PDAC_W0_0_27 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_27;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_27](pdac_w1_0_27) module"]
pub type PDAC_W1_0_27 = crate::Reg<u32, _PDAC_W1_0_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_27;
#[doc = "`read()` method returns [pdac_w1_0_27::R](pdac_w1_0_27::R) reader structure"]
impl crate::Readable for PDAC_W1_0_27 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_27::W](pdac_w1_0_27::W) writer structure"]
impl crate::Writable for PDAC_W1_0_27 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_27;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_32](pdac_w0_0_32) module"]
pub type PDAC_W0_0_32 = crate::Reg<u32, _PDAC_W0_0_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_32;
#[doc = "`read()` method returns [pdac_w0_0_32::R](pdac_w0_0_32::R) reader structure"]
impl crate::Readable for PDAC_W0_0_32 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_32::W](pdac_w0_0_32::W) writer structure"]
impl crate::Writable for PDAC_W0_0_32 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_32;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_32](pdac_w1_0_32) module"]
pub type PDAC_W1_0_32 = crate::Reg<u32, _PDAC_W1_0_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_32;
#[doc = "`read()` method returns [pdac_w1_0_32::R](pdac_w1_0_32::R) reader structure"]
impl crate::Readable for PDAC_W1_0_32 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_32::W](pdac_w1_0_32::W) writer structure"]
impl crate::Writable for PDAC_W1_0_32 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_32;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_33](pdac_w0_0_33) module"]
pub type PDAC_W0_0_33 = crate::Reg<u32, _PDAC_W0_0_33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_33;
#[doc = "`read()` method returns [pdac_w0_0_33::R](pdac_w0_0_33::R) reader structure"]
impl crate::Readable for PDAC_W0_0_33 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_33::W](pdac_w0_0_33::W) writer structure"]
impl crate::Writable for PDAC_W0_0_33 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_33;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_33](pdac_w1_0_33) module"]
pub type PDAC_W1_0_33 = crate::Reg<u32, _PDAC_W1_0_33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_33;
#[doc = "`read()` method returns [pdac_w1_0_33::R](pdac_w1_0_33::R) reader structure"]
impl crate::Readable for PDAC_W1_0_33 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_33::W](pdac_w1_0_33::W) writer structure"]
impl crate::Writable for PDAC_W1_0_33 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_33;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_34](pdac_w0_0_34) module"]
pub type PDAC_W0_0_34 = crate::Reg<u32, _PDAC_W0_0_34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_34;
#[doc = "`read()` method returns [pdac_w0_0_34::R](pdac_w0_0_34::R) reader structure"]
impl crate::Readable for PDAC_W0_0_34 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_34::W](pdac_w0_0_34::W) writer structure"]
impl crate::Writable for PDAC_W0_0_34 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_34;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_34](pdac_w1_0_34) module"]
pub type PDAC_W1_0_34 = crate::Reg<u32, _PDAC_W1_0_34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_34;
#[doc = "`read()` method returns [pdac_w1_0_34::R](pdac_w1_0_34::R) reader structure"]
impl crate::Readable for PDAC_W1_0_34 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_34::W](pdac_w1_0_34::W) writer structure"]
impl crate::Writable for PDAC_W1_0_34 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_34;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_35](pdac_w0_0_35) module"]
pub type PDAC_W0_0_35 = crate::Reg<u32, _PDAC_W0_0_35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_35;
#[doc = "`read()` method returns [pdac_w0_0_35::R](pdac_w0_0_35::R) reader structure"]
impl crate::Readable for PDAC_W0_0_35 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_35::W](pdac_w0_0_35::W) writer structure"]
impl crate::Writable for PDAC_W0_0_35 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_35;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_35](pdac_w1_0_35) module"]
pub type PDAC_W1_0_35 = crate::Reg<u32, _PDAC_W1_0_35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_35;
#[doc = "`read()` method returns [pdac_w1_0_35::R](pdac_w1_0_35::R) reader structure"]
impl crate::Readable for PDAC_W1_0_35 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_35::W](pdac_w1_0_35::W) writer structure"]
impl crate::Writable for PDAC_W1_0_35 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_35;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_36](pdac_w0_0_36) module"]
pub type PDAC_W0_0_36 = crate::Reg<u32, _PDAC_W0_0_36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_36;
#[doc = "`read()` method returns [pdac_w0_0_36::R](pdac_w0_0_36::R) reader structure"]
impl crate::Readable for PDAC_W0_0_36 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_36::W](pdac_w0_0_36::W) writer structure"]
impl crate::Writable for PDAC_W0_0_36 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_36;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_36](pdac_w1_0_36) module"]
pub type PDAC_W1_0_36 = crate::Reg<u32, _PDAC_W1_0_36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_36;
#[doc = "`read()` method returns [pdac_w1_0_36::R](pdac_w1_0_36::R) reader structure"]
impl crate::Readable for PDAC_W1_0_36 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_36::W](pdac_w1_0_36::W) writer structure"]
impl crate::Writable for PDAC_W1_0_36 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_36;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_37](pdac_w0_0_37) module"]
pub type PDAC_W0_0_37 = crate::Reg<u32, _PDAC_W0_0_37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_37;
#[doc = "`read()` method returns [pdac_w0_0_37::R](pdac_w0_0_37::R) reader structure"]
impl crate::Readable for PDAC_W0_0_37 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_37::W](pdac_w0_0_37::W) writer structure"]
impl crate::Writable for PDAC_W0_0_37 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_37;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_37](pdac_w1_0_37) module"]
pub type PDAC_W1_0_37 = crate::Reg<u32, _PDAC_W1_0_37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_37;
#[doc = "`read()` method returns [pdac_w1_0_37::R](pdac_w1_0_37::R) reader structure"]
impl crate::Readable for PDAC_W1_0_37 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_37::W](pdac_w1_0_37::W) writer structure"]
impl crate::Writable for PDAC_W1_0_37 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_37;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_38](pdac_w0_0_38) module"]
pub type PDAC_W0_0_38 = crate::Reg<u32, _PDAC_W0_0_38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_38;
#[doc = "`read()` method returns [pdac_w0_0_38::R](pdac_w0_0_38::R) reader structure"]
impl crate::Readable for PDAC_W0_0_38 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_38::W](pdac_w0_0_38::W) writer structure"]
impl crate::Writable for PDAC_W0_0_38 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_38;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_38](pdac_w1_0_38) module"]
pub type PDAC_W1_0_38 = crate::Reg<u32, _PDAC_W1_0_38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_38;
#[doc = "`read()` method returns [pdac_w1_0_38::R](pdac_w1_0_38::R) reader structure"]
impl crate::Readable for PDAC_W1_0_38 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_38::W](pdac_w1_0_38::W) writer structure"]
impl crate::Writable for PDAC_W1_0_38 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_38;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_39](pdac_w0_0_39) module"]
pub type PDAC_W0_0_39 = crate::Reg<u32, _PDAC_W0_0_39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_39;
#[doc = "`read()` method returns [pdac_w0_0_39::R](pdac_w0_0_39::R) reader structure"]
impl crate::Readable for PDAC_W0_0_39 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_39::W](pdac_w0_0_39::W) writer structure"]
impl crate::Writable for PDAC_W0_0_39 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_39;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_39](pdac_w1_0_39) module"]
pub type PDAC_W1_0_39 = crate::Reg<u32, _PDAC_W1_0_39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_39;
#[doc = "`read()` method returns [pdac_w1_0_39::R](pdac_w1_0_39::R) reader structure"]
impl crate::Readable for PDAC_W1_0_39 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_39::W](pdac_w1_0_39::W) writer structure"]
impl crate::Writable for PDAC_W1_0_39 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_39;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_40](pdac_w0_0_40) module"]
pub type PDAC_W0_0_40 = crate::Reg<u32, _PDAC_W0_0_40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_40;
#[doc = "`read()` method returns [pdac_w0_0_40::R](pdac_w0_0_40::R) reader structure"]
impl crate::Readable for PDAC_W0_0_40 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_40::W](pdac_w0_0_40::W) writer structure"]
impl crate::Writable for PDAC_W0_0_40 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_40;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_40](pdac_w1_0_40) module"]
pub type PDAC_W1_0_40 = crate::Reg<u32, _PDAC_W1_0_40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_40;
#[doc = "`read()` method returns [pdac_w1_0_40::R](pdac_w1_0_40::R) reader structure"]
impl crate::Readable for PDAC_W1_0_40 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_40::W](pdac_w1_0_40::W) writer structure"]
impl crate::Writable for PDAC_W1_0_40 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_40;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_41](pdac_w0_0_41) module"]
pub type PDAC_W0_0_41 = crate::Reg<u32, _PDAC_W0_0_41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_41;
#[doc = "`read()` method returns [pdac_w0_0_41::R](pdac_w0_0_41::R) reader structure"]
impl crate::Readable for PDAC_W0_0_41 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_41::W](pdac_w0_0_41::W) writer structure"]
impl crate::Writable for PDAC_W0_0_41 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_41;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_41](pdac_w1_0_41) module"]
pub type PDAC_W1_0_41 = crate::Reg<u32, _PDAC_W1_0_41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_41;
#[doc = "`read()` method returns [pdac_w1_0_41::R](pdac_w1_0_41::R) reader structure"]
impl crate::Readable for PDAC_W1_0_41 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_41::W](pdac_w1_0_41::W) writer structure"]
impl crate::Writable for PDAC_W1_0_41 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_41;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_42](pdac_w0_0_42) module"]
pub type PDAC_W0_0_42 = crate::Reg<u32, _PDAC_W0_0_42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_42;
#[doc = "`read()` method returns [pdac_w0_0_42::R](pdac_w0_0_42::R) reader structure"]
impl crate::Readable for PDAC_W0_0_42 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_42::W](pdac_w0_0_42::W) writer structure"]
impl crate::Writable for PDAC_W0_0_42 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_42;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_42](pdac_w1_0_42) module"]
pub type PDAC_W1_0_42 = crate::Reg<u32, _PDAC_W1_0_42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_42;
#[doc = "`read()` method returns [pdac_w1_0_42::R](pdac_w1_0_42::R) reader structure"]
impl crate::Readable for PDAC_W1_0_42 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_42::W](pdac_w1_0_42::W) writer structure"]
impl crate::Writable for PDAC_W1_0_42 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_42;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_43](pdac_w0_0_43) module"]
pub type PDAC_W0_0_43 = crate::Reg<u32, _PDAC_W0_0_43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_43;
#[doc = "`read()` method returns [pdac_w0_0_43::R](pdac_w0_0_43::R) reader structure"]
impl crate::Readable for PDAC_W0_0_43 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_43::W](pdac_w0_0_43::W) writer structure"]
impl crate::Writable for PDAC_W0_0_43 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_43;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_43](pdac_w1_0_43) module"]
pub type PDAC_W1_0_43 = crate::Reg<u32, _PDAC_W1_0_43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_43;
#[doc = "`read()` method returns [pdac_w1_0_43::R](pdac_w1_0_43::R) reader structure"]
impl crate::Readable for PDAC_W1_0_43 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_43::W](pdac_w1_0_43::W) writer structure"]
impl crate::Writable for PDAC_W1_0_43 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_43;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_44](pdac_w0_0_44) module"]
pub type PDAC_W0_0_44 = crate::Reg<u32, _PDAC_W0_0_44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_44;
#[doc = "`read()` method returns [pdac_w0_0_44::R](pdac_w0_0_44::R) reader structure"]
impl crate::Readable for PDAC_W0_0_44 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_44::W](pdac_w0_0_44::W) writer structure"]
impl crate::Writable for PDAC_W0_0_44 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_44;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_44](pdac_w1_0_44) module"]
pub type PDAC_W1_0_44 = crate::Reg<u32, _PDAC_W1_0_44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_44;
#[doc = "`read()` method returns [pdac_w1_0_44::R](pdac_w1_0_44::R) reader structure"]
impl crate::Readable for PDAC_W1_0_44 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_44::W](pdac_w1_0_44::W) writer structure"]
impl crate::Writable for PDAC_W1_0_44 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_44;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_45](pdac_w0_0_45) module"]
pub type PDAC_W0_0_45 = crate::Reg<u32, _PDAC_W0_0_45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_45;
#[doc = "`read()` method returns [pdac_w0_0_45::R](pdac_w0_0_45::R) reader structure"]
impl crate::Readable for PDAC_W0_0_45 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_45::W](pdac_w0_0_45::W) writer structure"]
impl crate::Writable for PDAC_W0_0_45 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_45;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_45](pdac_w1_0_45) module"]
pub type PDAC_W1_0_45 = crate::Reg<u32, _PDAC_W1_0_45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_45;
#[doc = "`read()` method returns [pdac_w1_0_45::R](pdac_w1_0_45::R) reader structure"]
impl crate::Readable for PDAC_W1_0_45 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_45::W](pdac_w1_0_45::W) writer structure"]
impl crate::Writable for PDAC_W1_0_45 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_45;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_46](pdac_w0_0_46) module"]
pub type PDAC_W0_0_46 = crate::Reg<u32, _PDAC_W0_0_46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_46;
#[doc = "`read()` method returns [pdac_w0_0_46::R](pdac_w0_0_46::R) reader structure"]
impl crate::Readable for PDAC_W0_0_46 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_46::W](pdac_w0_0_46::W) writer structure"]
impl crate::Writable for PDAC_W0_0_46 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_46;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_46](pdac_w1_0_46) module"]
pub type PDAC_W1_0_46 = crate::Reg<u32, _PDAC_W1_0_46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_46;
#[doc = "`read()` method returns [pdac_w1_0_46::R](pdac_w1_0_46::R) reader structure"]
impl crate::Readable for PDAC_W1_0_46 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_46::W](pdac_w1_0_46::W) writer structure"]
impl crate::Writable for PDAC_W1_0_46 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_46;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_47](pdac_w0_0_47) module"]
pub type PDAC_W0_0_47 = crate::Reg<u32, _PDAC_W0_0_47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_47;
#[doc = "`read()` method returns [pdac_w0_0_47::R](pdac_w0_0_47::R) reader structure"]
impl crate::Readable for PDAC_W0_0_47 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_47::W](pdac_w0_0_47::W) writer structure"]
impl crate::Writable for PDAC_W0_0_47 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_47;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_47](pdac_w1_0_47) module"]
pub type PDAC_W1_0_47 = crate::Reg<u32, _PDAC_W1_0_47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_47;
#[doc = "`read()` method returns [pdac_w1_0_47::R](pdac_w1_0_47::R) reader structure"]
impl crate::Readable for PDAC_W1_0_47 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_47::W](pdac_w1_0_47::W) writer structure"]
impl crate::Writable for PDAC_W1_0_47 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_47;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_48](pdac_w0_0_48) module"]
pub type PDAC_W0_0_48 = crate::Reg<u32, _PDAC_W0_0_48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_48;
#[doc = "`read()` method returns [pdac_w0_0_48::R](pdac_w0_0_48::R) reader structure"]
impl crate::Readable for PDAC_W0_0_48 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_48::W](pdac_w0_0_48::W) writer structure"]
impl crate::Writable for PDAC_W0_0_48 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_48;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_48](pdac_w1_0_48) module"]
pub type PDAC_W1_0_48 = crate::Reg<u32, _PDAC_W1_0_48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_48;
#[doc = "`read()` method returns [pdac_w1_0_48::R](pdac_w1_0_48::R) reader structure"]
impl crate::Readable for PDAC_W1_0_48 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_48::W](pdac_w1_0_48::W) writer structure"]
impl crate::Writable for PDAC_W1_0_48 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_48;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_49](pdac_w0_0_49) module"]
pub type PDAC_W0_0_49 = crate::Reg<u32, _PDAC_W0_0_49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_49;
#[doc = "`read()` method returns [pdac_w0_0_49::R](pdac_w0_0_49::R) reader structure"]
impl crate::Readable for PDAC_W0_0_49 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_49::W](pdac_w0_0_49::W) writer structure"]
impl crate::Writable for PDAC_W0_0_49 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_49;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_49](pdac_w1_0_49) module"]
pub type PDAC_W1_0_49 = crate::Reg<u32, _PDAC_W1_0_49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_49;
#[doc = "`read()` method returns [pdac_w1_0_49::R](pdac_w1_0_49::R) reader structure"]
impl crate::Readable for PDAC_W1_0_49 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_49::W](pdac_w1_0_49::W) writer structure"]
impl crate::Writable for PDAC_W1_0_49 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_49;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_50](pdac_w0_0_50) module"]
pub type PDAC_W0_0_50 = crate::Reg<u32, _PDAC_W0_0_50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_50;
#[doc = "`read()` method returns [pdac_w0_0_50::R](pdac_w0_0_50::R) reader structure"]
impl crate::Readable for PDAC_W0_0_50 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_50::W](pdac_w0_0_50::W) writer structure"]
impl crate::Writable for PDAC_W0_0_50 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_50;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_50](pdac_w1_0_50) module"]
pub type PDAC_W1_0_50 = crate::Reg<u32, _PDAC_W1_0_50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_50;
#[doc = "`read()` method returns [pdac_w1_0_50::R](pdac_w1_0_50::R) reader structure"]
impl crate::Readable for PDAC_W1_0_50 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_50::W](pdac_w1_0_50::W) writer structure"]
impl crate::Writable for PDAC_W1_0_50 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_50;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_51](pdac_w0_0_51) module"]
pub type PDAC_W0_0_51 = crate::Reg<u32, _PDAC_W0_0_51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_51;
#[doc = "`read()` method returns [pdac_w0_0_51::R](pdac_w0_0_51::R) reader structure"]
impl crate::Readable for PDAC_W0_0_51 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_51::W](pdac_w0_0_51::W) writer structure"]
impl crate::Writable for PDAC_W0_0_51 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_51;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_51](pdac_w1_0_51) module"]
pub type PDAC_W1_0_51 = crate::Reg<u32, _PDAC_W1_0_51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_51;
#[doc = "`read()` method returns [pdac_w1_0_51::R](pdac_w1_0_51::R) reader structure"]
impl crate::Readable for PDAC_W1_0_51 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_51::W](pdac_w1_0_51::W) writer structure"]
impl crate::Writable for PDAC_W1_0_51 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_51;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_52](pdac_w0_0_52) module"]
pub type PDAC_W0_0_52 = crate::Reg<u32, _PDAC_W0_0_52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_52;
#[doc = "`read()` method returns [pdac_w0_0_52::R](pdac_w0_0_52::R) reader structure"]
impl crate::Readable for PDAC_W0_0_52 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_52::W](pdac_w0_0_52::W) writer structure"]
impl crate::Writable for PDAC_W0_0_52 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_52;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_52](pdac_w1_0_52) module"]
pub type PDAC_W1_0_52 = crate::Reg<u32, _PDAC_W1_0_52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_52;
#[doc = "`read()` method returns [pdac_w1_0_52::R](pdac_w1_0_52::R) reader structure"]
impl crate::Readable for PDAC_W1_0_52 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_52::W](pdac_w1_0_52::W) writer structure"]
impl crate::Writable for PDAC_W1_0_52 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_52;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_53](pdac_w0_0_53) module"]
pub type PDAC_W0_0_53 = crate::Reg<u32, _PDAC_W0_0_53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_53;
#[doc = "`read()` method returns [pdac_w0_0_53::R](pdac_w0_0_53::R) reader structure"]
impl crate::Readable for PDAC_W0_0_53 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_53::W](pdac_w0_0_53::W) writer structure"]
impl crate::Writable for PDAC_W0_0_53 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_53;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_53](pdac_w1_0_53) module"]
pub type PDAC_W1_0_53 = crate::Reg<u32, _PDAC_W1_0_53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_53;
#[doc = "`read()` method returns [pdac_w1_0_53::R](pdac_w1_0_53::R) reader structure"]
impl crate::Readable for PDAC_W1_0_53 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_53::W](pdac_w1_0_53::W) writer structure"]
impl crate::Writable for PDAC_W1_0_53 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_53;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_54](pdac_w0_0_54) module"]
pub type PDAC_W0_0_54 = crate::Reg<u32, _PDAC_W0_0_54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_54;
#[doc = "`read()` method returns [pdac_w0_0_54::R](pdac_w0_0_54::R) reader structure"]
impl crate::Readable for PDAC_W0_0_54 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_54::W](pdac_w0_0_54::W) writer structure"]
impl crate::Writable for PDAC_W0_0_54 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_54;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_54](pdac_w1_0_54) module"]
pub type PDAC_W1_0_54 = crate::Reg<u32, _PDAC_W1_0_54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_54;
#[doc = "`read()` method returns [pdac_w1_0_54::R](pdac_w1_0_54::R) reader structure"]
impl crate::Readable for PDAC_W1_0_54 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_54::W](pdac_w1_0_54::W) writer structure"]
impl crate::Writable for PDAC_W1_0_54 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_54;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_55](pdac_w0_0_55) module"]
pub type PDAC_W0_0_55 = crate::Reg<u32, _PDAC_W0_0_55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_55;
#[doc = "`read()` method returns [pdac_w0_0_55::R](pdac_w0_0_55::R) reader structure"]
impl crate::Readable for PDAC_W0_0_55 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_55::W](pdac_w0_0_55::W) writer structure"]
impl crate::Writable for PDAC_W0_0_55 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_55;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_55](pdac_w1_0_55) module"]
pub type PDAC_W1_0_55 = crate::Reg<u32, _PDAC_W1_0_55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_55;
#[doc = "`read()` method returns [pdac_w1_0_55::R](pdac_w1_0_55::R) reader structure"]
impl crate::Readable for PDAC_W1_0_55 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_55::W](pdac_w1_0_55::W) writer structure"]
impl crate::Writable for PDAC_W1_0_55 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_55;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_56](pdac_w0_0_56) module"]
pub type PDAC_W0_0_56 = crate::Reg<u32, _PDAC_W0_0_56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_56;
#[doc = "`read()` method returns [pdac_w0_0_56::R](pdac_w0_0_56::R) reader structure"]
impl crate::Readable for PDAC_W0_0_56 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_56::W](pdac_w0_0_56::W) writer structure"]
impl crate::Writable for PDAC_W0_0_56 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_56;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_56](pdac_w1_0_56) module"]
pub type PDAC_W1_0_56 = crate::Reg<u32, _PDAC_W1_0_56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_56;
#[doc = "`read()` method returns [pdac_w1_0_56::R](pdac_w1_0_56::R) reader structure"]
impl crate::Readable for PDAC_W1_0_56 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_56::W](pdac_w1_0_56::W) writer structure"]
impl crate::Writable for PDAC_W1_0_56 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_56;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_57](pdac_w0_0_57) module"]
pub type PDAC_W0_0_57 = crate::Reg<u32, _PDAC_W0_0_57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_57;
#[doc = "`read()` method returns [pdac_w0_0_57::R](pdac_w0_0_57::R) reader structure"]
impl crate::Readable for PDAC_W0_0_57 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_57::W](pdac_w0_0_57::W) writer structure"]
impl crate::Writable for PDAC_W0_0_57 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_57;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_57](pdac_w1_0_57) module"]
pub type PDAC_W1_0_57 = crate::Reg<u32, _PDAC_W1_0_57>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_57;
#[doc = "`read()` method returns [pdac_w1_0_57::R](pdac_w1_0_57::R) reader structure"]
impl crate::Readable for PDAC_W1_0_57 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_57::W](pdac_w1_0_57::W) writer structure"]
impl crate::Writable for PDAC_W1_0_57 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_57;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_58](pdac_w0_0_58) module"]
pub type PDAC_W0_0_58 = crate::Reg<u32, _PDAC_W0_0_58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_58;
#[doc = "`read()` method returns [pdac_w0_0_58::R](pdac_w0_0_58::R) reader structure"]
impl crate::Readable for PDAC_W0_0_58 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_58::W](pdac_w0_0_58::W) writer structure"]
impl crate::Writable for PDAC_W0_0_58 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_58;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_58](pdac_w1_0_58) module"]
pub type PDAC_W1_0_58 = crate::Reg<u32, _PDAC_W1_0_58>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_58;
#[doc = "`read()` method returns [pdac_w1_0_58::R](pdac_w1_0_58::R) reader structure"]
impl crate::Readable for PDAC_W1_0_58 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_58::W](pdac_w1_0_58::W) writer structure"]
impl crate::Writable for PDAC_W1_0_58 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_58;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_59](pdac_w0_0_59) module"]
pub type PDAC_W0_0_59 = crate::Reg<u32, _PDAC_W0_0_59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_59;
#[doc = "`read()` method returns [pdac_w0_0_59::R](pdac_w0_0_59::R) reader structure"]
impl crate::Readable for PDAC_W0_0_59 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_59::W](pdac_w0_0_59::W) writer structure"]
impl crate::Writable for PDAC_W0_0_59 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_59;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_59](pdac_w1_0_59) module"]
pub type PDAC_W1_0_59 = crate::Reg<u32, _PDAC_W1_0_59>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_59;
#[doc = "`read()` method returns [pdac_w1_0_59::R](pdac_w1_0_59::R) reader structure"]
impl crate::Readable for PDAC_W1_0_59 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_59::W](pdac_w1_0_59::W) writer structure"]
impl crate::Writable for PDAC_W1_0_59 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_59;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_60](pdac_w0_0_60) module"]
pub type PDAC_W0_0_60 = crate::Reg<u32, _PDAC_W0_0_60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_60;
#[doc = "`read()` method returns [pdac_w0_0_60::R](pdac_w0_0_60::R) reader structure"]
impl crate::Readable for PDAC_W0_0_60 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_60::W](pdac_w0_0_60::W) writer structure"]
impl crate::Writable for PDAC_W0_0_60 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_60;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_60](pdac_w1_0_60) module"]
pub type PDAC_W1_0_60 = crate::Reg<u32, _PDAC_W1_0_60>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_60;
#[doc = "`read()` method returns [pdac_w1_0_60::R](pdac_w1_0_60::R) reader structure"]
impl crate::Readable for PDAC_W1_0_60 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_60::W](pdac_w1_0_60::W) writer structure"]
impl crate::Writable for PDAC_W1_0_60 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_60;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_61](pdac_w0_0_61) module"]
pub type PDAC_W0_0_61 = crate::Reg<u32, _PDAC_W0_0_61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_61;
#[doc = "`read()` method returns [pdac_w0_0_61::R](pdac_w0_0_61::R) reader structure"]
impl crate::Readable for PDAC_W0_0_61 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_61::W](pdac_w0_0_61::W) writer structure"]
impl crate::Writable for PDAC_W0_0_61 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_61;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_61](pdac_w1_0_61) module"]
pub type PDAC_W1_0_61 = crate::Reg<u32, _PDAC_W1_0_61>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_61;
#[doc = "`read()` method returns [pdac_w1_0_61::R](pdac_w1_0_61::R) reader structure"]
impl crate::Readable for PDAC_W1_0_61 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_61::W](pdac_w1_0_61::W) writer structure"]
impl crate::Writable for PDAC_W1_0_61 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_61;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_62](pdac_w0_0_62) module"]
pub type PDAC_W0_0_62 = crate::Reg<u32, _PDAC_W0_0_62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_62;
#[doc = "`read()` method returns [pdac_w0_0_62::R](pdac_w0_0_62::R) reader structure"]
impl crate::Readable for PDAC_W0_0_62 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_62::W](pdac_w0_0_62::W) writer structure"]
impl crate::Writable for PDAC_W0_0_62 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_62;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_62](pdac_w1_0_62) module"]
pub type PDAC_W1_0_62 = crate::Reg<u32, _PDAC_W1_0_62>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_62;
#[doc = "`read()` method returns [pdac_w1_0_62::R](pdac_w1_0_62::R) reader structure"]
impl crate::Readable for PDAC_W1_0_62 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_62::W](pdac_w1_0_62::W) writer structure"]
impl crate::Writable for PDAC_W1_0_62 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_62;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_63](pdac_w0_0_63) module"]
pub type PDAC_W0_0_63 = crate::Reg<u32, _PDAC_W0_0_63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_63;
#[doc = "`read()` method returns [pdac_w0_0_63::R](pdac_w0_0_63::R) reader structure"]
impl crate::Readable for PDAC_W0_0_63 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_63::W](pdac_w0_0_63::W) writer structure"]
impl crate::Writable for PDAC_W0_0_63 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_63;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_63](pdac_w1_0_63) module"]
pub type PDAC_W1_0_63 = crate::Reg<u32, _PDAC_W1_0_63>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_63;
#[doc = "`read()` method returns [pdac_w1_0_63::R](pdac_w1_0_63::R) reader structure"]
impl crate::Readable for PDAC_W1_0_63 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_63::W](pdac_w1_0_63::W) writer structure"]
impl crate::Writable for PDAC_W1_0_63 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_63;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_64](pdac_w0_0_64) module"]
pub type PDAC_W0_0_64 = crate::Reg<u32, _PDAC_W0_0_64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_64;
#[doc = "`read()` method returns [pdac_w0_0_64::R](pdac_w0_0_64::R) reader structure"]
impl crate::Readable for PDAC_W0_0_64 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_64::W](pdac_w0_0_64::W) writer structure"]
impl crate::Writable for PDAC_W0_0_64 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_64;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_64](pdac_w1_0_64) module"]
pub type PDAC_W1_0_64 = crate::Reg<u32, _PDAC_W1_0_64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_64;
#[doc = "`read()` method returns [pdac_w1_0_64::R](pdac_w1_0_64::R) reader structure"]
impl crate::Readable for PDAC_W1_0_64 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_64::W](pdac_w1_0_64::W) writer structure"]
impl crate::Writable for PDAC_W1_0_64 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_64;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_65](pdac_w0_0_65) module"]
pub type PDAC_W0_0_65 = crate::Reg<u32, _PDAC_W0_0_65>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_65;
#[doc = "`read()` method returns [pdac_w0_0_65::R](pdac_w0_0_65::R) reader structure"]
impl crate::Readable for PDAC_W0_0_65 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_65::W](pdac_w0_0_65::W) writer structure"]
impl crate::Writable for PDAC_W0_0_65 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_65;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_65](pdac_w1_0_65) module"]
pub type PDAC_W1_0_65 = crate::Reg<u32, _PDAC_W1_0_65>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_65;
#[doc = "`read()` method returns [pdac_w1_0_65::R](pdac_w1_0_65::R) reader structure"]
impl crate::Readable for PDAC_W1_0_65 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_65::W](pdac_w1_0_65::W) writer structure"]
impl crate::Writable for PDAC_W1_0_65 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_65;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_66](pdac_w0_0_66) module"]
pub type PDAC_W0_0_66 = crate::Reg<u32, _PDAC_W0_0_66>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_66;
#[doc = "`read()` method returns [pdac_w0_0_66::R](pdac_w0_0_66::R) reader structure"]
impl crate::Readable for PDAC_W0_0_66 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_66::W](pdac_w0_0_66::W) writer structure"]
impl crate::Writable for PDAC_W0_0_66 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_66;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_66](pdac_w1_0_66) module"]
pub type PDAC_W1_0_66 = crate::Reg<u32, _PDAC_W1_0_66>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_66;
#[doc = "`read()` method returns [pdac_w1_0_66::R](pdac_w1_0_66::R) reader structure"]
impl crate::Readable for PDAC_W1_0_66 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_66::W](pdac_w1_0_66::W) writer structure"]
impl crate::Writable for PDAC_W1_0_66 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_66;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_67](pdac_w0_0_67) module"]
pub type PDAC_W0_0_67 = crate::Reg<u32, _PDAC_W0_0_67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_67;
#[doc = "`read()` method returns [pdac_w0_0_67::R](pdac_w0_0_67::R) reader structure"]
impl crate::Readable for PDAC_W0_0_67 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_67::W](pdac_w0_0_67::W) writer structure"]
impl crate::Writable for PDAC_W0_0_67 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_67;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_67](pdac_w1_0_67) module"]
pub type PDAC_W1_0_67 = crate::Reg<u32, _PDAC_W1_0_67>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_67;
#[doc = "`read()` method returns [pdac_w1_0_67::R](pdac_w1_0_67::R) reader structure"]
impl crate::Readable for PDAC_W1_0_67 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_67::W](pdac_w1_0_67::W) writer structure"]
impl crate::Writable for PDAC_W1_0_67 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_67;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_68](pdac_w0_0_68) module"]
pub type PDAC_W0_0_68 = crate::Reg<u32, _PDAC_W0_0_68>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_68;
#[doc = "`read()` method returns [pdac_w0_0_68::R](pdac_w0_0_68::R) reader structure"]
impl crate::Readable for PDAC_W0_0_68 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_68::W](pdac_w0_0_68::W) writer structure"]
impl crate::Writable for PDAC_W0_0_68 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_68;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_68](pdac_w1_0_68) module"]
pub type PDAC_W1_0_68 = crate::Reg<u32, _PDAC_W1_0_68>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_68;
#[doc = "`read()` method returns [pdac_w1_0_68::R](pdac_w1_0_68::R) reader structure"]
impl crate::Readable for PDAC_W1_0_68 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_68::W](pdac_w1_0_68::W) writer structure"]
impl crate::Writable for PDAC_W1_0_68 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_68;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_69](pdac_w0_0_69) module"]
pub type PDAC_W0_0_69 = crate::Reg<u32, _PDAC_W0_0_69>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_69;
#[doc = "`read()` method returns [pdac_w0_0_69::R](pdac_w0_0_69::R) reader structure"]
impl crate::Readable for PDAC_W0_0_69 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_69::W](pdac_w0_0_69::W) writer structure"]
impl crate::Writable for PDAC_W0_0_69 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_69;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_69](pdac_w1_0_69) module"]
pub type PDAC_W1_0_69 = crate::Reg<u32, _PDAC_W1_0_69>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_69;
#[doc = "`read()` method returns [pdac_w1_0_69::R](pdac_w1_0_69::R) reader structure"]
impl crate::Readable for PDAC_W1_0_69 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_69::W](pdac_w1_0_69::W) writer structure"]
impl crate::Writable for PDAC_W1_0_69 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_69;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_70](pdac_w0_0_70) module"]
pub type PDAC_W0_0_70 = crate::Reg<u32, _PDAC_W0_0_70>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_70;
#[doc = "`read()` method returns [pdac_w0_0_70::R](pdac_w0_0_70::R) reader structure"]
impl crate::Readable for PDAC_W0_0_70 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_70::W](pdac_w0_0_70::W) writer structure"]
impl crate::Writable for PDAC_W0_0_70 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_70;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_70](pdac_w1_0_70) module"]
pub type PDAC_W1_0_70 = crate::Reg<u32, _PDAC_W1_0_70>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_70;
#[doc = "`read()` method returns [pdac_w1_0_70::R](pdac_w1_0_70::R) reader structure"]
impl crate::Readable for PDAC_W1_0_70 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_70::W](pdac_w1_0_70::W) writer structure"]
impl crate::Writable for PDAC_W1_0_70 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_70;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_71](pdac_w0_0_71) module"]
pub type PDAC_W0_0_71 = crate::Reg<u32, _PDAC_W0_0_71>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_71;
#[doc = "`read()` method returns [pdac_w0_0_71::R](pdac_w0_0_71::R) reader structure"]
impl crate::Readable for PDAC_W0_0_71 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_71::W](pdac_w0_0_71::W) writer structure"]
impl crate::Writable for PDAC_W0_0_71 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_71;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_71](pdac_w1_0_71) module"]
pub type PDAC_W1_0_71 = crate::Reg<u32, _PDAC_W1_0_71>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_71;
#[doc = "`read()` method returns [pdac_w1_0_71::R](pdac_w1_0_71::R) reader structure"]
impl crate::Readable for PDAC_W1_0_71 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_71::W](pdac_w1_0_71::W) writer structure"]
impl crate::Writable for PDAC_W1_0_71 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_71;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_72](pdac_w0_0_72) module"]
pub type PDAC_W0_0_72 = crate::Reg<u32, _PDAC_W0_0_72>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_72;
#[doc = "`read()` method returns [pdac_w0_0_72::R](pdac_w0_0_72::R) reader structure"]
impl crate::Readable for PDAC_W0_0_72 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_72::W](pdac_w0_0_72::W) writer structure"]
impl crate::Writable for PDAC_W0_0_72 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_72;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_72](pdac_w1_0_72) module"]
pub type PDAC_W1_0_72 = crate::Reg<u32, _PDAC_W1_0_72>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_72;
#[doc = "`read()` method returns [pdac_w1_0_72::R](pdac_w1_0_72::R) reader structure"]
impl crate::Readable for PDAC_W1_0_72 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_72::W](pdac_w1_0_72::W) writer structure"]
impl crate::Writable for PDAC_W1_0_72 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_72;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_73](pdac_w0_0_73) module"]
pub type PDAC_W0_0_73 = crate::Reg<u32, _PDAC_W0_0_73>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_73;
#[doc = "`read()` method returns [pdac_w0_0_73::R](pdac_w0_0_73::R) reader structure"]
impl crate::Readable for PDAC_W0_0_73 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_73::W](pdac_w0_0_73::W) writer structure"]
impl crate::Writable for PDAC_W0_0_73 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_73;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_73](pdac_w1_0_73) module"]
pub type PDAC_W1_0_73 = crate::Reg<u32, _PDAC_W1_0_73>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_73;
#[doc = "`read()` method returns [pdac_w1_0_73::R](pdac_w1_0_73::R) reader structure"]
impl crate::Readable for PDAC_W1_0_73 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_73::W](pdac_w1_0_73::W) writer structure"]
impl crate::Writable for PDAC_W1_0_73 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_73;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_74](pdac_w0_0_74) module"]
pub type PDAC_W0_0_74 = crate::Reg<u32, _PDAC_W0_0_74>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_74;
#[doc = "`read()` method returns [pdac_w0_0_74::R](pdac_w0_0_74::R) reader structure"]
impl crate::Readable for PDAC_W0_0_74 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_74::W](pdac_w0_0_74::W) writer structure"]
impl crate::Writable for PDAC_W0_0_74 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_74;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_74](pdac_w1_0_74) module"]
pub type PDAC_W1_0_74 = crate::Reg<u32, _PDAC_W1_0_74>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_74;
#[doc = "`read()` method returns [pdac_w1_0_74::R](pdac_w1_0_74::R) reader structure"]
impl crate::Readable for PDAC_W1_0_74 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_74::W](pdac_w1_0_74::W) writer structure"]
impl crate::Writable for PDAC_W1_0_74 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_74;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_75](pdac_w0_0_75) module"]
pub type PDAC_W0_0_75 = crate::Reg<u32, _PDAC_W0_0_75>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_75;
#[doc = "`read()` method returns [pdac_w0_0_75::R](pdac_w0_0_75::R) reader structure"]
impl crate::Readable for PDAC_W0_0_75 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_75::W](pdac_w0_0_75::W) writer structure"]
impl crate::Writable for PDAC_W0_0_75 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_75;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_75](pdac_w1_0_75) module"]
pub type PDAC_W1_0_75 = crate::Reg<u32, _PDAC_W1_0_75>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_75;
#[doc = "`read()` method returns [pdac_w1_0_75::R](pdac_w1_0_75::R) reader structure"]
impl crate::Readable for PDAC_W1_0_75 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_75::W](pdac_w1_0_75::W) writer structure"]
impl crate::Writable for PDAC_W1_0_75 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_75;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_76](pdac_w0_0_76) module"]
pub type PDAC_W0_0_76 = crate::Reg<u32, _PDAC_W0_0_76>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_76;
#[doc = "`read()` method returns [pdac_w0_0_76::R](pdac_w0_0_76::R) reader structure"]
impl crate::Readable for PDAC_W0_0_76 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_76::W](pdac_w0_0_76::W) writer structure"]
impl crate::Writable for PDAC_W0_0_76 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_76;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_76](pdac_w1_0_76) module"]
pub type PDAC_W1_0_76 = crate::Reg<u32, _PDAC_W1_0_76>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_76;
#[doc = "`read()` method returns [pdac_w1_0_76::R](pdac_w1_0_76::R) reader structure"]
impl crate::Readable for PDAC_W1_0_76 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_76::W](pdac_w1_0_76::W) writer structure"]
impl crate::Writable for PDAC_W1_0_76 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_76;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_77](pdac_w0_0_77) module"]
pub type PDAC_W0_0_77 = crate::Reg<u32, _PDAC_W0_0_77>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_77;
#[doc = "`read()` method returns [pdac_w0_0_77::R](pdac_w0_0_77::R) reader structure"]
impl crate::Readable for PDAC_W0_0_77 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_77::W](pdac_w0_0_77::W) writer structure"]
impl crate::Writable for PDAC_W0_0_77 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_77;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_77](pdac_w1_0_77) module"]
pub type PDAC_W1_0_77 = crate::Reg<u32, _PDAC_W1_0_77>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_77;
#[doc = "`read()` method returns [pdac_w1_0_77::R](pdac_w1_0_77::R) reader structure"]
impl crate::Readable for PDAC_W1_0_77 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_77::W](pdac_w1_0_77::W) writer structure"]
impl crate::Writable for PDAC_W1_0_77 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_77;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_78](pdac_w0_0_78) module"]
pub type PDAC_W0_0_78 = crate::Reg<u32, _PDAC_W0_0_78>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_78;
#[doc = "`read()` method returns [pdac_w0_0_78::R](pdac_w0_0_78::R) reader structure"]
impl crate::Readable for PDAC_W0_0_78 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_78::W](pdac_w0_0_78::W) writer structure"]
impl crate::Writable for PDAC_W0_0_78 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_78;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_78](pdac_w1_0_78) module"]
pub type PDAC_W1_0_78 = crate::Reg<u32, _PDAC_W1_0_78>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_78;
#[doc = "`read()` method returns [pdac_w1_0_78::R](pdac_w1_0_78::R) reader structure"]
impl crate::Readable for PDAC_W1_0_78 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_78::W](pdac_w1_0_78::W) writer structure"]
impl crate::Writable for PDAC_W1_0_78 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_78;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_125](pdac_w0_0_125) module"]
pub type PDAC_W0_0_125 = crate::Reg<u32, _PDAC_W0_0_125>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_125;
#[doc = "`read()` method returns [pdac_w0_0_125::R](pdac_w0_0_125::R) reader structure"]
impl crate::Readable for PDAC_W0_0_125 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_125::W](pdac_w0_0_125::W) writer structure"]
impl crate::Writable for PDAC_W0_0_125 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_125;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_125](pdac_w1_0_125) module"]
pub type PDAC_W1_0_125 = crate::Reg<u32, _PDAC_W1_0_125>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_125;
#[doc = "`read()` method returns [pdac_w1_0_125::R](pdac_w1_0_125::R) reader structure"]
impl crate::Readable for PDAC_W1_0_125 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_125::W](pdac_w1_0_125::W) writer structure"]
impl crate::Writable for PDAC_W1_0_125 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_125;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_126](pdac_w0_0_126) module"]
pub type PDAC_W0_0_126 = crate::Reg<u32, _PDAC_W0_0_126>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_126;
#[doc = "`read()` method returns [pdac_w0_0_126::R](pdac_w0_0_126::R) reader structure"]
impl crate::Readable for PDAC_W0_0_126 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_126::W](pdac_w0_0_126::W) writer structure"]
impl crate::Writable for PDAC_W0_0_126 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_126;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_126](pdac_w1_0_126) module"]
pub type PDAC_W1_0_126 = crate::Reg<u32, _PDAC_W1_0_126>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_126;
#[doc = "`read()` method returns [pdac_w1_0_126::R](pdac_w1_0_126::R) reader structure"]
impl crate::Readable for PDAC_W1_0_126 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_126::W](pdac_w1_0_126::W) writer structure"]
impl crate::Writable for PDAC_W1_0_126 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_126;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_0_127](pdac_w0_0_127) module"]
pub type PDAC_W0_0_127 = crate::Reg<u32, _PDAC_W0_0_127>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_0_127;
#[doc = "`read()` method returns [pdac_w0_0_127::R](pdac_w0_0_127::R) reader structure"]
impl crate::Readable for PDAC_W0_0_127 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_0_127::W](pdac_w0_0_127::W) writer structure"]
impl crate::Writable for PDAC_W0_0_127 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_0_127;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_0_127](pdac_w1_0_127) module"]
pub type PDAC_W1_0_127 = crate::Reg<u32, _PDAC_W1_0_127>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_0_127;
#[doc = "`read()` method returns [pdac_w1_0_127::R](pdac_w1_0_127::R) reader structure"]
impl crate::Readable for PDAC_W1_0_127 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_0_127::W](pdac_w1_0_127::W) writer structure"]
impl crate::Writable for PDAC_W1_0_127 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_0_127;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_8](pdac_w0_1_8) module"]
pub type PDAC_W0_1_8 = crate::Reg<u32, _PDAC_W0_1_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_8;
#[doc = "`read()` method returns [pdac_w0_1_8::R](pdac_w0_1_8::R) reader structure"]
impl crate::Readable for PDAC_W0_1_8 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_8::W](pdac_w0_1_8::W) writer structure"]
impl crate::Writable for PDAC_W0_1_8 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_8;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_8](pdac_w1_1_8) module"]
pub type PDAC_W1_1_8 = crate::Reg<u32, _PDAC_W1_1_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_8;
#[doc = "`read()` method returns [pdac_w1_1_8::R](pdac_w1_1_8::R) reader structure"]
impl crate::Readable for PDAC_W1_1_8 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_8::W](pdac_w1_1_8::W) writer structure"]
impl crate::Writable for PDAC_W1_1_8 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_8;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_9](pdac_w0_1_9) module"]
pub type PDAC_W0_1_9 = crate::Reg<u32, _PDAC_W0_1_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_9;
#[doc = "`read()` method returns [pdac_w0_1_9::R](pdac_w0_1_9::R) reader structure"]
impl crate::Readable for PDAC_W0_1_9 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_9::W](pdac_w0_1_9::W) writer structure"]
impl crate::Writable for PDAC_W0_1_9 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_9;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_9](pdac_w1_1_9) module"]
pub type PDAC_W1_1_9 = crate::Reg<u32, _PDAC_W1_1_9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_9;
#[doc = "`read()` method returns [pdac_w1_1_9::R](pdac_w1_1_9::R) reader structure"]
impl crate::Readable for PDAC_W1_1_9 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_9::W](pdac_w1_1_9::W) writer structure"]
impl crate::Writable for PDAC_W1_1_9 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_9;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_15](pdac_w0_1_15) module"]
pub type PDAC_W0_1_15 = crate::Reg<u32, _PDAC_W0_1_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_15;
#[doc = "`read()` method returns [pdac_w0_1_15::R](pdac_w0_1_15::R) reader structure"]
impl crate::Readable for PDAC_W0_1_15 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_15::W](pdac_w0_1_15::W) writer structure"]
impl crate::Writable for PDAC_W0_1_15 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_15;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_15](pdac_w1_1_15) module"]
pub type PDAC_W1_1_15 = crate::Reg<u32, _PDAC_W1_1_15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_15;
#[doc = "`read()` method returns [pdac_w1_1_15::R](pdac_w1_1_15::R) reader structure"]
impl crate::Readable for PDAC_W1_1_15 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_15::W](pdac_w1_1_15::W) writer structure"]
impl crate::Writable for PDAC_W1_1_15 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_15;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_27](pdac_w0_1_27) module"]
pub type PDAC_W0_1_27 = crate::Reg<u32, _PDAC_W0_1_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_27;
#[doc = "`read()` method returns [pdac_w0_1_27::R](pdac_w0_1_27::R) reader structure"]
impl crate::Readable for PDAC_W0_1_27 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_27::W](pdac_w0_1_27::W) writer structure"]
impl crate::Writable for PDAC_W0_1_27 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_27;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_27](pdac_w1_1_27) module"]
pub type PDAC_W1_1_27 = crate::Reg<u32, _PDAC_W1_1_27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_27;
#[doc = "`read()` method returns [pdac_w1_1_27::R](pdac_w1_1_27::R) reader structure"]
impl crate::Readable for PDAC_W1_1_27 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_27::W](pdac_w1_1_27::W) writer structure"]
impl crate::Writable for PDAC_W1_1_27 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_27;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_32](pdac_w0_1_32) module"]
pub type PDAC_W0_1_32 = crate::Reg<u32, _PDAC_W0_1_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_32;
#[doc = "`read()` method returns [pdac_w0_1_32::R](pdac_w0_1_32::R) reader structure"]
impl crate::Readable for PDAC_W0_1_32 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_32::W](pdac_w0_1_32::W) writer structure"]
impl crate::Writable for PDAC_W0_1_32 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_32;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_32](pdac_w1_1_32) module"]
pub type PDAC_W1_1_32 = crate::Reg<u32, _PDAC_W1_1_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_32;
#[doc = "`read()` method returns [pdac_w1_1_32::R](pdac_w1_1_32::R) reader structure"]
impl crate::Readable for PDAC_W1_1_32 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_32::W](pdac_w1_1_32::W) writer structure"]
impl crate::Writable for PDAC_W1_1_32 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_32;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_33](pdac_w0_1_33) module"]
pub type PDAC_W0_1_33 = crate::Reg<u32, _PDAC_W0_1_33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_33;
#[doc = "`read()` method returns [pdac_w0_1_33::R](pdac_w0_1_33::R) reader structure"]
impl crate::Readable for PDAC_W0_1_33 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_33::W](pdac_w0_1_33::W) writer structure"]
impl crate::Writable for PDAC_W0_1_33 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_33;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_33](pdac_w1_1_33) module"]
pub type PDAC_W1_1_33 = crate::Reg<u32, _PDAC_W1_1_33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_33;
#[doc = "`read()` method returns [pdac_w1_1_33::R](pdac_w1_1_33::R) reader structure"]
impl crate::Readable for PDAC_W1_1_33 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_33::W](pdac_w1_1_33::W) writer structure"]
impl crate::Writable for PDAC_W1_1_33 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_33;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_34](pdac_w0_1_34) module"]
pub type PDAC_W0_1_34 = crate::Reg<u32, _PDAC_W0_1_34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_34;
#[doc = "`read()` method returns [pdac_w0_1_34::R](pdac_w0_1_34::R) reader structure"]
impl crate::Readable for PDAC_W0_1_34 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_34::W](pdac_w0_1_34::W) writer structure"]
impl crate::Writable for PDAC_W0_1_34 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_34;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_34](pdac_w1_1_34) module"]
pub type PDAC_W1_1_34 = crate::Reg<u32, _PDAC_W1_1_34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_34;
#[doc = "`read()` method returns [pdac_w1_1_34::R](pdac_w1_1_34::R) reader structure"]
impl crate::Readable for PDAC_W1_1_34 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_34::W](pdac_w1_1_34::W) writer structure"]
impl crate::Writable for PDAC_W1_1_34 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_34;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_35](pdac_w0_1_35) module"]
pub type PDAC_W0_1_35 = crate::Reg<u32, _PDAC_W0_1_35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_35;
#[doc = "`read()` method returns [pdac_w0_1_35::R](pdac_w0_1_35::R) reader structure"]
impl crate::Readable for PDAC_W0_1_35 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_35::W](pdac_w0_1_35::W) writer structure"]
impl crate::Writable for PDAC_W0_1_35 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_35;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_35](pdac_w1_1_35) module"]
pub type PDAC_W1_1_35 = crate::Reg<u32, _PDAC_W1_1_35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_35;
#[doc = "`read()` method returns [pdac_w1_1_35::R](pdac_w1_1_35::R) reader structure"]
impl crate::Readable for PDAC_W1_1_35 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_35::W](pdac_w1_1_35::W) writer structure"]
impl crate::Writable for PDAC_W1_1_35 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_35;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_36](pdac_w0_1_36) module"]
pub type PDAC_W0_1_36 = crate::Reg<u32, _PDAC_W0_1_36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_36;
#[doc = "`read()` method returns [pdac_w0_1_36::R](pdac_w0_1_36::R) reader structure"]
impl crate::Readable for PDAC_W0_1_36 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_36::W](pdac_w0_1_36::W) writer structure"]
impl crate::Writable for PDAC_W0_1_36 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_36;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_36](pdac_w1_1_36) module"]
pub type PDAC_W1_1_36 = crate::Reg<u32, _PDAC_W1_1_36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_36;
#[doc = "`read()` method returns [pdac_w1_1_36::R](pdac_w1_1_36::R) reader structure"]
impl crate::Readable for PDAC_W1_1_36 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_36::W](pdac_w1_1_36::W) writer structure"]
impl crate::Writable for PDAC_W1_1_36 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_36;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_37](pdac_w0_1_37) module"]
pub type PDAC_W0_1_37 = crate::Reg<u32, _PDAC_W0_1_37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_37;
#[doc = "`read()` method returns [pdac_w0_1_37::R](pdac_w0_1_37::R) reader structure"]
impl crate::Readable for PDAC_W0_1_37 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_37::W](pdac_w0_1_37::W) writer structure"]
impl crate::Writable for PDAC_W0_1_37 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_37;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_37](pdac_w1_1_37) module"]
pub type PDAC_W1_1_37 = crate::Reg<u32, _PDAC_W1_1_37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_37;
#[doc = "`read()` method returns [pdac_w1_1_37::R](pdac_w1_1_37::R) reader structure"]
impl crate::Readable for PDAC_W1_1_37 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_37::W](pdac_w1_1_37::W) writer structure"]
impl crate::Writable for PDAC_W1_1_37 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_37;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_38](pdac_w0_1_38) module"]
pub type PDAC_W0_1_38 = crate::Reg<u32, _PDAC_W0_1_38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_38;
#[doc = "`read()` method returns [pdac_w0_1_38::R](pdac_w0_1_38::R) reader structure"]
impl crate::Readable for PDAC_W0_1_38 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_38::W](pdac_w0_1_38::W) writer structure"]
impl crate::Writable for PDAC_W0_1_38 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_38;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_38](pdac_w1_1_38) module"]
pub type PDAC_W1_1_38 = crate::Reg<u32, _PDAC_W1_1_38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_38;
#[doc = "`read()` method returns [pdac_w1_1_38::R](pdac_w1_1_38::R) reader structure"]
impl crate::Readable for PDAC_W1_1_38 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_38::W](pdac_w1_1_38::W) writer structure"]
impl crate::Writable for PDAC_W1_1_38 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_38;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_39](pdac_w0_1_39) module"]
pub type PDAC_W0_1_39 = crate::Reg<u32, _PDAC_W0_1_39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_39;
#[doc = "`read()` method returns [pdac_w0_1_39::R](pdac_w0_1_39::R) reader structure"]
impl crate::Readable for PDAC_W0_1_39 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_39::W](pdac_w0_1_39::W) writer structure"]
impl crate::Writable for PDAC_W0_1_39 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_39;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_39](pdac_w1_1_39) module"]
pub type PDAC_W1_1_39 = crate::Reg<u32, _PDAC_W1_1_39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_39;
#[doc = "`read()` method returns [pdac_w1_1_39::R](pdac_w1_1_39::R) reader structure"]
impl crate::Readable for PDAC_W1_1_39 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_39::W](pdac_w1_1_39::W) writer structure"]
impl crate::Writable for PDAC_W1_1_39 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_39;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_40](pdac_w0_1_40) module"]
pub type PDAC_W0_1_40 = crate::Reg<u32, _PDAC_W0_1_40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_40;
#[doc = "`read()` method returns [pdac_w0_1_40::R](pdac_w0_1_40::R) reader structure"]
impl crate::Readable for PDAC_W0_1_40 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_40::W](pdac_w0_1_40::W) writer structure"]
impl crate::Writable for PDAC_W0_1_40 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_40;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_40](pdac_w1_1_40) module"]
pub type PDAC_W1_1_40 = crate::Reg<u32, _PDAC_W1_1_40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_40;
#[doc = "`read()` method returns [pdac_w1_1_40::R](pdac_w1_1_40::R) reader structure"]
impl crate::Readable for PDAC_W1_1_40 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_40::W](pdac_w1_1_40::W) writer structure"]
impl crate::Writable for PDAC_W1_1_40 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_40;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_41](pdac_w0_1_41) module"]
pub type PDAC_W0_1_41 = crate::Reg<u32, _PDAC_W0_1_41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_41;
#[doc = "`read()` method returns [pdac_w0_1_41::R](pdac_w0_1_41::R) reader structure"]
impl crate::Readable for PDAC_W0_1_41 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_41::W](pdac_w0_1_41::W) writer structure"]
impl crate::Writable for PDAC_W0_1_41 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_41;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_41](pdac_w1_1_41) module"]
pub type PDAC_W1_1_41 = crate::Reg<u32, _PDAC_W1_1_41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_41;
#[doc = "`read()` method returns [pdac_w1_1_41::R](pdac_w1_1_41::R) reader structure"]
impl crate::Readable for PDAC_W1_1_41 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_41::W](pdac_w1_1_41::W) writer structure"]
impl crate::Writable for PDAC_W1_1_41 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_41;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_42](pdac_w0_1_42) module"]
pub type PDAC_W0_1_42 = crate::Reg<u32, _PDAC_W0_1_42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_42;
#[doc = "`read()` method returns [pdac_w0_1_42::R](pdac_w0_1_42::R) reader structure"]
impl crate::Readable for PDAC_W0_1_42 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_42::W](pdac_w0_1_42::W) writer structure"]
impl crate::Writable for PDAC_W0_1_42 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_42;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_42](pdac_w1_1_42) module"]
pub type PDAC_W1_1_42 = crate::Reg<u32, _PDAC_W1_1_42>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_42;
#[doc = "`read()` method returns [pdac_w1_1_42::R](pdac_w1_1_42::R) reader structure"]
impl crate::Readable for PDAC_W1_1_42 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_42::W](pdac_w1_1_42::W) writer structure"]
impl crate::Writable for PDAC_W1_1_42 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_42;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_43](pdac_w0_1_43) module"]
pub type PDAC_W0_1_43 = crate::Reg<u32, _PDAC_W0_1_43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_43;
#[doc = "`read()` method returns [pdac_w0_1_43::R](pdac_w0_1_43::R) reader structure"]
impl crate::Readable for PDAC_W0_1_43 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_43::W](pdac_w0_1_43::W) writer structure"]
impl crate::Writable for PDAC_W0_1_43 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_43;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_43](pdac_w1_1_43) module"]
pub type PDAC_W1_1_43 = crate::Reg<u32, _PDAC_W1_1_43>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_43;
#[doc = "`read()` method returns [pdac_w1_1_43::R](pdac_w1_1_43::R) reader structure"]
impl crate::Readable for PDAC_W1_1_43 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_43::W](pdac_w1_1_43::W) writer structure"]
impl crate::Writable for PDAC_W1_1_43 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_43;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_44](pdac_w0_1_44) module"]
pub type PDAC_W0_1_44 = crate::Reg<u32, _PDAC_W0_1_44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_44;
#[doc = "`read()` method returns [pdac_w0_1_44::R](pdac_w0_1_44::R) reader structure"]
impl crate::Readable for PDAC_W0_1_44 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_44::W](pdac_w0_1_44::W) writer structure"]
impl crate::Writable for PDAC_W0_1_44 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_44;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_44](pdac_w1_1_44) module"]
pub type PDAC_W1_1_44 = crate::Reg<u32, _PDAC_W1_1_44>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_44;
#[doc = "`read()` method returns [pdac_w1_1_44::R](pdac_w1_1_44::R) reader structure"]
impl crate::Readable for PDAC_W1_1_44 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_44::W](pdac_w1_1_44::W) writer structure"]
impl crate::Writable for PDAC_W1_1_44 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_44;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_45](pdac_w0_1_45) module"]
pub type PDAC_W0_1_45 = crate::Reg<u32, _PDAC_W0_1_45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_45;
#[doc = "`read()` method returns [pdac_w0_1_45::R](pdac_w0_1_45::R) reader structure"]
impl crate::Readable for PDAC_W0_1_45 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_45::W](pdac_w0_1_45::W) writer structure"]
impl crate::Writable for PDAC_W0_1_45 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_45;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_45](pdac_w1_1_45) module"]
pub type PDAC_W1_1_45 = crate::Reg<u32, _PDAC_W1_1_45>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_45;
#[doc = "`read()` method returns [pdac_w1_1_45::R](pdac_w1_1_45::R) reader structure"]
impl crate::Readable for PDAC_W1_1_45 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_45::W](pdac_w1_1_45::W) writer structure"]
impl crate::Writable for PDAC_W1_1_45 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_45;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_46](pdac_w0_1_46) module"]
pub type PDAC_W0_1_46 = crate::Reg<u32, _PDAC_W0_1_46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_46;
#[doc = "`read()` method returns [pdac_w0_1_46::R](pdac_w0_1_46::R) reader structure"]
impl crate::Readable for PDAC_W0_1_46 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_46::W](pdac_w0_1_46::W) writer structure"]
impl crate::Writable for PDAC_W0_1_46 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_46;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_46](pdac_w1_1_46) module"]
pub type PDAC_W1_1_46 = crate::Reg<u32, _PDAC_W1_1_46>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_46;
#[doc = "`read()` method returns [pdac_w1_1_46::R](pdac_w1_1_46::R) reader structure"]
impl crate::Readable for PDAC_W1_1_46 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_46::W](pdac_w1_1_46::W) writer structure"]
impl crate::Writable for PDAC_W1_1_46 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_46;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_47](pdac_w0_1_47) module"]
pub type PDAC_W0_1_47 = crate::Reg<u32, _PDAC_W0_1_47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_47;
#[doc = "`read()` method returns [pdac_w0_1_47::R](pdac_w0_1_47::R) reader structure"]
impl crate::Readable for PDAC_W0_1_47 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_47::W](pdac_w0_1_47::W) writer structure"]
impl crate::Writable for PDAC_W0_1_47 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_47;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_47](pdac_w1_1_47) module"]
pub type PDAC_W1_1_47 = crate::Reg<u32, _PDAC_W1_1_47>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_47;
#[doc = "`read()` method returns [pdac_w1_1_47::R](pdac_w1_1_47::R) reader structure"]
impl crate::Readable for PDAC_W1_1_47 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_47::W](pdac_w1_1_47::W) writer structure"]
impl crate::Writable for PDAC_W1_1_47 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_47;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_48](pdac_w0_1_48) module"]
pub type PDAC_W0_1_48 = crate::Reg<u32, _PDAC_W0_1_48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_48;
#[doc = "`read()` method returns [pdac_w0_1_48::R](pdac_w0_1_48::R) reader structure"]
impl crate::Readable for PDAC_W0_1_48 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_48::W](pdac_w0_1_48::W) writer structure"]
impl crate::Writable for PDAC_W0_1_48 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_48;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_48](pdac_w1_1_48) module"]
pub type PDAC_W1_1_48 = crate::Reg<u32, _PDAC_W1_1_48>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_48;
#[doc = "`read()` method returns [pdac_w1_1_48::R](pdac_w1_1_48::R) reader structure"]
impl crate::Readable for PDAC_W1_1_48 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_48::W](pdac_w1_1_48::W) writer structure"]
impl crate::Writable for PDAC_W1_1_48 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_48;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_49](pdac_w0_1_49) module"]
pub type PDAC_W0_1_49 = crate::Reg<u32, _PDAC_W0_1_49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_49;
#[doc = "`read()` method returns [pdac_w0_1_49::R](pdac_w0_1_49::R) reader structure"]
impl crate::Readable for PDAC_W0_1_49 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_49::W](pdac_w0_1_49::W) writer structure"]
impl crate::Writable for PDAC_W0_1_49 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_49;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_49](pdac_w1_1_49) module"]
pub type PDAC_W1_1_49 = crate::Reg<u32, _PDAC_W1_1_49>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_49;
#[doc = "`read()` method returns [pdac_w1_1_49::R](pdac_w1_1_49::R) reader structure"]
impl crate::Readable for PDAC_W1_1_49 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_49::W](pdac_w1_1_49::W) writer structure"]
impl crate::Writable for PDAC_W1_1_49 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_49;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_50](pdac_w0_1_50) module"]
pub type PDAC_W0_1_50 = crate::Reg<u32, _PDAC_W0_1_50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_50;
#[doc = "`read()` method returns [pdac_w0_1_50::R](pdac_w0_1_50::R) reader structure"]
impl crate::Readable for PDAC_W0_1_50 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_50::W](pdac_w0_1_50::W) writer structure"]
impl crate::Writable for PDAC_W0_1_50 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_50;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_50](pdac_w1_1_50) module"]
pub type PDAC_W1_1_50 = crate::Reg<u32, _PDAC_W1_1_50>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_50;
#[doc = "`read()` method returns [pdac_w1_1_50::R](pdac_w1_1_50::R) reader structure"]
impl crate::Readable for PDAC_W1_1_50 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_50::W](pdac_w1_1_50::W) writer structure"]
impl crate::Writable for PDAC_W1_1_50 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_50;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_51](pdac_w0_1_51) module"]
pub type PDAC_W0_1_51 = crate::Reg<u32, _PDAC_W0_1_51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_51;
#[doc = "`read()` method returns [pdac_w0_1_51::R](pdac_w0_1_51::R) reader structure"]
impl crate::Readable for PDAC_W0_1_51 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_51::W](pdac_w0_1_51::W) writer structure"]
impl crate::Writable for PDAC_W0_1_51 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_51;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_51](pdac_w1_1_51) module"]
pub type PDAC_W1_1_51 = crate::Reg<u32, _PDAC_W1_1_51>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_51;
#[doc = "`read()` method returns [pdac_w1_1_51::R](pdac_w1_1_51::R) reader structure"]
impl crate::Readable for PDAC_W1_1_51 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_51::W](pdac_w1_1_51::W) writer structure"]
impl crate::Writable for PDAC_W1_1_51 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_51;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_52](pdac_w0_1_52) module"]
pub type PDAC_W0_1_52 = crate::Reg<u32, _PDAC_W0_1_52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_52;
#[doc = "`read()` method returns [pdac_w0_1_52::R](pdac_w0_1_52::R) reader structure"]
impl crate::Readable for PDAC_W0_1_52 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_52::W](pdac_w0_1_52::W) writer structure"]
impl crate::Writable for PDAC_W0_1_52 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_52;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_52](pdac_w1_1_52) module"]
pub type PDAC_W1_1_52 = crate::Reg<u32, _PDAC_W1_1_52>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_52;
#[doc = "`read()` method returns [pdac_w1_1_52::R](pdac_w1_1_52::R) reader structure"]
impl crate::Readable for PDAC_W1_1_52 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_52::W](pdac_w1_1_52::W) writer structure"]
impl crate::Writable for PDAC_W1_1_52 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_52;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_53](pdac_w0_1_53) module"]
pub type PDAC_W0_1_53 = crate::Reg<u32, _PDAC_W0_1_53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_53;
#[doc = "`read()` method returns [pdac_w0_1_53::R](pdac_w0_1_53::R) reader structure"]
impl crate::Readable for PDAC_W0_1_53 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_53::W](pdac_w0_1_53::W) writer structure"]
impl crate::Writable for PDAC_W0_1_53 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_53;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_53](pdac_w1_1_53) module"]
pub type PDAC_W1_1_53 = crate::Reg<u32, _PDAC_W1_1_53>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_53;
#[doc = "`read()` method returns [pdac_w1_1_53::R](pdac_w1_1_53::R) reader structure"]
impl crate::Readable for PDAC_W1_1_53 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_53::W](pdac_w1_1_53::W) writer structure"]
impl crate::Writable for PDAC_W1_1_53 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_53;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_54](pdac_w0_1_54) module"]
pub type PDAC_W0_1_54 = crate::Reg<u32, _PDAC_W0_1_54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_54;
#[doc = "`read()` method returns [pdac_w0_1_54::R](pdac_w0_1_54::R) reader structure"]
impl crate::Readable for PDAC_W0_1_54 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_54::W](pdac_w0_1_54::W) writer structure"]
impl crate::Writable for PDAC_W0_1_54 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_54;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_54](pdac_w1_1_54) module"]
pub type PDAC_W1_1_54 = crate::Reg<u32, _PDAC_W1_1_54>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_54;
#[doc = "`read()` method returns [pdac_w1_1_54::R](pdac_w1_1_54::R) reader structure"]
impl crate::Readable for PDAC_W1_1_54 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_54::W](pdac_w1_1_54::W) writer structure"]
impl crate::Writable for PDAC_W1_1_54 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_54;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_55](pdac_w0_1_55) module"]
pub type PDAC_W0_1_55 = crate::Reg<u32, _PDAC_W0_1_55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_55;
#[doc = "`read()` method returns [pdac_w0_1_55::R](pdac_w0_1_55::R) reader structure"]
impl crate::Readable for PDAC_W0_1_55 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_55::W](pdac_w0_1_55::W) writer structure"]
impl crate::Writable for PDAC_W0_1_55 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_55;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_55](pdac_w1_1_55) module"]
pub type PDAC_W1_1_55 = crate::Reg<u32, _PDAC_W1_1_55>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_55;
#[doc = "`read()` method returns [pdac_w1_1_55::R](pdac_w1_1_55::R) reader structure"]
impl crate::Readable for PDAC_W1_1_55 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_55::W](pdac_w1_1_55::W) writer structure"]
impl crate::Writable for PDAC_W1_1_55 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_55;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_1_56](pdac_w0_1_56) module"]
pub type PDAC_W0_1_56 = crate::Reg<u32, _PDAC_W0_1_56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_1_56;
#[doc = "`read()` method returns [pdac_w0_1_56::R](pdac_w0_1_56::R) reader structure"]
impl crate::Readable for PDAC_W0_1_56 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_1_56::W](pdac_w0_1_56::W) writer structure"]
impl crate::Writable for PDAC_W0_1_56 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_1_56;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_1_56](pdac_w1_1_56) module"]
pub type PDAC_W1_1_56 = crate::Reg<u32, _PDAC_W1_1_56>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_1_56;
#[doc = "`read()` method returns [pdac_w1_1_56::R](pdac_w1_1_56::R) reader structure"]
impl crate::Readable for PDAC_W1_1_56 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_1_56::W](pdac_w1_1_56::W) writer structure"]
impl crate::Writable for PDAC_W1_1_56 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_1_56;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_2_16](pdac_w0_2_16) module"]
pub type PDAC_W0_2_16 = crate::Reg<u32, _PDAC_W0_2_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_2_16;
#[doc = "`read()` method returns [pdac_w0_2_16::R](pdac_w0_2_16::R) reader structure"]
impl crate::Readable for PDAC_W0_2_16 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_2_16::W](pdac_w0_2_16::W) writer structure"]
impl crate::Writable for PDAC_W0_2_16 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_2_16;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_2_16](pdac_w1_2_16) module"]
pub type PDAC_W1_2_16 = crate::Reg<u32, _PDAC_W1_2_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_2_16;
#[doc = "`read()` method returns [pdac_w1_2_16::R](pdac_w1_2_16::R) reader structure"]
impl crate::Readable for PDAC_W1_2_16 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_2_16::W](pdac_w1_2_16::W) writer structure"]
impl crate::Writable for PDAC_W1_2_16 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_2_16;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w0_2_32](pdac_w0_2_32) module"]
pub type PDAC_W0_2_32 = crate::Reg<u32, _PDAC_W0_2_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W0_2_32;
#[doc = "`read()` method returns [pdac_w0_2_32::R](pdac_w0_2_32::R) reader structure"]
impl crate::Readable for PDAC_W0_2_32 {}
#[doc = "`write(|w| ..)` method takes [pdac_w0_2_32::W](pdac_w0_2_32::W) writer structure"]
impl crate::Writable for PDAC_W0_2_32 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w0_2_32;
#[doc = "Peripheral Domain Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pdac_w1_2_32](pdac_w1_2_32) module"]
pub type PDAC_W1_2_32 = crate::Reg<u32, _PDAC_W1_2_32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDAC_W1_2_32;
#[doc = "`read()` method returns [pdac_w1_2_32::R](pdac_w1_2_32::R) reader structure"]
impl crate::Readable for PDAC_W1_2_32 {}
#[doc = "`write(|w| ..)` method takes [pdac_w1_2_32::W](pdac_w1_2_32::W) writer structure"]
impl crate::Writable for PDAC_W1_2_32 {}
#[doc = "Peripheral Domain Access Control"]
pub mod pdac_w1_2_32;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w0_0_0](mrgd_w0_0_0) module"]
pub type MRGD_W0_0_0 = crate::Reg<u32, _MRGD_W0_0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W0_0_0;
#[doc = "`read()` method returns [mrgd_w0_0_0::R](mrgd_w0_0_0::R) reader structure"]
impl crate::Readable for MRGD_W0_0_0 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w0_0_0::W](mrgd_w0_0_0::W) writer structure"]
impl crate::Writable for MRGD_W0_0_0 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w0_0_0;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w1_0_0](mrgd_w1_0_0) module"]
pub type MRGD_W1_0_0 = crate::Reg<u32, _MRGD_W1_0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W1_0_0;
#[doc = "`read()` method returns [mrgd_w1_0_0::R](mrgd_w1_0_0::R) reader structure"]
impl crate::Readable for MRGD_W1_0_0 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w1_0_0::W](mrgd_w1_0_0::W) writer structure"]
impl crate::Writable for MRGD_W1_0_0 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w1_0_0;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w2_0_0](mrgd_w2_0_0) module"]
pub type MRGD_W2_0_0 = crate::Reg<u32, _MRGD_W2_0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W2_0_0;
#[doc = "`read()` method returns [mrgd_w2_0_0::R](mrgd_w2_0_0::R) reader structure"]
impl crate::Readable for MRGD_W2_0_0 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w2_0_0::W](mrgd_w2_0_0::W) writer structure"]
impl crate::Writable for MRGD_W2_0_0 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w2_0_0;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w3_0_0](mrgd_w3_0_0) module"]
pub type MRGD_W3_0_0 = crate::Reg<u32, _MRGD_W3_0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W3_0_0;
#[doc = "`read()` method returns [mrgd_w3_0_0::R](mrgd_w3_0_0::R) reader structure"]
impl crate::Readable for MRGD_W3_0_0 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w3_0_0::W](mrgd_w3_0_0::W) writer structure"]
impl crate::Writable for MRGD_W3_0_0 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w3_0_0;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w4_0_0](mrgd_w4_0_0) module"]
pub type MRGD_W4_0_0 = crate::Reg<u32, _MRGD_W4_0_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W4_0_0;
#[doc = "`read()` method returns [mrgd_w4_0_0::R](mrgd_w4_0_0::R) reader structure"]
impl crate::Readable for MRGD_W4_0_0 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w4_0_0::W](mrgd_w4_0_0::W) writer structure"]
impl crate::Writable for MRGD_W4_0_0 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w4_0_0;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w0_0_1](mrgd_w0_0_1) module"]
pub type MRGD_W0_0_1 = crate::Reg<u32, _MRGD_W0_0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W0_0_1;
#[doc = "`read()` method returns [mrgd_w0_0_1::R](mrgd_w0_0_1::R) reader structure"]
impl crate::Readable for MRGD_W0_0_1 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w0_0_1::W](mrgd_w0_0_1::W) writer structure"]
impl crate::Writable for MRGD_W0_0_1 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w0_0_1;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w1_0_1](mrgd_w1_0_1) module"]
pub type MRGD_W1_0_1 = crate::Reg<u32, _MRGD_W1_0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W1_0_1;
#[doc = "`read()` method returns [mrgd_w1_0_1::R](mrgd_w1_0_1::R) reader structure"]
impl crate::Readable for MRGD_W1_0_1 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w1_0_1::W](mrgd_w1_0_1::W) writer structure"]
impl crate::Writable for MRGD_W1_0_1 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w1_0_1;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w2_0_1](mrgd_w2_0_1) module"]
pub type MRGD_W2_0_1 = crate::Reg<u32, _MRGD_W2_0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W2_0_1;
#[doc = "`read()` method returns [mrgd_w2_0_1::R](mrgd_w2_0_1::R) reader structure"]
impl crate::Readable for MRGD_W2_0_1 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w2_0_1::W](mrgd_w2_0_1::W) writer structure"]
impl crate::Writable for MRGD_W2_0_1 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w2_0_1;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w3_0_1](mrgd_w3_0_1) module"]
pub type MRGD_W3_0_1 = crate::Reg<u32, _MRGD_W3_0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W3_0_1;
#[doc = "`read()` method returns [mrgd_w3_0_1::R](mrgd_w3_0_1::R) reader structure"]
impl crate::Readable for MRGD_W3_0_1 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w3_0_1::W](mrgd_w3_0_1::W) writer structure"]
impl crate::Writable for MRGD_W3_0_1 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w3_0_1;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w4_0_1](mrgd_w4_0_1) module"]
pub type MRGD_W4_0_1 = crate::Reg<u32, _MRGD_W4_0_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W4_0_1;
#[doc = "`read()` method returns [mrgd_w4_0_1::R](mrgd_w4_0_1::R) reader structure"]
impl crate::Readable for MRGD_W4_0_1 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w4_0_1::W](mrgd_w4_0_1::W) writer structure"]
impl crate::Writable for MRGD_W4_0_1 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w4_0_1;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w0_0_2](mrgd_w0_0_2) module"]
pub type MRGD_W0_0_2 = crate::Reg<u32, _MRGD_W0_0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W0_0_2;
#[doc = "`read()` method returns [mrgd_w0_0_2::R](mrgd_w0_0_2::R) reader structure"]
impl crate::Readable for MRGD_W0_0_2 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w0_0_2::W](mrgd_w0_0_2::W) writer structure"]
impl crate::Writable for MRGD_W0_0_2 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w0_0_2;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w1_0_2](mrgd_w1_0_2) module"]
pub type MRGD_W1_0_2 = crate::Reg<u32, _MRGD_W1_0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W1_0_2;
#[doc = "`read()` method returns [mrgd_w1_0_2::R](mrgd_w1_0_2::R) reader structure"]
impl crate::Readable for MRGD_W1_0_2 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w1_0_2::W](mrgd_w1_0_2::W) writer structure"]
impl crate::Writable for MRGD_W1_0_2 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w1_0_2;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w2_0_2](mrgd_w2_0_2) module"]
pub type MRGD_W2_0_2 = crate::Reg<u32, _MRGD_W2_0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W2_0_2;
#[doc = "`read()` method returns [mrgd_w2_0_2::R](mrgd_w2_0_2::R) reader structure"]
impl crate::Readable for MRGD_W2_0_2 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w2_0_2::W](mrgd_w2_0_2::W) writer structure"]
impl crate::Writable for MRGD_W2_0_2 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w2_0_2;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w3_0_2](mrgd_w3_0_2) module"]
pub type MRGD_W3_0_2 = crate::Reg<u32, _MRGD_W3_0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W3_0_2;
#[doc = "`read()` method returns [mrgd_w3_0_2::R](mrgd_w3_0_2::R) reader structure"]
impl crate::Readable for MRGD_W3_0_2 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w3_0_2::W](mrgd_w3_0_2::W) writer structure"]
impl crate::Writable for MRGD_W3_0_2 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w3_0_2;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w4_0_2](mrgd_w4_0_2) module"]
pub type MRGD_W4_0_2 = crate::Reg<u32, _MRGD_W4_0_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W4_0_2;
#[doc = "`read()` method returns [mrgd_w4_0_2::R](mrgd_w4_0_2::R) reader structure"]
impl crate::Readable for MRGD_W4_0_2 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w4_0_2::W](mrgd_w4_0_2::W) writer structure"]
impl crate::Writable for MRGD_W4_0_2 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w4_0_2;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w0_0_3](mrgd_w0_0_3) module"]
pub type MRGD_W0_0_3 = crate::Reg<u32, _MRGD_W0_0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W0_0_3;
#[doc = "`read()` method returns [mrgd_w0_0_3::R](mrgd_w0_0_3::R) reader structure"]
impl crate::Readable for MRGD_W0_0_3 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w0_0_3::W](mrgd_w0_0_3::W) writer structure"]
impl crate::Writable for MRGD_W0_0_3 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w0_0_3;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w1_0_3](mrgd_w1_0_3) module"]
pub type MRGD_W1_0_3 = crate::Reg<u32, _MRGD_W1_0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W1_0_3;
#[doc = "`read()` method returns [mrgd_w1_0_3::R](mrgd_w1_0_3::R) reader structure"]
impl crate::Readable for MRGD_W1_0_3 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w1_0_3::W](mrgd_w1_0_3::W) writer structure"]
impl crate::Writable for MRGD_W1_0_3 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w1_0_3;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w2_0_3](mrgd_w2_0_3) module"]
pub type MRGD_W2_0_3 = crate::Reg<u32, _MRGD_W2_0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W2_0_3;
#[doc = "`read()` method returns [mrgd_w2_0_3::R](mrgd_w2_0_3::R) reader structure"]
impl crate::Readable for MRGD_W2_0_3 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w2_0_3::W](mrgd_w2_0_3::W) writer structure"]
impl crate::Writable for MRGD_W2_0_3 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w2_0_3;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w3_0_3](mrgd_w3_0_3) module"]
pub type MRGD_W3_0_3 = crate::Reg<u32, _MRGD_W3_0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W3_0_3;
#[doc = "`read()` method returns [mrgd_w3_0_3::R](mrgd_w3_0_3::R) reader structure"]
impl crate::Readable for MRGD_W3_0_3 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w3_0_3::W](mrgd_w3_0_3::W) writer structure"]
impl crate::Writable for MRGD_W3_0_3 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w3_0_3;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w4_0_3](mrgd_w4_0_3) module"]
pub type MRGD_W4_0_3 = crate::Reg<u32, _MRGD_W4_0_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W4_0_3;
#[doc = "`read()` method returns [mrgd_w4_0_3::R](mrgd_w4_0_3::R) reader structure"]
impl crate::Readable for MRGD_W4_0_3 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w4_0_3::W](mrgd_w4_0_3::W) writer structure"]
impl crate::Writable for MRGD_W4_0_3 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w4_0_3;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w0_0_4](mrgd_w0_0_4) module"]
pub type MRGD_W0_0_4 = crate::Reg<u32, _MRGD_W0_0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W0_0_4;
#[doc = "`read()` method returns [mrgd_w0_0_4::R](mrgd_w0_0_4::R) reader structure"]
impl crate::Readable for MRGD_W0_0_4 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w0_0_4::W](mrgd_w0_0_4::W) writer structure"]
impl crate::Writable for MRGD_W0_0_4 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w0_0_4;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w1_0_4](mrgd_w1_0_4) module"]
pub type MRGD_W1_0_4 = crate::Reg<u32, _MRGD_W1_0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W1_0_4;
#[doc = "`read()` method returns [mrgd_w1_0_4::R](mrgd_w1_0_4::R) reader structure"]
impl crate::Readable for MRGD_W1_0_4 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w1_0_4::W](mrgd_w1_0_4::W) writer structure"]
impl crate::Writable for MRGD_W1_0_4 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w1_0_4;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w2_0_4](mrgd_w2_0_4) module"]
pub type MRGD_W2_0_4 = crate::Reg<u32, _MRGD_W2_0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W2_0_4;
#[doc = "`read()` method returns [mrgd_w2_0_4::R](mrgd_w2_0_4::R) reader structure"]
impl crate::Readable for MRGD_W2_0_4 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w2_0_4::W](mrgd_w2_0_4::W) writer structure"]
impl crate::Writable for MRGD_W2_0_4 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w2_0_4;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w3_0_4](mrgd_w3_0_4) module"]
pub type MRGD_W3_0_4 = crate::Reg<u32, _MRGD_W3_0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W3_0_4;
#[doc = "`read()` method returns [mrgd_w3_0_4::R](mrgd_w3_0_4::R) reader structure"]
impl crate::Readable for MRGD_W3_0_4 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w3_0_4::W](mrgd_w3_0_4::W) writer structure"]
impl crate::Writable for MRGD_W3_0_4 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w3_0_4;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w4_0_4](mrgd_w4_0_4) module"]
pub type MRGD_W4_0_4 = crate::Reg<u32, _MRGD_W4_0_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W4_0_4;
#[doc = "`read()` method returns [mrgd_w4_0_4::R](mrgd_w4_0_4::R) reader structure"]
impl crate::Readable for MRGD_W4_0_4 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w4_0_4::W](mrgd_w4_0_4::W) writer structure"]
impl crate::Writable for MRGD_W4_0_4 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w4_0_4;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w0_0_5](mrgd_w0_0_5) module"]
pub type MRGD_W0_0_5 = crate::Reg<u32, _MRGD_W0_0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W0_0_5;
#[doc = "`read()` method returns [mrgd_w0_0_5::R](mrgd_w0_0_5::R) reader structure"]
impl crate::Readable for MRGD_W0_0_5 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w0_0_5::W](mrgd_w0_0_5::W) writer structure"]
impl crate::Writable for MRGD_W0_0_5 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w0_0_5;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w1_0_5](mrgd_w1_0_5) module"]
pub type MRGD_W1_0_5 = crate::Reg<u32, _MRGD_W1_0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W1_0_5;
#[doc = "`read()` method returns [mrgd_w1_0_5::R](mrgd_w1_0_5::R) reader structure"]
impl crate::Readable for MRGD_W1_0_5 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w1_0_5::W](mrgd_w1_0_5::W) writer structure"]
impl crate::Writable for MRGD_W1_0_5 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w1_0_5;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w2_0_5](mrgd_w2_0_5) module"]
pub type MRGD_W2_0_5 = crate::Reg<u32, _MRGD_W2_0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W2_0_5;
#[doc = "`read()` method returns [mrgd_w2_0_5::R](mrgd_w2_0_5::R) reader structure"]
impl crate::Readable for MRGD_W2_0_5 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w2_0_5::W](mrgd_w2_0_5::W) writer structure"]
impl crate::Writable for MRGD_W2_0_5 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w2_0_5;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w3_0_5](mrgd_w3_0_5) module"]
pub type MRGD_W3_0_5 = crate::Reg<u32, _MRGD_W3_0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W3_0_5;
#[doc = "`read()` method returns [mrgd_w3_0_5::R](mrgd_w3_0_5::R) reader structure"]
impl crate::Readable for MRGD_W3_0_5 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w3_0_5::W](mrgd_w3_0_5::W) writer structure"]
impl crate::Writable for MRGD_W3_0_5 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w3_0_5;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w4_0_5](mrgd_w4_0_5) module"]
pub type MRGD_W4_0_5 = crate::Reg<u32, _MRGD_W4_0_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W4_0_5;
#[doc = "`read()` method returns [mrgd_w4_0_5::R](mrgd_w4_0_5::R) reader structure"]
impl crate::Readable for MRGD_W4_0_5 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w4_0_5::W](mrgd_w4_0_5::W) writer structure"]
impl crate::Writable for MRGD_W4_0_5 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w4_0_5;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w0_0_6](mrgd_w0_0_6) module"]
pub type MRGD_W0_0_6 = crate::Reg<u32, _MRGD_W0_0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W0_0_6;
#[doc = "`read()` method returns [mrgd_w0_0_6::R](mrgd_w0_0_6::R) reader structure"]
impl crate::Readable for MRGD_W0_0_6 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w0_0_6::W](mrgd_w0_0_6::W) writer structure"]
impl crate::Writable for MRGD_W0_0_6 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w0_0_6;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w1_0_6](mrgd_w1_0_6) module"]
pub type MRGD_W1_0_6 = crate::Reg<u32, _MRGD_W1_0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W1_0_6;
#[doc = "`read()` method returns [mrgd_w1_0_6::R](mrgd_w1_0_6::R) reader structure"]
impl crate::Readable for MRGD_W1_0_6 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w1_0_6::W](mrgd_w1_0_6::W) writer structure"]
impl crate::Writable for MRGD_W1_0_6 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w1_0_6;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w2_0_6](mrgd_w2_0_6) module"]
pub type MRGD_W2_0_6 = crate::Reg<u32, _MRGD_W2_0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W2_0_6;
#[doc = "`read()` method returns [mrgd_w2_0_6::R](mrgd_w2_0_6::R) reader structure"]
impl crate::Readable for MRGD_W2_0_6 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w2_0_6::W](mrgd_w2_0_6::W) writer structure"]
impl crate::Writable for MRGD_W2_0_6 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w2_0_6;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w3_0_6](mrgd_w3_0_6) module"]
pub type MRGD_W3_0_6 = crate::Reg<u32, _MRGD_W3_0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W3_0_6;
#[doc = "`read()` method returns [mrgd_w3_0_6::R](mrgd_w3_0_6::R) reader structure"]
impl crate::Readable for MRGD_W3_0_6 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w3_0_6::W](mrgd_w3_0_6::W) writer structure"]
impl crate::Writable for MRGD_W3_0_6 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w3_0_6;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w4_0_6](mrgd_w4_0_6) module"]
pub type MRGD_W4_0_6 = crate::Reg<u32, _MRGD_W4_0_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W4_0_6;
#[doc = "`read()` method returns [mrgd_w4_0_6::R](mrgd_w4_0_6::R) reader structure"]
impl crate::Readable for MRGD_W4_0_6 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w4_0_6::W](mrgd_w4_0_6::W) writer structure"]
impl crate::Writable for MRGD_W4_0_6 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w4_0_6;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w0_0_7](mrgd_w0_0_7) module"]
pub type MRGD_W0_0_7 = crate::Reg<u32, _MRGD_W0_0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W0_0_7;
#[doc = "`read()` method returns [mrgd_w0_0_7::R](mrgd_w0_0_7::R) reader structure"]
impl crate::Readable for MRGD_W0_0_7 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w0_0_7::W](mrgd_w0_0_7::W) writer structure"]
impl crate::Writable for MRGD_W0_0_7 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w0_0_7;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w1_0_7](mrgd_w1_0_7) module"]
pub type MRGD_W1_0_7 = crate::Reg<u32, _MRGD_W1_0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W1_0_7;
#[doc = "`read()` method returns [mrgd_w1_0_7::R](mrgd_w1_0_7::R) reader structure"]
impl crate::Readable for MRGD_W1_0_7 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w1_0_7::W](mrgd_w1_0_7::W) writer structure"]
impl crate::Writable for MRGD_W1_0_7 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w1_0_7;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w2_0_7](mrgd_w2_0_7) module"]
pub type MRGD_W2_0_7 = crate::Reg<u32, _MRGD_W2_0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W2_0_7;
#[doc = "`read()` method returns [mrgd_w2_0_7::R](mrgd_w2_0_7::R) reader structure"]
impl crate::Readable for MRGD_W2_0_7 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w2_0_7::W](mrgd_w2_0_7::W) writer structure"]
impl crate::Writable for MRGD_W2_0_7 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w2_0_7;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w3_0_7](mrgd_w3_0_7) module"]
pub type MRGD_W3_0_7 = crate::Reg<u32, _MRGD_W3_0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W3_0_7;
#[doc = "`read()` method returns [mrgd_w3_0_7::R](mrgd_w3_0_7::R) reader structure"]
impl crate::Readable for MRGD_W3_0_7 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w3_0_7::W](mrgd_w3_0_7::W) writer structure"]
impl crate::Writable for MRGD_W3_0_7 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w3_0_7;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w4_0_7](mrgd_w4_0_7) module"]
pub type MRGD_W4_0_7 = crate::Reg<u32, _MRGD_W4_0_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W4_0_7;
#[doc = "`read()` method returns [mrgd_w4_0_7::R](mrgd_w4_0_7::R) reader structure"]
impl crate::Readable for MRGD_W4_0_7 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w4_0_7::W](mrgd_w4_0_7::W) writer structure"]
impl crate::Writable for MRGD_W4_0_7 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w4_0_7;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w0_1_0](mrgd_w0_1_0) module"]
pub type MRGD_W0_1_0 = crate::Reg<u32, _MRGD_W0_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W0_1_0;
#[doc = "`read()` method returns [mrgd_w0_1_0::R](mrgd_w0_1_0::R) reader structure"]
impl crate::Readable for MRGD_W0_1_0 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w0_1_0::W](mrgd_w0_1_0::W) writer structure"]
impl crate::Writable for MRGD_W0_1_0 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w0_1_0;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w1_1_0](mrgd_w1_1_0) module"]
pub type MRGD_W1_1_0 = crate::Reg<u32, _MRGD_W1_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W1_1_0;
#[doc = "`read()` method returns [mrgd_w1_1_0::R](mrgd_w1_1_0::R) reader structure"]
impl crate::Readable for MRGD_W1_1_0 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w1_1_0::W](mrgd_w1_1_0::W) writer structure"]
impl crate::Writable for MRGD_W1_1_0 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w1_1_0;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w2_1_0](mrgd_w2_1_0) module"]
pub type MRGD_W2_1_0 = crate::Reg<u32, _MRGD_W2_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W2_1_0;
#[doc = "`read()` method returns [mrgd_w2_1_0::R](mrgd_w2_1_0::R) reader structure"]
impl crate::Readable for MRGD_W2_1_0 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w2_1_0::W](mrgd_w2_1_0::W) writer structure"]
impl crate::Writable for MRGD_W2_1_0 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w2_1_0;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w3_1_0](mrgd_w3_1_0) module"]
pub type MRGD_W3_1_0 = crate::Reg<u32, _MRGD_W3_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W3_1_0;
#[doc = "`read()` method returns [mrgd_w3_1_0::R](mrgd_w3_1_0::R) reader structure"]
impl crate::Readable for MRGD_W3_1_0 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w3_1_0::W](mrgd_w3_1_0::W) writer structure"]
impl crate::Writable for MRGD_W3_1_0 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w3_1_0;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w4_1_0](mrgd_w4_1_0) module"]
pub type MRGD_W4_1_0 = crate::Reg<u32, _MRGD_W4_1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W4_1_0;
#[doc = "`read()` method returns [mrgd_w4_1_0::R](mrgd_w4_1_0::R) reader structure"]
impl crate::Readable for MRGD_W4_1_0 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w4_1_0::W](mrgd_w4_1_0::W) writer structure"]
impl crate::Writable for MRGD_W4_1_0 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w4_1_0;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w0_1_1](mrgd_w0_1_1) module"]
pub type MRGD_W0_1_1 = crate::Reg<u32, _MRGD_W0_1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W0_1_1;
#[doc = "`read()` method returns [mrgd_w0_1_1::R](mrgd_w0_1_1::R) reader structure"]
impl crate::Readable for MRGD_W0_1_1 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w0_1_1::W](mrgd_w0_1_1::W) writer structure"]
impl crate::Writable for MRGD_W0_1_1 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w0_1_1;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w1_1_1](mrgd_w1_1_1) module"]
pub type MRGD_W1_1_1 = crate::Reg<u32, _MRGD_W1_1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W1_1_1;
#[doc = "`read()` method returns [mrgd_w1_1_1::R](mrgd_w1_1_1::R) reader structure"]
impl crate::Readable for MRGD_W1_1_1 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w1_1_1::W](mrgd_w1_1_1::W) writer structure"]
impl crate::Writable for MRGD_W1_1_1 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w1_1_1;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w2_1_1](mrgd_w2_1_1) module"]
pub type MRGD_W2_1_1 = crate::Reg<u32, _MRGD_W2_1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W2_1_1;
#[doc = "`read()` method returns [mrgd_w2_1_1::R](mrgd_w2_1_1::R) reader structure"]
impl crate::Readable for MRGD_W2_1_1 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w2_1_1::W](mrgd_w2_1_1::W) writer structure"]
impl crate::Writable for MRGD_W2_1_1 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w2_1_1;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w3_1_1](mrgd_w3_1_1) module"]
pub type MRGD_W3_1_1 = crate::Reg<u32, _MRGD_W3_1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W3_1_1;
#[doc = "`read()` method returns [mrgd_w3_1_1::R](mrgd_w3_1_1::R) reader structure"]
impl crate::Readable for MRGD_W3_1_1 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w3_1_1::W](mrgd_w3_1_1::W) writer structure"]
impl crate::Writable for MRGD_W3_1_1 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w3_1_1;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w4_1_1](mrgd_w4_1_1) module"]
pub type MRGD_W4_1_1 = crate::Reg<u32, _MRGD_W4_1_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W4_1_1;
#[doc = "`read()` method returns [mrgd_w4_1_1::R](mrgd_w4_1_1::R) reader structure"]
impl crate::Readable for MRGD_W4_1_1 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w4_1_1::W](mrgd_w4_1_1::W) writer structure"]
impl crate::Writable for MRGD_W4_1_1 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w4_1_1;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w0_1_2](mrgd_w0_1_2) module"]
pub type MRGD_W0_1_2 = crate::Reg<u32, _MRGD_W0_1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W0_1_2;
#[doc = "`read()` method returns [mrgd_w0_1_2::R](mrgd_w0_1_2::R) reader structure"]
impl crate::Readable for MRGD_W0_1_2 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w0_1_2::W](mrgd_w0_1_2::W) writer structure"]
impl crate::Writable for MRGD_W0_1_2 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w0_1_2;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w1_1_2](mrgd_w1_1_2) module"]
pub type MRGD_W1_1_2 = crate::Reg<u32, _MRGD_W1_1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W1_1_2;
#[doc = "`read()` method returns [mrgd_w1_1_2::R](mrgd_w1_1_2::R) reader structure"]
impl crate::Readable for MRGD_W1_1_2 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w1_1_2::W](mrgd_w1_1_2::W) writer structure"]
impl crate::Writable for MRGD_W1_1_2 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w1_1_2;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w2_1_2](mrgd_w2_1_2) module"]
pub type MRGD_W2_1_2 = crate::Reg<u32, _MRGD_W2_1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W2_1_2;
#[doc = "`read()` method returns [mrgd_w2_1_2::R](mrgd_w2_1_2::R) reader structure"]
impl crate::Readable for MRGD_W2_1_2 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w2_1_2::W](mrgd_w2_1_2::W) writer structure"]
impl crate::Writable for MRGD_W2_1_2 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w2_1_2;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w3_1_2](mrgd_w3_1_2) module"]
pub type MRGD_W3_1_2 = crate::Reg<u32, _MRGD_W3_1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W3_1_2;
#[doc = "`read()` method returns [mrgd_w3_1_2::R](mrgd_w3_1_2::R) reader structure"]
impl crate::Readable for MRGD_W3_1_2 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w3_1_2::W](mrgd_w3_1_2::W) writer structure"]
impl crate::Writable for MRGD_W3_1_2 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w3_1_2;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w4_1_2](mrgd_w4_1_2) module"]
pub type MRGD_W4_1_2 = crate::Reg<u32, _MRGD_W4_1_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W4_1_2;
#[doc = "`read()` method returns [mrgd_w4_1_2::R](mrgd_w4_1_2::R) reader structure"]
impl crate::Readable for MRGD_W4_1_2 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w4_1_2::W](mrgd_w4_1_2::W) writer structure"]
impl crate::Writable for MRGD_W4_1_2 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w4_1_2;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w0_1_3](mrgd_w0_1_3) module"]
pub type MRGD_W0_1_3 = crate::Reg<u32, _MRGD_W0_1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W0_1_3;
#[doc = "`read()` method returns [mrgd_w0_1_3::R](mrgd_w0_1_3::R) reader structure"]
impl crate::Readable for MRGD_W0_1_3 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w0_1_3::W](mrgd_w0_1_3::W) writer structure"]
impl crate::Writable for MRGD_W0_1_3 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w0_1_3;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w1_1_3](mrgd_w1_1_3) module"]
pub type MRGD_W1_1_3 = crate::Reg<u32, _MRGD_W1_1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W1_1_3;
#[doc = "`read()` method returns [mrgd_w1_1_3::R](mrgd_w1_1_3::R) reader structure"]
impl crate::Readable for MRGD_W1_1_3 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w1_1_3::W](mrgd_w1_1_3::W) writer structure"]
impl crate::Writable for MRGD_W1_1_3 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w1_1_3;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w2_1_3](mrgd_w2_1_3) module"]
pub type MRGD_W2_1_3 = crate::Reg<u32, _MRGD_W2_1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W2_1_3;
#[doc = "`read()` method returns [mrgd_w2_1_3::R](mrgd_w2_1_3::R) reader structure"]
impl crate::Readable for MRGD_W2_1_3 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w2_1_3::W](mrgd_w2_1_3::W) writer structure"]
impl crate::Writable for MRGD_W2_1_3 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w2_1_3;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w3_1_3](mrgd_w3_1_3) module"]
pub type MRGD_W3_1_3 = crate::Reg<u32, _MRGD_W3_1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W3_1_3;
#[doc = "`read()` method returns [mrgd_w3_1_3::R](mrgd_w3_1_3::R) reader structure"]
impl crate::Readable for MRGD_W3_1_3 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w3_1_3::W](mrgd_w3_1_3::W) writer structure"]
impl crate::Writable for MRGD_W3_1_3 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w3_1_3;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w4_1_3](mrgd_w4_1_3) module"]
pub type MRGD_W4_1_3 = crate::Reg<u32, _MRGD_W4_1_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W4_1_3;
#[doc = "`read()` method returns [mrgd_w4_1_3::R](mrgd_w4_1_3::R) reader structure"]
impl crate::Readable for MRGD_W4_1_3 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w4_1_3::W](mrgd_w4_1_3::W) writer structure"]
impl crate::Writable for MRGD_W4_1_3 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w4_1_3;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w0_1_4](mrgd_w0_1_4) module"]
pub type MRGD_W0_1_4 = crate::Reg<u32, _MRGD_W0_1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W0_1_4;
#[doc = "`read()` method returns [mrgd_w0_1_4::R](mrgd_w0_1_4::R) reader structure"]
impl crate::Readable for MRGD_W0_1_4 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w0_1_4::W](mrgd_w0_1_4::W) writer structure"]
impl crate::Writable for MRGD_W0_1_4 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w0_1_4;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w1_1_4](mrgd_w1_1_4) module"]
pub type MRGD_W1_1_4 = crate::Reg<u32, _MRGD_W1_1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W1_1_4;
#[doc = "`read()` method returns [mrgd_w1_1_4::R](mrgd_w1_1_4::R) reader structure"]
impl crate::Readable for MRGD_W1_1_4 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w1_1_4::W](mrgd_w1_1_4::W) writer structure"]
impl crate::Writable for MRGD_W1_1_4 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w1_1_4;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w2_1_4](mrgd_w2_1_4) module"]
pub type MRGD_W2_1_4 = crate::Reg<u32, _MRGD_W2_1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W2_1_4;
#[doc = "`read()` method returns [mrgd_w2_1_4::R](mrgd_w2_1_4::R) reader structure"]
impl crate::Readable for MRGD_W2_1_4 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w2_1_4::W](mrgd_w2_1_4::W) writer structure"]
impl crate::Writable for MRGD_W2_1_4 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w2_1_4;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w3_1_4](mrgd_w3_1_4) module"]
pub type MRGD_W3_1_4 = crate::Reg<u32, _MRGD_W3_1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W3_1_4;
#[doc = "`read()` method returns [mrgd_w3_1_4::R](mrgd_w3_1_4::R) reader structure"]
impl crate::Readable for MRGD_W3_1_4 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w3_1_4::W](mrgd_w3_1_4::W) writer structure"]
impl crate::Writable for MRGD_W3_1_4 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w3_1_4;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w4_1_4](mrgd_w4_1_4) module"]
pub type MRGD_W4_1_4 = crate::Reg<u32, _MRGD_W4_1_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W4_1_4;
#[doc = "`read()` method returns [mrgd_w4_1_4::R](mrgd_w4_1_4::R) reader structure"]
impl crate::Readable for MRGD_W4_1_4 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w4_1_4::W](mrgd_w4_1_4::W) writer structure"]
impl crate::Writable for MRGD_W4_1_4 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w4_1_4;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w0_1_5](mrgd_w0_1_5) module"]
pub type MRGD_W0_1_5 = crate::Reg<u32, _MRGD_W0_1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W0_1_5;
#[doc = "`read()` method returns [mrgd_w0_1_5::R](mrgd_w0_1_5::R) reader structure"]
impl crate::Readable for MRGD_W0_1_5 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w0_1_5::W](mrgd_w0_1_5::W) writer structure"]
impl crate::Writable for MRGD_W0_1_5 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w0_1_5;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w1_1_5](mrgd_w1_1_5) module"]
pub type MRGD_W1_1_5 = crate::Reg<u32, _MRGD_W1_1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W1_1_5;
#[doc = "`read()` method returns [mrgd_w1_1_5::R](mrgd_w1_1_5::R) reader structure"]
impl crate::Readable for MRGD_W1_1_5 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w1_1_5::W](mrgd_w1_1_5::W) writer structure"]
impl crate::Writable for MRGD_W1_1_5 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w1_1_5;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w2_1_5](mrgd_w2_1_5) module"]
pub type MRGD_W2_1_5 = crate::Reg<u32, _MRGD_W2_1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W2_1_5;
#[doc = "`read()` method returns [mrgd_w2_1_5::R](mrgd_w2_1_5::R) reader structure"]
impl crate::Readable for MRGD_W2_1_5 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w2_1_5::W](mrgd_w2_1_5::W) writer structure"]
impl crate::Writable for MRGD_W2_1_5 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w2_1_5;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w3_1_5](mrgd_w3_1_5) module"]
pub type MRGD_W3_1_5 = crate::Reg<u32, _MRGD_W3_1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W3_1_5;
#[doc = "`read()` method returns [mrgd_w3_1_5::R](mrgd_w3_1_5::R) reader structure"]
impl crate::Readable for MRGD_W3_1_5 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w3_1_5::W](mrgd_w3_1_5::W) writer structure"]
impl crate::Writable for MRGD_W3_1_5 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w3_1_5;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w4_1_5](mrgd_w4_1_5) module"]
pub type MRGD_W4_1_5 = crate::Reg<u32, _MRGD_W4_1_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W4_1_5;
#[doc = "`read()` method returns [mrgd_w4_1_5::R](mrgd_w4_1_5::R) reader structure"]
impl crate::Readable for MRGD_W4_1_5 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w4_1_5::W](mrgd_w4_1_5::W) writer structure"]
impl crate::Writable for MRGD_W4_1_5 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w4_1_5;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w0_1_6](mrgd_w0_1_6) module"]
pub type MRGD_W0_1_6 = crate::Reg<u32, _MRGD_W0_1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W0_1_6;
#[doc = "`read()` method returns [mrgd_w0_1_6::R](mrgd_w0_1_6::R) reader structure"]
impl crate::Readable for MRGD_W0_1_6 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w0_1_6::W](mrgd_w0_1_6::W) writer structure"]
impl crate::Writable for MRGD_W0_1_6 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w0_1_6;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w1_1_6](mrgd_w1_1_6) module"]
pub type MRGD_W1_1_6 = crate::Reg<u32, _MRGD_W1_1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W1_1_6;
#[doc = "`read()` method returns [mrgd_w1_1_6::R](mrgd_w1_1_6::R) reader structure"]
impl crate::Readable for MRGD_W1_1_6 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w1_1_6::W](mrgd_w1_1_6::W) writer structure"]
impl crate::Writable for MRGD_W1_1_6 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w1_1_6;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w2_1_6](mrgd_w2_1_6) module"]
pub type MRGD_W2_1_6 = crate::Reg<u32, _MRGD_W2_1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W2_1_6;
#[doc = "`read()` method returns [mrgd_w2_1_6::R](mrgd_w2_1_6::R) reader structure"]
impl crate::Readable for MRGD_W2_1_6 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w2_1_6::W](mrgd_w2_1_6::W) writer structure"]
impl crate::Writable for MRGD_W2_1_6 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w2_1_6;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w3_1_6](mrgd_w3_1_6) module"]
pub type MRGD_W3_1_6 = crate::Reg<u32, _MRGD_W3_1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W3_1_6;
#[doc = "`read()` method returns [mrgd_w3_1_6::R](mrgd_w3_1_6::R) reader structure"]
impl crate::Readable for MRGD_W3_1_6 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w3_1_6::W](mrgd_w3_1_6::W) writer structure"]
impl crate::Writable for MRGD_W3_1_6 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w3_1_6;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w4_1_6](mrgd_w4_1_6) module"]
pub type MRGD_W4_1_6 = crate::Reg<u32, _MRGD_W4_1_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W4_1_6;
#[doc = "`read()` method returns [mrgd_w4_1_6::R](mrgd_w4_1_6::R) reader structure"]
impl crate::Readable for MRGD_W4_1_6 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w4_1_6::W](mrgd_w4_1_6::W) writer structure"]
impl crate::Writable for MRGD_W4_1_6 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w4_1_6;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w0_1_7](mrgd_w0_1_7) module"]
pub type MRGD_W0_1_7 = crate::Reg<u32, _MRGD_W0_1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W0_1_7;
#[doc = "`read()` method returns [mrgd_w0_1_7::R](mrgd_w0_1_7::R) reader structure"]
impl crate::Readable for MRGD_W0_1_7 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w0_1_7::W](mrgd_w0_1_7::W) writer structure"]
impl crate::Writable for MRGD_W0_1_7 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w0_1_7;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w1_1_7](mrgd_w1_1_7) module"]
pub type MRGD_W1_1_7 = crate::Reg<u32, _MRGD_W1_1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W1_1_7;
#[doc = "`read()` method returns [mrgd_w1_1_7::R](mrgd_w1_1_7::R) reader structure"]
impl crate::Readable for MRGD_W1_1_7 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w1_1_7::W](mrgd_w1_1_7::W) writer structure"]
impl crate::Writable for MRGD_W1_1_7 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w1_1_7;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w2_1_7](mrgd_w2_1_7) module"]
pub type MRGD_W2_1_7 = crate::Reg<u32, _MRGD_W2_1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W2_1_7;
#[doc = "`read()` method returns [mrgd_w2_1_7::R](mrgd_w2_1_7::R) reader structure"]
impl crate::Readable for MRGD_W2_1_7 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w2_1_7::W](mrgd_w2_1_7::W) writer structure"]
impl crate::Writable for MRGD_W2_1_7 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w2_1_7;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w3_1_7](mrgd_w3_1_7) module"]
pub type MRGD_W3_1_7 = crate::Reg<u32, _MRGD_W3_1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W3_1_7;
#[doc = "`read()` method returns [mrgd_w3_1_7::R](mrgd_w3_1_7::R) reader structure"]
impl crate::Readable for MRGD_W3_1_7 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w3_1_7::W](mrgd_w3_1_7::W) writer structure"]
impl crate::Writable for MRGD_W3_1_7 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w3_1_7;
#[doc = "Memory Region Descriptor\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mrgd_w4_1_7](mrgd_w4_1_7) module"]
pub type MRGD_W4_1_7 = crate::Reg<u32, _MRGD_W4_1_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRGD_W4_1_7;
#[doc = "`read()` method returns [mrgd_w4_1_7::R](mrgd_w4_1_7::R) reader structure"]
impl crate::Readable for MRGD_W4_1_7 {}
#[doc = "`write(|w| ..)` method takes [mrgd_w4_1_7::W](mrgd_w4_1_7::W) writer structure"]
impl crate::Writable for MRGD_W4_1_7 {}
#[doc = "Memory Region Descriptor"]
pub mod mrgd_w4_1_7;
