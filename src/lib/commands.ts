import { invoke } from "@tauri-apps/api/core"
import type { TimeNs } from "./time"

export type AddClipOptions = {
  filePath: string
  layerPriority: number
  startNs: TimeNs
  endNs: TimeNs
}

export const addClip = ({
  filePath,
  layerPriority,
  startNs,
  endNs,
}: AddClipOptions) =>
  invoke<void>("add_clip", {
    filePath,
    layerPriority,
    startNs: startNs.value,
    endNs: endNs.value,
  })
