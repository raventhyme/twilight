use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::EmptyBody},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::ChannelMarker};
use twilight_validate::{
    channel::{ChannelValidationError, voice_channel_status as validate_voice_channel_status},
    request::{ValidationError, audit_reason as validate_audit_reason},
};

// The Discord API doesn't require the `name` and `kind` fields to be present,
// but it does require them to be non-null.
#[derive(Serialize)]
struct SetVoiceChannelStatusFields<'a> {
    status: &'a str,
}

/// Update a channel.
///
/// The voice channel status must have a length of at most 500 characters.
///
/// # Errors
///
/// Returns an error of type [`ChannelValidationErrorType::VoiceChannelStatusInvalid`] if
/// the length of the voice channel status is greater than 500 characters.
///
/// [`ChannelValidationErrorType::VoiceChannelStatusInvalid`]: twilight_validate::channel::ChannelValidationErrorType::VoiceChannelStatusInvalid
#[must_use = "requests must be configured and executed"]
pub struct SetVoiceChannelStatus<'a> {
    channel_id: Id<ChannelMarker>,
    fields: Result<SetVoiceChannelStatusFields<'a>, ChannelValidationError>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> SetVoiceChannelStatus<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: Id<ChannelMarker>, status: &'a str) -> Self {
        let fields = Ok(SetVoiceChannelStatusFields { status }).and_then(|fields| {
            validate_voice_channel_status(status)?;

            Ok(fields)
        });

        Self {
            channel_id,
            fields,
            http,
            reason: Ok(None),
        }
    }
}

impl<'a> AuditLogReason<'a> for SetVoiceChannelStatus<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for SetVoiceChannelStatus<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for SetVoiceChannelStatus<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;
        let mut request = Request::builder(&Route::SetVoiceChannelStatus {
            channel_id: self.channel_id.get(),
        })
        .json(&fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
