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
            <div className="flex items-center gap-3">
              <AnthropicLogo />
              Anthropic
            </div>
          </AccordionTrigger>
          <AccordionContent>
            <p> To get started with Anthropic, enter your API key below: </p>
            <Input
              type="password"
              className="my-4"
              placeholder="sk-ant-xxxxxx-xx-..."
            />
            <p>
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
            <div className="flex items-center gap-3">
              <GoogleLogo />
              Google Gemini
            </div>
          </AccordionTrigger>
          <AccordionContent>
            <p> To get started with Google AI, enter your API key below: </p>
            <Input
              type="password"
              className="my-4"
              placeholder="xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
            />
            <p>
              Google AI offers access to the Gemini, Gemma, and other model
              series. For instructions on obtaining an API key, see:{" "}
              <Link href="https://ai.google.dev/">ai.google.dev</Link>
            </p>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="Ollama">
          <AccordionTrigger>
            <div className="flex items-center gap-3">
              <OllamaLogo />
              Ollama
            </div>
          </AccordionTrigger>
          <AccordionContent>
            <p>
              If your Ollama client is up and running, NoteFeratu should
              already be connected. You may <b>optionally</b> specify an
              alternative connection URL below:
            </p>
            <Input
              type="password"
              className="my-4"
              placeholder="http://localhost:11434"
            />
            <p>
              Ollama is an app that let&apos;s you run models locally. For more
              information, see{" "}
              <Link href="https://ollama.com/">ollama.com</Link>
            </p>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="OpenAI">
          <AccordionTrigger>
            <div className="flex items-center gap-3">
              <OpenAILogo />
              OpenAI
            </div>
          </AccordionTrigger>
          <AccordionContent>
            <p> To get started with OpenAI, enter your API key below: </p>
            <Input
              type="password"
              className="my-4"
              placeholder="sk-proj-xxxxxxx..."
            />
            <p>
              OpenAI offers access to the ChatGPT and O1 model
              series. For instructions on obtaining an API key, see:{" "}
              <Link href="https://openai.com/index/openai-api/">OpenAI API</Link>
            </p>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="OpenRouter">
          <AccordionTrigger>
            <div className="flex items-center gap-3">
              <OpenRouterLogo />
              OpenRouter
            </div>
          </AccordionTrigger>
          <AccordionContent>
            <p> To get started with OpenRouter, enter your API key below: </p>
            <Input
              type="password"
              className="my-4"
              placeholder="sk-or-v1-xxxxxxxxx..."
            />
            <p>
              OpenRouter lets you access hundreds of models from different
              providers using a single unified API. For instructions on
              obtaining an API key, see:{" "}
              <Link href="https://openrouter.ai/docs/quickstart">OpenRouter Quickstart</Link>
            </p>
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  )
}

export { AgentSettings }
