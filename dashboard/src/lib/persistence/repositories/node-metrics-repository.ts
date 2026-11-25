import { and, eq, inArray } from 'drizzle-orm'
import { graphMetrics } from './schema'
import { selectedRows } from './selected-rows'
import type { AppDatabase } from './connection'
import type { NodeMetricsRepositoryPort } from '../../repository-ports'

/** Read measured source metrics from normalized storage. */
export class NodeMetricsRepository implements NodeMetricsRepositoryPort {
  constructor(private readonly database: AppDatabase) {}

  /** Return stored measurements, including nulls, for selected node keys. */
  read(id: string, keys: Array<string>) {
    const read = (batch: Array<string>, includeExceptionMetric: boolean) =>
      this.database
        .select({
          stableKey: graphMetrics.stableKey,
          lineCount: graphMetrics.lineCount,
          parameterCount: graphMetrics.parameterCount,
          baseCyclomaticComplexity: graphMetrics.baseCyclomaticComplexity,
          nestedCyclomaticComplexity: graphMetrics.nestedCyclomaticComplexity,
          loopNestedComplexity: graphMetrics.loopNestedComplexity,
          ...(includeExceptionMetric && {
            nestedTryCatches: graphMetrics.nestedTryCatches,
          }),
          cyclomaticComplexity: graphMetrics.cyclomaticComplexity,
          maxDecisionDepth: graphMetrics.maxDecisionDepth,
        })
        .from(graphMetrics)
        .where(
          and(
            eq(graphMetrics.repositoryId, id),
            inArray(graphMetrics.stableKey, batch),
          ),
        )
        .all()
        .map((row) => ({
          ...row,
          // SQLite may return the quoted column name as text on older schemas.
          nestedTryCatches:
            typeof row.nestedTryCatches === 'number'
              ? row.nestedTryCatches
              : null,
        }))
    try {
      return selectedRows(keys, (batch) => read(batch, true))
    } catch (error) {
      // The dashboard does not migrate databases. Read old schemas without the new metric.
      if (
        error instanceof Error &&
        /no such column: .*nested_try_catches/.test(error.message)
      )
        return selectedRows(keys, (batch) => read(batch, false))
      throw error
    }
  }
}
