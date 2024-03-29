#[doc = "Register `PMTSR` reader"]
pub type R = crate::R<PmtsrSpec>;
#[doc = "Register `PMTSR` writer"]
pub type W = crate::W<PmtsrSpec>;
#[doc = "Test Enable Control for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mtenps {
    #[doc = "0: Standard operation"]
    Value1 = 0,
    #[doc = "1: Parity bits under test"]
    Value2 = 1,
}
impl From<Mtenps> for bool {
    #[inline(always)]
    fn from(variant: Mtenps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTENPS` reader - Test Enable Control for PSRAM"]
pub type MtenpsR = crate::BitReader<Mtenps>;
impl MtenpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mtenps {
        match self.bits {
            false => Mtenps::Value1,
            true => Mtenps::Value2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mtenps::Value1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mtenps::Value2
    }
}
#[doc = "Field `MTENPS` writer - Test Enable Control for PSRAM"]
pub type MtenpsW<'a, REG> = crate::BitWriter<'a, REG, Mtenps>;
impl<'a, REG> MtenpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mtenps::Value1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mtenps::Value2)
    }
}
#[doc = "Test Enable Control for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mtends1 {
    #[doc = "0: Standard operation"]
    Value1 = 0,
    #[doc = "1: Parity bits under test"]
    Value2 = 1,
}
impl From<Mtends1> for bool {
    #[inline(always)]
    fn from(variant: Mtends1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTENDS1` reader - Test Enable Control for DSRAM1"]
pub type Mtends1R = crate::BitReader<Mtends1>;
impl Mtends1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mtends1 {
        match self.bits {
            false => Mtends1::Value1,
            true => Mtends1::Value2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mtends1::Value1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mtends1::Value2
    }
}
#[doc = "Field `MTENDS1` writer - Test Enable Control for DSRAM1"]
pub type Mtends1W<'a, REG> = crate::BitWriter<'a, REG, Mtends1>;
impl<'a, REG> Mtends1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mtends1::Value1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mtends1::Value2)
    }
}
#[doc = "Test Enable Control for DSRAM2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mtends2 {
    #[doc = "0: Standard operation"]
    Value1 = 0,
    #[doc = "1: Parity bits under test"]
    Value2 = 1,
}
impl From<Mtends2> for bool {
    #[inline(always)]
    fn from(variant: Mtends2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTENDS2` reader - Test Enable Control for DSRAM2"]
pub type Mtends2R = crate::BitReader<Mtends2>;
impl Mtends2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mtends2 {
        match self.bits {
            false => Mtends2::Value1,
            true => Mtends2::Value2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mtends2::Value1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mtends2::Value2
    }
}
#[doc = "Field `MTENDS2` writer - Test Enable Control for DSRAM2"]
pub type Mtends2W<'a, REG> = crate::BitWriter<'a, REG, Mtends2>;
impl<'a, REG> Mtends2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mtends2::Value1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mtends2::Value2)
    }
}
#[doc = "Test Enable Control for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mteu0 {
    #[doc = "0: Standard operation"]
    Value1 = 0,
    #[doc = "1: Parity bits under test"]
    Value2 = 1,
}
impl From<Mteu0> for bool {
    #[inline(always)]
    fn from(variant: Mteu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTEU0` reader - Test Enable Control for USIC0 Memory"]
pub type Mteu0R = crate::BitReader<Mteu0>;
impl Mteu0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mteu0 {
        match self.bits {
            false => Mteu0::Value1,
            true => Mteu0::Value2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mteu0::Value1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mteu0::Value2
    }
}
#[doc = "Field `MTEU0` writer - Test Enable Control for USIC0 Memory"]
pub type Mteu0W<'a, REG> = crate::BitWriter<'a, REG, Mteu0>;
impl<'a, REG> Mteu0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mteu0::Value1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mteu0::Value2)
    }
}
#[doc = "Test Enable Control for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mteu1 {
    #[doc = "0: Standard operation"]
    Value1 = 0,
    #[doc = "1: Parity bits under test"]
    Value2 = 1,
}
impl From<Mteu1> for bool {
    #[inline(always)]
    fn from(variant: Mteu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTEU1` reader - Test Enable Control for USIC1 Memory"]
pub type Mteu1R = crate::BitReader<Mteu1>;
impl Mteu1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mteu1 {
        match self.bits {
            false => Mteu1::Value1,
            true => Mteu1::Value2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mteu1::Value1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mteu1::Value2
    }
}
#[doc = "Field `MTEU1` writer - Test Enable Control for USIC1 Memory"]
pub type Mteu1W<'a, REG> = crate::BitWriter<'a, REG, Mteu1>;
impl<'a, REG> Mteu1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mteu1::Value1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mteu1::Value2)
    }
}
#[doc = "Test Enable Control for USIC2 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mteu2 {
    #[doc = "0: Standard operation"]
    Value1 = 0,
    #[doc = "1: Parity bits under test"]
    Value2 = 1,
}
impl From<Mteu2> for bool {
    #[inline(always)]
    fn from(variant: Mteu2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTEU2` reader - Test Enable Control for USIC2 Memory"]
pub type Mteu2R = crate::BitReader<Mteu2>;
impl Mteu2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mteu2 {
        match self.bits {
            false => Mteu2::Value1,
            true => Mteu2::Value2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mteu2::Value1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mteu2::Value2
    }
}
#[doc = "Field `MTEU2` writer - Test Enable Control for USIC2 Memory"]
pub type Mteu2W<'a, REG> = crate::BitWriter<'a, REG, Mteu2>;
impl<'a, REG> Mteu2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mteu2::Value1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mteu2::Value2)
    }
}
#[doc = "Test Enable Control for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mtemc {
    #[doc = "0: Standard operation"]
    Value1 = 0,
    #[doc = "1: Parity bits under test"]
    Value2 = 1,
}
impl From<Mtemc> for bool {
    #[inline(always)]
    fn from(variant: Mtemc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTEMC` reader - Test Enable Control for MultiCAN Memory"]
pub type MtemcR = crate::BitReader<Mtemc>;
impl MtemcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mtemc {
        match self.bits {
            false => Mtemc::Value1,
            true => Mtemc::Value2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mtemc::Value1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mtemc::Value2
    }
}
#[doc = "Field `MTEMC` writer - Test Enable Control for MultiCAN Memory"]
pub type MtemcW<'a, REG> = crate::BitWriter<'a, REG, Mtemc>;
impl<'a, REG> MtemcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mtemc::Value1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mtemc::Value2)
    }
}
#[doc = "Test Enable Control for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mtepprf {
    #[doc = "0: Standard operation"]
    Value1 = 0,
    #[doc = "1: Parity bits under test"]
    Value2 = 1,
}
impl From<Mtepprf> for bool {
    #[inline(always)]
    fn from(variant: Mtepprf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTEPPRF` reader - Test Enable Control for PMU Prefetch Memory"]
pub type MtepprfR = crate::BitReader<Mtepprf>;
impl MtepprfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mtepprf {
        match self.bits {
            false => Mtepprf::Value1,
            true => Mtepprf::Value2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mtepprf::Value1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mtepprf::Value2
    }
}
#[doc = "Field `MTEPPRF` writer - Test Enable Control for PMU Prefetch Memory"]
pub type MtepprfW<'a, REG> = crate::BitWriter<'a, REG, Mtepprf>;
impl<'a, REG> MtepprfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mtepprf::Value1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mtepprf::Value2)
    }
}
#[doc = "Test Enable Control for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mtusb {
    #[doc = "0: Standard operation"]
    Value1 = 0,
    #[doc = "1: Parity bits under test"]
    Value2 = 1,
}
impl From<Mtusb> for bool {
    #[inline(always)]
    fn from(variant: Mtusb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTUSB` reader - Test Enable Control for USB Memory"]
pub type MtusbR = crate::BitReader<Mtusb>;
impl MtusbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mtusb {
        match self.bits {
            false => Mtusb::Value1,
            true => Mtusb::Value2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mtusb::Value1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mtusb::Value2
    }
}
#[doc = "Field `MTUSB` writer - Test Enable Control for USB Memory"]
pub type MtusbW<'a, REG> = crate::BitWriter<'a, REG, Mtusb>;
impl<'a, REG> MtusbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mtusb::Value1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mtusb::Value2)
    }
}
#[doc = "Test Enable Control for ETH TX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mteth0tx {
    #[doc = "0: Standard operation"]
    Value1 = 0,
    #[doc = "1: Parity bits under test"]
    Value2 = 1,
}
impl From<Mteth0tx> for bool {
    #[inline(always)]
    fn from(variant: Mteth0tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTETH0TX` reader - Test Enable Control for ETH TX Memory"]
pub type Mteth0txR = crate::BitReader<Mteth0tx>;
impl Mteth0txR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mteth0tx {
        match self.bits {
            false => Mteth0tx::Value1,
            true => Mteth0tx::Value2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mteth0tx::Value1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mteth0tx::Value2
    }
}
#[doc = "Field `MTETH0TX` writer - Test Enable Control for ETH TX Memory"]
pub type Mteth0txW<'a, REG> = crate::BitWriter<'a, REG, Mteth0tx>;
impl<'a, REG> Mteth0txW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mteth0tx::Value1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mteth0tx::Value2)
    }
}
#[doc = "Test Enable Control for ETH RX Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mteth0rx {
    #[doc = "0: Standard operation"]
    Value1 = 0,
    #[doc = "1: Parity bits under test"]
    Value2 = 1,
}
impl From<Mteth0rx> for bool {
    #[inline(always)]
    fn from(variant: Mteth0rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTETH0RX` reader - Test Enable Control for ETH RX Memory"]
pub type Mteth0rxR = crate::BitReader<Mteth0rx>;
impl Mteth0rxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mteth0rx {
        match self.bits {
            false => Mteth0rx::Value1,
            true => Mteth0rx::Value2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mteth0rx::Value1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mteth0rx::Value2
    }
}
#[doc = "Field `MTETH0RX` writer - Test Enable Control for ETH RX Memory"]
pub type Mteth0rxW<'a, REG> = crate::BitWriter<'a, REG, Mteth0rx>;
impl<'a, REG> Mteth0rxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mteth0rx::Value1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mteth0rx::Value2)
    }
}
#[doc = "Test Enable Control for SDMMC Memory 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mtsd0 {
    #[doc = "0: Standard operation"]
    Value1 = 0,
    #[doc = "1: Parity bits under test"]
    Value2 = 1,
}
impl From<Mtsd0> for bool {
    #[inline(always)]
    fn from(variant: Mtsd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTSD0` reader - Test Enable Control for SDMMC Memory 0"]
pub type Mtsd0R = crate::BitReader<Mtsd0>;
impl Mtsd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mtsd0 {
        match self.bits {
            false => Mtsd0::Value1,
            true => Mtsd0::Value2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mtsd0::Value1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mtsd0::Value2
    }
}
#[doc = "Field `MTSD0` writer - Test Enable Control for SDMMC Memory 0"]
pub type Mtsd0W<'a, REG> = crate::BitWriter<'a, REG, Mtsd0>;
impl<'a, REG> Mtsd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mtsd0::Value1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mtsd0::Value2)
    }
}
#[doc = "Test Enable Control for SDMMC Memory 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mtsd1 {
    #[doc = "0: Standard operation"]
    Value1 = 0,
    #[doc = "1: Parity bits under test"]
    Value2 = 1,
}
impl From<Mtsd1> for bool {
    #[inline(always)]
    fn from(variant: Mtsd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTSD1` reader - Test Enable Control for SDMMC Memory 1"]
pub type Mtsd1R = crate::BitReader<Mtsd1>;
impl Mtsd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mtsd1 {
        match self.bits {
            false => Mtsd1::Value1,
            true => Mtsd1::Value2,
        }
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mtsd1::Value1
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mtsd1::Value2
    }
}
#[doc = "Field `MTSD1` writer - Test Enable Control for SDMMC Memory 1"]
pub type Mtsd1W<'a, REG> = crate::BitWriter<'a, REG, Mtsd1>;
impl<'a, REG> Mtsd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mtsd1::Value1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mtsd1::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Test Enable Control for PSRAM"]
    #[inline(always)]
    pub fn mtenps(&self) -> MtenpsR {
        MtenpsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Test Enable Control for DSRAM1"]
    #[inline(always)]
    pub fn mtends1(&self) -> Mtends1R {
        Mtends1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Test Enable Control for DSRAM2"]
    #[inline(always)]
    pub fn mtends2(&self) -> Mtends2R {
        Mtends2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Test Enable Control for USIC0 Memory"]
    #[inline(always)]
    pub fn mteu0(&self) -> Mteu0R {
        Mteu0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Test Enable Control for USIC1 Memory"]
    #[inline(always)]
    pub fn mteu1(&self) -> Mteu1R {
        Mteu1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Test Enable Control for USIC2 Memory"]
    #[inline(always)]
    pub fn mteu2(&self) -> Mteu2R {
        Mteu2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Test Enable Control for MultiCAN Memory"]
    #[inline(always)]
    pub fn mtemc(&self) -> MtemcR {
        MtemcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Test Enable Control for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn mtepprf(&self) -> MtepprfR {
        MtepprfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Test Enable Control for USB Memory"]
    #[inline(always)]
    pub fn mtusb(&self) -> MtusbR {
        MtusbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Test Enable Control for ETH TX Memory"]
    #[inline(always)]
    pub fn mteth0tx(&self) -> Mteth0txR {
        Mteth0txR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Test Enable Control for ETH RX Memory"]
    #[inline(always)]
    pub fn mteth0rx(&self) -> Mteth0rxR {
        Mteth0rxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Test Enable Control for SDMMC Memory 0"]
    #[inline(always)]
    pub fn mtsd0(&self) -> Mtsd0R {
        Mtsd0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Test Enable Control for SDMMC Memory 1"]
    #[inline(always)]
    pub fn mtsd1(&self) -> Mtsd1R {
        Mtsd1R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Test Enable Control for PSRAM"]
    #[inline(always)]
    #[must_use]
    pub fn mtenps(&mut self) -> MtenpsW<PmtsrSpec> {
        MtenpsW::new(self, 0)
    }
    #[doc = "Bit 1 - Test Enable Control for DSRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn mtends1(&mut self) -> Mtends1W<PmtsrSpec> {
        Mtends1W::new(self, 1)
    }
    #[doc = "Bit 2 - Test Enable Control for DSRAM2"]
    #[inline(always)]
    #[must_use]
    pub fn mtends2(&mut self) -> Mtends2W<PmtsrSpec> {
        Mtends2W::new(self, 2)
    }
    #[doc = "Bit 8 - Test Enable Control for USIC0 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mteu0(&mut self) -> Mteu0W<PmtsrSpec> {
        Mteu0W::new(self, 8)
    }
    #[doc = "Bit 9 - Test Enable Control for USIC1 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mteu1(&mut self) -> Mteu1W<PmtsrSpec> {
        Mteu1W::new(self, 9)
    }
    #[doc = "Bit 10 - Test Enable Control for USIC2 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mteu2(&mut self) -> Mteu2W<PmtsrSpec> {
        Mteu2W::new(self, 10)
    }
    #[doc = "Bit 12 - Test Enable Control for MultiCAN Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mtemc(&mut self) -> MtemcW<PmtsrSpec> {
        MtemcW::new(self, 12)
    }
    #[doc = "Bit 13 - Test Enable Control for PMU Prefetch Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mtepprf(&mut self) -> MtepprfW<PmtsrSpec> {
        MtepprfW::new(self, 13)
    }
    #[doc = "Bit 16 - Test Enable Control for USB Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mtusb(&mut self) -> MtusbW<PmtsrSpec> {
        MtusbW::new(self, 16)
    }
    #[doc = "Bit 17 - Test Enable Control for ETH TX Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mteth0tx(&mut self) -> Mteth0txW<PmtsrSpec> {
        Mteth0txW::new(self, 17)
    }
    #[doc = "Bit 18 - Test Enable Control for ETH RX Memory"]
    #[inline(always)]
    #[must_use]
    pub fn mteth0rx(&mut self) -> Mteth0rxW<PmtsrSpec> {
        Mteth0rxW::new(self, 18)
    }
    #[doc = "Bit 19 - Test Enable Control for SDMMC Memory 0"]
    #[inline(always)]
    #[must_use]
    pub fn mtsd0(&mut self) -> Mtsd0W<PmtsrSpec> {
        Mtsd0W::new(self, 19)
    }
    #[doc = "Bit 20 - Test Enable Control for SDMMC Memory 1"]
    #[inline(always)]
    #[must_use]
    pub fn mtsd1(&mut self) -> Mtsd1W<PmtsrSpec> {
        Mtsd1W::new(self, 20)
    }
}
#[doc = "Parity Memory Test Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmtsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmtsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmtsrSpec;
impl crate::RegisterSpec for PmtsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmtsr::R`](R) reader structure"]
impl crate::Readable for PmtsrSpec {}
#[doc = "`write(|w| ..)` method takes [`pmtsr::W`](W) writer structure"]
impl crate::Writable for PmtsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMTSR to value 0"]
impl crate::Resettable for PmtsrSpec {
    const RESET_VALUE: u32 = 0;
}
