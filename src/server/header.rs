#![allow(non_upper_case_globals)]
use crate::prelude::*;

use axum::{
    extract::FromRequestParts,
    http::{HeaderMap, request::Parts},
};

/// The HTTP header name
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Header(pub &'static str);

impl Header {
    // --- Authentication ---
    /// WWW-Authenticate - Defines the authentication method that should be used to access a resource.
    pub const WWWAuthenticate: Self = Header("WWW-Authenticate");
    /// Authorization - Contains the credentials to authenticate a user-agent with a server.
    pub const Authorization: Header = Header("Authorization");
    /// Proxy-Authenticate - Defines the authentication method that should be used to access a resource behind a proxy server.
    pub const ProxyAuthenticate: Header = Header("Proxy-Authenticate");
    /// Proxy-Authorization - Contains the credentials to authenticate a user agent with a proxy server.
    pub const ProxyAuthorization: Header = Header("Proxy-Authorization");

    // --- Caching ---
    /// Age - The time, in seconds, that the object has been in a proxy cache.
    pub const Age: Header = Header("Age");
    /// Cache-Control - Directives for caching mechanisms in both requests and responses.
    pub const CacheControl: Header = Header("Cache-Control");
    /// Clear-Site-Data - Clears browsing data (e.g., cookies, storage, cache) associated with the requesting website.
    pub const ClearSiteData: Header = Header("Clear-Site-Data");
    /// Expires - The date/time after which the response is considered stale.
    pub const Expires: Header = Header("Expires");
    /// No-Vary-Search - Specifies a set of rules that define how a URL's query parameters will affect cache matching. These rules dictate whether the same URL with different URL parameters should be saved as separate browser cache entries.
    pub const NoVarySearch: Header = Header("No-Vary-Search");

    // --- Conditionals ---
    /// Last-Modified - The last modification date of the resource, used to compare several versions of the same resource. It is less accurate than ETag, but easier to calculate in some environments. Conditional requests using If-Modified-Since and If-Unmodified-Since use this value to change the behavior of the request.
    pub const LastModified: Header = Header("Last-Modified");
    /// ETag - A unique string identifying the version of the resource. Conditional requests using If-Match and If-None-Match use this value to change the behavior of the request.
    pub const ETag: Header = Header("ETag");
    /// If-Match - Makes the request conditional, and applies the method only if the stored resource matches one of the given ETags.
    pub const IfMatch: Header = Header("If-Match");
    /// If-None-Match - Makes the request conditional, and applies the method only if the stored resource doesn't match any of the given ETags. This is used to update caches (for safe requests), or to prevent uploading a new resource when one already exists.
    pub const IfNoneMatch: Header = Header("If-None-Match");
    /// If-Modified-Since - Makes the request conditional, and expects the resource to be transmitted only if it has been modified after the given date. This is used to transmit data only when the cache is out of date.
    pub const IfModifiedSince: Header = Header("If-Modified-Since");
    /// If-Unmodified-Since - Makes the request conditional, and expects the resource to be transmitted only if it has not been modified after the given date. This ensures the coherence of a new fragment of a specific range with previous ones, or to implement an optimistic concurrency control system when modifying existing documents.
    pub const IfUnmodifiedSince: Header = Header("If-Unmodified-Since");
    /// Vary - Determines how to match request headers to decide whether a cached response can be used rather than requesting a fresh one from the origin server.
    pub const Vary: Header = Header("Vary");

    // --- Connection management ---
    /// Connection - Controls whether the network connection stays open after the current transaction finishes.
    pub const Connection: Header = Header("Connection");
    /// Keep-Alive - Controls how long a persistent connection should stay open.
    pub const KeepAlive: Header = Header("Keep-Alive");

    // --- Content negotiation ---
    /// Accept - Informs the server about the types of data that can be sent back.
    pub const Accept: Header = Header("Accept");
    /// Accept-Encoding - The encoding algorithm, usually a compression algorithm, that can be used on the resource sent back.
    pub const AcceptEncoding: Header = Header("Accept-Encoding");
    /// Accept-Language - Informs the server about the human language the server is expected to send back. This is a hint and is not necessarily under the full control of the user: the server should always pay attention not to override an explicit user choice (like selecting a language from a dropdown).
    pub const AcceptLanguage: Header = Header("Accept-Language");
    /// Accept-Patch - A request content negotiation response header that advertises which media type the server is able to understand in a PATCH request.
    pub const AcceptPatch: Header = Header("Accept-Patch");
    /// Accept-Post - A request content negotiation response header that advertises which media type the server is able to understand in a POST request.
    pub const AcceptPost: Header = Header("Accept-Post");

    // --- Controls ---
    /// Expect - Indicates expectations that need to be fulfilled by the server to properly handle the request.
    pub const Expect: Header = Header("Expect");
    /// Max-Forwards - When using TRACE, indicates the maximum number of hops the request can do before being reflected to the sender.
    pub const MaxForwards: Header = Header("Max-Forwards");

    // --- Cookies ---
    /// Cookie - Contains stored HTTP cookies previously sent by the server with the Set-Cookie header.
    pub const Cookie: Header = Header("Cookie");
    /// Set-Cookie - Send cookies from the server to the user-agent.
    pub const SetCookie: Header = Header("Set-Cookie");

    // --- CORS ---
    /// Access-Control-Allow-Credentials - Indicates whether the response to the request can be exposed when the credentials flag is true.
    pub const AccessControlAllowCredentials: Header = Header("Access-Control-Allow-Credentials");
    /// Access-Control-Allow-Headers - Used in response to a preflight request to indicate which HTTP headers can be used when making the actual request.
    pub const AccessControlAllowHeaders: Header = Header("Access-Control-Allow-Headers");
    /// Access-Control-Allow-Methods - Specifies the methods allowed when accessing the resource in response to a preflight request.
    pub const AccessControlAllowMethods: Header = Header("Access-Control-Allow-Methods");
    /// Access-Control-Allow-Origin - Indicates whether the response can be shared.
    pub const AccessControlAllowOrigin: Header = Header("Access-Control-Allow-Origin");
    /// Access-Control-Expose-Headers - Indicates which headers can be exposed as part of the response by listing their names.
    pub const AccessControlExposeHeaders: Header = Header("Access-Control-Expose-Headers");
    /// Access-Control-Max-Age - Indicates how long the results of a preflight request can be cached.
    pub const AccessControlMaxAge: Header = Header("Access-Control-Max-Age");
    /// Access-Control-Request-Headers - Used when issuing a preflight request to let the server know which HTTP headers will be used when the actual request is made.
    pub const AccessControlRequestHeaders: Header = Header("Access-Control-Request-Headers");
    /// Access-Control-Request-Method - Used when issuing a preflight request to let the server know which HTTP method will be used when the actual request is made.
    pub const AccessControlRequestMethod: Header = Header("Access-Control-Request-Method");
    /// Origin - Indicates where a fetch originates from.
    pub const Origin: Header = Header("Origin");
    /// Timing-Allow-Origin - Specifies origins that are allowed to see values of attributes retrieved via features of the Resource Timing API, which would otherwise be reported as zero due to cross-origin restrictions.
    pub const TimingAllowOrigin: Header = Header("Timing-Allow-Origin");

    // --- Downloads ---
    /// Content-Disposition - Indicates if the resource transmitted should be displayed inline (default behavior without the header), or if it should be handled like a download and the browser should present a "Save As" dialog.
    pub const ContentDisposition: Header = Header("Content-Disposition");

    // --- Integrity digests ---
    /// Content-Digest - Provides a digest of the stream of octets framed in an HTTP message (the message content) dependent on Content-Encoding and Content-Range.
    pub const ContentDigest: Header = Header("Content-Digest");
    /// Repr-Digest - Provides a digest of the selected representation of the target resource before transmission. Unlike the Content-Digest, the digest does not consider Content-Encoding or Content-Range.
    pub const ReprDigest: Header = Header("Repr-Digest");
    /// Want-Content-Digest - States the wish for a Content-Digest header. It is the Content- analogue of Want-Repr-Digest.
    pub const WantContentDigest: Header = Header("Want-Content-Digest");
    /// Want-Repr-Digest - States the wish for a Repr-Digest header. It is the Repr- analogue of Want-Content-Digest.
    pub const WantReprDigest: Header = Header("Want-Repr-Digest");

    // --- Integrity policy ---
    /// Integrity-Policy - Ensures that all resources the user agent loads (of a certain type) have Subresource Integrity guarantees.
    pub const IntegrityPolicy: Header = Header("Integrity-Policy");
    /// Integrity-Policy-Report-Only - Reports on resources that the user agent loads that would violate Subresource Integrity guarantees if the integrity policy were enforced (using the Integrity-Policy header).
    pub const IntegrityPolicyReportOnly: Header = Header("Integrity-Policy-Report-Only");

    // --- Message body information ---
    /// Content-Length - The size of the resource, in decimal number of bytes.
    pub const ContentLength: Header = Header("Content-Length");
    /// Content-Type - Indicates the media type of the resource.
    pub const ContentType: Header = Header("Content-Type");
    /// Content-Encoding - Used to specify the compression algorithm.
    pub const ContentEncoding: Header = Header("Content-Encoding");
    /// Content-Language - Describes the human language(s) intended for the audience, so that it allows a user to differentiate according to the users' own preferred language.
    pub const ContentLanguage: Header = Header("Content-Language");
    /// Content-Location - Indicates an alternate location for the returned data.
    pub const ContentLocation: Header = Header("Content-Location");

    // --- Preferences ---
    /// Prefer - Indicates preferences for specific server behaviors during request processing. For example, it can request minimal response content (return=minimal) or asynchronous processing (respond-async). The server processes the request normally if the header is unsupported.
    pub const Prefer: Header = Header("Prefer");
    /// Preference-Applied - Informs the client which preferences specified in the Prefer header were applied by the server. It is a response-only header providing transparency about preference handling.
    pub const PreferenceApplied: Header = Header("Preference-Applied");

    // --- Proxies ---
    /// Forwarded - Contains information from the client-facing side of proxy servers that is altered or lost when a proxy is involved in the path of the request.
    pub const Forwarded: Header = Header("Forwarded");
    /// Via - Added by proxies, both forward and reverse proxies, and can appear in the request headers and the response headers.
    pub const Via: Header = Header("Via");

    // --- Range requests ---
    /// Accept-Ranges - Indicates if the server supports range requests, and if so in which unit the range can be expressed.
    pub const AcceptRanges: Header = Header("Accept-Ranges");
    /// Range - Indicates the part of a document that the server should return.
    pub const Range: Header = Header("Range");
    /// If-Range - Creates a conditional range request that is only fulfilled if the given etag or date matches the remote resource. Used to prevent downloading two ranges from incompatible version of the resource.
    pub const IfRange: Header = Header("If-Range");
    /// Content-Range - Indicates where in a full body message a partial message belongs.
    pub const ContentRange: Header = Header("Content-Range");

    // --- Redirects ---
    /// Location - Indicates the URL to redirect a page to.
    pub const Location: Header = Header("Location");
    /// Refresh - Directs the browser to reload the page or redirect to another. Takes the same value as the meta element with http-equiv="refresh".
    pub const Refresh: Header = Header("Refresh");

    // --- Request context ---
    /// From - Contains an Internet email address for a human user who controls the requesting user agent.
    pub const From: Header = Header("From");
    /// Host - Specifies the domain name of the server (for virtual hosting), and (optionally) the TCP port number on which the server is listening.
    pub const Host: Header = Header("Host");
    /// Referer - The address of the previous web page from which a link to the currently requested page was followed.
    pub const Referer: Header = Header("Referer");
    /// Referrer-Policy - Governs which referrer information sent in the Referer header should be included with requests made.
    pub const ReferrerPolicy: Header = Header("Referrer-Policy");
    /// User-Agent - Contains a characteristic string that allows the network protocol peers to identify the application type, operating system, software vendor or software version of the requesting software user agent.
    pub const UserAgent: Header = Header("User-Agent");

    // --- Response context ---
    /// Allow - Lists the set of HTTP request methods supported by a resource.
    pub const Allow: Header = Header("Allow");
    /// Server - Contains information about the software used by the origin server to handle the request.
    pub const Server: Header = Header("Server");

    // --- Security ---
    /// Cross-Origin-Embedder-Policy - Allows a server to declare an embedder policy for a given document.
    pub const CrossOriginEmbedderPolicy: Header = Header("Cross-Origin-Embedder-Policy");
    /// Cross-Origin-Opener-Policy - Prevents other domains from opening/controlling a window.
    pub const CrossOriginOpenerPolicy: Header = Header("Cross-Origin-Opener-Policy");
    /// Cross-Origin-Resource-Policy - Prevents other domains from reading the response of the resources to which this header is applied. See also CORP explainer article.
    pub const CrossOriginResourcePolicy: Header = Header("Cross-Origin-Resource-Policy");
    /// Content-Security-Policy - Controls resources the user agent is allowed to load for a given page.
    pub const ContentSecurityPolicy: Header = Header("Content-Security-Policy");
    /// Content-Security-Policy-Report-Only - Allows web developers to experiment with policies by monitoring, but not enforcing, their effects. These violation reports consist of JSON documents sent via an HTTP POST request to the specified URI.
    pub const ContentSecurityPolicyReportOnly: Header =
        Header("Content-Security-Policy-Report-Only");
    /// Permissions-Policy - Provides a mechanism to allow and deny the use of browser features in a website's own frame, and in <iframe>s that it embeds.
    pub const PermissionsPolicy: Header = Header("Permissions-Policy");
    /// Reporting-Endpoints - Response header that allows website owners to specify one or more endpoints used to receive errors such as CSP violation reports, Cross-Origin-Opener-Policy reports, or other generic violations.
    pub const ReportingEndpoints: Header = Header("Reporting-Endpoints");
    /// Strict-Transport-Security - Force communication using HTTPS instead of HTTP.
    pub const StrictTransportSecurity: Header = Header("Strict-Transport-Security");
    /// Upgrade-Insecure-Requests - Sends a signal to the server expressing the client's preference for an encrypted and authenticated response, and that it can successfully handle the upgrade-insecure-requests directive.
    pub const UpgradeInsecureRequests: Header = Header("Upgrade-Insecure-Requests");
    /// X-Content-Type-Options - Disables MIME sniffing and forces browser to use the type given in Content-Type.
    pub const XContentTypeOptions: Header = Header("X-Content-Type-Options");
    /// X-Frame-Options - Indicates whether a browser should be allowed to render a page in a <frame>, <iframe>, <embed> or <object>.
    pub const XFrameOptions: Header = Header("X-Frame-Options");
    /// X-Permitted-Cross-Domain-Policies - A cross-domain policy file may grant clients, such as Adobe Acrobat or Apache Flex (among others), permission to handle data across domains that would otherwise be restricted due to the Same-Origin Policy. The X-Permitted-Cross-Domain-Policies header overrides such policy files so that clients still block unwanted requests.
    pub const XPermittedCrossDomainPolicies: Header = Header("X-Permitted-Cross-Domain-Policies");
    /// X-Powered-By - May be set by hosting environments or other frameworks and contains information about them while not providing any usefulness to the application or its visitors. Unset this header to avoid exposing potential vulnerabilities.
    pub const XPoweredBy: Header = Header("X-Powered-By");
    /// X-XSS-Protection - Enables cross-site scripting filtering.
    pub const XXSSProtection: Header = Header("X-XSS-Protection");

    // --- Fetch metadata ---
    /// Sec-Fetch-Site - Indicates the relationship between a request initiator's origin and its target's origin. It is a Structured Header whose value is a token with possible values cross-site, same-origin, same-site, and none.
    pub const SecFetchSite: Header = Header("Sec-Fetch-Site");
    /// Sec-Fetch-Mode - Indicates the request's mode to a server. It is a Structured Header whose value is a token with possible values cors, navigate, no-cors, same-origin, and websocket.
    pub const SecFetchMode: Header = Header("Sec-Fetch-Mode");
    /// Sec-Fetch-User - Indicates whether or not a navigation request was triggered by user activation. It is a Structured Header whose value is a boolean so possible values are ?0 for false and ?1 for true.
    pub const SecFetchUser: Header = Header("Sec-Fetch-User");
    /// Sec-Fetch-Dest - Indicates the request's destination. It is a Structured Header whose value is a token with possible values audio, audioworklet, document, embed, empty, font, image, manifest, object, paintworklet, report, script, serviceworker, sharedworker, style, track, video, worker, and xslt.
    pub const SecFetchDest: Header = Header("Sec-Fetch-Dest");
    /// Sec-Purpose - Indicates the purpose of the request, when the purpose is something other than immediate use by the user-agent. The header currently has one possible value, prefetch, which indicates that the resource is being fetched preemptively for a possible future navigation.
    pub const SecPurpose: Header = Header("Sec-Purpose");
    /// Service-Worker-Navigation-Preload - A request header sent in preemptive request to fetch() a resource during service worker boot. The value, which is set with NavigationPreloadManager.setHeaderValue(), can be used to inform a server that a different resource should be returned than in a normal fetch() operation.
    pub const ServiceWorkerNavigationPreload: Header = Header("Service-Worker-Navigation-Preload");

    // --- Fetch storage access ---
    /// Sec-Fetch-Storage-Access - Indicates the "storage access status" for the current fetch context, which will be one of none, inactive, or active. The server may respond with Activate-Storage-Access to request that the browser activate an inactive permission and retry the request, or to load a resource with access to its third-party cookies if the status is active.
    pub const SecFetchStorageAccess: Header = Header("Sec-Fetch-Storage-Access");
    /// Activate-Storage-Access - Used in response to Sec-Fetch-Storage-Access to indicate that the browser can activate an existing permission for secure access and retry the request with cookies, or load a resource with cookie access if it already has an activated permission.
    pub const ActivateStorageAccess: Header = Header("Activate-Storage-Access");

    // --- Transfer coding ---
    /// Transfer-Encoding - Specifies the form of encoding used to safely transfer the resource to the user.
    pub const TransferEncoding: Header = Header("Transfer-Encoding");
    /// TE - Specifies the transfer encodings the user agent is willing to accept.
    pub const TE: Header = Header("TE");
    /// Trailer - Allows the sender to include additional fields at the end of chunked message.
    pub const Trailer: Header = Header("Trailer");

    // --- WebSockets ---
    /// Sec-WebSocket-Accept - Response header that indicates that the server is willing to upgrade to a WebSocket connection.
    pub const SecWebSocketAccept: Header = Header("Sec-WebSocket-Accept");
    /// Sec-WebSocket-Extensions - In requests, this header indicates the WebSocket extensions supported by the client in preferred order. In responses, it indicates the extension selected by the server from the client's preferences.
    pub const SecWebSocketExtensions: Header = Header("Sec-WebSocket-Extensions");
    /// Sec-WebSocket-Key - Request header containing a key that verifies that the client explicitly intends to open a WebSocket.
    pub const SecWebSocketKey: Header = Header("Sec-WebSocket-Key");
    /// Sec-WebSocket-Protocol - In requests, this header indicates the sub-protocols supported by the client in preferred order. In responses, it indicates the sub-protocol selected by the server from the client's preferences.
    pub const SecWebSocketProtocol: Header = Header("Sec-WebSocket-Protocol");
    /// Sec-WebSocket-Version - In requests, this header indicates the version of the WebSocket protocol used by the client. In responses, it is sent only if the requested protocol version is not supported by the server, and lists the versions that the server supports.
    pub const SecWebSocketVersion: Header = Header("Sec-WebSocket-Version");

    // --- Other ---
    /// Alt-Svc - Used to list alternate ways to reach this service.
    pub const AltSvc: Header = Header("Alt-Svc");
    /// Alt-Used - Used to identify the alternative service in use.
    pub const AltUsed: Header = Header("Alt-Used");
    /// Date - Contains the date and time at which the message was originated.
    pub const Date: Header = Header("Date");
    /// Link - This entity-header field provides a means for serializing one or more links in HTTP headers. It is semantically equivalent to the HTML <link> element.
    pub const Link: Header = Header("Link");
    /// Retry-After - Indicates how long the user agent should wait before making a follow-up request.
    pub const RetryAfter: Header = Header("Retry-After");
    /// Server-Timing - Communicates one or more metrics and descriptions for the given request-response cycle.
    pub const ServerTiming: Header = Header("Server-Timing");
    /// Service-Worker - Included in fetches for a service worker's script resource. This header helps administrators log service worker script requests for monitoring purposes.
    pub const ServiceWorker: Header = Header("Service-Worker");
    /// Service-Worker-Allowed - Used to remove the path restriction by including this header in the response of the Service Worker script.
    pub const ServiceWorkerAllowed: Header = Header("Service-Worker-Allowed");
    /// SourceMap - Links to a source map so that debuggers can step through original source code instead of generated or transformed code.
    pub const SourceMap: Header = Header("SourceMap");
    /// Upgrade - This HTTP/1.1 (only) header can be used to upgrade an already established client/server connection to a different protocol (over the same transport protocol). For example, it can be used by a client to upgrade a connection from HTTP 1.1 to HTTP 2.0, or an HTTP or HTTPS connection into a WebSocket.
    pub const Upgrade: Header = Header("Upgrade");
    /// Priority - Provides a hint from about the priority of a particular resource request on a particular connection. The value can be sent in a request to indicate the client priority, or in a response if the server chooses to reprioritize the request.
    pub const Priority: Header = Header("Priority");

    // --- Experimental ---

    // --- Attribution reporting ---
    /// Attribution-Reporting-Eligible - Used to indicate that the response corresponding to the current request is eligible to take part in attribution reporting, by registering either an attribution source or trigger.
    pub const AttributionReportingEligible: Header = Header("Attribution-Reporting-Eligible");
    /// Attribution-Reporting-Register-Source - Included as part of a response to a request that included an Attribution-Reporting-Eligible header, this is used to register an attribution source.
    pub const AttributionReportingRegisterSource: Header =
        Header("Attribution-Reporting-Register-Source");
    /// Attribution-Reporting-Register-Trigger - Included as part of a response to a request that included an Attribution-Reporting-Eligible header, this is used to register an attribution trigger.
    pub const AttributionReportingRegisterTrigger: Header =
        Header("Attribution-Reporting-Register-Trigger");

    // --- Client hints ---
    /// Accept-CH - Servers can advertise support for Client Hints using the Accept-CH header field or an equivalent HTML <meta> element with http-equiv attribute.
    pub const AcceptCH: Header = Header("Accept-CH");
    /// Critical-CH - Servers use Critical-CH along with Accept-CH to specify that accepted client hints are also critical client hints.
    pub const CriticalCH: Header = Header("Critical-CH");

    // --- User agent client hints ---
    /// Sec-CH-UA - User agent's branding and version.
    pub const SecCHUA: Header = Header("Sec-CH-UA");
    /// Sec-CH-UA-Arch - User agent's underlying platform architecture.
    pub const SecCHUAArch: Header = Header("Sec-CH-UA-Arch");
    /// Sec-CH-UA-Bitness - User agent's underlying CPU architecture bitness (for example "64" bit).
    pub const SecCHUABitness: Header = Header("Sec-CH-UA-Bitness");
    /// Sec-CH-UA-Form-Factors - User agent's form-factors, describing how the user interacts with the user-agent.
    pub const SecCHUAFormFactors: Header = Header("Sec-CH-UA-Form-Factors");
    /// Sec-CH-UA-Full-Version-List - Full version for each brand in the user agent's brand list.
    pub const SecCHUAFullVersionList: Header = Header("Sec-CH-UA-Full-Version-List");
    /// Sec-CH-UA-Mobile - User agent is running on a mobile device or, more generally, prefers a "mobile" user experience.
    pub const SecCHUAMobile: Header = Header("Sec-CH-UA-Mobile");
    /// Sec-CH-UA-Model - User agent's device model.
    pub const SecCHUAModel: Header = Header("Sec-CH-UA-Model");
    /// Sec-CH-UA-Platform - User agent's underlying operation system/platform.
    pub const SecCHUAPlatform: Header = Header("Sec-CH-UA-Platform");
    /// Sec-CH-UA-Platform-Version - User agent's underlying operation system version.
    pub const SecCHUAPlatformVersion: Header = Header("Sec-CH-UA-Platform-Version");
    /// Sec-CH-UA-WoW64 - Whether or not the user agent binary is running in 32-bit mode on 64-bit Windows.
    pub const SecCHUAWoW64: Header = Header("Sec-CH-UA-WoW64");
    /// Sec-CH-Prefers-Color-Scheme - User's preference of dark or light color scheme.
    pub const SecCHPrefersColorScheme: Header = Header("Sec-CH-Prefers-Color-Scheme");
    /// Sec-CH-Prefers-Reduced-Motion - User's preference to see fewer animations and content layout shifts.
    pub const SecCHPrefersReducedMotion: Header = Header("Sec-CH-Prefers-Reduced-Motion");
    /// Sec-CH-Prefers-Reduced-Transparency - Request header indicates the user agent's preference for reduced transparency.
    pub const SecCHPrefersReducedTransparency: Header =
        Header("Sec-CH-Prefers-Reduced-Transparency");

    // --- Device and responsive image client hints ---
    /// Sec-CH-Device-Memory - Approximate amount of available client RAM memory. This is part of the Device Memory API.
    pub const SecCHDeviceMemory: Header = Header("Sec-CH-Device-Memory");
    /// Sec-CH-DPR - Request header that provides the client device pixel ratio (the number of physical device pixels for each CSS pixel).
    pub const SecCHDPR: Header = Header("Sec-CH-DPR");
    /// Sec-CH-Viewport-Height - Request header provides the client's layout viewport height in CSS pixels.
    pub const SecCHViewportHeight: Header = Header("Sec-CH-Viewport-Height");
    /// Sec-CH-Viewport-Width - Request header provides the client's layout viewport width in CSS pixels.
    pub const SecCHViewportWidth: Header = Header("Sec-CH-Viewport-Width");
    /// Sec-CH-Width - Request header provides the image's width in CSS pixels.
    pub const SecCHWidth: Header = Header("Sec-CH-Width");

    // --- Network client hints ---
    /// Downlink - Approximate bandwidth of the client's connection to the server, in Mbps. This is part of the Network Information API.
    pub const Downlink: Header = Header("Downlink");
    /// ECT - The effective connection type ("network profile") that best matches the connection's latency and bandwidth. This is part of the Network Information API.
    pub const ECT: Header = Header("ECT");
    /// RTT - Application layer round trip time (RTT) in milliseconds, which includes the server processing time. This is part of the Network Information API.
    pub const RTT: Header = Header("RTT");
    /// Save-Data - A string on that indicates the user agent's preference for reduced data usage.
    pub const SaveData: Header = Header("Save-Data");

    // --- Compression Dictionary Transport ---
    /// Available-Dictionary - A browser can use this request header to indicate the best dictionary it has available for the server to use for compression.
    pub const AvailableDictionary: Header = Header("Available-Dictionary");
    /// Dictionary-ID - Used when a browser already has a dictionary available for a resource and the server provided an id for the dictionary in the Use-As-Dictionary header. Requests for resources that can use the dictionary have an Available-Dictionary header and the server-provided dictionary id in the Dictionary-ID header.
    pub const DictionaryID: Header = Header("Dictionary-ID");
    /// Use-As-Dictionary - Lists the matching criteria that the dictionary can be used for in future requests.
    pub const UseAsDictionary: Header = Header("Use-As-Dictionary");

    // --- Privacy ---
    /// Sec-GPC - Indicates whether the user consents to a website or service selling or sharing their personal information with third parties.
    pub const SecGPC: Header = Header("Sec-GPC");

    // --- Security ---
    /// Origin-Agent-Cluster - Response header used to indicate that the associated Document should be placed in an origin-keyed agent cluster. This isolation allows user agents to allocate implementation-specific resources for agent clusters, such as processes or threads, more efficiently.
    pub const OriginAgentCluster: Header = Header("Origin-Agent-Cluster");

    // --- Server-sent events ---
    /// NEL - Defines a mechanism that enables developers to declare a network error reporting policy.
    pub const NEL: Header = Header("NEL");

    // --- Topics API ---
    /// Observe-Browsing-Topics - Response header used to mark topics of interest inferred from a calling site's URL as observed in the response to a request generated by a feature that enables the Topics API.
    pub const ObserveBrowsingTopics: Header = Header("Observe-Browsing-Topics");
    /// Sec-Browsing-Topics - Request header that sends the selected topics for the current user along with the associated request, which are used by an ad tech platform to choose a personalized ad to display.
    pub const SecBrowsingTopics: Header = Header("Sec-Browsing-Topics");

    // --- Other ---
    /// Accept-Signature - A client can send the Accept-Signature header field to indicate intention to take advantage of any available signatures and to indicate what kinds of signatures it supports.
    pub const AcceptSignature: Header = Header("Accept-Signature");
    /// Early-Data - Indicates that the request has been conveyed in TLS early data.
    pub const EarlyData: Header = Header("Early-Data");
    /// Idempotency-Key - Provides a unique key for POST and PATCH requests, allowing them to be made idempotent.
    pub const IdempotencyKey: Header = Header("Idempotency-Key");
    /// Set-Login - Response header sent by a federated identity provider (IdP) to set its login status, meaning whether any users are logged into the IdP on the current browser or not. This is stored by the browser and used by the FedCM API.
    pub const SetLogin: Header = Header("Set-Login");
    /// Signature - The Signature header field conveys a list of signatures for an exchange, each one accompanied by information about how to determine the authority of and refresh that signature.
    pub const Signature: Header = Header("Signature");
    /// Signed-Headers - The Signed-Headers header field identifies an ordered list of response header fields to include in a signature.
    pub const SignedHeaders: Header = Header("Signed-Headers");
    /// Speculation-Rules - Provides a list of URLs pointing to text resources containing speculation rule JSON definitions. When the response is an HTML document, these rules will be added to the document's speculation rule set.
    pub const SpeculationRules: Header = Header("Speculation-Rules");
    /// Sec-Speculation-Tags - Contains one or more tag values from the speculation rules that resulted in the speculation so a server can identify which rule(s) caused a speculation and potentially block them.
    pub const SecSpeculationTags: Header = Header("Sec-Speculation-Tags");
    /// Supports-Loading-Mode - Set by a navigation target to opt-in to using various higher-risk loading modes. For example, cross-origin, same-site prerendering requires a Supports-Loading-Mode value of credentialed-prerender.
    pub const SupportsLoadingMode: Header = Header("Supports-Loading-Mode");

    // --- Non-standard headers ---
    /// X-Forwarded-For - Identifies the originating IP addresses of a client connecting to a web server through an HTTP proxy or a load balancer.
    pub const XForwardedFor: Header = Header("X-Forwarded-For");
    /// X-Forwarded-Host - Identifies the original host requested that a client used to connect to your proxy or load balancer.
    pub const XForwardedHost: Header = Header("X-Forwarded-Host");
    /// X-Forwarded-Proto - Identifies the protocol (HTTP or HTTPS) that a client used to connect to your proxy or load balancer.
    pub const XForwardedProto: Header = Header("X-Forwarded-Proto");
    /// X-DNS-Prefetch-Control - Controls DNS prefetching, a feature by which browsers proactively perform domain name resolution on both links that the user may choose to follow as well as URLs for items referenced by the document, including images, CSS, JavaScript, and so forth.
    pub const XDNSPrefetchControl: Header = Header("X-DNS-Prefetch-Control");
    /// X-Robots-Tag - The X-Robots-Tag HTTP header is used to indicate how a web page is to be indexed within public search engine results. The header is equivalent to <meta name="robots"> elements.
    pub const XRobotsTag: Header = Header("X-Robots-Tag");
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&'static str> for Header {
    fn from(value: &'static str) -> Self {
        Header(value)
    }
}

impl From<Header> for &'static str {
    fn from(value: Header) -> Self {
        value.0
    }
}

/// The HTTP header body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderBody<'a>(pub &'a str);

impl<'a> std::fmt::Display for HeaderBody<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> From<&'a str> for HeaderBody<'a> {
    fn from(value: &'a str) -> Self {
        HeaderBody(value)
    }
}

impl<'a> From<HeaderBody<'a>> for &'a str {
    fn from(value: HeaderBody<'a>) -> Self {
        value.0
    }
}

use std::convert::Infallible;

pub struct Headers(pub HeaderMap);

impl<S> FromRequestParts<S> for Headers
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> StdResult<Self, Self::Rejection> {
        Ok(Headers(parts.headers.clone()))
    }
}

impl Headers {
    /// Returns header by name
    pub fn get(&self, name: impl Into<Header>) -> Option<&str> {
        self.0.get(name.into().0).and_then(|v| v.to_str().ok())
    }

    /// Checks header for exists
    pub fn contains(&self, name: impl Into<Header>) -> bool {
        self.0.contains_key(name.into().0)
    }
}
