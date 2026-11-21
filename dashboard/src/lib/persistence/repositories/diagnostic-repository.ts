import { eq } from 'drizzle-orm'
import { graphDiagnostics } from './schema'
import type { AppDatabase } from './connection'
import type { DiagnosticRepositoryPort } from '../../repository-ports'

/** Read diagnostic rows from the saved graph. */
export class DiagnosticRepository implements DiagnosticRepositoryPort {
  constructor(private readonly database: AppDatabase) {}

  /** Return all diagnostics for the repository in artifact order. */
  read(id: string) {
    return this.database
      .select({
        severity: graphDiagnostics.severity,
        path: graphDiagnostics.path,
        language: graphDiagnostics.language,
        message: graphDiagnostics.message,
      })
      .from(graphDiagnostics)
      .where(eq(graphDiagnostics.repositoryId, id))
      .orderBy(graphDiagnostics.ordinal)
      .all()
  }
}
