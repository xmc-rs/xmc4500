#[doc = "Register `EXICON[%s]` reader"]
pub struct R(crate::R<EXICON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXICON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXICON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXICON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXICON[%s]` writer"]
pub struct W(crate::W<EXICON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXICON_SPEC>;
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
impl From<crate::W<EXICON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXICON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PE` reader - Output Trigger Pulse Enable for ETLx"]
pub type PE_R = crate::BitReader<PE_A>;
#[doc = "Output Trigger Pulse Enable for ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE_A {
    #[doc = "0: The trigger pulse generation is disabled"]
    VALUE1 = 0,
    #[doc = "1: The trigger pulse generation is enabled"]
    VALUE2 = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
impl PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::VALUE1,
            true => PE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PE_A::VALUE2
    }
}
#[doc = "Field `PE` writer - Output Trigger Pulse Enable for ETLx"]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXICON_SPEC, PE_A, O>;
impl<'a, const O: u8> PE_W<'a, O> {
    #[doc = "The trigger pulse generation is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PE_A::VALUE1)
    }
    #[doc = "The trigger pulse generation is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PE_A::VALUE2)
    }
}
#[doc = "Field `LD` reader - Rebuild Level Detection for Status Flag for ETLx"]
pub type LD_R = crate::BitReader<LD_A>;
#[doc = "Rebuild Level Detection for Status Flag for ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LD_A {
    #[doc = "0: The status flag FL is not cleared by hardware and is used as \"sticky\" bit. Once set, it is not influenced by any edge until it becomes cleared by software."]
    VALUE1 = 0,
    #[doc = "1: The status flag FL rebuilds a level detection of the desired event. It becomes automatically set with a rising edge if RE = 1 or with a falling edge if FE = 1. It becomes automatically cleared with a rising edge if RE = 0 or with a falling edge if FE = 0."]
    VALUE2 = 1,
}
impl From<LD_A> for bool {
    #[inline(always)]
    fn from(variant: LD_A) -> Self {
        variant as u8 != 0
    }
}
impl LD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LD_A {
        match self.bits {
            false => LD_A::VALUE1,
            true => LD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LD_A::VALUE2
    }
}
#[doc = "Field `LD` writer - Rebuild Level Detection for Status Flag for ETLx"]
pub type LD_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXICON_SPEC, LD_A, O>;
impl<'a, const O: u8> LD_W<'a, O> {
    #[doc = "The status flag FL is not cleared by hardware and is used as \"sticky\" bit. Once set, it is not influenced by any edge until it becomes cleared by software."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LD_A::VALUE1)
    }
    #[doc = "The status flag FL rebuilds a level detection of the desired event. It becomes automatically set with a rising edge if RE = 1 or with a falling edge if FE = 1. It becomes automatically cleared with a rising edge if RE = 0 or with a falling edge if FE = 0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LD_A::VALUE2)
    }
}
#[doc = "Field `RE` reader - Rising Edge Detection Enable ETLx"]
pub type RE_R = crate::BitReader<RE_A>;
#[doc = "Rising Edge Detection Enable ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_A {
    #[doc = "0: A rising edge is not considered as edge event"]
    VALUE1 = 0,
    #[doc = "1: A rising edge is considered as edge event"]
    VALUE2 = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
impl RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::VALUE1,
            true => RE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RE_A::VALUE2
    }
}
#[doc = "Field `RE` writer - Rising Edge Detection Enable ETLx"]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXICON_SPEC, RE_A, O>;
impl<'a, const O: u8> RE_W<'a, O> {
    #[doc = "A rising edge is not considered as edge event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RE_A::VALUE1)
    }
    #[doc = "A rising edge is considered as edge event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RE_A::VALUE2)
    }
}
#[doc = "Field `FE` reader - Falling Edge Detection Enable ETLx"]
pub type FE_R = crate::BitReader<FE_A>;
#[doc = "Falling Edge Detection Enable ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE_A {
    #[doc = "0: A falling edge is not considered as edge event"]
    VALUE1 = 0,
    #[doc = "1: A falling edge is considered as edge event"]
    VALUE2 = 1,
}
impl From<FE_A> for bool {
    #[inline(always)]
    fn from(variant: FE_A) -> Self {
        variant as u8 != 0
    }
}
impl FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FE_A {
        match self.bits {
            false => FE_A::VALUE1,
            true => FE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FE_A::VALUE2
    }
}
#[doc = "Field `FE` writer - Falling Edge Detection Enable ETLx"]
pub type FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXICON_SPEC, FE_A, O>;
impl<'a, const O: u8> FE_W<'a, O> {
    #[doc = "A falling edge is not considered as edge event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FE_A::VALUE1)
    }
    #[doc = "A falling edge is considered as edge event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FE_A::VALUE2)
    }
}
#[doc = "Field `OCS` reader - Output Channel Select for ETLx Output Trigger Pulse"]
pub type OCS_R = crate::FieldReader<u8, OCS_A>;
#[doc = "Output Channel Select for ETLx Output Trigger Pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OCS_A {
    #[doc = "0: Trigger pulses are sent to OGU0"]
    VALUE1 = 0,
    #[doc = "1: Trigger pulses are sent to OGU1"]
    VALUE2 = 1,
    #[doc = "2: Trigger pulses are sent to OGU2"]
    VALUE3 = 2,
    #[doc = "3: Trigger pulses are sent to OGU3"]
    VALUE4 = 3,
}
impl From<OCS_A> for u8 {
    #[inline(always)]
    fn from(variant: OCS_A) -> Self {
        variant as _
    }
}
impl OCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OCS_A> {
        match self.bits {
            0 => Some(OCS_A::VALUE1),
            1 => Some(OCS_A::VALUE2),
            2 => Some(OCS_A::VALUE3),
            3 => Some(OCS_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == OCS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == OCS_A::VALUE4
    }
}
#[doc = "Field `OCS` writer - Output Channel Select for ETLx Output Trigger Pulse"]
pub type OCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXICON_SPEC, u8, OCS_A, 3, O>;
impl<'a, const O: u8> OCS_W<'a, O> {
    #[doc = "Trigger pulses are sent to OGU0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS_A::VALUE1)
    }
    #[doc = "Trigger pulses are sent to OGU1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS_A::VALUE2)
    }
    #[doc = "Trigger pulses are sent to OGU2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(OCS_A::VALUE3)
    }
    #[doc = "Trigger pulses are sent to OGU3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(OCS_A::VALUE4)
    }
}
#[doc = "Field `FL` reader - Status Flag for ETLx"]
pub type FL_R = crate::BitReader<FL_A>;
#[doc = "Status Flag for ETLx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FL_A {
    #[doc = "0: The enabled edge event has not been detected"]
    VALUE1 = 0,
    #[doc = "1: The enabled edge event has been detected"]
    VALUE2 = 1,
}
impl From<FL_A> for bool {
    #[inline(always)]
    fn from(variant: FL_A) -> Self {
        variant as u8 != 0
    }
}
impl FL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FL_A {
        match self.bits {
            false => FL_A::VALUE1,
            true => FL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FL_A::VALUE2
    }
}
#[doc = "Field `FL` writer - Status Flag for ETLx"]
pub type FL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXICON_SPEC, FL_A, O>;
impl<'a, const O: u8> FL_W<'a, O> {
    #[doc = "The enabled edge event has not been detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FL_A::VALUE1)
    }
    #[doc = "The enabled edge event has been detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FL_A::VALUE2)
    }
}
#[doc = "Field `SS` reader - Input Source Select for ERSx"]
pub type SS_R = crate::FieldReader<u8, SS_A>;
#[doc = "Input Source Select for ERSx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SS_A {
    #[doc = "0: Input A without additional combination"]
    VALUE1 = 0,
    #[doc = "1: Input B without additional combination"]
    VALUE2 = 1,
    #[doc = "2: Input A OR input B"]
    VALUE3 = 2,
    #[doc = "3: Input A AND input B"]
    VALUE4 = 3,
}
impl From<SS_A> for u8 {
    #[inline(always)]
    fn from(variant: SS_A) -> Self {
        variant as _
    }
}
impl SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_A {
        match self.bits {
            0 => SS_A::VALUE1,
            1 => SS_A::VALUE2,
            2 => SS_A::VALUE3,
            3 => SS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SS_A::VALUE4
    }
}
#[doc = "Field `SS` writer - Input Source Select for ERSx"]
pub type SS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, EXICON_SPEC, u8, SS_A, 2, O>;
impl<'a, const O: u8> SS_W<'a, O> {
    #[doc = "Input A without additional combination"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SS_A::VALUE1)
    }
    #[doc = "Input B without additional combination"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SS_A::VALUE2)
    }
    #[doc = "Input A OR input B"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SS_A::VALUE3)
    }
    #[doc = "Input A AND input B"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SS_A::VALUE4)
    }
}
#[doc = "Field `NA` reader - Input A Negation Select for ERSx"]
pub type NA_R = crate::BitReader<NA_A>;
#[doc = "Input A Negation Select for ERSx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NA_A {
    #[doc = "0: Input A is used directly"]
    VALUE1 = 0,
    #[doc = "1: Input A is inverted"]
    VALUE2 = 1,
}
impl From<NA_A> for bool {
    #[inline(always)]
    fn from(variant: NA_A) -> Self {
        variant as u8 != 0
    }
}
impl NA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NA_A {
        match self.bits {
            false => NA_A::VALUE1,
            true => NA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NA_A::VALUE2
    }
}
#[doc = "Field `NA` writer - Input A Negation Select for ERSx"]
pub type NA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXICON_SPEC, NA_A, O>;
impl<'a, const O: u8> NA_W<'a, O> {
    #[doc = "Input A is used directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NA_A::VALUE1)
    }
    #[doc = "Input A is inverted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NA_A::VALUE2)
    }
}
#[doc = "Field `NB` reader - Input B Negation Select for ERSx"]
pub type NB_R = crate::BitReader<NB_A>;
#[doc = "Input B Negation Select for ERSx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NB_A {
    #[doc = "0: Input B is used directly"]
    VALUE1 = 0,
    #[doc = "1: Input B is inverted"]
    VALUE2 = 1,
}
impl From<NB_A> for bool {
    #[inline(always)]
    fn from(variant: NB_A) -> Self {
        variant as u8 != 0
    }
}
impl NB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NB_A {
        match self.bits {
            false => NB_A::VALUE1,
            true => NB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NB_A::VALUE2
    }
}
#[doc = "Field `NB` writer - Input B Negation Select for ERSx"]
pub type NB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXICON_SPEC, NB_A, O>;
impl<'a, const O: u8> NB_W<'a, O> {
    #[doc = "Input B is used directly"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NB_A::VALUE1)
    }
    #[doc = "Input B is inverted"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NB_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Output Trigger Pulse Enable for ETLx"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rebuild Level Detection for Status Flag for ETLx"]
    #[inline(always)]
    pub fn ld(&self) -> LD_R {
        LD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising Edge Detection Enable ETLx"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling Edge Detection Enable ETLx"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output Channel Select for ETLx Output Trigger Pulse"]
    #[inline(always)]
    pub fn ocs(&self) -> OCS_R {
        OCS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Status Flag for ETLx"]
    #[inline(always)]
    pub fn fl(&self) -> FL_R {
        FL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Input Source Select for ERSx"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Input A Negation Select for ERSx"]
    #[inline(always)]
    pub fn na(&self) -> NA_R {
        NA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input B Negation Select for ERSx"]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Trigger Pulse Enable for ETLx"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<0> {
        PE_W::new(self)
    }
    #[doc = "Bit 1 - Rebuild Level Detection for Status Flag for ETLx"]
    #[inline(always)]
    #[must_use]
    pub fn ld(&mut self) -> LD_W<1> {
        LD_W::new(self)
    }
    #[doc = "Bit 2 - Rising Edge Detection Enable ETLx"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<2> {
        RE_W::new(self)
    }
    #[doc = "Bit 3 - Falling Edge Detection Enable ETLx"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FE_W<3> {
        FE_W::new(self)
    }
    #[doc = "Bits 4:6 - Output Channel Select for ETLx Output Trigger Pulse"]
    #[inline(always)]
    #[must_use]
    pub fn ocs(&mut self) -> OCS_W<4> {
        OCS_W::new(self)
    }
    #[doc = "Bit 7 - Status Flag for ETLx"]
    #[inline(always)]
    #[must_use]
    pub fn fl(&mut self) -> FL_W<7> {
        FL_W::new(self)
    }
    #[doc = "Bits 8:9 - Input Source Select for ERSx"]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<8> {
        SS_W::new(self)
    }
    #[doc = "Bit 10 - Input A Negation Select for ERSx"]
    #[inline(always)]
    #[must_use]
    pub fn na(&mut self) -> NA_W<10> {
        NA_W::new(self)
    }
    #[doc = "Bit 11 - Input B Negation Select for ERSx"]
    #[inline(always)]
    #[must_use]
    pub fn nb(&mut self) -> NB_W<11> {
        NB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Input Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exicon](index.html) module"]
pub struct EXICON_SPEC;
impl crate::RegisterSpec for EXICON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exicon::R](R) reader structure"]
impl crate::Readable for EXICON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exicon::W](W) writer structure"]
impl crate::Writable for EXICON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXICON[%s]
to value 0"]
impl crate::Resettable for EXICON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
