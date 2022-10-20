// web_sys_html_encode_mod.rs

// TODO: check that literals are correct microxml?

/* TODO
The HtmlAttributeEncode method encodes characters appropriate for insertion into an HTML attribute value.
The string result from the HtmlAttributeEncode method should be used only for double-quoted attributes. Security issues might arise when using the HtmlAttributeEncode method with single-quoted attributes.
 */

/// Format unencoded html literal with encoded html variables
/// Similar use as format! macro.
/// html_encoded_push!(html, template, param_1)
/// html_encoded_push!(exp.explanation_all.,"{}", &exp.reg_str)
#[macro_export]
macro_rules! html_encoded_push {
    // TODO: refactor to accept any number of params
    ($html: expr, $template:expr) => {
        $html.push_to_use_only_by_the_macro_html_encoded_push(&format!($template))
    };
    ($html: expr, $template:expr, $param_1: expr) => {
        $html.push_to_use_only_by_the_macro_html_encoded_push(&format!(
            $template,
            crate::web_sys_html_encode_mod::html_encode($param_1)
        ))
    };
    ($html: expr, $template:expr, $param_1: expr, $param_2: expr) => {
        $html.push_to_use_only_by_the_macro_html_encoded_push(&format!(
            $template,
            crate::web_sys_html_encode_mod::html_encode($param_1),
            crate::web_sys_html_encode_mod::html_encode($param_2)
        ))
    };
    ($html: expr, $template:expr, $param_1: expr, $param_2: expr, $param_3: expr) => {
        $html.push_to_use_only_by_the_macro_html_encoded_push(&format!(
            $template,
            crate::web_sys_html_encode_mod::html_encode($param_1),
            crate::web_sys_html_encode_mod::html_encode($param_2),
            crate::web_sys_html_encode_mod::html_encode($param_3),
        ))
    };
    ($html: expr, $template:expr, $param_1: expr, $param_2: expr, $param_3: expr, $param_4: expr) => {
        $html.push_to_use_only_by_the_macro_html_encoded_push(&format!(
            $template,
            crate::web_sys_html_encode_mod::html_encode($param_1),
            crate::web_sys_html_encode_mod::html_encode($param_2),
            crate::web_sys_html_encode_mod::html_encode($param_3),
            crate::web_sys_html_encode_mod::html_encode($param_4),
        ))
    };
}

/// works like the format! macro, but the variables are html-encoded
#[macro_export]
macro_rules! format_html {
    // TODO: refactor to accept any number of params
    ($template:expr, $param_1: expr) => {
        format!(
            $template,
            crate::web_sys_html_encode_mod::html_encode($param_1)
        )
    };
}

/// Build a html, that is correctly encoded.
/// Use the `html_encoded_push!` macro.
/// Literals are pushed unencoded.
/// Variables are always encoded.
#[derive(Debug, Default)]
pub struct HtmlEncoded {
    // private field accessible only with methods
    html: String,
}
impl HtmlEncoded {
    /// constructor of empty object
    pub fn new() -> HtmlEncoded {
        // return
        HtmlEncoded {
            html: String::new(),
        }
    }
    /// html encode this str and create the object
    pub fn from_str(param_1: &str) -> HtmlEncoded {
        let mut html = HtmlEncoded::new();
        html_encoded_push!(html, "{}", param_1);
        //return
        html
    }
    /// Don't use this method in your code.
    /// Use the html_encoded_push! macro instead.
    pub fn push_to_use_only_by_the_macro_html_encoded_push(&mut self, encoded: &str) {
        self.html.push_str(encoded);
    }
    /// push new line is very common
    pub fn push_new_line(&mut self) {
        self.html.push_str("\n");
    }

    /// Replace inside the section with encode.
    pub fn replace_with_html_encode(&mut self, old: &str, new: &str) {
        self.html = self.html.replace(old, &html_encode(new));
    }
    /// Return the string containing correctly encoded html.
    pub fn get_html(&self) -> String {
        // return
        self.html.to_string()
    }
    /// insert html as a position
    pub fn insert_html(&mut self, pos: usize, html: &HtmlEncoded) {
        self.html.insert_str(pos, &html.get_html());
    }
}

/// HTML encode - naive. Only the bare minimum
pub fn html_encode(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace(r#"""#, "&quot;")
        .replace("'", "&apos;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}
/// Minimal HTML encode for text nodes.
/// This is wildly different then encoding for attribute values.
pub fn html_encode_text_node(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}
/// Minimal HTML encode for attributes values.
/// This is wildly different then encoding for text nodes.
/// The value MUST be properly delimited with quotes (").
/// Warning: The HTML5 standard allows attribute values delimited with aps (') and even un-delimited values!?
/// That is a garbage standard. This code will NOT allow that.
pub fn html_encode_attribute_quoted_value(input: &str) -> String {
    input.replace("&", "&amp;").replace("\"", "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_encode() {
        assert_eq!(
            html_encode(r#" & " ' < > "#),
            " &amp; &quot; &apos; &lt; &gt; "
        );
    }

    #[test]
    fn test_format_html_1() {
        let x = format_html!(r#"<div atr="{}"></div>"#, r#" & " ' < > ""#);
        assert_eq!(
            x,
            r#"<div atr=" &amp; &quot; &apos; &lt; &gt; &quot;"></div>"#
        );
    }

    #[test]
    fn test_format_html_2() {
        let x = format_html!(r#"<div>{}</div>"#, r#" & " ' < > ""#);
        assert_eq!(x, r#"<div> &amp; &quot; &apos; &lt; &gt; &quot;</div>"#);
    }
    /*
    Check html with online validator:
    https://www.freeformatter.com/html-validator.html

    <!DOCTYPE html>
    <html>
    <head>
    <meta charset="utf-8" />
    <title>authn_login</title>
    </head>
    <body>
    <div id="&amp;&quot;&apos;&lt;&gt;&quot;"></div>
    </body>
    </html>


         */
}
