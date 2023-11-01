#[doc = "Register `DIEPCTL0` reader"]
pub type R = crate::R<DIEPCTL0_SPEC>;
#[doc = "Register `DIEPCTL0` writer"]
pub type W = crate::W<DIEPCTL0_SPEC>;
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MPS_R = crate::FieldReader<MPS_A>;
#[doc = "Maximum Packet Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MPS_A {
    #[doc = "0: 64 bytes"]
    VALUE1 = 0,
    #[doc = "1: 32 bytes"]
    VALUE2 = 1,
    #[doc = "2: 16 bytes"]
    VALUE3 = 2,
    #[doc = "3: 8 bytes"]
    VALUE4 = 3,
}
impl From<MPS_A> for u8 {
    #[inline(always)]
    fn from(variant: MPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MPS_A {
    type Ux = u8;
}
impl MPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MPS_A {
        match self.bits {
            0 => MPS_A::VALUE1,
            1 => MPS_A::VALUE2,
            2 => MPS_A::VALUE3,
            3 => MPS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MPS_A::VALUE1
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MPS_A::VALUE2
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MPS_A::VALUE3
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MPS_A::VALUE4
    }
}
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MPS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, MPS_A>;
impl<'a, REG, const O: u8> MPS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MPS_A::VALUE1)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MPS_A::VALUE2)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(MPS_A::VALUE3)
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(MPS_A::VALUE4)
    }
}
#[doc = "Field `USBActEP` reader - USB Active Endpoint"]
pub type USBACT_EP_R = crate::BitReader;
#[doc = "Field `NAKSts` reader - NAK Status"]
pub type NAKSTS_R = crate::BitReader<NAKSTS_A>;
#[doc = "NAK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NAKSTS_A {
    #[doc = "0: The core is transmitting non-NAK handshakes based on the FIFO status"]
    VALUE1 = 0,
    #[doc = "1: The core is transmitting NAK handshakes on this endpoint."]
    VALUE2 = 1,
}
impl From<NAKSTS_A> for bool {
    #[inline(always)]
    fn from(variant: NAKSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl NAKSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NAKSTS_A {
        match self.bits {
            false => NAKSTS_A::VALUE1,
            true => NAKSTS_A::VALUE2,
        }
    }
    #[doc = "The core is transmitting non-NAK handshakes based on the FIFO status"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NAKSTS_A::VALUE1
    }
    #[doc = "The core is transmitting NAK handshakes on this endpoint."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NAKSTS_A::VALUE2
    }
}
#[doc = "Field `EPType` reader - Endpoint Type"]
pub type EPTYPE_R = crate::FieldReader;
#[doc = "Field `Stall` reader - STALL Handshake"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `Stall` writer - STALL Handshake"]
pub type STALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TxFNum` reader - TxFIFO Number"]
pub type TX_FNUM_R = crate::FieldReader;
#[doc = "Field `TxFNum` writer - TxFIFO Number"]
pub type TX_FNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPDis` reader - Endpoint Disable"]
pub type EPDIS_R = crate::BitReader;
#[doc = "Field `EPDis` writer - Endpoint Disable"]
pub type EPDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPEna` reader - Endpoint Enable"]
pub type EPENA_R = crate::BitReader;
#[doc = "Field `EPEna` writer - Endpoint Enable"]
pub type EPENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbact_ep(&self) -> USBACT_EP_R {
        USBACT_EP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK Status"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - STALL Handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    pub fn tx_fnum(&self) -> TX_FNUM_R {
        TX_FNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Maximum Packet Size"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MPS_W<DIEPCTL0_SPEC, 0> {
        MPS_W::new(self)
    }
    #[doc = "Bit 21 - STALL Handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<DIEPCTL0_SPEC, 21> {
        STALL_W::new(self)
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fnum(&mut self) -> TX_FNUM_W<DIEPCTL0_SPEC, 22> {
        TX_FNUM_W::new(self)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<DIEPCTL0_SPEC, 26> {
        CNAK_W::new(self)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<DIEPCTL0_SPEC, 27> {
        SNAK_W::new(self)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    #[must_use]
    pub fn epdis(&mut self) -> EPDIS_W<DIEPCTL0_SPEC, 30> {
        EPDIS_W::new(self)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EPENA_W<DIEPCTL0_SPEC, 31> {
        EPENA_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device Control IN Endpoint 0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPCTL0_SPEC;
impl crate::RegisterSpec for DIEPCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepctl0::R`](R) reader structure"]
impl crate::Readable for DIEPCTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepctl0::W`](W) writer structure"]
impl crate::Writable for DIEPCTL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPCTL0 to value 0x8000"]
impl crate::Resettable for DIEPCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
