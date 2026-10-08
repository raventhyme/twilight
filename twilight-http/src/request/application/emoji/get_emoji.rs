use crate::{
    Client, Error, Response,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Emoji,
    id::{
        Id,
        marker::{ApplicationMarker, EmojiMarker},
    },
};

/// Get an emoji owned by an application.
///
/// The [`Emoji::user`] field is included in the emoji.
///
/// [`Emoji::user`]: ::twilight_model::guild::Emoji::user
#[must_use = "requests must be configured and executed"]
pub struct GetApplicationEmoji<'a> {
    application_id: Id<ApplicationMarker>,
    emoji_id: Id<EmojiMarker>,
    http: &'a Client,
}

impl<'a> GetApplicationEmoji<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        emoji_id: Id<EmojiMarker>,
    ) -> Self {
        Self {
            application_id,
            emoji_id,
            http,
        }
    }
}

impl IntoFuture for GetApplicationEmoji<'_> {
    type Output = Result<Response<Emoji>, Error>;

    type IntoFuture = ResponseFuture<Emoji>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetApplicationEmoji<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetApplicationEmoji {
            application_id: self.application_id.get(),
            emoji_id: self.emoji_id.get(),
        }))
    }
}
