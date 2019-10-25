use crate::{
    client_msg::Get,
    notify_msg::{EmailResetPassword, EmailUpdateEmail, EmailUpdatePassword},
    AuditBuilder, AuditMeta, ClientActor, CoreError, CoreResult, CsrfCreate, Driver, Jwt,
    JwtClaimsType, KeyRead, KeyType, KeyWithValue, NotifyActor, Service, ServiceRead, User,
    UserPasswordMeta, UserRead, UserToken,
};
use actix::Addr;
use futures::{future, Future};
use libreauth::oath::TOTPBuilder;
use sha1::{Digest, Sha1};
use uuid::Uuid;

/// Authentication functions.
#[derive(Debug)]
pub struct Auth;

// TODO(refactor): Move this logic, other core methods into api/driver.

impl Auth {
    /// Returns password strength and pwned checks.
    pub fn password_meta(
        enabled: bool,
        client: &Addr<ClientActor>,
        password: Option<&str>,
    ) -> impl Future<Item = UserPasswordMeta, Error = CoreError> {
        match password {
            Some(password) => {
                let password_strength = Self::password_meta_strength(password).then(|r| match r {
                    Ok(entropy) => future::ok(Some(entropy.score)),
                    Err(err) => {
                        warn!("{}", err);
                        future::ok(None)
                    }
                });
                let password_pwned =
                    Self::password_meta_pwned(enabled, client, password).then(|r| match r {
                        Ok(password_pwned) => future::ok(Some(password_pwned)),
                        Err(err) => {
                            warn!("{}", err);
                            future::ok(None)
                        }
                    });
                future::Either::A(password_strength.join(password_pwned).map(
                    |(password_strength, password_pwned)| UserPasswordMeta {
                        password_strength,
                        password_pwned,
                    },
                ))
            }
            None => future::Either::B(future::ok(UserPasswordMeta::default())),
        }
    }

    /// Returns password strength test performed by `zxcvbn`.
    /// <https://github.com/shssoichiro/zxcvbn-rs>
    fn password_meta_strength(
        password: &str,
    ) -> impl Future<Item = zxcvbn::Entropy, Error = CoreError> {
        // TODO(fix): Fix "Zxcvbn cannot evaluate a blank password" warning.
        future::result(zxcvbn::zxcvbn(password, &[]).map_err(CoreError::Zxcvbn))
    }

    /// Returns true if password is present in `Pwned Passwords` index, else false.
    /// <https://haveibeenpwned.com/Passwords>
    fn password_meta_pwned(
        enabled: bool,
        client: &Addr<ClientActor>,
        password: &str,
    ) -> impl Future<Item = bool, Error = CoreError> {
        if enabled {
            // Make request to API using first 5 characters of SHA1 password hash.
            let mut hash = Sha1::new();
            hash.input(password);
            let hash = format!("{:X}", hash.result());
            let route = format!("/range/{:.5}", hash);

            future::Either::A(
                // Make API request.
                client
                    .send(Get::new("https://api.pwnedpasswords.com", route))
                    .map_err(CoreError::ActixMailbox)
                    .and_then(|res| res.map_err(CoreError::Client))
                    // Compare suffix of hash to lines to determine if password is pwned.
                    .and_then(move |text| {
                        for line in text.lines() {
                            if hash[5..] == line[..35] {
                                return Ok(true);
                            }
                        }
                        Ok(false)
                    }),
            )
        } else {
            future::Either::B(future::err(CoreError::PwnedPasswordsDisabled))
        }
    }

    pub fn notify_email_reset_password(
        notify: &Addr<NotifyActor>,
        service: Service,
        user: User,
        token: String,
        audit: AuditMeta,
    ) -> CoreResult<()> {
        notify
            .try_send(EmailResetPassword::new(service, user, token, audit))
            .map_err(|_err| CoreError::NotifySendError)
    }

    pub fn encode_reset_password_token(
        driver: &dyn Driver,
        service: &Service,
        user: &User,
        key: &KeyWithValue,
        token_expires: i64,
    ) -> CoreResult<String> {
        let csrf = driver
            .csrf_create(&CsrfCreate::generate(token_expires, service.id))
            .map_err(CoreError::Driver)?;
        let (token, _) = Jwt::encode_token_csrf(
            service.id,
            user.id,
            JwtClaimsType::ResetPasswordToken,
            &csrf.key,
            &key.value,
            token_expires,
        )?;
        Ok(token)
    }

    pub fn decode_reset_password_token(
        service: &Service,
        user: &User,
        key: &KeyWithValue,
        token: &str,
    ) -> CoreResult<String> {
        let decoded = Jwt::decode_token(
            service.id,
            user.id,
            JwtClaimsType::ResetPasswordToken,
            &key.value,
            &token,
        );
        match decoded {
            Ok((_, csrf_key)) => csrf_key.ok_or_else(|| CoreError::CsrfNotFoundOrUsed),
            Err(_err) => Err(CoreError::JwtInvalidOrExpired),
        }
    }

    pub fn encode_update_email_token(
        driver: &dyn Driver,
        service: &Service,
        user: &User,
        key: &KeyWithValue,
        token_expires: i64,
    ) -> CoreResult<String> {
        let csrf = driver
            .csrf_create(&CsrfCreate::generate(token_expires, service.id))
            .map_err(CoreError::Driver)?;
        let (revoke_token, _) = Jwt::encode_token_csrf(
            service.id,
            user.id,
            JwtClaimsType::UpdateEmailRevokeToken,
            &csrf.key,
            &key.value,
            token_expires,
        )?;
        Ok(revoke_token)
    }

    pub fn decode_update_email_token(
        service: &Service,
        user: &User,
        key: &KeyWithValue,
        token: &str,
    ) -> CoreResult<String> {
        let decoded = Jwt::decode_token(
            service.id,
            user.id,
            JwtClaimsType::UpdateEmailRevokeToken,
            &key.value,
            &token,
        );
        match decoded {
            Ok((_, csrf_key)) => csrf_key.ok_or_else(|| CoreError::CsrfNotFoundOrUsed),
            Err(_err) => Err(CoreError::JwtInvalidOrExpired),
        }
    }

    pub fn notify_email_update_email(
        notify: &Addr<NotifyActor>,
        service: Service,
        user: User,
        old_email: String,
        revoke_token: String,
        audit: AuditMeta,
    ) -> CoreResult<()> {
        notify
            .try_send(EmailUpdateEmail::new(
                service,
                user,
                old_email,
                revoke_token,
                audit,
            ))
            .map_err(|_err| CoreError::NotifySendError)
    }

    pub fn encode_update_password_token(
        driver: &dyn Driver,
        service: &Service,
        user: &User,
        key: &KeyWithValue,
        token_expires: i64,
    ) -> CoreResult<String> {
        let csrf = driver
            .csrf_create(&CsrfCreate::generate(token_expires, service.id))
            .map_err(CoreError::Driver)?;
        let (revoke_token, _) = Jwt::encode_token_csrf(
            service.id,
            user.id,
            JwtClaimsType::UpdatePasswordRevokeToken,
            &csrf.key,
            &key.value,
            token_expires,
        )?;
        Ok(revoke_token)
    }

    pub fn notify_email_update_password(
        notify: &Addr<NotifyActor>,
        service: Service,
        user: User,
        revoke_token: String,
        audit: AuditMeta,
    ) -> CoreResult<()> {
        notify
            .try_send(EmailUpdatePassword::new(service, user, revoke_token, audit))
            .map_err(|_err| CoreError::NotifySendError)
    }

    pub fn decode_update_password_token(
        service: &Service,
        user: &User,
        key: &KeyWithValue,
        token: &str,
    ) -> CoreResult<String> {
        let decoded = Jwt::decode_token(
            service.id,
            user.id,
            JwtClaimsType::UpdatePasswordRevokeToken,
            &key.value,
            &token,
        );
        match decoded {
            Ok((_, csrf_key)) => csrf_key.ok_or_else(|| CoreError::CsrfNotFoundOrUsed),
            Err(_err) => Err(CoreError::JwtInvalidOrExpired),
        }
    }

    pub fn decode_access_token(
        service: &Service,
        user: &User,
        key: &KeyWithValue,
        token: &str,
    ) -> CoreResult<i64> {
        let decoded = Jwt::decode_token(
            service.id,
            user.id,
            JwtClaimsType::AccessToken,
            &key.value,
            &token,
        );
        match decoded {
            Ok((access_token_expires, _)) => Ok(access_token_expires),
            Err(_err) => Err(CoreError::JwtInvalidOrExpired),
        }
    }

    pub fn decode_refresh_token(
        service: &Service,
        user: &User,
        key: &KeyWithValue,
        token: &str,
    ) -> CoreResult<String> {
        let decoded = Jwt::decode_token(
            service.id,
            user.id,
            JwtClaimsType::RefreshToken,
            &key.value,
            &token,
        );
        match decoded {
            Ok((_, csrf_key)) => csrf_key.ok_or_else(|| CoreError::CsrfNotFoundOrUsed),
            Err(_err) => Err(CoreError::JwtInvalidOrExpired),
        }
    }

    pub fn decode_csrf_key(
        service: &Service,
        user: &User,
        key: &KeyWithValue,
        token_type: JwtClaimsType,
        token: &str,
    ) -> CoreResult<Option<String>> {
        match Jwt::decode_token(service.id, user.id, token_type, &key.value, &token) {
            Ok((_, csrf_key)) => Ok(csrf_key),
            Err(_err) => Err(CoreError::JwtInvalidOrExpired),
        }
    }

    /// TOTP code verification.
    pub fn totp(key: &str, totp_code: &str) -> CoreResult<()> {
        let totp = TOTPBuilder::new()
            .base32_key(key)
            .finalize()
            .map_err(CoreError::libreauth_oath)?;

        if !totp.is_valid(&totp_code) {
            Err(CoreError::TotpInvalid)
        } else {
            Ok(())
        }
    }

    /// Authenticate root key.
    pub fn authenticate_root(
        driver: &dyn Driver,
        audit: &mut AuditBuilder,
        key_value: Option<String>,
    ) -> CoreResult<()> {
        match key_value {
            Some(key_value) => {
                let read = KeyRead::RootValue(key_value);
                driver
                    .key_read(&read)
                    .map_err(CoreError::Driver)?
                    .ok_or_else(|| CoreError::KeyNotFound)
                    .map(|key| {
                        audit.key(Some(&key));
                        key
                    })
                    .map(|_key| ())
            }
            None => Err(CoreError::KeyUndefined),
        }
    }

    /// Authenticate service key.
    pub fn authenticate_service(
        driver: &dyn Driver,
        audit: &mut AuditBuilder,
        key_value: Option<String>,
    ) -> CoreResult<Service> {
        Auth::authenticate_service_try(driver, audit, key_value)
    }

    /// Authenticate service or root key.
    pub fn authenticate(
        driver: &dyn Driver,
        audit: &mut AuditBuilder,
        key_value: Option<String>,
    ) -> CoreResult<Option<Service>> {
        let key_value_1 = key_value.to_owned();

        Auth::authenticate_service_try(driver, audit, key_value)
            .map(Some)
            .or_else(move |_err| Auth::authenticate_root(driver, audit, key_value_1).map(|_| None))
    }

    fn authenticate_service_try(
        driver: &dyn Driver,
        audit: &mut AuditBuilder,
        key_value: Option<String>,
    ) -> CoreResult<Service> {
        match key_value {
            Some(key_value) => driver
                .key_read(&KeyRead::ServiceValue(key_value))
                .map_err(CoreError::Driver)?
                .ok_or_else(|| CoreError::KeyNotFound)
                .and_then(|key| key.service_id.ok_or_else(|| CoreError::KeyServiceUndefined))
                .and_then(|service_id| Auth::authenticate_service_inner(driver, audit, service_id)),
            None => Err(CoreError::KeyUndefined),
        }
    }

    fn authenticate_service_inner(
        driver: &dyn Driver,
        audit: &mut AuditBuilder,
        service_id: Uuid,
    ) -> CoreResult<Service> {
        let service = driver
            .service_read(&ServiceRead::new(service_id))?
            .ok_or_else(|| CoreError::ServiceNotFound)?
            .check()?;
        audit.service(Some(&service));
        Ok(service)
    }

    /// Read user by ID.
    /// Checks user is enabled, returns bad request if disabled.
    pub fn user_read_by_id(
        driver: &dyn Driver,
        _service_mask: Option<&Service>,
        audit: &mut AuditBuilder,
        id: Uuid,
    ) -> CoreResult<User> {
        let read = UserRead::Id(id);
        let user = driver
            .user_read(&read)
            .map_err(CoreError::Driver)?
            .ok_or_else(|| CoreError::UserNotFound)?;
        audit.user(Some(&user));
        if !user.is_enabled {
            return Err(CoreError::UserDisabled);
        }
        Ok(user)
    }

    /// Unchecked read user by ID.
    /// Does not check user is enabled.
    pub fn user_read_by_id_unchecked(
        driver: &dyn Driver,
        _service_mask: Option<&Service>,
        audit: &mut AuditBuilder,
        id: Uuid,
    ) -> CoreResult<User> {
        let read = UserRead::Id(id);
        let user = driver
            .user_read(&read)
            .map_err(CoreError::Driver)?
            .ok_or_else(|| CoreError::UserNotFound)?;
        audit.user(Some(&user));
        Ok(user)
    }

    /// Read user by email address.
    /// Also checks user is enabled, returns bad request if disabled.
    pub fn user_read_by_email(
        driver: &dyn Driver,
        _service_mask: Option<&Service>,
        audit: &mut AuditBuilder,
        email: String,
    ) -> CoreResult<User> {
        let read = UserRead::Email(email);
        let user = driver
            .user_read(&read)
            .map_err(CoreError::Driver)?
            .ok_or_else(|| CoreError::UserNotFound)?;
        audit.user(Some(&user));
        if !user.is_enabled {
            return Err(CoreError::UserDisabled);
        }
        Ok(user)
    }

    /// Read key by user reference and key type.
    /// Also checks key is enabled and not revoked, returns bad request if disabled.
    pub fn key_read_by_user(
        driver: &dyn Driver,
        service: &Service,
        audit: &mut AuditBuilder,
        user: &User,
        key_type: KeyType,
    ) -> CoreResult<KeyWithValue> {
        let key = driver
            .key_read(&KeyRead::user_id(
                service.id, user.id, true, false, key_type,
            ))
            .map_err(CoreError::Driver)?
            .ok_or_else(|| CoreError::KeyNotFound)?;
        audit.user_key(Some(&key));
        if !key.is_enabled {
            Err(CoreError::KeyDisabled)
        } else if key.is_revoked {
            Err(CoreError::KeyRevoked)
        } else {
            Ok(key)
        }
    }

    /// Unchecked read key by user reference.
    /// Does not check key is enabled or not revoked.
    pub fn key_read_by_user_unchecked(
        driver: &dyn Driver,
        service: &Service,
        audit: &mut AuditBuilder,
        user: &User,
        key_type: KeyType,
    ) -> CoreResult<KeyWithValue> {
        let key = driver
            .key_read(&KeyRead::user_id(
                service.id, user.id, true, false, key_type,
            ))
            .map_err(CoreError::Driver)?
            .ok_or_else(|| CoreError::KeyNotFound)?;
        audit.user_key(Some(&key));
        Ok(key)
    }

    /// Read key by user value.
    /// Also checks key is enabled and not revoked, returns bad request if disabled.
    pub fn key_read_by_user_value(
        driver: &dyn Driver,
        service: &Service,
        audit: &mut AuditBuilder,
        key: String,
        key_type: KeyType,
    ) -> CoreResult<KeyWithValue> {
        let key = driver
            .key_read(&KeyRead::user_value(service.id, key, true, false, key_type))
            .map_err(CoreError::Driver)?
            .ok_or_else(|| CoreError::KeyNotFound)?;
        audit.user_key(Some(&key));
        if !key.is_enabled {
            Err(CoreError::KeyDisabled)
        } else if key.is_revoked {
            Err(CoreError::KeyRevoked)
        } else {
            Ok(key)
        }
    }

    /// Unchecked read key by user value.
    /// Does not check key is enabled and not revoked.
    pub fn key_read_by_user_value_unchecked(
        driver: &dyn Driver,
        service: &Service,
        audit: &mut AuditBuilder,
        key: String,
        key_type: KeyType,
    ) -> CoreResult<KeyWithValue> {
        let key = driver
            .key_read(&KeyRead::user_value(service.id, key, true, false, key_type))
            .map_err(CoreError::Driver)?
            .ok_or_else(|| CoreError::KeyNotFound)?;
        audit.user_key(Some(&key));
        Ok(key)
    }

    /// Build user token by encoding access and refresh tokens.
    pub fn encode_user_token(
        driver: &dyn Driver,
        service: &Service,
        user: User,
        key: &KeyWithValue,
        access_token_expires: i64,
        refresh_token_expires: i64,
    ) -> CoreResult<UserToken> {
        let csrf = driver
            .csrf_create(&CsrfCreate::generate(refresh_token_expires, service.id))
            .map_err(CoreError::Driver)?;
        let (access_token, access_token_expires) = Jwt::encode_token(
            service.id,
            user.id,
            JwtClaimsType::AccessToken,
            &key.value,
            access_token_expires,
        )?;
        let (refresh_token, refresh_token_expires) = Jwt::encode_token_csrf(
            service.id,
            user.id,
            JwtClaimsType::RefreshToken,
            &csrf.key,
            &key.value,
            refresh_token_expires,
        )?;
        Ok(UserToken {
            user,
            access_token,
            access_token_expires,
            refresh_token,
            refresh_token_expires,
        })
    }
}
