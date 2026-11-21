import { and, eq, inArray, ne } from 'drizzle-orm'
import { graphEdges } from './schema'
import { selectedRows } from './selected-rows'
import type { AppDatabase } from './connection'
import type { RelationshipRepositoryPort } from '../../repository-ports'

/** Read graph edges for rendering and dependency analysis. */
export class RelationshipRepository implements RelationshipRepositoryPort {
  constructor(private readonly database: AppDatabase) {}

  /** Read non-containment edges in artifact order. Filter by source key when keys are supplied. */
  outgoing(id: string, keys?: Array<string>) {
    const read = (batch?: Array<string>) =>
      this.database
        .select({
          ordinal: graphEdges.ordinal,
          sourceStableKey: graphEdges.sourceStableKey,
          targetStableKey: graphEdges.targetStableKey,
          type: graphEdges.type,
          confidence: graphEdges.confidence,
          path: graphEdges.path,
          line: graphEdges.line,
          evidence: graphEdges.evidence,
          typeOnly: graphEdges.typeOnly,
        })
        .from(graphEdges)
        .where(
          and(
            eq(graphEdges.repositoryId, id),
            ne(graphEdges.type, 'CONTAINS'),
            batch ? inArray(graphEdges.sourceStableKey, batch) : undefined,
          ),
        )
        .orderBy(graphEdges.ordinal)
        .all()
    return (keys ? selectedRows(keys, read) : read()).sort(
      (a, b) => a.ordinal - b.ordinal,
    )
  }

  /** Read only the endpoint keys of non-containment edges. */
  relations(id: string) {
    return this.database
      .select({
        sourceStableKey: graphEdges.sourceStableKey,
        targetStableKey: graphEdges.targetStableKey,
      })
      .from(graphEdges)
      .where(
        and(eq(graphEdges.repositoryId, id), ne(graphEdges.type, 'CONTAINS')),
      )
      .orderBy(graphEdges.ordinal)
      .all()
  }

  /** Read saved import edges, excluding type-only imports. */
  imports(id: string) {
    return this.database
      .select()
      .from(graphEdges)
      .where(
        and(
          eq(graphEdges.repositoryId, id),
          eq(graphEdges.type, 'IMPORTS'),
          eq(graphEdges.typeOnly, false),
        ),
      )
      .orderBy(graphEdges.ordinal)
      .all()
  }
}
