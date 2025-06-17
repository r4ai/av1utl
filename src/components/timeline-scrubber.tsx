import { useCallback, useEffect, useRef, useState } from "react"
import { seekTo } from "../lib/commands"
import type { TimeNs } from "../lib/time"
import { formatTimeNs, timeNs } from "../lib/time"

interface TimelineScrubberProps {
  duration: TimeNs
  currentPosition: TimeNs
  onPositionChange?: (position: TimeNs) => void
}

export const TimelineScrubber = ({
  duration,
  currentPosition,
  onPositionChange,
}: TimelineScrubberProps) => {
  const [isDragging, setIsDragging] = useState(false)
  const [localPosition, setLocalPosition] = useState(currentPosition)
  const scrubberRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    if (!isDragging) {
      setLocalPosition(currentPosition)
    }
  }, [currentPosition, isDragging])

  const updatePosition = useCallback(
    (e: MouseEvent | React.MouseEvent) => {
      if (!scrubberRef.current) return

      const rect = scrubberRef.current.getBoundingClientRect()
      const x = e.clientX - rect.left
      const percentage = Math.max(0, Math.min(1, x / rect.width))
      const newPosition = timeNs(duration.value * percentage)
      setLocalPosition(newPosition)
    },
    [duration],
  )

  const handleMouseMove = useCallback(
    (e: MouseEvent) => {
      if (isDragging) {
        updatePosition(e)
      }
    },
    [isDragging, updatePosition],
  )

  const handleMouseUp = useCallback(async () => {
    if (isDragging) {
      setIsDragging(false)
      await seekTo(localPosition)
      onPositionChange?.(localPosition)
    }
  }, [isDragging, localPosition, onPositionChange])

  const handleMouseDown = (e: React.MouseEvent) => {
    setIsDragging(true)
    updatePosition(e)
  }

  useEffect(() => {
    if (isDragging) {
      document.addEventListener("mousemove", handleMouseMove)
      document.addEventListener("mouseup", handleMouseUp)

      return () => {
        document.removeEventListener("mousemove", handleMouseMove)
        document.removeEventListener("mouseup", handleMouseUp)
      }
    }
  }, [isDragging, handleMouseMove, handleMouseUp])

  const progressPercentage =
    duration.value > 0 ? (localPosition.value / duration.value) * 100 : 0

  return (
    <div className="timeline-scrubber w-full">
      <div className="flex justify-between text-sm text-gray-600 mb-2">
        <span>{formatTimeNs(localPosition)}</span>
        <span>{formatTimeNs(duration)}</span>
      </div>
      <div
        ref={scrubberRef}
        className="relative w-full h-4 bg-gray-300 rounded cursor-pointer"
        onMouseDown={handleMouseDown}
      >
        <div
          className="absolute top-0 left-0 h-full bg-blue-500 rounded"
          style={{ width: `${progressPercentage}%` }}
        />
        <div
          className="absolute top-1/2 transform -translate-y-1/2 w-4 h-4 bg-blue-700 rounded-full border-2 border-white shadow cursor-grab active:cursor-grabbing"
          style={{ left: `calc(${progressPercentage}% - 8px)` }}
        />
      </div>
    </div>
  )
}
