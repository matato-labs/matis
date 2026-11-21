import type { NodeMetricsRepositoryPort } from '../repository-ports'

/** Expose stored node measurements without reading live source. */
export class NodeMetricsService {
  constructor(private readonly metrics: NodeMetricsRepositoryPort) {}

  /** Return available measurements by stable key, omitting null values. */
  read(id: string, keys: Array<string>) {
    return new Map(
      this.metrics
        .read(id, keys)
        .map(({ stableKey, ...values }) => [
          stableKey,
          Object.fromEntries(
            Object.entries(values).filter(([, value]) => value !== null),
          ),
        ]),
    )
  }

  /** Return saved weighted complexity totals for the requested nodes. */
  complexities(id: string, keys: Array<string>) {
    return this.metrics.read(id, keys).map((row) => ({
      stableKey: row.stableKey,
      total: row.cyclomaticComplexity,
    }))
  }
}
