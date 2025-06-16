export type TimeNs = {
  unit: "ns"
  value: number
}

export type TimeS = {
  unit: "s"
  value: number
}

export const timeNs = (value: number): TimeNs => ({
  unit: "ns",
  value,
})

export const timeS = (value: number): TimeS => ({
  unit: "s",
  value,
})
