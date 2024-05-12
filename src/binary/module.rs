use nom::{IResult, number::complete::le_u32, bytes::complete::tag};

#[derive(Debug, PartialEq, Eq)]
pub struct Module {
    pub magic: String,
    pub version: u32,
}

impl Module {
    pub fn new(input: &[u8]) -> anyhow::Result<Module> {
        let (_, module) = Module::decode(input).map_err(|e| anyhow::anyhow!("failed to parse wasm: {}", e))?;
        Ok(module)
    }

    fn decode(input: &[u8]) -> IResult<&[u8], Module> {
        let (input, _) = tag(b"\0asm")(input)?;
        let (input, version) = le_u32(input)?;

        let module = Module {
            magic: "\0asm".into(),
            version,
        };
        Ok((input, module))
    }
}

impl Default for Module {
    fn default() -> Self {
        Self {
            magic: "\0asm".to_string(),
            version: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::binary::module::Module;
    use anyhow::Result;

    #[test]
    fn decode_simplest_module() -> Result<()> {
        // プリアンブルしか存在しないwasmバイナリを生成
        let wasm = wat::parse_str("(module)")?;
        // バイナリをデコードしてModule構造体を生成
        let module = Module::new(&wasm)?;
        // 生成したModule構造体が想定通りになっているかを確認
        assert_eq!(module, Module::default());
        Ok(())
    }
}