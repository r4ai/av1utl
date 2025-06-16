import { useEffect, useRef } from "react"
import { listenNewFrame } from "../lib/commands"

export const VideoPreview = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null)

  useEffect(() => {
    const canvas = canvasRef.current
    if (!canvas) return

    const ctx = canvas.getContext("2d")
    if (!ctx) return

    let imageData: ImageData | undefined = undefined

    listenNewFrame((event) => {
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
  }, [])

  return <canvas ref={canvasRef} />
}
