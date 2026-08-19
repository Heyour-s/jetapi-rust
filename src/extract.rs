//! Наши обёртки над экстракторами и типами из poem::web.

pub use poem::web::{
    Accept as JetAccept,
    Compress as JetCompress,
    CsrfToken as JetCsrfToken,
    CsrfVerifier as JetCsrfVerifier,
    Data as JetData,
    Field as JetField,
    Form as JetForm,
    Html as JetHtml,
    Json as JetJson,
    LocalAddr as JetLocalAddr,
    Multipart as JetMultipart,
    Path as JetPath,
    Query as JetQuery,
    RealIp as JetRealIp,
    Redirect as JetRedirect,
    RemoteAddr as JetRemoteAddr,
    RequestBody as JetRequestBody,
    StaticFileRequest as JetStaticFileRequest,
    TempFile as JetTempFile,
    TypedHeader as JetTypedHeader,
    WithBody as JetWithBody,
    WithContentType as JetWithContentType,
    WithHeader as JetWithHeader,
    WithStatus as JetWithStatus,
    Xml as JetXml,
    Yaml as JetYaml,
};

// SSE и WebSocket находятся в подмодулях
pub use poem::web::sse::{Sse as JetSse, Event as JetEvent};
pub use poem::web::websocket::{WebSocket as JetWebsocket};