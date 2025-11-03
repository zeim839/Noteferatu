// Ignore "unused fields" warning in structs.
#![allow(dead_code)]

use renfield::tools::{schema, tool, Schema, Tool};

// Implements examples from the Anthropic API docs.
// https://docs.claude.com/en/api/messages
#[test]
fn test_anthropic_tools() {
    #[derive(tool, schema)]
    #[tool(
        rename = "get_stock_price",
        desc = "Get the current stock price for a given ticker symbol."
    )]
    struct GetStockPrice {
        #[schema(desc = "The stock ticker symbol, e.g. AAPL for Apple Inc.")]
        ticker: String,
    }
    assert_eq!(
        GetStockPrice::as_anthropic_tool(),
        serde_json::json!({
            "name": "get_stock_price",
            "description": "Get the current stock price for a given ticker symbol.",
            "input_schema": {
              "type": "object",
              "properties": {
                "ticker": {
                  "type": "string",
                  "description": "The stock ticker symbol, e.g. AAPL for Apple Inc."
                }
              },
              "required": ["ticker"]
            }
          }
        )
    );
}

// Implements examples from the OpenAI API docs.
// https://platform.openai.com/docs/guides/function-calling
#[test]
fn test_openai_tools() {
    #[derive(tool, schema)]
    #[tool(
        rename = "get_horoscope",
        desc = "Get today's horoscope for an astrological sign.",
        strict = false
    )]
    struct GetHoroscope {
        #[schema(desc = "An astrological sign like Taurus or Aquarius")]
        sign: String,
    }
    assert_eq!(
        GetHoroscope::as_openai_tool(),
        serde_json::json!({
            "type": "function",
            "function": {
                "name": "get_horoscope",
                "description": "Get today's horoscope for an astrological sign.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "sign": {
                            "type": "string",
                            "description": "An astrological sign like Taurus or Aquarius",
                        },
                    },
                    "required": ["sign"],
                },
            },
        })
    );
    #[derive(schema)]
    #[schema(rename_all = "lowercase")]
    #[schema(desc = "Units the temperature will be returned in.")]
    enum TempUnits {
        Celsius,
        Fahrenheit,
    }
    #[derive(tool, schema)]
    #[tool(
        rename = "get_weather",
        desc = "Retrieves current weather for the given location.",
        strict = true
    )]
    struct GetWeather {
        #[schema(desc = "City and country e.g. Bogotá, Colombia")]
        location: String,
        units: TempUnits,
    }
    assert_eq!(
        GetWeather::as_openai_tool(),
        serde_json::json!({
            "type": "function",
            "function": {
                "name": "get_weather",
                "description": "Retrieves current weather for the given location.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "City and country e.g. Bogotá, Colombia"
                        },
                        "units": {
                            "enum": ["celsius", "fahrenheit"],
                            "description": "Units the temperature will be returned in.",
                        }
                    },
                    "required": ["location", "units"],
                },
                "strict": true
            },
        })
    );
}

// Implements examples from the Gemini API docs.
// https://ai.google.dev/gemini-api/docs/function-calling
#[test]
fn test_gemini_tools() {
    #[derive(tool, schema)]
    #[tool(
        rename = "get_current_temperature",
        desc = "Gets the current temperature for a given location."
    )]
    struct GetCurrentTemperature {
        #[schema(desc = "The city name, e.g. San Francisco")]
        location: String,
    }
    assert_eq!(
        GetCurrentTemperature::as_gemini_tool(),
        serde_json::json!({
            "name": "get_current_temperature",
            "description": "Gets the current temperature for a given location.",
            "parameters": {
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city name, e.g. San Francisco",
                },
            },
            "required": ["location"],
            }
        })
    );
    #[derive(tool, schema)]
    #[tool(
        rename = "schedule_meeting",
        desc = "Schedules a meeting with specified attendees at a given time and date."
    )]
    struct ScheduleMeeting {
        #[schema(desc = "List of people attending the meeting.")]
        attendees: Vec<String>,
        #[schema(desc = "Date of the meeting (e.g., '2024-07-29')")]
        date: String,
        #[schema(desc = "Time of the meeting (e.g., '15:00')")]
        time: String,
        #[schema(desc = "The subject or topic of the meeting.")]
        topic: String,
    }
    assert_eq!(
        ScheduleMeeting::as_gemini_tool(),
        serde_json::json!({
            "name": "schedule_meeting",
            "description": "Schedules a meeting with specified attendees at a given time and date.",
            "parameters": {
                "type": "object",
                "properties": {
                    "attendees": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "List of people attending the meeting.",
                    },
                    "date": {
                        "type": "string",
                        "description": "Date of the meeting (e.g., '2024-07-29')",
                    },
                    "time": {
                        "type": "string",
                        "description": "Time of the meeting (e.g., '15:00')",
                    },
                    "topic": {
                        "type": "string",
                        "description": "The subject or topic of the meeting.",
                    },
                },
                "required": ["attendees", "date", "time", "topic"],
            },
        })
    );
    #[derive(tool, schema)]
    #[tool(
        rename = "create_bar_chart",
        desc = "Creates a bar chart given a title, labels, and corresponding values."
    )]
    struct CreateChart {
        #[schema(desc = "The title for the chart.")]
        title: String,
        #[schema(desc = "List of labels for the data points (e.g., ['Q1', 'Q2', 'Q3']).")]
        labels: Vec<String>,
        #[schema(
            desc = "List of numerical values corresponding to the labels (e.g., [50000, 75000, 60000])."
        )]
        values: Vec<f32>,
    }
    assert_eq!(
        CreateChart::as_gemini_tool(),
        serde_json::json!({
            "name": "create_bar_chart",
            "description": "Creates a bar chart given a title, labels, and corresponding values.",
            "parameters": {
                "type": "object",
                "properties": {
                    "title": {
                        "type": "string",
                        "description": "The title for the chart.",
                    },
                    "labels": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "List of labels for the data points (e.g., ['Q1', 'Q2', 'Q3']).",
                    },
                    "values": {
                        "type": "array",
                        "items": f32::schema(),
                        "description": "List of numerical values corresponding to the labels (e.g., [50000, 75000, 60000]).",
                    },
                },
                "required": ["title", "labels", "values"],
            },
        })
    );
}

// Implements examples from the OpenRouter API docs.
// https://openrouter.ai/docs/features/tool-calling
#[test]
fn test_openrouter_tools() {
    #[derive(tool, schema)]
    #[tool(
        rename = "search_gutenberg_books",
        desc = "Search for books in the Project Gutenberg library"
    )]
    struct SearchGutenbergBooks {
        #[schema(desc = "List of search terms to find books")]
        search_terms: Vec<String>,
    }
    assert_eq!(
        SearchGutenbergBooks::as_openrouter_tool(),
        serde_json::json!({
          "type": "function",
          "function": {
            "name": "search_gutenberg_books",
            "description": "Search for books in the Project Gutenberg library",
            "parameters": {
              "type": "object",
              "properties": {
                "search_terms": {
                  "type": "array",
                  "items": {"type": "string"},
                  "description": "List of search terms to find books"
                }
              },
              "required": ["search_terms"]
            }
          }
        })
    );
    #[derive(tool, schema)]
    #[tool(
        rename = "search_academic_papers",
        desc = "Search for academic papers on a given topic"
    )]
    struct SearchAcademicPapers {
        query: String,
        field: Option<String>,
    }
    assert_eq!(
        SearchAcademicPapers::as_openrouter_tool(),
        serde_json::json!({
          "type": "function",
          "function": {
            "name": "search_academic_papers",
            "description": "Search for academic papers on a given topic",
            "parameters": {
              "type": "object",
              "properties": {
                "query": {"type": "string"},
                "field": {"type": "string"}
              },
              "required": ["query"]
            }
          }
        })
    );
    #[derive(tool, schema)]
    #[tool(
        rename = "get_latest_statistics",
        desc = "Get latest statistics on a topic"
    )]
    struct GetLatestStatistics {
        topic: String,
    }
    assert_eq!(
        GetLatestStatistics::as_openrouter_tool(),
        serde_json::json!({
          "type": "function",
          "function": {
            "name": "get_latest_statistics",
            "description": "Get latest statistics on a topic",
            "parameters": {
              "type": "object",
              "properties": {
                "topic": {"type": "string"},
              },
              "required": ["topic"]
            }
          }
        })
    );
}

// Implements examples from the Ollama API docs.
// https://docs.ollama.com/capabilities/tool-calling
#[test]
fn test_ollama_tools() {
    #[derive(schema, tool)]
    #[tool(
        rename = "get_temperature",
        desc = "Get the current temperature for a city"
    )]
    struct GetTemperature {
        #[schema(desc = "The name of the city")]
        city: String,
    }
    assert_eq!(
        GetTemperature::as_ollama_tool(),
        serde_json::json!({
            "type": "function",
            "function": {
                "name": "get_temperature",
                "description": "Get the current temperature for a city",
                "parameters": {
                    "type": "object",
                    "required": ["city"],
                    "properties": {
                        "city": {"type": "string", "description": "The name of the city"}
                    }
                }
            }
        })
    );
    #[derive(schema, tool)]
    #[tool(
        rename = "get_conditions",
        desc = "Get the current weather conditions for a city"
    )]
    struct GetConditions {
        #[schema(desc = "The name of the city")]
        city: String,
    }
    assert_eq!(
        GetConditions::as_ollama_tool(),
        serde_json::json!({
          "type": "function",
          "function": {
            "name": "get_conditions",
            "description": "Get the current weather conditions for a city",
            "parameters": {
              "type": "object",
              "required": ["city"],
              "properties": {
                "city": {"type": "string", "description": "The name of the city"}
              }
            }
          }
        })
    );
}

#[test]
fn test_docstring_description() {
    /// This is a description.
    #[derive(schema, tool)]
    struct Foo {}
    assert_eq!(Foo::as_gemini_tool(), serde_json::json!({
        "name": "Foo",
        "parameters": {
            "type": "object",
            "description": "This is a description.",
            "properties": {},
        },
        "description": "This is a description.",
    }));
    /// This description is overwritten.
    #[derive(schema, tool)]
    #[tool(desc = "new description")]
    struct Bar {}
    assert_eq!(Bar::as_gemini_tool(), serde_json::json!({
        "name": "Bar",
        "parameters": {
            "type": "object",
            "description": "This description is overwritten.",
            "properties": {},
        },
        "description": "new description",
    }));
}
