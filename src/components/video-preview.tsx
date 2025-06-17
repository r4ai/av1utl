import { useEffect, useRef, useState } from "react"
import {
  listenNewFrame,
  playPause,
  startPreview,
  stopPreview,
} from "../lib/commands"

export const VideoPreview = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const [isPlaying, setIsPlaying] = useState(false)
  const [isPreviewActive, setIsPreviewActive] = useState(false)

  useEffect(() => {
    const canvas = canvasRef.current
    if (!canvas) return

    const ctx = canvas.getContext("2d")
    if (!ctx) return

    let imageData: ImageData | undefined = undefined

    const unlistenFrames = listenNewFrame((event) => {
      const { width, height, bytes } = event.payload

      if (canvas.width !== width || canvas.height !== height) {
        canvas.width = width
        canvas.height = height
        imageData = ctx.createImageData(width, height)
      }

      if (imageData) {
        imageData.data.set(new Uint8ClampedArray(bytes))
        ctx.putImageData(imageData, 0, 0)
      }
    })

    return () => {
      unlistenFrames.then((unlisten) => unlisten())
    }
  }, [])

  const handleStartPreview = async () => {
    try {
      await startPreview()
      setIsPreviewActive(true)
      setIsPlaying(true)
    } catch (error) {
      console.error("Failed to start preview:", error)
    }
  }

  const handleStopPreview = async () => {
    try {
      await stopPreview()
      setIsPreviewActive(false)
      setIsPlaying(false)
    } catch (error) {
      console.error("Failed to stop preview:", error)
    }
  }

  const handlePlayPause = async () => {
    try {
      await playPause()
      setIsPlaying(!isPlaying)
    } catch (error) {
      console.error("Failed to play/pause:", error)
    }
  }

  return (
    <div className="video-preview">
      <canvas
        ref={canvasRef}
        className="w-full h-auto border border-gray-300 rounded"
        style={{ maxWidth: "100%", maxHeight: "400px" }}
      />
      <div className="controls mt-4 flex gap-2">
        {!isPreviewActive ? (
          <button
            type="button"
            onClick={handleStartPreview}
            className="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
          >
            プレビュー開始
          </button>
        ) : (
          <>
            <button
              type="button"
              onClick={handlePlayPause}
              className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
            >
              {isPlaying ? "一時停止" : "再生"}
            </button>
            <button
              type="button"
              onClick={handleStopPreview}
              className="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
            >
              停止
            </button>
          </>
        )}
      </div>
    </div>
  )
}
