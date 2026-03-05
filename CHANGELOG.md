## [4.1.0](https://github.com/m3idnotfree/twitch_oauth/compare/v4.0.1..v4.1.0) - 2026-03-05

### Features

- **(scope)** add new scopes ([a987e43](https://github.com/m3idnotfree/twitch_oauth/commit/a987e4319d4bd387e8762f4cb60b940b4536e32f))

## [4.0.0](https://github.com/m3idnotfree/twitch_oauth/compare/v3.1.0..v4.0.0) - 2026-02-07

### Bug Fixes

- **(request)** check HTTP status in validate_access_token ([e6301ec](https://github.com/m3idnotfree/twitch_oauth/commit/e6301ecbda5a9b99cc6b6916017eae473370e2c9))

### Refactoring

- [**breaking**] remove Response wrapper and return deserialized types directly ([12093e2](https://github.com/m3idnotfree/twitch_oauth/commit/12093e276c92667be811ec343dc60672973cc886))
- **(error)** [**breaking**] consolidate response parsing errors into decode error type ([55a863a](https://github.com/m3idnotfree/twitch_oauth/commit/55a863a0ca01c3578e34055e21eab6873d8c4212))
- **(oauth)** [**breaking**] rename set_redirect_uri to with_redirect_uri ([77f10d7](https://github.com/m3idnotfree/twitch_oauth/commit/77f10d780b620819e4ae5e760a8a01a6f5e349bc))
- **(types)** simplify GrantType Display implementation ([41959c9](https://github.com/m3idnotfree/twitch_oauth/commit/41959c9fea5b78e15d089dac233eb2b46b3581a9))
- **(request)** [**breaking**] rename CodeTokenRequest to ExchangeCodeRequest ([4ee7a05](https://github.com/m3idnotfree/twitch_oauth/commit/4ee7a05c6d325cc29b249c1638ba771d52d047eb))
- **(oauth)** [**breaking**] rename user_access_token to exchange_code ([0857485](https://github.com/m3idnotfree/twitch_oauth/commit/0857485c026db3d2604304d8738f7881f8ea193a))
- **(oneshot)** [**breaking**] migrate to asknothingx2-util/oauth-server ([0e0de6f](https://github.com/m3idnotfree/twitch_oauth/commit/0e0de6f32df25bd19381d024ef487490b3f1c091))
- **(types)** [**breaking**] rename OAuthCallbackQuery to AuthCallback ([52c981c](https://github.com/m3idnotfree/twitch_oauth/commit/52c981c2f09d61a7ed8f26baf5d9117c5b7b1c11))
- **(error)** add HTTP status and response body to errors ([77f99f3](https://github.com/m3idnotfree/twitch_oauth/commit/77f99f3730c07e0186757f0bd533bded4509fef5))
- **(error)** [**breaking**] rename is_network_error to is_request_error ([bd6d2c5](https://github.com/m3idnotfree/twitch_oauth/commit/bd6d2c5211a6074e90c32304d019759b5dbc5084))
- **(client)** [**breaking**] simplify setup API and migrate to asknothingx2-util 0.4.0 ([58a90fa](https://github.com/m3idnotfree/twitch_oauth/commit/58a90faef10113f3f966fad85cc3ab7b1264bd6b))
- **(client)** extract client to client.rs ([cbf8414](https://github.com/m3idnotfree/twitch_oauth/commit/cbf8414d6836689831f803ce2758c26f4e22e9a4))
- **(tokens)** [**breaking**] rename ValidateToken to TokenInfo ([dc27aca](https://github.com/m3idnotfree/twitch_oauth/commit/dc27aca6510ec922d42fb142a0fd949163e8c73b))

### Documentation

- **(readme)** update version to 3 ([59e4075](https://github.com/m3idnotfree/twitch_oauth/commit/59e4075377aa45cbcb745bd6ecfd6c5a36f9357c))
- update ([1f39975](https://github.com/m3idnotfree/twitch_oauth/commit/1f399758f4ac39d118635b3543bbedc521ff464b))

### Miscellaneous

- add cliff.toml and CHANGELOG.md ([c9eb153](https://github.com/m3idnotfree/twitch_oauth/commit/c9eb1531ed2b228ad3cd33bb754b6405d0799465))
- **(cliff.toml)** switch from scope-based grouping to chronological order ([df6acf9](https://github.com/m3idnotfree/twitch_oauth/commit/df6acf9aa7f35fd95606f64cfa6d052d4b1a5dfb))

## [3.1.0](https://github.com/m3idnotfree/twitch_oauth/compare/v3.0.3..v3.1.0) - 2026-01-25

### Features

- [**breaking**] remove deprecated set\_\* builder methods ([aa32388](https://github.com/m3idnotfree/twitch_oauth/commit/aa32388ebe0272f876a3a2d5adca137258e056dc))
- **(oauth)** add mutable set\_\* methods for runtime configuration ([5399a3a](https://github.com/m3idnotfree/twitch_oauth/commit/5399a3a07dee85e8f809dc7db33745020890210f))

## [3.0.3](https://github.com/m3idnotfree/twitch_oauth/compare/v3.0.2..v3.0.3) - 2026-01-24

### Features

- **(oauth)** expose client accessors and endpint URL configuration ([ee42d46](https://github.com/m3idnotfree/twitch_oauth/commit/ee42d465356333041c07425b76ba644dde612e8b))

### Refactoring

- **(oauth)** rename set*\* to with*\* methods ([01a39bd](https://github.com/m3idnotfree/twitch_oauth/commit/01a39bd446f8b5a129bb6243d96b962d6860a924))

## [3.0.2](https://github.com/m3idnotfree/twitch_oauth/compare/v3.0.1..v3.0.2) - 2026-01-23

### Refactoring

- **(csrf)** move to asknothingx2-util for code reuse ([4a9d202](https://github.com/m3idnotfree/twitch_oauth/commit/4a9d202be16ab0f827ec8b779ad9bedaf2273598))

## [3.0.1](https://github.com/m3idnotfree/twitch_oauth/compare/v3.0.0..v3.0.1) - 2026-01-12

### Miscellaneous

- migrate from taplo to tombi for TOML formatting ([348bf43](https://github.com/m3idnotfree/twitch_oauth/commit/348bf436402b6afe09e17dda045ae3434a9048af))
- **(deps)** update dependencies ([665fe23](https://github.com/m3idnotfree/twitch_oauth/commit/665fe23e364f39f306817b3c9e5e3e72562b5a4e))
- add GitHub Actions workflow ([0402f80](https://github.com/m3idnotfree/twitch_oauth/commit/0402f803be263722f67cace99ee05916d0793715))

## [3.0.0](https://github.com/m3idnotfree/twitch_oauth/compare/v2.0.9..v3.0.0) - 2025-12-08

### Features

- **(scope)** add ChatbotScopes trait ([581e478](https://github.com/m3idnotfree/twitch_oauth/commit/581e47867e71a0a52b560fae5ab86284fcc3acb9))

### Refactoring

- **(scope)** [**breaking**] remove incorrect ChatScopes methods ([9749760](https://github.com/m3idnotfree/twitch_oauth/commit/974976010428850fc93beb81c71b00129c802d0d))

### Documentation

- update ChatScopes method ([c3f0b64](https://github.com/m3idnotfree/twitch_oauth/commit/c3f0b643842e65cc9d8775ac2499c1c4da2c05c0))

### Style

- format scopes_mut ([5615d43](https://github.com/m3idnotfree/twitch_oauth/commit/5615d4357ab391b8ee3c79f298fd944f515f4bc9))

## [2.0.9](https://github.com/m3idnotfree/twitch_oauth/compare/v2.0.8..v2.0.9) - 2025-12-08

### Features

- **(examples)** add app access token example ([e7c3696](https://github.com/m3idnotfree/twitch_oauth/commit/e7c369622b8ba81230396d7fa2e0db052b36c795))
- **(oauth)** add set_secret_key to TwitchOauth<UserAuth> ([c449ca7](https://github.com/m3idnotfree/twitch_oauth/commit/c449ca7fe4e3a702169544e14b9546822951ae31))
- **(authorize_request)** add url_with_state for cookie-based validation ([a516c63](https://github.com/m3idnotfree/twitch_oauth/commit/a516c63a3b8d2020711476a7824cd9d39f8e9dbe))
- **(examples)** add user access token example ([48e8722](https://github.com/m3idnotfree/twitch_oauth/commit/48e8722d6969cca94ab0684b09914de40a706d72))

### Documentation

- replace doc_auto_cfg with doc_cfg ([88f6bb4](https://github.com/m3idnotfree/twitch_oauth/commit/88f6bb4fb6748115fe7016e5c6e82d523c3476b5))
- improve accuracy of CSRF token explanation ([c6dffba](https://github.com/m3idnotfree/twitch_oauth/commit/c6dffbaeeb565b7943809ed523321fb6377ede3b))

### Miscellaneous

- **(deps)** replace dotenv with dotenvy ([1a649e9](https://github.com/m3idnotfree/twitch_oauth/commit/1a649e9cfdffbbdc167494e4f6ee5e606663be0b))
- **(dev-deps)** add anyhow ([38da491](https://github.com/m3idnotfree/twitch_oauth/commit/38da491e560adde777b7d9fe1d97592c454036d1))

## [2.0.8](https://github.com/m3idnotfree/twitch_oauth/compare/v2.0.7..v2.0.8) - 2025-10-03

### Features

- **(tokens)** handle empty string scopes in deserialization ([b7bd1ad](https://github.com/m3idnotfree/twitch_oauth/commit/b7bd1ad9f9b5ed58b8c3bd251db634adb1bb2107))

## [2.0.7](https://github.com/m3idnotfree/twitch_oauth/compare/v2.0.6..v2.0.7) - 2025-09-30

### Features

- **(oauth)** implement Display for TwitchOauth ([c605b69](https://github.com/m3idnotfree/twitch_oauth/commit/c605b69daa8c73af5805341f217d86ccecff8c5d))

### Refactoring

- **(oauth)** simplify test structure with with_test method ([ae4eca4](https://github.com/m3idnotfree/twitch_oauth/commit/ae4eca408fa1a63680fec24e0b741c3b86ad3ea6))

### Documentation

- **(README)** update license to dual MIT/Apache-2.0 ([360aa0e](https://github.com/m3idnotfree/twitch_oauth/commit/360aa0ec15cf91b3bdf603b783fa9162f2fb0cb1))

### Miscellaneous

- **(error)** remove unused TokenError ([28ec942](https://github.com/m3idnotfree/twitch_oauth/commit/28ec94207427c9c2e1235a7d5b853fe00ea33efa))
- **(deps)** update dependencies ([1311aaa](https://github.com/m3idnotfree/twitch_oauth/commit/1311aaaac8afa9d399353279e491b34cd08f72f5))

## [2.0.6](https://github.com/m3idnotfree/twitch_oauth/compare/v2.0.5..v2.0.6) - 2025-09-15

### Refactoring

- migrate to asknothingx2-util 0.1.9 ([356d7e5](https://github.com/m3idnotfree/twitch_oauth/commit/356d7e56713942312c24141107d2410bd402e6c9))
- **(lib)** reorganize public API with explicit exports ([80320a1](https://github.com/m3idnotfree/twitch_oauth/commit/80320a18bd3d2b5d64b68edfbbdf6d936aa1c37c))
- **(oauth)** simplify URL handling ([0f9bca4](https://github.com/m3idnotfree/twitch_oauth/commit/0f9bca43907e9cf6017871fc6ae8086e2f0483fa))

## [2.0.5](https://github.com/m3idnotfree/twitch_oauth/compare/v2.0.4..v2.0.5) - 2025-09-13

### Refactoring

- **(scope)** rename with_teams_api to teams_api ([59e1e53](https://github.com/m3idnotfree/twitch_oauth/commit/59e1e53e2fcd351a2ea99ee2a99a8574989679fd))

## [2.0.4](https://github.com/m3idnotfree/twitch_oauth/compare/v2.0.3..v2.0.4) - 2025-09-13

### Refactoring

- **(scope)** [**breaking**] remove EmptyString variant ([ae46324](https://github.com/m3idnotfree/twitch_oauth/commit/ae46324685936f4a3e0d97ce647c125f7970311c))

### Documentation

- update method names ([20b6462](https://github.com/m3idnotfree/twitch_oauth/commit/20b64620f73d768084f8a602552fc31684666557))

## [2.0.3](https://github.com/m3idnotfree/twitch_oauth/compare/v2.0.2..v2.0.3) - 2025-09-13

### Refactoring

- **(scope)** [**breaking**] redesign scope API to match Twitch endpoint names ([7aabff1](https://github.com/m3idnotfree/twitch_oauth/commit/7aabff16016977ef4571f07981dc539b4545001f))

### Miscellaneous

- adopt dual license (MIT OR Apache-2.0) ([07e468c](https://github.com/m3idnotfree/twitch_oauth/commit/07e468c1a6b3f8d3ee747e6cc2c49607b0b6ed56))

## [2.0.2](https://github.com/m3idnotfree/twitch_oauth/compare/v2.0.1..v2.0.2) - 2025-09-11

### Features

- add Clone derive ([1359f7a](https://github.com/m3idnotfree/twitch_oauth/commit/1359f7a38be4fd29b8682b193ea5cfbef66bdef7))

## [2.0.1](https://github.com/m3idnotfree/twitch_oauth/compare/v2.0.0..v2.0.1) - 2025-08-10

### Bug Fixes

- add feature flags for conditional compilation ([558d21c](https://github.com/m3idnotfree/twitch_oauth/commit/558d21cff14cc808f4b21245ca6ae9dac14affd1))

### Refactoring

- **(error)** make error creation methods private ([4262ffa](https://github.com/m3idnotfree/twitch_oauth/commit/4262ffa8e8165e2e7ae3dd7bcbc3719d5fecaed1))

### Miscellaneous

- improve package metadata and visibility ([eb361d6](https://github.com/m3idnotfree/twitch_oauth/commit/eb361d69bcda02dab15d4e5b7242a81613c7574b))

## [2.0.0](https://github.com/m3idnotfree/twitch_oauth/compare/v1.1.12..v2.0.0) - 2025-08-10

### Features

- **(response)** add Response type using type-state pattern ([1b23575](https://github.com/m3idnotfree/twitch_oauth/commit/1b23575a18247268ae61f1b8c117caab92588116))
- **(oauth)** add configurable URL fields for testing ([92f3459](https://github.com/m3idnotfree/twitch_oauth/commit/92f3459520f816cd6fde420197b26325ddad9601))
- **(error)** add response parsing error kinds for Response type ([3762305](https://github.com/m3idnotfree/twitch_oauth/commit/3762305226e2e0cc3c224b5d3d2af62cb56ec1ec))
- **(client)** add configurable client preset ([32e728d](https://github.com/m3idnotfree/twitch_oauth/commit/32e728d156ecfae4420f604c24b30585929fa1e3))
- **(token)** add created_at field to enable local expiry check ([089ebe7](https://github.com/m3idnotfree/twitch_oauth/commit/089ebe7e7995a549a7750871321dd2495602201a))
- **(response)** implement Debug trait ([b0bded8](https://github.com/m3idnotfree/twitch_oauth/commit/b0bded89fb1ae2f8b9528cd306a4b15a22749dd8))
- **(oauth)** enhance type safety with associated type RedirectUrl ([8f1b9a9](https://github.com/m3idnotfree/twitch_oauth/commit/8f1b9a94c29763938149d4bc5842873fc1e744b2))
- **(test)** add MockApiUnits client for Twitch mock API ([cf240e1](https://github.com/m3idnotfree/twitch_oauth/commit/cf240e16344bacb9803165013e48813b3a4a3922))
- **(oauth)** add configurable CSRF token validation ([13096da](https://github.com/m3idnotfree/twitch_oauth/commit/13096daf4c5b47372d75abd69430f8c38f707c46))
- complete documentation and API restructure ([9adec08](https://github.com/m3idnotfree/twitch_oauth/commit/9adec0851e469a6021abfc57e089dd21ec9b5ae0))

### Bug Fixes

- **(scope)** add missing channel:moderate scope ([df0c221](https://github.com/m3idnotfree/twitch_oauth/commit/df0c2214dc9cd90f93b5453c76260513a158035f))
- **(oauth)** inverted csrf validation and missing client_id ([3f08310](https://github.com/m3idnotfree/twitch_oauth/commit/3f08310df877cb1276fb3cf058f4027fae53dfa1))
- **(scopes)** correct spelling ([eba2951](https://github.com/m3idnotfree/twitch_oauth/commit/eba2951b5930e3b09b94cffee8aee7c953d0d580))
- **(oauth)** remove Result wrapper from from_credentials ([66a7c8f](https://github.com/m3idnotfree/twitch_oauth/commit/66a7c8f9d208d25b913d555bd50ef025b90a9a3e))

### Refactoring

- **(oauth)** [**breaking**] rename methods consistently ([6ddb1b0](https://github.com/m3idnotfree/twitch_oauth/commit/6ddb1b026a58eca1438516b7fb0af743e1d8546b))
- move IntoRequestBuilder to asknothingx2_util ([017c848](https://github.com/m3idnotfree/twitch_oauth/commit/017c84829585c87dd13f9af71af9b70a41829a0a))
- remove unnecessary step in OAuth flow ([49272dc](https://github.com/m3idnotfree/twitch_oauth/commit/49272dcd60236750f6576e69170d95a2a345040e))
- **(oauth)** authentication error handling to send method ([3b2e358](https://github.com/m3idnotfree/twitch_oauth/commit/3b2e358eb92445b1eb5d8be6ecc71e4a11893f4c))
- improve OAuth flow types and add documentation ([09748f3](https://github.com/m3idnotfree/twitch_oauth/commit/09748f3354492e7006f58f3a908e95058081e83c))
- **(response)** rename ResponseState to ResponseType ([b6d7ee6](https://github.com/m3idnotfree/twitch_oauth/commit/b6d7ee603719ad0b9ab82d009a76d1002496febb))
- **(oauth)** remove unused getters and extract test-only methods ([de801dd](https://github.com/m3idnotfree/twitch_oauth/commit/de801dd890bcf8b638d1e691aecce79be5c1b8df))
- **(client_credentials)** add grant_type field for mock api ([ccc4d82](https://github.com/m3idnotfree/twitch_oauth/commit/ccc4d820f4ac2b75d697f598384e91402090bb91))
- improve naming consistency and module organization ([e7cf22d](https://github.com/m3idnotfree/twitch_oauth/commit/e7cf22d047a5b900e592c32f786a9e2dc646500f))
- **(error)** add FormData type, remove unused kinds ([ab90052](https://github.com/m3idnotfree/twitch_oauth/commit/ab9005277d3c0f911441a7cfe8987d4c59de5add))
- **(oneshot-server)** migrate from manual HTTP parsing to Axum framework ([ac5dc14](https://github.com/m3idnotfree/twitch_oauth/commit/ac5dc143c3eb9532b045b430ba3a171d389c1d9e))

### Documentation

- use doc_auto_cfg to automatically detect feature requirements ([77230ad](https://github.com/m3idnotfree/twitch_oauth/commit/77230ad8b5600e094bc06ee5a93e87bb469e2632))
- rewrite README ([57ea5e7](https://github.com/m3idnotfree/twitch_oauth/commit/57ea5e773490a367f43fa79bc451f7d6356872a0))

### Tests

- restructure test suite organization ([1ef100d](https://github.com/m3idnotfree/twitch_oauth/commit/1ef100de2bbdd2ba569f99ae223914743b87eae5))

### Miscellaneous

- **(deps)** update asknothingx2_util to 0.1.7 and adapt to API changes ([7d2c83b](https://github.com/m3idnotfree/twitch_oauth/commit/7d2c83be8e94a93de72fa97c67130c2766f57235))

## [1.1.12](https://github.com/m3idnotfree/twitch_oauth/compare/v1.1.11..v1.1.12) - 2025-07-20

### Refactoring

- [**breaking**] remove oauth and types features ([778972a](https://github.com/m3idnotfree/twitch_oauth/commit/778972ac9a210496bfa13630d17c1f271e126324))

## [1.1.11](https://github.com/m3idnotfree/twitch_oauth/compare/v1.1.10..v1.1.11) - 2025-07-20

### Features

- add APPTYPE constant for asknothingx2_util::api ([25fb082](https://github.com/m3idnotfree/twitch_oauth/commit/25fb0829a5365a7104b0c78d7c3adb9453f6eff6))
- add stateless CSRF token generation and validation ([558b3c6](https://github.com/m3idnotfree/twitch_oauth/commit/558b3c6e25dc7abfc41b91aa2b36c6b61aa5b976))
- **(oauth)** add authorization client ([280e2c8](https://github.com/m3idnotfree/twitch_oauth/commit/280e2c896f87c8902ea8b824891012c39c23448d))

### Bug Fixes

- (types): handle missing scope field in Token deserialization ([94eead5](https://github.com/m3idnotfree/twitch_oauth/commit/94eead5ddc7c1e3869a399bc5731a12d284dbfbd))

### Refactoring

- redesign error ([136c62a](https://github.com/m3idnotfree/twitch_oauth/commit/136c62a4c342dd128eaa9393084ea68ca7476943))
- [**breaking**] migrate to new asknothingx2_util API patterns ([04ddf30](https://github.com/m3idnotfree/twitch_oauth/commit/04ddf305ef2cd8d1f87d20a1d350a7c1e8736877))
- **(oauth)** implement typestate pattern for redirect URI ([556da2c](https://github.com/m3idnotfree/twitch_oauth/commit/556da2c94c0e975b0b1e457018653745dffe75ea))
- **(test)** [**breaking**] extract test oauth functionality into separate module ([51a6b50](https://github.com/m3idnotfree/twitch_oauth/commit/51a6b50af8c590dcdf133ee3ca348faf0aae7089))
- **(oneshot_server)** [**breaking**] restructure API and error handling ([e62653d](https://github.com/m3idnotfree/twitch_oauth/commit/e62653d1c1fd8e58eddfcb21e7cb09b365d7de45))
- **(TwitchOauth)** [**breaking**] simplefy API and add convenience methods ([25403a8](https://github.com/m3idnotfree/twitch_oauth/commit/25403a89e623bba8a6ea0e3945d8f00da68abd26))
- **(error)** remove unnecessary error kinds ([0b4c61b](https://github.com/m3idnotfree/twitch_oauth/commit/0b4c61b3db0d3957a7da7a0d932521c88894d10e))

### Documentation

- update ([3abddf1](https://github.com/m3idnotfree/twitch_oauth/commit/3abddf1af1baffd37afd269ecebd896d1344c231))

### Tests

- update ([fa71516](https://github.com/m3idnotfree/twitch_oauth/commit/fa715166ac59c61eddb75f824ab16814fc525c78))

### Miscellaneous

- **(deps)** upgrade asknothingx2_util ([98a2b06](https://github.com/m3idnotfree/twitch_oauth/commit/98a2b06ef296f012a379245c456b04514fd64040))

### Other

- remove unnecessary clones ([b72f491](https://github.com/m3idnotfree/twitch_oauth/commit/b72f4915c833badd2b7cf94f596c76bcde4420c6))

## [1.1.10](https://github.com/m3idnotfree/twitch_oauth/compare/v1.1.9..v1.1.10) - 2025-01-15

### Bug Fixes

- **(test_url)** request_access_token return APIResponse ([bbd1026](https://github.com/m3idnotfree/twitch_oauth/commit/bbd10265c2a73e1993db34828219a5c3961025d9))

## [1.1.9](https://github.com/m3idnotfree/twitch_oauth/compare/v1.1.8..v1.1.9) - 2025-01-11

### Miscellaneous

- **(deps)** upgrade asknothingx2_util ([3382c00](https://github.com/m3idnotfree/twitch_oauth/commit/3382c00c3fef4491988849c0e7fb2c9a169d7dbc))

## [1.1.8](https://github.com/m3idnotfree/twitch_oauth/compare/v1.1.7..v1.1.8) - 2025-01-10

### Refactoring

- **(oauth)** update APIResponse struct ([31adbfc](https://github.com/m3idnotfree/twitch_oauth/commit/31adbfc563455d87c2fd43c3e9630465b3364ef8))

### Documentation

- **(example)** add examples ([b361d07](https://github.com/m3idnotfree/twitch_oauth/commit/b361d0785bff774c64357f85a52d4d6bc8685337))

## [1.1.7](https://github.com/m3idnotfree/twitch_oauth/compare/v1.1.6..v1.1.7) - 2025-01-10

### Refactoring

- **(error)** reorganize error handling ([bdf22b4](https://github.com/m3idnotfree/twitch_oauth/commit/bdf22b4996be0f72b7db6f351a8a9c6874ebc8d6))

## [1.1.6](https://github.com/m3idnotfree/twitch_oauth/compare/v1.1.5..v1.1.6) - 2025-01-08

### Features

- **(test_url)** add request_acces_token method ([757be37](https://github.com/m3idnotfree/twitch_oauth/commit/757be374b47d30528132bc5bea98f0c705672a6c))

## [1.1.5](https://github.com/m3idnotfree/twitch_oauth/compare/v1.1.4..v1.1.5) - 2025-01-08

### Refactoring

- **(validate)** extract token validation into standalone function ([7b07da8](https://github.com/m3idnotfree/twitch_oauth/commit/7b07da8dd78dfa51182b1fd1fdd1cc82b3b489dc))

## [1.1.4](https://github.com/m3idnotfree/twitch_oauth/compare/v1.1.3..v1.1.4) - 2025-01-08

### Features

- **(scopes)** categorize API scopes ([89d33e6](https://github.com/m3idnotfree/twitch_oauth/commit/89d33e6c7fb0bbff2f824fa9abfad975c7661ba6))

## [1.1.3](https://github.com/m3idnotfree/twitch_oauth/compare/v1.1.2..v1.1.3) - 2025-01-06

### Refactoring

- **(oauth)** simplify generic constraints by using String directly ([1d5845b](https://github.com/m3idnotfree/twitch_oauth/commit/1d5845b90b3273d65671811bd8dc0e2f34713d17))

## [1.1.2](https://github.com/m3idnotfree/twitch_oauth/compare/v1.1.1..v1.1.2) - 2025-01-02

### Tests

- **(test_url)** add ([5af62ce](https://github.com/m3idnotfree/twitch_oauth/commit/5af62ce776fad546e3680b826b8b4c84d6e7cb13))

## [1.1.1](https://github.com/m3idnotfree/twitch_oauth/compare/v1.1.0..v1.1.1) - 2024-12-31

### Refactoring

- **(test_url)** rename Users to UsersResponse ([7cff4c2](https://github.com/m3idnotfree/twitch_oauth/commit/7cff4c2ee46a80e64c452d38d442f3a78d006e3e))
- **(oauth)** restructure TwitchOauth initialization and visibility ([48e6b39](https://github.com/m3idnotfree/twitch_oauth/commit/48e6b3935629f4a0a396e4109cba8bae92f14841))

### Documentation

- update example ([5e6ed04](https://github.com/m3idnotfree/twitch_oauth/commit/5e6ed04a3527af2d06ecb4cc0422bf8357e094da))

## [1.1.0](https://github.com/m3idnotfree/twitch_oauth/compare/v1.0.8..v1.1.0) - 2024-12-29

### Features

- **(scope)** introduce API and IRC scope traits for authorization ([f36ff18](https://github.com/m3idnotfree/twitch_oauth/commit/f36ff185bbd54db3483ed5e99ffc063e259d5970))
- **(request)** implement force verification option ([ae6919b](https://github.com/m3idnotfree/twitch_oauth/commit/ae6919b43e9c48bf94e4d5a31408983443d3b7a6))
- **(oneshot-server)** expose CodeState, ServerStatus ([83c4b76](https://github.com/m3idnotfree/twitch_oauth/commit/83c4b76fb4b710efa84b8b1f3f74e55b9aaed217))
- **(request)** expose request module ([adf997b](https://github.com/m3idnotfree/twitch_oauth/commit/adf997b217d98727a3219964a302f2a476a50abe))

### Bug Fixes

- **(lib)** test_help to test_url ([3878efc](https://github.com/m3idnotfree/twitch_oauth/commit/3878efc8ebc5a7422727d5b4f4797469d7ba6468))

### Refactoring

- **(oneshot_server)** extract helper functions ([1c3cf7d](https://github.com/m3idnotfree/twitch_oauth/commit/1c3cf7db387ff5b41f537b0b073e0f16a86feb28))
- **(oauth)** reorganize oauth directory ([3d45892](https://github.com/m3idnotfree/twitch_oauth/commit/3d4589292d6a29f3283f2b29603b984d5775a975))
- **(oauth)** move exchange_code method behind oneshot-server feature flag ([4a4e518](https://github.com/m3idnotfree/twitch_oauth/commit/4a4e5183d7a19a292f2c10401a4002380f42105a))
- **(types)** relocate CodeState and ServerStatus struct to oneshot_server module ([fb28140](https://github.com/m3idnotfree/twitch_oauth/commit/fb2814089182f465f0f237ee2b47307afc2758ae))

### Documentation

- update usage and example ([966bcd4](https://github.com/m3idnotfree/twitch_oauth/commit/966bcd4d6c5c61f321be7e7ac722b3c152ee796a))

### Tests

- update ([6d2d01a](https://github.com/m3idnotfree/twitch_oauth/commit/6d2d01a3efe848a9e0c63540980796340b15a97d))

## [1.0.8](https://github.com/m3idnotfree/twitch_oauth/compare/v1.0.7..v1.0.8) - 2024-12-29

### Miscellaneous

- **(flag)** add missing feature flag ([166a980](https://github.com/m3idnotfree/twitch_oauth/commit/166a9804edb88187c3cdc06737b95cf409ffb629))

### Other

- Rename Scopes to Scope enum ([23b7c25](https://github.com/m3idnotfree/twitch_oauth/commit/23b7c258c5982a93c0bba352dd70becd504ca36c))

## [1.0.7](https://github.com/m3idnotfree/twitch_oauth/compare/v1.0.6..v1.0.7) - 2024-12-29

### Refactoring

- reorganize structure ([964bc5b](https://github.com/m3idnotfree/twitch_oauth/commit/964bc5b1c1a34326371d0de0904d545269953963))

## [1.0.6](https://github.com/m3idnotfree/twitch_oauth/compare/v1.0.5..v1.0.6) - 2024-12-28

### Other

- reorganize feature flag ([f171b3e](https://github.com/m3idnotfree/twitch_oauth/commit/f171b3eb234eed49346b7a1c077591d996fce512))

## [1.0.5](https://github.com/m3idnotfree/twitch_oauth/compare/v1.0.4..v1.0.5) - 2024-12-28

### Style

- run cargo fmt ([eda91d5](https://github.com/m3idnotfree/twitch_oauth/commit/eda91d50ca5200164aa6a3dcfabd2e44fd204b84))

### Other

- reorganize feature flag ([55a479f](https://github.com/m3idnotfree/twitch_oauth/commit/55a479f78b93b3040115dc726a740707e5bfdcaf))

## [1.0.4](https://github.com/m3idnotfree/twitch_oauth/compare/v1.0.3..v1.0.4) - 2024-12-23

### Features

- **(scopes)** add EmptyString variant ([71cab7b](https://github.com/m3idnotfree/twitch_oauth/commit/71cab7b7d7fcfe6adbb1937bbd8953f65bfd56d2))

## [1.0.3](https://github.com/m3idnotfree/twitch_oauth/compare/v1.0.2..v1.0.3) - 2024-12-23

### Refactoring

- **(types)** change scope from Vec<String> to Vec<Scopes> ([1507853](https://github.com/m3idnotfree/twitch_oauth/commit/1507853858e43185ce2567378d66fc91ded3d4cd))

## [1.0.2](https://github.com/m3idnotfree/twitch_oauth/compare/v1.0.1..v1.0.2) - 2024-12-21

### Features

- **(scopes)** add help methods ([732fd72](https://github.com/m3idnotfree/twitch_oauth/commit/732fd72f042efefe6c3dbc9eff09556dbb1a16d7))

### Refactoring

- **(test_help)** use strong types ([929981c](https://github.com/m3idnotfree/twitch_oauth/commit/929981c6f1dbc2343fbf80ae54aaa70fdb72f7bd))

## [1.0.1](https://github.com/m3idnotfree/twitch_oauth/compare/v1.0.0..v1.0.1) - 2024-12-20

### Refactoring

- **(types)** add explicit as_str method ([719b936](https://github.com/m3idnotfree/twitch_oauth/commit/719b936f549b694570912afdfa3f894e4b361097))
- **(oauth)** improve CSRF token handling ([0d6ced7](https://github.com/m3idnotfree/twitch_oauth/commit/0d6ced7cabd491fb119fa27e52f3836c1cc12c5e))
- **(server)** improve OAuth redirect host validation errors ([dfffd6b](https://github.com/m3idnotfree/twitch_oauth/commit/dfffd6b97cd62f8af03117a18f5969011c8b689f))
- **(test_help)** remove return type from with_url method ([a88d316](https://github.com/m3idnotfree/twitch_oauth/commit/a88d316b8050d21acebcb944d6489aedaf227621))
- **(oauth)** remove lifetime from validate_token method ([957d2f7](https://github.com/m3idnotfree/twitch_oauth/commit/957d2f79c3db48c003d079994b7a4e1492c32b4d))

### Documentation

- update example ([6f24de4](https://github.com/m3idnotfree/twitch_oauth/commit/6f24de42d45ba563d1a73de293235a311ec65490))

### Other

- **(errors)** enhance error messages for better clarity ([ef6ac08](https://github.com/m3idnotfree/twitch_oauth/commit/ef6ac08da5dd12a3c09578277446a6fde9d61238))

## [1.0.0](https://github.com/m3idnotfree/twitch_oauth/compare/v0.3.10..v1.0.0) - 2024-12-20

### Refactoring

- **(request)** make module exports more explicit ([9c0bd5b](https://github.com/m3idnotfree/twitch_oauth/commit/9c0bd5b1b30aaefa6bb609b7f76ab2f9673f07c5))
- **(request)** remove lifetimes all request ([c9e861a](https://github.com/m3idnotfree/twitch_oauth/commit/c9e861a62a26287d23e4fccdd02a0b5528dce52d))
- **(test)** move to tests directory ([5a041d7](https://github.com/m3idnotfree/twitch_oauth/commit/5a041d7ea0db7f0f6ec60dca3554d54ae1649798))
- **(lib, scopes)** make module exports more explicit ([2cec669](https://github.com/m3idnotfree/twitch_oauth/commit/2cec669728daf2076b9ebfcfc448f0b3db87b2b1))

## [0.3.10](https://github.com/m3idnotfree/twitch_oauth/compare/v0.3.9..v0.3.10) - 2024-12-20

### Features

- **(oauth)** expose redirect_url field as public ([c986d6d](https://github.com/m3idnotfree/twitch_oauth/commit/c986d6d18e7397ef1b8b37229060c229109ad9a0))

### Refactoring

- **(deps, features)** remove anyhow, rename feature ([81451c0](https://github.com/m3idnotfree/twitch_oauth/commit/81451c07e2e00e4195e9e96bece02a465fd71623))
- **(scopes)** remove needless lifetime, rename eventsub to read_eventsub method ([19f4469](https://github.com/m3idnotfree/twitch_oauth/commit/19f446975a5afa1a9e7a18bef689891622960f82))
- **(request)** remove lifetimes from AuthorizationRequest ([b019964](https://github.com/m3idnotfree/twitch_oauth/commit/b019964d56ab520f787fac52aad7638df68065a4))
- **(oauth)** simplify TwitchOauth struct ([d715aa5](https://github.com/m3idnotfree/twitch_oauth/commit/d715aa59e22a6159e10d02df9db5651cb5ad73c0))
- **(server)** replace SocketAddr with Url ([11e0888](https://github.com/m3idnotfree/twitch_oauth/commit/11e0888b6a184e7f5f9344adccd0de5d90153e6c))

### Documentation

- **(test_help)** update ([e5fc1e0](https://github.com/m3idnotfree/twitch_oauth/commit/e5fc1e05ff8f20b681ea756f05b122666ec69332))

### Miscellaneous

- **(gitignore)** update ([4e7f50e](https://github.com/m3idnotfree/twitch_oauth/commit/4e7f50e87562e89f9e30bd9a2c1ae799b3ac0c60))

### Other

- Update crates version ([e325f0b](https://github.com/m3idnotfree/twitch_oauth/commit/e325f0b4954462c1e4936c9167cbbcaabf0cfbed))
- Fix example ([300e94e](https://github.com/m3idnotfree/twitch_oauth/commit/300e94e40b5073bff870a6eef86e9f63208bd2fe))
- **(lib)** update example ([d326dfb](https://github.com/m3idnotfree/twitch_oauth/commit/d326dfbd26612a043933630e63daa4ca9ffa41a6))

## [0.3.9](https://github.com/m3idnotfree/twitch_oauth/compare/v0.3.8..v0.3.9) - 2024-11-12

### Refactoring

- [**breaking**] change scopes storage from Vec to HashSet ([b48df65](https://github.com/m3idnotfree/twitch_oauth/commit/b48df65c71bb4daaa9e13f7da25e7df57608e32c))

### Other

- authorize_request ([0e5cf27](https://github.com/m3idnotfree/twitch_oauth/commit/0e5cf27d4e6a37f0fdf18984c8751406173ac197))
- Update twitch_oauth auth_url ([bcc2133](https://github.com/m3idnotfree/twitch_oauth/commit/bcc2133b9e14ad9516dd30fa15bc3a00f2415f7d))
- test_access_token use HashSet instead of Vec ([b7fb3d8](https://github.com/m3idnotfree/twitch_oauth/commit/b7fb3d8e5e9fdf307393e59e292495eefc91b40c))

## [0.3.8](https://github.com/m3idnotfree/twitch_oauth/compare/v0.3.7..v0.3.8) - 2024-11-11

### Style

- run cargo fmt ([14a0342](https://github.com/m3idnotfree/twitch_oauth/commit/14a0342bf90b03436e91bcaf1c8e44c8506ac243))

### Other

- Update dependencies ([0737b56](https://github.com/m3idnotfree/twitch_oauth/commit/0737b5635f94d7dca8019ff52fd220b271bb8ffa))
- Update license ([c7b0403](https://github.com/m3idnotfree/twitch_oauth/commit/c7b0403f7fbefdeeb7153ff6076565ae023afb8d))

## [0.3.7](https://github.com/m3idnotfree/twitch_oauth/compare/v0.3.6..v0.3.7) - 2024-10-23

### Other

- fix toml format ([3325bdc](https://github.com/m3idnotfree/twitch_oauth/commit/3325bdcada671d8f1924f0fbf14a0248903f01bf))

## [0.3.6](https://github.com/m3idnotfree/twitch_oauth/compare/v0.3.5..v0.3.6) - 2024-10-23

### Other

- extract get_users_info and Users struct ([1565afd](https://github.com/m3idnotfree/twitch_oauth/commit/1565afde6ced08fe472b3a41f878bfa6b552cfe7))
- TwitchTest get_mock_users_info ([a0d3de4](https://github.com/m3idnotfree/twitch_oauth/commit/a0d3de4c7d2c2c15d2a8f2720424edf81a5e64f9))
- Update example ([c261b3c](https://github.com/m3idnotfree/twitch_oauth/commit/c261b3c0e88030b5a04226cd9aa38ebc4c551452))

## [0.3.5](https://github.com/m3idnotfree/twitch_oauth/compare/v0.3.4..v0.3.5) - 2024-10-23

### Other

- Add TwitchOauth new method ([50e6bb2](https://github.com/m3idnotfree/twitch_oauth/commit/50e6bb24409784d61b76db3c437439b02ab7c59e))
- Remove comment ([04d091d](https://github.com/m3idnotfree/twitch_oauth/commit/04d091d40f48d87296e8eb65082ad436335e819c))
- Update Cargo.toml ([8369632](https://github.com/m3idnotfree/twitch_oauth/commit/83696329ba787f55dcce6f521512f34372d1584b))
- Update use crates ([d66b04b](https://github.com/m3idnotfree/twitch_oauth/commit/d66b04b1a39a6e5f259d23a8df6eba37072cf85b))
- scopes_mut ([b8836aa](https://github.com/m3idnotfree/twitch_oauth/commit/b8836aaa163b561f4f6ff5e3c37b9ef7aa8cc5c2))
- ScopeBuldier -> ScopeMut ([0e00827](https://github.com/m3idnotfree/twitch_oauth/commit/0e008271d3f0ca28d0191eb03d93163096844838))
- Update lib ([ca77e9f](https://github.com/m3idnotfree/twitch_oauth/commit/ca77e9f16510c16146724555d560666c0ee35dd7))

## [0.3.4](https://github.com/m3idnotfree/twitch_oauth/compare/v0.3.3..v0.3.4) - 2024-10-22

### Other

- Update example ([da59589](https://github.com/m3idnotfree/twitch_oauth/commit/da59589f4270218cdbbbf6a3eaeee1db37d38606))
- twitch cli mock server oauth ([debc233](https://github.com/m3idnotfree/twitch_oauth/commit/debc233a09677ad147c09f7b2414b4293bfd89d9))

## [0.3.3](https://github.com/m3idnotfree/twitch_oauth/compare/v0.3.2..v0.3.3) - 2024-10-22

### Features

- [**breaking**] wrap API responses in OauthResponse type ([7b9d9f6](https://github.com/m3idnotfree/twitch_oauth/commit/7b9d9f69c4b9cf798c95e4576345eb7171b8f3a3))

### Other

- Add haeders to codetoken_request ([6325a22](https://github.com/m3idnotfree/twitch_oauth/commit/6325a22e166de442e269cfd68b6425614eaaf717))
- Add struct OauthResponse ([c8c9c55](https://github.com/m3idnotfree/twitch_oauth/commit/c8c9c55be85b1dab58e93cb96d83bc059b8541fa))
- Update codetoken_request ([bca8409](https://github.com/m3idnotfree/twitch_oauth/commit/bca840922f9b38da03b0411ada3b38143d39f3e0))

## [0.3.2](https://github.com/m3idnotfree/twitch_oauth/compare/v0.3.1..v0.3.2) - 2024-10-22

### Other

- unused field ([7ac6b9a](https://github.com/m3idnotfree/twitch_oauth/commit/7ac6b9a7881850cb0a85a3071db4c4f6a09c7961))
- Add Debug to CodeState ([cc0a5cc](https://github.com/m3idnotfree/twitch_oauth/commit/cc0a5ccaaced5fb83651e9148bde47f027eeff9d))

## [0.3.1](https://github.com/m3idnotfree/twitch_oauth/compare/v0.3.0..v0.3.1) - 2024-10-22

### Other

- Change location feature oneshot-server ([e966483](https://github.com/m3idnotfree/twitch_oauth/commit/e9664838caff6e2800c4394e80f96b59b3876034))
- Update features ([c526258](https://github.com/m3idnotfree/twitch_oauth/commit/c526258afc17ab6595a4d264802193053f936262))
- Cargo.toml ([694b422](https://github.com/m3idnotfree/twitch_oauth/commit/694b4227a28b19c254459934545bc434f446ef2e))
- Cargo.toml ([fef4feb](https://github.com/m3idnotfree/twitch_oauth/commit/fef4febccb74ac757a6fc10adc6cf258dbb4e982))
- feature oneshot-server ([11c72d3](https://github.com/m3idnotfree/twitch_oauth/commit/11c72d3d718d78e8ae1507586c09189dc1e1efdf))
- all ([4cde93a](https://github.com/m3idnotfree/twitch_oauth/commit/4cde93a85ce409cc940575c76c1eaa1994b8e4e0))
- Remove asknothingx2 optional ([9ddca13](https://github.com/m3idnotfree/twitch_oauth/commit/9ddca1387f0abe2b6e46fc052c1fa8dbcc30154e))

## [0.3.0](https://github.com/m3idnotfree/twitch_oauth/compare/v0.2.5..v0.3.0) - 2024-10-22

### Miscellaneous

- **(cargo.toml)** tokio feature enable macro ([7cf0908](https://github.com/m3idnotfree/twitch_oauth/commit/7cf09086c38cb52ffc4b45e9f443245405596f2d))

### Other

- Remove oauth2 crate add asknothingx2-util ([a410344](https://github.com/m3idnotfree/twitch_oauth/commit/a4103444d7f983bd88589ffa35100ea1a34f207a))
- exchange crate oauth2 -> asknothingx2-util ([945eb74](https://github.com/m3idnotfree/twitch_oauth/commit/945eb74e72739b1b1c501865e39698caf3ae9140))
- Update all ([5074c8e](https://github.com/m3idnotfree/twitch_oauth/commit/5074c8e335c0c0a68c35bd0c15c5d8bfff7fc2a0))
- Add client credentials headers ([f8abd42](https://github.com/m3idnotfree/twitch_oauth/commit/f8abd4297deeb0cb52d592753f1224858f8f4dfe))
- extract fn POST_headers ([d252dd8](https://github.com/m3idnotfree/twitch_oauth/commit/d252dd86ac3de0f63b2faafe7f1b441327381d82))
- oauth_request ([e189bfa](https://github.com/m3idnotfree/twitch_oauth/commit/e189bfa89c1f63686028f7ae9f304c21792f330e))
- ScopeBuilder ([a006e4d](https://github.com/m3idnotfree/twitch_oauth/commit/a006e4dcf1fafdd7aa554bf99403ed7e540e3312))
- use ScopeBuilder for the scope field ([dbde8a1](https://github.com/m3idnotfree/twitch_oauth/commit/dbde8a145d623b9bd30b6f7958c3905a8d273b0a))
- client_credentials update ([c808eaa](https://github.com/m3idnotfree/twitch_oauth/commit/c808eaa10875305adc5a68980892a2f316ee34b3))
- Add enum Scopes ([7cd0d6c](https://github.com/m3idnotfree/twitch_oauth/commit/7cd0d6c06c8910edd5ed0909fea0a35ad4fb5b81))
- Use Scopes param instead of &str ([fa7bec6](https://github.com/m3idnotfree/twitch_oauth/commit/fa7bec638a0a74d9d3f4232f48633f91fab6384e))
- Scopes ([31b356f](https://github.com/m3idnotfree/twitch_oauth/commit/31b356fd06704c23afd792e9086a3d2180d5aedd))
- twitch cli mockserver auth ([561c25f](https://github.com/m3idnotfree/twitch_oauth/commit/561c25f9f7a6a0c8c0011bf8b39ca7eea7e60ecd))
- Update gitignore ([4744401](https://github.com/m3idnotfree/twitch_oauth/commit/4744401b05cccfff4d62658710f72e555b928c87))
- Update Cargo.toml ([dd9935d](https://github.com/m3idnotfree/twitch_oauth/commit/dd9935d9a4308061e04377104ed5a826d37f5f66))

## [0.2.5](https://github.com/m3idnotfree/twitch_oauth/compare/v0.2.4..v0.2.5) - 2024-10-20

### Features

- **(lib)** re-export oauth2 newtypes ([adadb97](https://github.com/m3idnotfree/twitch_oauth/commit/adadb978050aa4fd10dcb9f1b1427e5d0c3edd98))

### Miscellaneous

- **(cargo.toml)** tokio features update ([59039dd](https://github.com/m3idnotfree/twitch_oauth/commit/59039dded35903d054154d03e0678a39a8436a11))

## [0.2.4](https://github.com/m3idnotfree/twitch_oauth/compare/v0.2.3..v0.2.4) - 2024-10-20

### Features

- **(types)** grant_type add user_token ([1d1ef60](https://github.com/m3idnotfree/twitch_oauth/commit/1d1ef6005589400541c581e4ba675e43ad0499d0))

## [0.2.3](https://github.com/m3idnotfree/twitch_oauth/compare/v0.2.2..v0.2.3) - 2024-10-20

### Features

- **(twitch_oauth)** set_urls ([87f612a](https://github.com/m3idnotfree/twitch_oauth/commit/87f612a0630aecd0c26fb4b934d843bb87f98a03))

## [0.2.2](https://github.com/m3idnotfree/twitch_oauth/compare/v0.2.1..v0.2.2) - 2024-10-19

### Bug Fixes

- **(client_cretentials)** pub field ([681724c](https://github.com/m3idnotfree/twitch_oauth/commit/681724cf0d5d323a63de126826f0761bb893a99d))

## [0.2.1](https://github.com/m3idnotfree/twitch_oauth/compare/v0.2.0..v0.2.1) - 2024-10-19

### Miscellaneous

- **(lib)** example update ([c9ed127](https://github.com/m3idnotfree/twitch_oauth/commit/c9ed1270ce6e4770fbe17e2ee17dc8e9e2efacf8))

## [0.2.0](https://github.com/m3idnotfree/twitch_oauth/compare/v0.1.1..v0.2.0) - 2024-10-19

### Features

- **(client_credentials)** enable client credentials ([42c1a2d](https://github.com/m3idnotfree/twitch_oauth/commit/42c1a2d787674f86c313405d3805868d09c709c3))

## [0.1.1](https://github.com/m3idnotfree/twitch_oauth/compare/v0.1.0..v0.1.1) - 2024-10-19

### Features

- **(twitchOauth)** pub client_id filed ([ad39b3f](https://github.com/m3idnotfree/twitch_oauth/commit/ad39b3fe34475c40a127e9556214a50fd90bbb13))

## [0.1.0](https://github.com/m3idnotfree/twitch_oauth/compare/v0.0.4..v0.1.0) - 2024-10-19

### Features

- **(util)** cli ([9ad50f6](https://github.com/m3idnotfree/twitch_oauth/commit/9ad50f6845642ed7f3f6c95f7e9d88b159488dbc))
- **(v2)** update v2 ([f2d4b68](https://github.com/m3idnotfree/twitch_oauth/commit/f2d4b68a77a3afa743341b4c9ab1bd9b4cfa8e73))
- **(v2)** exchange_code return Result ([f211b1d](https://github.com/m3idnotfree/twitch_oauth/commit/f211b1d004f10b0a9c60a83e4931e5799b20a01a))
- **(v2)** refresh token ([078f1cf](https://github.com/m3idnotfree/twitch_oauth/commit/078f1cf8b1b1a3b47a0ecbb720a4a70fe166e51c))
- **(v2)** revoke, validate token ([d857fdb](https://github.com/m3idnotfree/twitch_oauth/commit/d857fdbd7f3a50d5f093804a4c47de4992a77a91))
- **(v2)** use, set_revocation_uri, type ([4089841](https://github.com/m3idnotfree/twitch_oauth/commit/4089841a40de60c2ff77e0b76070d75db7465f20))
- **(v2)** validate_token ([9cd4ca6](https://github.com/m3idnotfree/twitch_oauth/commit/9cd4ca65b4304e66d6d8c9bc7ccf921b07027ec5))
- **(TwitchOauth)** derive(Clone) ([7a72407](https://github.com/m3idnotfree/twitch_oauth/commit/7a724072c77f7c1b932b9fba711ba076a4e00ab5))
- **(types)** token, errorresponse, response_type, grant_type ([76ec1e3](https://github.com/m3idnotfree/twitch_oauth/commit/76ec1e3f04df19e4df2c63967cd46cd519921d8e))
- **(traits)** OauthRequest trait ([87fa260](https://github.com/m3idnotfree/twitch_oauth/commit/87fa2601b97a07213758499b0ba9a40feb0f1b3e))
- **(error)** first commit ([23ee526](https://github.com/m3idnotfree/twitch_oauth/commit/23ee526bf8b76ba081e8eee6200bb4c831a5ef98))
- **(request)** authorize_request struct ([5118bce](https://github.com/m3idnotfree/twitch_oauth/commit/5118bce57fdea0b27578e2fea63f8d371e01a540))
- **(request)** codetoken_request struct ([ef1a908](https://github.com/m3idnotfree/twitch_oauth/commit/ef1a9087eddbca64f785d612149c6af37302553a))
- **(request)** refresh_request struct ([f46ea2c](https://github.com/m3idnotfree/twitch_oauth/commit/f46ea2c5c874b13e949d7ac7fcc1a7d84da9b47a))
- **(request)** revoke_request struct ([a25d2a0](https://github.com/m3idnotfree/twitch_oauth/commit/a25d2a054e9e71c4fd2190b8f6bbc1876dbfe49b))
- **(request)** validate_request struct, validateUrl newtype ([de85a6a](https://github.com/m3idnotfree/twitch_oauth/commit/de85a6afb072bc41b6779958159b6ebe1bb3eed2))
- **(request)** oauth_request function ([80e0dfa](https://github.com/m3idnotfree/twitch_oauth/commit/80e0dfac61df285738299148d2c9239447db5b18))
- **(twitch_oauth)** twitch_oauth struct ([0a79d0d](https://github.com/m3idnotfree/twitch_oauth/commit/0a79d0d65cf03e1f3efdf03c3871b3fd9a216ad7))
- **(oauth_oneshot_server)** oauth oneshot server ([2008048](https://github.com/m3idnotfree/twitch_oauth/commit/20080488692325c215963facbd1ce24bf4518964))

### Bug Fixes

- **(twitchOauth)** remove async ([e06630d](https://github.com/m3idnotfree/twitch_oauth/commit/e06630d0110fb38f175f540cc043ac14b6311e0f))
- **(TwitchOauth)** pub BasicClient ([5b1999d](https://github.com/m3idnotfree/twitch_oauth/commit/5b1999ddda712e2f35fd2209456c92e5c5680bc6))
- **(request)** validate_request &name -> ValidateUrl ([d480399](https://github.com/m3idnotfree/twitch_oauth/commit/d480399e617b4d15a45a19c0b4b0b23c8f762b49))

### Refactoring

- [**breaking**] migrate to v2 API architecture ([2e29721](https://github.com/m3idnotfree/twitch_oauth/commit/2e29721d7d9c14ef6ee4424c5e0d0a0935f9bc5a))
- **(v2)** move server ([6a8db7e](https://github.com/m3idnotfree/twitch_oauth/commit/6a8db7eceea1ea3e2aec6002f85e293e0b0ff8c7))
- **(auth_url)** scopes -> Vec<String> ([d7a7db2](https://github.com/m3idnotfree/twitch_oauth/commit/d7a7db21f1a2967fe1cdf62b9c86285fd1dad009))
- **(TwitchOauth)** remove all parameter pkce_challenge ([d7c2426](https://github.com/m3idnotfree/twitch_oauth/commit/d7c2426c6640db39fb2bf2429584c292a2a41235))

### Documentation

- **(lib)** example update ([5956942](https://github.com/m3idnotfree/twitch_oauth/commit/59569421f7c38156d38db2faea91171850380896))

### Style

- run cargo fmt ([46cd156](https://github.com/m3idnotfree/twitch_oauth/commit/46cd1562f2bb004012d0e04db44a4794a4084b15))

### Tests

- update ([5324a36](https://github.com/m3idnotfree/twitch_oauth/commit/5324a3650bd599b63c2181e9b511c997551b495f))
- update ([6d0acb9](https://github.com/m3idnotfree/twitch_oauth/commit/6d0acb96415b2db9e88a1aefb398b9ebdeb4ed9d))

### Miscellaneous

- update gitignore, lib.rs ([1ea0e14](https://github.com/m3idnotfree/twitch_oauth/commit/1ea0e1422544e07786a39dafcdd96e2ca4095b3e))
- **(lib)** remove comment ([dfbdecc](https://github.com/m3idnotfree/twitch_oauth/commit/dfbdecc4bd1db4fde4b1f95bd0fb1f086d5cdb10))
- remove pkce, util files ([f4d56d4](https://github.com/m3idnotfree/twitch_oauth/commit/f4d56d4029fc19bfc9a45e30e1f103e4a3284f30))
- **(cago.toml)** update ([9d8dc79](https://github.com/m3idnotfree/twitch_oauth/commit/9d8dc79677afa720c11c0fa969ab903b1c33e4fc))
- **(gitignore)** update ([f74f19c](https://github.com/m3idnotfree/twitch_oauth/commit/f74f19c21995c417074196c918516b890c91827e))
- update ([3f53e0e](https://github.com/m3idnotfree/twitch_oauth/commit/3f53e0efc6282e15c29ca4f6ea16d91eaddb177b))
- **(cargo)** update ([9fc94bb](https://github.com/m3idnotfree/twitch_oauth/commit/9fc94bb161e848f316c58e9278acee68b69526ad))
- **(delete)** some file ([576ec81](https://github.com/m3idnotfree/twitch_oauth/commit/576ec81d59705b8797bf31e0a4e27bb94d9fcbc9))
- **(all)** format, rename, remove comment ([da94fce](https://github.com/m3idnotfree/twitch_oauth/commit/da94fce0d8366bb31032664769c7878c29b8c711))
- **(README)** update ([5384c9a](https://github.com/m3idnotfree/twitch_oauth/commit/5384c9a9c775575d7903e02f4cc1025c274b0a43))

### Other

- test/pkce ([ee56507](https://github.com/m3idnotfree/twitch_oauth/commit/ee565077b0405766ac82a6c92186cc5c4cc2b2b9))

## [0.0.4](https://github.com/m3idnotfree/twitch_oauth/compare/v0.0.3..v0.0.4) - 2024-03-02

### Features

- add methods refresh_token, validate_token ([b776ebb](https://github.com/m3idnotfree/twitch_oauth/commit/b776ebbd636a8e7a9a00a824828fc3c04d6a082f))

## [0.0.3](https://github.com/m3idnotfree/twitch_oauth/compare/v0.0.2..v0.0.3) - 2024-02-29

### Bug Fixes

- **(error)** consistent word order ([4a0754c](https://github.com/m3idnotfree/twitch_oauth/commit/4a0754c1ff556c652b8047842e0488968cdc91db))

## [0.0.2](https://github.com/m3idnotfree/twitch_oauth/compare/v0.0.1..v0.0.2) - 2024-02-29

### Features

- add readme ([167fb3d](https://github.com/m3idnotfree/twitch_oauth/commit/167fb3d6d3930e84f020dd63867a68fb04f5bb2a))

### Bug Fixes

- crate name ([3106bcb](https://github.com/m3idnotfree/twitch_oauth/commit/3106bcb56473ae13969a5f93aee4145cac9986cd))

### Documentation

- add ignore ([8d16245](https://github.com/m3idnotfree/twitch_oauth/commit/8d1624557d0fd6894ac8ea544412db08af2b0f14))

### Miscellaneous

- rename crate ([2be523a](https://github.com/m3idnotfree/twitch_oauth/commit/2be523abca4cd590008e054f3a859e6dd6605222))

## [0.0.1] - 2024-02-29

### Features

- initialize project ([4b45ad6](https://github.com/m3idnotfree/twitch_oauth/commit/4b45ad6c145b2324ea26661563f0276568fba1e8))
- all made &str ([4c54a2f](https://github.com/m3idnotfree/twitch_oauth/commit/4c54a2f5a3506e88438354d34ccc52a7969d437a))

### Miscellaneous

- **(license)** add LICENSE-APACHE ([93a996c](https://github.com/m3idnotfree/twitch_oauth/commit/93a996cc9d6a751d3a12bbb345e8c9a0128173c7))
- **(license)** add LICENSE-MIT ([ca33be1](https://github.com/m3idnotfree/twitch_oauth/commit/ca33be178b0bc4207cd54305f9658405dfd31c87))
- **(pkce)** PkceError -> Error ([1183cc0](https://github.com/m3idnotfree/twitch_oauth/commit/1183cc08f9ae4e827220b79d32b8d76caceef02c))
- **(pkce)** remove println ([fb1588f](https://github.com/m3idnotfree/twitch_oauth/commit/fb1588f6f9010021290d06d9ddbd4aeb434f2748))
- **(cargo.toml)** reqwest features rustls-tls ([e5efe50](https://github.com/m3idnotfree/twitch_oauth/commit/e5efe509f5420923a1d7232cac90c9e0cf75bc05))
