use std::env;

use eyre::Result;

fn main() -> Result<()> {
    if env::var_os("CARGO_FEATURE_NOSCRIPT").is_some() {
        return Ok(());
    }

    #[cfg(windows)]
    {
        use winres;

        let mut res = winres::WindowsResource::new();

        res.set_manifest(
            r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="highestAvailable" uiAccess="false" />
      </requestedPrivileges>
    </security>
  </trustInfo>
</assembly>
            "#,
        );
        res.set_icon("assets/icon.ico");

        res.compile()?;
    }

    Ok(())
}
