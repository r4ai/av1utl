import { invoke } from "@tauri-apps/api/core"
import { type EventCallback, listen } from "@tauri-apps/api/event"
import type { Time } from "./time"

export type AddClipOptions = {
  filePath: string
  layerPriority: number
  startNs: Time<"ns">
  durationNs: Time<"ns">
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

export const stopPreview = () => invoke<void>("stop_preview")

export const seekTo = (positionNs: Time<"ns">) =>
  invoke<void>("seek_to", { positionNs: positionNs.value })

export const playPause = () => invoke<void>("play_pause")

export type FramePayload = {
  width: number
  height: number
  bytes: Uint8Array
}

export const listenNewFrame = (handler: EventCallback<FramePayload>) =>
  listen<FramePayload>("new-frame", handler)
