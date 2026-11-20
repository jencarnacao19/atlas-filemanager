# Signed extension registry

`registry.json` pins the SHA-256 of every extension archive, and each extension
manifest pins the SHA-256 of the tools it downloads. That makes the registry the
root of trust for everything the marketplace installs. Without a signature,
anyone who can write to the bucket (or the CDN in front of it) can publish a new
registry with new hashes.

Atlas can verify an Ed25519 signature over the registry. It is off until a
public key is compiled in, so existing builds keep working.

## How it fits together

- `atlas-publisher publish` uploads `registry.json` as before. When
  `ATLAS_REGISTRY_SIGNING_KEY` is set, it also uploads `registry.signed.json`:
  `{ "payload": "<registry.json, verbatim>", "signature": "<hex>" }`. Payload and
  signature live in one object, so a reader never sees one without the other.
- A build compiled with `ATLAS_REGISTRY_PUBLIC_KEY` reads only
  `registry.signed.json` and refuses it unless the signature verifies
  (`crates/atlas-core/src/ext/signing.rs`). Builds without the key read the
  unsigned `registry.json`, exactly as before.

## Turning it on

1. Generate the key pair once, on a trusted machine:

   ```bash
   cargo run -p atlas-publisher -- keygen --out atlas-registry.key
   ```

   It prints the public key. Store the private key file in a password manager
   or secret store; never commit it.
2. Publish every extension with the private key from then on, so
   `registry.signed.json` exists and stays current:

   ```bash
   ATLAS_REGISTRY_SIGNING_KEY="$(cat atlas-registry.key)" cargo run -p atlas-publisher -- publish <extension-dir>
   ```

3. Add the public key as the repository variable `ATLAS_REGISTRY_PUBLIC_KEY`
   (Settings > Secrets and variables > Actions > Variables). The release
   workflows pass it to the build, and from then on releases verify the
   registry. Microsoft Store builds made outside these workflows need the same
   environment variable at build time.

Do step 2 before step 3: a build that verifies signatures cannot see extensions
until `registry.signed.json` exists. If the publisher runs without the key, it
warns and leaves the previous signed registry in place, so verifying builds
keep working but do not see the new release.

## Moving off r2.dev

Cloudflare documents `*.r2.dev` URLs as rate-limited and meant for development.
After pointing a custom domain at the bucket, set the repository variables
`ATLAS_REGISTRY_BASE` (for example `https://downloads.example.com`) and
`ATLAS_UPDATE_MANIFEST_URL` (the full `latest.json` URL). Empty variables keep
the built-in defaults.
