#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug, Hash)]
pub enum UploadRegion {
    AsiaMumbai,
    AsiaSeoul,
    AsiaSydney,
    CanadaCentral,
    EuCentralFrankfurt,
    EuCentralZurich,
    EuWestDublin,
    UsEastOhio,
    UsWestSanFrancisco,
    UsWestSeattle,
}
impl Default for UploadRegion {
    fn default() -> Self {
        Self::UsWestSeattle
    }
}
impl UploadRegion {
    pub fn alias(&self) -> &'static str {
        match self {
            Self::AsiaMumbai => "bom1",
            Self::AsiaSeoul => "icn1",
            Self::AsiaSydney => "syd1",
            Self::CanadaCentral => "can1",
            Self::EuCentralFrankfurt => "fra1",
            Self::EuCentralZurich => "zrh1",
            Self::EuWestDublin => "dub1",
            Self::UsEastOhio => "cle1",
            Self::UsWestSanFrancisco => "sfo1",
            Self::UsWestSeattle => "sea1",
        }
    }
}

impl Into<&str> for UploadRegion {
    fn into(self) -> &'static str {
        match self {
            Self::AsiaMumbai => "asia-mumbai",
            Self::AsiaSeoul => "asia-seoul",
            Self::AsiaSydney => "asia-sydney",
            Self::CanadaCentral => "canada-central",
            Self::EuCentralFrankfurt => "eu-central-frankfurt",
            Self::EuCentralZurich => "eu-central-zurich",
            Self::EuWestDublin => "eu-west-dublin",
            Self::UsEastOhio => "us-east-ohio",
            Self::UsWestSanFrancisco => "us-west-san-francisco",
            Self::UsWestSeattle => "us-west-seattle",
        }
    }
}
