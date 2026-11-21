import type { GraphRevisionRepositoryPort } from '../repository-ports'

/** Read revision metadata independently of graph content. */
export class GraphRevisionService {
  constructor(private readonly revisions: GraphRevisionRepositoryPort) {}

  /** Look up graph metadata for one repository, including legacy rows without a revision ID. */
  find(id: string) {
    return this.revisions.find(id)
  }

  /** Read graph metadata keyed by repository ID. */
  read(ids: Array<string>) {
    return new Map(
      this.revisions.read(ids).map((row) => [row.repositoryId, row]),
    )
  }
}
