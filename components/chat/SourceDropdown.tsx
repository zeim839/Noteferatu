import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"

export type SourceDropdownProps = {
  value?         : string
  onValueChange? : (value: string) => void
}

export default function SourceDropdown
({ value, onValueChange } : SourceDropdownProps) {
  return (
    <Select
      defaultValue='deepseek/deepseek-r1:free'
      onValueChange={onValueChange}
      value={value}
    >
      <SelectTrigger className="w-[180px]">
        <SelectValue placeholder="Source" />
      </SelectTrigger>
      <SelectContent className='max-h-[350px]'>
        <SelectGroup>
          <SelectLabel>OpenAI</SelectLabel>
          <SelectItem value="openai/o1">
            o1
          </SelectItem>
          <SelectItem value="openai/gpt-4.5-preview">
            GPT-4.5 (Preview)
          </SelectItem>
          <SelectItem value="openai/gpt-4o-2024-11-20">
            GPT-4o
          </SelectItem>
          <SelectItem value="openai/o1-mini-2024-09-12">
            o1-mini
          </SelectItem>
          <SelectItem value="openai/gpt-4">
            GPT-4
          </SelectItem>
          <SelectItem value="openai/gpt-3.5-turbo">
            GPT-3.5-Turbo
          </SelectItem>
        </SelectGroup>
        <SelectGroup>
          <SelectLabel>Anthropic</SelectLabel>
          <SelectItem value="anthropic/claude-3.7-sonnet:beta">
            Claude 3.7 Sonnet
          </SelectItem>
          <SelectItem value="anthropic/claude-3.5-haiku-20241022:beta">
            Claude 3.5 Haiku
          </SelectItem>
          <SelectItem value="anthropic/claude-3-opus:beta">
            Claude 3 Opus
          </SelectItem>
        </SelectGroup>
        <SelectGroup>
          <SelectLabel>DeepSeek</SelectLabel>
          <SelectItem value="deepseek/deepseek-r1:free" >
            DeepSeek R1 (free)
          </SelectItem>
          <SelectItem value="deepseek/deepseek-chat:free">
            DeepSeek V3 (free)
          </SelectItem>
        </SelectGroup>
        <SelectGroup>
          <SelectLabel>Gemini</SelectLabel>
          <SelectItem value="google/gemma-3-27b-it:free">
            Gemma 3 27B (free)
          </SelectItem>
          <SelectItem value="google/gemma-3-12b-it:free">
            Gemma 3 12B (free)
          </SelectItem>
          <SelectItem value="google/gemma-3-4b-it:free">
            Gemma 3 4B (free)
          </SelectItem>
          <SelectItem value="google/gemma-3-1b-it:free">
            Gemma 3 1B (free)
          </SelectItem>
        </SelectGroup>
      </SelectContent>
    </Select>
  )
}
