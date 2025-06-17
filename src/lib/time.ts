export type TimeUnit = "ns" | "s"

export type Time<Unit extends TimeUnit> = {
  value: number
  unit: Unit
  to: <T extends TimeUnit>(unit: T) => Time<T>
}

const convertTimeValue = <From extends TimeUnit, To extends TimeUnit>(
  value: number,
  from: From,
  to: To,
): number => {
  if (from === "ns" && to === "s") {
    return value / 1_000_000_000
  }
  if (from === "s" && to === "ns") {
    return value * 1_000_000_000
  }
  return value // No conversion needed if units are the same
}

const convertTime = <From extends TimeUnit, To extends TimeUnit>(
  value: number,
  from: From,
  to: To,
): Time<To> => {
  const convertedValue = convertTimeValue(value, from, to)

  return {
    value: convertedValue,
    unit: to,
    to: (unit) => convertTime(convertedValue, to, unit),
  }
}

export const time = {
  ns: (value: number): Time<"ns"> => ({
    value,
    unit: "ns",
    to: (unit) => convertTime(value, "ns", unit),
  }),
  s: (value: number): Time<"s"> => ({
    value,
    unit: "s",
    to: (unit) => convertTime(value, "s", unit),
  }),
}

export const formatTimeNs = (time: Time<"ns">): string => {
  const totalSeconds = time.value / 1_000_000_000
  const hours = Math.floor(totalSeconds / 3600)
  const minutes = Math.floor((totalSeconds % 3600) / 60)
  const seconds = Math.floor(totalSeconds % 60)
  const milliseconds = Math.floor((time.value % 1_000_000_000) / 1_000_000)

  if (hours > 0) {
    return `${hours.toString().padStart(2, "0")}:${minutes.toString().padStart(2, "0")}:${seconds.toString().padStart(2, "0")}.${milliseconds.toString().padStart(3, "0")}`
  }
  return `${minutes.toString().padStart(2, "0")}:${seconds.toString().padStart(2, "0")}.${milliseconds.toString().padStart(3, "0")}`
}
