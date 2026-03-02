import { invoke } from '@tauri-apps/api/core'
import { listen, UnlistenFn } from '@tauri-apps/api/event'

// ============================================================================
// Types
// ============================================================================

/** Disconnect reason */
export type DisconnectReason =
  | { type: 'Normal' }
  | { type: 'Reset' }
  | { type: 'Timeout' }
  | { type: 'Error'; data: string }

/** Event types */
export type EventType =
  | { type: 'Connect'; data: { endpoint: string } }
  | { type: 'Disconnect'; data: { endpoint: string; reason: DisconnectReason } }
  | { type: 'Message'; data: { endpoint: string; data: number[] } }
  | { type: 'Error'; data: { endpoint: string; message: string } }

/** Event payload */
export interface EventPayload {
  id: string
  event: EventType
}

/** Event callback type */
export type EventCallback = (event: { payload: EventPayload }) => void

// ============================================================================
// API Functions
// ============================================================================

/**
 * Connect to a TCP server
 * @param id - Unique identifier for this connection
 * @param endpoint - Server address (e.g., "192.168.1.100:8080")
 */
export async function connect(id: string, endpoint: string): Promise<void> {
  await invoke('plugin:tcpclient|connect', { id, endpoint })
}

/**
 * Connect to a TCP server with a specific local address
 * @param id - Unique identifier for this connection
 * @param localAddr - Local address to bind (e.g., "192.168.1.100:0")
 * @param endpoint - Server address (e.g., "192.168.1.100:8080")
 */
export async function connectWithBind(
  id: string,
  localAddr: string,
  endpoint: string
): Promise<void> {
  await invoke('plugin:tcpclient|connect_with_bind', { id, localAddr, endpoint })
}

/**
 * Disconnect from a TCP server
 * @param id - Connection identifier
 */
export async function disconnect(id: string): Promise<void> {
  await invoke('plugin:tcpclient|disconnect', { id })
}

/**
 * Send data to the TCP server
 * @param id - Connection identifier
 * @param data - Data to send (as byte array or Uint8Array)
 */
export async function send(id: string, data: Uint8Array | number[]): Promise<void> {
  const dataArray = data instanceof Uint8Array ? Array.from(data) : data
  await invoke('plugin:tcpclient|send', { id, data: dataArray })
}

/**
 * Send a string to the TCP server (UTF-8 encoded)
 * @param id - Connection identifier
 * @param text - Text to send
 */
export async function sendString(id: string, text: string): Promise<void> {
  const encoder = new TextEncoder()
  await send(id, encoder.encode(text))
}

/**
 * Check if a connection exists
 * @param id - Connection identifier
 */
export async function isConnected(id: string): Promise<boolean> {
  return await invoke<boolean>('plugin:tcpclient|is_connected', { id })
}

/**
 * Get all active connection IDs
 */
export async function getConnections(): Promise<string[]> {
  return await invoke<string[]>('plugin:tcpclient|get_connections')
}

/**
 * Listen for TCP client events
 * @param callback - Event callback function
 */
export async function listenEvents(callback: EventCallback): Promise<UnlistenFn> {
  return await listen<EventPayload>('plugin://tcpclient', callback)
}

/**
 * Decode message data to string
 */
export function decodeMessage(data: number[]): string {
  return new TextDecoder().decode(new Uint8Array(data))
}
