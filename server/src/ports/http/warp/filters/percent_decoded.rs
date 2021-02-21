use std::convert::Infallible;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub(crate) struct PercentDecoded {
    s: String,
}

impl FromStr for PercentDecoded {
    type Err = Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = percent_encoding::percent_decode_str(s)
            .decode_utf8_lossy()
            .to_string();
        Ok(PercentDecoded { s })
    }
}

impl ToString for PercentDecoded {
    #[inline]
    fn to_string(&self) -> String {
        self.s.clone()
    }
}
