import type { DiagnosticRepositoryPort } from '../repository-ports'

/** Expose diagnostics recorded during indexing. */
export class DiagnosticService {
  constructor(private readonly diagnostics: DiagnosticRepositoryPort) {}

  /** Read saved diagnostics for the repository without scanning source. */
  read(id: string) {
    return this.diagnostics.read(id)
  }
}
