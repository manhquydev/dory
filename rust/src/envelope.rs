//! CLI JSON envelopes (contract §3). Hand-written. No serde.
#![allow(dead_code)] // dispatch does not call these yet

pub(crate) fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str("\\u");
                let n = c as u32;
                for shift in [12, 8, 4, 0] {
                    let nibble = ((n >> shift) & 0xf) as u8;
                    out.push(char::from(if nibble < 10 {
                        b'0' + nibble
                    } else {
                        b'a' + (nibble - 10)
                    }));
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// stdout body: `{"ok":true,"result":<object>}`
pub fn success(result_json_object: &str) -> String {
    format!("{{\"ok\":true,\"result\":{result_json_object}}}")
}

/// stderr body: `{"ok":false,"error":"..."}`
pub fn runtime_error(msg: &str) -> String {
    format!("{{\"ok\":false,\"error\":{}}}", json_string(msg))
}

/// `{"workspace":{"id":...},"tab":{"id":...},"root_pane":{"id":...,"pane_id":...}}`
///
/// IDs come from the arguments. This helper never invents `w1`.
pub fn result_workspace(id: &str, tab_id: &str, pane_id: &str) -> String {
    format!(
        "{{\"workspace\":{{\"id\":{}}},\"tab\":{{\"id\":{}}},\"root_pane\":{{\"id\":{},\"pane_id\":{}}}}}",
        json_string(id),
        json_string(tab_id),
        json_string(pane_id),
        json_string(pane_id)
    )
}

/// `{"pane":{"id":...}}`
pub fn result_pane(id: &str) -> String {
    format!("{{\"pane\":{{\"id\":{}}}}}", json_string(id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[derive(Debug, PartialEq)]
    enum J {
        Bool(bool),
        Str(String),
        Obj(BTreeMap<String, J>),
    }

    impl J {
        fn obj(&self) -> &BTreeMap<String, J> {
            match self {
                J::Obj(m) => m,
                other => panic!("expected object, got {other:?}"),
            }
        }

        fn str(&self) -> &str {
            match self {
                J::Str(s) => s,
                other => panic!("expected string, got {other:?}"),
            }
        }
    }

    struct P<'a> {
        s: &'a [u8],
        i: usize,
    }

    impl<'a> P<'a> {
        fn new(s: &'a str) -> Self {
            Self {
                s: s.as_bytes(),
                i: 0,
            }
        }

        fn skip_ws(&mut self) {
            while matches!(self.s.get(self.i), Some(b' ' | b'\n' | b'\r' | b'\t')) {
                self.i += 1;
            }
        }

        fn peek(&self) -> Option<u8> {
            self.s.get(self.i).copied()
        }

        fn bump(&mut self) -> Option<u8> {
            let c = self.peek()?;
            self.i += 1;
            Some(c)
        }

        fn eat(&mut self, want: u8) -> Result<(), String> {
            match self.bump() {
                Some(c) if c == want => Ok(()),
                got => Err(format!("expected {} got {got:?}", want as char)),
            }
        }

        fn parse(s: &str) -> Result<J, String> {
            let mut p = P::new(s);
            let v = p.value()?;
            p.skip_ws();
            if p.i != p.s.len() {
                return Err("trailing bytes".into());
            }
            Ok(v)
        }

        fn value(&mut self) -> Result<J, String> {
            self.skip_ws();
            match self.peek() {
                Some(b'{') => self.object(),
                Some(b'"') => Ok(J::Str(self.string()?)),
                Some(b't') => {
                    self.lit(b"true")?;
                    Ok(J::Bool(true))
                }
                Some(b'f') => {
                    self.lit(b"false")?;
                    Ok(J::Bool(false))
                }
                other => Err(format!("bad value start {other:?}")),
            }
        }

        fn lit(&mut self, want: &[u8]) -> Result<(), String> {
            for &b in want {
                if self.bump() != Some(b) {
                    return Err("bad literal".into());
                }
            }
            Ok(())
        }

        fn object(&mut self) -> Result<J, String> {
            self.eat(b'{')?;
            self.skip_ws();
            let mut map = BTreeMap::new();
            if self.peek() == Some(b'}') {
                self.i += 1;
                return Ok(J::Obj(map));
            }
            loop {
                self.skip_ws();
                let k = self.string()?;
                self.skip_ws();
                self.eat(b':')?;
                let v = self.value()?;
                map.insert(k, v);
                self.skip_ws();
                match self.bump() {
                    Some(b',') => continue,
                    Some(b'}') => break,
                    other => return Err(format!("bad object sep {other:?}")),
                }
            }
            Ok(J::Obj(map))
        }

        fn string(&mut self) -> Result<String, String> {
            self.eat(b'"')?;
            let mut out = String::new();
            loop {
                match self.bump() {
                    Some(b'"') => return Ok(out),
                    Some(b'\\') => match self.bump() {
                        Some(b'"') => out.push('"'),
                        Some(b'\\') => out.push('\\'),
                        Some(b'n') => out.push('\n'),
                        Some(b'r') => out.push('\r'),
                        Some(b't') => out.push('\t'),
                        Some(b'u') => {
                            let mut n = 0u32;
                            for _ in 0..4 {
                                let d = self.bump().ok_or("short \\u")?;
                                n = (n << 4)
                                    + match d {
                                        b'0'..=b'9' => u32::from(d - b'0'),
                                        b'a'..=b'f' => u32::from(d - b'a' + 10),
                                        b'A'..=b'F' => u32::from(d - b'A' + 10),
                                        _ => return Err("bad hex".into()),
                                    };
                            }
                            out.push(char::from_u32(n).ok_or("bad scalar")?);
                        }
                        other => return Err(format!("bad escape {other:?}")),
                    },
                    Some(c) if c >= 0x20 => out.push(c as char),
                    Some(_) => return Err("raw control in string".into()),
                    None => return Err("unterminated string".into()),
                }
            }
        }
    }

    #[test]
    fn success_is_parseable_ok_result_not_spike() {
        let result = result_workspace("w3", "w3:t2", "w3:p4");
        let body = success(&result);
        let v = P::parse(&body).expect("parseable");
        let top = v.obj();
        assert_eq!(top.get("ok"), Some(&J::Bool(true)));
        assert!(top.contains_key("result"), "success wraps .result");
        assert!(
            !top.contains_key("workplace"),
            "spike shape {{ok, workplace}} is forbidden"
        );
        let r = top.get("result").unwrap().obj();
        assert_eq!(
            r.get("workspace").unwrap().obj().get("id").unwrap().str(),
            "w3"
        );
        assert_eq!(
            r.get("tab").unwrap().obj().get("id").unwrap().str(),
            "w3:t2"
        );
        assert_eq!(
            r.get("root_pane").unwrap().obj().get("id").unwrap().str(),
            "w3:p4"
        );
        assert_eq!(
            r.get("root_pane")
                .unwrap()
                .obj()
                .get("pane_id")
                .unwrap()
                .str(),
            "w3:p4"
        );
        assert!(!body.contains("/workplace"));
    }

    #[test]
    fn runtime_error_is_parseable() {
        let body = runtime_error(r#"no "pane""#);
        let v = P::parse(&body).expect("parseable");
        let top = v.obj();
        assert_eq!(top.get("ok"), Some(&J::Bool(false)));
        assert_eq!(top.get("error").unwrap().str(), r#"no "pane""#);
        assert!(!top.contains_key("workplace"));
        assert!(!body.contains("/workplace"));
    }

    #[test]
    fn result_ids_come_from_arguments_never_w1() {
        let ws = result_workspace("wx", "wx:t9", "wx:p8");
        let pane = result_pane("wx:p8");
        assert!(!ws.contains("w1"), "must not invent w1, got {ws}");
        assert!(!pane.contains("w1"), "must not invent w1, got {pane}");
        let ws_v = P::parse(&ws).expect("parseable workspace result");
        let pane_v = P::parse(&pane).expect("parseable pane result");
        assert_eq!(
            ws_v.obj()
                .get("workspace")
                .unwrap()
                .obj()
                .get("id")
                .unwrap()
                .str(),
            "wx"
        );
        assert_eq!(
            pane_v
                .obj()
                .get("pane")
                .unwrap()
                .obj()
                .get("id")
                .unwrap()
                .str(),
            "wx:p8"
        );
    }
}
