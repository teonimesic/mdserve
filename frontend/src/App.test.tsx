import { describe, it, expect, vi, beforeEach } from 'vitest'
import { render, waitFor } from '@testing-library/react'
import App from './App'

// Mock fetch globally
global.fetch = vi.fn()

describe('App WebSocket handling', () => {
  beforeEach(() => {
    vi.clearAllMocks()

    // Mock initial /api/files response
    ;(global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/files') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({
            files: [
              { name: 'test1.md', path: 'test1.md', is_directory: false },
              { name: 'test2.md', path: 'test2.md', is_directory: false }
            ]
          })
        })
      }
      if (url.startsWith('/api/files/')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({
            markdown: '# Test',
            metadata: {}
          })
        })
      }
      return Promise.reject(new Error('Unknown URL'))
    })
  })

  it('should refresh file list when FileAdded message is received', async () => {
    // Mock WebSocket
    const mockWebSocket = {
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
      close: vi.fn(),
      send: vi.fn(),
      readyState: WebSocket.OPEN,
      onopen: null as any,
      onmessage: null as any,
      onerror: null as any,
      onclose: null as any,
    }

    global.WebSocket = vi.fn(function(this: any) {
      return mockWebSocket
    }) as any

    const { container } = render(<App />)

    // Wait for initial load
    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/files')
    })

    // Clear fetch calls after initial load
    vi.clearAllMocks()

    // Mock updated file list with new file
    ;(global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/files') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({
            files: [
              { name: 'test1.md', path: 'test1.md', is_directory: false },
              { name: 'test2.md', path: 'test2.md', is_directory: false },
              { name: 'new-file.md', path: 'new-file.md', is_directory: false }
            ]
          })
        })
      }
      return Promise.reject(new Error('Unknown URL'))
    })

    // Simulate FileAdded WebSocket message
    const fileAddedMessage = {
      type: 'FileAdded',
      name: 'new-file.md'
    }

    // Trigger onmessage callback
    if (mockWebSocket.onmessage) {
      mockWebSocket.onmessage({
        data: JSON.stringify(fileAddedMessage)
      } as MessageEvent)
    }

    // Verify that /api/files was called to refresh the file list
    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/files')
    })

    // Verify the file list was updated
    await waitFor(() => {
      const fileList = container.querySelector('.file-list')
      expect(fileList).toBeInTheDocument()
    })
  })

  it('should NOT refresh file list when Reload message is received', async () => {
    // Mock WebSocket
    const mockWebSocket = {
      addEventListener: vi.fn(),
      removeEventListener: vi.fn(),
      close: vi.fn(),
      send: vi.fn(),
      readyState: WebSocket.OPEN,
      onopen: null as any,
      onmessage: null as any,
      onerror: null as any,
      onclose: null as any,
    }

    global.WebSocket = vi.fn(function(this: any) {
      return mockWebSocket
    }) as any

    render(<App />)

    // Wait for initial load
    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/files')
    })

    // Clear fetch calls after initial load
    vi.clearAllMocks()

    // Simulate Reload WebSocket message (file content change, not new file)
    const reloadMessage = {
      type: 'Reload'
    }

    // Trigger onmessage callback
    if (mockWebSocket.onmessage) {
      mockWebSocket.onmessage({
        data: JSON.stringify(reloadMessage)
      } as MessageEvent)
    }

    // Wait a bit
    await new Promise(resolve => setTimeout(resolve, 100))

    // Verify that /api/files was NOT called (file list should not refresh)
    expect(global.fetch).not.toHaveBeenCalledWith('/api/files')
  })
})
