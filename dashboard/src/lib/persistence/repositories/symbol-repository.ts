import { and, eq, inArray } from 'drizzle-orm'
import { graphSymbols } from './schema'
import { selectedRows } from './selected-rows'
import type { AppDatabase } from './connection'
import type { SymbolRepositoryPort } from '../../repository-ports'

/** Read stored source spans and declaration metadata. */
export class SymbolRepository implements SymbolRepositoryPort {
  constructor(private readonly database: AppDatabase) {}

  /** Read symbol details for selected keys, or all symbols if keys are omitted. */
  read(id: string, keys?: Array<string>) {
    const read = (batch?: Array<string>) =>
      this.database
        .select({
          stableKey: graphSymbols.stableKey,
          startLine: graphSymbols.startLine,
          endLine: graphSymbols.endLine,
          startColumn: graphSymbols.startColumn,
          endColumn: graphSymbols.endColumn,
          signature: graphSymbols.signature,
          visibility: graphSymbols.visibility,
        })
        .from(graphSymbols)
        .where(
          and(
            eq(graphSymbols.repositoryId, id),
            batch ? inArray(graphSymbols.stableKey, batch) : undefined,
          ),
        )
        .all()
    return keys ? selectedRows(keys, read) : read()
  }
}
