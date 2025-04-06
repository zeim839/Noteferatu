import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import '@testing-library/jest-dom'

function Hello({ name }: { name: string }) {
  return <h1>Hello, {name}!</h1>
}

describe('Hello component', () => {
  it('renders with the correct name', () => {
    render(<Hello name='World' />)
    expect(screen.getByText('Hello, World!')).toBeInTheDocument()
  })

  it('renders an h1 tag', () => {
    render(<Hello name='Tauri' />)
    expect(screen.getByRole('heading', { level: 1 })).toHaveTextContent(
      'Hello, Tauri!'
    )
  })
})
