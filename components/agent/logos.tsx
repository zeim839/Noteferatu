import {
  siAnthropic,
  siGooglegemini,
  siOllama,
  siOpenai,
} from 'simple-icons'

export const AnthropicLogo = () => (
  <svg role="img" viewBox="0 0 24 24" width={16} height={16} fill="#6E747D">
    <title>{siAnthropic.title}</title>
    <path d={siAnthropic.path} />
  </svg>
)

export const GoogleLogo = () => (
  <svg role="img" viewBox="0 0 24 24" width={16} height={16} fill="#6E747D">
    <title>{siGooglegemini.title}</title>
    <path d={siGooglegemini.path} />
  </svg>
)

export const OllamaLogo = () => (
  <svg role="img" viewBox="0 0 24 24" width={16} height={16} fill="#6E747D">
    <title>{siOllama.title}</title>
    <path d={siOllama.path} />
  </svg>
)

export const OpenAILogo = () => (
  <svg role="img" viewBox="0 0 24 24" width={16} height={16} fill="#6E747D">
    <title>{siOpenai.title}</title>
    <path d={siOpenai.path} />
  </svg>
)

export const OpenRouterLogo = () => (
  <svg role="img" width="16" height="16" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg" fill="#6E747D" stroke="#6E747D">
    <g clipPath="url(#clip0_205_3)">
      <path d="M0.094 7.78c0.469 0 2.281 -0.405 3.219 -0.936s0.938 -0.531 2.875 -1.906c2.453 -1.741 4.188 -1.158 7.031 -1.158" strokeWidth="2.8125" />
      <path d="m15.969 3.797 -4.805 2.774V1.023z" />
      <path d="M0 7.781c0.469 0 2.281 0.405 3.219 0.936s0.938 0.531 2.875 1.906C8.547 12.364 10.281 11.781 13.125 11.781" strokeWidth="2.8125" />
      <path d="m15.875 11.764 -4.805 -2.774v5.548z" />
    </g>
  </svg>
)
