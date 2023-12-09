export function loop(m: any, keys: any, result?: any): string[] | void {
  if (m) {
    if (typeof m === 'string') {
      if (result) result.add(m)
      return [m]
    }

    let idx: number | string, arr: Set<string>

    if (Array.isArray(m)) {
      arr = result || new Set()

      for (idx = 0; idx < m.length; idx++) {
        loop(m[idx], keys, arr)
      }

      // return if initialized set
      if (!result && arr.size) {
        return [...arr]
      }
    } else
      for (idx in m) {
        if (keys.has(idx)) {
          return loop(m[idx], keys, result)
        }
      }
  }
}
