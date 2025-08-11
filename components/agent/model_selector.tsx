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

  // Populates the model selector with models, grouped by
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
                  ctx.setTotalTokens(model.contextSize)
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
          tooltip="Select Model"
          className="flex h-6 w-min px-1 max-w-30 items-center rounded-sm bg-[#D4D8E1] shadow-xs hover:bg-[#ABB0BE]"
        >
          <CrosshairIcon strokeWidth={1.6} className="flex-shrink-0" />
          <span
            className="overflow-hidden text-ellipsis whitespace-nowrap text-xs"
          >
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
