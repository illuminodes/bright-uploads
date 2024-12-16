pub enum UtRoute {
    PresignedUrl,
}
impl Into<&str> for UtRoute {
    fn into(self) -> &'static str {
        match self {
            UtRoute::PresignedUrl=> "/api/uploadthing/presigned-url",
        }
    }
}
