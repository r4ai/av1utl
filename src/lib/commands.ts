import { invoke } from "@tauri-apps/api/core"
import { type EventCallback, listen } from "@tauri-apps/api/event"
import type { TimeNs } from "./time"

export type AddClipOptions = {
  filePath: string
  layerPriority: number
  startNs: TimeNs
  durationNs: TimeNs
}

export const addClip = ({
  filePath,
  layerPriority,
  startNs,
  durationNs,
}: AddClipOptions) =>
  invoke<void>("add_clip", {
    filePath,
    layerPriority,
    startNs: startNs.value,
    durationNs: durationNs.value,
  })

export const startPreview = () => invoke<void>("start_preview")

export type FramePayload = {
  width: number
  height: number
  bytes: Uint8Array
}

export const listenNewFrame = (handler: EventCallback<FramePayload>) =>
  listen<FramePayload>("new-frame", handler)
