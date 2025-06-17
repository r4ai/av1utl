import { createFileRoute } from "@tanstack/react-router"
import { type ComponentPropsWithoutRef, useState } from "react"
import { VideoPreview } from "../components/video-preview"
import { addClip } from "../lib/commands"
import { time } from "../lib/time"

const Button = ({
  onClick,
  children,
  ...props
}: ComponentPropsWithoutRef<"button">) => {
  return (
    <button
      type="button"
      className="border border-zinc-300 p-2 hover:bg-zinc-100 rounded-lg transition"
      {...props}
    >
      {children}
    </button>
  )
}

const AddClipForm = () => {
  const [filePath, setFilePath] = useState("")
  const [layerPriority, setLayerPriority] = useState(0)
  const [startNs, setStartNs] = useState(0)
  const [endNs, setEndNs] = useState(time.s(10).to("ns").value) // Default to 10 seconds

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault()
    addClip({
      filePath,
      layerPriority,
      startNs: time.ns(startNs),
      durationNs: time.ns(endNs),
    })
      .then(() => {
        // Reset form fields after successful submission
        setFilePath("")
        setLayerPriority(0)
        setStartNs(0)
        setEndNs(0)
      })
      .catch((error) => {
        console.error("Failed to add clip:", error)
        // Handle error (e.g., show a notification)
      })
  }

  return (
    <form className="flex flex-col gap-2" onSubmit={handleSubmit}>
      <label>
        File Path:
        <input
          type="text"
          name="filePath"
          className="border p-1 rounded"
          value={filePath}
          onChange={(e) => setFilePath(e.target.value)}
        />
      </label>
      <label>
        Layer Priority:
        <input
          type="number"
          name="layerPriority"
          className="border p-1 rounded"
          value={layerPriority}
          onChange={(e) => setLayerPriority(Number(e.target.value))}
        />
      </label>
      <label>
        Start Time (ns):
        <input
          type="number"
          name="startNs"
          className="border p-1 rounded"
          value={startNs}
          onChange={(e) => setStartNs(Number(e.target.value))}
        />
      </label>
      <label>
        End Time (ns):
        <input
          type="number"
          name="endNs"
          className="border p-1 rounded"
          value={endNs}
          onChange={(e) => setEndNs(Number(e.target.value))}
        />
      </label>
      <Button type="submit">Add Clip</Button>
    </form>
  )
}

const Index = () => {
  return (
    <div className="p-2 flex flex-col gap-6">
      <div className="flex flex-col gap-6">
        <AddClipForm />
      </div>
      <hr />
      <VideoPreview />
    </div>
  )
}

export const Route = createFileRoute("/")({
  component: Index,
})
