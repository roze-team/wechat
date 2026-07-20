use aes::Aes256;
use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose, Engine};
use cbc::{Decryptor, Encryptor};
use cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use hmac::{Hmac, Mac};
use rand::{distributions::Alphanumeric, Rng};
use rsa::{
    pkcs1v15::{Signature, SigningKey, VerifyingKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    signature::{RandomizedSigner, SignatureEncoding, Verifier},
    Oaep, RsaPrivateKey, RsaPublicKey,
};
use sha1::{Digest as Sha1Digest, Sha1};
use sha2::Sha256;

use crate::error::{Result, WechatError};

type HmacSha256 = Hmac<Sha256>;
type Aes256CbcEncryptor = Encryptor<Aes256>;
type Aes256CbcDecryptor = Decryptor<Aes256>;

pub fn nonce_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn sha1_signature(parts: &[&str]) -> String {
    let mut sorted = parts.to_vec();
    sorted.sort_unstable();
    let payload = sorted.join("");
    sha1_hex(payload.as_bytes())
}

pub fn sha1_hex(payload: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(payload);
    hex::encode(hasher.finalize())
}

pub fn verify_callback_signature(
    token: &str,
    timestamp: &str,
    nonce: &str,
    signature: &str,
) -> bool {
    sha1_signature(&[token, timestamp, nonce]) == signature
}

pub fn hmac_sha256_hex(secret: &[u8], message: &[u8]) -> Result<String> {
    let mut mac = <HmacSha256 as Mac>::new_from_slice(secret)
        .map_err(|err| WechatError::Crypto(format!("invalid hmac key: {err}")))?;
    mac.update(message);
    Ok(hex::encode(mac.finalize().into_bytes()))
}

pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

pub fn payment_v3_message(
    method: &str,
    url_path_query: &str,
    timestamp: i64,
    nonce: &str,
    body: &str,
) -> String {
    format!("{method}\n{url_path_query}\n{timestamp}\n{nonce}\n{body}\n")
}

pub fn payment_legacy_sign(params: &[(String, String)], api_key: &str) -> String {
    let mut pairs = params
        .iter()
        .filter(|(key, value)| key != "sign" && !value.is_empty())
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect::<Vec<_>>();
    pairs.sort_unstable_by(|left, right| left.0.cmp(right.0));

    let mut payload = pairs
        .into_iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join("&");
    payload.push_str("&key=");
    payload.push_str(api_key);

    format!("{:x}", md5::compute(payload.as_bytes())).to_uppercase()
}

pub fn payment_legacy_xml(params: &[(String, String)]) -> String {
    let mut xml = String::from("<xml>");
    for (key, value) in params {
        xml.push('<');
        xml.push_str(key);
        xml.push_str("><![CDATA[");
        xml.push_str(&value.replace("]]>", "]]]]><![CDATA[>"));
        xml.push_str("]]></");
        xml.push_str(key);
        xml.push('>');
    }
    xml.push_str("</xml>");
    xml
}

pub fn rsa_sha256_sign_base64(private_key_pem: &str, message: &[u8]) -> Result<String> {
    let private_key = RsaPrivateKey::from_pkcs8_pem(private_key_pem)
        .map_err(|err| WechatError::Crypto(format!("invalid private key: {err}")))?;
    let signing_key = SigningKey::<Sha256>::new(private_key);
    let mut rng = rand::thread_rng();
    let signature = signing_key.sign_with_rng(&mut rng, message);
    Ok(general_purpose::STANDARD.encode(signature.to_bytes()))
}

pub fn rsa_sha256_verify_base64(
    public_key_pem: &str,
    message: &[u8],
    signature_base64: &str,
) -> Result<bool> {
    let public_key = RsaPublicKey::from_public_key_pem(public_key_pem)
        .map_err(|err| WechatError::Crypto(format!("invalid public key: {err}")))?;
    let verifying_key = VerifyingKey::<Sha256>::new(public_key);
    let signature_bytes = general_purpose::STANDARD
        .decode(signature_base64)
        .map_err(|err| WechatError::Crypto(format!("invalid base64 signature: {err}")))?;
    let signature = Signature::try_from(signature_bytes.as_slice())
        .map_err(|err| WechatError::Crypto(format!("invalid rsa signature: {err}")))?;
    Ok(verifying_key.verify(message, &signature).is_ok())
}

pub fn rsa_oaep_sha1_decrypt_base64(
    private_key_pem: &str,
    ciphertext_base64: &str,
) -> Result<String> {
    let private_key = RsaPrivateKey::from_pkcs8_pem(private_key_pem)
        .map_err(|err| WechatError::Crypto(format!("invalid private key: {err}")))?;
    let ciphertext = general_purpose::STANDARD
        .decode(ciphertext_base64)
        .map_err(|err| WechatError::Crypto(format!("invalid base64 ciphertext: {err}")))?;
    let plaintext = private_key
        .decrypt(Oaep::new::<Sha1>(), &ciphertext)
        .map_err(|err| WechatError::Crypto(format!("rsa oaep decrypt failed: {err}")))?;
    String::from_utf8(plaintext)
        .map_err(|err| WechatError::Crypto(format!("rsa oaep plaintext is not UTF-8: {err}")))
}

pub fn callback_aes_key(encoding_aes_key: &str) -> Result<[u8; 32]> {
    let normalized = format!("{encoding_aes_key}=");
    let key = general_purpose::STANDARD
        .decode(normalized)
        .map_err(|err| WechatError::Crypto(format!("invalid encoding aes key: {err}")))?;
    key.try_into()
        .map_err(|_| WechatError::Crypto("encoding aes key must decode to 32 bytes".to_string()))
}

pub fn callback_aes_encrypt_base64(encoding_aes_key: &str, plaintext: &[u8]) -> Result<String> {
    let key = callback_aes_key(encoding_aes_key)?;
    let iv = &key[..16];
    let mut buffer = plaintext.to_vec();
    let msg_len = buffer.len();
    buffer.resize(msg_len + 16, 0);
    let encrypted = Aes256CbcEncryptor::new((&key).into(), iv.into())
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, msg_len)
        .map_err(|err| WechatError::Crypto(format!("callback aes encrypt failed: {err}")))?;
    Ok(general_purpose::STANDARD.encode(encrypted))
}

pub fn callback_aes_decrypt_base64(
    encoding_aes_key: &str,
    ciphertext_base64: &str,
) -> Result<Vec<u8>> {
    let key = callback_aes_key(encoding_aes_key)?;
    let iv = &key[..16];
    let mut ciphertext = general_purpose::STANDARD
        .decode(ciphertext_base64)
        .map_err(|err| WechatError::Crypto(format!("invalid callback ciphertext: {err}")))?;
    Aes256CbcDecryptor::new((&key).into(), iv.into())
        .decrypt_padded_mut::<Pkcs7>(&mut ciphertext)
        .map(|plaintext| plaintext.to_vec())
        .map_err(|err| WechatError::Crypto(format!("callback aes decrypt failed: {err}")))
}

pub fn payment_v3_decrypt(
    api_v3_key: &str,
    nonce: &str,
    associated_data: &str,
    ciphertext_base64: &str,
) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(api_v3_key.as_bytes())
        .map_err(|err| WechatError::Crypto(format!("invalid api v3 key: {err}")))?;
    let ciphertext = general_purpose::STANDARD
        .decode(ciphertext_base64)
        .map_err(|err| WechatError::Crypto(format!("invalid payment ciphertext: {err}")))?;
    cipher
        .decrypt(
            Nonce::from_slice(nonce.as_bytes()),
            Payload {
                msg: &ciphertext,
                aad: associated_data.as_bytes(),
            },
        )
        .map_err(|err| WechatError::Crypto(format!("payment v3 decrypt failed: {err}")))
}

pub fn payment_v3_encrypt_for_test(
    api_v3_key: &str,
    nonce: &str,
    associated_data: &str,
    plaintext: &[u8],
) -> Result<String> {
    let cipher = Aes256Gcm::new_from_slice(api_v3_key.as_bytes())
        .map_err(|err| WechatError::Crypto(format!("invalid api v3 key: {err}")))?;
    let encrypted = cipher
        .encrypt(
            Nonce::from_slice(nonce.as_bytes()),
            Payload {
                msg: plaintext,
                aad: associated_data.as_bytes(),
            },
        )
        .map_err(|err| WechatError::Crypto(format!("payment v3 encrypt failed: {err}")))?;
    Ok(general_purpose::STANDARD.encode(encrypted))
}

#[cfg(test)]
mod tests {
    use rsa::pkcs8::{EncodePrivateKey, LineEnding};

    use super::*;

    #[test]
    fn verifies_official_account_signature() {
        let signature = sha1_signature(&["token", "1710000000", "nonce"]);
        assert!(verify_callback_signature(
            "token",
            "1710000000",
            "nonce",
            &signature
        ));
        assert!(!verify_callback_signature(
            "token",
            "1710000000",
            "other",
            &signature
        ));
    }

    #[test]
    fn hashes_sha1_payload_without_reordering() {
        assert_eq!(sha1_hex(b"abc"), "a9993e364706816aba3e25717850c26c9cd0d89d");
    }

    #[test]
    fn builds_payment_v3_message() {
        assert_eq!(
            payment_v3_message("POST", "/v3/pay/transactions/jsapi", 1, "abc", "{}"),
            "POST\n/v3/pay/transactions/jsapi\n1\nabc\n{}\n"
        );
    }

    #[test]
    fn builds_sha256_hex() {
        assert_eq!(
            sha256_hex(b"hello"),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn builds_payment_legacy_md5_signature() {
        let params = vec![
            ("nonce_str".to_string(), "abc".to_string()),
            ("empty".to_string(), String::new()),
            ("mch_id".to_string(), "1900000109".to_string()),
            ("sign".to_string(), "ignored".to_string()),
        ];
        let expected_payload = "mch_id=1900000109&nonce_str=abc&key=secret";
        let expected = format!("{:x}", md5::compute(expected_payload.as_bytes())).to_uppercase();

        assert_eq!(payment_legacy_sign(&params, "secret"), expected);
    }

    #[test]
    fn builds_payment_legacy_xml() {
        let xml = payment_legacy_xml(&[
            ("mch_id".to_string(), "1900000109".to_string()),
            ("nonce_str".to_string(), "abc".to_string()),
        ]);

        assert_eq!(
            xml,
            "<xml><mch_id><![CDATA[1900000109]]></mch_id><nonce_str><![CDATA[abc]]></nonce_str></xml>"
        );
    }

    #[test]
    fn escapes_payment_legacy_xml_cdata_end_marker() {
        let xml = payment_legacy_xml(&[("body".to_string(), "a]]>b".to_string())]);

        assert_eq!(xml, "<xml><body><![CDATA[a]]]]><![CDATA[>b]]></body></xml>");
    }

    #[test]
    fn decrypts_callback_aes_ciphertext() {
        let encoding_key = "AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQE";
        let encrypted = callback_aes_encrypt_base64(encoding_key, b"hello callback").unwrap();
        let decrypted = callback_aes_decrypt_base64(encoding_key, &encrypted).unwrap();
        assert_eq!(decrypted, b"hello callback");
    }

    #[test]
    fn decrypts_payment_v3_ciphertext() {
        let key = "0123456789abcdef0123456789abcdef";
        let nonce = "nonce-123456";
        let aad = "transaction";
        let encrypted = payment_v3_encrypt_for_test(key, nonce, aad, br#"{"ok":true}"#).unwrap();
        let decrypted = payment_v3_decrypt(key, nonce, aad, &encrypted).unwrap();
        assert_eq!(decrypted, br#"{"ok":true}"#);
    }

    #[test]
    fn decrypts_rsa_oaep_sha1_sensitive_field() {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 1024).unwrap();
        let public_key = RsaPublicKey::from(&private_key);
        let ciphertext = public_key
            .encrypt(&mut rng, Oaep::new::<Sha1>(), b"13800000000")
            .unwrap();
        let private_key_pem = private_key.to_pkcs8_pem(LineEnding::LF).unwrap();
        let encoded = general_purpose::STANDARD.encode(ciphertext);

        assert_eq!(
            rsa_oaep_sha1_decrypt_base64(private_key_pem.as_str(), &encoded).unwrap(),
            "13800000000"
        );
    }
}
