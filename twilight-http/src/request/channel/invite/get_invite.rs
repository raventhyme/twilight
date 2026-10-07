use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::invite::Invite,
    id::{Id, marker::GuildMarker},
};

struct GetInviteFields {
    guild_scheduled_event_id: Option<Id<GuildMarker>>,
    with_counts: bool,
}

/// Get information about an invite by its code.
///
/// If [`with_counts`] is called, the returned invite will contain approximate
/// member counts. If [`with_expiration`] is called, it will contain the
/// expiration date.
///
/// # Examples
///
/// ```no_run
/// use twilight_http::Client;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let invite = client.invite("code").with_counts().await?;
/// # Ok(()) }
/// ```
///
/// [`with_counts`]: Self::with_counts
/// [`with_expiration`]: Self::with_expiration
#[must_use = "requests must be configured and executed"]
pub struct GetInvite<'a> {
    code: &'a str,
    fields: GetInviteFields,
    http: &'a Client,
}

impl<'a> GetInvite<'a> {
    pub(crate) const fn new(http: &'a Client, code: &'a str) -> Self {
        Self {
            code,
            fields: GetInviteFields {
                guild_scheduled_event_id: None,
                with_counts: false,
            },
            http,
        }
    }

    /// Guild scheduled event to include with the invite.
    pub const fn guild_scheduled_event_id(
        mut self,
        guild_scheduled_event_id: Id<GuildMarker>,
    ) -> Self {
        self.fields.guild_scheduled_event_id = Some(guild_scheduled_event_id);

        self
    }

    /// Whether the invite returned should contain approximate member counts.
    pub const fn with_counts(mut self) -> Self {
        self.fields.with_counts = true;

        self
    }
}

impl IntoFuture for GetInvite<'_> {
    type Output = Result<Response<Invite>, Error>;

    type IntoFuture = ResponseFuture<Invite>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetInvite<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetInvite {
            code: self.code,
            guild_scheduled_event_id: self.fields.guild_scheduled_event_id.map(Id::get),
            with_counts: self.fields.with_counts,
        }))
    }
}
