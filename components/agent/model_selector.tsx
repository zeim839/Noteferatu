import * as React from "react"

import { CrosshairIcon } from "lucide-react"
import { Combobox } from "@/components/core/combobox"
import { Button } from "@/components/core/button"
import { useAgentContext } from "./context"

function ModelSelector() {
  const ctx = useAgentContext()

  // Toggle the settings panel.
  const onToggleSettings = () => {
    if (!ctx.isSettingsOpen) {
      ctx.setIsModelSelectorOpen(false)
    }
    ctx.toggleSettings()
  }

  // Populates the LLM model selector with models, grouped by
  // their providers.
  const modelGroups = () => {
    const models = ctx.models
    const groups = []
    for (const key in models) {
      groups.push(
        <Combobox.Group key={key} heading={key}>
          {
            models[key].map((model) => (
              <Combobox.Item
                key={`${model.provider}/${model.id}`}
                value={`${model.provider}/${model.id}`}
                onSelect={() => {
                  ctx.setSelectedModel(model)
                  ctx.setIsModelSelectorOpen(false)
                }}
              >
                {model.displayName}
              </Combobox.Item>
            ))
          }
        </Combobox.Group>
      )
    }
    return groups
  }

  return (
    <Combobox
      open={ctx.isModelSelectorOpen}
      onOpenChange={ctx.setIsModelSelectorOpen}
    >
      <Combobox.Trigger>
        <Button
          variant="outline"
          className="group/model-selector flex items-center gap-1.5 px-2 h-6 rounded-sm"
        >
          <CrosshairIcon strokeWidth={1.6} />
          <span className={`text-xs whitespace-nowrap overflow-hidden text-ellipsis transition-all duration-300 ${ctx.isModelSelectorOpen ? 'max-w-[150px]' : 'max-w-0 group-hover/model-selector:max-w-[150px]'}`}>
            {ctx.selectedModel?.displayName || "No Model Selected"}
          </span>
        </Button>
      </Combobox.Trigger>
      <Combobox.EmptyBody className="p-4">
        <div className="flex flex-col items-center gap-2.5">
          <p className="text-sm text-center">
            No models available. Please configure a LLM provider.
          </p>
          <Button size="sm" onClick={onToggleSettings}>
            Open Agent Settings
          </Button>
        </div>
      </Combobox.EmptyBody>
      { modelGroups() }
    </Combobox>
  )
}

export { ModelSelector }
