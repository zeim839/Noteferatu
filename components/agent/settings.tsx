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
    <div className="px-2 pt-3 min-w-[250px]">
      <p className="font-bold">Agent Settings</p>
      <p className="my-3 text-sm">
        LLM service providers give you access to the latest GenAI
        models. Configure at least one provider to access
        NoteFeratu&apos;s AI capabilities.
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
            <p>
              Anthropic provides access to the Claude model series.
              To get started with Anthropic, you&apos;ll need to register
              an API key.
              <br/>
              <br/>
              See <a href="https://docs.anthropic.com/en/docs/get-started"> Get started with Claude </a> for official instructions.
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
            Yes. It adheres to the WAI-ARIA design pattern.
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
            Yes. It adheres to the WAI-ARIA design pattern.
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
            Yes. It adheres to the WAI-ARIA design pattern.
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
            Yes. It adheres to the WAI-ARIA design pattern.
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  )
}

export {AgentSettings}
