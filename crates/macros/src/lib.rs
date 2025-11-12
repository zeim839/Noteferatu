#[macro_export]
macro_rules! oai_msg {
    ("developer", $first:expr $(, $rest:expr)*) => {{
        let mut content = renfield::client::openai::Content::from($first);
        $(
            let other = renfield::client::openai::Content::from($rest);
            content = content.combine(other);
        )*;
        renfield::client::openai::Message::Developer{ content }
    }};
    ("system", $first:expr $(, $rest:expr)*) => {{
        let mut content = renfield::client::openai::Content::from($first);
        $(
            let other = renfield::client::openai::Content::from($rest);
            content = content.combine(other);
        )*;
        renfield::client::openai::Message::System{ content }
    }};
    ("user", $first:expr $(, $rest:expr)*) => {{
        let mut content = renfield::client::openai::Content::from($first);
        $(
            let other = renfield::client::openai::Content::from($rest);
            content = content.combine(other);
        )*;
        renfield::client::openai::Message::User{ content }
    }};
    ("assistant", $first:expr $(, $rest:expr)*) => {{
        let mut content = renfield::client::openai::Content::from($first);
        $(
            let other = renfield::client::openai::Content::from($rest);
            content = content.combine(other);
        )*;
        renfield::client::openai::Message::Assistant{
            audio: None,
            content: Some(content),
            refusal: None,
            tool_calls: None,
        }
    }};
    ("tool", $id:expr, $first:expr $(, $rest:expr)*) => {{
        let id: String = String::from($id);
        let mut content = renfield::client::openai::Content::from($first);
        $(
            let other = renfield::client::openai::Content::from($rest);
            content = content.combine(other);
        )*;
        renfield::client::openai::Message::Tool{
            tool_call_id: id,
            content,
        }
    }};
}

#[macro_export]
macro_rules! ollama_msg {
    ("system", $text:expr) => {
        renfield::client::ollama::Message::System {
            content: String::from($text),
        }
    };
    ("user", $text:expr) => {
        renfield::client::ollama::Message::User {
            content: String::from($text),
            images: Vec::new(),
        }
    };
    ("user", $text:expr, ($($img:expr),+)) => {
        renfield::client::ollama::Message::User {
            content: String::from($text),
            images: vec![
                $( String::from($img), )*
            ],
        }
    };
    ("assistant", $text:expr) => {
        renfield::client::ollama::Message::Assistant {
            content: String::from($text),
            tool_calls: None,
            thinking: None,
            images: None,
        }
    };
    ("assistant", $text:expr, ($($img:expr),+)) => {
        renfield::client::ollama::Message::Assistant {
            content: String::from($text),
            tool_calls: None,
            thinking: None,
            images: Some(vec![
                $( String::from($img), )*
            ]),
        }
    };
    ("tool", $text:expr) => {
        renfield::client::ollama::Message::Tool {
            content: String::from($text),
        }
    };
}

#[macro_export]
macro_rules! openrouter_msg {
    ("system", $first:expr $(, $rest:expr)*) => {{
        let mut content = renfield::client::openrouter::Content::from($first);
        $(
            let other = renfield::client::openrouter::Content::from($rest);
            content = content.combine(other);
        )*;
        renfield::client::openrouter::Message::System{name: None, content}
    }};
    ("user", $first:expr $(, $rest:expr)*) => {{
        let mut content = renfield::client::openrouter::Content::from($first);
        $(
            let other = renfield::client::openrouter::Content::from($rest);
            content = content.combine(other);
        )*;
        renfield::client::openrouter::Message::User{name: None, content}
    }};
    ("developer", $first:expr $(, $rest:expr)*) => {{
        let mut content = renfield::client::openrouter::Content::from($first);
        $(
            let other = renfield::client::openrouter::Content::from($rest);
            content = content.combine(other);
        )*;
        renfield::client::openrouter::Message::Developer{name: None, content}
    }};
    ("assistant", $first:expr $(, $rest:expr)*) => {{
        let mut content = renfield::client::openrouter::Content::from($first);
        $(
            let other = renfield::client::openrouter::Content::from($rest);
            content = content.combine(other);
        )*;
        renfield::client::openrouter::Message::Assistant {
            content: Some(content),
            name: None,
            tool_calls: None,
            refusal: None,
            reasoning: None,
        }
    }};
    ("tool", $id:expr, $first:expr $(, $rest:expr)*) => {{
        let id: String = String::from($id);
        let mut content = renfield::client::openrouter::Content::from($first);
        $(
            let other = renfield::client::openrouter::Content::from($rest);
            content = content.combine(other);
        )*;
        renfield::client::openrouter::Message::Tool{
            tool_call_id: id,
            content,
        }
    }};
}

#[macro_export]
macro_rules! gemini_msg {
    ("user", $first:expr $(, $rest:expr)*) => {{
        let mut content = renfield::client::gemini::Content::from(
            renfield::client::gemini::Part::from($first)
        );
        $(
            let mut other = renfield::client::gemini::Content::from(
                renfield::client::gemini::Part::from($rest)
            );
            content = content.combine(other);
        )*;
        content.role = Some(renfield::client::gemini::Role::User);
        content
    }};
    ("model", $first:expr $(, $rest:expr)*) => {{
        let mut content = renfield::client::gemini::Content::from(
            renfield::client::gemini::Part::from($first)
        );
        $(
            let mut other = renfield::client::gemini::Content::from(
                renfield::client::gemini::Part::from($rest)
            );
            content = content.combine(other);
        )*;
        content.role = Some(renfield::client::gemini::Role::Model);
        content
    }};
}

#[macro_export]
macro_rules! anthropic_msg {
    ("user", $first:expr $(, $rest:expr)*) => {{
        let mut content = renfield::client::anthropic::Content::from(
            renfield::client::anthropic::ContentPart::from($first)
        );
        $(
            let mut other = renfield::client::anthropic::Content::from(
                renfield::client::anthropic::ContentPart::from($rest)
            );
            content = content.combine(other);
        )*;
        renfield::client::anthropic::Message::User{content}
    }};
    ("assistant", $first:expr $(, $rest:expr)*) => {{
        let mut content = renfield::client::anthropic::Content::from(
            renfield::client::anthropic::ContentPart::from($first)
        );
        $(
            let mut other = renfield::client::anthropic::Content::from(
                renfield::client::anthropic::ContentPart::from($rest)
            );
            content = content.combine(other);
        )*;
        renfield::client::anthropic::Message::Assistant{content}
    }};
}
