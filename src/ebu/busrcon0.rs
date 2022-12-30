#[doc = "Register `BUSRCON0` reader"]
pub struct R(crate::R<BUSRCON0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSRCON0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSRCON0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSRCON0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUSRCON0` writer"]
pub struct W(crate::W<BUSRCON0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUSRCON0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BUSRCON0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUSRCON0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FETBLEN` reader - Burst Length for Synchronous Burst"]
pub type FETBLEN_R = crate::FieldReader<u8, FETBLEN_A>;
#[doc = "Burst Length for Synchronous Burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FETBLEN_A {
    #[doc = "0: 1 data access (default after reset)."]
    VALUE1 = 0,
    #[doc = "1: 2 data accesses."]
    VALUE2 = 1,
    #[doc = "2: 4 data accesses."]
    VALUE3 = 2,
    #[doc = "3: 8 data accesses."]
    VALUE4 = 3,
}
impl From<FETBLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: FETBLEN_A) -> Self {
        variant as _
    }
}
impl FETBLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FETBLEN_A> {
        match self.bits {
            0 => Some(FETBLEN_A::VALUE1),
            1 => Some(FETBLEN_A::VALUE2),
            2 => Some(FETBLEN_A::VALUE3),
            3 => Some(FETBLEN_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FETBLEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FETBLEN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == FETBLEN_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == FETBLEN_A::VALUE4
    }
}
#[doc = "Field `FETBLEN` writer - Burst Length for Synchronous Burst"]
pub type FETBLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUSRCON0_SPEC, u8, FETBLEN_A, 3, O>;
impl<'a, const O: u8> FETBLEN_W<'a, O> {
    #[doc = "1 data access (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FETBLEN_A::VALUE1)
    }
    #[doc = "2 data accesses."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FETBLEN_A::VALUE2)
    }
    #[doc = "4 data accesses."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(FETBLEN_A::VALUE3)
    }
    #[doc = "8 data accesses."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(FETBLEN_A::VALUE4)
    }
}
#[doc = "Field `FBBMSEL` reader - Synchronous burst buffer mode select"]
pub type FBBMSEL_R = crate::BitReader<FBBMSEL_A>;
#[doc = "Synchronous burst buffer mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FBBMSEL_A {
    #[doc = "0: Burst buffer length defined by value in FETBLEN (default after reset)."]
    VALUE1 = 0,
    #[doc = "1: Continuous mode. All data required for transaction is transferred in a single burst."]
    VALUE2 = 1,
}
impl From<FBBMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: FBBMSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl FBBMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FBBMSEL_A {
        match self.bits {
            false => FBBMSEL_A::VALUE1,
            true => FBBMSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FBBMSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FBBMSEL_A::VALUE2
    }
}
#[doc = "Field `FBBMSEL` writer - Synchronous burst buffer mode select"]
pub type FBBMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUSRCON0_SPEC, FBBMSEL_A, O>;
impl<'a, const O: u8> FBBMSEL_W<'a, O> {
    #[doc = "Burst buffer length defined by value in FETBLEN (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FBBMSEL_A::VALUE1)
    }
    #[doc = "Continuous mode. All data required for transaction is transferred in a single burst."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FBBMSEL_A::VALUE2)
    }
}
#[doc = "Field `BFSSS` reader - Read Single Stage Synchronization:"]
pub type BFSSS_R = crate::BitReader<BFSSS_A>;
#[doc = "Read Single Stage Synchronization:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFSSS_A {
    #[doc = "0: Two stages of synchronization used. (maximum margin)"]
    VALUE1 = 0,
    #[doc = "1: One stage of synchronization used. (minimum latency)"]
    VALUE2 = 1,
}
impl From<BFSSS_A> for bool {
    #[inline(always)]
    fn from(variant: BFSSS_A) -> Self {
        variant as u8 != 0
    }
}
impl BFSSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFSSS_A {
        match self.bits {
            false => BFSSS_A::VALUE1,
            true => BFSSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFSSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFSSS_A::VALUE2
    }
}
#[doc = "Field `BFSSS` writer - Read Single Stage Synchronization:"]
pub type BFSSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUSRCON0_SPEC, BFSSS_A, O>;
impl<'a, const O: u8> BFSSS_W<'a, O> {
    #[doc = "Two stages of synchronization used. (maximum margin)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFSSS_A::VALUE1)
    }
    #[doc = "One stage of synchronization used. (minimum latency)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFSSS_A::VALUE2)
    }
}
#[doc = "Field `FDBKEN` reader - Burst FLASH Clock Feedback Enable"]
pub type FDBKEN_R = crate::BitReader<FDBKEN_A>;
#[doc = "Burst FLASH Clock Feedback Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDBKEN_A {
    #[doc = "0: BFCLK feedback not used."]
    VALUE1 = 0,
    #[doc = "1: Incoming data and control signals (from the Burst FLASH device) are re-synchronized to the BFCLKI input."]
    VALUE2 = 1,
}
impl From<FDBKEN_A> for bool {
    #[inline(always)]
    fn from(variant: FDBKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FDBKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDBKEN_A {
        match self.bits {
            false => FDBKEN_A::VALUE1,
            true => FDBKEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FDBKEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FDBKEN_A::VALUE2
    }
}
#[doc = "Field `FDBKEN` writer - Burst FLASH Clock Feedback Enable"]
pub type FDBKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUSRCON0_SPEC, FDBKEN_A, O>;
impl<'a, const O: u8> FDBKEN_W<'a, O> {
    #[doc = "BFCLK feedback not used."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FDBKEN_A::VALUE1)
    }
    #[doc = "Incoming data and control signals (from the Burst FLASH device) are re-synchronized to the BFCLKI input."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FDBKEN_A::VALUE2)
    }
}
#[doc = "Field `BFCMSEL` reader - Burst Flash Clock Mode Select"]
pub type BFCMSEL_R = crate::BitReader<BFCMSEL_A>;
#[doc = "Burst Flash Clock Mode Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFCMSEL_A {
    #[doc = "0: Burst Flash Clock runs continuously with values selected by this register"]
    VALUE1 = 0,
    #[doc = "1: Burst Flash Clock is disabled between accesses"]
    VALUE2 = 1,
}
impl From<BFCMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: BFCMSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl BFCMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFCMSEL_A {
        match self.bits {
            false => BFCMSEL_A::VALUE1,
            true => BFCMSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BFCMSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BFCMSEL_A::VALUE2
    }
}
#[doc = "Field `BFCMSEL` writer - Burst Flash Clock Mode Select"]
pub type BFCMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUSRCON0_SPEC, BFCMSEL_A, O>;
impl<'a, const O: u8> BFCMSEL_W<'a, O> {
    #[doc = "Burst Flash Clock runs continuously with values selected by this register"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFCMSEL_A::VALUE1)
    }
    #[doc = "Burst Flash Clock is disabled between accesses"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFCMSEL_A::VALUE2)
    }
}
#[doc = "Field `NAA` reader - Enable flash non-array access workaround"]
pub type NAA_R = crate::BitReader<bool>;
#[doc = "Field `NAA` writer - Enable flash non-array access workaround"]
pub type NAA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUSRCON0_SPEC, bool, O>;
#[doc = "Field `ECSE` reader - Early Chip Select for Synchronous Burst"]
pub type ECSE_R = crate::BitReader<ECSE_A>;
#[doc = "Early Chip Select for Synchronous Burst\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECSE_A {
    #[doc = "0: CS is delayed."]
    VALUE1 = 0,
    #[doc = "1: CS is not delayed."]
    VALUE2 = 1,
}
impl From<ECSE_A> for bool {
    #[inline(always)]
    fn from(variant: ECSE_A) -> Self {
        variant as u8 != 0
    }
}
impl ECSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECSE_A {
        match self.bits {
            false => ECSE_A::VALUE1,
            true => ECSE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECSE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECSE_A::VALUE2
    }
}
#[doc = "Field `ECSE` writer - Early Chip Select for Synchronous Burst"]
pub type ECSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUSRCON0_SPEC, ECSE_A, O>;
impl<'a, const O: u8> ECSE_W<'a, O> {
    #[doc = "CS is delayed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECSE_A::VALUE1)
    }
    #[doc = "CS is not delayed."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECSE_A::VALUE2)
    }
}
#[doc = "Field `EBSE` reader - Early Burst Signal Enable for Synchronous Burst"]
pub type EBSE_R = crate::BitReader<EBSE_A>;
#[doc = "Early Burst Signal Enable for Synchronous Burst\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EBSE_A {
    #[doc = "0: ADV is delayed."]
    VALUE1 = 0,
    #[doc = "1: ADV is not delayed."]
    VALUE2 = 1,
}
impl From<EBSE_A> for bool {
    #[inline(always)]
    fn from(variant: EBSE_A) -> Self {
        variant as u8 != 0
    }
}
impl EBSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EBSE_A {
        match self.bits {
            false => EBSE_A::VALUE1,
            true => EBSE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EBSE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EBSE_A::VALUE2
    }
}
#[doc = "Field `EBSE` writer - Early Burst Signal Enable for Synchronous Burst"]
pub type EBSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUSRCON0_SPEC, EBSE_A, O>;
impl<'a, const O: u8> EBSE_W<'a, O> {
    #[doc = "ADV is delayed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EBSE_A::VALUE1)
    }
    #[doc = "ADV is not delayed."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EBSE_A::VALUE2)
    }
}
#[doc = "Field `DBA` reader - Disable Burst Address Wrapping"]
pub type DBA_R = crate::BitReader<DBA_A>;
#[doc = "Disable Burst Address Wrapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBA_A {
    #[doc = "0: Memory Controller automatically re-aligns any non-aligned synchronous burst access so that data can be fetched from the device in a single burst transaction."]
    VALUE1 = 0,
    #[doc = "1: Memory Controller always starts any burst access to a synchronous burst device at the address specified by the AHB request. Any required address wrapping must be automatically provided by the Burst FLASH device."]
    VALUE2 = 1,
}
impl From<DBA_A> for bool {
    #[inline(always)]
    fn from(variant: DBA_A) -> Self {
        variant as u8 != 0
    }
}
impl DBA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBA_A {
        match self.bits {
            false => DBA_A::VALUE1,
            true => DBA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DBA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DBA_A::VALUE2
    }
}
#[doc = "Field `DBA` writer - Disable Burst Address Wrapping"]
pub type DBA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUSRCON0_SPEC, DBA_A, O>;
impl<'a, const O: u8> DBA_W<'a, O> {
    #[doc = "Memory Controller automatically re-aligns any non-aligned synchronous burst access so that data can be fetched from the device in a single burst transaction."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DBA_A::VALUE1)
    }
    #[doc = "Memory Controller always starts any burst access to a synchronous burst device at the address specified by the AHB request. Any required address wrapping must be automatically provided by the Burst FLASH device."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DBA_A::VALUE2)
    }
}
#[doc = "Field `WAITINV` reader - Reversed polarity at WAIT"]
pub type WAITINV_R = crate::BitReader<WAITINV_A>;
#[doc = "Reversed polarity at WAIT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITINV_A {
    #[doc = "0: input at WAIT pin is active low (default after reset)."]
    VALUE1 = 0,
    #[doc = "1: input at WAIT pin is active high."]
    VALUE2 = 1,
}
impl From<WAITINV_A> for bool {
    #[inline(always)]
    fn from(variant: WAITINV_A) -> Self {
        variant as u8 != 0
    }
}
impl WAITINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITINV_A {
        match self.bits {
            false => WAITINV_A::VALUE1,
            true => WAITINV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAITINV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAITINV_A::VALUE2
    }
}
#[doc = "Field `WAITINV` writer - Reversed polarity at WAIT"]
pub type WAITINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUSRCON0_SPEC, WAITINV_A, O>;
impl<'a, const O: u8> WAITINV_W<'a, O> {
    #[doc = "input at WAIT pin is active low (default after reset)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAITINV_A::VALUE1)
    }
    #[doc = "input at WAIT pin is active high."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAITINV_A::VALUE2)
    }
}
#[doc = "Field `BCGEN` reader - Byte Control Signal Control"]
pub type BCGEN_R = crate::FieldReader<u8, BCGEN_A>;
#[doc = "Byte Control Signal Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BCGEN_A {
    #[doc = "0: Byte control signals follow chip select timing."]
    VALUE1 = 0,
    #[doc = "1: Byte control signals follow control signal timing (RD, RD/WR) (default after reset)."]
    VALUE2 = 1,
    #[doc = "2: Byte control signals follow write enable signal timing (RD/WR only)."]
    VALUE3 = 2,
}
impl From<BCGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: BCGEN_A) -> Self {
        variant as _
    }
}
impl BCGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BCGEN_A> {
        match self.bits {
            0 => Some(BCGEN_A::VALUE1),
            1 => Some(BCGEN_A::VALUE2),
            2 => Some(BCGEN_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BCGEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BCGEN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == BCGEN_A::VALUE3
    }
}
#[doc = "Field `BCGEN` writer - Byte Control Signal Control"]
pub type BCGEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUSRCON0_SPEC, u8, BCGEN_A, 2, O>;
impl<'a, const O: u8> BCGEN_W<'a, O> {
    #[doc = "Byte control signals follow chip select timing."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BCGEN_A::VALUE1)
    }
    #[doc = "Byte control signals follow control signal timing (RD, RD/WR) (default after reset)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BCGEN_A::VALUE2)
    }
    #[doc = "Byte control signals follow write enable signal timing (RD/WR only)."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(BCGEN_A::VALUE3)
    }
}
#[doc = "Field `PORTW` reader - Device Addressing Mode"]
pub type PORTW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORTW` writer - Device Addressing Mode"]
pub type PORTW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUSRCON0_SPEC, u8, u8, 2, O>;
#[doc = "Field `WAIT` reader - External Wait Control: 0=OFF (default after reset)., 1=Asynchronous input at WAIT., 2=Synchronous input at WAIT., 3=reserved., 0=OFF (default after reset)., 1=Wait for page load (Early WAIT)., 2=Wait for page load (WAIT with data)., 3=Abort and retry access.,"]
pub type WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAIT` writer - External Wait Control: 0=OFF (default after reset)., 1=Asynchronous input at WAIT., 2=Synchronous input at WAIT., 3=reserved., 0=OFF (default after reset)., 1=Wait for page load (Early WAIT)., 2=Wait for page load (WAIT with data)., 3=Abort and retry access.,"]
pub type WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUSRCON0_SPEC, u8, u8, 2, O>;
#[doc = "Field `AAP` reader - Asynchronous Address phase:"]
pub type AAP_R = crate::BitReader<AAP_A>;
#[doc = "Asynchronous Address phase:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AAP_A {
    #[doc = "0: Clock is enabled at beginning of access."]
    VALUE1 = 0,
    #[doc = "1: Clock is enabled at after address phase."]
    VALUE2 = 1,
}
impl From<AAP_A> for bool {
    #[inline(always)]
    fn from(variant: AAP_A) -> Self {
        variant as u8 != 0
    }
}
impl AAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AAP_A {
        match self.bits {
            false => AAP_A::VALUE1,
            true => AAP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AAP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AAP_A::VALUE2
    }
}
#[doc = "Field `AAP` writer - Asynchronous Address phase:"]
pub type AAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUSRCON0_SPEC, AAP_A, O>;
impl<'a, const O: u8> AAP_W<'a, O> {
    #[doc = "Clock is enabled at beginning of access."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AAP_A::VALUE1)
    }
    #[doc = "Clock is enabled at after address phase."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AAP_A::VALUE2)
    }
}
#[doc = "Field `AGEN` reader - Device Type for Region"]
pub type AGEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AGEN` writer - Device Type for Region"]
pub type AGEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUSRCON0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:2 - Burst Length for Synchronous Burst"]
    #[inline(always)]
    pub fn fetblen(&self) -> FETBLEN_R {
        FETBLEN_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Synchronous burst buffer mode select"]
    #[inline(always)]
    pub fn fbbmsel(&self) -> FBBMSEL_R {
        FBBMSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read Single Stage Synchronization:"]
    #[inline(always)]
    pub fn bfsss(&self) -> BFSSS_R {
        BFSSS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Burst FLASH Clock Feedback Enable"]
    #[inline(always)]
    pub fn fdbken(&self) -> FDBKEN_R {
        FDBKEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Burst Flash Clock Mode Select"]
    #[inline(always)]
    pub fn bfcmsel(&self) -> BFCMSEL_R {
        BFCMSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable flash non-array access workaround"]
    #[inline(always)]
    pub fn naa(&self) -> NAA_R {
        NAA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Early Chip Select for Synchronous Burst"]
    #[inline(always)]
    pub fn ecse(&self) -> ECSE_R {
        ECSE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Early Burst Signal Enable for Synchronous Burst"]
    #[inline(always)]
    pub fn ebse(&self) -> EBSE_R {
        EBSE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Disable Burst Address Wrapping"]
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reversed polarity at WAIT"]
    #[inline(always)]
    pub fn waitinv(&self) -> WAITINV_R {
        WAITINV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Byte Control Signal Control"]
    #[inline(always)]
    pub fn bcgen(&self) -> BCGEN_R {
        BCGEN_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Device Addressing Mode"]
    #[inline(always)]
    pub fn portw(&self) -> PORTW_R {
        PORTW_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - External Wait Control: 0=OFF (default after reset)., 1=Asynchronous input at WAIT., 2=Synchronous input at WAIT., 3=reserved., 0=OFF (default after reset)., 1=Wait for page load (Early WAIT)., 2=Wait for page load (WAIT with data)., 3=Abort and retry access.,"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Asynchronous Address phase:"]
    #[inline(always)]
    pub fn aap(&self) -> AAP_R {
        AAP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Device Type for Region"]
    #[inline(always)]
    pub fn agen(&self) -> AGEN_R {
        AGEN_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Burst Length for Synchronous Burst"]
    #[inline(always)]
    #[must_use]
    pub fn fetblen(&mut self) -> FETBLEN_W<0> {
        FETBLEN_W::new(self)
    }
    #[doc = "Bit 3 - Synchronous burst buffer mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fbbmsel(&mut self) -> FBBMSEL_W<3> {
        FBBMSEL_W::new(self)
    }
    #[doc = "Bit 4 - Read Single Stage Synchronization:"]
    #[inline(always)]
    #[must_use]
    pub fn bfsss(&mut self) -> BFSSS_W<4> {
        BFSSS_W::new(self)
    }
    #[doc = "Bit 5 - Burst FLASH Clock Feedback Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdbken(&mut self) -> FDBKEN_W<5> {
        FDBKEN_W::new(self)
    }
    #[doc = "Bit 6 - Burst Flash Clock Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn bfcmsel(&mut self) -> BFCMSEL_W<6> {
        BFCMSEL_W::new(self)
    }
    #[doc = "Bit 7 - Enable flash non-array access workaround"]
    #[inline(always)]
    #[must_use]
    pub fn naa(&mut self) -> NAA_W<7> {
        NAA_W::new(self)
    }
    #[doc = "Bit 16 - Early Chip Select for Synchronous Burst"]
    #[inline(always)]
    #[must_use]
    pub fn ecse(&mut self) -> ECSE_W<16> {
        ECSE_W::new(self)
    }
    #[doc = "Bit 17 - Early Burst Signal Enable for Synchronous Burst"]
    #[inline(always)]
    #[must_use]
    pub fn ebse(&mut self) -> EBSE_W<17> {
        EBSE_W::new(self)
    }
    #[doc = "Bit 18 - Disable Burst Address Wrapping"]
    #[inline(always)]
    #[must_use]
    pub fn dba(&mut self) -> DBA_W<18> {
        DBA_W::new(self)
    }
    #[doc = "Bit 19 - Reversed polarity at WAIT"]
    #[inline(always)]
    #[must_use]
    pub fn waitinv(&mut self) -> WAITINV_W<19> {
        WAITINV_W::new(self)
    }
    #[doc = "Bits 20:21 - Byte Control Signal Control"]
    #[inline(always)]
    #[must_use]
    pub fn bcgen(&mut self) -> BCGEN_W<20> {
        BCGEN_W::new(self)
    }
    #[doc = "Bits 22:23 - Device Addressing Mode"]
    #[inline(always)]
    #[must_use]
    pub fn portw(&mut self) -> PORTW_W<22> {
        PORTW_W::new(self)
    }
    #[doc = "Bits 24:25 - External Wait Control: 0=OFF (default after reset)., 1=Asynchronous input at WAIT., 2=Synchronous input at WAIT., 3=reserved., 0=OFF (default after reset)., 1=Wait for page load (Early WAIT)., 2=Wait for page load (WAIT with data)., 3=Abort and retry access.,"]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<24> {
        WAIT_W::new(self)
    }
    #[doc = "Bit 26 - Asynchronous Address phase:"]
    #[inline(always)]
    #[must_use]
    pub fn aap(&mut self) -> AAP_W<26> {
        AAP_W::new(self)
    }
    #[doc = "Bits 28:31 - Device Type for Region"]
    #[inline(always)]
    #[must_use]
    pub fn agen(&mut self) -> AGEN_W<28> {
        AGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBU Bus Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busrcon0](index.html) module"]
pub struct BUSRCON0_SPEC;
impl crate::RegisterSpec for BUSRCON0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [busrcon0::R](R) reader structure"]
impl crate::Readable for BUSRCON0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [busrcon0::W](W) writer structure"]
impl crate::Writable for BUSRCON0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUSRCON0 to value 0x00d3_0040"]
impl crate::Resettable for BUSRCON0_SPEC {
    const RESET_VALUE: Self::Ux = 0x00d3_0040;
}
