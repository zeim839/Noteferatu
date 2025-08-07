import * as React from "react"
import { tryConnect, listModels }  from "@/lib/agent"
import { useAgentContext } from "./context"
import { CheckIcon } from "lucide-react"

import { Link } from "@/components/core/link"
import { Input } from "@/components/core/input"

import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/core/accordion"

import {
  AnthropicLogo,
  GoogleLogo,
  OllamaLogo,
  OpenAILogo,
  OpenRouterLogo,
} from "./logos"

function AgentSettings() {
  const { setModels } = useAgentContext()
  const [apiKeys, setApiKeys] = React.useState<Record<string, string>>({})
  const [statuses, setStatuses] = React.useState<Record<string, Status>>({})
  const [errors, setErrors] = React.useState<Record<string, string>>({})
  const [timeoutIds, setTimeoutIds] = React.useState<
    Record<string, NodeJS.Timeout>
  >({})

  // Clear timeouts when the component unmounts.
  React.useEffect(() => {
    return () => {
      Object.values(timeoutIds).forEach(clearTimeout)
    }
  }, [timeoutIds])

  const onChangeKey = (
    provider: string,
    event: React.ChangeEvent<HTMLInputElement>,
  ) => {
    event.preventDefault()
    const newApiKey = event.target.value

    // Update the API key for the specific provider immediately.
    setApiKeys((prevKeys) => ({
      ...prevKeys,
      [provider]: newApiKey,
    }))

    // Set status to waiting and clear any previous errors.
    setStatuses((prev) => ({ ...prev, [provider]: "waiting" }))
    setErrors((prev) => ({ ...prev, [provider]: "" }))

    // Clear any existing timeout for this provider.
    if (timeoutIds[provider]) {
      clearTimeout(timeoutIds[provider])
    }

    // Set a new timeout to debounce the tryConnect call.
    const newTimeoutId = setTimeout(async () => {
      if (newApiKey.trim() !== "") {
        setStatuses((prev) => ({ ...prev, [provider]: "connecting" }))
        try {
          await tryConnect(provider, newApiKey)
          setStatuses((prev) => ({ ...prev, [provider]: "connected" }))
          listModels().then((models) => setModels(provider, models))
        } catch (error: unknown) {
          setStatuses((prev) => ({ ...prev, [provider]: "error" }))
          setModels(provider, [])
          if (
            error &&
            typeof error === "object" &&
            "error" in error &&
            (error as { error: unknown }).error
          ) {
            setErrors((prev) => ({
              ...prev,
              [provider]:
                (error as { error: { message?: string } }).error?.message ||
                "Invalid API Key",
            }))
          }
        }
      } else {
        setStatuses((prev) => ({ ...prev, [provider]: "idle" }))
        setModels(provider, [])
      }
    }, 500)

    setTimeoutIds((prev) => ({ ...prev, [provider]: newTimeoutId }))
  }

  return (
    <div className="h-full px-2 pt-3 min-w-[250px] overflow-auto scrollbar-hide">
      <p className="font-bold">Agent Settings</p>
      <p className="my-3 text-sm">
        LLM providers allow you to use the latest GenAI models. Configure at
        least one provider to access NoteFeratu&apos;s AI capabilities.
      </p>
      <Accordion type="single" collapsible>
        <AccordionItem value="Anthropic">
          <AccordionTrigger>
            <div className="flex items-center justify-between w-full">
              <div className="flex items-center gap-3">
                <AnthropicLogo />
                Anthropic
              </div>
              {statuses.Anthropic === "connected" && (
                <CheckIcon className="w-4 h-4 text-green-600" />
              )}
            </div>
          </AccordionTrigger>
          <AccordionContent>
            <p> To get started with Anthropic, enter your API key below: </p>
            <Input
              type="password"
              className="my-4"
              placeholder="sk-ant-xxxxxx-xx-..."
              onChange={(event) => onChangeKey("Anthropic", event)}
              value={apiKeys.Anthropic || ""}
            />
            <StatusIndicator
              status={statuses.Anthropic}
              error={errors.Anthropic}
            />
            <p className="mt-4">
              Anthropic offers access to the Claude model series. For
              instructions on obtaining an API key, see:{" "}
              <Link href="https://docs.anthropic.com/en/docs/get-started">
                Get Started with Claude
              </Link>
            </p>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="Google">
          <AccordionTrigger>
            <div className="flex items-center justify-between w-full">
              <div className="flex items-center gap-3">
                <GoogleLogo />
                Google Gemini
              </div>
              {statuses.Google === "connected" && (
                <CheckIcon className="w-4 h-4 text-green-600" />
              )}
            </div>
          </AccordionTrigger>
          <AccordionContent>
            <p> To get started with Google AI, enter your API key below: </p>
            <Input
              type="password"
              className="my-4"
              placeholder="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
              onChange={(event) => onChangeKey("Google", event)}
              value={apiKeys.Google || ""}
            />
            <StatusIndicator
              status={statuses.Google}
              error={errors.Google}
            />
            <p className="mt-4">
              Google AI offers access to the Gemini, Gemma, and other model
              series. For instructions on obtaining an API key, see:{" "}
              <Link href="https://ai.google.dev/">ai.google.dev</Link>
            </p>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="Ollama">
          <AccordionTrigger>
            <div className="flex items-center justify-between w-full">
              <div className="flex items-center gap-3">
                <OllamaLogo />
                Ollama
              </div>
              {statuses.Ollama === "connected" && (
                <CheckIcon className="w-4 h-4 text-green-600" />
              )}
            </div>
          </AccordionTrigger>
          <AccordionContent>
            <p>
              If your Ollama client is up and running, NoteFeratu should already
              be connected. You may <b>optionally</b> specify an alternative
              connection URL below:
            </p>
            <Input
              type="text"
              className="my-4"
              placeholder="http://localhost:11434"
              onChange={(event) => onChangeKey("Ollama", event)}
              value={apiKeys.Ollama || ""}
            />
            <StatusIndicator
              status={statuses.Ollama}
              error={errors.Ollama}
            />
            <p className="mt-4">
              Ollama is an app that let&apos;s you run models locally. For more
              information, see{" "}
              <Link href="https://ollama.com/">ollama.com</Link>
            </p>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="OpenAI">
          <AccordionTrigger>
            <div className="flex items-center justify-between w-full">
              <div className="flex items-center gap-3">
                <OpenAILogo />
                OpenAI
              </div>
              {statuses.OpenAI === "connected" && (
                <CheckIcon className="w-4 h-4 text-green-600" />
              )}
            </div>
          </AccordionTrigger>
          <AccordionContent>
            <p> To get started with OpenAI, enter your API key below: </p>
            <Input
              type="password"
              className="my-4"
              placeholder="sk-proj-xxxxxxx..."
              onChange={(event) => onChangeKey("OpenAI", event)}
              value={apiKeys.OpenAI || ""}
            />
            <StatusIndicator
              status={statuses.OpenAI}
              error={errors.OpenAI}
            />
            <p className="mt-4">
              OpenAI offers access to the ChatGPT and O1 model series. For
              instructions on obtaining an API key, see:{" "}
              <Link href="https://openai.com/index/openai-api/">
                OpenAI API
              </Link>
            </p>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="OpenRouter">
          <AccordionTrigger>
            <div className="flex items-center justify-between w-full">
              <div className="flex items-center gap-3">
                <OpenRouterLogo />
                OpenRouter
              </div>
              {statuses.OpenRouter === "connected" && (
                <CheckIcon className="w-4 h-4 text-green-600" />
              )}
            </div>
          </AccordionTrigger>
          <AccordionContent>
            <p> To get started with OpenRouter, enter your API key below: </p>
            <Input
              type="password"
              className="my-4"
              placeholder="sk-or-v1-xxxxxxxxx..."
              onChange={(event) => onChangeKey("OpenRouter", event)}
              value={apiKeys.OpenRouter || ""}
            />
            <StatusIndicator
              status={statuses.OpenRouter}
              error={errors.OpenRouter}
            />
            <p className="mt-4">
              OpenRouter lets you access hundreds of models from different
              providers using a single unified API. For instructions on
              obtaining an API key, see:{" "}
              <Link href="https://openrouter.ai/docs/quickstart">
                OpenRouter Quickstart
              </Link>
            </p>
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  )
}

// Possible connection statuses.
type Status = "idle" | "waiting" | "connecting" | "connected" | "error"

interface StatusIndicatorProps {
  status: Status
  error?: string
}

// Display the connection status.
function StatusIndicator({ status, error }: StatusIndicatorProps) {
  if (!status || status === "idle") {
    return null
  }

  let text = ""
  let colorClass = ""

  switch (status) {
    case "waiting":
      text = "Waiting..."
      colorClass = "text-muted-foreground"
      break
    case "connecting":
      text = "Connecting..."
      colorClass = "text-yellow-500"
      break
    case "connected":
      text = "Connected"
      colorClass = "text-green-600"
      break
    case "error":
      text = `Error: ${error || "Invalid API key."}`
      colorClass = "text-red-500"
      break
  }

  return <p className={`text-xs mt-1.5 ${colorClass}`}>{text}</p>
}

export { AgentSettings }
